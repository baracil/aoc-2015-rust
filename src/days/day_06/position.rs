use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Position {
    pub fn at(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(point_as_string: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<usize> = point_as_string
            .trim()
            .split(',')
            .map(|s| {
                s.parse::<usize>()
                    .unwrap_or_else(|_| panic!("Cannot parse usize '{}'", s))
            })
            .collect();

        let x = *coordinates
            .get(0)
            .ok_or_else(|| format!("Cannot parse '{}'", point_as_string))?;
        let y = *coordinates
            .get(1)
            .ok_or_else(|| format!("Cannot parse '{}'", point_as_string))?;

        Ok(Position { x, y })
    }
}
