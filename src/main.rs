mod one;
mod two;

use crate::one::one;
use crate::two::two;
use std::env;
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Invalid number of arguments")
  }

  let day: &str = args.get(1).unwrap();

  match day {
    "1" => one(),
    "2" => two(),
    _ => unreachable!(),
  }
}
