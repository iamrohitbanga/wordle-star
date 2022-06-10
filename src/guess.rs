#[derive(Debug, PartialEq)]
pub enum CharState {
    NotFound,
    CorrectPosition,
    IncorrectPosition,
}

type CharGuess = (char, CharState);

#[derive(Debug, PartialEq)]
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
