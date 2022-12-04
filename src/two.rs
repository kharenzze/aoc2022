use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_data() -> Vec<Vec<usize>> {
  let filename = format!("./resources/1.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let mut line_iter = reader.lines();
  let mut lists: Vec<Vec<usize>> = vec![];
  let mut current: Vec<usize> = vec![];

  while let Some(Ok(line)) = line_iter.next() {
    if line.len() == 0 {
      lists.push(current);
      current = vec![];
      continue;
    }
    let value: usize = line.parse().unwrap();
    current.push(value);
  }
  if current.len() != 0 {
    lists.push(current);
  }
  lists
}

pub fn one() {
  let lists = read_data();
  let mut summaries: Vec<usize> = lists.iter().map(|list| list.iter().sum()).collect();
  summaries.sort();
  summaries.reverse();
  let max: usize = summaries.iter().take(3).sum();
  println!("{max}");
}
