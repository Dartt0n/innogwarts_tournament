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

/// Structure that represents player according to the problem description
#[derive(PartialEq, Clone)]
struct Player {
    name: String,
    team_number: u64,
    power: u64,
    is_visible: bool,
}

impl Player {
    /// Compare Player and &RefMut<Player> by comparing every field
    fn equal(&self, other: &RefMut<Player>) -> bool {
        self.name == other.name
            && self.team_number == other.team_number
            && self.power == other.power
            && self.is_visible == other.is_visible
    }

    /// ## A player attacks other player
    /// The player with the highest power gains half difference between powers,
    /// power of the player with the lowest power drops to 0. A player with 0 power
    /// considered as frozen. If the powers of the players are equal, both their powers
    /// drop to 0
    /// ### Returns:
    /// - Ok(()) (if attack was successful)
    /// - Err(message) (otherwise)
    /// ### Possible error messages:
    /// - CANT_PLAY
    ///     (returned if player is invisible so they can not perform attack action)
    /// - FROZEN_PLAYER
    ///     (returned if player is frozen so they can not perform attack action)
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

    /// ## A player heals other player
    /// The target player gains half of the action player's power (ceiled up).
    /// The action player's power drops to the half (ceiled up). Both target and action
    /// players should be from the same team.
    /// ### Returns:
    /// - Ok(()) (if healing was successful)
    /// - Err(message) (otherwise)
    /// ### Possible error messages:
    /// - CANT_PLAY
    ///     (returned if player is invisible so they can not perform healing action)
    /// - FROZEN_PLAYER
    ///     (returned if player is frozen so they can not perform healing action)
    /// - DIFFERENT_TEAM
    ///     (returned if players are from different teams)
    /// - TRY_HEAL_ITSELF
    ///     (returned when the action player tryes to heal itself)
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

    /// ## Player flips its visibility
    /// The action player changes its visibility to the opposite one.
    /// (True to False or backwards)
    /// ### Returns:
    /// - Ok(()) (if fliping was successful)
    /// - Err(message) (otherwise)
    /// ### Possible error messages:
    /// - FROZEN_PLAYER
    ///     (returned if the player is frozen so they can not perform
    /// flip visibility action)
    fn flip_visibility(&mut self) -> Result<(), &'static str> {
        if self.power == 0 {
            return Err(FROZEN_PLAYER);
        }

        self.is_visible = !self.is_visible;

        Ok(())
    }
}

/// Structure representing game
struct Game {
    // Vector of teams
    teams: Vec<String>,
    // It is required to use RefCell here as we need to modify players during the game
    players: HashMap<String, RefCell<Player>>,
    // Vector of warnings, generated while running the game
    warnings: Vec<&'static str>,
    // Counter of existing super players (needed for naming super players)
    super_player_counter: u64,
}
/// Try to read next line from BufReader.
/// If operation was successful, returns the line.
/// Returns error INVALID_INPUT otherwise
fn next_string(buffer: &mut Lines<BufReader<&mut File>>) -> Result<String, &'static str> {
    Ok(buffer
        .next()
        .ok_or(INVALID_INPUT)?
        .map_err(|_| INVALID_INPUT)?)
}

impl Game {
    /// Main logic function representing game process.
    /// Read the problem description to see the steps of the game
    fn run(data_source: &mut File) -> Result<Self, &'static str> {
        let mut game = Self {
            teams: Vec::new(),
            players: HashMap::new(),
            warnings: Vec::new(),
            super_player_counter: 0,
        };

        let mut buffer = BufReader::new(data_source).lines();

        let total_teams: u64 = next_string(&mut buffer)? // Get line with number N and return error if reading failed
            .parse() // Try convert the value into the u64 value
            .map_or_else(
                |_| Err(INVALID_INPUT), // If parsing fails, map parse error to INVALID_INPUT
                Game::validate_total_teams, // Otherwise validate the number.
                                        // If the number if valid, returns Ok(u64) and error(INVALID_INPUT) otherwise
            )?; // If the value is error, return error. Unwrap u64 value otherwise.

