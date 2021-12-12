use crate::days::day_10::reader::read;
use crate::Part;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day10_launch(part: Part) -> AOCResult<String> {
    let input = parse_input(false)?;
    match part {
        Part::Part1 => part1(&input),
        Part::Part2 => part2(&input)
    }
}

fn part1(input:&str) -> AOCResult<String> {
    solve(input,40)
}

fn part2(input:&str) -> AOCResult<String> {
    solve(input,50)
}

fn solve(input:&str,nb_steps:usize) -> AOCResult<String> {
    let mut current = input.to_string();

    for _ in 0..nb_steps {
        let new = read(current.as_str()).to_string();
        current = new;
    };

    Ok(current.len().to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<String> {
    Problem::factory(for_test)(10).read_input()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_10::main::{parse_input, part2};
    use crate::days::day_10::reader::read;

    #[test]
    fn day10_part1_1()  {
        let result = read("1");
        assert_eq!(result,"11")
    }

    #[test]
    fn day10_part1_11()  {
        let result = read("11");
        assert_eq!(result,"21")
    }

    #[test]
    fn day10_part1_21()  {
        let result = read("21");
        assert_eq!(result,"1211")
    }

    #[test]
    fn day10_part1_111221()  {
        let result = read("111221");
        assert_eq!(result,"312211")
    }

    #[test]
    #[ignore]
    fn day10_part2_test()  {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result,"")
    }
}