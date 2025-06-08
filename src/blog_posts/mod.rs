use std::sync::LazyLock;

use maud::{Markup, html};

mod pocari;

pub struct BlogPost {
    pub title: &'static str,
    pub url: &'static str,
    #[allow(dead_code)]
    pub html: maud::Markup,
}

impl BlogPost {
    fn new(page: fn() -> (&'static str, &'static str, Markup)) -> Self {
        let (title, url, html) = page();
        Self { url, title, html }
    }
}

pub(crate) static BLOG_POSTS: LazyLock<[BlogPost; 1]> =
    LazyLock::new(|| [BlogPost::new(pocari::blog_post)]);

pub(in crate::blog_posts) fn paragraph(text: &'static str) -> Markup {
    html! {
        p."text-justify"{(text)}
        br;
    }
}
