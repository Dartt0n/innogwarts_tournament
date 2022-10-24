mod test;

use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines, Write},
};

const INVALID_INPUT: &'static str = "Invalid inputs";
const CANT_PLAY: &'static str = "This player can't play";
const FROZEN_PLAYER: &'static str = "This player is frozen";
const DIFFERENT_TEAM: &'static str = "Both players should be from the same team";
const TRY_HEAL_ITSELF: &'static str = "The player cannot heal itself";
const TRY_SUPER_ITSELF: &'static str = "The player cannot do super action with itself";

#[derive(PartialEq, Clone)]
struct Player {
    team_number: u64,
    power: u64,
    is_visible: bool,
}

impl Player {
    fn equal(&self, other: &RefMut<Player>) -> bool {
        self.team_number == other.team_number
            && self.power == other.power
            && self.is_visible == other.is_visible
    }

    fn attack(&mut self, other: &mut RefMut<Player>) -> Result<(), &'static str> {
        if !self.is_visible {
            return Err(CANT_PLAY);
        }
        if self.power == 0 {
            return Err(FROZEN_PLAYER);
        }

        if !other.is_visible {
            self.power = 0;
            return Ok(());
        }

        if self.power > other.power {
            self.power = 1000.min(self.power + self.power - other.power);
            other.power = 0;
        } else if self.power < other.power {
            other.power = 1000.min(other.power + other.power - self.power);
            self.power = 0;
        } else {
            self.power = 0;
            other.power = 0;
        }

        Ok(())
    }

    fn heal(&mut self, other: &mut RefMut<Player>) -> Result<(), &'static str> {
        if !self.is_visible {
            return Err(CANT_PLAY);
        }

        if self.power == 0 {
            return Err(FROZEN_PLAYER);
        }

        if self.team_number != other.team_number {
            return Err(DIFFERENT_TEAM);
        }

        if self.equal(&other) {
            return Err(TRY_HEAL_ITSELF);
        }

        let heal_points = ((self.power as f64) / 2.0).ceil() as u64;
        self.power = heal_points;
        other.power = 1000.min(heal_points + other.power);

        Ok(())
    }

    fn flip_visibility(&mut self) -> Result<(), &'static str> {
        if self.power == 0 {
            return Err(FROZEN_PLAYER);
        }

        self.is_visible = !self.is_visible;

        Ok(())
    }
}

struct Game {
    teams: Vec<String>,
    players: HashMap<String, RefCell<Player>>,
    warnings: Vec<&'static str>,
    super_player_counter: u64,
}

fn next_string(buffer: &mut Lines<BufReader<&mut File>>) -> Result<String, &'static str> {
    Ok(buffer
        .next()
        .ok_or(INVALID_INPUT)?
        .map_err(|_| INVALID_INPUT)?)
}

impl Game {
    fn run(data_source: &mut File) -> Result<Self, &'static str> {
        let mut game = Self {
            teams: Vec::new(),
            players: HashMap::new(),
            warnings: Vec::new(),
            super_player_counter: 0,
        };

        let mut buffer = BufReader::new(data_source).lines();

        let total_teams: u64 = next_string(&mut buffer)?
            .parse()
            .map_or_else(|_| Err(INVALID_INPUT), Game::validate_total_teams)?;

        for _ in 0..total_teams {
            game.teams
                .push(Game::validate_name(&next_string(&mut buffer)?)?.clone())
        }

        let total_playes: u64 = next_string(&mut buffer)?.parse().map_or_else(
            |_| Err(INVALID_INPUT),
            |m| Game::validate_total_players(total_teams, m),
        )?;

        for _ in 0..total_playes {
            let name: String = Game::validate_name(&next_string(&mut buffer)?)?.clone();

            let team_number: u64 = next_string(&mut buffer)?.parse().map_or_else(
                |_| Err(INVALID_INPUT),
                |tn| Game::validate_team_number(total_teams, tn),
            )?;

            let power: u64 = next_string(&mut buffer)?
                .parse()
                .map_or_else(|_| Err(INVALID_INPUT), Game::validate_power)?;

            let is_visible: bool = Game::validate_visibility(next_string(&mut buffer)?)?;

            game.players.insert(
                name,
                RefCell::new(Player {
                    team_number,
                    power,
                    is_visible,
                }),
            );
        }

