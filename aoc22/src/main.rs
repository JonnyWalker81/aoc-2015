type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    part1()?;
    part2()?;

    Ok(())
}

const SPELLS: [Spell; 5] = [
    Spell::new("Magic Missile", 53, 4, 0, 0, 0, 0),
    Spell::new("Drains", 73, 2, 2, 0, 0, 0),
    Spell::new("Sheild", 113, 0, 0, 7, 0, 6),
    Spell::new("Poison", 173, 3, 0, 0, 0, 6),
    Spell::new("Recharge", 229, 0, 0, 0, 101, 5),
];

#[derive(Debug, Clone)]
struct Game {
    player_hp: i64,
    boss_hp: i64,
    mana: i64,
    damage: i64,
    effects: Vec<Spell>,
    armor: i64,
    spent: i64,
    turn: i64,
    difficulty: i64,
    gamelog: String,
}

impl Game {
    fn new(player_hp: i64, boss_hp: i64, mana: i64, damage: i64) -> Self {
        Self {
            player_hp,
            boss_hp,
            mana,
            damage,
            effects: vec![],
            armor: 0,
            spent: 0,
            turn: 0,
            difficulty: 0,
            gamelog: String::new(),
        }
    }

    fn start_turn(&self) -> Self {
        let mut mana = self.mana;
        let mut boss_hp = self.boss_hp;
        let mut player_hp = self.player_hp;
        let mut armor = 0;
        let mut effects = vec![];
        let mut gamelog = if self.turn > 0 {
            format!("\n{}", self.gamelog)
        } else {
            String::new()
        };

        gamelog = format!(
            "{gamelog}\n--  {}  turn --\n- Player has {} hit points, {} armor {} mana\n- Boss has {} hit points\n",
            if self.turn % 2 == 0 { "Player" } else { "Boss" }, self.player_hp, self.armor, self.mana, self.boss_hp
        );

        if self.difficulty > 0 {
            player_hp -= 1;
        }

        for e in &self.effects {
            if e.armor > 0 {
                armor = e.armor;
            }

            if e.damage > 0 {
                boss_hp -= e.damage;
                gamelog = format!(
                    "{gamelog}{} deals {} damage: its timer is now {}\n",
                    e._name,
                    e.damage,
                    e.duration - 1
                );
            }

            if e.mana > 0 {
                gamelog = format!(
                    "{gamelog}{} provides {} mana: its timeer is now {}\n",
                    e._name,
                    e.mana,
                    e.duration - 1
                );
                mana += e.mana;
            }

            if e.duration > 1 {
                effects.push(e.tick());
            }
        }

        Self {
            player_hp,
            boss_hp,
            mana,
            damage: self.damage,
            effects,
            armor,
            spent: self.spent,
            turn: self.turn + 1,
            difficulty: self.difficulty,
            gamelog,
        }
    }

    fn cast(&self, spell: &Spell) -> Self {
        let gamelog = format!("{}Player casts {}\n", self.gamelog, spell._name);
        if self.mana < spell.cost {
            panic!("should not get here...");
        }

        let mut next = self.clone();
        next.gamelog = gamelog;
        next.spent += spell.cost;

        if spell.duration > 0 {
            let mut effects = self.effects.clone();
            effects.push(spell.clone());
            next.effects = effects;
            next.mana -= spell.cost;
        } else {
            next.player_hp += spell.heal;
            next.boss_hp -= spell.damage;
            next.mana += spell.mana - spell.cost;
            next.effects = self.effects.clone();
        }

        next
    }

    fn moves(&self) -> Vec<Spell> {
        let mut result = Vec::new();
        'outer: for spell in SPELLS {
            for effect in &self.effects {
                if effect._name == spell._name {
                    continue 'outer;
                }
            }

            if spell.cost <= self.mana {
                result.push(spell.clone());
            }
        }

        result
    }

    fn boss_move(&self) -> Self {
        let damage = 1.max(self.damage - self.armor);
        let gamelog = format!("{}Boss attacks for {} damage\n", self.gamelog, damage);
        let mut boss = self.clone();
        boss.player_hp -= damage;
        boss.gamelog = gamelog;

        // println!("Boss move: {boss:?}");

        boss
    }

    fn cheapest_win(&self, mut max: i64) -> Option<Self> {
        let turn = self.start_turn();
        if turn.boss_hp <= 0 {
            // println!(
            //     "{}\nPLAYER WINS! ({} mana spent)\n",
            //     self.gamelog, self.spent
            // );
            return Some(turn);
        }

        if turn.player_hp <= 0 {
            return None;
        }

        if turn.turn > 50 {
            // println!("OUT OF TURNS! {}\n", self.gamelog);
            return None;
        }

        if turn.turn % 2 == 0 {
            let boss_move = turn.boss_move();
            if boss_move.player_hp <= 0 {
                return None;
            }

            return boss_move.cheapest_win(max);
        };

        let mut best: Option<Game> = None;
        for spell in &turn.moves() {
            if spell.cost + self.spent >= max {
                continue;
            }

            let move_state = turn.cast(spell);

            if let Some(b) = &best {
                if b.spent < max {
                    max = b.spent;
                }
            }

            if move_state.boss_hp <= 0 {
                if let Some(b) = &best {
                    if b.spent > move_state.spent {
                        // println!("POSSIBLE SOLUTION\n{}", move_state.gamelog);
                        best = Some(move_state);
                    }
                } else {
                    best = Some(move_state);
                }
            } else if move_state.player_hp <= 0 {
                continue;
            } else if let Some(cheapest) = move_state.cheapest_win(max) {
                if let Some(b) = &best {
                    if b.spent > cheapest.spent {
                        // println!("CHEAP SOLUTION\n{}", cheapest.gamelog);
                        best = Some(cheapest);
                    }
                } else {
                    best = Some(cheapest);
                }
            }
        }

        best
    }
}

