use crate::dictionary::Dictionary;
use std::collections::HashMap;

pub enum KeyState {
    Unknown,
    DoesNotExist,
    ExistsAtPositions(Vec<u16>),
    DoesNotExistAtPositions(Vec<u16>),
}

pub struct Game<'a> {
    dict: &'a Dictionary,
    keymap: HashMap<char, KeyState>,
    target_word: String,
}

impl<'a> Game<'a> {
    pub fn new(dict: &'a Dictionary, target_word: String) -> Game<'a> {
        if !Game::is_word_allowed_in_dict(&dict, &target_word) {
            panic!("target word not present in dictionary");
        }

        return Game {
            dict: dict,
            keymap: HashMap::new(),
            target_word: target_word,
        };
    }

    fn is_word_allowed_in_dict(dict: &Dictionary, word: &String) -> bool {
        dict.contains(&word)
    }

    fn is_word_allowed(&self, word: &String) -> bool {
        Game::is_word_allowed_in_dict(&self.dict, &word)
    }

    pub fn guess_word(&self, word: &String) -> bool {
        if !self.is_word_allowed(&word) {
            return false;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_setup() {
        let mut dict = Dictionary::new(3);
        dict.add_word_str("rat");
        dict.add_word_str("dog");
        dict.add_word_str("cat");

        Game::new(&dict, "dog".to_string());
    }

    #[test]
    #[should_panic(expected="target word not present in dictionary")]
    fn test_target_word_length_not_same() {
        let mut dict = Dictionary::new(3);
        dict.add_word_str("rat");
        dict.add_word_str("dog");
        dict.add_word_str("cat");

        Game::new(&dict, "star".to_string());
    }

    #[test]
    #[should_panic(expected="target word not present in dictionary")]
    fn test_target_word_not_in_dictionary() {
        let mut dict = Dictionary::new(3);
        dict.add_word_str("rat");
        dict.add_word_str("dog");
        dict.add_word_str("cat");

        Game::new(&dict, "mat".to_string());
    }

    #[test]
    fn test_guess_invalid_word() {
        let dict = sample_dict();
        let game = Game::new(&dict, "mat".to_string());
        assert_eq!(false, game.guess_word(&"abc".to_string()));
    }

    #[test]
    fn test_guess_valid_word() {
        let dict = sample_dict();
        let game = Game::new(&dict, "mat".to_string());
        assert_eq!(true, game.guess_word(&"sat".to_string()));
    }

    fn sample_dict() -> Dictionary {
        let mut dict = Dictionary::new(3);
        dict.add_word_str("rat");
        dict.add_word_str("sat");
        dict.add_word_str("mat");
        dict.add_word_str("cat");
        dict
    }
}
