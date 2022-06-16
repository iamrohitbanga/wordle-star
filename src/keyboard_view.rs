use crate::guess::CharGuess;
use crate::guess::CharState;
use std::cmp;
use std::collections::HashMap;

/// View of the keyboard at any point in the game.
/// Initially the state of all characters is unknown.
/// As guesses are submitted, we will update the view of the keyboard
/// with the state of the characters.
/// Note that it is possible that one guess finds the correct position of the character,
/// while the second guess does not find the exact position of the character. In this case,
/// the keyboard view contains the aggregate state, and it would say that the character
/// has been found at the correct position.
pub struct KeyboardView {
    keymap: HashMap<char, CharState>,
}

impl KeyboardView {
    pub fn new() -> KeyboardView {
        KeyboardView {
            keymap: HashMap::new(),
        }
    }

    pub fn record_guess(&mut self, char_guess: &CharGuess) {
        let ch = char_guess.0;
        let ch_state = &char_guess.1;

        let existing = self.keymap.get(&ch);
        match existing {
            None => {
                self.keymap.insert(ch, *ch_state);
            }
            Some(current_state) => {
                if *current_state == CharState::NotFound && current_state != ch_state {
                    panic!("invalid state: character {}. previous state: {:?} incompatible with new state: {:?}", ch, current_state, ch_state);
                }

                let new_state = *(cmp::max(current_state, &ch_state));
                self.keymap.insert(ch, new_state);
            }
        }
    }

    pub fn get(&self, ch: char) -> Option<CharState> {
        self.keymap.get(&ch).map(|x| *x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_each_state() {
        let mut keyview = KeyboardView::new();
        keyview.record_guess(&('a', CharState::NotFound));
        keyview.record_guess(&('b', CharState::IncorrectPosition));
        keyview.record_guess(&('c', CharState::CorrectPosition));

        assert_eq!(Some(CharState::NotFound), keyview.get('a'));
        assert_eq!(Some(CharState::IncorrectPosition), keyview.get('b'));
        assert_eq!(Some(CharState::CorrectPosition), keyview.get('c'));
    }

    #[test]
    fn test_record_correct_position_found_later() {
        let mut keyview = KeyboardView::new();
        keyview.record_guess(&('a', CharState::IncorrectPosition));
        assert_eq!(Some(CharState::IncorrectPosition), keyview.get('a'));

        keyview.record_guess(&('a', CharState::CorrectPosition));
        assert_eq!(Some(CharState::CorrectPosition), keyview.get('a'));
    }

    #[test]
    fn test_record_correct_position_not_reverted() {
        let mut keyview = KeyboardView::new();
        keyview.record_guess(&('a', CharState::CorrectPosition));
        assert_eq!(Some(CharState::CorrectPosition), keyview.get('a'));

        keyview.record_guess(&('a', CharState::IncorrectPosition));
        assert_eq!(Some(CharState::CorrectPosition), keyview.get('a'));
    }

    #[test]
    #[should_panic(
        expected = "invalid state: character a. previous state: NotFound incompatible with new state: CorrectPosition"
    )]
    fn test_invalid_state_change() {
        let mut keyview = KeyboardView::new();
        keyview.record_guess(&('a', CharState::NotFound));
        keyview.record_guess(&('a', CharState::CorrectPosition));
    }
}
