use crate::days::day_14::reinder::Reinder;
use crate::{parse_input, Part};
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day14_launch(part: Part) -> AOCResult<String> {
    let reinders = parse_input(false)?;
    match part {
        Part::Part1 => part1(&reinders),
        Part::Part2 => part2(&reinders)
    }
}

fn part1(reinders:&Vec<Reinder>) -> AOCResult<String> {
    compute_max_distance(reinders,2503)
}

fn part2(reinders:&Vec<Reinder>) -> AOCResult<String> {
    compute_points(reinders,2503)
}

fn compute_max_distance(reinders:&Vec<Reinder>, time:u32) -> AOCResult<String> {
    Ok(reinders.iter()
        .map(|r| r.distance(time))
        .max()
        .ok_or("No reinders !!!")?.to_string())
}

fn compute_points(reinders:&Vec<Reinder>, time:u32) -> AOCResult<String> {

    let nb_reinders = reinders.len();

    let mut points = vec![0u32;reinders.len()];
    let mut distance = vec![0u32;reinders.len()];
    for t in 1..=time {
        for r in 0..nb_reinders {
            distance[r] =  reinders[r].distance(t);
        }

        let max_distance = *distance.iter().max().unwrap();

        for r in 0..nb_reinders {
            if distance[r] == max_distance {
                points[r] += 1;
            }
        }
    }

    Ok(points.iter().max().expect("No Reinders !!!").to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<Vec<Reinder>> {
    Problem::factory(for_test)(14).read_input_as_mapped_lines(|l| parse_input!(l,Reinder))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_14::main::{compute_max_distance, compute_points, parse_input};

    #[test]
    fn day14_part1_test()  {
        let reinders = parse_input(true).unwrap();
        let result = compute_max_distance(&reinders, 1000).unwrap();
        assert_eq!(result,"1120")
    }

    #[test]
    fn day14_part2_test()  {
        let reinders = parse_input(true).unwrap();
        let result = compute_points(&reinders, 1000).unwrap();
        assert_eq!(result,"689")
    }
}