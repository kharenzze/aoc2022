use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
  pub x: usize,
  pub y: usize,
}

const X: Point = Point::new(1, 0);
const Y: Point = Point::new(0, 1);
const ZERO: Point = Point::new(0, 0);

impl Display for Point {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl From<(usize, usize)> for Point {
  fn from(input: (usize, usize)) -> Self {
    Self {
      x: input.0,
      y: input.1,
    }
  }
}

impl Add<Point> for Point {
  type Output = Point;

  fn add(self, rhs: Point) -> Self::Output {
    Point::from((self.x + rhs.x, self.y + rhs.y))
  }
}

impl Point {
  pub const fn new(x: usize, y: usize) -> Self {
    Self { x, y }
  }

  pub fn is_contained_in(&self, reference: &Self) -> bool {
    self.x < reference.x && self.y < reference.y
  }

  pub fn contains(&self, other: &Self) -> bool {
    self.x > other.x && self.y > other.y
  }

  pub fn get_points_around(&self) -> [Option<Point>; 4] {
    [
      Some(*self + X),
      Some(*self + Y),
      self.checked_sub(&X),
      self.checked_sub(&Y),
    ]
  }

  pub fn checked_sub(&self, rhs: &Point) -> Option<Point> {
    let x = self.x.checked_sub(rhs.x)?;
    let y = self.y.checked_sub(rhs.y)?;
    Some(Point::new(x, y))
  }

  pub fn squared_norm(&self) -> usize {
    self.x * self.x + self.y * self.y
  }

  pub fn squared_distance(&self, p: Point) -> usize {
    let x = usize_diff(self.x, p.x);
    let y = usize_diff(self.y, p.y);
    Point::new(x, y).squared_norm()
  }
}

fn usize_diff(a: usize, b: usize) -> usize {
  if a > b {
    a - b
  } else {
    b - a
  }
}

#[cfg(test)]
mod tests {
  use super::{Point, X, Y, ZERO};

  #[test]
  fn is_contained() {
    let ref_ = Point::new(4, 4);
    let a = Point::new(2, 2);
    assert!(a.is_contained_in(&ref_));

    let a = Point::new(4, 2);
    assert!(!a.is_contained_in(&ref_));

    let a = Point::new(2, 4);
    assert!(!a.is_contained_in(&ref_));
  }

  #[test]
  fn contains() {
    let ref_ = Point::new(4, 4);
    let a = Point::new(2, 2);
    assert!(!a.contains(&ref_));
    assert!(ref_.contains(&a));

    let a = Point::new(4, 2);
    assert!(!a.contains(&ref_));
  }

  #[test]
  fn add() {
    assert_eq!(X + Y, Point::new(1, 1))
  }

  #[test]
  fn checked_sub() {
    assert_eq!(X.checked_sub(&Y), None);
    assert_eq!(X.checked_sub(&X), Some(Point::new(0, 0)));
  }

  #[test]
  fn get_points_around() {
    let points = ZERO.get_points_around();
    assert_eq!(points.len(), 2);
    assert_eq!(points.iter().find(|v| v.eq(&&Some(X))).is_some(), true);
    assert_eq!(points.iter().find(|v| v.eq(&&Some(Y))).is_some(), true);
  }

  #[test]
  fn squared_norm() {
    assert_eq!(ZERO.squared_norm(), 0);
    let p = Point::new(3, 3);
    assert_eq!(p.squared_norm(), 9 + 9);
  }

  #[test]
  fn usize_diff() {
    let p = Point::new(2, 2);
    let dist = p.squared_distance(ZERO);
    assert_eq!(dist, ZERO.squared_distance(p));
    assert_eq!(dist, p.squared_norm());
  }

  #[test]
  fn display() {
    let text = format!("{}", &ZERO);
    let expected = "(0, 0)";
    assert_eq!(text, expected);
  }
}
