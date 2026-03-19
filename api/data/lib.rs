pub mod config;
pub mod error;
mod models;
pub mod routes;
mod utils;

use config::Config;
use error::DataError;
use sqlx::sqlite::SqlitePool;

pub async fn run(config: Config) -> Result<(), DataError> {
    let db_url = utils::get_db_url(&config.env);
    let conn_string = format!("{db_url}?mode=rwc");

    let pool = SqlitePool::connect(&conn_string)
        .await
        .map_err(|e| DataError::DatabaseError(e.to_string()))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| DataError::DatabaseError(e.to_string()))?;

    Ok(())
}
