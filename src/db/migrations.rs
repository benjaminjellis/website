use color_eyre::eyre::Context;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, Pool, Postgres};

use crate::{config::Config, error::WebsiteError};

pub(crate) static MIGRATOR: Migrator = sqlx::migrate!("./migrations/");

pub async fn get_db_pool_and_run_migrations(
    config: &Config,
) -> Result<Pool<Postgres>, WebsiteError> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await
        .context("Failed to connect to db")?;

    MIGRATOR.run(&pool).await.expect("Failed to run migrations");
    Ok(pool)
}
