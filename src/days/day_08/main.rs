use crate::days::day_08::string_info::StringInfo;
use crate::{parse_input, Part};
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day08_launch(part: Part) -> AOCResult<String> {
    let lines = parse_input(false)?;
    match part {
        Part::Part1 => part1(&lines),
        Part::Part2 => part2(&lines)
    }
}

fn part1(lines:&[String]) -> AOCResult<String> {
    let sum:usize = lines.iter()
        .map(|l| parse_input!(l, StringInfo))
        .map(|l| l.memory_overhead())
        .sum();
    Ok(sum.to_string())
}

fn part2(lines:&[String]) -> AOCResult<String> {
    let to_escape_counter = |l:&String| -> usize {2+l.chars().filter(|c| *c=='"' || *c=='\\').count()};
//    let to_escape_counter = |l:&String| (l.matches("\\").count()+l.matches("\"").count());
    let sum:usize = lines.iter()
        .map(|l| to_escape_counter(l))
        .sum();

    Ok(sum.to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<Vec<String>> {
    Problem::factory(for_test)(8).read_input_as_vec_of_lines()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_08::main::{parse_input, part1, part2};

    #[test]
    fn day08_part1_test()  {
        let _input = parse_input(true).unwrap();
        let result = part1(&_input).unwrap();
        assert_eq!(result,"12")
    }

    #[test]
    fn day08_part2_test()  {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result,"19")
    }
}