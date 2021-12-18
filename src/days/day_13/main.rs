use crate::days::day_13::graph::Graph;
use crate::days::day_13::path::Path;
use crate::days::day_13::seating_finder::SeatingFinder;
use crate::problem::{AOCResult, Problem};
use crate::{parse_input, Part};

#[allow(dead_code)]
pub fn day13_launch(part: Part) -> AOCResult<String> {
    let paths = parse_input(false)?;
    match part {
        Part::Part1 => part1(&paths),
        Part::Part2 => part2(&paths),
    }
}

fn part1(paths: &[Path]) -> AOCResult<String> {
    let graph = Graph::new(paths);
    let happiest_seating = SeatingFinder::find_happiest_seating(&graph);
    Ok(happiest_seating.to_string())
}

fn part2(paths: &[Path]) -> AOCResult<String> {
    let graph = Graph::new_with_myself(paths);
    let happiest_seating = SeatingFinder::find_happiest_seating(&graph);
    Ok(happiest_seating.to_string())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<Path>> {
    Problem::factory(for_test)(13).read_input_as_mapped_lines(|l| parse_input!(l, Path))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_13::main::{parse_input, part1, part2};

    #[test]
    fn day13_part1_test() {
        let graph = parse_input(true).unwrap();
        let result = part1(&graph).unwrap();
        assert_eq!(result, "330")
    }

    #[test]
    #[ignore]
    fn day13_part2_test() {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result, "")
    }
}
