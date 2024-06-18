mod dev_db;

use std::error::Error;
use tokio::sync::OnceCell;
use tracing::log::info;

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev", "FOR-DEV-ONLY");

        let d = dev_db::init_dev_db().await;
        match d {
            Ok(_) => {
                println!("Database initialized");
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
            }
        }
    })
        .await;
}