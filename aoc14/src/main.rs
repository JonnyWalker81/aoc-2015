use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

struct Racer {
    name: String,
    speed: i64,
    duration: i64,
    rest_needed: i64,
    distance: i64,
    rest_time: i64,
    race_time: i64,
    is_resting: bool,
    points: i64,
}

impl Racer {
    fn new(name: String, speed: i64, duration: i64, rest_needed: i64) -> Self {
        Self {
            name,
            speed,
            duration,
            rest_needed,
            distance: 0,
            rest_time: 0,
            race_time: 0,
            is_resting: false,
            points: 0,
        }
    }

    fn tick(&mut self) {
        if self.is_resting {
            self.rest_time += 1;
            if self.rest_time == self.rest_needed {
                self.is_resting = false;
                self.rest_time = 0;
            }
        } else {
            self.race_time += 1;
            if self.race_time == self.duration {
                self.is_resting = true;
                self.race_time = 0;
            }
            self.distance += self.speed;
        }
    }

    fn award_point(&mut self) {
        self.points += 1;
    }
}

impl From<&str> for Racer {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split_whitespace().collect();
        let name = parts[0].to_string();
        let speed = parts[3].parse().unwrap();
        let duration = parts[6].parse().unwrap();
        let rest_needed = parts[13].parse().unwrap();

        Self::new(name, speed, duration, rest_needed)
    }
}

fn part1(input: &str) -> Result<()> {
    let mut racers: Vec<Racer> = input.lines().map(|l| l.into()).collect();

    for _ in 0..2503 {
        for r in &mut racers {
            r.tick();
        }
    }

    // for r in &racers {
    //     println!("{}: {}", r.name, r.distance);
    // }

    let winner = racers.iter().map(|r| r.distance).max().unwrap();
    println!("Winner: {}", winner);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut racers: Vec<Racer> = input.lines().map(|l| l.into()).collect();

    for _ in 0..2503 {
        for r in &mut racers {
            r.tick();
        }
        let winning = racers.iter().map(|r| r.distance).max().unwrap();

        for r in &mut racers {
            if r.distance == winning {
                r.award_point();
            }
        }
    }

    // for r in &racers {
    //     println!("{}: {}", r.name, r.points);
    // }

    let winner = racers.iter().map(|r| r.points).max().unwrap();
    println!("Winner: {}", winner);
    Ok(())
}
