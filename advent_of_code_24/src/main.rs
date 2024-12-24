use anyhow::Result;
use itertools::Itertools;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let output = calculate_output("input.txt")?;
    println!("Output: {output}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn calculate_output(filename: &str) -> Result<u64> {
    let mut device = parse_file(filename)?;

    device.initialize();

    while !device.finished() {
        device.calculate();
    }

    Ok(device.output())
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<Device> {
    let data = read_to_string(filename)?;

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut gates = Vec::new();

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        if let Some((name, value)) = line.split_once(':') {
            inputs.push(Input {
                name: name.to_string(),
                value: value.trim().parse()?,
            });
        } else {
            let (gate, output) = line.split_once("->").expect("No valid gate");
            let gate_values: Vec<&str> = gate.split_whitespace().collect();

            let output = Output::new(output.trim());
            let gate = Gate {
                inputs: (
                    Output::new(gate_values[0].trim()),
                    Output::new(gate_values[2].trim()),
                ),
                output: output.clone(),
                op: GateOp::from(gate_values[1]),
            };

            gates.push(gate);
            outputs.push(output);
        }
    }

    Ok(Device {
        inputs,
        outputs,
        gates,
    })
}

struct Device {
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    gates: Vec<Gate>,
}

impl Device {
    fn initialize(&mut self) {
        for input in &self.inputs {
            for gate in &mut self.gates {
                if let Some(gate_input) = gate.get_input(&input.name) {
                    gate_input.value = Some(input.value);
                }
            }
        }
    }

    fn finished(&self) -> bool {
        self.outputs.iter().all(|output| output.value.is_some())
    }

    fn calculate(&mut self) {
        let mut changed_outputs = Vec::new();
        for gate in self.gates.iter_mut().filter(|gate| {
            gate.inputs.0.value.is_some()
                && gate.inputs.1.value.is_some()
                && gate.output.value.is_none()
        }) {
            let output = Some(
                gate.op
                    .calculate(gate.inputs.0.value.unwrap(), gate.inputs.1.value.unwrap()),
            );

            gate.output.value = output;
            changed_outputs.push(gate.output.clone());
        }

        for output in changed_outputs {
            if let Some(device_output) = self.outputs.iter_mut().find(|o| o.name == output.name) {
                device_output.value = output.value;
            }

            for input in self
                .gates
                .iter_mut()
                .filter_map(|gate| gate.get_input(&output.name))
            {
                input.value = output.value;
            }
        }
    }

    fn output(&self) -> u64 {
        let outputs = self
            .outputs
            .iter()
            .filter(|output| output.name.starts_with('z'))
            .sorted_by_key(|output| output.name.clone());

        let mut output = 0;
        for (i, v) in outputs.enumerate() {
            output += (v.value.unwrap() as u64) << i;
        }

        output
    }
}

#[derive(Debug, Clone)]
struct Input {
    name: String,
    value: u8,
}

#[derive(Debug, Clone)]
struct Output {
    name: String,
    value: Option<u8>,
}

impl Output {
    fn new(name: &str) -> Output {
        Output {
            name: name.to_string(),
            value: None,
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    inputs: (Output, Output),
    output: Output,
    op: GateOp,
}

impl Gate {
    fn get_input(&mut self, name: &str) -> Option<&mut Output> {
        if self.inputs.0.name == name {
            Some(&mut self.inputs.0)
        } else if self.inputs.1.name == name {
            Some(&mut self.inputs.1)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum GateOp {
    And,
    Or,
    Xor,
}

impl From<&str> for GateOp {
    fn from(value: &str) -> Self {
        match value {
            "AND" => GateOp::And,
            "OR" => GateOp::Or,
            "XOR" => GateOp::Xor,
            _ => panic!("No valid operation"),
        }
    }
}

impl GateOp {
    fn calculate(&self, a: u8, b: u8) -> u8 {
        match self {
            GateOp::And => a & b,
            GateOp::Or => a | b,
            GateOp::Xor => a ^ b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallir_a() {
        let result = calculate_output("input_smaller.txt");
        assert!(result.is_ok());
        assert_eq!(4, result.unwrap())
    }

    #[test]
    fn test_small_a() {
        let result = calculate_output("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(2024, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = calculate_output("input.txt");
        assert!(result.is_ok());
        assert_eq!(1579939, result.unwrap())
    }

    #[test]
    #[ignore = "reason"]
    fn test_small_b() {
        let result = calculate_similarity_score("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(31, result.unwrap())
    }

    #[test]
    #[ignore = "reason"]
    fn test_input_b() {
        let result = calculate_similarity_score("input.txt");
        assert!(result.is_ok());
        assert_eq!(20351745, result.unwrap())
    }
}
