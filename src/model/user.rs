use std::vec;

use crate::ctx::Ctx;
use crate::model::base::{self, Repository};
use crate::model::DbContext;
use crate::model::Result;
use crate::pwd::{self, ContentToHash};
use hmac::digest::typenum::Exp;
use modql::field::{Field, Fields, HasFields};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::de::value;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow, Fields, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

#[derive(Fields)]
pub struct UserForInsert {
    username: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,

    pub pwd: Option<String>,
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,

    pub token_salt: Uuid,
}

pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

#[derive(Iden)]
pub enum UserIden {
    Id,
    Username,
    Pwd,
}

pub struct UserRepository;

impl Repository for UserRepository {
    const TABLE: &'static str = "user";
}

impl UserRepository {
    pub async fn get<E>(ctx: &Ctx, db_context: &DbContext, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, E>(ctx, db_context, id).await
    }

    pub async fn first_by_username<E>(
        ctx: &Ctx,
        db_context: &DbContext,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = db_context.db();

        let mut query = Query::select();
        query
            .from(Self::table())
            .columns(E::field_idens())
            .and_where(Expr::col(UserIden::Username).eq(username));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let user = sqlx::query_as_with(&sql, values).fetch_optional(db).await?;

        Ok(user)
    }

    pub async fn update_pwd(
        ctx: &Ctx,
        db_context: &DbContext,
        id: i64,
        pwd_clear: &str,
    ) -> Result<()> {
        let db = db_context.db();

        let user: UserForLogin = Self::get(ctx, db_context, id).await?;

        let pwd = pwd::hash_pwd(&ContentToHash {
            content: pwd_clear.to_string(),
            salt: user.pwd_salt,
        })?;

        let mut query = Query::update();
        query
            .table(Self::table())
            .value(UserIden::Pwd, SimpleExpr::from(pwd))
            .and_where(Expr::col(UserIden::Id).eq(id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let _count = sqlx::query_with(&sql, values)
            .execute(db)
            .await?
            .rows_affected();
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Ok, Result};

    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        let db_context = DbContext::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";

        let user: User = UserRepository::first_by_username(&ctx, &db_context, fx_username)
            .await?
            .context("Should have user 'demo1'")?;

        assert_eq!(user.username, fx_username);

        Ok(())
    }
}
