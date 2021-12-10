use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::thread::sleep;

use crate::days::day_07::circuit::Evaluation;
use crate::days::day_07::gate::{Gate, Parameter};
use crate::days::day_07::gate::Parameter::{Literal, Wire};

pub struct Node {
    gate: Gate
}

