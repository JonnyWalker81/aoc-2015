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

enum Direction {
    Up,
    Down,
}

impl From<char> for Direction {
    fn from(item: char) -> Self {
        match item {
            '(' => Direction::Up,
            ')' => Direction::Down,
            _ => panic!("unrecognized char: {}", item),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let directions: Vec<Direction> = input.trim().chars().map(|c| c.into()).collect();

    let mut floor = 0;
    for d in directions {
        match d {
            Direction::Up => floor += 1,
            Direction::Down => floor -= 1,
        }
    }

    println!("Final floor: {floor}");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let directions: Vec<Direction> = input.trim().chars().map(|c| c.into()).collect();

    let mut floor = 0;
    for (i, d) in directions.iter().enumerate() {
        match d {
            Direction::Up => floor += 1,
            Direction::Down => floor -= 1,
        }

        if floor < 0 {
            println!("Position: {}", i + 1);
            break;
        }
    }

    Ok(())
}
