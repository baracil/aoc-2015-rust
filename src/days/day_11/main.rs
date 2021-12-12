use crate::{parse_input, Part};
use crate::days::day_11::password::Password;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day11_launch(part: Part) -> AOCResult<String> {
    let line = parse_input(false)?;
    match part {
        Part::Part1 => part1(&line),
        Part::Part2 => part2(&line)
    }
}

fn part1(line:&str) -> AOCResult<String> {
    let mut password = Password::new(line);
    Ok(password.generate_next_valid())
}

fn part2(line:&str) -> AOCResult<String> {
    let mut password = Password::new(line);
    password.generate_next_valid();
    Ok(password.generate_next_valid())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<String> {
    Problem::factory(for_test)(11).read_input()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_11::main::{parse_input, part1, part2};
    use crate::days::day_11::password::Password;

    #[test]
    fn day11_part1_test_01()  {
        let result = part1("abcdefgh").unwrap();
        assert_eq!(result,"abcdffaa")
    }

    #[test]
    fn day11_part1_test_02()  {
        let result = part1("ghijklmn").unwrap();
        assert_eq!(result,"ghjaabcc")
    }

    #[test]
    fn day11_part1_test_requirement2()  {
        let password = Password::new("ghjaabcc");
        assert_eq!(password.is_requirement2_fulfil(), true);
    }

    #[test]
    fn day11_part1_test_requirement1_and_3()  {
        let password = Password::new("ghjaabcc");
        assert_eq!(password.are_requirement1_and_3_fulfil(), true);
    }

    #[test]
    #[ignore]
    fn day11_part2_test()  {
        let result = part2("").unwrap();
        assert_eq!(result,"")
    }
}