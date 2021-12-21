use std::collections::{HashMap, VecDeque};
use crate::{parse_input, Part};
use crate::days::day_17::filler::Filling;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day17_launch(part: Part) -> AOCResult<String> {
    let containers = parse_input(false)?;
    match part {
        Part::Part1 => part1(&containers),
        Part::Part2 => part2(&containers)
    }
}

fn part1(containers: &[u32]) -> AOCResult<String> {
    Ok(compute_number_of_combinations(containers, 150).to_string())
}

fn part2(containers: &[u32]) -> AOCResult<String> {
    Ok(compute_number_of_minimum_combinations(containers, 150).to_string())
}


fn compute_number_of_combinations(containers: &[u32], amount_of_eggnog: usize) -> usize {
    compute_combinations(containers, amount_of_eggnog).values().sum()
}

fn compute_number_of_minimum_combinations(containers: &[u32], amount_of_eggnog: usize) -> usize {
    compute_combinations(containers, amount_of_eggnog)
        .iter()
        .min_by(|(key1, value1), (key2, value2)| key1.cmp(key2))
        .map(|(key, value)| *value)
        .unwrap_or(0)
}

fn compute_combinations(containers: &[u32], amount_of_eggnog: usize) -> HashMap::<usize, usize> {
    let mut available = vec![true; containers.len()];

    let mut combinations = HashMap::<usize, usize>::new();

    let mut fillings = VecDeque::<(usize, usize, usize)>::new();

    (0..containers.len()).for_each(|c| fillings.push_back((0, c, 0)));


    while let Some((volume, container_idx, nb_container_used)) = fillings.pop_front() {
        let container_volume = containers[container_idx] as usize;
        let new_volume = volume + container_volume;

        if new_volume == amount_of_eggnog {
            let value = combinations.entry(nb_container_used).or_insert(0);
            *value += 1;
        } else if new_volume < amount_of_eggnog {
            available[container_idx] = false;

            containers
                .iter()
                .enumerate()
                .filter(|(idx, _)| container_idx < *idx && available[*idx])
                .for_each(|(idx, cont)| {
                    fillings.push_back((new_volume, idx, nb_container_used + 1))
                });

            available[container_idx] = true;
        }
    }

    combinations
}


#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<u32>> {
    Problem::factory(for_test)(17).read_input_as_mapped_lines(|l| parse_input!(l,u32))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_17::main::{compute_number_of_combinations, compute_number_of_minimum_combinations, parse_input, part1, part2};

    #[test]
    fn day17_part1_test() {
        let containers = [20_u32, 15, 10, 5, 5];
        let actual = compute_number_of_combinations(&containers, 25);
        assert_eq!(actual, 4)
    }

    #[test]
    fn day17_part2_test() {
        let containers = [20_u32, 15, 10, 5, 5];
        let actual = compute_number_of_minimum_combinations(&containers, 25);
        assert_eq!(actual, 3)
    }
}