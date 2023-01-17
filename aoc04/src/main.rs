use std::{
    collections::HashSet,
    fmt::format,
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    // let input = "abcdef";
    let input = "bgvyzdsv";
    part1(input)?;
    part2(input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut dec = 1;
    loop {
        let i = format!("{}{}", input, dec);
        let hash = md5::compute(i.clone());
        let hash_str = format!("{:?}", hash);
        let start: String = hash_str.chars().take(5).collect();
        if start == "00000" {
            println!("MD5: {:?}", hash);
            println!("Input: {}", i);
            break;
        }
        dec += 1;
    }
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut dec = 1;
    loop {
        let i = format!("{}{}", input, dec);
        let hash = md5::compute(i.clone());
        let hash_str = format!("{:?}", hash);
        let start: String = hash_str.chars().take(6).collect();
        if start == "000000" {
            println!("MD5: {:?}", hash);
            println!("Input: {}", i);
            break;
        }
        dec += 1;
    }
    Ok(())
}
