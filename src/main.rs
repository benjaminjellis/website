mod blog_posts;
mod handlers;
pub(crate) mod shared;

use axum::{Router, routing::get};
use handlers::{blog_overview, blog_post, index};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/blog", get(blog_overview))
        .route("/blog/{post_id}", get(blog_post))
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .nest_service("/static", ServeDir::new("static"));

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
