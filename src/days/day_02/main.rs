use std::str::FromStr;
use std::result::Result as StdResult;
use crate::{parse_input, Part};
use crate::problem::{Problem, AOCResult};


#[allow(dead_code)]
pub fn day02_launch(part: Part) -> AOCResult<String> {
    let presents = parse_input(false)?;
    match part {
        Part::Part1 => part1(&presents),
        Part::Part2 => part2(&presents)
    }
}

fn part1(present:&Vec<Present>) -> AOCResult<String> {
    let sum = present.iter().fold(0,|area,present| area+present.compute_required_wrapping_area());
    return Ok(sum.to_string())
}

fn part2(present:&Vec<Present>) -> AOCResult<String> {
    let sum = present.iter().fold(0,|area,present| area+present.compute_ribbon_length());
    return Ok(sum.to_string())
}


#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<Present>> {
    Problem::factory(for_test)(2).read_input_as_mapped_lines(|l| parse_input!(l,Present))
}


struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {

    fn compute_required_wrapping_area(&self) -> u32 {
        let area1 = self.length*self.width;
        let area2 = self.length*self.height;
        let area3 = self.width*self.height;
        return (area1+area2+area3)*2 + (area1.min(area2).min(area3))
    }

    fn compute_volume(&self) -> u32 {
        return self.height * self.width * self.length;
    }

    fn compute_smallest_side_perimeter(&self) -> u32 {
        let max_size = self.length.max(self.width).max(self.height);

        return (self.length + self.width + self.height - max_size)*2
    }

    fn compute_ribbon_length(&self) -> u32 {
        return self.compute_volume()+self.compute_smallest_side_perimeter();
    }
}

impl FromStr for Present {
    type Err = String;

    fn from_str(present_as_str: &str) -> StdResult<Self, Self::Err> {
        let sizes:Vec<&str> = present_as_str.split("x").collect();
        let length = parse_dimension(&sizes, 0)?;
        let width = parse_dimension(&sizes, 1)?;
        let height = parse_dimension(&sizes, 2)?;
        Ok(Present{length,width,height})
    }
}


fn parse_dimension(sizes:&Vec<&str>, index:usize) -> StdResult<u32,String> {
    match sizes.get(index) {
        Some(str) => str.parse::<u32>().map_err(|e| e.to_string()),
        None => StdResult::Err(format!("No dimension at index {}",index))
    }
}
