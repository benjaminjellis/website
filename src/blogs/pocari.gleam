import layout.{layout, paragraph}
import lustre/attribute.{class, href}
import lustre/element/html.{a, br, h1, li, text, ul}

fn stockist(name: String, link: String) {
  li([class("italic hover:underline")], [a([href(link)], [text(name)])])
}

pub fn pocari() {
  let title =
    "A (not yet) comprehensive blog on where to find pocari sweat in london"
  layout(title, [
    h1([class("text-xl font-bold")], [text(title)]),
    br([]),
    paragraph(
      "While ubiquitous across Asia, Pocari Sweat isn't as easy to find in London (where I happen to live). But that doesn't mean it can't be found.",
    ),
    br([]),
    paragraph("Below is a list of all the stockists that I have found so far."),
    br([]),
    paragraph(
      "I'll endeavor to update as I find more, but expect this list to be heavily weighted towards Hackney and Bloomsbury, given that's where I spend most of my time.",
    ),
    br([]),
    ul([class("list-disc list-inside")], [
      stockist("Oseyo (Waterloo)", "https://g.co/kgs/B2RCbGm"),
      stockist("Oseyo (Tottenham Court Road)", "https://g.co/kgs/obkpPZe"),
      stockist(
        "Centre Point Food Store (Tottenham Court Road)",
        "https://g.co/kgs/ijMbsBg",
      ),
      stockist("Longdan (Shoreditch)", "https://g.co/kgs/x928kw1"),
      stockist("Japan Centre (Leicester Square)", "https://g.co/kgs/TCUZprx"),
    ]),
  ])
}
