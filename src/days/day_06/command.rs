use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use crate::days::day_06::command::Command::{Toggle, TurnOff, TurnOn};
use crate::days::day_06::position::Position;
use crate::days::day_06::rectangle::Rectangle;

#[derive(Debug)]
pub enum Command {
    TurnOn(Rectangle),
    TurnOff(Rectangle),
    Toggle(Rectangle),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TurnOn(rect) => write!(f,"turn on {} through {}", rect.left_up(), rect.right_down()),
            TurnOff(rect) => write!(f,"turn off {} through {}", rect.left_up(), rect.right_down()),
            Toggle(rect) => write!(f,"toggle {} through {}", rect.left_up(), rect.right_down()),
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref COMMAND_PATTERN: Regex = Regex::new("(turn on|turn off|toggle) ([0-9]+,[0-9]+) through ([0-9]+,[0-9]+)").expect("Cannot build Regexp");
        }

        let cap = COMMAND_PATTERN.captures(line.trim()).ok_or(format!("Cannot parse command '{}'",line))?;

        let corner_1 = cap[2].parse::<Position>()?;
        let corner_2 = cap[3].parse::<Position>()?;
        let rectangle = Rectangle::with_corners(corner_1, corner_2);
        match &cap[1] {
            "turn on" => Ok(TurnOn(rectangle)),
            "turn off" => Ok(TurnOff(rectangle)),
            "toggle" => Ok(Toggle(rectangle)),
            _ => Err(format!("Cannot parse command '{}'",line))
        }
    }
}
