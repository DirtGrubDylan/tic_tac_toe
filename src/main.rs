extern crate rand;

mod game;

fn main() {
    println!("Welcome to Tic-Tac-Toe!");

    let mut game = game::Game::new();

    game.play_game();
}
