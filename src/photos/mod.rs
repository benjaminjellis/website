use maud::{Markup, html};

use crate::shared::layout;

pub(crate) async fn photos() -> Markup {
    // (image url, alt text, caption)
    let items: Vec<(&str, &str, &str)> = vec![
        (
            "https://picsum.photos/id/1015/800/600",
            "mountain landscape",
            "Landscape study",
        ),
        (
            "https://picsum.photos/id/1025/800/600",
            "dog portrait",
            "Portrait",
        ),
        (
            "https://picsum.photos/id/1035/800/600",
            "architecture",
            "Structure",
        ),
        (
            "https://picsum.photos/id/1043/800/600",
            "street scene",
            "Street",
        ),
        ("https://picsum.photos/id/1050/800/600", "water", "Water"),
        ("https://picsum.photos/id/1062/800/600", "forest", "Light"),
    ];

    let content = html! {
        h1."text-3xl font-extrabold" { "PHOTOS" }
        p."mt-2 text-base" {
            "A small set. No albums. No endless scroll."
        }

        hr."my-8 border-0 border-t-4 border-black";

        div."grid gap-6 sm:grid-cols-2 lg:grid-cols-3" {
            @for (src, alt, caption) in items {
                figure."border-2 border-black bg-white" {
                    a href=(src) {
                        img
                            ."block w-full h-64 object-cover border-b-2 border-black"
                            src=(src)
                            alt=(alt);
                    }
                    figcaption."p-3 text-sm" {
                        (caption)
                    }
                }
            }
        }
    };

    layout("photos", &content)
}
