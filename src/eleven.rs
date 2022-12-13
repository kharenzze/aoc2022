use std::default;
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
  Number(isize),
}

impl OpValue {
  fn from_str(s: &str) -> Self {
    match s {
      "old" => OpValue::Old,
      x => OpValue::Number(x.parse().unwrap()),
    }
  }
}

impl Default for OpValue {
  fn default() -> Self {
    Self::Old
  }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
  Sum,
  Mult,
}

impl Operator {
  fn from_str(s: &str) -> Self {
    match s {
      "+" => Self::Sum,
      "*" => Self::Mult,
      _ => unreachable!(),
    }
  }
}

impl Default for Operator {
  fn default() -> Self {
    Self::Sum
  }
}

#[derive(Debug, Clone, Copy, Default)]
struct Operation {
  a: OpValue,
  b: OpValue,
  operator: Operator,
}

#[derive(Debug, Clone, Copy, Default)]
struct MonkeyTest {
  test_value: isize,
  target_true: usize,
  target_false: usize,
}

impl MonkeyTest {
  fn test(&self, v: isize) -> usize {
    if v % self.test_value == 0 {
      self.target_true
    } else {
      self.target_false
    }
  }
}

#[derive(Debug, Clone, Default)]
struct Monkey {
  id: usize,
  items: Vec<isize>,
  operation: Operation,
  test: MonkeyTest,
}

impl Monkey {
  fn from_lines(lines: Vec<String>) -> Self {
    let mut instance: Self = Default::default();
    for (i, l) in lines.iter().enumerate() {
      match i {
        0 => {
          let id = l
            .strip_prefix("Monkey ")
            .unwrap()
            .strip_suffix(":")
            .unwrap();
          let id: usize = id.parse().unwrap();
          instance.id = id;
        }
        1 => {
          let items = l.trim().strip_prefix("Starting items: ").unwrap();
          let items: Vec<isize> = items
            .split(",")
            .map(|item| item.trim().parse().unwrap())
            .collect();
          instance.items = items;
        }
        2 => {
          let items = l.trim().strip_prefix("Operation: new = ").unwrap();
          let items: Vec<&str> = items.split(" ").collect();
          let op = Operation {
            a: OpValue::from_str(items[0]),
            b: OpValue::from_str(items[2]),
            operator: Operator::from_str(items[1]),
          };
          instance.operation = op;
        }
        3 => {
          let value: isize = l
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
          instance.test.test_value = value;
        }
        4 => {
          let value: usize = l
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
          instance.test.target_true = value;
        }
        5 => {
          let value: usize = l
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
          instance.test.target_false = value;
        }
        _ => unreachable!(),
      }
    }

    instance
  }
}

fn solve(input: Input) -> usize {
  let filtered: Input = input.into_iter().filter(|s| s.is_empty()).collect();
  let grouped: Vec<Input> = filtered.chunks(6).map(|c| c.to_owned()).collect();
  let monkeys: Vec<Monkey> = grouped.into_iter().map(Monkey::from_lines).collect();
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
