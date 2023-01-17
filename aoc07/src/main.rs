use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    Ok(())
}

#[derive(Debug)]
enum Operation {
    Set(u16),
    SetReg(String),
    And(String, String),
    Or(String, String),
    LShift(String, u16),
    RShift(String, u16),
    Not(String),
}

impl From<&str> for Operation {
    fn from(item: &str) -> Self {
        let parts: Vec<_> = item.split_whitespace().collect();
        match parts.len() {
            3 => {
                if let Ok(p) = parts[0].parse::<u16>() {
                    Self::Set(p)
                } else {
                    Self::SetReg(parts[0].to_string())
                }
            }
            4 => Self::Not(parts[1].parse().unwrap()),
            5 => match parts[1] {
                "AND" => Self::And(parts[0].parse().unwrap(), parts[2].parse().unwrap()),
                "OR" => Self::Or(parts[0].parse().unwrap(), parts[2].parse().unwrap()),
                "LSHIFT" => Self::LShift(parts[0].parse().unwrap(), parts[2].parse().unwrap()),
                "RSHIFT" => Self::RShift(parts[0].parse().unwrap(), parts[2].parse().unwrap()),
                _ => panic!("unexpected operator: {}", parts[1]),
            },
            _ => panic!("unexpected number of parts: {}", parts.len()),
        }
    }
}

#[derive(Debug)]
struct Circuit {
    commands: HashMap<String, Operation>,
    memory: RefCell<HashMap<String, u16>>,
}

impl Circuit {
    fn new(commands: HashMap<String, Operation>) -> Self {
        Self {
            commands,
            memory: HashMap::new().into(),
        }
    }

    fn get_value(&self, reg: &str) -> u16 {
        if let Some(v) = self.memory.borrow().get(reg) {
            return *v;
        }

        let op = self.commands.get(reg).unwrap();
        let result = match &op {
            Operation::Set(val) => *val,
            Operation::SetReg(src) => {
                let val = self.get_value(&src.to_string());
                val
            }
            Operation::Or(x, y) => {
                let x_val = if let Ok(x) = x.parse() {
                    x
                } else {
                    self.get_value(&x)
                };
                let y_val = if let Ok(y) = y.parse() {
                    y
                } else {
                    self.get_value(&y)
                };
                let or_val = x_val | y_val;
                or_val
            }
            Operation::And(x, y) => {
                let x_val = if let Ok(x) = x.parse() {
                    x
                } else {
                    self.get_value(&x)
                };
                let y_val = if let Ok(y) = y.parse() {
                    y
                } else {
                    self.get_value(&y)
                };
                let and_val = x_val & y_val;
                and_val
            }
            Operation::LShift(x, s) => {
                let x_val = self.get_value(&x);
                let shift_val = x_val << s;
                shift_val
            }
            Operation::RShift(x, s) => {
                let x_val = self.get_value(&x);
                let shift_val = x_val >> s;
                shift_val
            }
            Operation::Not(src) => {
                let val = self.get_value(&src);
                let not_val = !val;
                not_val
            }
        };
        self.memory.borrow_mut().insert(reg.to_string(), result);
        return result;
    }
}

fn part1(input: &str) -> Result<()> {
    let commands: HashMap<String, Operation> = input
        .lines()
        .map(|l| {
            let op = l.into();
            let (_, reg) = l.split_once(" -> ").unwrap();
            (reg.to_string(), op)
        })
        .collect();
    let mut circuit = Circuit::new(commands);

    let val = circuit.get_value("a");
    println!("A: {}", val);

    circuit.memory.borrow_mut().clear();
    circuit.memory.borrow_mut().insert("b".to_string(), val);
    let val = circuit.get_value("a");
    println!("A: {}", val);

    Ok(())
}