        loop {
            let command_string = match next_string(&mut buffer) {
                Ok(string) => string,
                Err(_) => break,
            };

            let words = command_string.split(" ").collect::<Vec<&str>>();

            if words.len() < 2 || words.len() >= 4 {
                return Err(INVALID_INPUT);
            }

            match words[0] {
                "attack" => {
                    if words.len() != 3 {
                        return Err(INVALID_INPUT);
                    }

                    let mut action_player = game
                        .players
                        .get(words[1])
                        .ok_or(INVALID_INPUT)?
                        .borrow_mut();
                    let mut target_player = game
                        .players
                        .get(words[2])
                        .ok_or(INVALID_INPUT)?
                        .borrow_mut();

                    match action_player.attack(&mut target_player) {
                        Err(e) => game.warnings.push(e),
                        _ => {}
                    }
                }
                "heal" => {
                    if words.len() != 3 {
                        return Err(INVALID_INPUT);
                    }

                    let mut action_player = game
                        .players
                        .get(words[1])
                        .ok_or(INVALID_INPUT)?
                        .borrow_mut();
                    let mut target_player = game
                        .players
                        .get(words[2])
                        .ok_or(INVALID_INPUT)?
                        .borrow_mut();

                    match action_player.heal(&mut target_player) {
                        Err(e) => game.warnings.push(e),
                        _ => {}
                    }
                }
                "flip_visibility" => {
                    if words.len() != 2 {
                        return Err(INVALID_INPUT);
                    }

                    match game
                        .players
                        .get_mut(words[1])
                        .ok_or(INVALID_INPUT)?
                        .get_mut()
                        .flip_visibility()
                    {
                        Err(e) => game.warnings.push(e),
                        _ => {}
                    }
                }
                "super" => {
                    if words.len() != 3 {
                        return Err(INVALID_INPUT);
                    }

                    let _action_cell = game.players.get(words[1]).cloned().ok_or(INVALID_INPUT)?;
                    let action_player = _action_cell.borrow();

                    let _target_cell = game.players.get(words[2]).cloned().ok_or(INVALID_INPUT)?;
                    let target_player = _target_cell.borrow();

                    if !action_player.is_visible {
                        game.warnings.push(CANT_PLAY);
                        continue;
                    }
                    if action_player.power == 0 {
                        game.warnings.push(FROZEN_PLAYER);
                        continue;
                    }
                    if action_player.team_number != target_player.team_number {
                        game.warnings.push(DIFFERENT_TEAM);
                        continue;
                    }
                    if words[1] == words[2] {
                        game.warnings.push(TRY_SUPER_ITSELF);
                        continue;
                    }

                    game.players.remove(words[1]);
                    game.players.remove(words[2]);

                    let new_player = Player {
                        power: 1000.min(action_player.power + target_player.power),
                        is_visible: true,
                        team_number: action_player.team_number,
                    };
                    game.players.insert(
                        format!("S_{}", game.super_player_counter),
                        RefCell::new(new_player),
                    );

                    game.super_player_counter += 1;
                }
                _ => return Err(INVALID_INPUT),
            }
        }

        Ok(game)
    }

    fn validate_total_teams(value: u64) -> Result<u64, &'static str> {
        match value {
            1..=10 => Ok(value),
            _ => Err(INVALID_INPUT),
        }
    }

    fn validate_total_players(teams: u64, value: u64) -> Result<u64, &'static str> {
        if teams <= value && value <= 100 {
            Ok(value)
        } else {
            Err(INVALID_INPUT)
        }
    }

    fn validate_name(name: &String) -> Result<&String, &'static str> {
        if !(2 <= name.len() && name.len() <= 20) {
            return Err(INVALID_INPUT);
        }
        let mut chars = name.chars();

        if !chars.next().unwrap().is_ascii_uppercase() {
            return Err(INVALID_INPUT);
        }

        for symbol in chars {
            if !symbol.is_alphabetic() {
                return Err(INVALID_INPUT);
            }
        }

        Ok(name)
    }

    fn validate_team_number(total_teams: u64, value: u64) -> Result<u64, &'static str> {
        if value < total_teams {
            Ok(value)
        } else {
            Err(INVALID_INPUT)
        }
    }

    fn validate_power(value: u64) -> Result<u64, &'static str> {
        match value {
            0..=1000 => Ok(value),
            _ => Err(INVALID_INPUT),
        }
    }

    fn validate_visibility(value: String) -> Result<bool, &'static str> {
        if value.eq("False") {
            Ok(false)
        } else if value.eq("True") {
            Ok(true)
        } else {
            Err(INVALID_INPUT)
        }
    }

    fn get_winner_index<'a>(&self) -> Option<usize> {
        let mut team_powers: Vec<u64> = Vec::with_capacity(self.teams.len());

        for _ in 0..self.teams.len() {
            team_powers.push(0)
        }

        for player_cell in self.players.values() {
            let player = player_cell.borrow();
            team_powers[player.team_number as usize] += player.power;
        }

        let max_power = team_powers.iter().max().unwrap_or(&0u64);

        let winners: Vec<usize> = team_powers
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == *max_power)
            .map(|(i, _)| i)
            .collect();

        if winners.len() == 1 {
            Some(winners[0])
        } else {
            None
        }
    }
}

fn solution(input_file: &str, output_file: &str) {
    let mut input = File::open(input_file).expect("File does not exist");
    let mut output = File::create(output_file).expect("Failed to create output file");

    let game = match Game::run(&mut input) {
        Err(e) => {
            writeln!(&mut output, "{}", e).unwrap();
            return;
        }
        Ok(g) => g,
    };

    for warning in &game.warnings {
        writeln!(&mut output, "{}", warning).unwrap();
    }

    writeln!(
        &mut output,
        "{}",
        match game.get_winner_index() {
            Some(winner) => format!("The chosen wizard is {}", &game.teams[winner]),
            None => "It's a tie".to_string(),
        }
    )
    .unwrap();
}

fn main() {
    solution("input.txt", "output.txt");
}
