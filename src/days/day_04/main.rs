use crate::problem::{AOCResult, Problem};
use crate::Part;
use md5::Digest;

#[allow(dead_code)]
pub fn day04_launch(part: Part) -> AOCResult<String> {
    let secret_key = Problem::of_day(4).read_input()?;
    match part {
        Part::Part1 => part1(&secret_key),
        Part::Part2 => part2(&secret_key),
    }
}

fn compute_md5(secret_key: &str, suffix: usize) -> Digest {
    let key = format!("{}{}", secret_key, suffix);
    md5::compute(key)
}

fn is_md5_valid(md5: &Digest, last_digit_mask: u8) -> bool {
    if md5.0[0] != 0u8 {
        return false;
    }

    if md5.0[1] != 0u8 {
        return false;
    }
    // 11110000
    md5.0[2] & last_digit_mask == 0u8
}

fn part1(secret_key: &str) -> AOCResult<String> {
    solve_day04(secret_key, 240)
}

fn part2(secret_key: &str) -> AOCResult<String> {
    solve_day04(secret_key, 255)
}

fn solve_day04(secret_key: &str, last_digit_mask: u8) -> AOCResult<String> {
    let mut suffix: usize = 0;
    loop {
        suffix += 1;
        let md5 = compute_md5(secret_key, suffix);
        if is_md5_valid(&md5, last_digit_mask) {
            return Ok(suffix.to_string());
        }
    }
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<String> {
    Problem::factory(for_test)(3).read_input()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_04::main::{compute_md5, is_md5_valid, part1};

    #[test]
    fn day04_part1_test_01() {
        let md5 = compute_md5("abcdef", 609043);
        let valid = is_md5_valid(&md5, 240);
        assert_eq!(true, valid)
    }

    #[test]
    #[ignore]
    fn day04_part1_test_02() {
        let result = part1("abcdef").unwrap();
        assert_eq!(result, "609043")
    }

    #[test]
    #[ignore]
    fn day04_part1_test_03() {
        let result = part1("pqrstuv").unwrap();
        assert_eq!(result, "1048970")
    }

    // #[test]
    // #[ignore]
    // fn day04_part2_test()  {
    //     let _input = parse_input(true).unwrap();
    //     let result = part2().unwrap();
    //     assert_eq!(result,"")
    // }
}
