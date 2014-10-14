extern crate std;
extern crate term;

enum Color {
    Green,
    Red,
    Yellow,
    Blue,
    Blank
}

fn _ask(question: &str, col: Color) -> String {
    _say(question, col);
    std::io::stdio::stdin().read_line().unwrap()
}

pub fn ask(question: &str) -> String {
    _ask(question, Blank)
}

pub fn ask_red(question: &str) -> String {
    _ask(question, Red)
}

fn _say (s: &str, col: Color) {
    let mut t = term::stdout().unwrap();
    match col {
        Green => { t.fg(term::color::GREEN).unwrap(); }
        Red => { t.fg(term::color::RED).unwrap(); }
        Yellow => { t.fg(term::color::BRIGHT_YELLOW).unwrap(); }
        Blue => { t.fg(term::color::BRIGHT_BLUE).unwrap(); }
        Blank => {}
    }
    (writeln!(t, "{}", s)).unwrap();
    t.reset().unwrap();
}

pub fn say (s: &str) {
    _say(s, Blank)
}

pub fn say_red (s: &str) {
    _say(s, Red)
}

pub fn say_green (s: &str) {
    _say(s, Green)
}

pub fn say_yellow (s: &str) {
    _say(s, Yellow)
}

pub fn say_blue (s: &str) {
    _say(s, Blue)
}

