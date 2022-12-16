use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
  min: usize,
  max: usize,
}

impl Range {
  fn new(min: usize, max: usize) -> Range {
    Range { min, max }
  }

  fn from_str(s: &str) -> Self {
    let mut parts = s.split('-');
    let min = parts.next().unwrap().parse().unwrap();
    let max = parts.next().unwrap().parse().unwrap();
    Range { min, max }
  }

  fn contains(&self, other: &Self) -> bool {
    other.min >= self.min && other.max <= self.max
  }

  fn overlaps(&self, other: &Self) -> bool {
    other.min <= self.max && other.max >= self.min
  }
}

fn read_data() -> Input {
  let filename = format!("./resources/4.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn solve(input: Input) -> usize {
  let pairs: Vec<(Range, Range)> = input
    .iter()
    .map(|line| {
      let mut parts = line.split(',');
      let first = Range::from_str(parts.next().unwrap());
      let second = Range::from_str(parts.next().unwrap());
      (first, second)
    })
    .collect();
  let mut score: usize = 0;
  for (a, b) in pairs.iter() {
    if a.contains(b) || b.contains(a) {
      score += 1;
    }
  }
  score
}

fn solve_v2(input: Input) -> usize {
  let pairs: Vec<(Range, Range)> = input
    .iter()
    .map(|line| {
      let mut parts = line.split(',');
      let first = Range::from_str(parts.next().unwrap());
      let second = Range::from_str(parts.next().unwrap());
      (first, second)
    })
    .collect();
  let mut score: usize = 0;
  for (a, b) in pairs.iter() {
    if a.overlaps(b) {
      score += 1;
    }
  }
  score
}

pub fn four() {
  let input = read_data();
  let score = solve_v2(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {

  #[test]
  fn simple() {
    let input: Vec<String> = SAMPLE.lines().map(|l| l.to_string()).collect();
    let score = super::solve(input);
    assert_eq!(score, 2);
  }

  #[test]
  fn v2() {
    let input: Vec<String> = SAMPLE.lines().map(|l| l.to_string()).collect();
    let score = super::solve_v2(input);
    assert_eq!(score, 4);
  }

  const SAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
}
