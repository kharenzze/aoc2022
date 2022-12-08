use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

#[derive(Debug)]
enum ConsoleLine {
  CD(String),
  LS,
  Directory(String),
  File(String, usize),
}

impl ConsoleLine {
  fn from_line(l: &String) -> Self {
    let chunks: Vec<&str> = l.split(' ').collect();
    match chunks[0] {
      "$" => match chunks[1] {
        "cd" => ConsoleLine::CD(chunks[2].to_owned()),
        "ls" => ConsoleLine::LS,
        _ => unreachable!(),
      },
      "dir" => ConsoleLine::Directory(chunks[1].to_owned()),
      size => {
        let s: usize = size.parse().unwrap();
        let name: String = chunks[1].to_owned();
        ConsoleLine::File(name, s)
      }
    }
  }
}

fn read_data() -> Input {
  let filename = format!("./resources/7.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|r| r.unwrap()).collect()
}

fn solve(input: Input) -> usize {
  for cmd_line in input {
    let cmd = ConsoleLine::from_line(&cmd_line);
  }
  unimplemented!()
}

pub fn seven() {
  let input = read_data();
  let score = solve(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {

  #[test]
  fn simple() {
    unimplemented!()
  }
}
