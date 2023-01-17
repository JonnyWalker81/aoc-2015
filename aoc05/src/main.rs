use std::{
    collections::HashSet,
    io::{self, Read},
    iter::zip,
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
    let strings: Vec<&str> = input.lines().collect();

    let mut valid = 0;
    for s in strings {
        if is_valid(&s) {
            valid += 1;
        }
    }

    println!("Nice Strings: {}", valid);
    Ok(())
}

fn is_valid(s: &str) -> bool {
    let vowels: HashSet<_> = vec!['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
    let mut vowel_count = 0;

    for v in s.chars() {
        if vowels.contains(&v) {
            vowel_count += 1;
        }
    }

    if vowel_count < 3 {
        return false;
    }

    let s_chars: Vec<_> = s.chars().collect();
    let mut repeated_chars = false;
    for i in 0..s_chars.len() - 1 {
        if s_chars[i] == s_chars[i + 1] {
            repeated_chars = true;
            break;
        }
    }

    let not_allowed = vec!["ab", "cd", "pq", "xy"];
    for na in not_allowed {
        if s.contains(na) {
            return false;
        }
    }

    repeated_chars
}

fn part2(input: &str) -> Result<()> {
    let strings: Vec<&str> = input.lines().collect();

    let mut valid = 0;
    for s in strings {
        if is_valid2(&s) {
            valid += 1;
        }
    }

    println!("Nice Strings: {}", valid);

    Ok(())
}

fn is_valid2(s: &str) -> bool {
    // let s_chars: Vec<_> = s.chars().collect();
    let mut found = false;
    for (i, pair) in zip(s.chars(), s.chars().skip(1)).enumerate() {
        let pattern = format!("{}{}", pair.0, pair.1);
        if let Some(index) = s.rfind(&pattern) {
            if index > i + 1 {
                found = true;
                break;
            }
        }
    }

    if !found {
        return false;
    }

    found = false;
    for pair in zip(s.chars(), s.chars().skip(2)) {
        if pair.0 == pair.1 {
            found = true;
            break;
        }
    }

    found
}
