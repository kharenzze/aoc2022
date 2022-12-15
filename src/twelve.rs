use crate::point::{Point, DIRECTIONS};
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MapCell {
  Start,
  End,
  Empty(usize),
}

impl MapCell {
  fn from_char(c: char) -> Self {
    match c {
      'S' => Self::Start,
      'E' => Self::End,
      n => Self::Empty(n as usize),
      _ => unreachable!(),
    }
  }

  fn can_proceed_to(&self, other: &Self) -> bool {
    match other {
      Self::End => *self == Self::from_char('z'),
      Self::Start => false,
      Self::Empty(n) => match self {
        Self::Start => true,
        Self::Empty(m) => *n <= (*m + 1),
        _ => false,
      },
    }
  }
}

fn read_data() -> Input {
  let filename = format!("./resources/12.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

const ZERO: Point = Point::new(0, 0);

fn solve(input: Input) -> usize {
  let mut visited: HashSet<Point> = HashSet::new();
  let grid: Vec<Vec<MapCell>> = input
    .iter()
    .map(|l| l.chars().map(|c| MapCell::from_char(c)).collect())
    .collect();
  let H: usize = grid.len();
  let W: usize = grid[0].len();
  let dimensions = Point::new(H, W);
  let mut starting_point: Point = Point::new(0, 0);
  for (x, row) in grid.iter().enumerate() {
    for (y, cell) in row.iter().enumerate() {
      if let MapCell::Start = cell {
        starting_point = Point::new(x, y);
        break;
      }
    }
    if starting_point != ZERO {
      break;
    }
  }

  let mut open: HashSet<Point> = HashSet::new();
  open.insert(starting_point);
  let mut score: usize = 0;

  loop {
    score += 1;
    let mut new_open: HashSet<Point> = HashSet::new();
    for point in open.iter() {
      visited.insert(*point);
      let old = &grid[point.x][point.y];
      for new_point in point.get_points_around_inside_container(&dimensions) {
        if visited.contains(&new_point) {
          continue;
        }
        let cell: &MapCell = &grid[new_point.x][new_point.y];
        match cell {
          MapCell::End => {
            if old.can_proceed_to(cell) {
              return score;
            }
          }
          MapCell::Empty(_) => {
            if old.can_proceed_to(cell) {
              new_open.insert(new_point);
            }
          }
          _ => (),
        }
      }
    }
    open = new_open;
  }
}

pub fn twelve() {
  let input = read_data();
  let score = solve(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use crate::twelve::Input;

  #[test]
  fn simple() {
    let input: Input = INPUT.lines().map(|l| l.to_string()).collect();
    let score = super::solve(input);
    assert_eq!(score, 31);
  }

  const INPUT: &'static str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
}
