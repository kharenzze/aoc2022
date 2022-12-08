use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::iter::FromIterator;

fn read_data() -> String {
  let filename = format!("./resources/8.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let mut line_iter = reader.lines();
  line_iter.next().unwrap().unwrap()
}

pub fn eight() {
  let input = read_data();
  unimplemented!()
}

#[cfg(test)]
mod tests {

  #[test]
  fn simple() {}
}
