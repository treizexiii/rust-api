mod dev_db;

use std::error::Error;
use tokio::sync::OnceCell;
use tracing::log::info;
use crate::ctx::Ctx;
use crate::model;
use crate::model::DbContext;
use crate::model::task::{Task, TaskRepository, TaskForCreate};

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        let d = dev_db::init_dev_db().await;
        match d {
            Ok(_) => {
                info!("{:<12} - Database initialized", "FOR-DEV-ONLY");
            }
            Err(e) => {
                info!("{:<12} - Database error:{e:?}", "FOR-DEV-ONLY");
            }
        }
    })
        .await;
}

pub async fn init_test() -> DbContext {
    static INIT: OnceCell<DbContext> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            DbContext::new().await.unwrap()
        })
        .await;

    mm.clone()
}

pub async fn seed_task(
    ctx: &Ctx,
    model_manager: &DbContext,
    titles: &[&str],
) -> model::Result<Vec<Task>> {
    let mut tasks = Vec::new();

    for title in titles {
        let id =
            TaskRepository::create(&ctx, &model_manager, TaskForCreate { title: title.to_string() })
                .await?;
        let task = TaskRepository::get(&ctx, &model_manager, id).await?;

        tasks.push(task);
    }

    Ok(tasks)
}