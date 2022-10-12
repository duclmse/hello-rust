// module webservice

use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

/// Define HTTP actor
struct TestWs;

impl Actor for TestWs {
  type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TestWs {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => {
        println!("ping -> pong");
        ctx.pong(&msg)
      },
      Ok(ws::Message::Text(text)) => {
        println!("Received text: '{}'", text);
        ctx.text(text)
      },
      Ok(ws::Message::Binary(bin)) => {
        println!("Received binary: '{}'", bin.escape_ascii().to_string());
        ctx.binary(bin)
      },
      _ => (),
    }
  }
}

pub(crate) async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  let resp = ws::start(TestWs {}, &req, stream);
  println!("{:?}", resp);
  resp
}
