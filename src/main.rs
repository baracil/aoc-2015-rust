use crate::days::day_17::main::day17_launch;
use crate::problem::Part;

mod days;
mod domain;
mod macros;
mod problem;

#[warn(dead_code)]
fn main() {
    let launch = day17_launch;

    println!("part 1 : {}", launch(Part::Part1).unwrap_or_else(|e| e));
    println!("part 2 : {}", launch(Part::Part2).unwrap_or_else(|e| e));
}
