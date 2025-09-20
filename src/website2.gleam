import gleam/erlang/application
import gleam/erlang/process
import gleam/http/request.{type Request}
import gleam/http/response.{type Response}
import gleam/io
import gleam/string
import logging
import mist.{type Connection, type ResponseData}
import router

pub fn main() {
  logging.configure()
  logging.set_level(logging.Debug)
  let assert Ok(priv) = application.priv_directory("website2")
  io.println(priv)

  let assert Ok(_) =
    fn(req: Request(Connection)) -> Response(ResponseData) {
      logging.log(
        logging.Info,
        "Got a request from: " <> string.inspect(mist.get_client_info(req.body)),
      )
      router.route(req, priv)
    }
    |> mist.new
    |> mist.bind("localhost")
    |> mist.port(3000)
    |> mist.start

  process.sleep_forever()
}
