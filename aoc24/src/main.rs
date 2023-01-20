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
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let weights: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();

    let sum = weights.iter().sum::<i64>();
    let target: i64 = sum / 3;
    let len = weights.len();
    println!("Sum: {sum}");
    println!("Target: {target}");

    let mut done = false;
    let mut qe = i64::MAX;
    for k in 1..weights.len() {
        for combo in weights.iter().combinations(k) {
            let sum: i64 = combo.iter().cloned().sum();
            if sum == target {
                // println!("{combo:?}");
                let prod: i64 = combo.iter().cloned().product();
                if qe == 0 || prod < qe {
                    qe = prod;
                    done = true;
                }
            }
        }

        if done {
            break;
        }
    }

    println!("qe: {}", qe);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let weights: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();

    let sum = weights.iter().sum::<i64>();
    let target: i64 = sum / 4;
    let len = weights.len();
    println!("Sum: {sum}");
    println!("Target: {target}");

    let mut done = false;
    let mut qe = i64::MAX;
    for k in 1..weights.len() {
        for combo in weights.iter().combinations(k) {
            let sum: i64 = combo.iter().cloned().sum();
            if sum == target {
                println!("{combo:?}");
                let prod: i64 = combo.iter().cloned().product();
                if qe == 0 || prod < qe {
                    qe = prod;
                    done = true;
                }
            }
        }

        if done {
            break;
        }
    }

    println!("qe: {}", qe);

    Ok(())
}