        // Get team names from the file, or report
        for _ in 0..total_teams {
            game.teams.push(
                // Validate name
                Game::validate_name(
                    next_string(&mut buffer)?, // Get line with team name. Return error if reading failed
                )?, // Return Ok(&name) if the name is valid and error INVALID_INPUT otherwise
            )
        }

        // Get total amount of players
        let total_playes: u64 = next_string(&mut buffer)? // Reads the next line and returns error if reading failed
            .parse() // Try convert the line into the u64 value
            .map_or_else(
                |_| Err(INVALID_INPUT), // If parsing failed, map parse error into INVALID_INPUT error
                |m| Game::validate_total_players(total_teams, m), // Validate amount of players otherwse
                                                                  // Returns Ok(u64) if the number is valid or error INVALID_INPUT otherwise
            )?; // Return the value if it is correct, return error INVALID_INPUT otherwise

        for _ in 0..total_playes {
            let name: String = Game::validate_name(
                // Validate name
                next_string(&mut buffer)?, // Reads the next line and returns error if reading failed
            )?; // Return name if it is valid and return error INVALID_INPUT otherwise

            let team_number: u64 = next_string(&mut buffer)? // Read next line and return error if readig failed
                .parse() // Try convert the line into the u64 value
                .map_or_else(
                    |_| Err(INVALID_INPUT), // If parsing failed, covert parse error into INVALID_INPUT error
                    |tn| Game::validate_team_number(total_teams, tn), // Validate the number otherwise
                )?; // Returns u64 if the value is valid and returns error INVALID_INPUT otherwise

            let power: u64 = next_string(&mut buffer)? // Reads the next line and returns error if reading failed
                .parse() // Try convert the line into the u64 value
                .map_or_else(
                    |_| Err(INVALID_INPUT), // If parsing failed, convert parse error into INVALID_INPUT error
                    Game::validate_power,   // Validate power otherwise
                )?; // Returns u64 value if power is valid and returns error INVALID_INPUT otherwise

            let is_visible: bool = Game::validate_visibility(
                next_string(&mut buffer)?, // Reads the next line and returns error if reading failed
            )?; // Return bool value if visibility is valid and returns error INVALID_INPUT otherwise

            // Save new player into the players hashmap
            game.players.insert(
                name.clone(), // We need to clone this string
                RefCell::new(Player {
                    name: name, // As value moved here into the structure
                    team_number,
                    power,
                    is_visible,
                }),
            );
        }

