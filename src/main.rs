/// Server config
mod config;
/// Database
mod db;
/// Error type
mod error;
/// Routes
mod routes;
/// Templates
mod templates;

use crate::routes::{blog_index, blog_item, index, not_found};
use axum::{routing::get, Router};
use color_eyre::eyre::Result;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new();

    let pool = db::get_db_pool_and_run_migrations(&config).await?;

    let app = Router::new()
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        .nest_service(
            "/favicon.ico",
            tower_http::services::ServeFile::new("static/favicon.ico"),
        )
        .route("/", get(index))
        .route("/blog", get(blog_index))
        .route("/post/:post_id", get(blog_item))
        .fallback(not_found)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(config.socket).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
