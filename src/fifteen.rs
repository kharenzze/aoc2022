use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, Sub};

type Input = Vec<String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Point {
  x: isize,
  y: isize,
}

impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Sub for Point {
  type Output = Point;

  fn sub(self, other: Point) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl Point {
  fn new(x: isize, y: isize) -> Point {
    Point { x, y }
  }

  fn manhattan_distance(&self) -> isize {
    self.x.abs() + self.y.abs()
  }

  fn distance_to(&self, other: &Point) -> isize {
    (*self - *other).manhattan_distance()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Range {
  min: isize,
  max: isize,
}

impl Range {
  fn new(min: isize, max: isize) -> Range {
    Range { min, max }
  }

  fn new_centered(center: isize, radius: isize) -> Range {
    Range {
      min: center - radius,
      max: center + radius,
    }
  }

  fn overlaps(&self, other: &Self) -> bool {
    other.min <= self.max && other.max >= self.min
  }

  fn contains(&self, x: isize) -> bool {
    x >= self.min && x <= self.max
  }

  fn len(&self) -> usize {
    (self.max - self.min + 1) as usize
  }

  fn merge(&self, other: &Self) -> Option<Self> {
    if self.overlaps(other) {
      Some(Self {
        min: self.min.min(other.min),
        max: self.max.max(other.max),
      })
    } else {
      None
    }
  }

  fn into_iter(self) -> impl Iterator<Item = isize> {
    self.min..=self.max
  }

  fn intersection(&self, bound: Self) -> Option<Self> {
    if self.overlaps(&bound) {
      Some(Self {
        min: self.min.max(bound.min),
        max: self.max.min(bound.max),
      })
    } else {
      None
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Sensor {
  pos: Point,
  beacon: Point,
}

impl Sensor {
  fn from_str(s: &str) -> Self {
    lazy_static! {
      static ref RE: Regex = Regex::new(
        r"^Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)$"
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

  fn blocked_cells_in_line(&self, l: isize) -> Option<Range> {
    let d = self.pos.distance_to(&self.beacon);
    let distance_to_line = (self.pos.y - l).abs();
    if distance_to_line > d {
      return None;
    }
    let remaining_distance = d - distance_to_line;
    let blocked_range = Range::new_centered(self.pos.x, remaining_distance);
    Some(blocked_range)
  }
}

fn to_isize(x: &str) -> isize {
  x.parse().unwrap()
}

fn read_data() -> Input {
  let filename = format!("./resources/15.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|l| l.unwrap()).collect()
}

fn solve(input: Input, line: isize) -> usize {
  let sensors: Vec<Sensor> = input.iter().map(|s| Sensor::from_str(s)).collect();

  let known_beacons: HashSet<Point> = sensors.iter().map(|s| s.beacon).collect();

  let mut blocked_ranges: Vec<Range> = sensors
    .iter()
    .filter_map(|s| s.blocked_cells_in_line(line))
    .collect();
  blocked_ranges.sort_by_key(|r| r.min);
  let mut simplified_ranges: Vec<Range> = blocked_ranges.into_iter().fold(vec![], |mut acc, r| {
    if let Some(last) = acc.last_mut() {
      if let Some(merged) = last.merge(&r) {
        *last = merged;
        return acc;
      }
    }
    acc.push(r);
    acc
  });
  let mut blocked_cells = simplified_ranges.iter().fold(0, |acc, r| acc + r.len());

  let mut used_by_sensors: usize = known_beacons
    .iter()
    .filter_map(|beacon| {
      if beacon.y != line {
        return None;
      }
      let x = beacon.x;
      simplified_ranges.iter().find(|r| r.contains(x)).map(|_| 1)
    })
    .sum();
  blocked_cells - used_by_sensors
}

fn solve_v2(input: Input, valid_range: Range) -> usize {
  const TUNING_FREC_MULT: isize = 4000000;
  let sensors: Vec<Sensor> = input.iter().map(|s| Sensor::from_str(s)).collect();

  let mut position = Point::default();

  for line in valid_range.into_iter() {
    let mut blocked_ranges: Vec<Range> = sensors
      .iter()
      .filter_map(|s| {
        let calculated = s.blocked_cells_in_line(line);
        if let Some(r) = calculated {
          r.intersection(valid_range)
        } else {
          None
        }
      })
      .collect();
    blocked_ranges.sort_by_key(|r| r.min);
    let mut simplified_ranges: Vec<Range> =
      blocked_ranges.into_iter().fold(vec![], |mut acc, r| {
        if let Some(last) = acc.last_mut() {
          if let Some(merged) = last.merge(&r) {
            *last = merged;
            return acc;
          }
        }
        acc.push(r);
        acc
      });
    if simplified_ranges.len() == 1 {
      let r = simplified_ranges[0];
      if r.len() == valid_range.len() {
        continue;
      } else {
        let x = if r.contains(valid_range.min) {
          r.min
        } else {
          r.max
        };
        position = Point { x, y: line };
        break;
      }
    } else {
      let r = simplified_ranges[0];
      let x = r.max + 1;
      position = Point { x, y: line };
      break;
    }
  }

  (position.x * TUNING_FREC_MULT + position.y) as usize
}

pub fn fifteen() {
  let input = read_data();
  const LINE: isize = 2000000;
  // let score = solve(input, LINE);
  const MAX: isize = 4000000;
  let score = solve_v2(input, Range::new(0, MAX));
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::{Point, Range, Sensor};
  use indoc::indoc;

  #[test]
  fn parse() {
    let input = "Sensor at x=0, y=0: closest beacon is at x=0, y=0";
    let sensor = Sensor::from_str(input);
    assert_eq!(sensor, Sensor::default());
  }

  #[test]
  fn blocked() {
    let input = "Sensor at x=3, y=3: closest beacon is at x=0, y=0";
    let sensor = Sensor::from_str(input);
    let blocked = sensor.blocked_cells_in_line(0);
    assert_eq!(blocked, Some(Range::new(0, 6)));
  }

  #[test]
  fn join() {
    let r1 = Range::new(0, 6);
    let r2 = Range::new(3, 9);
    let r3 = Range::new(0, 9);
    assert_eq!(r1.merge(&r2), Some(r3));
  }

  #[test]
  fn solve() {
    let input = SAMPLE.lines().map(|s| s.to_string()).collect();
    let score = super::solve(input, 10);
    assert_eq!(score, 26);
  }

  #[test]
  fn solve_v2() {
    let input = SAMPLE.lines().map(|s| s.to_string()).collect();
    let score = super::solve_v2(input, Range::new(0, 20));
    assert_eq!(score, 56000011);
  }

  #[test]
  fn sort() {
    let mut a = vec![Point::new(3, 0), Point::new(1, 1), Point::new(2, 2)];
    a.sort_by_key(|p| p.x);
    assert_eq!(a[0].x, 1);
  }

  const SAMPLE: &'static str = indoc! {"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"};
}
