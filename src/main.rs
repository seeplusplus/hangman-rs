mod word_list;
mod game_state;
mod utils;

use word_list::{DefaultWordList, WordList};
use game_state::{GameState, GameStatus};
use std::io::{self, Read, Stdin};

use log::{debug};

fn main() {
    env_logger::init();
    let mut hangman = Hangman::new();
    hangman.play();
}

struct Hangman {
    stdin: Stdin,
    game_state: GameState
}
impl Hangman {
    pub fn new() -> Hangman  {
        debug!("initializing game state");
        Hangman {
            stdin: io::stdin(),
            game_state: GameState::new(DefaultWordList::get_word())
        }
    }
    pub fn display_word(&self) {
        println!("You word is: {}", self.game_state.get_word());
    }
    pub fn get_user_guess(&mut self) -> io::Result<char> {
        let mut buffer = String::new();
        self.stdin.read_to_string(&mut buffer)?;
        let input_char = buffer.to_ascii_lowercase().chars().enumerate().nth(0).unwrap().1;

        Ok(input_char)
    }

    pub fn play(&mut self) {
        let mut guessed_char: char;
        debug!("entering game loop");
        loop {
            self.display_word();
            guessed_char = self.get_user_guess().unwrap();
            self.game_state.guess_letter(guessed_char);

            match self.game_state.game_status() 
            {
                GameStatus::Won => {break;},
                GameStatus::Lost => {break;},
                _ => {}
            }
            self.display_word();
            break;
        }
    }
}

