use crate::ctx::Ctx;
use crate::model::DbContext;
use crate::model::{Error, Result};
use modql::field::HasFields;
use modql::SIden;
use sea_query::{Expr, Iden, IntoIden, PostgresQueryBuilder, Query, TableRef};
use sea_query_binder::SqlxBinder;
use serde::de::value;
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use std::mem::take;

#[derive(Iden)]
pub enum CommonIden {
    Id,
}

pub trait Repository {
    const TABLE: &'static str;

    fn table() -> TableRef {
        TableRef::Table(SIden(Self::TABLE).into_iden())
    }
}

pub async fn create<EntityRepository, Entity>(
    _ctx: &Ctx,
    mm: &DbContext,
    entity: Entity,
) -> Result<i64>
where
    EntityRepository: Repository,
    Entity: HasFields,
{
    let db = mm.db();

    let fields = entity.not_none_fields();
    let (columns, sea_values) = fields.for_sea_insert();

    let mut query = Query::insert();
    query
        .into_table(EntityRepository::table())
        .columns(columns)
        .values(sea_values)?
        .returning(Query::returning().columns([CommonIden::Id]));

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let (id,) = sqlx::query_as_with::<_, (i64,), _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(id)
}

pub async fn get<EntityRepository, Entity>(
    _ctx: &Ctx,
    db_context: &DbContext,
    id: i64,
) -> Result<Entity>
where
    EntityRepository: Repository,
    Entity: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    Entity: HasFields,
{
    let db = db_context.db();

    let mut query = Query::select();
    query
        .from(EntityRepository::table())
        .columns(Entity::field_column_refs())
        .and_where(Expr::col(CommonIden::Id).eq(id));

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let entity = sqlx::query_as_with::<_, Entity, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: EntityRepository::TABLE,
            id: id,
        })?;

    Ok(entity)
}

pub async fn list<EntityRepository, Entity>(
    _ctx: &Ctx,
    db_context: &DbContext,
) -> Result<Vec<Entity>>
where
    EntityRepository: Repository,
    Entity: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    Entity: HasFields,
{
    let db = db_context.db();

    let mut query = Query::select();
    query
        .from(EntityRepository::table())
        .columns(Entity::field_column_refs());

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let entities = sqlx::query_as_with::<_, Entity, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(entities)
}

pub async fn update<EntityRepository, Entity>(
    _ctx: &Ctx,
    mm: &DbContext,
    id: i64,
    entity: Entity,
) -> Result<()>
where
    EntityRepository: Repository,
    Entity: HasFields,
{
    let db = mm.db();

    let fields = entity.not_none_fields();
    let fields = fields.for_sea_update();

    let mut query = Query::update();
    query
        .table(EntityRepository::table())
        .values(fields)
        .and_where(Expr::col(CommonIden::Id).eq(id));

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let count = sqlx::query_with(&sql, values)
        .execute(db)
        .await?
        .rows_affected();

    if count == 0 {
        Err(Error::EntityNotFound {
            entity: EntityRepository::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}

pub async fn delete<EntityRepository, Entity>(
    _ctx: &Ctx,
    db_context: &DbContext,
    id: i64,
) -> Result<()>
where
    EntityRepository: Repository,
    Entity: HasFields,
{
    let db = db_context.db();

    let mut query = Query::delete();
    query
        .from_table(EntityRepository::table())
        .and_where(Expr::col(CommonIden::Id).eq(id));

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let count = sqlx::query_with(&sql, values)
        .execute(db)
        .await?
        .rows_affected();


    if count == 0 {
        return Err(Error::EntityNotFound {
            entity: EntityRepository::TABLE,
            id,
        });
    }

    Ok(())
}
