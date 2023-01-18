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
struct Sue {
    id: usize,
    properties: HashMap<String, i64>,
}

impl From<&str> for Sue {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split_whitespace().collect();

        let id = parts[1].trim_end_matches(':').parse().unwrap();

        let mut properties = HashMap::new();
        properties.insert(
            parts[2].trim_end_matches(':').to_string(),
            parts[3].trim_end_matches(',').parse().unwrap(),
        );
        properties.insert(
            parts[4].trim_end_matches(':').to_string(),
            parts[5].trim_end_matches(',').parse().unwrap(),
        );
        properties.insert(
            parts[6].trim_end_matches(':').to_string(),
            parts[7].trim_end_matches(',').parse().unwrap(),
        );

        Self { id, properties }
    }
}

fn part1(input: &str) -> Result<()> {
    let sues: Vec<Sue> = input.lines().map(|l| l.into()).collect();

    let mapping = HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);

    'outer: for sue in sues {
        for (k, v) in &mapping {
            if let Some(val) = sue.properties.get(k) {
                if *v != *val {
                    continue 'outer;
                }
            }
        }

        println!("Sue: {}", sue.id);
    }
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let sues: Vec<Sue> = input.lines().map(|l| l.into()).collect();

    let mapping = HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);

    'outer: for sue in sues {
        for (k, v) in &mapping {
            if let Some(val) = sue.properties.get(k) {
                if k == "cats" || k == "trees" {
                    if *val < *v {
                        continue 'outer;
                    }
                } else if k == "pomeranians" || k == "goldfish" {
                    if *val > *v {
                        continue 'outer;
                    }
                } else {
                    if *v != *val {
                        continue 'outer;
                    }
                }
            }
        }

        println!("Sue: {}", sue.id);
    }
    Ok(())
}
