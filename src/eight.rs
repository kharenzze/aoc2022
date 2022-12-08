use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<Vec<char>>;

fn read_data() -> Input {
  let filename = format!("./resources/8.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter
    .map(|l| l.unwrap().chars().collect::<Vec<char>>())
    .collect()
}

fn solve(input: Input) -> usize {
  unimplemented!()
}

pub fn eight() {
  let input = read_data();
  let score = solve(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn simple() {
    let sample = vec!["30373", "25512", "65332", "33549", "35390"];
    let sample = sample
      .iter()
      .map(|s| s.chars().collect::<Vec<char>>())
      .collect();
    assert_eq!(solve(sample), 21);
  }
}
