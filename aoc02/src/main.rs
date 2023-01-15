use std::{
    collections::HashSet,
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

struct Dimension {
    length: i64,
    width: i64,
    height: i64,
}

impl From<&str> for Dimension {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split('x').collect();
        Self {
            length: parts[0].parse().unwrap(),
            width: parts[1].parse().unwrap(),
            height: parts[2].parse().unwrap(),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let dimensions: Vec<Dimension> = input.lines().map(|l| l.into()).collect();

    let mut sum = 0;
    for d in dimensions {
        let mut smallest = i64::MAX;
        let a = 2 * d.length * d.width;
        smallest = smallest.min(d.length * d.width);

        let b = 2 * d.width * d.height;
        smallest = smallest.min(d.width * d.height);

        let c = 2 * d.height * d.length;
        smallest = smallest.min(d.height * d.length);

        let total = a + b + c + smallest;
        sum += total;
    }

    println!("Total square feet: {}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let dimensions: Vec<Dimension> = input.lines().map(|l| l.into()).collect();

    let mut sum = 0;
    for d in dimensions {
        let mut dims = vec![d.length, d.width, d.height];
        dims.sort();

        let ribbon_length = dims[0] * 2 + dims[1] * 2;
        let ribbon_area = d.length * d.width * d.height;

        sum += ribbon_length + ribbon_area;
    }

    println!("Total square feet: {}", sum);
    Ok(())
}
