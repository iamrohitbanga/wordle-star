/// A char in a guessed word may be in different states. It may not have been found,
/// or may be in its correct position, or incorrect position.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum CharState {
    NotFound = 1,
    IncorrectPosition = 2,
    CorrectPosition = 3,
}

/// A pair of a char with its state in a guess
pub type CharGuess = (char, CharState);

/// Encapsulate state of all characters in a guessed word.
#[derive(Clone, Debug, PartialEq)]
pub struct GuessResult {
    pub char_guesses: Vec<CharGuess>,
}

impl GuessResult {
    pub fn new(char_guesses: Vec<CharGuess>) -> GuessResult {
        GuessResult {
            char_guesses: char_guesses,
        }
    }

    pub fn is_correct(&self) -> bool {
        (&self.char_guesses)
            .into_iter()
            .all(|c| c.1 == CharState::CorrectPosition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_correct() {
        let gr = GuessResult::new(vec![
            ('c', CharState::CorrectPosition),
            ('l', CharState::CorrectPosition),
            ('i', CharState::CorrectPosition),
        ]);
        assert_eq!(true, gr.is_correct());

        let gr = GuessResult::new(vec![
            ('c', CharState::CorrectPosition),
            ('l', CharState::IncorrectPosition),
            ('i', CharState::CorrectPosition),
        ]);
        assert_eq!(false, gr.is_correct());

        let gr = GuessResult::new(vec![
            ('c', CharState::NotFound),
            ('l', CharState::CorrectPosition),
            ('i', CharState::CorrectPosition),
        ]);
        assert_eq!(false, gr.is_correct());
    }
}
