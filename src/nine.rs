use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

impl Direction {
  fn from_str(s: &str) -> Self {
    match s {
      "R" => Direction::Right,
      "D" => Direction::Down,
      "U" => Direction::Up,
      "L" => Direction::Left,
      _ => unreachable!(),
    }
  }
}

type Input = Vec<String>;

fn read_data() -> Input {
  let filename = format!("./resources/9.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

struct Instruction {
  dir: Direction,
  count: usize,
}

impl Instruction {
  fn from_line(l: &String) -> Self {
    let chunks: Vec<&str> = l.split(" ").collect();
    let dir = Direction::from_str(chunks[0]);
    let count: usize = chunks[1].parse().unwrap();
    Self { dir, count }
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
  pub x: isize,
  pub y: isize,
}

impl Add<Point> for Point {
  type Output = Point;

  fn add(self, rhs: Point) -> Self::Output {
    Point::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl Sub<Point> for Point {
  type Output = Point;

  fn sub(self, rhs: Point) -> Self::Output {
    Point::new(self.x - rhs.x, self.y - rhs.y)
  }
}

fn in_range(x: isize, min: isize, max: isize) -> bool {
  x >= min && x <= max
}

impl Point {
  pub const fn new(x: isize, y: isize) -> Self {
    Self { x, y }
  }

  fn is_adjacent(&self, other: &Point) -> bool {
    let diff_x: isize = self.x - other.x;
    let diff_y: isize = self.y - other.y;
    in_range(diff_x, -1, 1) && in_range(diff_y, -1, 1)
  }

  fn move_to(&mut self, d: Direction) {
    match d {
      Direction::Up => *self = *self + X,
      Direction::Right => *self = *self + Y,
      Direction::Down => *self = *self - X,
      Direction::Left => *self = *self - Y,
    }
  }
}

const X: Point = Point::new(1, 0);
const Y: Point = Point::new(0, 1);
const ZERO: Point = Point::new(0, 0);

const SNAKE_LEN: usize = 10;
const LAST: usize = SNAKE_LEN - 1;

fn solve(input: Input) -> usize {
  let snake = &mut [ZERO; SNAKE_LEN];
  let mut visited: HashSet<Point> = Default::default();
  visited.insert(ZERO);
  let iter = input.iter().map(Instruction::from_line);
  for instruction in iter {
    for _ in 0..instruction.count {
      {
        let head = &mut snake[0];
        head.move_to(instruction.dir);
      }
      for i in 1..SNAKE_LEN {
        let head = snake[i - 1];
        let tail = &mut snake[i];
        if head.is_adjacent(tail) {
          continue;
        }
        let mut diff = head - *tail;
        diff.x = diff.x.clamp(-1, 1);
        diff.y = diff.y.clamp(-1, 1);
        *tail = *tail + diff;
        if i == LAST {
          visited.insert(*tail);
        }
      }
    }
  }

  visited.iter().count()
}

pub fn nine() {
  let input = read_data();
  let score = solve(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {

  #[test]
  fn simple() {
    let input_string = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;
    let input: Vec<String> = input_string.lines().map(|l| l.to_owned()).collect();
    let score = super::solve(input);
    assert_eq!(score, 1);
  }
}
