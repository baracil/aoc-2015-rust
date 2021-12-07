use std::str::FromStr;
use regex::Regex;


pub enum Parameter {
    WIRE(String),
    LITERAL(u8),
}

pub enum Gate {
    AND(Parameter, Parameter,String),
    OR(Parameter, Parameter, String),
    LSHIFT(Parameter, Parameter, String),
    RSHIFT(Parameter, Parameter, String),
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err("TODO".to_string())
    }
}