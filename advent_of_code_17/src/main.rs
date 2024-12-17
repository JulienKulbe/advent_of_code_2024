use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let output = interpret_program("input.txt")?;
    println!("Output: {output}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn interpret_program(filename: &str) -> Result<String> {
    let mut program = parse_file(filename)?;
    program.run();

    Ok(program.output())
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<Program> {
    let data = read_to_string(filename)?;
    let lines: Vec<&str> = data.lines().collect();
    if lines.len() != 5 {
        bail!("Invalid number of lines");
    }

    let registers = Registers {
        a: get_register_value(lines[0])?,
        b: get_register_value(lines[1])?,
        c: get_register_value(lines[2])?,
    };
    let instructions = get_instructions(lines[4])?;

    Ok(Program::new(registers, instructions))
}
fn get_register_value(line: &str) -> Result<u64> {
    let (_, register) = line.split_once(':').context("Invalid Register line")?;
    let register = register.trim().parse()?;
    Ok(register)
}

fn get_instructions(line: &str) -> Result<Vec<Instruction>> {
    let (_, instructions) = line.split_once(':').context("Invalid Program line")?;
    let values: Vec<&str> = instructions.trim().split(',').collect();

    let mut instructions = Vec::new();
    for mut chunk in &values.iter().chunks(2) {
        let opcode = chunk.next().unwrap().parse()?;
        let operand = chunk.next().unwrap().parse()?;
        instructions.push(Instruction::new(opcode, operand));
    }

    Ok(instructions)
}

struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

struct Program {
    registers: Registers,
    instructions: Vec<Instruction>,
    index: usize,
    output: Vec<u64>,
}

impl Program {
    fn new(registers: Registers, instructions: Vec<Instruction>) -> Self {
        Program {
            registers,
            instructions,
            index: 0,
            output: Vec::new(),
        }
    }

    fn output(&self) -> String {
        self.output.iter().map(|v| v.to_string()).join(",")
    }

    fn run(&mut self) {
        while self.index < self.instructions.len() {
            let instruction = self.instructions[self.index];
            match instruction {
                Instruction::Adv(denominator) => {
                    let operand = self.combo_operand(denominator);
                    let divison = self.registers.a / (2_u64.pow(operand as u32));
                    self.registers.a = divison;
                }
                Instruction::Bxl(operand) => {
                    let result = self.registers.b ^ operand as u64;
                    self.registers.b = result;
                }
                Instruction::Bst(operand) => {
                    self.registers.b = self.combo_operand(operand) % 8;
                }
                Instruction::Jnz(operand) => {
                    if self.registers.a != 0 {
                        self.index = operand as usize;
                        continue;
                    }
                }
                Instruction::Bxc => {
                    let result = self.registers.b ^ self.registers.c;
                    self.registers.b = result;
                }
                Instruction::Out(operand) => {
                    let value = self.combo_operand(operand) % 8;
                    self.output.push(value);
                }
                Instruction::Bdv(denominator) => {
                    let operand = self.combo_operand(denominator);
                    let divison = self.registers.a / (2_u64.pow(operand as u32));
                    self.registers.b = divison;
                }
                Instruction::Cdv(denominator) => {
                    let operand = self.combo_operand(denominator);
                    let divison = self.registers.a / (2_u64.pow(operand as u32));
                    self.registers.c = divison;
                }
            }

            self.index += 1;
        }
    }

    fn combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.registers.a,
            5 => self.registers.b,
            6 => self.registers.c,
            _ => panic!("invalid combo operand"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc,
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

impl Instruction {
    fn new(opcode: u8, operand: u8) -> Instruction {
        match opcode {
            0 => Instruction::Adv(operand),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(operand),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc,
            5 => Instruction::Out(operand),
            6 => Instruction::Bdv(operand),
            7 => Instruction::Cdv(operand),
            _ => panic!("Invalid opcode"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = interpret_program("input_small.txt");
        assert!(result.is_ok());
        assert_eq!("4,6,3,5,6,3,5,2,1,0", result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = interpret_program("input.txt");
        assert!(result.is_ok());
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
