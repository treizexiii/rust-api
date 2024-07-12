use crate::ctx::Ctx;
use crate::model::user::{User, UserRepository};
use crate::model::DbContext;
use crate::Error;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tracing::log::info;

type Db = Pool<Postgres>;

const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost:5434/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost:5434/app_db";

const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO_PWD: &str = "welcome";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db", "FOR-DEV-ONLY");
    // scope for init db only
    {
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    let app_db = new_db_pool(PG_DEV_APP_URL).await?;
    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace('\\', "/");

            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                pexec(&app_db, &path).await?
            }
        }
    }

    let db = DbContext::new().await?;
    let ctx = Ctx::root_ctx();

    let demo1_user: User = UserRepository::first_by_username(&ctx, &db, "demo1")
        .await?
        .unwrap();

    UserRepository::update_pwd(&ctx, &db, demo1_user.id, DEMO_PWD).await?;

    info!("{:<12} - init-dev-db - set demo1 pwd", "FOR-DEV-ONLY");

    Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

    let content = fs::read_to_string(file)?;

    let sql: Vec<&str> = content.split(';').collect();

    for query in sql {
        sqlx::query(query).execute(db).await?;
    }

    Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_con_url)
        .await
}
