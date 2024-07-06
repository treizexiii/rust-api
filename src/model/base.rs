use std::mem::take;
use sqlb::{HasFields, SqlBuilder, Whereable};
use crate::ctx::Ctx;
use crate::model::DbContext;
use crate::model::{Error, Result};
use sqlx::FromRow;
use sqlx::postgres::PgRow;

pub trait Repository {
    const TABLE: &'static str;
}

pub async fn create<EntityRepository, Entity>(_ctx: &Ctx, mm: &DbContext, entity: Entity) -> Result<i64>
where
    EntityRepository: Repository,
    Entity: HasFields,
{
    let db = mm.db();

    let fields = entity.not_none_fields();

    let (id,) = sqlb::insert()
        .table(EntityRepository::TABLE)
        .data(fields)
        .returning(&["id"])
        .fetch_one(db)
        .await?;

    Ok(id)
}

pub async fn get<EntityRepository, Entity>(_ctx: &Ctx, db_context: &DbContext, id: i64) -> Result<Entity>
where
    EntityRepository: Repository,
    Entity: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    Entity: HasFields,
{
    let db = db_context.db();

    let entity: Entity = sqlb::select()
        .table(EntityRepository::TABLE)
        .columns(Entity::field_names())
        .and_where_eq("id", id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound { entity: EntityRepository::TABLE, id })?;

    Ok(entity)
}

pub async fn list<EntityRepository, Entity>(_ctx: &Ctx, db_context: &DbContext) -> Result<Vec<Entity>>
where
    EntityRepository: Repository,
    Entity: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    Entity: HasFields,
{
    let db = db_context.db();

    let entities: Vec<Entity> = sqlb::select()
        .table(EntityRepository::TABLE)
        .columns(Entity::field_names())
        .order_by("id")
        .fetch_all(db)
        .await?;

    Ok(entities)
}

pub async fn update<EntityRepository, Entity>(_ctx: &Ctx, mm: &DbContext, id: i64, entity: Entity) -> Result<()>
where
    EntityRepository: Repository,
    Entity: HasFields,
{
    let db = mm.db();

    let fields = entity.not_none_fields();

    let count = sqlb::update()
        .table(EntityRepository::TABLE)
        .and_where_eq("id", id)
        .data(fields)
        .exec(db)
        .await?;

    if count == 0 {
        return Err(Error::EntityNotFound { entity: EntityRepository::TABLE, id });
    }

    Ok(())
}

pub async fn delete<EntityRepository, Entity>(_ctx: &Ctx, db_context: &DbContext, id: i64) -> Result<()>
where
    EntityRepository: Repository,
    Entity: HasFields,
{
    let db = db_context.db();

    let count = sqlb::delete()
        .table(EntityRepository::TABLE)
        .and_where_eq("id", id)
        .exec(db)
        .await?;

    if count == 0 {
        return Err(Error::EntityNotFound { entity: EntityRepository::TABLE, id });
    }

    Ok(())
}