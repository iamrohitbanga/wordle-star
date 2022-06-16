use cursive::theme::*;
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, EditView, TextView};
use cursive::{
    theme::{BaseColor, Color, ColorStyle, ColorType, Effect},
    view::View,
    views::LinearLayout,
    Cursive, Printer, Vec2,
};

use cursive_core::view;

use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;
use wordle_star::dictionary::Dictionary;
use wordle_star::game::Game;
use wordle_star::game::GameState;
use wordle_star::guess::CharState;

const MAX_WORD_LENGTH: usize = 5;
const MAX_ATTEMPTS: usize = 6;

fn main() {
    let filename = "data/en-dict-5letters.txt";
    let dict = load_dict(filename);

    // pick a random word as the target from the dictionary
    let target_word = dict.random_word();
    let game = Game::new(dict, &target_word, MAX_ATTEMPTS);

    let mut siv = cursive::default();
    siv.load_theme_file("assets/style.toml").unwrap();

    // wrap the game into a shared Rc, so that we can pass it to the closure
    // for cursive event handlers. Dynamic Borrowing!
    let shared_game: Rc<RefCell<_>> = Rc::new(RefCell::new(game));

    // Custom View to draw the wordle board
    let gameboard = BoardView::new(shared_game.clone());

    let main_panel = LinearLayout::vertical().child(gameboard).child(
        EditView::new()
            .max_content_width(MAX_WORD_LENGTH) // no more than N chars
            .on_submit(move |csiv, guess| {
                // search by name of EditView
                csiv.call_on(&view::Selector::Name("guess"), |view: &mut EditView| {
                    // clear the edit box first, allow user to enter next guess
                    view.set_content("");
                });

                process_guess(csiv, guess, shared_game.clone());
            })
            .with_name("guess")
            .fixed_width(MAX_WORD_LENGTH + 1), // N characters allowed, set width to N+1 so that
                                               // all N characters are readable.
    );

    let view = LinearLayout::vertical().child(main_panel);
    siv.add_layer(view);

    siv.run();
}

/**
 * Given a guess, submit it to the game and process the result.
 */
fn process_guess(s: &mut Cursive, guess: &str, shared_game: Rc<RefCell<Game>>) {
    let mut game = shared_game.borrow_mut();
    let guess_result = game.guess_word(&guess);
    match guess_result {
        None => invalid_word_popup(s, guess),
        Some(_) => {
            match game.state {
                GameState::Playing => (), // Answer not found, attempts remaining
                GameState::Win => win(s), // Answer found!
                GameState::Lose => lose(s, &game.target_word), // attempts exhausted
            }
        }
    }
}

/**
 * Display popup saying that the word is invalid.
 */
fn invalid_word_popup(s: &mut Cursive, guess: &str) {
    let mut message = StyledString::plain("\n\n");
    message.append(StyledString::styled(
        guess,
        Style::from(Color::Light(BaseColor::Red)).combine(Effect::Bold),
    ));
    message.append(StyledString::plain(" is not a valid word.\n\n"));
    s.add_layer(
        Dialog::around(TextView::new(message))
            .title("Oops!")
            .button("Back", |s| {
                s.pop_layer(); // remove this dialog when the button is pressed
            }),
    );
}

/**
 * Show message after the user wins.
 */
fn win(s: &mut Cursive) {
    let mut message = StyledString::plain("\n\n");
    message.append(StyledString::styled(
        "\tYou Win! 🥳",
        Style::from(Color::Light(BaseColor::Green)).combine(Effect::Bold),
    ));
    s.add_layer(
        Dialog::around(TextView::new(message))
            .title("Congratulations!")
            .button("Ok", |s| s.quit()),
    );
}

/**
 * Show message after the user loses.
 */
fn lose(s: &mut Cursive, correct_word: &str) {
    let mut message = StyledString::plain("\n\n");
    message.append(StyledString::styled(
        "\tYou Lost! 😣\n Better Luck next time.",
        Style::from(Color::Light(BaseColor::Cyan)).combine(Effect::Bold),
    ));
    message.append(StyledString::styled(
        format!("\n\tAnswer: {correct_word}"),
        Style::from(Color::Light(BaseColor::Blue)).combine(Effect::Bold),
    ));

    s.add_layer(
        Dialog::around(TextView::new(message))
            .title("Oh no!")
            .button("Ok", |s| s.quit()),
    );
}

/**
 * Load dictionary from a file.
 */
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

/**
 * A struct that wraps the wordle game into a board.
 * Used to render a view of the Wordle board.
 */
struct BoardView {
    game: Rc<RefCell<Game>>,
}

impl BoardView {
    pub fn new(game: Rc<RefCell<Game>>) -> BoardView {
        BoardView { game: game }
    }

    /**
     * Draw the wordle board.
     */
    pub fn draw_board(&self, printer: &Printer) {
        self.draw_guesses(printer);
        self.draw_keyboard_view(printer);
    }

    /**
     * Render all guesses so far.
     */
    fn draw_guesses(&self, printer: &Printer) {
        let game = self.game.borrow();

        for (guess_index, guess_result) in game.guess_results.iter().enumerate() {
            // for each guess
            for (ch_index, char_guess) in guess_result.char_guesses.iter().enumerate() {
                // for each character of guess

                // background color of each character based on whether it is correct or not
                let bg_color = match char_guess.1 {
                    CharState::NotFound => BaseColor::Red,
                    CharState::IncorrectPosition => BaseColor::Yellow,
                    CharState::CorrectPosition => BaseColor::Green,
                };

                let style = ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                    ColorType::Color(Color::Dark(bg_color)),
                );
                printer.with_color(style, |p| {
                    p.print(
                        (ch_index + 3, guess_index * 2 + 5),
                        &char_guess.0.to_string(),
                    );
                });
            }
        }
    }

    /**
     * Render the English alpha keyboard (qwerty), with hints
     * based on guesses so far.
     */
    fn draw_keyboard_view(&self, printer: &Printer) {
        let game = self.game.borrow();
        let lines = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];
        for (line_num, line) in lines.into_iter().enumerate() {
            for (pos, ch) in line.chars().enumerate() {
                // background color based on the keyboard view
                let bg_color = match game.keyboard_view.get(ch) {
                    None => BaseColor::Black,
                    Some(char_state) => match char_state {
                        CharState::NotFound => BaseColor::Red,
                        CharState::IncorrectPosition => BaseColor::Yellow,
                        CharState::CorrectPosition => BaseColor::Green,
                    },
                };

                let fg_color = match game.keyboard_view.get(ch) {
                    None => BaseColor::White,
                    Some(_) => BaseColor::Black,
                };

                let style = ColorStyle::new(
                    ColorType::Color(Color::Dark(fg_color)),
                    ColorType::Color(Color::Dark(bg_color)),
                );

                printer.with_color(style, |p| {
                    p.print((pos * 2 + 30 + line_num, line_num * 2 + 5), &ch.to_string());
                });
            }
        }
    }
}

/**
 * Cursive View to render the wordle board.
 */
impl View for BoardView {
    fn draw(&self, printer: &Printer) {
        self.draw_board(printer);
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(150, 20)
    }
}
