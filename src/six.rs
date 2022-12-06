use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::iter::FromIterator;

fn read_data() -> String {
  let filename = format!("./resources/6.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let mut line_iter = reader.lines();
  line_iter.next().unwrap().unwrap()
}

pub fn six() {
  let input = read_data();
  let score = detect_marker(&input);
  println!("{score}")
}

const CHUNK_SIZE: usize = 4;

fn detect_marker(input: &str) -> usize {
  let mut start: usize = 0;
  let mut found: Option<usize> = None;

  while found.is_none() {
    let end = start + CHUNK_SIZE - 1;
    let chunk = &input[start..=end];
    let set: HashSet<char> = HashSet::from_iter(chunk.chars());
    if set.len() == CHUNK_SIZE {
      found = Some(end);
    } else {
      start += 1;
    }
  }

  found.unwrap() + 1
}

#[cfg(test)]
mod tests {
  use super::detect_marker;

  #[test]
  fn detection() {
    let inputs = vec![
      "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
      "bvwbjplbgvbhsrlpgdmjqwftvncz",
      "nppdvjthqldpwncqszvftbrmjlhg",
      "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
      "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    let expected = vec![7, 5, 6, 10, 11];
    for (i, input) in inputs.iter().enumerate() {
      assert_eq!(detect_marker(*input), expected[i]);
    }
  }
}
