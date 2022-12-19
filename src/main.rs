mod eight;
mod eleven;
mod fifteen;
mod five;
mod four;
mod fourteen;
mod nine;
mod one;
mod point;
mod seven;
mod seventeen;
mod six;
mod ten;
mod thirteen;
mod three;
mod twelve;
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
    "4" => crate::four::four(),
    "5" => crate::five::five(),
    "6" => crate::six::six(),
    "7" => crate::seven::seven(),
    "8" => crate::eight::eight(),
    "9" => crate::nine::nine(),
    "10" => crate::ten::ten(),
    "11" => crate::eleven::eleven(),
    "12" => crate::twelve::twelve(),
    "13" => crate::thirteen::thirteen(),
    "14" => crate::fourteen::fourteen(),
    "15" => crate::fifteen::fifteen(),
    "17" => crate::seventeen::seventeen(),
    _ => unreachable!(),
  }
}
