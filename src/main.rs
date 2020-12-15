use hangman::word_list::{DefaultWordList, WordList};
use hangman::game_state::{GameState, GameStatus};
use std::io::{self, Stdin};

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
    pub fn display_status(&self) {
        match self.game_state.game_status() {
            GameStatus::Won => {
                println!("You have won! The word was \"{}\"", self.game_state.get_word());
            },
            GameStatus::Lost => {
                println!("You have lost! Try again!");
            },
            GameStatus::Playing => {
                println!("The word is {}", self.game_state.get_word());
            }
        }
    }
    pub fn get_user_guess(&mut self) -> char {
        let mut c: char = ' ';
        println!("Enter your guess...");
        while !c.is_alphabetic() {
            let mut buffer = String::new();
            self.stdin.read_line(&mut buffer).unwrap();
            c = buffer.to_ascii_lowercase().chars().nth(0).unwrap_or(' ');
        }
        c
    }

    pub fn play(&mut self) {
        let mut guessed_char: char;
        debug!("entering game loop");
        loop {
            self.display_status();
            guessed_char = self.get_user_guess();
            self.game_state.guess_letter(guessed_char);

            match self.game_state.game_status() 
            {
                GameStatus::Won => {break;},
                GameStatus::Lost => {break;},
                _ => {}
            }
        }
        self.display_status();
    }
}

