use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::iter::zip;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let input = "hepxcrrq";
    // let input = "ghijklmn";
    // let input = "zzizzlyy";

    part1(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut s = input.to_string();
    let mut valid_count = 0;
    loop {
        s = add_one(&s);
        if is_valid(&s) {
            valid_count += 1;
            println!("{}", s);

            if valid_count == 2 {
                break;
            }
        }
        // break;
    }
    Ok(())
}

fn is_valid(input: &str) -> bool {
    let mut found = false;
    let mut repeats = HashSet::new();
    // let mut iter = input.into_iter();
    // let mut prev = iter.next();

    for pair in zip(input.chars(), input.chars().skip(1)) {
        if pair.0 == pair.1 {
            let p = format!("{}{}", pair.0, pair.1);
            // println!("Repeat: {}", p);
            if !repeats.contains(&p) {
                repeats.insert(p);
            }
        }
    }

    // println!("Repeat count: {}", repeats.len());

    let bad = vec!['i', 'o', 'l'];
    let chars: HashSet<char> = input.chars().collect();
    let mut found_bad = false;
    for b in bad {
        if chars.contains(&b) {
            found_bad = true;
            break;
        }
    }

    let nums: Vec<u8> = input.chars().map(|c| c as u8).collect();

    let mut increasing_count = 1;
    let mut has_straight = false;
    for i in 0..nums.len() - 1 {
        if nums[i + 1] - nums[i] == 1 {
            increasing_count += 1;
        } else {
            if increasing_count >= 3 {
                has_straight = true;
            }
            increasing_count = 1;
        }
    }

    repeats.len() > 1 && !found_bad && has_straight
}

fn add_one(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    // println!("{:?}", chars);
    for i in (0..chars.len()).rev() {
        if chars[i] != 'z' {
            chars[i] = (chars[i] as u8 + 1) as char;
            break;
        } else {
            chars[i] = 'a';
        }
    }

    // println!("{:?}", chars);
    // let mut c = 'a';
    // c = (c as u8 + 1) as char;
    // println!("{}", c);
    chars.iter().collect()
}
