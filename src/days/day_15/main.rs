use crate::days::day_15::ingredients::Ingredient;
use crate::days::day_15::solver::{solve, solve_with_calories};
use crate::problem::{AOCResult, Problem};
use crate::{parse_input, Part};

#[allow(dead_code)]
pub fn day15_launch(part: Part) -> AOCResult<String> {
    let ingredients = parse_input(false)?;
    match part {
        Part::Part1 => part1(&ingredients),
        Part::Part2 => part2(&ingredients),
    }
}

fn part1(ingredients: &[Ingredient]) -> AOCResult<String> {
    Ok(solve(ingredients).to_string())
}

fn part2(ingredients: &[Ingredient]) -> AOCResult<String> {
    Ok(solve_with_calories(ingredients).to_string())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<Ingredient>> {
    Problem::factory(for_test)(15).read_input_as_mapped_lines(|l| parse_input!(l, Ingredient))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_15::main::{parse_input, part1, part2};

    #[test]
    fn day15_part1_test() {
        let _input = parse_input(true).unwrap();
        let result = part1(&_input).unwrap();
        assert_eq!(result, "62842880")
    }

    #[test]
    fn day15_part2_test() {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result, "57600000")
    }
}
