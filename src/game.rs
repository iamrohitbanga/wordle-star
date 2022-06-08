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
        let actual_length = target_word.chars().count();
        if dict.word_length != actual_length {
            panic!("target word length incorrect. Actual: {0}, Expected: {1}", actual_length, dict.word_length);
        }

        if !dict.contains(&target_word) {
            panic!("target word not present in dictionary");
        }

        return Game {
            dict: dict,
            keymap: HashMap::new(),
            target_word: target_word,
        };
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
    #[should_panic(expected="target word length incorrect. Actual: 4, Expected: 3")]
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
}
