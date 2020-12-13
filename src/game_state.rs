use envconfig::Envconfig;
use log::{debug, trace};


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
        debug!("initializing game state with word: {}", word);
        debug!("initializing game with max guesses: {}", config.max_incorrect_guesses);
        let word_length = (&word).len() as usize;
        let mut lower_word = word.clone();
        lower_word.make_ascii_lowercase();
        GameState {
            hidden_word : lower_word,
            guessed_word : "_".repeat(word_length),
            incorrect_guesses : 0,
            game_config : config
        }
    }

    fn is_correct_guess(&self, letter: char) -> bool {
        let pos_vec: Vec<usize> = Vec::new();
        // for i in self.
        // rewrite this to return some kind of a list to positions where the guess is found
        true
    }

    pub fn incorrect_guesses(&self) -> u16 {
        self.incorrect_guesses
    }

    pub fn guess_letter(&mut self, letter: char) -> bool {
        trace!("guessed {}", letter);
        // see comment above
        // change this logic so it operates based on the length of the list
        // i.e., len == 0 => incorrect guess
        // len > 0 => correct guess
        // then fill in letters
        let is_correct = self.is_correct_guess(letter);
        if !is_correct
        {
            self.incorrect_guesses = self.incorrect_guesses + 1;
        }
        is_correct
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
        let guessed_correctly = game_state.guess_letter('z');
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
