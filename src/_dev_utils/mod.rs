mod dev_db;

use std::error::Error;
use tokio::sync::OnceCell;
use tracing::log::info;

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