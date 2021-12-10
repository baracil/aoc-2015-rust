use std::str::FromStr;
use crate::days::day_08::string_info::Char::{Hexa, Literal};
use crate::days::day_08::string_info::State::Default;


#[derive(Copy, Clone,Debug)]
pub enum Char {
    Literal(char),
    Hexa(u8),
}


#[derive(Copy, Clone, Debug)]
pub enum State {
    Default,
    Escape,
    HexaEscape(u8, u8),
}

impl State {
    pub fn push(&self, c: char) -> (Self, Option<Char>) {
        use State::*;

        match (self, c) {
            (Default, '\\') => (Escape, None),
            (Default, _) => (Default, Some(Literal(c))),

            (Escape, '\\' | '\"') => (Default, Some(Literal(c))),
            (Escape, 'x') => (HexaEscape(0, 0), None),

            (HexaEscape(0, v), _) => (HexaEscape(1, v + to_hexa(c) * 16), None),
            (HexaEscape(1, v), _) => (Default, Some(Hexa(v + to_hexa(c)))),

            (_, _) => panic!("Cannot get next state state={:?} c={}",self, c),
        }
    }
}

pub fn to_hexa(c: char) -> u8 {
    match c {
        '0'..='9' => (c as u8 - '0' as u8),
        'a'..='f' => (c as u8 - 'a' as u8),
        'A'..='F' => (c as u8 - 'A' as u8),
        _ => panic!("Cannot convert value to hexa {}", c)
    }
}


#[derive(Debug)]
pub struct StringInfo {
    string_length: usize,
    content: Vec<Char>,
}


impl StringInfo {
    pub fn memory_overhead(&self) -> usize {
        self.string_length - self.content.len() +2
    }
}


impl FromStr for StringInfo {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let string_length = line.len();
        let mut chars = Vec::new();

        let mut state = Default;
        for c in line.chars() {
            let (next_state, c) = state.push(c);

            for c in c {
                chars.push(c);
            }
            state = next_state;
        };

        Ok(StringInfo { string_length, content: chars })
    }
}