//module http

use actix_web::{get, web::Data};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};


pub(crate) mod fs;
pub(crate) mod model;
pub(crate) mod test;

#[allow(dead_code)]
pub(crate) fn cert_builder(key: &str, cert: &str) -> SslAcceptorBuilder {
  let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
  builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
  builder.set_certificate_chain_file(cert).unwrap();
  builder
}

#[get("/")]
async fn index(data: Data<model::AppState>, app_counter: Data<model::AppStateWithCounter>) -> String {
  let mut counter = app_counter.counter.lock().unwrap();
  *counter += 1;
  let app_name = &data.app_name;
  format!("Hello {app_name}! ({counter} times)")
}
