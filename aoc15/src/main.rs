type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    part1()?;
    part2()?;

    Ok(())
}

fn part1() -> Result<()> {
    let mut max_score = 0;
    for sugar in 0..100 {
        for sprinkles in 0..100 {
            for candy in 0..100 {
                let chocolate = 100 - candy - sprinkles - sugar;

                if chocolate < 0 {
                    continue;
                }

                let capacity = (3 * sugar + -3 * sprinkles + -1 * candy + 0 * chocolate).max(0);
                let durability = (0 * sugar + 3 * sprinkles + 0 * candy + 0 * chocolate).max(0);
                let flavor = (0 * sugar + 0 * sprinkles + 4 * candy + -2 * chocolate).max(0);
                let texture = (-3 * sugar + 0 * sprinkles + 0 * candy + 2 * chocolate).max(0);

                let total = capacity * durability * flavor * texture;
                max_score = max_score.max(total);
            }
        }
    }

    println!("Max score: {}", max_score);
    Ok(())
}

fn part2() -> Result<()> {
    let mut max_score = 0;
    for sugar in 0..100 {
        for sprinkles in 0..100 {
            for candy in 0..100 {
                let chocolate = 100 - candy - sprinkles - sugar;

                if chocolate < 0 {
                    continue;
                }

                let capacity = (3 * sugar + -3 * sprinkles + -1 * candy + 0 * chocolate).max(0);
                let durability = (0 * sugar + 3 * sprinkles + 0 * candy + 0 * chocolate).max(0);
                let flavor = (0 * sugar + 0 * sprinkles + 4 * candy + -2 * chocolate).max(0);
                let texture = (-3 * sugar + 0 * sprinkles + 0 * candy + 2 * chocolate).max(0);
                let calories = (2 * sugar + 9 * sprinkles + 1 * candy + 8 * chocolate).max(0);

                if calories != 500 {
                    continue;
                }

                let total = capacity * durability * flavor * texture;
                max_score = max_score.max(total);
            }
        }
    }

    println!("Max score: {}", max_score);
    Ok(())
}
