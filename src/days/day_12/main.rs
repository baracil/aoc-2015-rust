use regex::Regex;
use serde_json::{Value};
use crate::Part;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day12_launch(part: Part) -> AOCResult<String> {
    let line = parse_input(false)?;
    match part {
        Part::Part1 => part1(&line),
        Part::Part2 => part2(&line)
    }
}

fn part1(line: &str) -> AOCResult<String> {
    let regexp = Regex::new("(-?[0-9]+)").map_err(|r| r.to_string())?;
    let sum: i32 = regexp.captures_iter(line)
        .map(|c| c[0].parse::<i32>().unwrap())
        .sum();
    Ok(sum.to_string())
}

fn part2(line: &str) -> AOCResult<String> {
    let book =  serde_json::from_str::<Value>(line).map_err(|o| o.to_string())?;
    Ok(sum_not_red(&book).unwrap_or(0).to_string())
}

pub fn sum_not_red(value:&Value) -> Option<i64> {
    match value {
        Value::Null => Some(0),
        Value::Bool(_) => Some(0),
        Value::Number(number) => number.as_i64(),
        Value::String(value) => {
            if value == "red" {
                None
            } else {
                Some(0)
            }
        }
        Value::Array(array) => {
            Some(array.iter().fold(0i64, |sum, v| sum_not_red(v).unwrap_or(0)+ sum))
        },
        Value::Object(map) => {
            Some(map.values().fold(Some(0i64), |sum, v| sum_option(sum_not_red(v), sum)).unwrap_or(0))
        }
    }
}

fn sum_option(v1:Option<i64>, v2:Option<i64>) -> Option<i64> {
    match (v1,v2) {
        (Some(v1),Some(v2)) => Some(v1+v2),
        _ => None
    }
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<String> {
    Problem::factory(for_test)(12).read_input()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_12::main::{part1, part2};

    #[test]
    fn day12_part1_test_01() {
        let result = part1("[1,2,3]").unwrap();
        assert_eq!(result, "6")
    }

    #[test]
    fn day12_part1_test_02() {
        let result = part1("{\"a\":2,\"b\":4}").unwrap();
        assert_eq!(result, "6")
    }

    #[test]
    fn day12_part1_test_03() {
        let result = part1("[[[3]]]").unwrap();
        assert_eq!(result, "3")
    }

    #[test]
    fn day12_part1_test_04() {
        let result = part1("{\"a\":{\"b\":4},\"c\":-1}").unwrap();
        assert_eq!(result, "3")
    }

    #[test]
    fn day12_part1_test_05() {
        let result = part1(r#"{"a":[-1,1]}"#).unwrap();
        assert_eq!(result, "0")
    }

    #[test]
    fn day12_part2_test_01() {
        let result = part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap();
        assert_eq!(result, "0")
    }

    #[test]
    fn day12_part2_test_02() {
        let result = part2(r#"[1,{"c":"red","b":2},3]"#).unwrap();
        assert_eq!(result, "4")
    }

    #[test]
    fn day12_part2_test_03() {
        let result = part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap();
        assert_eq!(result, "0")
    }

    #[test]
    fn day12_part2_test_04() {
        let result = part2(r#"[1,"red",5]"#).unwrap();
        assert_eq!(result, "6")
    }
}