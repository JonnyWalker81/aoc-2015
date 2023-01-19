use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let input = 33_100_000;

    // part1(input)?;
    part2(input)?;

    Ok(())
}

fn part1(input: i64) -> Result<()> {
    let mut i = 1;
    loop {
        let mut sum = 0;
        for j in 1..=i {
            if i % j == 0 {
                sum += j * 10;
            }
        }
        if sum >= input {
            println!("{}: {}", i, sum);
            break;
        }
        i += 1;
    }
    Ok(())
}

fn part2(input: u64) -> Result<()> {
    let mut limit = 1_000_000;
    let mut houses = vec![0u64; limit];
    for elf in 1..limit {
        let fifty = limit.min(elf * 50);
        for visited in (elf..fifty).step_by(elf) {
            houses[visited] += elf as u64 * 11;
            if houses[visited] >= input && visited < limit {
                limit = visited;
                break;
            }
        }
    }

    for (i, house) in houses.iter().enumerate() {
        if *house >= input {
            println!("House: {}", i);
        }
    }
    Ok(())
}
