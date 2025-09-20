import blogs/generic_photos
import blogs/pocari
import gleam/bytes_tree
import gleam/http/request.{type Request}
import gleam/http/response
import gleam/option.{None}
import gleam/string
import lustre/element
import mist.{type Connection, type ResponseData}
import pages

pub const favicon = "favicon.ico"

fn body_from_element(element: fn() -> element.Element(t)) -> ResponseData {
  mist.Bytes(bytes_tree.from_string_tree(
    element() |> element.to_document_string_tree,
  ))
}

fn not_found() {
  response.new(404)
  |> response.set_body(mist.Bytes(bytes_tree.new()))
}

fn photos() {
  not_found()
}

fn serve_file(priv: String, file_path: List(String)) {
  let file_path = string.join([priv, ..file_path], "/")
  case mist.send_file(file_path, offset: 0, limit: None) {
    Ok(file) -> response.new(200) |> response.set_body(file)
    Error(_) -> not_found()
  }
}

fn serve_image(priv: String, file_name: String) {
  let file_path = string.join([priv, ..[file_name]], "/")
  case mist.send_file(file_path, 0, None) {
    Ok(file) -> response.new(200) |> response.set_body(file)
    Error(_) -> not_found()
  }
}

fn index() {
  response.new(200)
  |> response.set_body(body_from_element(pages.index))
}

fn blog_post(blog_name) {
  case blog_name {
    "pocari" ->
      response.new(200) |> response.set_body(body_from_element(pocari.pocari))
    "2025_photos" ->
      response.new(200)
      |> response.set_body(body_from_element(generic_photos.generic_photos))
    _ -> not_found()
  }
}

fn blog_index() {
  response.new(200)
  |> response.set_body(body_from_element(pages.blog_index))
}

pub fn route(req: Request(Connection), priv: String) {
  case request.path_segments(req) {
    [] -> index()
    ["blog"] -> blog_index()
    ["blog", blog_name] -> blog_post(blog_name)
    ["photos"] -> photos()
    ["priv", ..file_path] -> serve_file(priv, file_path)
    ["image", file_name] -> serve_image(priv, file_name)
    [favicon] -> serve_file(priv, [favicon])
    _ -> not_found()
  }
}
