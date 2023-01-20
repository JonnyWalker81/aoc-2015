use std::{
    collections::HashMap,
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Hlf(String),
    Tpl(String),
    Inc(String),
    Jmp(i64),
    Jie(String, i64),
    Jio(String, i64),
}

impl From<&str> for Instruction {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split_whitespace().collect();
        if parts.len() == 3 {
            let sign: i64 = if parts[2].starts_with("+") { 1 } else { -1 };
            match parts[0] {
                "jie" => Self::Jie(
                    parts[1].trim_end_matches(',').to_string(),
                    parts[2][1..].parse::<i64>().unwrap() * sign,
                ),
                "jio" => Self::Jio(
                    parts[1].trim_end_matches(',').to_string(),
                    parts[2][1..].parse::<i64>().unwrap() * sign,
                ),
                _ => panic!("shoud not get here: {:?}", parts),
            }
        } else {
            match parts[0] {
                "hlf" => Self::Hlf(parts[1].to_string()),
                "tpl" => Self::Tpl(parts[1].to_string()),
                "inc" => Self::Inc(parts[1].to_string()),
                "jmp" => {
                    let sign = if parts[1].starts_with("+") { 1 } else { -1 };
                    Self::Jmp(parts[1][1..].parse::<i64>().unwrap() * sign)
                }
                _ => panic!("should not get here: {:?}", parts),
            }
        }
    }
}

#[derive(Debug)]
struct Cpu {
    registers: HashMap<String, i64>,
    instructions: Vec<Instruction>,
    pc: i64,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            registers: HashMap::new(),
            instructions,
            pc: 0,
        }
    }

    fn run(&mut self) {
        loop {
            let ins = &self.instructions[self.pc as usize];
            match ins {
                Instruction::Hlf(r) => {
                    *self.registers.entry(r.to_string()).or_insert(0) /= 2;
                    self.pc += 1;
                }
                Instruction::Tpl(r) => {
                    *self.registers.entry(r.to_string()).or_insert(0) *= 3;
                    self.pc += 1;
                }
                Instruction::Inc(r) => {
                    *self.registers.entry(r.to_string()).or_insert(0) += 1;
                    self.pc += 1;
                }
                Instruction::Jmp(o) => {
                    self.pc += o;
                }
                Instruction::Jie(r, o) => {
                    let mut did_jump = false;
                    if let Some(v) = self.registers.get(r) {
                        if *v % 2 == 0 {
                            self.pc += o;
                            did_jump = true;
                        }
                    }

                    if !did_jump {
                        self.pc += 1;
                    }
                }
                Instruction::Jio(r, o) => {
                    let mut did_jump = false;
                    if let Some(v) = self.registers.get(r) {
                        if *v == 1 {
                            self.pc += o;
                            did_jump = true;
                        }
                    }

                    if !did_jump {
                        self.pc += 1;
                    }
                }
            }

            if self.pc as usize >= self.instructions.len() || self.pc < 0 {
                break;
            }
        }
    }

    fn get(&self, reg: &str) -> i64 {
        self.registers.get(reg).unwrap_or(&-1).clone()
    }
}

fn part1(input: &str) -> Result<()> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect();
    let mut cpu = Cpu::new(instructions);

    // println!("{cpu:?}");
    cpu.run();

    let val = cpu.get("b");
    println!("b: {}", val);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect();
    let mut cpu = Cpu::new(instructions);
    cpu.registers.insert("a".to_string(), 1);

    // println!("{cpu:?}");
    cpu.run();

    let val = cpu.get("b");
    println!("b: {}", val);

    Ok(())
}
