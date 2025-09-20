import layout.{layout}
import lustre/element/html.{h1, text}

pub fn generic_photos() {
  layout("Photos from 2025", [h1([], [text("test!")])])
}
