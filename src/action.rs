extern crate libc;
use std::io::fs;
use std::io::fs::PathExtensions;
use cli;
use config;

pub enum Action {
    Inside,
    Up,
    Skip,
    Trash,
    Movie,
    Music,
    Var,
    Quit,
    Custom,
    Nothing
}

pub enum ActionResult {
    Repeat,
    Next,
    Out,
    In
}

fn movePath(dest: &String, path: &Path) {
    let newpath = if path.is_dir() {
        dest.clone().append("/").append(path.filestem_str().unwrap())
    } else {
        dest.clone().append("/").append(path.filename_str().unwrap())
    };
    println!("Moving to {}", newpath);
    match fs::rename(path, &Path::new(newpath.as_slice())) {
        Ok(e) => cli::say_green("Success!"),
        Err(e) => fail!("Fatal: {}", e)
    }
}

impl Action {
    pub fn handle(&self, config: &Box<config::TomlPaths>, path: &Path) -> ActionResult {
        match *self {
            Inside if path.is_dir() => {
                cli::say("Entering dir..");
                return In;
            }
            Skip => {
                cli::say("skipping");
            }
            Trash => {
                cli::say("moving to trash..");
                movePath(&config.trash, path);
            }
            Movie => {
                cli::say("moving to movies..");
                movePath(&config.movie, path);
            }
            Music => {
                cli::say("moving to music..");
                movePath(&config.music, path);
            }
            Var => {
                cli::say("moving to var..");
                movePath(&config.var, path);
            }
            Quit => {
                cli::say("bye bye");
                unsafe { libc::exit(0 as libc::c_int); }
            }
            Custom => {
                let dir = cli::ask("where should I move this?");
                movePath(&dir, path);
            }
            Up => {
                cli::say("Leaving dir..");
                return Out;
            }
            _ => {
                cli::say_red("Option not valid");
                return Repeat;
            }
        }
        Next
    }
}

pub fn get_action(ans: &str) -> Action {
    if ans == "i" { Inside }
    else if ans == "o" { Up }
    else if ans == "s" || ans == "n" { Skip }
    else if ans == "t" { Trash }
    else if ans == "m" { Movie }
    else if ans == "u" { Music }
    else if ans == "v" { Var }
    else if ans == "q" { Quit }
    else if ans == "c" { Custom }
    else { Nothing }
}
