use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use wordle_star::dictionary::Dictionary;
use wordle_star::game::Game;

fn main() {
    let filename = "data/en-dict-5letters.txt";
    load_dict(filename);

    let mut dict = load_dict(filename);

    let mut game = Game::new(&dict, "colon");
    let guess_result = game.guess_word("clone");
    println!("guess result: {:?}", guess_result);
}

fn load_dict(filename: &str) -> Dictionary {
    println!("loading dict from path: {}", filename);

    let mut dict = Dictionary::new(5);

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word) = line {
                dict.add_word_str(word.trim());
            }
        }
    }

    dict
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
