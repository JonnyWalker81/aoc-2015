use std::{
    collections::HashSet,
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    On,
    Off,
    Toggle,
}

impl State {
    fn toggle(&self) -> State {
        match self {
            State::On => State::Off,
            State::Off => State::On,
            _ => panic!("should not get here"),
        }
    }
}

type Point = (usize, usize);

#[derive(Debug, Clone)]
struct Instruction {
    state: State,
    start: Point,
    end: Point,
}

impl From<&str> for Instruction {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split_whitespace().collect();
        if parts[0] == "toggle" {
            let start_coords: Vec<_> = parts[1].split(',').collect();
            let end_coords: Vec<_> = parts[3].split(',').collect();
            let start_row = start_coords[0].parse().unwrap();
            let start_col = start_coords[1].parse().unwrap();
            let end_row = end_coords[0].parse().unwrap();
            let end_col = end_coords[1].parse().unwrap();
            Self {
                state: State::Toggle,
                start: (start_row, start_col),
                end: (end_row, end_col),
            }
        } else {
            let state = match parts[1] {
                "on" => State::On,
                "off" => State::Off,
                _ => panic!("unexpected string: {}", parts[1]),
            };
            let start_coords: Vec<_> = parts[2].split(',').collect();
            let end_coords: Vec<_> = parts[4].split(',').collect();
            let start_row = start_coords[0].parse().unwrap();
            let start_col = start_coords[1].parse().unwrap();
            let end_row = end_coords[0].parse().unwrap();
            let end_col = end_coords[1].parse().unwrap();
            Self {
                state,
                start: (start_row, start_col),
                end: (end_row, end_col),
            }
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect();
    let mut grid = vec![vec![State::Off; 1000]; 1000];

    for i in instructions {
        for r in i.start.0..=i.end.0 {
            for c in i.start.1..=i.end.1 {
                match i.state {
                    State::On => grid[r][c] = State::On,
                    State::Off => grid[r][c] = State::Off,
                    State::Toggle => grid[r][c] = grid[r][c].toggle(),
                }
            }
        }
    }

    let mut sum = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == State::On {
                sum += 1;
            }
        }
    }

    println!("Turned on lights: {}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect();
    let mut grid = vec![vec![0; 1000]; 1000];

    for i in instructions {
        for r in i.start.0..=i.end.0 {
            for c in i.start.1..=i.end.1 {
                match i.state {
                    State::On => grid[r][c] += 1,
                    State::Off => {
                        grid[r][c] -= 1;
                        if grid[r][c] < 0 {
                            grid[r][c] = 0;
                        }
                    }
                    State::Toggle => grid[r][c] += 2,
                }
            }
        }
    }

    let mut sum = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            sum += grid[r][c];
        }
    }

    println!("Turned on lights: {}", sum);

    Ok(())
}
