use crate::{
    db::types::PostDb,
    error::WebsiteError,
    templates::{BlogIndexTemplate, IndexTemplate, NotFoundTemplate},
};
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    http::Uri,
};
use chrono::{Datelike, NaiveDateTime};
use itertools::Itertools;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub(crate) async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

pub(crate) async fn blog_index(
    State(_pool): State<Pool<Postgres>>,
) -> Result<impl IntoResponse, WebsiteError> {
    // TODO: set this up so we can do pagination, currently this gets the newest 20 posts
    let posts: Vec<PostDb> = vec![];
    let posts_with_month = posts
        .into_iter()
        .map(|post| Ok((naive_datetime_to_month_year(post.published).unwrap(), post)))
        .collect::<Result<Vec<(String, PostDb)>, WebsiteError>>()?;

    let grouped_posts = posts_with_month.into_iter().into_group_map();

    Ok(BlogIndexTemplate::from_grouped_posts(grouped_posts))
}

pub(crate) async fn blog_item(
    Path(_post_id): Path<Uuid>,
    State(_pool): State<Pool<Postgres>>,
) -> Result<impl IntoResponse, WebsiteError> {
    Ok(())
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
