#[cfg(test)]
mod test {
  use glob::glob;
  use lnk::Lnk;

  #[test]
  fn test_win7() {
    for entry in glob("samples\\WIN7\\*\\*.lnk").expect("Failed to read glob pattern") {
      match entry {
        Ok(path) => {
          let full_path = path.to_str().unwrap().to_string();
          println!("{}", full_path);
          println!("{:?}", Lnk::from_path(&full_path).unwrap());
        },
        Err(e) => eprintln!("{:?}", e),
      }
    }
  }

  #[test]
  fn test_win10() {
    for entry in glob("samples\\WIN10\\*\\*.lnk").expect("Failed to read glob pattern") {
      match entry {
        Ok(path) => {
          let full_path = path.to_str().unwrap().to_string();
          println!("{}", full_path);
          println!("{:?}", Lnk::from_path(&full_path).unwrap());
        },
        Err(e) => {
          eprintln!("{:?}", e);
        },
      }
    }
  }

  #[cfg(test)]
  #[test]
  fn test_ws12r2() {
    for entry in glob("samples\\WS12R2\\*\\*.lnk").expect("Failed to read glob pattern") {
      match entry {
        Ok(path) => {
          let full_path = path.to_str().unwrap().to_string();
          println!("{}", full_path);
          println!("{:?}", Lnk::from_path(&full_path).unwrap());
        },
        Err(e) => {
          eprintln!("{:?}", e);
        },
      }
    }
  }
}
