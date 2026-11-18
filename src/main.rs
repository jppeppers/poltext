use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::iter::Iterator;
use std::path::Path;

fn main() -> std::io::Result<()> {
  // println!("{:?}", env::args().nth(0));

  for var in env::vars_os().enumerate() {
    println!("{:?}", var)
  }
  return Ok(());

  if env::args().count() < 3 || 
    env::args().nth(1) != Some(String::from("ptxt")) ||
    env::args().nth(2).is_none()
  {
    println!("Incorrect usage. Correct usage is: \"ptxt [FILEPATH]\"");
    return Err(
      Error::new(
        ErrorKind::Other, 
        "Incorrect usage."
      )
    )
  }

  let filepath_str: String = String::from(env::args().nth(2).unwrap());
  let filepath: &Path      = Path::new(&filepath_str);

  if !fs::exists(filepath).unwrap() {
    println!("Unable to find file: {}", &filepath_str);

    return Err(
      Error::new(
        ErrorKind::NotFound, 
        format!("File not found: {filepath_str}")
      )
    )
  }

  let file = File::open(filepath)?;
  let reader = BufReader::new(file);

  let lines = reader.lines().map( |l| l.unwrap() );

  for line in lines.enumerate() {
    println!("{} | {}", line.0, line.1)
  }

  Ok(())
}
