use std::cell::RefCell;
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

  fn to_value(&self, old: isize) -> isize {
    match self {
      Self::Old => old,
      Self::Number(n) => *n,
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

impl Operation {
  fn exec(&self, val: isize) -> isize {
    let a = self.a.to_value(val);
    let b = self.b.to_value(val);
    match self.operator {
      Operator::Sum => a + b,
      Operator::Mult => a * b,
    }
  }
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
  inspected_items: usize,
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

const ITERATIONS: usize = 10000;

fn solve(input: Input) -> usize {
  let filtered: Input = input.into_iter().filter(|s| !s.is_empty()).collect();
  let grouped: Vec<Input> = filtered.chunks(6).map(|c| c.to_owned()).collect();
  let monkeys: Vec<RefCell<Monkey>> = grouped
    .into_iter()
    .map(Monkey::from_lines)
    .map(|m| RefCell::new(m))
    .collect();
  let common_multiplier = monkeys
    .iter()
    .map(|m| m.borrow().test.test_value)
    .fold(1, |acc, x| acc * x);
  for _ in 0..ITERATIONS {
    for monkey_ref in monkeys.iter() {
      {
        let m = monkey_ref.borrow();
        for &item in m.items.iter() {
          let next_worry_level = m.operation.exec(item);
          let target = m.test.test(next_worry_level);
          let mut target = monkeys[target].borrow_mut();
          target.items.push(next_worry_level % common_multiplier);
        }
      }
      let mut m = monkey_ref.borrow_mut();
      let inspected = m.items.len();
      m.items = vec![];
      m.inspected_items += inspected;
    }
  }

  let mut monkeys: Vec<usize> = monkeys
    .into_iter()
    .map(|m| m.into_inner())
    .map(|m| m.inspected_items)
    .collect();

  monkeys.sort();
  let score: usize = monkeys.iter().rev().take(2).fold(1, |acc, v| acc * *v);
  score
}

pub fn eleven() {
  let input = read_data();
  let score = solve(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn simple() {
    let input: Vec<String> = SAMPLE
      .lines()
      .map(|r| r.to_owned())
      .filter(|s| !s.is_empty())
      .collect();
    println!("{input:?}");
    let score = solve(input);
    assert_eq!(score, 2713310158);
  }

  const SAMPLE: &'static str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;
}
