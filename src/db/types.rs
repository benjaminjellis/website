use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub(crate) struct PostDb {
    pub(crate) id: Uuid,
    pub(crate) title: String,
    pub(crate) lede: String,
    pub(crate) published: NaiveDateTime,
    pub(crate) content: String,
}
