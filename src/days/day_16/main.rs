use crate::days::day_16::aunt::Aunt;
use crate::problem::{AOCResult, Problem};
use crate::{parse_input, Part};
use phf::phf_map;

static CLUES: phf::Map<&'static str, u32> = phf_map! {
        "children" => 3,
        "cats" => 7,
        "samoyeds" => 2,
        "pomeranians" => 3,
        "akitas" => 0,
        "vizslas" => 0,
        "goldfish" => 5,
        "trees" => 3,
        "cars" => 2,
        "perfumes" => 1,
};

#[allow(dead_code)]
pub fn day16_launch(part: Part) -> AOCResult<String> {
    let aunts = parse_input(false)?;
    match part {
        Part::Part1 => part1(&aunts),
        Part::Part2 => part2(&aunts),
    }
}

fn part1(aunts: &[Aunt]) -> AOCResult<String> {
    solve(aunts, |a| a.matches_part1(&CLUES))
}

fn part2(aunts: &[Aunt]) -> AOCResult<String> {
    solve(aunts, |a| a.matches_part2(&CLUES))
}

fn solve<F>(aunts: &[Aunt], predicate: F) -> AOCResult<String>
where
    F: FnMut(&&Aunt) -> bool,
{
    aunts
        .iter()
        .find(predicate)
        .map(|a| a.id())
        .map(|id| id.to_string())
        .ok_or_else(|| "Cannot find any aunt".to_string())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<Aunt>> {
    Problem::factory(for_test)(16).read_input_as_mapped_lines(|l| parse_input!(l, Aunt))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {

}
