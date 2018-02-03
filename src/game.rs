//! This a module for setting up a game of Tic-Tac-Toe.
//!
//! It contains an aliased type for the game board, an enum for the game turn, and a struct for the
//! game itself.

/// The game board as an aliased type.
type Board = Vec<Vec<char>>;

/// A turn in the game as an Enum.
#[derive(Debug)]
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
    board: Vec<Vec<char>>,
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
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ],
            current_turn: Turn::Player,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
