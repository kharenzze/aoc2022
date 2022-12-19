use std::collections::HashMap;
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

  fn from_str(s: &str) -> Point {
    let mut chars = s.split(",");
    let x: isize = chars.next().unwrap().parse().unwrap();
    let y: isize = chars.next().unwrap().parse().unwrap();
    Point::new(x, y)
  }

  fn to_unit(&self) -> Self {
    Self {
      x: self.x.signum(),
      y: self.y.signum(),
    }
  }
}

type Path = Vec<Point>;

fn line_to_path(line: &str) -> Path {
  line
    .split("->")
    .map(|chunk| {
      let raw_point = chunk.trim();
      Point::from_str(raw_point)
    })
    .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
  Empty,
  Sand,
  Block,
}

struct Game {
  max: isize,
  map: HashMap<Point, Cell>,
}

impl Game {
  fn new(blocks: Vec<Path>) -> Self {
    let mut map = HashMap::new();
    let max: isize = blocks
      .iter()
      .map(|list| list.iter().map(|p| p.y).max().unwrap())
      .max()
      .unwrap();

    for block in blocks {
      for window in block.windows(2) {
        let a = window[0];
        let b = window[1];
        let dir = (b - a).to_unit();
        let mut point = a.clone();
        loop {
          map.insert(point, Cell::Block);
          if point == b {
            break;
          }
          point = point + dir;
        }
      }
    }
    Game { map, max }
  }

  fn play(&mut self) -> usize {
    let mut dropped_sand: usize = 0;
    const ORIGIN: Point = Point::new(500, 0);
    'main: loop {
      let mut current = ORIGIN;
      //iterations for a sand unit
      'sand: loop {
        if (current.y + 1) > self.max {
          break 'main;
        }

        let possible_steps = Game::possible_steps(current);
        for step in possible_steps {
          let content = self.map.get(&step).unwrap_or(&Cell::Empty);
          match *content {
            Cell::Empty => {
              current = step;
              continue 'sand;
            }
            _ => (),
          }
        }
        //no empty cell found
        //fill the current cell
        self.map.insert(current, Cell::Sand);
        break;
      }
      dropped_sand += 1;
    }
    dropped_sand
  }

  fn play_v2(&mut self) -> usize {
    let mut dropped_sand: usize = 0;
    const ORIGIN: Point = Point::new(500, 0);
    'main: loop {
      let mut current = ORIGIN;
      //iterations for a sand unit
      'sand: loop {
        let possible_steps = Game::possible_steps(current);
        for step in possible_steps {
          let content = self.map.get(&step).unwrap_or_else(|| {
            if step.y == (self.max + 2) {
              &Cell::Block
            } else {
              &Cell::Empty
            }
          });
          match *content {
            Cell::Empty => {
              current = step;
              continue 'sand;
            }
            _ => (),
          }
        }
        //no empty cell found

        //fill the current cell
        self.map.insert(current, Cell::Sand);
        break;
      }
      dropped_sand += 1;
      if current == ORIGIN {
        break;
      }
    }
    dropped_sand
  }

  fn possible_steps(prev: Point) -> [Point; 3] {
    const STEPS: [Point; 3] = [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)];
    STEPS.map(|step| prev + step)
  }
}

fn read_data() -> Input {
  let filename = format!("./resources/14.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn solve(input: Input) -> usize {
  let paths: Vec<Path> = input
    .into_iter()
    .map(|line| line_to_path(line.as_str()))
    .collect();
  let mut game = Game::new(paths);
  let score = game.play();
  score
}

fn solve_v2(input: Input) -> usize {
  let paths: Vec<Path> = input
    .into_iter()
    .map(|line| line_to_path(line.as_str()))
    .collect();
  let mut game = Game::new(paths);
  let score = game.play_v2();
  score
}

pub fn fourteen() {
  let input = read_data();
  let score = solve_v2(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use indoc::indoc;

  #[test]
  fn simple() {
    let input: super::Input = SAMPLE.lines().map(|l| l.to_string()).collect();
    let score = super::solve(input);
    assert_eq!(score, 24);
  }

  #[test]
  fn v2() {
    let input: super::Input = SAMPLE.lines().map(|l| l.to_string()).collect();
    let score = super::solve_v2(input);
    assert_eq!(score, 93);
  }

  const SAMPLE: &'static str = indoc! {r"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"};
}
