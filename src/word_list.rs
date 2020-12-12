pub struct DefaultWordList {
}

pub trait WordList {
    fn get_word() -> String;
}

impl WordList for DefaultWordList {
    fn get_word() -> String {
        String::from("Hello")
    }
}
