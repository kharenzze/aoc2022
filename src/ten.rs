use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

fn read_data() -> Input {
  let filename = format!("./resources/10.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

#[derive(Debug, Clone, Copy)]
enum Operation {
  Noop,
  Add(isize),
}

impl Operation {
  fn from_line(l: &String) -> Self {
    let chunks: Vec<&str> = l.split(" ").collect();
    match chunks[0] {
      "noop" => Operation::Noop,
      "addx" => {
        let i: isize = chunks[1].parse().unwrap();
        Operation::Add(i)
      }
      _ => unreachable!(),
    }
  }

  fn cycles(&self) -> usize {
    match self {
      Self::Noop => 1,
      Self::Add(_) => 2,
    }
  }
}

const FIRST: isize = 20;
fn is_point_of_interest(c: isize) -> bool {
  let v = c - FIRST;
  v >= 0 && v % 40 == 0
}

fn solve(input: Input) -> isize {
  let mut cycle: isize = 1;
  let mut pending_cycles: usize = 0;
  let mut poi_list: Vec<isize> = vec![];
  let mut x: isize = 1;
  let mut current_op: Option<&Operation> = Some(&Operation::Noop);
  let operations: Vec<Operation> = input.iter().map(Operation::from_line).collect();
  let mut operations_iter = operations.iter();
  loop {
    if pending_cycles == 0 {
      //apply op
      let op = current_op.unwrap();
      if let Operation::Add(i) = op {
        x += i;
      }

      //charge next
      current_op = operations_iter.next();
    }

    //compute points of interest
    if is_point_of_interest(cycle) {
      poi_list.push(cycle * x);
    }

    if pending_cycles == 0 {
      if current_op.is_some() {
        pending_cycles = current_op.unwrap().cycles();
      } else {
        break;
      }
    }

    pending_cycles -= 1;
    cycle += 1;
  }

  poi_list.iter().sum()
}

pub fn ten() {
  let input = read_data();
  let score = solve(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {

  use super::is_point_of_interest;
  use super::solve;
  #[test]
  fn simple() {
    let input: Vec<String> = SAMPLE.lines().map(|l| l.to_owned()).collect();
    let score = solve(input);
    assert_eq!(score, 13140);
  }

  #[test]
  fn poi() {
    assert!(!is_point_of_interest(19));
    assert!(is_point_of_interest(20));
    assert!(is_point_of_interest(60));
    assert!(!is_point_of_interest(101));
  }

  const SAMPLE: &'static str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;
}
