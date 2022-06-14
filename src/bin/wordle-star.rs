use cursive::theme::*;
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, EditView, TextView};
use cursive::{Cursive, CursiveExt, Printer};
use cursive_core::{
    views::{Button, FixedLayout, LinearLayout},
    Rect,
};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use wordle_star::dictionary::Dictionary;
use wordle_star::game::Game;

fn main() {
    let mut siv = cursive::default();
    let filename = "data/en-dict-5letters.txt";
    let mut dict = load_dict(filename);
    let mut game = Game::new(dict, "colon");

    let guess_result = game.guess_word("clone");
    let guess_result = game.guess_word("corns");
    println!("guess result: {:?}", guess_result);

    // You can load a theme from a file at runtime for fast development.
    siv.load_theme_file("assets/style.toml").unwrap();

    let panel1 = LinearLayout::horizontal().child(game.with_name("board"));

    let buttons2 = LinearLayout::horizontal()
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    let view =
        Dialog::around(LinearLayout::vertical().child(panel1).child(buttons2)).title("SUDOKU");

    siv.add_layer(view);

    siv.run();
}

fn show_popup(s: &mut Cursive, name: &str) {}

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

fn restart(s: &mut Cursive) {}

fn hint(s: &mut Cursive) {}

fn undo(s: &mut Cursive) {}

fn redo(s: &mut Cursive) {}