        loop {
            // Iterate over the rest of the lines until we meet the end the file
            let command_string = match next_string(&mut buffer) {
                Ok(string) => string,
                Err(_) => break,
            };
            // Split the line into the words
            let words = command_string.split(" ").collect::<Vec<&str>>();
            // All commands contain between 2 and 3 words, so
            // any other amount of words is invalid command
            if words.len() < 2 || words.len() >= 4 {
                return Err(INVALID_INPUT);
            }

            // Match the first word with possible actions
            match words[0] {
                "attack" => {
                    // If action is "attack", the amount of words should be equal to 3
                    if words.len() != 3 {
                        return Err(INVALID_INPUT);
                    }
                    let mut action_player = game
                        .players
                        .get(words[1]) // Try find the 1st player
                        .ok_or(INVALID_INPUT)? // return reference to a player or raise INVALID_INPUT error
                        .borrow_mut(); // borrow as mutable, because we need to modify it during the attack action
                    let mut target_player = game
                        .players
                        .get(words[2]) // Try find the 2nd player
                        .ok_or(INVALID_INPUT)? // return reference to a player or raise INVALID_INPUT error
                        .borrow_mut(); // borrow as mutable, because we need to modify it during the attack action

                    // Perform the attack
                    // If any errors occur, add then as warnings
                    if let Err(e) = action_player.attack(&mut target_player) {
                        game.warnings.push(e);
                    }
                }
                "heal" => {
                    // If the action is "heal", the amount of words should be 3
                    if words.len() != 3 {
                        return Err(INVALID_INPUT);
                    }

                    let mut action_player = game
                        .players
                        .get(words[1]) // Try find the 1st player
                        .ok_or(INVALID_INPUT)? // return reference to a player or raise INVALID_INPUT error
                        .borrow_mut(); // borrow as mutable, because we need to modify it during the heal action

                    let mut target_player = game
                        .players
                        .get(words[2]) // Try find the 2nd player
                        .ok_or(INVALID_INPUT)? // return reference to a player or raise INVALID_INPUT error
                        .borrow_mut(); // borrow as mutable, because we need to modify it during the heal action

                    // Perform heal action and if any errors occur save them to the vector of warnings
                    if let Err(e) = action_player.heal(&mut target_player) {
                        game.warnings.push(e)
                    }
                }
                "flip_visibility" => {
                    // If the action is "flip_visibility" the amount of words should be equal 2
                    if words.len() != 2 {
                        return Err(INVALID_INPUT);
                    }

                    // Perform flip_visibility action
                    // and if any errors occur, save them to the vector of warnings
                    if let Err(e) = game
                        .players
                        .get_mut(words[1])
                        .ok_or(INVALID_INPUT)?
                        .get_mut()
                        .flip_visibility()
                    {
                        game.warnings.push(e)
                    }
                }
                "super" => {
                    // Super action can not be performed inside player actions,
                    // as this action modify global game state (amount of players)

                    // If the action is "super", the amount of words should be equal to 3
                    if words.len() != 3 {
                        return Err(INVALID_INPUT);
                    }

                    // Refcell is needed to be cloned because we are going to modify several objects from
                    // the hashmap at the same time
                    // If there is no such player, raise INVALID_INPUT error
                    let _action_cell = game.players.get(words[1]).cloned().ok_or(INVALID_INPUT)?;
                    // _action_cell should be declared, or the value will be dropped in this scope
                    // player will not be modifed so borrow as immutable
                    let action_player = _action_cell.borrow();

                    let _target_cell = game.players.get(words[2]).cloned().ok_or(INVALID_INPUT)?;
                    let target_player = _target_cell.borrow();

                    // Invisible player can not perform any actions other then flip_visibility
                    if !action_player.is_visible {
                        game.warnings.push(CANT_PLAY);
                        continue;
                    }
                    // Frozen player can not perform any actions
                    if action_player.power == 0 {
                        game.warnings.push(FROZEN_PLAYER);
                        continue;
                    }

                    // Players should be from the same team
                    if action_player.team_number != target_player.team_number {
                        game.warnings.push(DIFFERENT_TEAM);
                        continue;
                    }

                    // Super actions with itself is prohibited
                    // names of players are unique by the task, so we can use them
                    if words[1] == words[2] {
                        game.warnings.push(TRY_SUPER_ITSELF);
                        continue;
                    }

                    // Remove both players from the players hashmap
                    game.players.remove(words[1]);
                    game.players.remove(words[2]);

                    // create a new player and save it to the players of the game
                    let super_player_name = format!("S_{}", game.super_player_counter);
                    game.super_player_counter += 1;

                    game.players.insert(
                        super_player_name.clone(),
                        RefCell::new(Player {
                            name: super_player_name,
                            power: 1000.min(action_player.power + target_player.power),
                            is_visible: true,
                            team_number: action_player.team_number,
                        }),
                    );
                }
                // If unknown command is met, raise INVALID_INPUT error
                _ => return Err(INVALID_INPUT),
            }
        }
        Ok(game)
    }

    /// Validates total amount of teams. By the task, the number of teams
    /// N should be in range \[1, 10\]
    ///
    /// Returns Ok(value) if all conditions are satisfied and Err(INVALID_INPUT) otherwise
    fn validate_total_teams(value: u64) -> Result<u64, &'static str> {
        match value {
            1..=10 => Ok(value),
            _ => Err(INVALID_INPUT),
        }
    }

    // Validates total amount of players. By the task, the number of players
    // M should be in range \[N, 100\];
    ///
    /// Returns Ok(value) if all conditions are satisfied and Err(INVALID_INPUT) otherwise
    fn validate_total_players(teams: u64, value: u64) -> Result<u64, &'static str> {
        if teams <= value && value <= 100 {
            Ok(value)
        } else {
            Err(INVALID_INPUT)
        }
    }

    /// Validates name. By the task, the name should
    /// - should be of length more or equal to 2 and less or equal to 20
    /// - starts with the upper english letter
    /// - consists of only english letters
    ///
    /// Returns Ok(value) if all conditions are satisfied and Err(INVALID_INPUT) otherwise
    fn validate_name(name: String) -> Result<String, &'static str> {
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

    /// Validates team number. Team number is an index of a team, so team number
    /// should be in the range \[0, N\];
    ///
    /// Returns Ok(value) if all conditions are satisfied and Err(INVALID_INPUT) otherwise
    fn validate_team_number(total_teams: u64, value: u64) -> Result<u64, &'static str> {
        // It is unnecessary to check if value >= 0 because all u64 values are >= 0
        if value < total_teams {
            Ok(value)
        } else {
            Err(INVALID_INPUT)
        }
    }

    /// Validates power of a player. By the task, the power should be range \[0, 1000\]
    ///
    /// Returns Ok(value) if all conditions are satisfied and Err(INVALID_INPUT) otherwise
    fn validate_power(value: u64) -> Result<u64, &'static str> {
        match value {
            0..=1000 => Ok(value),
            _ => Err(INVALID_INPUT),
        }
    }

    /// Validates visibility of a player. By the task, the visibility value
    /// shoule be either "True" or "False"
    ///
    /// Returns Ok(value) if all conditions are satisfied and Err(INVALID_INPUT) otherwise
    fn validate_visibility(value: String) -> Result<bool, &'static str> {
        if value.eq("False") {
            Ok(false)
        } else if value.eq("True") {
            Ok(true)
        } else {
            Err(INVALID_INPUT)
        }
    }

    /// Finds the winner team. The winner team is a team with the largest
    /// sum of the players' powers. If top two or more teams has equal powers
    /// return is undefined, function returns None.
    fn get_winner_index<'a>(&self) -> Option<usize> {
        // Create array for calculating total scores
        let mut team_powers: Vec<u64> = Vec::with_capacity(self.teams.len());
        for _ in 0..self.teams.len() {
            team_powers.push(0)
        }
        // Iterate over every players
        for player_cell in self.players.values() {
            let player = player_cell.borrow();
            // Add player's power to its team
            team_powers[player.team_number as usize] += player.power;
        }

        // Find maximum value
        let max_power = team_powers.iter().max().unwrap_or(&0u64);

        // Get all teams with the maximum power
        let winners: Vec<usize> = team_powers
            .iter()
            .enumerate() // Transform iterator of values into the iterator of pairs (index, value)
            .filter(|(_, &v)| v == *max_power) // Accept only teams with the maximum power
            .map(|(i, _)| i) // Transform iterator of pairs into the iterator of indexes
            .collect(); // Collect iterator to the vector

        // If we have the only winner, return its index and None otherwise
        if winners.len() == 1 {
            Some(winners[0])
        } else {
            None
        }
    }
}

fn solution(input_file: &str, output_file: &str) {
    // Open input file for reading and output file for writing
    let mut input = File::open(input_file).expect("File does not exist");
    let mut output = File::create(output_file).expect("Failed to create output file");

    let game = match Game::run(&mut input) {
        Err(e) => {
            // If we catched error during game process report only the error
            writeln!(&mut output, "{}", e).unwrap();
            return;
        }
        // Unpack game object otherwise
        Ok(g) => g,
    };

    for warning in &game.warnings {
        // Report all warnings
        writeln!(&mut output, "{}", warning).unwrap();
    }

    // Write final score to the file
    writeln!(
        &mut output,
        "{}",
        match game.get_winner_index() {
            // Print the chosen wizard, if it was found
            Some(winner) => format!("The chosen wizard is {}", &game.teams[winner]),
            // Print `it's a tie` otherwise
            None => "It's a tie".to_string(),
        }
    )
    .unwrap();

    // Files are automaticly closed here, while being destroyed
}

fn main() {
    solution("input.txt", "output.txt");
}
