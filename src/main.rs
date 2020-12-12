mod word_list;
mod game_state;

use word_list::{DefaultWordList, WordList};
use game_state::{GameState, GameStatus};

use log::{debug};

fn main() {
    env_logger::init();

    debug!("initializing game state");
    let mut game_state = GameState::new(DefaultWordList::get_word());

    let mut guessed_char: char;
    debug!("entering game loop");
    loop {
        guessed_char = get_user_guess();
        game_state.guess_letter(guessed_char);

        match game_state.game_status() 
        {
            GameStatus::Won => {break;},
            GameStatus::Lost => {break;},
            _ => {}
        }
    }
}

fn print_word(word: &String) {
    println!("You word is: {}", word);
}

fn get_user_guess() -> char {
    'e'
}
