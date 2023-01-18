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
    let mut people = HashSet::new();
    let mapping: HashMap<(String, String), i64> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let person_a = parts[0].to_string();
            let person_b = parts[parts.len() - 1].trim_end_matches('.').to_string();
            let happiness = if parts[2] == "gain" {
                parts[3].parse().unwrap()
            } else {
                parts[3].parse::<i64>().unwrap() * -1
            };

            people.insert(person_a.clone());
            people.insert(person_b.clone());
            ((person_a, person_b), happiness)
        })
        .collect();

    let mut people_vec: Vec<String> = people.into_iter().collect();

    let k = people_vec.len();

    let perms = permutations(&mut people_vec, k);
    let mut max_hap = 0;
    for entry in perms {
        let h = happiness(&mapping, entry);
        max_hap = max_hap.max(h);
    }
    println!("Optimal Hap: {}", max_hap);

    Ok(())
}

fn happiness(data: &HashMap<(String, String), i64>, v: Vec<String>) -> i64 {
    let mut result = 0;
    let last = v.len() - 1;

    result += data.get(&(v[0].clone(), v[1].clone())).unwrap()
        + data.get(&(v[0].clone(), v[last].clone())).unwrap();

    for entry in 1..last {
        result += data.get(&(v[entry].clone(), v[entry + 1].clone())).unwrap()
            + data.get(&(v[entry].clone(), v[entry - 1].clone())).unwrap();
    }

    result += data.get(&(v[last].clone(), v[last - 1].clone())).unwrap()
        + data.get(&(v[last].clone(), v[0].clone())).unwrap();

    result
}

fn part2(input: &str) -> Result<()> {
    let mut people = HashSet::new();
    let mut mapping: HashMap<(String, String), i64> = HashMap::new();
    for l in input.lines() {
        let parts: Vec<&str> = l.split_whitespace().collect();
        let person_a = parts[0].to_string();
        let person_b = parts[parts.len() - 1].trim_end_matches('.').to_string();
        let happiness = if parts[2] == "gain" {
            parts[3].parse().unwrap()
        } else {
            parts[3].parse::<i64>().unwrap() * -1
        };

        people.insert(person_a.clone());
        people.insert(person_b.clone());
        mapping.insert((person_a.clone(), person_b.clone()), happiness);
        mapping.insert(("me".to_string(), person_a.clone()), 0);
        mapping.insert(("me".to_string(), person_b.clone()), 0);
        mapping.insert((person_a, "me".to_string()), 0);
        mapping.insert((person_b, "me".to_string()), 0);
    }

    people.insert("me".to_string());
    let mut people_vec: Vec<String> = people.into_iter().collect();

    let k = people_vec.len();

    let perms = permutations(&mut people_vec, k);

    let mut max_hap = 0;
    for entry in perms {
        let h = happiness(&mapping, entry);
        max_hap = max_hap.max(h);
    }
    println!("Optimal Hap: {}", max_hap);

    Ok(())
}

fn permutations<T: Clone>(input: &mut [T], size: usize) -> Vec<Vec<T>> {
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
