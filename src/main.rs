#![feature(macro_rules)]
#![feature(log_syntax)]

extern crate term;
extern crate libc;
use std::io::fs::PathExtensions;
use std::io::fs;
use std::io;
use libc::funcs::c95::stdlib;

mod cli;

fn main() {
    println!("Hello, world!");
    let path = Path::new("/home/tha/Dev/RUST/PRJ/rust-file-mover");
    cli::say_red("reading directory:");
    visit_dirs(&path, all_files);
    cli::say_red("bye");
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
    loop {
        cli::say_yellow(path.as_str().unwrap());
        let a = if path.is_dir() {
            cli::ask("[Skip | Trash | Inside-dir | Outside-dir | Movie | mUsic | Var | Quit ]")
        } else {
            cli::ask("[Skip | Trash | Outside-dir | Movie | mUsic | Var | Quit ]")
        };
        let ans = a.as_slice().trim();
        let res = get_action(ans);
        match res {
            Inside if path.is_dir() => {
                cli::say_green("entering dir");
                visit_dirs(path, all_files);
            }
            Skip => {
                cli::say("skipping");
                break;
            }
            Trash => {
                cli::say("moving to trash..");
                break;
            }
            Movie => {
                cli::say("moving to movies..");
                break;
            }
            Music => {
                cli::say("moving to music..");
                break;
            }
            Var => {
                cli::say("moving to var..");
                break;
            }
            Quit => {
                cli::say("bye bye");
                unsafe { libc::exit(0 as libc::c_int); }
            }
            Up => {
                cli::say_green("leaving dir");
                return None;
            }
            _ => {
                cli::say_red("option not valid");
            }
        }
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
