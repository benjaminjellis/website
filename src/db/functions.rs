use crate::error::WebsiteError;

use super::types::PostDb;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub(crate) async fn get_10_latest_posts_from_db(
    pool: &Pool<Postgres>,
) -> Result<Vec<PostDb>, WebsiteError> {
    let posts = sqlx::query_as::<_, PostDb>("SELECT * FROM blog_posts")
        .fetch_all(pool)
        .await?;
    Ok(posts)
}

pub(crate) async fn get_post_by_id(
    id: Uuid,
    pool: &Pool<Postgres>,
) -> Result<Option<PostDb>, WebsiteError> {
    let post = sqlx::query_as::<_, PostDb>("SELECT * FROM blog_posts WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(post)
}
