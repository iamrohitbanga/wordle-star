use std::collections::HashSet;

/**
 * A dictionary of fixed length words. All words in the dictionary
 * have the same length drawn from the same alphabet.
 */
#[derive(Debug)]
pub struct Dictionary {
    pub wordset: HashSet<String>,
    pub word_length: usize,
}

impl Dictionary {
    pub fn new(word_length: usize) -> Dictionary {
        if word_length == 0 {
            panic!("word length must be positive");
        }
        Dictionary {
            wordset: HashSet::new(),
            word_length: word_length,
        }
    }

    pub fn add_word(&mut self, word: &String) {
        let actual_length = word.chars().count();
        if self.word_length != actual_length {
            panic!("Incorrect word length. Actual: {0}, Expected: {1}", actual_length, self.word_length);
        }
        self.wordset.insert(word.to_string());
    }

    pub fn add_word_str(&mut self, word: &str) {
        self.add_word(&word.to_string());
    }

    pub fn contains(&self, word: &String) -> bool {
        self.wordset.contains(word)
    }

    pub fn len(&self) -> usize {
        self.wordset.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected="word length must be positive")]
    fn test_word_length_zero() {
        Dictionary::new(0);
    }

    #[test]
    fn test_add_words() {
        let mut dict = Dictionary::new(2);
        dict.add_word_str("ab");
        dict.add_word_str("bc");

        assert_eq!(true, dict.contains(&"ab".to_string()));
        assert_eq!(true, dict.contains(&"bc".to_string()));
        assert_eq!(false, dict.contains(&"ca".to_string()));
        assert_eq!(2, dict.len());
    }

    #[test]
    fn test_repeat_words() {
        let mut dict = Dictionary::new(2);
        dict.add_word_str("ab");
        dict.add_word_str("bc");
        dict.add_word_str("ab");
        dict.add_word_str("bc");

        assert_eq!(true, dict.contains(&"ab".to_string()));
        assert_eq!(true, dict.contains(&"bc".to_string()));
        assert_eq!(false, dict.contains(&"ca".to_string()));
        assert_eq!(2, dict.len());
     }

    #[test]
    #[should_panic(expected="Incorrect word length. Actual: 3, Expected: 4")]
    fn test_word_length_mismatch() {
        let mut dict = Dictionary::new(4);
        dict.add_word_str("abc");
    }
}
