use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = Vec<String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Point {
  x: isize,
  y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Sensor {
  pos: Point,
  beacon: Point,
}

fn to_isize(x: &str) -> isize {
  x.parse().unwrap()
}

impl Sensor {
  fn from_str(s: &str) -> Self {
    lazy_static! {
      static ref RE: Regex = Regex::new(
        r"^Sensor at x=(\-?\d+\), y=(\-?\d+\): closest beacon is at x=(\-?\d+\), y=(\-?\d+)$"
      )
      .unwrap();
    }
    let mut sensor = Sensor::default();
    let caps = RE.captures(s).unwrap();
    sensor.pos.x = to_isize(&caps[1]);
    sensor.pos.y = to_isize(&caps[2]);
    sensor.beacon.x = to_isize(&caps[3]);
    sensor.beacon.y = to_isize(&caps[4]);
    sensor
  }
}

fn read_data() -> Input {
  let filename = format!("./resources/15.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn solve(input: Input) -> usize {
  unimplemented!()
}

pub fn seven() {
  let input = read_data();
  let score = solve(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::Sensor;
  #[test]
  fn parse() {
    let input = "Sensor at x=0, y=0: closest beacon is at x=0, y=0";
    let sensor = Sensor::from_str(input);
    assert_eq!(sensor, Sensor::default());
  }
}
