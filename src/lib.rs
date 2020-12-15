//! # Hangman
//! A crate for playing the game hangman or for creating implementations of the hangman game.
pub mod game_state;
pub mod utils;
pub mod word_list;

pub use envconfig::Envconfig;

/// "Versions" of the game.
pub trait HangmanClient {
    /// Give the calling code a method to show the user their word so far.
    fn display_word();
    /// Give the calling code a way for the user to enter a guess.
    fn get_player_guess();
}

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "MAX_GUESSES")]
    pub max_incorrect_guesses: u16
}
impl Default for Config {
    fn default() -> Self {
        Config {
            max_incorrect_guesses : 6
        }
    }
}
