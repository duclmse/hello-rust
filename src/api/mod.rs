// module api

pub(crate) mod http;
pub(crate) mod middleware;
pub(crate) mod websocket;

use actix_web::{
  web::{self, Data},
  App, HttpServer,
};
use std::{io::Result, sync::Mutex};

pub async fn serve(addr: &str) -> Result<()> {
  let counter = Data::new(http::model::AppStateWithCounter { counter: Mutex::new(0) });
  let app_name = Data::new(http::model::AppState {
    app_name: String::from("Actix Web"),
  });

  #[rustfmt::skip]
  HttpServer::new(move || {
    App::new()
      .app_data(app_name.clone())
      .app_data(counter.clone())
      .service(http::route())
      .service(http::fs::route())
      .service(http::test::route())
      .route("/ws", web::get().to(websocket::index))
  })
  // .bind_openssl(addr, builder)?
  .bind(addr)?
  .run()
  .await?;
  Ok(())
}
