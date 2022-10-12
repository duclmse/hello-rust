use actix_web::web;

mod api;

pub(crate) fn route() -> actix_web::Scope {
  web::scope("/test")
    // .service(web::resource("/{id}/{name}").route(web::get().to(test)))
    .service(api::test)
}
