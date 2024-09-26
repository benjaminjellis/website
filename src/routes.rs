#![allow(unused_imports, dead_code)]
use crate::{
    db::{get_10_latest_posts_from_db, get_post_by_id, types::PostDb},
    error::WebsiteError,
    templates::{BlogIndexTemplate, IndexTemplate, NotFoundTemplate, Post, PostTemplate},
};
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    http::Uri,
    response::Redirect,
};
use chrono::{Datelike, NaiveDateTime};
use itertools::Itertools;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub(crate) async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

pub(crate) async fn blog_index(
    State(pool): State<Pool<Postgres>>,
) -> Result<impl IntoResponse, WebsiteError> {
    let posts = get_10_latest_posts_from_db(&pool).await?;
    let posts_with_month = posts
        .into_iter()
        .map(|post| Ok((naive_datetime_to_month_year(post.published).unwrap(), post)))
        .collect::<Result<Vec<(String, PostDb)>, WebsiteError>>()?;

    let grouped_posts = posts_with_month.into_iter().into_group_map();

    Ok(BlogIndexTemplate::from_grouped_posts(grouped_posts))
}

pub(crate) async fn blog_item(
    Path(post_id): Path<Uuid>,
    State(pool): State<Pool<Postgres>>,
) -> Result<impl IntoResponse, WebsiteError> {
    match get_post_by_id(post_id, &pool).await? {
        Some(post) => Ok(PostTemplate { post: post.into() }),
        None => Err(WebsiteError::NotFound("Post not found".into())),
    }
}

pub(crate) async fn not_found(uri: Uri) -> impl IntoResponse {
    NotFoundTemplate {
        uri: uri.to_string(),
    }
}

fn naive_datetime_to_month_year(naive_datetime: NaiveDateTime) -> Result<String, WebsiteError> {
    let month = naive_datetime.month();
    let month = match month {
        1 => Some("January"),
        2 => Some("February"),
        3 => Some("March"),
        4 => Some("April"),
        5 => Some("May"),
        6 => Some("June"),
        7 => Some("July"),
        8 => Some("August"),
        9 => Some("September"),
        10 => Some("October"),
        11 => Some("November"),
        12 => Some("December"),
        _ => None,
    }
    .ok_or_else(|| WebsiteError::DateFormat)?;

    let year = naive_datetime.year();
    Ok(format!("{month} {year}"))
}
