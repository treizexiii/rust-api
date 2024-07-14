use crate::ctx::Ctx;
use crate::model::DbContext;
use crate::model::task::{Task, TaskForCreate, TaskForUpdate, TaskRepository};
use crate::web::Result;
use crate::web::rpc::{ParamsForCreate, ParamsForUpdate, ParamsId};

pub async fn create_task(ctx: Ctx, db_context: DbContext, params: ParamsForCreate<TaskForCreate>)
    -> Result<Task> {
    let ParamsForCreate { data } = params;

    let id = TaskRepository::create(&ctx, &db_context, data).await?;
    let task = TaskRepository::get(&ctx, &db_context, id).await?;

    Ok(task)
}

pub async fn get_task(ctx: Ctx,db_context: DbContext,params: ParamsId) -> Result<Task> {
    let ParamsId {id} = params;

    let task = TaskRepository::get(&ctx,&db_context,id).await?;

    Ok(task)
}

pub async fn list_task(ctx: Ctx, db_context: DbContext)
    -> Result<Vec<Task>> {
    let tasks = TaskRepository::list(&ctx, &db_context).await?;

    Ok(tasks)
}

pub async fn update_task(ctx: Ctx, db_context: DbContext, params: ParamsForUpdate<TaskForUpdate>)
    -> Result<Task> {
    let ParamsForUpdate { id, data } = params;

    TaskRepository::update(&ctx, &db_context, id, data).await;
    let task = TaskRepository::get(&ctx, &db_context, id).await?;

    Ok(task)
}

pub async fn delete_task(ctx: Ctx,db_context: DbContext, params: ParamsId)  -> Result<Task> {
    let ParamsId {id} = params;

    let task = TaskRepository::get(&ctx,&db_context,id).await?;
    TaskRepository::delete(&ctx,&db_context,id).await;

    Ok(task)
}
