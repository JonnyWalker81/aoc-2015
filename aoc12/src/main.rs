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
    let json: serde_json::Value = serde_json::from_str(input).unwrap();
    let mut sum = 0i64;
    process(&json, false, &mut sum);
    println!("Sum: {}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let json: serde_json::Value = serde_json::from_str(input).unwrap();
    let mut sum = 0i64;
    process(&json, true, &mut sum);
    println!("Sum: {}", sum);
    Ok(())
}

fn process(json: &serde_json::Value, skip_red: bool, sum: &mut i64) {
    match json {
        serde_json::Value::Null => {}
        serde_json::Value::Bool(_) => {}
        serde_json::Value::Number(n) => *sum += n.as_i64().unwrap(),
        serde_json::Value::String(_) => {}
        serde_json::Value::Array(a) => {
            for e in a {
                process(e, skip_red, sum);
            }
        }
        serde_json::Value::Object(o) => {
            let mut contains_red = false;
            for (_, v) in o {
                if skip_red && v.is_string() && v.as_str().unwrap() == "red" {
                    contains_red = true;
                    break;
                }
            }

            if !contains_red {
                for (_, v) in o {
                    process(v, skip_red, sum);
                }
            }
        }
    }
}
