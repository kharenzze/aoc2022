use std::cmp::{Eq, PartialEq};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Choice {
  Rock,
  Paper,
  Scissor,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Strategy {
  X,
  Y,
  Z,
}

impl From<&str> for Choice {
  fn from(i: &str) -> Self {
    match i {
      "A" => Choice::Rock,
      "B" => Choice::Paper,
      "C" => Choice::Scissor,
      _ => unreachable!(),
    }
  }
}

impl From<usize> for Choice {
  fn from(i: usize) -> Self {
    match i % 3 {
      0 => Choice::Rock,
      1 => Choice::Paper,
      2 => Choice::Scissor,
      _ => unreachable!(),
    }
  }
}

impl Into<usize> for Choice {
  fn into(self) -> usize {
    match self {
      Choice::Rock => 0,
      Choice::Paper => 1,
      Choice::Scissor => 2,
    }
  }
}

impl From<&str> for Strategy {
  fn from(i: &str) -> Self {
    match i {
      "X" => Strategy::X,
      "Y" => Strategy::Y,
      "Z" => Strategy::Z,
      _ => unreachable!(),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct PlannedMove(Choice, Strategy);

impl PlannedMove {
  fn from_line(l: &str) -> Self {
    let chars: Vec<&str> = l.split(" ").collect();
    let choice = Choice::from(chars[0]);
    let strategy = Strategy::from(chars[1]);
    Self(choice, strategy)
  }

  fn into_move(&self) -> Move {
    let my_choice = match self.1 {
      Strategy::X => Choice::Rock,
      Strategy::Y => Choice::Paper,
      Strategy::Z => Choice::Scissor,
    };
    Move(self.0, my_choice)
  }

  fn into_move_extra(&self) -> Move {
    let my_choice = match self.1 {
      Strategy::X => {
        let x: usize = self.0.into();
        Choice::from(x + 2)
      }
      Strategy::Y => self.0.clone(),
      Strategy::Z => {
        let x: usize = self.0.into();
        Choice::from(x + 1)
      }
    };
    Move(self.0, my_choice)
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Move(Choice, Choice);

const LOST: usize = 0;
const DRAW: usize = 3;
const WON: usize = 6;

impl Move {
  fn score(&self) -> usize {
    self.score_by_shape() + self.score_by_outcome()
  }

  fn score_by_shape(&self) -> usize {
    match self.1 {
      Choice::Rock => 1,
      Choice::Scissor => 3,
      Choice::Paper => 2,
    }
  }

  fn score_by_outcome(&self) -> usize {
    if self.0 == self.1 {
      return DRAW;
    }
    if self.0 == Choice::Rock && self.1 == Choice::Scissor {
      return LOST;
    }
    if self.0 == Choice::Scissor && self.1 == Choice::Paper {
      return LOST;
    }
    if self.0 == Choice::Paper && self.1 == Choice::Rock {
      return LOST;
    }
    WON
  }
}

fn read_data() -> Vec<PlannedMove> {
  let filename = format!("./resources/2.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter
    .map(|l| l.unwrap())
    .map(|l| PlannedMove::from_line(&l))
    .collect()
}

fn eval(game: &Vec<PlannedMove>) -> usize {
  game
    .iter()
    .map(|m| m.into_move_extra())
    .map(|m| m.score())
    .sum()
}

pub fn two() {
  let game = read_data();
  let score = eval(&game);
  println!("{score}");
}
