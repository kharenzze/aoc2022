use std::cmp::{Eq, PartialEq};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_data() -> () {
  let filename = format!("./resources/3.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter
    .map(|l| l.unwrap())
    .map(|l| PlannedMove::from_line(&l))
    .collect()
}

pub fn three() {}
