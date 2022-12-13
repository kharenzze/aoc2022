use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

fn read_data() -> Input {
  let filename = format!("./resources/11.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

#[derive(Debug, Clone, Copy)]
enum OpValue {
  Old,
  number(isize),
}

#[derive(Debug, Clone, Copy)]
enum Operator {
  Sum,
  Mult,
}

#[derive(Debug, Clone, Copy)]
struct Operation {
  a: OpValue,
  b: OpValue,
  operator: Operator,
}

struct Monkey {
  id: usize,
  items: Vec<isize>,
  operation: Operation,
  test: isize,
}

fn solve(input: Input) -> usize {
  let filtered: Input = input.into_iter().filter(|s| s.is_empty()).collect();
  let grouped: Vec<Input> = filtered.chunks(6).map(|c| c.to_owned()).collect();

  1
}

pub fn eleven() {
  let input = read_data();
  let score = solve(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {

  #[test]
  fn simple() {
    assert!(true)
  }
}
