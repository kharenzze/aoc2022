use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = usize;

fn read_data() -> Input {
  let filename = format!("./resources/8.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  unimplemented!()
}

fn solve(input: Input) -> usize {
  unimplemented!()
}

pub fn seven() {
  let input = read_data();
  let score = solve(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {

  #[test]
  fn simple() {
    unimplemented!()
  }
}
