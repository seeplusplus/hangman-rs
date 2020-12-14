use envconfig::Envconfig;
use log::{debug};
use crate::utils;


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

pub struct GameState {
    hidden_word: String,
    guessed_word: String,
    incorrect_guesses: u16,
    game_config: Config
}

#[derive(PartialEq)]
pub enum GameStatus {
    Won,
    Lost,
    Playing
}
impl GameState {
    pub fn new(word: String) -> GameState
    {
        let config = Config::init_from_env().unwrap_or(Config::default());
        let word_length = (&word).len() as usize;
        debug!("initializing game state with word: {}", word.to_ascii_lowercase());
        debug!("initializing game with max guesses: {}", config.max_incorrect_guesses);
        GameState {
            hidden_word : word.to_ascii_lowercase(),
            guessed_word : "_".repeat(word_length),
            incorrect_guesses : 0,
            game_config : config
        }
    }

    pub fn incorrect_guesses(&self) -> u16 {
        self.incorrect_guesses
    }

    pub fn get_word(&self) -> &String {
        &self.guessed_word
    }

    pub fn guess_letter(&mut self, letter: char) -> bool {
        debug!("guessed {}", letter);
        if let Some(positions) = utils::search_word(&self.hidden_word, letter) {
            debug!("{} found at {:?}", letter, positions);
            // let is_correct = !positions.is_empty();
            debug!("before change {}", self.guessed_word);
            self.guessed_word = self.guessed_word.chars().enumerate()
                .map(|(i, c)| if positions.contains(&i) { letter } else { c })
                .collect();
            debug!("after change {}", self.guessed_word);
            true
        } else {
            self.incorrect_guesses = self.incorrect_guesses + 1;
            false
        }
    }

    pub fn game_status(&self) -> GameStatus {
        if self.guessed_word == self.hidden_word {
            GameStatus::Won
        }
        else if self.incorrect_guesses >= self.game_config.max_incorrect_guesses {
            GameStatus::Lost
        }
        else {
            GameStatus::Playing
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::game_state::*;

    #[test]
    pub fn initialize_new_gamestate() {
        let game_state = GameState::new(String::from("Hello"));
        assert!(game_state.incorrect_guesses() == 0);
    }

    #[test]
    pub fn test_guess_positive() {
        let mut game_state = GameState::new(String::from("Hello"));
        let incorrect_before_guess = game_state.incorrect_guesses();
        let guessed_correctly = game_state.guess_letter('l');
        let incorrect_after_guess = game_state.incorrect_guesses();

        assert!((incorrect_after_guess - incorrect_before_guess == 0) && guessed_correctly);
    }

    #[test]
    pub fn test_guess_negative() {
        let mut game_state = GameState::new(String::from("Hello"));
        let incorrect_before_guess = game_state.incorrect_guesses();
        let guessed_correctly = game_state.guess_letter('z');
        let incorrect_after_guess = game_state.incorrect_guesses();

        assert!((incorrect_after_guess - incorrect_before_guess == 1) && !guessed_correctly);
    }

    #[test]
    pub fn test_game_status_win () {
        let mut game_state = GameState::new(String::from("Hello"));
        game_state.guess_letter('h');
        game_state.guess_letter('e');
        game_state.guess_letter('l');
        game_state.guess_letter('o');
        assert!(game_state.game_status() == GameStatus::Won);
    }

    #[test]
    pub fn test_game_status_playing () {
        let mut game_state = GameState::new(String::from("Hello"));
        assert!(game_state.game_status() == GameStatus::Playing);
    }

    #[test]
    pub fn test_game_status_lost () {
        let mut game_state = GameState::new(String::from("Hello"));
        game_state.guess_letter('z');
        game_state.guess_letter('z');
        game_state.guess_letter('z');
        game_state.guess_letter('z');
        game_state.guess_letter('z');
        game_state.guess_letter('z');
        assert!(game_state.game_status() == GameStatus::Lost);
    }
}
