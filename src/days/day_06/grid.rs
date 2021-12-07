use crate::days::day_06::command::Command;
use crate::days::day_06::rectangle::Rectangle;

const GRID_SIZE:usize = 1000;

pub struct Grid {
    brightness:[u8;1000000],
}

impl Grid {
    pub fn count_brightness(&self) -> usize {
        self.brightness.iter().map(|u| *u as usize).sum()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid{ brightness:[0;1000000]}
    }
}

impl Grid {

    pub fn apply_command_part1(&mut self, command:&Command) {
        let (rect,action):(&Rectangle, fn(u8) -> u8) = match command {
            Command::Toggle(rect) => (rect, |b:u8| 1-b),
            Command::TurnOn(rect) => (rect, |_b:u8| 1),
            Command::TurnOff(rect) => (rect, |_b:u8| 0),
        };

        rect.indices(GRID_SIZE).for_each(|i| {
            let current = self.brightness[i];
            self.brightness[i] = action(current)
        } )

    }

    pub fn apply_command_part2(&mut self, command:&Command) {
        let (rect,action):(&Rectangle, fn(u8) -> u8) = match command {
            Command::Toggle(rect) => (rect, |b:u8| b+2),
            Command::TurnOn(rect) => (rect, |b:u8| b+1),
            Command::TurnOff(rect) => (rect, |b:u8| b.max(1)-1),
        };

        rect.indices(GRID_SIZE).for_each(|i| {
            let current = self.brightness[i];
            self.brightness[i] = action(current)
        } )

    }
}