#[derive(Debug, Clone)]
struct Spell {
    _name: &'static str,
    cost: i64,
    damage: i64,
    heal: i64,
    armor: i64,
    mana: i64,
    duration: i64,
}

impl Spell {
    const fn new(
        _name: &'static str,
        cost: i64,
        damage: i64,
        heal: i64,
        armor: i64,
        mana: i64,
        duration: i64,
    ) -> Self {
        Self {
            _name,
            cost,
            damage,
            heal,
            armor,
            mana,
            duration,
        }
    }

    fn tick(&self) -> Self {
        let mut next = self.clone();
        next.duration -= 1;
        next
    }
}

fn part1() -> Result<()> {
    let starting_state = Game::new(50, 58, 500, 9);
    // let starting_state = Game::new(10, 14, 250, 8);
    let win = starting_state.cheapest_win(i64::MAX).unwrap();
    // println!("{}", win.gamelog);
    println!("Spent: {}", win.spent);
    Ok(())
}

fn part2() -> Result<()> {
    let mut starting_state = Game::new(50, 58, 500, 9);
    starting_state.difficulty = 1;
    // let starting_state = Game::new(10, 14, 250, 8);
    let win = starting_state.cheapest_win(i64::MAX).unwrap();
    // println!("{}", win.gamelog);
    println!("Spent: {}", win.spent);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_start_a_turn_with_no_effects() {
        let state = Game::new(10, 13, 250, 8);
        let next_state = state.start_turn();
        println!("state: {state:?}");
        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp, next_state.boss_hp);
        assert_eq!(state.mana, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
    }

    #[test]
    fn can_start_a_turn_with_sheild() {
        let mut state = Game::new(10, 13, 250, 8);
        state.effects.push(SPELLS[2].clone());
        let next_state = state.start_turn();
        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp, next_state.boss_hp);
        assert_eq!(state.mana, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.armor, 7);
        assert_eq!(next_state.effects.len(), 1);
        assert_eq!(next_state.effects[0].duration, SPELLS[2].duration - 1);
    }

    #[test]
    fn can_start_a_turn_with_poison() {
        let mut state = Game::new(10, 13, 250, 8);
        state.effects.push(SPELLS[3].clone());
        let next_state = state.start_turn();

        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp - 3, next_state.boss_hp);
        assert_eq!(state.mana, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.effects.len(), 1);
        assert_eq!(next_state.effects[0].duration, SPELLS[3].duration - 1);
    }

    #[test]
    fn can_start_a_turn_with_recharge() {
        let mut state = Game::new(10, 13, 250, 8);
        state.effects.push(SPELLS[4].clone());
        let next_state = state.start_turn();

        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp - 4, next_state.boss_hp);
        assert_eq!(state.mana + 101, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.effects.len(), 1);
        assert_eq!(next_state.effects[0].duration, SPELLS[4].duration - 1);
    }

    fn can_cast_mangic_missle() {
        let state = Game::new(10, 13, 250, 8);
        let next_state = state.cast(&SPELLS[0]);

        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp - SPELLS[0].damage, next_state.boss_hp);
        assert_eq!(state.mana - SPELLS[0].cost, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.spent, SPELLS[0].cost);
        assert_eq!(next_state.effects.len(), 0);
    }

    #[test]
    fn can_cast_drain() {
        let state = Game::new(10, 13, 250, 8);
        let next_state = state.cast(&SPELLS[1]);

        assert_eq!(state.player_hp + SPELLS[1].heal, next_state.player_hp);
        assert_eq!(state.boss_hp - SPELLS[1].damage, next_state.boss_hp);
        assert_eq!(state.mana - SPELLS[1].cost, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.spent, SPELLS[1].cost);
        assert_eq!(next_state.effects.len(), 0);
    }

    #[test]
    fn can_find_all_spells_to_cast() {
        let state = Game::new(10, 13, 250, 8);
        let spells = state.moves();
        assert_eq!(spells.len(), 5);
    }

    #[test]
    fn can_find_only_two_spells() {
        let state = Game::new(10, 13, 73, 8);
        let spells = state.moves();
        assert_eq!(spells.len(), 2);
    }

    #[test]
    fn can_cast_poison() {
        let state = Game::new(10, 13, 250, 8);
        let next_state = state.cast(&SPELLS[3]);

        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp, next_state.boss_hp);
        assert_eq!(state.mana - SPELLS[3].cost, next_state.mana);
        assert_eq!(state.spent + SPELLS[3].cost, next_state.spent);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.effects.len(), 1);
        assert_eq!(next_state.effects[0].damage, 3);
        assert_eq!(next_state.spent, SPELLS[3].cost);
    }

    #[test]
    fn cheapest_win_without_mana() {
        assert!(Game::new(10, 13, 0, 8).cheapest_win(i64::MAX).is_none());
    }
}
