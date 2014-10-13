#![feature(phase, macro_rules)]
#![feature(log_syntax)]

extern crate term;
extern crate serialize;
extern crate docopt;
#[phase(plugin)] extern crate docopt_macros;

use std::io::fs::PathExtensions;
use std::io::fs;
use std::io;
use std::os;
use docopt::FlagParser;

mod cli;
mod action;
mod config;
docopt!(Args, "
Usage: rust-file-mover [PATH]
       rust-file-mover (--help | --version)

If PATH it's not supplied it will default to cwd

Options:
    -h, --help         Show this message.
    --version          Show the version of rustc.
")

fn main() {
    let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());
    println!("{}", args.arg_PATH);

    let dirname = if args.arg_PATH.as_slice() == "" {
        String::from_str(os::getcwd().as_str().unwrap())
    } else {
        args.arg_PATH
    };


    let paths = config::read_config();
    println!("{}", paths.movie);

    let path = Path::new(dirname);
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
