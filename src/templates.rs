use std::collections::HashMap;

use askama::Template;
use uuid::Uuid;

use crate::db::types::PostDb;

#[derive(Template)]
#[template(path = "blog_index.html")]
pub struct BlogIndexTemplate {
    pub months: Vec<PostsForMonth>,
}

impl BlogIndexTemplate {
    pub(crate) fn from_grouped_posts(grouped_posts: HashMap<String, Vec<PostDb>>) -> Self {
        Self {
            months: grouped_posts
                .into_iter()
                .map(|(month, posts)| PostsForMonth {
                    month,
                    posts: posts.into_iter().map(Into::into).collect::<_>(),
                })
                .collect::<_>(),
        }
    }
}

pub struct PostsForMonth {
    pub month: String,
    pub posts: Vec<Post>,
}

#[allow(dead_code)]
pub struct PostOverview {
    pub title: String,
    pub id: Uuid,
    pub lede: String,
}

impl From<PostDb> for PostOverview {
    fn from(value: PostDb) -> Self {
        Self {
            title: value.title,
            id: value.id,
            lede: value.lede,
        }
    }
}

pub struct Post {
    pub title: String,
    pub id: Uuid,
    pub lede: String,
    pub content: String,
}

impl From<PostDb> for Post {
    fn from(value: PostDb) -> Self {
        Self {
            title: value.title,
            id: value.id,
            lede: value.lede,
            content: value.content,
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub post: Post,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate {
    pub uri: String,
}

#[derive(Template)]
#[template(path = "500.html")]
pub struct InternalServerErrorTemplate {
    pub message: String,
}
