use actix_web::{get, web::Path, Responder};

#[get("/{id}/{name}")]
pub(crate) async fn test(path: Path<(u32, String)>) -> impl Responder {
  let (id, name) = path.into_inner();
  format!("Hello {}! id:{}", name, id)
}
