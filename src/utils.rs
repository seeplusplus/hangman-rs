pub fn search_word(word: &String, letter: char) -> Vec<usize> {
    let mut pos_vec: Vec<usize> = Vec::new();
    let mut substr_index_start = 0;
    let mut substr_loc: usize;
    loop
    {
        match word[substr_index_start..].find(letter) {
            Some(i) => substr_loc = i,
            None => break
        }
        let first_loc = substr_loc + substr_index_start;
        pos_vec.push(first_loc);
        if word.rfind(letter).unwrap() != first_loc {
            substr_index_start = first_loc + 1;
        }
        else {
            break;
        }
    }
    pos_vec
}

pub trait CharExt {
    fn to_other_case(&self) -> char;
}

impl CharExt for char {
    fn to_other_case(&self) -> char {
        return if self.is_ascii_lowercase() {
            self.to_ascii_uppercase()
        }
        else if self.is_ascii_uppercase() {
            self.to_ascii_lowercase()
        }
        else {
            self.clone()
        }
    }
}
