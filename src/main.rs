mod blog_posts;
mod handlers;
mod photos;
pub(crate) mod shared;

use axum::{Router, routing::get};
use handlers::{blog_index, blog_post, index, photos};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/blog", get(blog_index))
        .route("/blog/{post_id}", get(blog_post))
        .route("/photos", get(photos))
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
