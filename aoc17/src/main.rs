use std::{
    collections::HashMap,
    io::{self, Read},
};

use itertools::Itertools;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    // let containers = vec![20, 15, 10, 5, 5];
    let containers: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();

    let target = 150;

    let mut count = 0;

    let mut min_containers = usize::MAX;
    for k in 0..containers.len() {
        for c in containers.iter().combinations(k) {
            let s: usize = c.clone().into_iter().sum::<usize>();
            if s == target {
                count += 1;
                min_containers = min_containers.min(c.len());
            }
        }
    }
    println!("Combinations: {}", count);

    count = 0;
    for c in containers.iter().combinations(min_containers) {
        let s: usize = c.clone().into_iter().sum::<usize>();
        if s == target {
            count += 1;
        }
    }

    println!("Combinations: {}", count);
    println!("Min containers: {}", min_containers);

    Ok(())
}
