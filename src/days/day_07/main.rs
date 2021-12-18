use crate::days::day_07::circuit::Circuit;
use crate::days::day_07::gate::Gate;
use crate::problem::{AOCResult, Problem};
use crate::Part;

#[allow(dead_code)]
pub fn day07_launch(part: Part) -> AOCResult<String> {
    let gates = parse_input(false)?;

    match part {
        Part::Part1 => part1(&gates),
        Part::Part2 => part2(&gates),
    }
}

fn part1(gates: &[Gate]) -> AOCResult<String> {
    let circuit = circuit_part1(gates)?;
    circuit
        .get_value("a")
        .ok_or_else(|| "No value".to_string())
        .map(|u| u.to_string())
}

fn circuit_part1(gates: &[Gate]) -> AOCResult<Circuit> {
    let mut circuit = Circuit::create(gates);
    circuit.evaluate();
    Ok(circuit)
}

fn circuit_part2(gates: &[Gate]) -> AOCResult<Circuit> {
    let mut circuit = Circuit::create(gates);
    circuit.evaluate();
    let a = circuit.get_value("a").ok_or("Fail to evaluate circuit")?;

    circuit.clear_registry();
    circuit.set_value("b", a);
    circuit.evaluate();

    Ok(circuit)
}

fn part2(gates: &[Gate]) -> AOCResult<String> {
    let circuit = circuit_part2(gates)?;
    circuit
        .get_value("a")
        .ok_or_else(|| "No value".to_string())
        .map(|u| u.to_string())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<Gate>> {
    Problem::factory(for_test)(7).read_input_as_mapped_lines(|l| l.parse::<Gate>().unwrap())
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_07::main::{circuit_part1, parse_input, part2};

    #[test]
    fn day07_part1_test() {
        let _input = parse_input(true).unwrap();
        let result = circuit_part1(&_input).unwrap();

        assert_eq!(result.get_value("d"), Some(72));
        assert_eq!(result.get_value("e"), Some(507));
        assert_eq!(result.get_value("f"), Some(492));
        assert_eq!(result.get_value("g"), Some(114));
        assert_eq!(result.get_value("h"), Some(65412));
        assert_eq!(result.get_value("i"), Some(65079));
        assert_eq!(result.get_value("x"), Some(123));
        assert_eq!(result.get_value("y"), Some(456));
    }

    #[test]
    #[ignore]
    fn day07_part2_test() {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result, "")
    }
}
