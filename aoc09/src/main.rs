use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut distances: HashMap<(String, String), i32> = HashMap::new();
    let mut cities = HashSet::new();
    for l in input.lines() {
        let (c, w) = l.split_once(" = ").unwrap();
        let weight = w.parse().unwrap();
        let (from, to) = c.split_once(" to ").unwrap();

        distances.insert((from.to_string(), to.to_string()), weight);
        distances.insert((to.to_string(), from.to_string()), weight);
        cities.insert(from.to_string());
        cities.insert(to.to_string());
    }

    let mut keys: Vec<String> = cities.iter().map(|c| c.clone()).collect();
    let perms = generate(&mut keys, cities.len());
    let mut min_distance = i32::MAX;
    let mut max_distance = 0;
    for p in perms {
        // println!("Perm: {:?}", p);
        let mut distance = 0;
        for pair in p.windows(2) {
            if let Some(w) = distances.get(&(pair[0].clone(), pair[1].clone())) {
                distance += w;
            }
        }
        min_distance = min_distance.min(distance);
        max_distance = max_distance.max(distance);
    }
    println!("Min Distance: {}", min_distance);
    println!("Max Distance: {}", max_distance);

    Ok(())
}

fn generate<T: Clone>(input: &mut [T], size: usize) -> Vec<Vec<T>> {
    if input.is_empty() {
        return vec![];
    }

    let mut result = vec![];
    let mut c = vec![0; size];

    let mut perm = vec![];
    for t in input.into_iter() {
        perm.push(t.clone());
    }

    result.push(perm);

    let mut i = 1;
    while i < size {
        if c[i] < i {
            if i % 2 == 0 {
                input.swap(0, i);
            } else {
                input.swap(c[i], i);
            }

            let mut perm = vec![];
            for t in input.into_iter() {
                perm.push(t.clone());
            }

            result.push(perm);

            c[i] += 1;
            i = 1;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    result
}
