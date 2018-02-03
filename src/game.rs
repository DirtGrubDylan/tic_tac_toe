//! This a module for setting up a game of Tic-Tac-Toe.
//!
//! It contains an aliased type for the game board, an enum for the game turn, and a struct for the
//! game itself.
use std::io;

/// The game board as an aliased type.
type Board = Vec<Vec<String>>;

/// A turn in the game as an Enum.
#[derive(Debug, PartialEq)]
enum Turn {
    /// The player's turn.
    Player,
    /// The bot's turn.
    Bot,
}

/// The game represented as a struct.
#[derive(Debug)]
pub struct Game {
    /// The game board.
    board: Board,
    /// The current turn of the game.
    current_turn: Turn,
}

impl Game {
    /// Constructs a `Game` object.
    ///
    /// The board will default to a vector of chars indicating the available moves, and the first
    /// turn will default to the player. For fun, a user could randomize the starting player.
    ///
    /// # Example
    ///
    /// ```
    /// use game::Game;
    ///
    /// let game = Game::new();
    /// ```
    pub fn new() -> Game {
        Game {
            board: vec![
                vec![String::from("1"), String::from("2"), String::from("3")],
                vec![String::from("4"), String::from("5"), String::from("6")],
                vec![String::from("7"), String::from("8"), String::from("9")],
            ],
            current_turn: Turn::Player,
        }
    }

    /// Plays the game.
    pub fn play_game(&mut self) {
        let mut finished = false;

        while !finished {
            self.play_turn();

            if self.game_is_won() {
                self.reset();
                finished = Self::player_is_finished();
            }
        }
    }

    /// Plays a turn of the game, getting moves from the player or bot.
    fn play_turn(&mut self) {
        self.print_board();

        let (valid_token, valid_move) = match self.current_turn {
            Turn::Player => (String::from("X"), self.get_player_move()),
            Turn::Bot => (String::from("O"), self.get_bot_move()),
        };

        let (row, col) = Self::move_to_board_location(valid_move);

        self.board[row][col] = valid_token;
        self.current_turn = self.get_next_turn();
    }

    /// Prints the game board
    ///
    /// # Example
    ///
    /// ```
    /// use game::Game;
    ///
    /// let game = Game::new();
    ///
    /// game.print_board()
    ///
    /// // You should see (including blank lines):
    /// //
    /// // +---+---+---+
    /// // | 1 | 2 | 3 |
    /// // +---+---+---+
    /// // | 4 | 5 | 6 |
    /// // +---+---+---+
    /// // | 7 | 8 | 9 |
    /// // +---+---+---+
    /// //
    /// ```
    fn print_board(&self) {
        let seperator = "+---+---+---+";

        println!("\n{}", seperator);

        for row in &self.board {
            println!("| {} |\n{}", row.join(" | "), seperator);
        }

        print!("\n");
    }

    /// Gets move from player.
    fn get_player_move(&self) -> u32 {
        let mut player_input = String::new();

        loop {
            match io::stdin().read_line(&mut player_input) {
                Err(_) => println!("Error reading input, try again!"),
                Ok(_) => match self.validate_player_input(&player_input) {
                    Err(err) => println!("{}", err),
                    Ok(num) => return num,
                },
            }
        }
    }

    /// Validates player input.
    fn validate_player_input(&self, player_input: &str) -> Result<u32, String> {
        match player_input.parse::<u32>() {
            Err(_) => Err(String::from("Please input a valid unsigned integer!")),
            Ok(number) => {
                if self.is_valid_move(number) {
                    Ok(number)
                } else {
                    Err(String::from(
                        "Please input a number, between 1 and 9, not already chosen!",
                    ))
                }
            }
        }
    }

    /// Gets move from bot.
    fn get_bot_move(&self) -> u32 {
        unimplemented!();
    }

    /// Determins if move is valid.
    fn is_valid_move(&self, unchecked_move: u32) -> bool {
        match unchecked_move {
            1...9 => {
                let temp_location = Self::move_to_board_location(unchecked_move);

                match self.board[temp_location.0][temp_location.1].as_str() {
                    "X" | "O" => false,
                    _ => true,
                }
            },
            _ => false,
        }
    }

    /// Turns a move integer into the respective row and column board location.
    fn move_to_board_location(game_move: u32) -> (usize, usize) {
        let row = (game_move - 1) / 3;
        let col = (game_move - 1) % 3;

        (row as usize, col as usize)
    }

    /// Get the next turn, either the player or bot.
    fn get_next_turn(&self) -> Turn {
        match self.current_turn {
            Turn::Player => Turn::Bot,
            Turn::Bot => Turn::Player,
        }
    }

    /// Determines if game is won.
    fn game_is_won(&self) -> bool {
        unimplemented!();
    }

    /// Determines if player wants to play again.
    fn player_is_finished() -> bool {
        unimplemented!();
    }

    /// Resets the game.
    fn reset(&mut self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_move() {
        let mut test_game = Game::new();

        test_game.board[2][2] = String::from("X");

        for test_move in 1..9 {
            assert!(test_game.is_valid_move(test_move));
        }

        for bad_move in 10..20 {
            assert!(!test_game.is_valid_move(bad_move));
        }
    }

    #[test]
    fn test_validate_player_input() {
        let mut test_game = Game::new();

        test_game.board[2][2] = String::from("X");

        for test_move in 1..9 {
            assert!(test_game.validate_player_input(&test_move.to_string()).is_ok());
        }

        for bad_move in 10..20 {
            assert!(test_game.validate_player_input(&bad_move.to_string()).is_err());
        }
    }

    #[test]
    fn test_move_to_board_location() {
        assert_eq!(Game::move_to_board_location(1), (0, 0));
        assert_eq!(Game::move_to_board_location(2), (0, 1));
        assert_eq!(Game::move_to_board_location(3), (0, 2));

        assert_eq!(Game::move_to_board_location(4), (1, 0));
        assert_eq!(Game::move_to_board_location(5), (1, 1));
        assert_eq!(Game::move_to_board_location(6), (1, 2));

        assert_eq!(Game::move_to_board_location(7), (2, 0));
        assert_eq!(Game::move_to_board_location(8), (2, 1));
        assert_eq!(Game::move_to_board_location(9), (2, 2));
    }

    #[test]
    fn test_get_next_turn() {
        let mut test_game = Game::new();

        assert_eq!(test_game.get_next_turn(), Turn::Bot);

        test_game.current_turn = Turn::Bot;

        assert_eq!(test_game.get_next_turn(), Turn::Player);
    }
}
