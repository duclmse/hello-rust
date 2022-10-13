use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, HttpRequest, Result, Responder};

/// GET /fs/{filename:.*}
#[get("/fs/{filename:.*}")]
pub(crate) async fn index_files(req: HttpRequest) -> Result<NamedFile> {
  let path: PathBuf = req.match_info().query("filename").parse().unwrap();
  let mut root = PathBuf::from("/");
  root.push(path);
  println!("path={}", root.display());
  Ok(NamedFile::open(root)?)
}

/// GET /fs/lnk
#[get("/lnk")]
pub(crate) async fn link() -> impl Responder {
  let path = "/home/duclm/Downloads/@Animation.lnk";
  let file = lnk::Lnk::from_path(path).unwrap();
  file.target_full_path.unwrap()
}
