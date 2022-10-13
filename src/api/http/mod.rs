//module http

use actix_web::{
  get,
  web::{self, Data},
};

pub(crate) mod fs;
pub(crate) mod model;
pub(crate) mod test;

#[get("/")]
async fn index(data: Data<model::AppState>, app_counter: Data<model::AppStateWithCounter>) -> String {
  let mut counter = app_counter.counter.lock().unwrap();
  *counter += 1;
  let app_name = &data.app_name;
  format!("Hello {app_name}! ({counter} times)")
}

pub(crate) fn route() -> actix_web::Scope {
  web::scope("/").service(index)
}
