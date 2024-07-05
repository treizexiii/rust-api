mod dev_db;

use std::error::Error;
use tokio::sync::OnceCell;
use tracing::log::info;
use crate::model::ModelManager;

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

pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}