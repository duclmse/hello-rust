use std::{
  fs::{self, File, OpenOptions},
  io,
  io::prelude::*,
  os::unix,
  path::Path,
};

fn main() {
  let path = "/home/duclm/Downloads/@Animation.lnk";
  let file = File::open(path).unwrap();
  let lnk_file = lnk::Lnk::from_path(path).unwrap();
  println!("{:#?}", lnk_file);
  let metadata = file.metadata().unwrap();
  println!("{:#?}", metadata)
  // fs()
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
  name: String,
  age: u8,
  phones: Vec<String>,
}

#[test]
fn typed_example() -> Result<()> {
  // Some JSON input data as a &str. Maybe this comes from the user.
  let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

  // Parse the string of data into a Person object. This is exactly the same function as the one that produced 
  // serde_json::Value above, but now we are asking it for a Person as output.
  let p = serde_json::from_str::<Person>(data)?;

  // Do things just like with any other Rust data structure.
  println!("Please call {} (age {}) at the number {}", p.name, p.age, p.phones[0]);

  Ok(())
}

fn fs() {
  println!("mkdir a");
  // Create a directory, returns io::Result<()>
  match fs::create_dir("a") {
    Err(why) => println!("! {:?}", why.kind()),
    Ok(_) => {},
  }

  println!("$ echo hello > a/b.txt");
  // The previous match can be simplified using the unwrap_or_else method
  echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });

  println!("$ mkdir -p a/c/d");
  // Recursively create a directory, returns io::Result<()>
  fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });

  println!("$ touch a/c/e.txt");
  touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });

  println!("$ ln -s ../b.txt a/c/b.txt");
  // Create a symbolic link, returns io::Result<()>
  if cfg!(target_family = "unix") {
    unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
      println!("! {:?}", why.kind());
    });
  } else {
    println!("cannot create symlink")
  }

  println!("$ cat a/c/b.txt");
  match cat(&Path::new("a/c/b.txt")) {
    Err(why) => println!("! {:?}", why.kind()),
    Ok(s) => println!("> {}", s),
  }

  println!("$ ls a");
  // Read the contents of a directory, returns io::Result<Vec<Path>>
  match fs::read_dir("a") {
    Err(why) => println!("! {:?}", why.kind()),
    Ok(paths) => paths.for_each(|path| {
      println!("> {:#?}", path.unwrap().path());
    }),
  }

  println!("$ rm a/c/e.txt");
  // Remove a file, returns io::Result<()>
  fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });

  println!("$ rmdir a");
  // Remove an empty directory, returns io::Result<()>
  fs::remove_dir_all("a").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
}

// A simple implementation of % cat path
fn cat(path: &Path) -> io::Result<String> {
  let mut f = File::open(path)?;
  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}

// A simple implementation of % echo s > path
fn echo(s: &str, path: &Path) -> io::Result<()> {
  let mut f = File::create(path)?;

  f.write_all(s.as_bytes())
}

// A simple implementation of % touch path (ignores existing files)
fn touch(path: &Path) -> io::Result<()> {
  match OpenOptions::new().create(true).write(true).open(path) {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}
