use maud::{DOCTYPE, Markup, html};

fn nav_item(link: &str, text: &str) -> Markup {
    html! {
        li {
            a
            ."
            block
            py-2
            text-center
            font-bold
            text-black
            border-b-2 border-black
            hover:bg-black hover:text-yellow-200
            transition-none
            "
            href=(link)
            {
                (text)
            }
        }
    }
}

pub(crate) fn four_oh_four() -> Markup {
    layout(
        "uh oh :(",
        &html!(
            p{"we coulnd't find"}
        ),
    )
}

pub(crate) fn layout(title: &str, content: &Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link href="/static/output.css" rel="stylesheet";
            title { (title) }
        }
        body."min-h-screen bg-lime-200 text-black font-mono"{
            div."min-h-screen flex"{
                aside."w-64 shrink-0 border-r-4 border-black p-6 flex flex-col justify-between"{
                    nav {
                        ul."space-y-2 text-lg" {
                            (nav_item("/", "home"))
                            (nav_item("/blog", "blog"))
                            (nav_item("https://github.com/benjaminjellis", "github"))
                            (nav_item("https://lichess.org/@/agnesmartinstan", "lichess"))
                            (nav_item("https://www.linkedin.com/in/benjamin-ellis-7420b1150/", "linkedin"))
                        }
                    }

                    div."pt-6 border-t-4 border-black"{
                        h1."text-3xl font-extrabold leading-none" { "Benjamin" }
                        h1."text-3xl font-extrabold leading-none" { "Ellis" }
                    }
                }

            main."flex-1 p-8 flex justify-center items-start"{
                section."w-fit max-w-3xl border-4 border-black bg-white p-6 shadow-[10px_10px_0_0_#000]"{
                (content)
                }
            }
        }
        }

    }
}
