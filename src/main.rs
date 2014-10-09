#![feature(macro_rules)]
#![feature(log_syntax)]

extern crate term;
extern crate libc;
use std::io::fs::PathExtensions;
use std::io::fs;
use std::io;
use libc::funcs::c95::stdlib;

enum Color {
    Green,
    Red,
    Yellow,
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
        Yellow => { t.fg(term::color::BRIGHT_YELLOW).unwrap(); }
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

fn say_yellow (s: &str) {
    _say(s, Yellow)
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
    Up,
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
    else if ans == "o" { Up }
    else if ans == "s" { Skip }
    else if ans == "t" { Trash }
    else if ans == "m" { Movie }
    else if ans == "u" { Music }
    else if ans == "v" { Var }
    else if ans == "q" { Quit }
    else { Nothing }
}

fn all_files(path: &Path) -> Option<()> {
    say_yellow(path.as_str().unwrap());
    let a = ask("[Skip | Trash | Inside-dir | Outside-dir | Movie | mUsic | Var | Quit ]");
    let ans = a.as_slice().trim();
    let res = get_action(ans);
    match res {
        Inside if path.is_dir() => {
            say_green("entering dir");
            visit_dirs(path, all_files);
        }
        Skip => {
            say("skipping");
        }
        Trash => {
            say("moving to trash..");
        }
        Movie => {
            say("moving to movies..");
        }
        Music => {
            say("moving to music..");
        }
        Var => {
            say("moving to var..");
        }
        Quit => {
            say("bye bye");
            unsafe { libc::exit(0 as libc::c_int); }
        }
        Up => {
            say_green("leaving dir");
            return None;
        }
        _ => { say_red("option not valid"); }
    }
    Some(())
}

fn visit_dirs(dir: &Path, cb: |&Path| -> Option<()>) -> io::IoResult<()> {
    if dir.is_dir() {
        let contents = try!(fs::readdir(dir));
        for entry in contents.iter() {
            match cb(entry) {
                Some(_) => {}
                None => break
            };
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
