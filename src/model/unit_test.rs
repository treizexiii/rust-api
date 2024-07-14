use crate::ctx::Ctx;
use crate::model::task::{TaskRepository, TaskForCreate};

// region: -- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    use anyhow::Result;
    use serial_test::serial;
    use sqlx::postgres::PgSeverity::Error;
    use crate::{_dev_utils, model};
    use crate::model::task::Task;

    #[serial]
    #[tokio::test]
    pub async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        let task_c = TaskForCreate {
            title: fx_title.to_string()
        };
        let id = TaskRepository::create(&ctx, &mm, task_c).await?;

        let task = TaskRepository::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, fx_title);

        TaskRepository::delete(&ctx, &mm, id);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    pub async fn test_get_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;
        let fx_not_found_err = model::Error::EntityNotFound {
            entity: "task",
            id: 100,
        };

        let res = TaskRepository::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(res,
            Err(fx_not_found_err)),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    pub async fn test_update_ok() -> Result<()> {
        Ok(())
    }

    #[serial]
    #[tokio::test]
    pub async fn test_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;
        let fx_not_found_err = model::Error::EntityNotFound {
            entity: "task",
            id: 100,
        };

        let res = TaskRepository::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(res,
            Err(fx_not_found_err)),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    pub async fn test_list_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &[
            "test_list_ok-task 01",
            "test_list_ok-task 02",
            "test_list_ok-task 03"
        ];
        _dev_utils::seed_task(&ctx, &mm, fx_titles).await?;

        let tasks = TaskRepository::list(&ctx, &mm).await?;

        let task: Vec<Task> = tasks
            .clone()
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_ok-task"))
            .collect();
        assert_eq!(task.len(), 3, "Number of seeded tasks");

        for task in tasks {
            TaskRepository::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }
}
// endregion: -- Tests
