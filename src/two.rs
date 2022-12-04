use std::fs::File;
use std::io::{prelude::*, BufReader};

enum Choice {
  Rock,
  Paper,
  Scissor,
}

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

struct Move(Choice, Strategy);

impl Move {
  fn from_line(l: &str) -> Self {
    let chars: Vec<&str> = l.split(" ").collect();
    let choice = Choice::from(chars[0]);
    let strategy = Strategy::from(chars[1]);
    Move(choice, strategy)
  }
}

fn read_data() -> Vec<Move> {
  let filename = format!("./resources/2.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let mut line_iter = reader.lines();
  line_iter
    .map(|l| l.unwrap())
    .map(|l| Move::from_line(&l))
    .collect()
}

fn eval(game: Vec<Move>, truthy: bool) -> usize {
  0
}

pub fn two() {
  let game = read_data();
}
