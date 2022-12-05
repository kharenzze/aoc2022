use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_data() -> Vec<String> {
  let filename = format!("./resources/3.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

const A_UPPER: usize = 'A' as usize;
const A_LOWER: usize = 'a' as usize;

fn char_to_priority(c: char) -> usize {
  let int = c as usize;
  if int >= A_LOWER {
    return int - A_LOWER + 1;
  } else {
    return int - A_UPPER + 27;
  }
}

fn find_item(line: &String) -> char {
  let n = line.len();
  let (a, b) = line.split_at(n / 2);
  let char_set: HashSet<char> = a.chars().collect();
  for c in b.chars() {
    if char_set.contains(&c) {
      return c;
    }
  }
  unreachable!()
}

pub fn three() {
  let input = read_data();
  let score: usize = input.iter().map(find_item).map(char_to_priority).sum();
  println!("{score}");
}

#[cfg(test)]
mod tests {
  use super::char_to_priority;

  #[test]
  fn priority() {
    assert_eq!(1, char_to_priority('a'));
    assert_eq!(26, char_to_priority('z'));
    assert_eq!(27, char_to_priority('A'));
    assert_eq!(52, char_to_priority('Z'));
  }
}
