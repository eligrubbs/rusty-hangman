
pub mod dictionary;
pub mod game;
use game::{Game, GameResult};

pub fn run(secret_len: usize) {
    let mut game: Game = Game::new(secret_len);

    game.start_screen();

    let result: GameResult = loop {
        match game.is_over() {
            Some(x) => break x,
            None => {
                game.print_game_screen();
                game.let_user_guess();
            }
        }
    };

    game.print_game_screen();

    match result {
        GameResult::Guessed => {
            println!("You guessed the secret word: {}", game.get_secret())
        }
        GameResult::Stumped => {
            println!("You failed to guess {}... Better luck next time.", game.get_secret())
        }
    }
}
