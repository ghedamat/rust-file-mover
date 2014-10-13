#![feature(macro_rules)]
#![feature(log_syntax)]

extern crate term;
extern crate serialize;
use std::io::fs::PathExtensions;
use std::io::fs;
use std::io;

mod cli;
mod action;
mod config;

fn main() {

    //let dec= toml::decode(value);

    //println!("{:s}", cfg.to_str());
    let paths = config::read_config();
    println!("{}", paths.unwrap().movie);

    let path = Path::new("/home/tha/Dev/RUST/PRJ/rust-file-mover");
    cli::say_green("Working directory:");
    cli::say_green(path.as_str().unwrap());
    match visit_dirs(&path, process_dir) {
        Ok(()) => {}
        Err(m) => { println!("Error: {}", m) }
    }
    cli::say_red("Bye!");
}

fn process_dir(path: &Path) -> Option<()> {
    loop {
        let a = if path.is_dir() {
            cli::say_red("Processing dir");
            cli::say_yellow(path.as_str().unwrap());
            cli::ask("[Skip | Trash | Inside-dir | Outside-dir | Movie | mUsic | Var | Quit ]")
        } else {
            cli::say_green("Processing file");
            cli::say_yellow(path.as_str().unwrap());
            cli::ask("[Skip | Trash | Outside-dir | Movie | mUsic | Var | Quit ]")
        };
        let ans = a.as_slice().trim();
        let act = action::get_action(ans);
        match act.handle(path) {
            action::Next => { break }
            action::Out => { return None; }
            action::In => { visit_dirs(path, process_dir); }
            action::Repeat => {}
        };
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
