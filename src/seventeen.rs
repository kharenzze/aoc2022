use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, Sub};

type Input = Vec<String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Point {
  x: isize,
  y: isize,
}

impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Sub for Point {
  type Output = Point;

  fn sub(self, other: Point) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl Point {
  const fn new(x: isize, y: isize) -> Point {
    Point { x, y }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GasDir {
  Left,
  Right,
}

impl GasDir {
  fn from_char(c: char) -> GasDir {
    match c {
      '<' => GasDir::Left,
      '>' => GasDir::Right,
      _ => unreachable!(),
    }
  }
}

fn read_data() -> Input {
  let filename = format!("./resources/17.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn solve(input: Input) -> usize {
  let pattern: Vec<GasDir> = input
    .first()
    .unwrap()
    .chars()
    .map(GasDir::from_char)
    .collect();
  let mut pattern_iter = pattern.iter().cycle();
  unimplemented!()
}

pub fn seventeen() {
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
