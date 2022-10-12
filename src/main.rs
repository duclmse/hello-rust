use std::io::Result;

mod api;
mod service;

#[actix_web::main]
async fn main() -> Result<()> {
  api::serve("0.0.0.0:8888").await
}
