use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_data() -> String {
  let filename = format!("./resources/5.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let mut line_iter = reader.lines();
  line_iter.next().unwrap().unwrap()
}

pub fn six() {
  let input = read_data();
}

fn detect_marker(input: &str) -> usize {
  unimplemented!()
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
