mod one;
mod three;
mod two;

use std::env;
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Invalid number of arguments")
  }

  let day: &str = args.get(1).unwrap();

  match day {
    "1" => crate::one::one(),
    "2" => crate::two::two(),
    "3" => crate::three::three(),
    _ => unreachable!(),
  }
}
