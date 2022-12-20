use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

#[derive(Debug)]
struct Node {
  name: String,
  rate: usize,
  is_open: bool,
  children: Vec<String>,
}

impl Node {
  fn from_line(line: &str) -> Node {
    lazy_static! {
      static ref RE: Regex =
        Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels lead to valves (.*)$").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let name = caps.get(1).unwrap().as_str().to_string();
    let rate: usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let children: Vec<String> = caps
      .get(3)
      .unwrap()
      .as_str()
      .split(", ")
      .map(|s| s.to_string())
      .collect();
    Node {
      name,
      rate,
      is_open: false,
      children,
    }
  }
}

fn read_data() -> Input {
  let filename = format!("./resources/16.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

const MINUTES: usize = 30;
fn solve(input: Input) -> usize {
  let node_list: Vec<Node> = input.iter().map(|l| Node::from_line(l)).collect();
  let mut nodes: HashMap<String, RefCell<Node>> = HashMap::new();
  for node in node_list {
    nodes.insert(node.name.clone(), RefCell::new(node));
  }
  let current = "AA".to_string();
  let mut score: usize = 0;
  for i in 1..=MINUTES {
    let pending = nodes
      .iter()
      .filter(|(_, nref)| {
        let n = nref.borrow();
        !n.is_open && n.rate > 0
      })
      .count();
  }

  1
}

pub fn sixteen() {
  let input = read_data();
  let score = solve(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    let input = "Valve TM has flow rate=0; tunnels lead to valves KF, AA";
    let node = Node::from_line(input);
    let expected = Node {
      name: "TM".to_string(),
      rate: 0,
      is_open: false,
      children: vec!["KF".to_string(), "AA".to_string()],
    };

    assert_eq!(node.name, expected.name);
    assert_eq!(node.is_open, expected.is_open);
    assert_eq!(node.rate, expected.rate);
    assert_eq!(node.children, expected.children);
    for i in 0..node.children.len() {
      assert_eq!(node.children[i], expected.children[i]);
    }
  }
}
