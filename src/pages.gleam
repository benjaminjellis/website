import gleam/string
import layout.{layout}
import lustre/attribute.{class, href}
import lustre/element
import lustre/element/html.{a, br, h2, li, p, text, ul}

fn blog_link(link: String, blog_name: String) {
  let link = string.join(["blog", ..[link]], "/")
  li([class("italic")], [
    a([href(link), class("italic hover:underline")], [text(blog_name)]),
  ])
}

pub fn blog_index() {
  layout("benjamin ellis - blog", [
    h2([class("font-bold text-xl")], [text("2025:")]),
    ul([class("list-disc list-inside")], [
      blog_link("pocari", "pocari"),
      // blog_link("2025_photos", "2025 photos"),
    ]),
  ])
}

pub fn index() -> element.Element(t) {
  layout("benjamin ellis", [
    h2([class("font-bold")], [text("about me:")]),
    p([], [
      text(
        "in my day to day I'm a backend engineer, primarily writing Rust at a stealth fintech startup",
      ),
    ]),
    br([]),
    h2([class("font-bold")], [text("some projects of varying usefulnesss:")]),
    ul([class("list-disc list-inside")], [
      li([class("italic")], [
        a(
          [
            class("hover:underline"),
            href("https://github.com/benjaminjellis/gegen"),
          ],
          [
            text(
              "gegen: football scores and fixtures from across the world, in the terminal ",
            ),
          ],
        ),
      ]),
      li([class("italic")], [
        a(
          [
            class("hover:underline"),
            href("https://github.com/benjaminjellis/cherry2"),
          ],
          [
            text(
              "cherry: a a coffee logbook to help you keep track of your brews (currently WIP)",
            ),
          ],
        ),
      ]),
    ]),
    br([]),
    h2([class("font-bold")], [text("current:")]),
    ul([class("list-disc list-inside")], [
      li([], [text("backend engineer @ stealth fintech startup")]),
    ]),
    br([]),
    h2([class("font-bold")], [text("previous:")]),
    ul([class("list-disc list-inside")], [
      li([], [text("data scientist @ Capgemini Invent")]),
      li([], [text("financial engineer @ IHS Markit")]),
      li([], [text("consultant @ EY")]),
    ]),
  ])
}
