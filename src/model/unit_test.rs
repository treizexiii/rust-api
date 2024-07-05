use crate::ctx::Ctx;
use crate::model::task::{TaskController, TaskForCreate};

// region: -- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    use anyhow::Result;
    use serial_test::serial;
    use sqlx::postgres::PgSeverity::Error;
    use crate::{_dev_utils, model};

    #[serial]
    #[tokio::test]
    pub async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        let task_c = TaskForCreate {
            title: fx_title.to_string()
        };
        let id = TaskController::create(&ctx, &mm, task_c).await?;

        let task = TaskController::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, fx_title);

        TaskController::delete(&ctx, &mm, id);

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

        let res = TaskController::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(res,
            Err(fx_not_found_err)),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
// endregion: -- Tests