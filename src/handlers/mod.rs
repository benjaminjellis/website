use axum::extract::Path;
use maud::{Markup, html};

use crate::{
    blog_posts::BLOG_POSTS,
    shared::{four_oh_four, layout},
};

pub(crate) use crate::photos::photos;

pub(crate) async fn index() -> Markup {
    let content = html! {
        h1."text-4xl font-extrabold tracking-tight" { "ABOUT" }
        p."mt-4 text-lg" {
            "in my day to day I'm a backend engineer, primarily writing Rust at a stealth fintech startup"
        }

        hr."my-8 border-0 border-t-4 border-black";

        h2."text-xl font-extrabold" { "PROJECTS" }
        ul."mt-3 list-disc pl-6 space-y-2"{
            li {
                 a."underline decoration-4 hover:no-underline" href="https://github.com/benjaminjellis/mond"{
                    "mond"
                }
                " — an experimental functional language with a Lisp-inspired syntax and ML-style static types that targets the BEAM"
            }
            li {
                a."underline decoration-4 hover:no-underline" href="https://github.com/benjaminjellis/gegen"{
                    "gegen"
                }
                " — football scores and fixtures from across the world, in the terminal"
            }
            li {
                a."underline decoration-4 hover:no-underline" href="https://github.com/benjaminjellis/cherry2"{
                    "cherry"
                }
                " — a coffee logbook to keep track of your brews (WIP)"
            }
        }

        hr."my-8 border-0 border-t-4 border-black";

        h2."text-xl font-extrabold" { "CURRENT" }
        ul."mt-3 list-disc pl-6"{
            li { "backend engineer @ stealth fintech startup" }
        }

        hr."my-8 border-0 border-t-4 border-black";

        h2."text-xl font-extrabold" { "PREVIOUS" }
        ul."mt-3 list-disc pl-6 space-y-1"{
            li { "data scientist @ Capgemini Invent" }
            li { "financial engineer @ IHS Markit" }
            li { "consultant @ EY" }
        }
    };

    layout("benjamin ellis", &content)
}

pub(crate) async fn blog_index() -> Markup {
    let post = BLOG_POSTS
        .iter()
        .map(|post| (post.title, post.url))
        .collect::<Vec<_>>();

    let blog_posts = html! {
        ul class="list-disc list-inside"{
        @for (titie, url) in &post{
            @let url = format!("/blog/{url}");
            li { a href=(url) { (titie) } }
        }
     }
    };

    layout("blog", &blog_posts)
}

pub(crate) async fn blog_post(Path(post_name): Path<String>) -> Markup {
    match BLOG_POSTS.iter().find(|post| post.url == post_name) {
        Some(blog_post) => layout(blog_post.title, &blog_post.html),
        None => four_oh_four(),
    }
}
