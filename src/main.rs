mod eight;
mod five;
mod nine;
mod one;
mod point;
mod seven;
mod six;
mod ten;
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
    "5" => crate::five::five(),
    "6" => crate::six::six(),
    "7" => crate::seven::seven(),
    "8" => crate::eight::eight(),
    "9" => crate::nine::nine(),
    "10" => crate::ten::ten(),
    _ => unreachable!(),
  }
}
