use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

#[derive(Debug, Clone)]
enum PacketElement {
  Integer(isize),
  List(Vec<PacketElement>),
}

impl PacketElement {
  fn from_str(s: &str) -> Self {
    if s.starts_with('[') {
      let mut list = Vec::new();
      let mut current = String::new();
      let mut depth = 0;
      for c in s.chars() {
        match c {
          '[' => {
            depth += 1;
            if depth > 1 {
              current.push(c);
            }
          }
          ']' => {
            depth -= 1;
            if depth == 0 {
              list.push(PacketElement::from_str(&current));
              current = String::new();
            } else {
              current.push(c);
            }
          }
          ',' => {
            if depth == 1 {
              list.push(PacketElement::from_str(&current));
              current = String::new();
            } else {
              current.push(c);
            }
          }
          _ => current.push(c),
        }
      }
      PacketElement::List(list)
    } else if s.is_empty() {
      PacketElement::List(Vec::new())
    } else {
      PacketElement::Integer(s.parse().unwrap())
    }
  }
}

impl PartialOrd for PacketElement {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (PacketElement::Integer(a), PacketElement::Integer(b)) => a.partial_cmp(b),
      (PacketElement::List(a), PacketElement::List(b)) => {
        let mut a = a.iter();
        let mut b = b.iter();
        loop {
          match (a.next(), b.next()) {
            (Some(a), Some(b)) => match a.partial_cmp(b) {
              Some(std::cmp::Ordering::Equal) => continue,
              Some(order) => return Some(order),
              None => return None,
            },
            (Some(_), None) => return Some(std::cmp::Ordering::Greater),
            (None, Some(_)) => return Some(std::cmp::Ordering::Less),
            (None, None) => return Some(std::cmp::Ordering::Equal),
          }
        }
      }
      (PacketElement::Integer(_), PacketElement::List(_)) => {
        PacketElement::List(vec![self.clone()]).partial_cmp(other)
      }
      (PacketElement::List(_), PacketElement::Integer(_)) => {
        self.partial_cmp(&PacketElement::List(vec![other.clone()]))
      }
    }
  }
}

impl Ord for PacketElement {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl Eq for PacketElement {}

impl PartialEq for PacketElement {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
  }
}

fn read_data() -> Input {
  let filename = format!("./resources/13.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn solve(input: Input) -> usize {
  let mapped: Vec<PacketElement> = input
    .into_iter()
    .filter(|p| !p.is_empty())
    .map(|s| PacketElement::from_str(s.as_str()))
    .collect();
  let pairs: Vec<&[PacketElement]> = mapped.chunks_exact(2).collect();
  let mut score = 0;
  for (i, p) in pairs.iter().enumerate() {
    if p[0] < p[1] {
      score += i + 1;
    }
  }

  score
}

fn solve_v2(input: Input) -> usize {
  let mut mapped: Vec<PacketElement> = input
    .into_iter()
    .filter(|p| !p.is_empty())
    .map(|s| PacketElement::from_str(s.as_str()))
    .collect();
  let beacon1: PacketElement = PacketElement::from_str("[[2]]");
  let beacon2: PacketElement = PacketElement::from_str("[[6]]");
  mapped.push(beacon1.clone());
  mapped.push(beacon2.clone());
  mapped.sort();
  let b1_index = mapped.iter().position(|p| p == &beacon1).unwrap() + 1;
  let b2_index = mapped.iter().position(|p| p == &beacon2).unwrap() + 1;

  b2_index * b1_index
}

pub fn thirteen() {
  let input = read_data();
  // let score = solve(input);
  let score = solve_v2(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::PacketElement;
  use indoc::indoc;

  #[test]
  fn parse() {
    let input = "[1,2,3]";
    let result = PacketElement::from_str(input);
    let expected = PacketElement::List(vec![
      PacketElement::Integer(1),
      PacketElement::Integer(2),
      PacketElement::Integer(3),
    ]);
    assert_eq!(result, expected);
  }

  const SAMPLE: &'static str = indoc! {r#"
  [1,1,3,1,1]
  [1,1,10,1,1]

  [[1],[2,3,4]]
  [[1],4]

  [9]
  [[8,7,6]]

  [[4,4],4,4]
  [[4,4],4,4,4]

  [7,7,7,7]
  [7,7,7]

  []
  [3]

  [[[]]]
  [[]]

  [1,[2,[3,[4,[5,6,7]]]],8,9]
  [1,[2,[3,[4,[5,6,0]]]],8,9]
"#};

  #[test]
  fn solve() {
    let input = SAMPLE.lines().map(|s| s.to_string()).collect();
    let result = super::solve(input);
    assert_eq!(result, 13);
  }

  #[test]
  fn solve_v2() {
    let input = SAMPLE.lines().map(|s| s.to_string()).collect();
    let result = super::solve_v2(input);
    assert_eq!(result, 140);
  }

  #[test]
  fn simple() {
    assert!(true);
  }
}
