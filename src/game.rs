//! This a module for setting up a game of Tic-Tac-Toe.
//!
//! It contains an aliased type for the game board, an enum for the game turn, and a struct for the
//! game itself.

/// The game board as an aliased type.
type Board = Vec<Vec<char>>;

/// A turn in the game as an Enum.
#[derive(Debug)]
enum Turn {
    Player,
    Bot,
}

/// The game represented as a struct.
#[derive(Debug)]
pub struct Game {
    board: Vec<Vec<char>>,
    current_turn: Turn,
}

impl Game {
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
