use crate::ctx::Ctx;
use crate::model::base::{ self, Repository };
use crate::model::{ Error, Result };
use crate::model::DbContext;
use serde::{ Deserialize, Serialize };
use sqlb::{ Field, Fields, HasFields };
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

pub struct UserRepository;

impl Repository for UserRepository {
    const TABLE: &'static str = "user";
}

impl UserRepository {
    pub async fn get<E>(ctx: &Ctx, db_context: &DbContext, id: i64) -> Result<E> where E: UserBy {
        base::get::<Self, E>(ctx, db_context, id).await
    }

    pub async fn first_by_username<E>(
        ctx: &Ctx,
        db_context: &DbContext,
        username: &str
    ) -> Result<Option<E>>
        where E: UserBy
    {
        let db = db_context.db();

        let user = sqlb
            ::select()
            .table(Self::TABLE)
            .and_where_eq("username", username)
            .fetch_optional::<_, E>(db).await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{ Context, Ok, Result };

    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        let db_context = DbContext::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";

        let user: User = UserRepository::first_by_username(
            &ctx,
            &db_context,
            fx_username
        ).await?.context("Should have user 'demo1'")?;

        assert_eq!(user.username, fx_username);

        Ok(())
    }
}
