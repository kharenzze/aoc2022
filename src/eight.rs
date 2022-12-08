use crate::point::{Direction, Point, DIRECTIONS};
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<Vec<usize>>;
const RADIX: u32 = 10;

fn char_to_usize(c: char) -> usize {
  c.to_digit(RADIX).unwrap() as usize
}

fn read_data() -> Input {
  let filename = format!("./resources/8.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter
    .map(|l| {
      l.unwrap()
        .chars()
        .map(char_to_usize)
        .collect::<Vec<usize>>()
    })
    .collect()
}

fn solve_v2(input: &Input) -> usize {
  let w = input[0].len();
  let h = input.len();
  let dimensions = Point::new(h, w);
  let mut best = 0_usize;
  for i in 1..h {
    for j in 1..w {
      let pos = Point::new(i, j);
      let value = input[i][j];
      let mut view: Vec<usize> = vec![];
      for dir in DIRECTIONS {
        let mut count: usize = 0;
        let mut current_pos = pos.clone();
        while let Some(p) = current_pos.get_next(*dir, &dimensions) {
          current_pos = p.clone();
          count += 1;
          let current_value = input[p.x][p.y];
          if value <= current_value {
            break;
          }
        }
        view.push(count);
      }

      let score: usize = view.iter().product();
      if score > best {
        best = score;
      }
    }
  }
  best
}

fn solve(input: &Input) -> usize {
  let mut visible: HashSet<Point> = HashSet::new();
  let w = input[0].len();
  let h = input.len();

  for i in 0..h {
    let mut min: usize = 0;
    for j in 0..w {
      let current = input[i][j];
      let p = Point::new(i, j);
      if j == 0 {
        visible.insert(p);
        min = current;
        continue;
      }
      if current > min {
        min = current;
        visible.insert(p);
      }
      if min == 9 {
        break;
      }
    }

    for j in (0..w).rev() {
      let current = input[i][j];
      let p = Point::new(i, j);
      if j == (w - 1) {
        visible.insert(p);
        min = current;
        continue;
      }
      if current > min {
        min = current;
        visible.insert(p);
      }
      if min == 9 {
        break;
      }
    }
  }

  for j in 0..w {
    let mut min: usize = 0;
    for i in 0..h {
      let current = input[i][j];
      let p = Point::new(i, j);
      if i == 0 {
        visible.insert(p);
        min = current;
        continue;
      }
      if current > min {
        min = current;
        visible.insert(p);
      }
      if min == 9 {
        break;
      }
    }

    for i in (0..h).rev() {
      let current = input[i][j];
      let p = Point::new(i, j);
      if i == (w - 1) {
        visible.insert(p);
        min = current;
        continue;
      }
      if current > min {
        min = current;
        visible.insert(p);
      }
      if min == 9 {
        break;
      }
    }
  }

  visible.len()
}

pub fn eight() {
  let input = read_data();
  let score = solve_v2(&input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::{char_to_usize, solve, solve_v2};

  #[test]
  fn simple() {
    let sample = vec!["30373", "25512", "65332", "33549", "35390"];
    let sample = sample
      .iter()
      .map(|s| s.chars().map(char_to_usize).collect::<Vec<usize>>())
      .collect();
    assert_eq!(solve(&sample), 21);
    assert_eq!(solve_v2(&sample), 8);
  }
}
