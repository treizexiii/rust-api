use crate::model::ModelManager;
use crate::model::{Error, Result};
use crate::ctx::Ctx;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: -- Task Types

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}

// endregion: -- Task Types

// region: -- TaskController

pub struct TaskController {}

impl TaskController {
    pub async fn create(
        _ctx: &Ctx,
        model_manager: &ModelManager,
        task_c: TaskForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id, ) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO task (title) values ($1) returning id;"
        )
            .bind(task_c.title)
            .fetch_one(db)
            .await?;

        Ok(id)
    }

    pub async fn get(
        _ctx: &Ctx,
        model_manager: &ModelManager,
        id: i64
    ) -> Result<Task> {
        let db = model_manager.db();

        let task: Task = sqlx::query_as("SELECT * FROM task WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound { entity: "task", id })?;

        Ok(task)
    }

    pub async fn delete(
        _ctx: &Ctx,
        model_manager: &ModelManager,
        id: i64
    ) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query(
            "DELETE FROM task WHERE id = $1;"
        )
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound { entity: "task", id});
        }

        Ok(())
    }

}

// endregion: -- TaskController
