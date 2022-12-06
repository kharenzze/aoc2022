use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Movement {
  from: usize,
  to: usize,
  count: usize,
}

impl Movement {
  fn from_line(l: &String) -> Self {
    let words: Vec<_> = l.split_whitespace().collect();
    let numbers: Vec<usize> = words
      .iter()
      .enumerate()
      .filter(|(i, _)| i % 2 == 1)
      .map(|(_, v)| v.parse().unwrap())
      .collect();
    Movement {
      from: numbers[1] - 1,
      to: numbers[2] - 1,
      count: numbers[0],
    }
  }
}

fn read_data() -> Vec<Movement> {
  let filename = format!("./resources/5.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter
    .map(|l| l.unwrap())
    .filter(|l| l.starts_with('m'))
    .map(|m| Movement::from_line(&m))
    .collect()
}

pub fn five() {
  let mut data = vec![
    vec!['Q', 'M', 'G', 'C', 'L'],
    vec!['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G'],
    vec!['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R'],
    vec!['J', 'F', 'D', 'V', 'Q', 'P'],
    vec!['N', 'F', 'M', 'S', 'L', 'B', 'T'],
    vec!['R', 'N', 'V', 'H', 'C', 'D', 'P'],
    vec!['H', 'C', 'T'],
    vec!['G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P'],
    vec!['Z', 'F', 'H', 'G'],
  ];
  let movements = read_data();
  for m in movements {
    let from: &mut Vec<_> = data.get_mut(m.from).unwrap();
    let end = from.len();
    let start = end - m.count;
    let mut dropped: Vec<char> = from.drain(start..end).collect();
    dropped.reverse();
    let to: &mut Vec<_> = data.get_mut(m.to).unwrap();
    to.append(&mut dropped);
  }
  let score: String = data
    .iter()
    .map(|list| list.last().unwrap().to_owned())
    .collect();
  println!("{score}");
}
