use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::iter::Iterator;
use std::path::Path;

use libc::{ECHO, STDIN_FILENO, TCSAFLUSH, termios, tcgetattr, tcsetattr};

fn main() -> std::io::Result<()> {
  // let mut orig_termios = enable_raw_mode();

  let mut c:char;

  while io::stdin().read(&mut c).unwrap() == 1 && c != 'q' {};

  println!("{:?}", c);

  // disable_raw_mode(&mut orig_termios);

  return Ok(());

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

fn enable_raw_mode() -> termios {
  unsafe {
    let mut orig_termios = termios {
      c_iflag: 0,
      c_oflag: 0,
      c_cflag: 0,
      c_lflag: 0,
      c_line: 0,
      c_cc: [0; 32],
      c_ispeed: 0,
      c_ospeed: 0
    };

    tcgetattr(STDIN_FILENO, &mut orig_termios);

    let mut term = orig_termios.clone();

    term.c_lflag &= !(ECHO);

    tcsetattr(STDIN_FILENO, TCSAFLUSH, &term);

    return orig_termios
  }
}

fn disable_raw_mode(term: &mut termios) {
  unsafe {
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &*term);

    println!("{:?}", term)
  }
}
