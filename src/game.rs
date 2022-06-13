use crate::dictionary::Dictionary;
use crate::guess::CharState;
use crate::guess::GuessResult;
use crate::keyboard_view::KeyboardView;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Game<'a> {
    dict: &'a Dictionary,
    target_word: String,
    target_positions_map: HashMap<char, HashSet<usize>>,
    keyboard_view: KeyboardView,
    guess_results: Vec<GuessResult>,
}

impl<'a> Game<'a> {
    pub fn new(dict: &'a Dictionary, target_word: &str) -> Game<'a> {
        if !Game::is_word_allowed_in_dict(&dict, &target_word) {
            panic!("target word not present in dictionary");
        }
        let positions_map = Game::compute_char_positions_map(&target_word);
        println!("positions map defined: {:?}", positions_map);
        return Game {
            dict: dict,
            target_word: target_word.to_string(),
            target_positions_map: positions_map,
            keyboard_view: KeyboardView::new(),
            guess_results: vec![],
        };
    }

    fn is_word_allowed_in_dict(dict: &Dictionary, word: &str) -> bool {
        dict.contains(&word.to_string())
    }

    fn is_word_allowed(&self, word: &str) -> bool {
        Game::is_word_allowed_in_dict(&self.dict, &word)
    }

    pub fn guess_word(&mut self, word: &str) -> Option<GuessResult> {
        if !self.is_word_allowed(&word) {
            return None;
        }

        let guess_result = self.compute_char_guesses(&word);

        for char_guess in (&guess_result.char_guesses).into_iter() {
            self.keyboard_view.record_guess(&char_guess);
        }

        self.guess_results.push(guess_result.clone());

        Some(guess_result)
    }

    fn compute_char_guesses(&mut self, word: &str) -> GuessResult {
        let mut char_guesses = vec![];

        for ch in word.chars() {
            char_guesses.push((ch, CharState::NotFound));
        }

        let guess_map = Game::compute_char_positions_map(&word);

        for (&ch, target_positions) in &self.target_positions_map {
            match guess_map.get(&ch) {
                None => (),
                Some(guess_positions) => {
                    let intersection = guess_positions.intersection(&target_positions);
                    let mut intersection_len = 0;
                    for correct_position in intersection {
                        char_guesses[*correct_position] = (ch, CharState::CorrectPosition);
                        intersection_len += 1;
                    }

                    // how many occurrences of ch in target but not in the intersection?
                    let extra_count = target_positions.len() - intersection_len;
                    // positions for this character in guess but not in target
                    let diff = guess_positions.difference(target_positions);
                    // there are extra positions for this character that were not counted in intersection.
                    // there are also some occurences of this character in the guess.
                    // pick extra_count positions from the diff, pick the smaller ones.
                    let mut sorted_diff = diff.map(|x| *x).collect::<Vec<usize>>();
                    sorted_diff.sort();

                    if extra_count > 0 {
                        let trimmed_length = cmp::min(extra_count, sorted_diff.len());
                        for incorrect_position in sorted_diff[0..trimmed_length].into_iter() {
                            char_guesses[*incorrect_position] = (ch, CharState::IncorrectPosition);
                        }
                    }
                }
            }
        }

        GuessResult::new(char_guesses)
    }

    fn compute_char_positions_map(word: &str) -> HashMap<char, HashSet<usize>> {
        let mut map = HashMap::new();
        for (index, ch) in word.chars().enumerate() {
            println!("index: {}, char: {}", index, ch);
            if let None = map.get(&ch) {
                map.insert(ch, HashSet::new());
            }

            let positions = map.get_mut(&ch).unwrap();
            positions.insert(index);
        }
        map
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

        Game::new(&dict, "dog");
    }

    #[test]
    #[should_panic(expected = "target word not present in dictionary")]
    fn test_target_word_length_not_same() {
        let mut dict = Dictionary::new(3);
        dict.add_word_str("rat");
        dict.add_word_str("dog");
        dict.add_word_str("cat");

        Game::new(&dict, "star");
    }

    #[test]
    #[should_panic(expected = "target word not present in dictionary")]
    fn test_target_word_not_in_dictionary() {
        let mut dict = Dictionary::new(3);
        dict.add_word_str("rat");
        dict.add_word_str("dog");
        dict.add_word_str("cat");

        Game::new(&dict, "mat");
    }

    #[test]
    fn test_guess_invalid_word() {
        let dict = basic_dict();
        let mut game = Game::new(&dict, "mat");
        assert_eq!(true, game.guess_word("abc").is_none());
    }

    #[test]
    fn test_guess_valid_word() {
        let dict = basic_dict();
        let mut game = Game::new(&dict, "mat");
        assert_eq!(false, game.guess_word("sat").is_none());
    }

    #[test]
    fn test_char_guesses_for_colon() {
        let dict = big_dict();

        let mut game = Game::new(&dict, "colon");
        assert_char_guesses(
            &GuessResult::new(vec![
                ('c', CharState::CorrectPosition),
                ('l', CharState::IncorrectPosition),
                ('o', CharState::IncorrectPosition),
                ('n', CharState::IncorrectPosition),
                ('e', CharState::NotFound),
            ]),
            game.guess_word("clone"),
        );
        assert_char_guesses(
            &GuessResult::new(vec![
                ('s', CharState::NotFound),
                ('p', CharState::NotFound),
                ('o', CharState::IncorrectPosition),
                ('o', CharState::CorrectPosition),
                ('n', CharState::CorrectPosition),
            ]),
            game.guess_word("spoon"),
        );

        // test with three occurences of same character
        // when the target word only contains it twice
        assert_char_guesses(
            &GuessResult::new(vec![
                ('o', CharState::IncorrectPosition),
                ('v', CharState::NotFound),
                ('o', CharState::IncorrectPosition),
                ('l', CharState::IncorrectPosition),
                ('o', CharState::NotFound),
            ]),
            game.guess_word("ovolo"),
        );
    }

    #[test]
    fn test_char_guesses_for_clone() {
        let dict = big_dict();

        let mut game = Game::new(&dict, "clone");

        assert_char_guesses(
            &GuessResult::new(vec![
                ('c', CharState::CorrectPosition),
                ('o', CharState::IncorrectPosition),
                ('l', CharState::IncorrectPosition),
                ('o', CharState::NotFound), // o has already been counted once
                ('n', CharState::IncorrectPosition),
            ]),
            game.guess_word("colon"),
        );

        assert_char_guesses(
            &GuessResult::new(vec![
                ('s', CharState::NotFound),
                ('p', CharState::NotFound),
                ('o', CharState::CorrectPosition),
                ('o', CharState::NotFound), // o has already been counted once
                ('n', CharState::IncorrectPosition),
            ]),
            game.guess_word("spoon"),
        );

        // test with three occurences of same character
        // when the target word only contains it twice.
        // One of the occurences is in the right position.
        assert_char_guesses(
            &GuessResult::new(vec![
                ('o', CharState::NotFound),
                ('v', CharState::NotFound),
                ('o', CharState::CorrectPosition),
                ('l', CharState::IncorrectPosition),
                ('o', CharState::NotFound),
            ]),
            game.guess_word("ovolo"),
        );

        assert_char_guesses(
            &GuessResult::new(vec![
                ('s', CharState::NotFound),
                ('i', CharState::NotFound),
                ('e', CharState::IncorrectPosition),
                ('n', CharState::CorrectPosition),
                ('a', CharState::NotFound),
            ]),
            game.guess_word("siena"),
        );
    }

    #[test]
    fn test_char_guesses_for_ovolo() {
        let dict = big_dict();

        let mut game = Game::new(&dict, "ovolo");

        assert_char_guesses(
            &GuessResult::new(vec![
                ('c', CharState::NotFound),
                ('o', CharState::IncorrectPosition),
                ('l', CharState::IncorrectPosition),
                ('o', CharState::IncorrectPosition),
                ('n', CharState::NotFound),
            ]),
            game.guess_word("colon"),
        );
        assert_keyboard_view("c", "ol", "", &game.keyboard_view);

        assert_char_guesses(
            &GuessResult::new(vec![
                ('s', CharState::NotFound),
                ('p', CharState::NotFound),
                ('o', CharState::CorrectPosition),
                ('o', CharState::IncorrectPosition),
                ('n', CharState::NotFound),
            ]),
            game.guess_word("spoon"),
        );
        assert_keyboard_view("spn", "l", "o", &game.keyboard_view);

        assert_char_guesses(
            &GuessResult::new(vec![
                ('p', CharState::NotFound),
                ('o', CharState::IncorrectPosition),
                ('t', CharState::NotFound),
                ('o', CharState::IncorrectPosition),
                ('o', CharState::CorrectPosition),
            ]),
            game.guess_word("potoo"),
        );
        assert_keyboard_view("spnt", "l", "o", &game.keyboard_view);

        assert_char_guesses(
            &GuessResult::new(vec![
                ('s', CharState::NotFound),
                ('i', CharState::NotFound),
                ('e', CharState::NotFound),
                ('n', CharState::NotFound),
                ('a', CharState::NotFound),
            ]),
            game.guess_word("siena"),
        );
        assert_keyboard_view("siepnta", "l", "o", &game.keyboard_view);

        assert_char_guesses(
            &GuessResult::new(vec![
                ('o', CharState::CorrectPosition),
                ('v', CharState::CorrectPosition),
                ('o', CharState::CorrectPosition),
                ('l', CharState::CorrectPosition),
                ('o', CharState::CorrectPosition),
            ]),
            game.guess_word("ovolo"),
        );
        assert_keyboard_view("spnt", "", "ovl", &game.keyboard_view);
    }

    fn assert_char_guesses(
        expected_states: &GuessResult,
        actual_char_guesses: Option<GuessResult>,
    ) {
        match actual_char_guesses {
            None => panic!("guess states not found"),
            Some(actual_guess) => assert_eq!(expected_states, &actual_guess),
        }
    }

    fn assert_keyboard_view(
        not_found: &str,
        incorrect_position: &str,
        correct_position: &str,
        keyboard_view: &KeyboardView,
    ) {
        for ch in not_found.chars() {
            assert_eq!(CharState::NotFound, keyboard_view.get(ch).unwrap(), "did not match for char: {ch}");
        }
        for ch in incorrect_position.chars() {
            assert_eq!(CharState::IncorrectPosition, keyboard_view.get(ch).unwrap(), "did not match for char: {ch}");
        }
        for ch in correct_position.chars() {
            assert_eq!(CharState::CorrectPosition, keyboard_view.get(ch).unwrap(), "did not match for char: {ch}");
        }
    }

    fn basic_dict() -> Dictionary {
        let mut dict = Dictionary::new(3);
        dict.add_word_str("rat");
        dict.add_word_str("sat");
        dict.add_word_str("mat");
        dict.add_word_str("cat");
        dict
    }

    fn big_dict() -> Dictionary {
        let mut dict = Dictionary::new(5);
        dict.add_word_str("clone");
        dict.add_word_str("colon");
        dict.add_word_str("spoon");
        dict.add_word_str("ovolo"); // a rounded convex molding, in cross section a quarter of a
                                    // circle or ellipse.
        dict.add_word_str("potoo"); // a type of bird
        dict.add_word_str("other");
        dict.add_word_str("siena");
        dict
    }
}
