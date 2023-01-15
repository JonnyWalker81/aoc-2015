use std::{
    collections::{HashMap, HashSet},
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
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(item: char) -> Self {
        match item {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => panic!("unrecognized char: {}", item),
        }
    }
}

impl Direction {
    fn next(&self) -> (i64, i64) {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let directions: Vec<Direction> = input.trim().chars().map(|c| c.into()).collect();

    let mut position = (0, 0);
    let mut positions: HashMap<(i64, i64), i64> = HashMap::new();
    positions.insert((0, 0), 1);

    for d in directions {
        let (r, c) = d.next();
        position.0 += r;
        position.1 += c;

        positions
            .entry(position)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    println!("Positions: {}", positions.len());

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let directions: Vec<Direction> = input.trim().chars().map(|c| c.into()).collect();

    let mut santa_position = (0, 0);
    let mut robo_position = (0, 0);
    let mut positions: HashMap<(i64, i64), i64> = HashMap::new();
    positions.insert((0, 0), 1);

    let mut santas_turn = true;
    for d in directions {
        let (r, c) = d.next();

        if santas_turn {
            santa_position.0 += r;
            santa_position.1 += c;
            positions
                .entry(santa_position)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        } else {
            robo_position.0 += r;
            robo_position.1 += c;
            positions
                .entry(robo_position)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        santas_turn = !santas_turn;
    }

    println!("Positions: {}", positions.len());

    Ok(())
}
