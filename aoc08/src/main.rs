use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let list: Vec<&str> = input.lines().collect();

    let mut code_count = 0;
    let mut memory_count = 0;
    for l in list {
        let (cc, mc) = count_chars(l);
        code_count += cc;
        memory_count += mc;
    }

    println!("Code Count: {}", code_count);
    println!("Memory Count: {}", memory_count);
    let total = code_count - memory_count;
    println!("Total: {}", total);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let list: Vec<&str> = input.lines().collect();

    let mut code_count = 0;
    let mut memory_count = 0;
    for l in &list {
        let (cc, mc) = count_chars(l);
        code_count += cc;
        memory_count += mc;
    }

    println!("Code Count: {}", code_count);
    println!("Memory Count: {}", memory_count);
    let total = code_count - memory_count;
    println!("Total: {}", total);

    let mut total_chars = 0i32;
    for l in &list {
        let encoded = encode(l);
        total_chars += encoded.len() as i32 + 2;
    }

    println!("Total encoded: {}", total_chars);
    let final_total = total_chars - code_count;
    println!("Final Total: {}", final_total);

    Ok(())
}

fn encode(line: &str) -> String {
    let mut encoded = String::new();

    let mut peek_chars = line.chars().peekable();

    while let Some(c) = peek_chars.next() {
        match c {
            '"' => encoded.push_str("\\\""),
            '\\' => encoded.push_str("\\\\"),
            _ => encoded.push(c),
        }
    }

    encoded
}

fn count_chars(line: &str) -> (i32, i32) {
    let mut peek_chars = line.chars().peekable();
    let mut code_count = 0;
    let mut memory_count = 0;
    while let Some(c) = peek_chars.next() {
        match c {
            '\\' => {
                if let Some(pc) = peek_chars.peek() {
                    match pc {
                        '\\' => {
                            code_count += 1;
                            memory_count += 1;
                            peek_chars.next();
                        }
                        '"' => {
                            code_count += 1;
                            memory_count += 1;
                            peek_chars.next();
                        }
                        'x' => {
                            code_count += 3;
                            memory_count += 1;
                            peek_chars.next();
                            peek_chars.next();
                            peek_chars.next();
                        }
                        _ => panic!("should not get here: {}", pc),
                    }
                }
            }
            '"' => {}
            _ => memory_count += 1,
        }
        code_count += 1;
    }

    (code_count, memory_count)
}
