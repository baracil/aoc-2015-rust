use crate::days::day_14::main::day14_launch;
use crate::problem::Part;

mod problem;
mod domain;
mod days;
mod macros;


#[warn(dead_code)]
fn main() {
    let launch = day14_launch;

    println!("part 1 : {}",launch(Part::Part1).unwrap_or_else(|e| e));
    println!("part 2 : {}",launch(Part::Part2).unwrap_or_else(|e| e));
}
