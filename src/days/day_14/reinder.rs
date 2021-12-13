use std::str::FromStr;
use crate::parse_input;

pub struct Reinder {
    speed:u32,
    fly_duration:u32,
    rest_duration:u32
}

impl FromStr for Reinder {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // 0       1   2  3  4    5   6  7        8  9    10   11   12  13
        //Prancer can fly 9 km/s for 12 seconds, but then must rest for 97 seconds.
        let tokens:Vec<&str> = line.split(' ').collect();

        let speed = parse_input!(tokens[3],u32);
        let fly_duration = parse_input!(tokens[6],u32);
        let rest_duration = parse_input!(tokens[13],u32);

        Ok(Reinder {  speed, fly_duration, rest_duration })
    }
}

impl Reinder {

    pub fn distance(&self, time:u32) -> u32{
        let full_duration = self.fly_duration + self.rest_duration;

        let nb_cycles = time.div_euclid(full_duration);
        let remaining = time.rem_euclid(full_duration);

        self.speed*(nb_cycles*self.fly_duration + self.fly_duration.min(remaining))
    }
}
