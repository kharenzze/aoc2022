use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

fn read_data() -> Input {
  let filename = format!("./resources/7.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
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
    assert!(true);
  }
}
