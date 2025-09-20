import lustre/attribute.{charset, class, content, href, name, rel}
import lustre/element
import lustre/element/html.{
  a, aside, body, div, h1, head, html, li, link, main, meta, nav, p, text, title,
  ul,
}

pub fn paragraph(content: String) {
  p([class("text-justify")], [text(content)])
}

pub fn layout(
  page_title: String,
  main_content: List(element.Element(t)),
) -> element.Element(t) {
  html([], [
    head([], [
      meta([
        charset("UTF-8"),
        name("viewport"),
        content("width=device-width, initial-scale=1.0"),
      ]),
      link([href("/priv/output.css"), rel("stylesheet")]),
      title([], page_title),
    ]),
    body([class("flex h-screen")], [
      aside(
        [
          class(
            "bg-lime-200 w-1/10 flex flex-col border-r border-solid border-black-1000",
          ),
        ],
        [
          nav([class("h-4/5 p-8")], [
            ul([class("space-y-1")], [
              nav_item("/", "home"),
              nav_item("/blog", "blog"),
              nav_item("/photos", "photos"),
              nav_item("https://github.com/benjaminjellis", "github"),
              nav_item("https://lichess.org/@/agnesmartinstan", "lichess"),
              nav_item(
                "https://www.linkedin.com/in/benjamin-ellis-7420b1150/",
                "linkedin",
              ),
            ]),
          ]),
          div([class("h-1/5 p-4 flex items-center justify-center flex-col")], [
            h1(
              [
                class(
                  "font-bold font-helvetica text-black-1000 text-center leading-tight",
                ),
              ],
              [text("benjamin")],
            ),
            h1(
              [
                class(" font-bold text-black-1000 text-center leading-tight"),
              ],
              [text("ellis")],
            ),
          ]),
        ],
      ),
      main([class("flex-1 p-8 ")], main_content),
    ]),
  ])
}

fn nav_item(link: String, item_text: String) {
  li([], [
    a(
      [
        class(
          "block py-2 text-black-1000 hover:underline text-center font-bold",
        ),
        href(link),
      ],
      [text(item_text)],
    ),
  ])
}
