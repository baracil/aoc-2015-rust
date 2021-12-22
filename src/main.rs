use crate::days::day_18::main::day18_launch;
use crate::problem::Part;

mod days;
mod domain;
mod macros;
mod problem;

#[warn(dead_code)]
fn main() {
    let launch = day18_launch;

    println!("part 1 : {}", launch(Part::Part1).unwrap_or_else(|e| e));
    println!("part 2 : {}", launch(Part::Part2).unwrap_or_else(|e| e));
}
