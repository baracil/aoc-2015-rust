use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use crate::parse_input;

#[derive(Eq,Clone)]
pub struct Path {
    city1:String,
    city2:String,
    distance:u32,
}

impl FromStr for Path {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let tokens:Vec<&str> = line.split(" ").collect();
        Ok(Path::new(tokens[0], tokens[2], parse_input!(tokens[4],u32)))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.city1.eq(&other.city1) && self.city2.eq(&other.city2)
    }
}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.city1.hash(state);
        self.city2.hash(state);
    }
}

impl Path {

    pub fn new(city1:&str, city2:&str, distance:u32) -> Self {
        match city1.cmp(city2) {
            Ordering::Less | Ordering::Equal => Path{city1:city1.to_string(),city2:city2.to_string(),distance},
            Ordering::Greater => Path{city1:city2.to_string(),city2:city1.to_string(),distance}
        }
    }


    pub fn city1(&self) -> &str {
        &self.city1
    }
    pub fn city2(&self) -> &str {
        &self.city2
    }
    pub fn distance(&self) -> u32 {
        self.distance
    }
}