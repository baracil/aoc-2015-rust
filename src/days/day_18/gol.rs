use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};

pub struct Gol {
    lights: HashSet<Position>,
    grid_size:i32,
    with_corners:bool,
}

const NEIGHBORS: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];


#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub(crate) fn inside_square(&self, size: i32) -> bool {
        self.x >= 0 && self.x < size && self.y >= 0 && self.y < size
    }
}

impl Position {
    pub fn with(x:i32, y:i32) -> Self {
        Position{x,y}
    }

    pub fn neighbors(&self) -> impl Iterator<Item=Position> {
        let x = self.x;
        let y = self.y;
        NEIGHBORS.iter().map(move |(dx, dy)| Position { x: x + dx, y: y + dy })
    }
}


impl Display for Gol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in -1..self.grid_size+1 {
            f.write_char('\n')?;
            for x in -1..self.grid_size+1 {
                if self.lights.contains(&Position{x,y}) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
        }
        Ok(())
    }
}

impl Gol {

    pub fn parse(lines:&[String], with_corners:bool) -> Self {
        let grid_size = lines.len() as i32;
        let mut lights: HashSet<Position> = lines.iter()
            .enumerate()
            .flat_map(|(y, line)|
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| Position::with(x as i32, y as i32)))
            .collect();

        if with_corners {
            insert_corners(&mut lights, grid_size)
        }


        Gol{lights,grid_size, with_corners}
    }

    pub fn number_of_lights_on(&self) -> usize {
        self.lights.len()
    }

    pub fn perform_several_steps(&mut self, nb_step:usize) {
        for _ in 0..nb_step {
            self.perform_one_step()
        }
    }

    pub fn perform_one_step(&mut self) {
        let mut count = HashMap::<Position, u8>::new();

        self.lights.iter()
            .flat_map(|pos| pos.neighbors())
            .for_each(|pos| *count.entry(pos).or_insert(0) += 1);


        let mut new_lights: HashSet::<Position> = count.iter()
            .filter(|(pos, _)| pos.inside_square(self.grid_size))
            .filter(|(pos, nb)| {
                match nb {
                    3 => true,
                    2 => self.lights.contains(pos),
                    _ => false
                }
            })
            .map(|(pos, _)| *pos)
            .collect();


        if self.with_corners {
            insert_corners(&mut new_lights, self.grid_size);
        }


        self.lights = new_lights;
    }
}

fn insert_corners(positions:&mut HashSet<Position>, grid_size:i32) {
    positions.insert(Position{x:0,y:0});
    positions.insert(Position{x:0,y:grid_size-1});
    positions.insert(Position{x:grid_size-1,y:0});
    positions.insert(Position{x:grid_size-1,y:grid_size-1});
}