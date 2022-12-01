use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Invalid number of arguments")
  }
  let day: &str = args.get(1).unwrap();
  let filename = format!("./resources/{day}.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let mut line_iter = reader.lines();

  let mut max = 0;
  let mut current = 0;

  while let Some(Ok(line)) = line_iter.next() {
    if line.len() == 0 {
      current = 0;
      continue;
    }
    let value: usize = line.parse().unwrap();
    current += value;
    if current >= max {
      max = current;
    }
  }
  println!("{max}");
}
