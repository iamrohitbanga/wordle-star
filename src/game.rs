use crate::dictionary::Dictionary;
use crate::guess::CharState;
use crate::guess::GuessResult;
use crate::keyboard_view::KeyboardView;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

/// Enum representing current state of the game.
#[derive(Debug, PartialEq)]
pub enum GameState {
    Playing,
    Win,
    Lose,
}

/// This struct encapsulates all properties of a wordle game.
pub struct Game {
    /// Dictionary of fixed length words to use
    dict: Dictionary,

    /// word that has to be guessed, must be in the dictionary
    pub target_word: String,

    /// helper map to store index of each character in the target word
    target_char_indexes: HashMap<char, HashSet<usize>>,

    /// max guesses allowed
    max_guesses: usize,

    /// state of the keyboard, given the guesses made so far in the game
    pub keyboard_view: KeyboardView,

    /// results of each guess in order of submission
    /// stores details of each character in the guessed word.
    pub guess_results: Vec<GuessResult>,

    /// state of the game, in progress or finished?
    pub state: GameState,
}

impl Game {
    /// Create a new Game with a given dictionary, and a target word.
    /// panics if the target word is not in the dictionary.
    /// Only max_guesses attempts may be made.
    pub fn new(dict: Dictionary, target_word: &str, max_guesses: usize) -> Game {
        if !Game::is_word_allowed_in_dict(&dict, &target_word) {
            panic!("target word not present in dictionary");
        }
        let positions_map = Game::compute_char_positions_map(&target_word);
        return Game {
            dict: dict,
            target_word: target_word.to_string(),
            target_char_indexes: positions_map,
            keyboard_view: KeyboardView::new(),
            guess_results: vec![],
            state: GameState::Playing,
            max_guesses: max_guesses,
        };
    }

    /// Submit a guess to the game.
    /// panics if max guesses have already been submitted.
    /// Returns an `Option` of `GuessResult`.
    ///  -> None if the word is not allowed per the dictionary.
    ///  -> Some(GuessResult) contains the result of submitting a guess.
    /// The internal state of the game is updated based on the submission.
    /// Game may be marked as won or lost accordingly. Other internal states
    /// tracking the guesses are also updated.
    pub fn guess_word(&mut self, word: &str) -> Option<GuessResult> {
        if !self.allow_more_guesses() {
            panic!("no more guesses allowed")
        }

        // TODO: normalize for casing
        if !self.is_word_allowed(&word) {
            return None;
        }

        let guess_result = self.compute_guess_result(&word);

        for char_guess in (&guess_result.char_guesses).into_iter() {
            self.keyboard_view.record_guess(&char_guess);
        }

        // append to internal guess results for later use
        self.guess_results.push(guess_result.clone());

        if guess_result.is_correct() {
            self.state = GameState::Win;
        } else if !self.allow_more_guesses() {
            self.state = GameState::Lose;
        }

        Some(guess_result)
    }

    /// Check if the provided word is allowed per the dictionary.
    fn is_word_allowed_in_dict(dict: &Dictionary, word: &str) -> bool {
        dict.contains(&word.to_string())
    }

    /// Check if a word is allowed per the dictionary of the game.
    fn is_word_allowed(&self, word: &str) -> bool {
        Game::is_word_allowed_in_dict(&self.dict, &word)
    }

    /// Should we allow submitting more guesses? Returns true or false.
    fn allow_more_guesses(&self) -> bool {
        self.guess_results.len() < self.max_guesses && self.state == GameState::Playing
    }

