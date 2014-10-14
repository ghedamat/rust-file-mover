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
Usage: rust-file-mover [-c CFG] [PATH]
       rust-file-mover (--help)

If PATH it's not supplied it will default to cwd

Options:
    -c CFG   Specify config file
    -h, --help         Show this message.
")

fn main() {
    let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());

    let dirname = if args.arg_PATH.as_slice() == "" {
        String::from_str(os::getcwd().as_str().unwrap())
    } else {
        args.arg_PATH
    };
    let cfg_path = if args.flag_c.as_slice() == "" {
        String::from_str("~/.rust-file-mover.toml")
    } else {
        args.flag_c
    };
    let config = config::read_config(cfg_path);

    let path = Path::new(dirname);
    cli::say_green("Selected directory:");
    cli::say_green(path.as_str().unwrap());
    config.print_config();
    match visit_dirs(&config, &path, process_dir) {
        Ok(()) => {}
        Err(m) => { println!("Error: {}", m) }
    }
    cli::say_red("Bye!");
}

fn process_dir(config: &Box<config::TomlPaths>, path: &Path) -> Option<()> {
    loop {
        let a = if path.is_dir() {
            cli::say_red("Processing dir");
            cli::say_yellow(format!("{}/", path.as_str().unwrap()).as_slice());
            cli::ask("[Skip | Trash | Inside-dir | Outside-dir | Movie | mUsic | Var | Quit ]")
        } else {
            cli::say_green("Processing file");
            cli::say_yellow(path.as_str().unwrap());
            cli::ask("[Skip | Trash | Outside-dir | Movie | mUsic | Var | Quit ]")
        };
        let ans = a.as_slice().trim();
        let act = action::get_action(ans);
        match act.handle(config, path) {
            action::Next => { break }
            action::Out => { return None; }
            action::In => {
                match visit_dirs(config, path, process_dir) {
                    Ok(()) => {}
                    Err(m) => { println!("Error: {}", m) }
                }
            }
            action::Repeat => {}
        };
    }
    Some(())
}

fn visit_dirs(config: &Box<config::TomlPaths>, dir: &Path, cb: |&Box<config::TomlPaths>, &Path| -> Option<()>) -> io::IoResult<()> {
    if dir.is_dir() {
        let contents = try!(fs::readdir(dir));
        for entry in contents.iter() {
            match cb(config, entry) {
                Some(_) => {}
                None => break
            };
        }
        Ok(())
    } else {
        Err(io::standard_error(io::FileNotFound))
    }
}
