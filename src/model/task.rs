use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::ctx::Ctx;

// region: -- Task Types

#[derive(Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub cid: u64,
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

    // pub async fn get(
    //     _ctx: &Ctx,
    //     model_manager: &ModelManager,
    //     id: i64
    // ) -> Result<Task> {
    //     let db = model_manager.db();
    //
    //     let task: Task = sqlx::query_as("SELECT * FROM task WHERE id = $1")
    //         .bind(id)
    //         .fetch_optional(db)
    //         .await?
    //         .ok_or(Error::EntityNotFound { entity: "task".to_string(), id})?;
    //
    //     Ok(task)
    // }
}

// endregion: -- TaskController

// region: -- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;
    use crate::_dev_utils;

    #[tokio::test]
    pub async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        let task_c = TaskForCreate {
            title: fx_title.to_string()
        };
        let id = TaskController::create(&ctx, &mm, task_c).await?;

        let (title,): (String,) =
            sqlx::query_as("SELECT title FROM task WHERE id = $1")
                .bind(id)
                .fetch_one(mm.db())
                .await?;
        assert_eq!(title, fx_title);

        let count = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();

        Ok(())
    }
}
// endregion: -- Tests
