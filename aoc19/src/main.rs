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

fn part1(input: &str) -> Result<()> {
    let (replacements_input, molecule_input) = input.split_once("\n\n").unwrap();

    let replacements: Vec<(String, String)> = replacements_input
        .lines()
        .map(|l| {
            let (src, rep) = l.split_once(" => ").unwrap();

            (src.to_string(), rep.to_string())
        })
        .collect();

    let mut molecules = HashSet::new();
    for (k, v) in &replacements {
        let m: Vec<_> = molecule_input.match_indices(k).collect();
        for (i, _) in m {
            let s = molecule_input[i..].replacen(k, v, 1);
            let molecule = format!("{}{}", molecule_input[..i].to_string(), s);
            molecules.insert(molecule);
        }
    }

    println!("Molecules: {}", molecules.len());
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let (replacements_input, molecule_input) = input.split_once("\n\n").unwrap();

    let mut molecule = molecule_input.to_string();

    let replacements: HashMap<String, String> = replacements_input
        .lines()
        .map(|l| {
            let (src, rep) = l.split_once(" => ").unwrap();

            (rep.to_string(), src.to_string())
        })
        .collect();

    let mut count = 0;
    loop {
        let mut done = true;
        for (k, v) in &replacements {
            if let Some(pos) = molecule.find(&*k) {
                let (left, right) = molecule.split_at(pos);
                let right = right.to_string().split_off(k.len());
                molecule = format!("{left}{v}{right}");
                done = false;
                count += 1;
            }
        }
        if done {
            break;
        }
    }

    println!("Count: {}", count);

    Ok(())
}
