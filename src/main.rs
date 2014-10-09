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
    _say(question, col);
    std::io::stdio::stdin().read_line().unwrap()
}

fn ask(question: &str) -> String {
    _ask(question, Blank)
}

fn ask_red(question: &str) -> String {
    _ask(question, Red)
}

fn _say (s: &str, col: Color) {
    let mut t = term::stdout().unwrap();
    match col {
        Green => { t.fg(term::color::GREEN).unwrap(); }
        Red => { t.fg(term::color::RED).unwrap(); }
        Blank => {}
    }
    (writeln!(t, "{}", s)).unwrap();
    t.reset().unwrap();
}

fn say (s: &str) {
    _say(s, Blank)
}

fn say_red (s: &str) {
    _say(s, Red)
}

fn say_green (s: &str) {
    _say(s, Green)
}

fn main() {
    println!("Hello, world!");
    let path = Path::new("/home/tha/Dev/RUST/PRJ/rust-file-mover");
    say_red("reading directory:");
    visit_dirs(&path, all_files);
    say_red("bye");
}

enum Action {
    Inside,
    Skip,
    Trash,
    Movie,
    Music,
    Var,
    Quit,
    Nothing
}

fn get_action(ans: &str) -> Action {
    if ans == "i" { Inside }
    else if ans == "s" { Skip }
    else if ans == "t" { Trash }
    else if ans == "m" { Movie }
    else if ans == "u" { Music }
    else if ans == "v" { Var }
    else if ans == "q" { Quit }
    else { Nothing }
}

fn all_files(path: &Path) {
    println!("{}", path.display());
    let a = ask("(Skip | Trash | Inside-dir | Movie | mUsic | Var | Quit )");
    let ans = a.as_slice().trim();
    let res = get_action(ans);
    match res {
        Inside if path.is_dir() => { visit_dirs(path, all_files); }
        Skip => { /*next */ say("skipping"); }
        Trash => { say("moving to trash.."); }
        Movie => { say("moving to movies.."); }
        Music => { say("moving to music.."); }
        Var => { say("moving to var.."); }
        Quit => { say("bye bye"); }
        _ => { say_red("option not valid"); }
    }
}

fn visit_dirs(dir: &Path, cb: |&Path|) -> io::IoResult<()> {
    if dir.is_dir() {
        let contents = try!(fs::readdir(dir));
        for entry in contents.iter() {
            cb(entry);
        }
        Ok(())
    } else {
        Err(io::standard_error(io::InvalidInput))
    }
}

/*
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
*/
