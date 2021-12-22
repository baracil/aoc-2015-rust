use std::collections::HashSet;
use crate::days::day_18::gol::{Gol, Position};
use crate::Part;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day18_launch(part: Part) -> AOCResult<String> {
    let content = parse_input(false)?;
    match part {
        Part::Part1 => part1(&content),
        Part::Part2 => part2(&content)
    }
}

fn part1(lines:&[String]) -> AOCResult<String> {
    let mut gol = Gol::parse(lines,false);
    gol.perform_several_steps(100);
    Ok(gol.number_of_lights_on().to_string())
}

fn part2(lines:&[String]) -> AOCResult<String> {
    let mut gol = Gol::parse(lines,true);
    gol.perform_several_steps(100);
    Ok(gol.number_of_lights_on().to_string())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<String>> {
    Problem::factory(for_test)(18).read_input_as_vec_of_lines()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_18::gol::Gol;
    use crate::days::day_18::main::{parse_input, part1, part2};

    #[test]
    fn day18_part1_test() {
        let content = parse_input(true).unwrap();
        let mut gol = Gol::parse(&content,false);
        gol.perform_several_steps(4);
        assert_eq!(gol.number_of_lights_on(), 4)
    }

    #[test]
    fn day18_part2_test() {
        let content = parse_input(true).unwrap();
        let mut gol = Gol::parse(&content,true);
        gol.perform_several_steps(5);
        assert_eq!(gol.number_of_lights_on(), 17)
    }
}