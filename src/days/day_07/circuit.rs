use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::usize;
use crate::days::day_07::gate::{Gate};

#[derive(Clone, Debug)]
pub struct Evaluation {
    wire: String,
    value: u16,
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.wire, self.value)
    }
}

impl Evaluation {
    pub fn with(wire: String, value: u16) -> Self {
        Evaluation { wire, value }
    }

    pub fn wire(&self) -> &str {
        &self.wire
    }

    pub fn _value(&self) -> u16 {
        self.value
    }
}

pub struct Circuit {
    registry: HashMap<String, u16>,
    gates: Vec<Gate>,
    gates_by_wire: HashMap<String, HashSet<usize>>,
}


impl Circuit {
    pub fn get_value(&self, wire: &str) -> Option<u16> {
        self.registry.get(wire).cloned()
    }

    pub fn create(gates: &Vec<Gate>) -> Self {
        let gates = gates.clone();
        let mut gates_by_wire = HashMap::<String, HashSet<usize>>::new();

        for (idx, gate) in gates.iter().enumerate() {
            for wire in gate.wires() {
                gates_by_wire
                    .entry(wire.to_string())
                    .or_insert_with(|| HashSet::<usize>::new())
                    .insert(idx);
            }
        }

        return Circuit { registry: HashMap::new(), gates, gates_by_wire };
    }

    pub fn clear_registry(&mut self) {
        self.registry.clear();
    }

    pub fn set_value(&mut self, wire:&str, value:u16) {
        self.registry.insert(wire.to_string(),value);
    }

    pub fn evaluate(&mut self) {
        let mut values = Vec::<Evaluation>::new();
        for node in &self.gates {
            for x in node.evaluate(&self.registry) {
                values.push(x)
            }
        };

        self.evaluate_rec(&values);
    }

    fn evaluate_rec(&mut self, evaluations: &Vec<Evaluation>) {

        let mut new_evaluations = Vec::<Evaluation>::new();

        for evaluation in evaluations {
            if self.registry.contains_key(evaluation.wire()) {
                continue
            }

            self.registry.insert(evaluation.wire.to_string(), evaluation.value);

            for indices in self.gates_by_wire.get(&evaluation.wire) {
                for index in indices {
                    let gate = &mut self.gates[*index];
                    for e in gate.evaluate(&self.registry) {
                        new_evaluations.push(e.clone())
                    }
                }
            }
        };

        if !new_evaluations.is_empty() {
            self.evaluate_rec(&new_evaluations)
        }
    }
}