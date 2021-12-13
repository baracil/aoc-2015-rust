use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::days::day_07::circuit::Evaluation;
use crate::days::day_07::gate::Gate::{And, LShift, Not, Or, RShift, Set};
use crate::days::day_07::gate::Parameter::{Literal, Wire};
use crate::problem::AOCResult;


#[derive(Debug, Clone)]
pub enum Parameter {
    Wire(String),
    Literal(u16),
}

impl Parameter {
    fn get_value(&self, registries:&HashMap<String,u16>) -> Option<u16> {
        match &self {
            Wire(w) => registries.get(w).cloned(),
            Literal(l) => Some(*l)
        }
    }

}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Wire(w) => write!(f,"{}",w),
            Literal(v) => write!(f,"{}",v)
        }
    }
}

impl Parameter {
    pub fn wire_name(&self) -> Option<&str> {
        match &self {
            Wire(w) => Some(w),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub enum Gate {
    And(Parameter, Parameter, String),
    Or(Parameter, Parameter, String),
    LShift(Parameter, Parameter, String),
    RShift(Parameter, Parameter, String),
    Set(Parameter, String),
    Not(Parameter, String),
}

impl Gate {
    pub fn wires(&self) -> Vec<&str> {
        let param: Vec<&Parameter> = match &self {
            Gate::And(p1, p2, _)
            | Gate::Or(p1, p2, _)
            | Gate::LShift(p1, p2, _)
            | Gate::RShift(p1, p2, _) =>
                if p1.wire_name() == p2.wire_name() { vec![p1] } else { vec![p1, p2] },
            Gate::Set(p, _)
            | Gate::Not(p, _) => vec![p]
        };

        param.iter().filter_map(|p| p.wire_name()).collect()
    }

    pub fn _value1(&self) -> Option<u16> {
        match self {
            And(Literal(v), _, _)
            | Or(Literal(v), _, _)
            | LShift(Literal(v), _, _)
            | RShift(Literal(v), _, _)
            | Set(Literal(v), _)
            | Not(Literal(v), _) => Some(*v),
            _ => None
        }
    }

    pub fn _value2(&self) -> Option<u16> {
        match self {
            And(_, Literal(v), _)
            | Or(_, Literal(v), _)
            | LShift(_, Literal(v), _)
            | RShift(_, Literal(v), _) => Some(*v),
            _ => None
        }
    }
}

impl FromStr for Parameter {
    type Err = String;

    fn from_str(par: &str) -> Result<Self, Self::Err> {
        Ok(par.parse::<u16>()
            .map(Literal)
            .unwrap_or_else(|_| Wire(par.to_string())))
    }
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = line.trim().split(' ').collect();

        match tokens.len() {
            5 => Self::parse_gate(&tokens),
            4 => Self::parse_not(&tokens),
            3 => Self::parse_set(&tokens),
            n => Err(format!("Cannot parse command '{}' nb-tokens={}", line, n))
        }
    }
}

impl Gate {
    fn parse_gate(tokens: &[&str]) -> AOCResult<Gate> {
        let par1 = tokens[0].parse::<Parameter>()?;
        let par2 = tokens[2].parse::<Parameter>()?;
        let result = tokens[4].to_string();

        let gate = match tokens[1] {
            "OR" => Or(par1, par2, result),
            "AND" => And(par1, par2, result),
            "LSHIFT" => LShift(par1, par2, result),
            "RSHIFT" => RShift(par1, par2, result),
            _ => return Err(format!("Cannot parse gate '{}'", tokens[1]))
        };

        Ok(gate)
    }

    fn parse_not(tokens: &[&str]) -> AOCResult<Gate> {
        let parameter = tokens[1].parse::<Parameter>()?;
        let wire = tokens[3];
        Ok(Not(parameter, wire.to_string()))
    }
    fn parse_set(tokens: &[&str]) -> AOCResult<Gate> {
        let parameter = tokens[0].parse::<Parameter>()?;
        let wire = tokens[2];
        Ok(Set(parameter, wire.to_string()))
    }
}



impl Gate {

    pub fn evaluate(&self, registries:&HashMap<String,u16>) -> Option<Evaluation> {
        match &self {
            Gate::And(p1,p2,w) => Gate::evaluate_biop(|v1,v2| v1&v2, p1,p2,w,registries),
            Gate::Or(p1,p2,w) => Gate::evaluate_biop(|v1,v2| v1|v2, p1,p2,w,registries),
            Gate::LShift(p1,p2,w) => Gate::evaluate_biop(|v1,v2| v1<<v2, p1,p2,w,registries),
            Gate::RShift(p1,p2,w) => Gate::evaluate_biop(|v1,v2| v1>>v2, p1,p2,w,registries),

            Gate::Set(p,w) => Gate::evaluate_unop(|v| v, p, w,registries),
            Gate::Not(p,w) => Gate::evaluate_unop(|v| !v, p, w,registries),
        }
    }

    pub fn evaluate_biop(operator:fn(u16, u16) -> u16, p1:&Parameter, p2:&Parameter, wire:&str, registries:&HashMap<String,u16>) -> Option<Evaluation> {
        let v1 = p1.get_value(registries);
        let v2 = p2.get_value(registries);
        match (v1,v2) {
            (Some(v1),Some(v2)) => Some(Evaluation::with(wire.to_string(),operator(v1, v2))),
            _ => None
        }
    }
    pub fn evaluate_unop(operator:fn(u16) -> u16, p:&Parameter, wire:&str, registries:&HashMap<String,u16>) -> Option<Evaluation> {
        p.get_value(registries)
            .map(|v| Evaluation::with(wire.to_string(), operator(v)))
    }


}