    /// Internal helper method that computes the guess result for the provided word.
    /// Assumes that the word is in the dictionary.
    fn compute_guess_result(&mut self, word: &str) -> GuessResult {
        let mut char_guesses = vec![];

        for ch in word.chars() {
            char_guesses.push((ch, CharState::NotFound));
        }

        let guess_map = Game::compute_char_positions_map(&word);

        for (&ch, target_positions) in &self.target_char_indexes {
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

    /// Compute a map of indexes of each character in the word provided.
    fn compute_char_positions_map(word: &str) -> HashMap<char, HashSet<usize>> {
        let mut map = HashMap::new();
        for (index, ch) in word.chars().enumerate() {
            if let None = map.get(&ch) {
                // first occurrence of a character
                map.insert(ch, HashSet::new());
            }

            // must be found, safe to unwrap
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

        let game = Game::new(dict, "dog", 3);
        assert_eq!(GameState::Playing, game.state);
        assert_eq!(0, game.guess_results.len());
        assert_keyboard_view("", "", "", &game.keyboard_view);
    }

    #[test]
    #[should_panic(expected = "no more guesses allowed")]
    fn test_game_win() {
        let mut dict = Dictionary::new(3);
        dict.add_word_str("rat");
        dict.add_word_str("dog");
        dict.add_word_str("cat");
        dict.add_word_str("tar");

        let mut game = Game::new(dict, "dog", 3);
        assert_eq!(GameState::Playing, game.state);
        game.guess_word("rat");
        assert_eq!(GameState::Playing, game.state);
        game.guess_word("cat");
        assert_eq!(GameState::Playing, game.state);
        game.guess_word("dog");
        assert_eq!(GameState::Win, game.state);
        assert_keyboard_view("rtac", "", "dog", &game.keyboard_view);

        // no more guesses after the game has been won
        game.guess_word("tar");
    }

    #[test]
    #[should_panic(expected = "no more guesses allowed")]
    fn test_game_lose() {
        let mut dict = Dictionary::new(3);
        dict.add_word_str("rat");
        dict.add_word_str("dog");
        dict.add_word_str("cat");

        // only two attempts to win the game
        let mut game = Game::new(dict, "dog", 2);

        assert_eq!(GameState::Playing, game.state);
        game.guess_word("rat");
        assert_eq!(GameState::Playing, game.state);
        game.guess_word("cat");
        assert_eq!(GameState::Lose, game.state);
        assert_keyboard_view("rtac", "", "", &game.keyboard_view);
        // already lost, no more guesses
        game.guess_word("dog");
    }

    #[test]
    #[should_panic(expected = "target word not present in dictionary")]
    fn test_target_word_length_not_same() {
        let mut dict = Dictionary::new(3);
        dict.add_word_str("rat");
        dict.add_word_str("dog");
        dict.add_word_str("cat");

        Game::new(dict, "star", 3);
    }

    #[test]
    #[should_panic(expected = "target word not present in dictionary")]
    fn test_target_word_not_in_dictionary() {
        let mut dict = Dictionary::new(3);
        dict.add_word_str("rat");
        dict.add_word_str("dog");
        dict.add_word_str("cat");

        Game::new(dict, "mat", 3);
    }

    #[test]
    fn test_guess_invalid_word() {
        let dict = basic_dict();
        let mut game = Game::new(dict, "mat", 6);
        assert_eq!(true, game.guess_word("abc").is_none());
    }

    #[test]
    fn test_guess_valid_word() {
        let dict = basic_dict();
        let mut game = Game::new(dict, "mat", 6);
        assert_eq!(false, game.guess_word("sat").is_none());
    }

    #[test]
    fn test_char_guesses_for_colon() {
        let dict = big_dict();

        let mut game = Game::new(dict, "colon", 6);
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

        let mut game = Game::new(dict, "clone", 6);

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
        assert_eq!(GameState::Playing, game.state);

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
        assert_eq!(GameState::Playing, game.state);

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
        assert_eq!(GameState::Playing, game.state);

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
        assert_eq!(GameState::Playing, game.state);
    }

    #[test]
    fn test_char_guesses_for_ovolo() {
        let dict = big_dict();

        let mut game = Game::new(dict, "ovolo", 6);

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
        assert_eq!(GameState::Playing, game.state);

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
        assert_eq!(GameState::Playing, game.state);

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
        assert_eq!(GameState::Playing, game.state);

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
        assert_eq!(GameState::Playing, game.state);

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
        assert_eq!(GameState::Win, game.state);
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
            assert_eq!(
                CharState::NotFound,
                keyboard_view.get(ch).unwrap(),
                "did not match for char: {ch}"
            );
        }
        for ch in incorrect_position.chars() {
            assert_eq!(
                CharState::IncorrectPosition,
                keyboard_view.get(ch).unwrap(),
                "did not match for char: {ch}"
            );
        }
        for ch in correct_position.chars() {
            assert_eq!(
                CharState::CorrectPosition,
                keyboard_view.get(ch).unwrap(),
                "did not match for char: {ch}"
            );
        }
        // TODO: Add stricter checks for characters that were not asserted
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
