use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug)]
struct Game {
    boss_hp: i64,
    boss_damage: i64,
    boss_armor: i64,
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<[Item; 2]>,
}

impl Game {
    fn new(boss_hp: i64, boss_damage: i64, boss_armor: i64) -> Self {
        let weapons = vec![
            Item::new("Dagger", 8, 4, 0),
            Item::new("Shortswords", 10, 5, 0),
            Item::new("Warhammer", 25, 6, 0),
            Item::new("Longsword", 40, 7, 0),
            Item::new("Greataxe", 74, 8, 0),
        ];

        let armor = vec![
            Item::new("Nothing", 0, 0, 0),
            Item::new("Leather", 13, 0, 1),
            Item::new("Chainmail", 31, 0, 2),
            Item::new("Splintmain", 53, 0, 3),
            Item::new("Bandedmail", 75, 0, 4),
            Item::new("Platemail", 102, 0, 5),
        ];

        let rings_list = vec![
            Item::new("Nothing", 0, 0, 0),
            Item::new("Nothing", 0, 0, 0),
            Item::new("Damage +1", 25, 1, 0),
            Item::new("Damage +2", 50, 2, 0),
            Item::new("Damage +3", 100, 3, 0),
            Item::new("Defense +1", 20, 0, 1),
            Item::new("Defense +2", 40, 0, 2),
            Item::new("Defense +3", 80, 0, 3),
        ];

        let mut rings = vec![];
        for i in 0..rings_list.len() - 1 {
            for j in i + 1..rings_list.len() {
                rings.push([rings_list[i].clone(), rings_list[j].clone()]);
            }
        }

        Self {
            boss_hp,
            boss_damage,
            boss_armor,
            weapons,
            armor,
            rings,
        }
    }
}

#[derive(Debug, Clone)]
struct Item {
    name: String,
    cost: i64,
    damage: i64,
    armor: i64,
}

impl Item {
    fn new(name: &str, cost: i64, damage: i64, armor: i64) -> Self {
        Self {
            name: name.to_string(),
            cost,
            damage,
            armor,
        }
    }
}

struct Player {
    hp: i64,
    damage: i64,
    armor: i64,
}

impl Player {
    fn new(hp: i64, damage: i64, armor: i64) -> Self {
        Self { hp, damage, armor }
    }

    fn would_win_against(&self, Player { hp, damage, armor }: &Player) -> bool {
        let mut player_hp = self.hp;
        let mut boss_hp = *hp;

        loop {
            let player_damage = 1.max(self.damage - armor);
            boss_hp -= player_damage;
            // println!("The player deals {}-{armor} = {player_damage}; the boss goes down to {boss_hp} hit points.", self.damage);
            if boss_hp <= 0 {
                return true;
            }

            let boss_damage = 1.max(damage - self.armor);
            player_hp -= boss_damage;
            // println!("The boss deals {damage}-{} = {boss_damage}; the boss goes down to {player_hp} hit points.", self.armor);
            if player_hp <= 0 {
                return false;
            }
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.lines().collect();
    let (_, hp) = lines[0].split_once(": ").unwrap();
    let (_, damage) = lines[1].split_once(": ").unwrap();
    let (_, armor) = lines[2].split_once(": ").unwrap();

    let game = Game::new(
        hp.parse().unwrap(),
        damage.parse().unwrap(),
        armor.parse().unwrap(),
    );

    let mut min_cost = i64::MAX;
    for w in &game.weapons {
        for a in &game.armor {
            for r in &game.rings {
                let player = Player::new(
                    100,
                    w.damage + r[0].damage + r[1].damage,
                    a.armor + r[0].armor + r[1].armor,
                );
                let boss = Player::new(game.boss_hp, game.boss_damage, game.boss_armor);
                if player.would_win_against(&boss) {
                    let cost = w.cost + a.cost + r[0].cost + r[1].cost;
                    min_cost = min_cost.min(cost);
                }
            }
        }
    }
    println!("Min Cost: {}", min_cost);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.lines().collect();
    let (_, hp) = lines[0].split_once(": ").unwrap();
    let (_, damage) = lines[1].split_once(": ").unwrap();
    let (_, armor) = lines[2].split_once(": ").unwrap();

    let game = Game::new(
        hp.parse().unwrap(),
        damage.parse().unwrap(),
        armor.parse().unwrap(),
    );

    let mut max_cost = 0;
    for w in &game.weapons {
        for a in &game.armor {
            for r in &game.rings {
                let player = Player::new(
                    100,
                    w.damage + r[0].damage + r[1].damage,
                    a.armor + r[0].armor + r[1].armor,
                );
                let boss = Player::new(game.boss_hp, game.boss_damage, game.boss_armor);
                if !player.would_win_against(&boss) {
                    let cost = w.cost + a.cost + r[0].cost + r[1].cost;
                    max_cost = max_cost.max(cost);
                }
            }
        }
    }
    println!("Max Cost: {}", max_cost);

    Ok(())
}
