use crate::model::user::{User, UserRepository};
use crate::model::DbContext;
use crate::ctx::Ctx;
use crate::Error;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::log::info;


type Db = Pool<Postgres>;

const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost:5434/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost:5434/app_db";

// const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_RECREATE_DB_FILE_NAME: &str = "00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO_PWD: &str = "welcome";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db", "FOR-DEV-ONLY");

    let current_dir = std::env::current_dir().unwrap();
    let v: Vec<_> = current_dir.components().collect();
    let path_comp = v.get(v.len().wrapping_sub(3));
    let base_dir = if Some(true) == path_comp.map(|c| c.as_os_str() == "crates") {
        v[..v.len() - 3].iter().collect::<PathBuf>()
    } else {
        current_dir.clone()
    };
    let sql_dir = base_dir.join(SQL_DIR);

    // scope for init db only
    {
        let sql_recreate_db_file = sql_dir.join(SQL_RECREATE_DB_FILE_NAME);
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, &sql_recreate_db_file).await?;
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(sql_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    let app_db = new_db_pool(PG_DEV_APP_URL).await?;

    for path in paths {
        let path_str = path.to_string_lossy();

        if path_str.ends_with(".sql")
            && !path_str.ends_with(SQL_RECREATE_DB_FILE_NAME)
        {
            pexec(&app_db, &path).await?;
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

async fn pexec(db: &Db, file: &Path) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file:?}", "FOR-DEV-ONLY");

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
