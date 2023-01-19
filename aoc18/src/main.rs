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

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    On,
    Off,
}

impl From<char> for State {
    fn from(input: char) -> Self {
        match input {
            '#' => Self::On,
            '.' => Self::Off,
            _ => panic!("should not get here: {}", input),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let size = 6;
    let mut lights: Vec<Vec<State>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.into()).collect())
        .collect();

    display_lights(&lights);

    let mut update = lights.clone();
    for _ in 0..100 {
        update = animate(&update, false);
    }

    let lights_on = update.iter().fold(0, |acc, g| {
        acc + g.iter().filter(|l| **l == State::On).count()
    });

    display_lights(&update);
    println!("Lignts On: {}", lights_on);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let size = 6;
    let mut lights: Vec<Vec<State>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.into()).collect())
        .collect();

    let corners: HashSet<(i64, i64)> = HashSet::from([
        (0i64, 0),
        (0i64, lights.len() as i64 - 1),
        (lights.len() as i64 - 1, 0i64),
        (lights.len() as i64 - 1, lights.len() as i64 - 1),
    ]);

    corners
        .iter()
        .for_each(|(r, c)| lights[*r as usize][*c as usize] = State::On);

    // display_lights(&lights);

    let mut update = lights.clone();
    for _ in 0..100 {
        update = animate(&update, true);
    }

    let lights_on = update.iter().fold(0, |acc, g| {
        acc + g.iter().filter(|l| **l == State::On).count()
    });

    display_lights(&update);
    println!("Lignts On: {}", lights_on);

    Ok(())
}

fn animate(lights: &Vec<Vec<State>>, ignore_corners: bool) -> Vec<Vec<State>> {
    let dirs: Vec<(i64, i64)> = vec![
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];

    let corners: HashSet<(i64, i64)> = HashSet::from([
        (0i64, 0),
        (0i64, lights.len() as i64 - 1),
        (lights.len() as i64 - 1, 0i64),
        (lights.len() as i64 - 1, lights.len() as i64 - 1),
    ]);

    let mut update = lights.clone();
    if ignore_corners {
        for (r, c) in &corners {
            update[*r as usize][*c as usize] = State::On;
        }
    }

    // display_lights(&update);
    for r in 0..lights.len() {
        for c in 0..lights[r].len() {
            if ignore_corners {
                if corners.contains(&(r as i64, c as i64)) {
                    continue;
                }
            }
            let mut count_on = 0;
            for d in &dirs {
                let rr = r as i64 + d.0;
                let cc = c as i64 + d.1;

                if rr >= lights.len() as i64 || rr < 0 || cc >= lights[r].len() as i64 || cc < 0 {
                    continue;
                }

                match lights[rr as usize][cc as usize] {
                    State::On => count_on += 1,
                    _ => {}
                }
            }

            match lights[r][c] {
                State::On => {
                    if count_on != 2 && count_on != 3 {
                        update[r][c] = State::Off;
                    }
                }
                State::Off => {
                    if count_on == 3 {
                        update[r][c] = State::On;
                    }
                }
            }
        }
    }

    // display_lights(&update);
    update
}

fn display_lights(lights: &Vec<Vec<State>>) {
    for r in 0..lights.len() {
        for c in 0..lights[r].len() {
            match lights[r][c] {
                State::On => print!("#"),
                State::Off => print!("."),
            }
        }
        println!();
    }
    println!();
}
