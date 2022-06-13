#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum CharState {
    NotFound = 1,
    IncorrectPosition = 2,
    CorrectPosition = 3,
}

pub type CharGuess = (char, CharState);

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
