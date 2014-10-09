#![feature(macro_rules)]
#![feature(log_syntax)]

extern crate term;
use std::io::fs::PathExtensions;
use std::io::fs;
use std::io;

enum Color {
    Green,
    Red,
    Blank
}

macro_rules! say(
    ($s:expr, $col:expr) => ({
        let mut t = term::stdout().unwrap();
        if $col == "green" {
            t.fg(term::color::GREEN).unwrap();
        } else if $col == "red" {
            t.fg(term::color::RED).unwrap();
        }
        (writeln!(t, "{}", $s)).unwrap();
        t.reset().unwrap();
    });
)

macro_rules! green(
    ($s:expr) => ({
        say!($s, "green")
    });
)

macro_rules! red(
    ($s:expr) => ({
        say!($s, "red")
    });
)

macro_rules! ask(
    ($question:expr) => ({
        println!($question);
        std::io::stdio::stdin().read_line().unwrap()
    });
)

fn _ask(question: &str, col: Color) -> String {
    say(question, col);
    std::io::stdio::stdin().read_line().unwrap()
}

fn ask(question: &str) -> String {
    _ask(question, Blank)
}

fn ask_red(question: &str) -> String {
    _ask(question, Red)
}

fn say (s: &str, col: Color) {
    let mut t = term::stdout().unwrap();
    match col {
        Green => { t.fg(term::color::GREEN).unwrap(); }
        Red => { t.fg(term::color::RED).unwrap(); }
        Blank => {}
    }
    (writeln!(t, "{}", s)).unwrap();
    t.reset().unwrap();
}

fn say_red (s: &str) {
    say(s, Red)
}

fn say_green (s: &str) {
    say(s, Green)
}

fn main() {
    println!("Hello, world!");
    let path = Path::new("/home/tha/Dev/RUST/PRJ/rust-file-mover");
    let mut arr = Vec::new();
    visit_dirs(&mut arr, &path);
    say_red("reading directory:");
    all_files(&mut arr);
    say_red("bye");
}


fn all_files(arr: &mut Vec<String>) {
    //arr.sort_by(|a, b| a.cmp(b));
    for a in arr.iter() {
        println!("{}", a);
        let a = ask_red("what should I do?");
    }
}

// one possible implementation of fs::walk_dir only visiting files
fn visit_dirs<'a>(arr: &mut Vec<String>, dir: &Path) -> io::IoResult<()>{
    if dir.is_dir() {
        let contents = try!(fs::readdir(dir));
        for entry in contents.iter() {
            if entry.is_dir() {
                visit_dirs(arr, entry);
            } else {
                let s = entry.as_str();
                match s {
                    Some(x) => {
                        arr.push(String::from_str(x))
                    }
                    None => {}
                }
            }
        }
        Ok(())
    } else {
        Err(io::standard_error(io::InvalidInput))
    }
}
