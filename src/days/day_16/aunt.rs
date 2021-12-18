use crate::parse_input;
use itertools::Itertools;
use phf::Map;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Aunt {
    id: u32,
    properties: HashMap<String, u32>,
}

impl Aunt {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn matches_part1(&self, clues: &Map<&'static str, u32>) -> bool {
        self.properties
            .iter()
            .all(|(property, number)| clues.get(property) == Some(number))
    }

    //greater cats and trees
    //fewer pomeranians and goldfis

    pub fn matches_part2(&self, clues: &Map<&'static str, u32>) -> bool {
        self.properties.iter().all(|(property, number)| {
            if let Some(number_from_clue) = clues.get(property) {
                match property.as_ref() {
                    "cats" | "trees" => number > number_from_clue,
                    "pomeranians" | "goldfish" => number < number_from_clue,
                    _ => number == number_from_clue,
                }
            } else {
                false
            }
        })
    }
}

impl FromStr for Aunt {
    type Err = String;

    //Sue 443: akitas: 3, perfumes: 3, cats: 9
    //akitas: 3, perfumes: 3, cats: 9
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_once(": ").ok_or("Cannot parse aunt")?;

        let id = parse_input!(tokens.0.split_at(4).1, u32);

        let properties = tokens
            .1
            .split(|c: char| c == ',' || c == ' ' || c == ':')
            .filter(|s| !s.is_empty())
            .tuples::<(_, _)>()
            .map(|(kind, number)| (kind.to_string(), parse_input!(number, u32)))
            .collect::<HashMap<String, u32>>();

        Ok(Aunt { id, properties })
    }
}
