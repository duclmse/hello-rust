use actix_web::web;

pub(crate) mod api;

pub(crate) fn route() -> actix_web::Scope {
  web::scope("/fs")
    .service(actix_files::Files::new("/static", "/").show_files_listing())
    .service(api::index_files)
    .service(api::link)
}
