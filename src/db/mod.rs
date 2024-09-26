use color_eyre::eyre::Context;
use migrations::MIGRATOR;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{config::Config, error::WebsiteError};
pub(crate) use functions::{get_10_latest_posts_from_db, get_post_by_id};

mod functions;
mod migrations;
pub(crate) mod types;

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
