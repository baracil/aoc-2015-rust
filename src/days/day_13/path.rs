use std::str::FromStr;
use crate::parse_input;

pub struct Path {
    name:String,
    neighbor:String,
    happiness:i32,
}

impl Path {

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn neighbor(&self) -> &str {
        &self.neighbor
    }
    pub fn happiness(&self) -> i32 {
        self.happiness
    }
}


impl FromStr for Path {
    type Err = String;

    // 0      1    2    3   4        5     6   7      8   9  10
    //Alice would gain 89 happiness units by sitting next to Eric.
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let token:Vec<&str> = line.split(" ").collect();
        let name = token[0].to_string();
        let action = token[2];
        let level = parse_input!(token[3],i32);
        let neighbor = token[10];
        let happiness = match action {
            "lose" => -level,
            "gain" => level,
            _ => panic!("Dunno this action {}",action)
        };
        Ok(Path { name, neighbor:neighbor[0..neighbor.len()-1].to_string(), happiness })
    }


}