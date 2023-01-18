use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    // let input = "1";
    let input = "1113122113";

    part1(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut seq = input.to_string();
    for _ in 0..40 {
        seq = say(&seq);
    }
    println!("{}", seq.len());

    println!();

    seq = input.to_string();
    for _ in 0..50 {
        seq = say(&seq);
    }
    println!("{}", seq.len());

    Ok(())
}

fn say(input: &str) -> String {
    let mut peek_chars = input.chars().peekable();
    let mut result = String::new();
    while let Some(c) = peek_chars.next() {
        let mut count = 1;
        while let Some(pc) = peek_chars.peek() {
            if c == *pc {
                count += 1;
                peek_chars.next();
            } else {
                break;
            }
        }

        let val = format!("{}{}", count, c);
        result.push_str(&val);
    }

    result
}
