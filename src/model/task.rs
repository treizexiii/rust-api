use crate::model::{base, DbContext};
use crate::model::Result;
use crate::ctx::Ctx;
use serde::{Deserialize, Serialize};
use modql::field::Fields;
use sqlx::FromRow;
use crate::model::base::Repository;
// region: -- Task Types

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,

    // #[field(skip)]
    // pub desc: String,
}

#[derive(Fields, Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Fields, Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}

// endregion: -- Task Types

// region: -- TaskController

pub struct TaskRepository {}

impl Repository for TaskRepository {
    const TABLE: &'static str = "task";
}

impl TaskRepository {
    pub async fn create(
        ctx: &Ctx,
        db_context: &DbContext,
        task_c: TaskForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, db_context, task_c).await
    }

    pub async fn list(
        ctx: &Ctx,
        db_context: &DbContext,
    ) -> Result<Vec<Task>> {
        base::list::<Self, _>(ctx, db_context).await
    }

    pub async fn get(
        ctx: &Ctx,
        db_context: &DbContext,
        id: i64,
    ) -> Result<Task> {
        base::get::<Self, _>(ctx, db_context, id).await
    }

    pub async fn update(
        ctx: &Ctx,
        db_context: &DbContext,
        id: i64,
        task_u: TaskForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, db_context, id, task_u).await
    }

    pub async fn delete(
        ctx: &Ctx,
        db_context: &DbContext,
        id: i64,
    ) -> Result<()> {
        base::delete::<Self, Task>(ctx, db_context, id).await
    }
}

// endregion: -- TaskController
