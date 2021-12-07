use crate::Part;
use crate::problem::{Problem, AOCResult};


#[allow(dead_code)]
pub fn day01_launch(part: Part) -> AOCResult<String> {
    let directions = Problem::of_day(1).read_input()?;
    match part {
        Part::Part1 => part1(&directions),
        Part::Part2 => part2(&directions)
    }
}


fn part1(directions:&str) -> AOCResult<String> {
    Ok(directions
        .chars()
        .fold(0,|level,direction| level + compute_level_offset(direction)))
        .map(|i| i.to_string())
}


fn part2(directions:&str) -> AOCResult<String> {
    let mut level = 0;
    for (position,direction) in directions.chars().enumerate() {
        level += compute_level_offset(direction);
        if level == -1 {
            return Ok((position+1).to_string())
        }
    };
    Err("Basement never reached".to_string())
}



fn compute_level_offset(direction:char) -> i32 {
    if direction =='(' {1} else {-1}
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<String> {
    Problem::factory(for_test)(1).read_input()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_02::{part1, part2};

    #[test]
    fn day01_part1_test_01()  {
        let result = part1("(())").unwrap();
        assert_eq!(result,"0")
    }

    #[test]
    fn day01_part1_test_02()  {
        let result = part1("(()(()(").unwrap();
        assert_eq!(result,"3")
    }
    #[test]
    fn day01_part1_test_03()  {
        let result = part1("()()").unwrap();
        assert_eq!(result,"0")
    }
    #[test]
    fn day01_part1_test_04()  {
        let result = part1("))(((((").unwrap();
        assert_eq!(result,"3")
    }


    #[test]
    fn day01_part2_test_01()  {
        let result = part2(")").unwrap();
        assert_eq!(result,"1")
    }

    #[test]
    fn day01_part2_test_02()  {
        let result = part2("()())").unwrap();
        assert_eq!(result,"5")
    }

}