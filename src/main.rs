use std::io::{self, Read};

fn main() {
  let input = read_input().unwrap();
  println!("{}", input);
}

fn read_input() -> io::Result<String> {
  let mut buffer = String::new();
  let stdin = io::stdin();
  let mut handle = stdin.lock();

  handle.lines()
  handle.read_to_string(&mut buffer)?;
  Ok(buffer)
}