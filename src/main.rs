use crate::days::day_16::main::day16_launch;
use crate::problem::Part;

mod problem;
mod domain;
mod days;
mod macros;


#[warn(dead_code)]
fn main() {
    let launch = day16_launch;

    println!("part 1 : {}",launch(Part::Part1).unwrap_or_else(|e| e));
    println!("part 2 : {}",launch(Part::Part2).unwrap_or_else(|e| e));
}
