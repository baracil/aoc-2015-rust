use std::collections::HashMap;
use std::result::Result as StdResult;
use crate::days::day_03::Direction::{East, North, South, West};
use crate::{Part};
use crate::problem::{Problem, AOCResult};


enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq,Hash, Default, Copy, Clone)]
struct Position {
    x:i32,
    y:i32,
}

struct City {
    visited_houses:HashMap<Position,u32>,
}

impl Default for City {
    fn default() -> Self {
        City{visited_houses:HashMap::new()}
    }
}

impl City {
    fn visit_house_at(&mut self, position:&Position) {
        let current_count = self.visited_houses.get(&position).unwrap_or(&0) + 1;
        self.visited_houses.insert(*position,current_count);
    }

    fn number_of_visited_houses(&self) -> usize {
        self.visited_houses.len()
    }
}


impl Position {

    fn translated(self,dx:i32,dy:i32) -> Self {
        Position{x:self.x+dx, y:self.y+dy}
    }

    fn displaced(self, direction:&Direction) -> Self {
        match direction  {
            North => self.translated(0,1),
            South => self.translated(0,-1),
            East => self.translated(1,0),
            West => self.translated(-1,0)
        }
    }
}


impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> StdResult<Self, Self::Error> {
        match value {
            '^' => Ok(North),
            '>' => Ok(East),
            'v' => Ok(South),
            '<' => Ok(West),
            _ => Err(format!("Cannot convert '{}' to Direction",value))
        }
    }
}


#[allow(dead_code)]
pub fn day03_launch(part: Part) -> AOCResult<String> {
    let directions = parse_input(false)?;
    match part {
        Part::Part1 => part1(&directions),
        Part::Part2 => part2(&directions)
    }
}



fn part1(directions:&Vec<Direction>) -> AOCResult<String> {
    let mut city = City::default();

    let position = Position::default();
    city.visit_house_at(&position);

    directions.iter().fold(position, |pos, direction| {
        let pos = pos.displaced(&direction);
        city.visit_house_at(&pos);
        pos
    });

    Ok(city.number_of_visited_houses().to_string())
}

fn part2(directions:&Vec<Direction>) -> AOCResult<String> {
    let mut city = City::default();

    let mut santa_position = Position::default();
    let mut robo_santa_position = Position::default();


    city.visit_house_at(&santa_position);
    city.visit_house_at(&robo_santa_position);

    let mut flag = true;
    for direction in directions {
        if flag {
            santa_position = santa_position.displaced(direction);
            city.visit_house_at(&santa_position);
        } else {
            robo_santa_position = robo_santa_position.displaced(direction);
            city.visit_house_at(&robo_santa_position);
        }
        flag = !flag;
    };

    Ok(city.number_of_visited_houses().to_string())
}




#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<Vec<Direction>> {
    Ok(Problem::factory(for_test)(3).read_input()?.chars().map(|c| Direction::try_from(c).unwrap()).collect())
}
