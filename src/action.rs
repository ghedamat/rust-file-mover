extern crate libc;
use std::io::fs::PathExtensions;
use cli;

pub enum Action {
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

pub enum ActionResult {
    Repeat,
    Next,
    Out,
    In
}

impl Action {
    pub fn handle(&self, path: &Path) -> ActionResult {
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
                //trash(path)
            }
            Movie => {
                cli::say("moving to movies..");
                //movies(path)
            }
            Music => {
                cli::say("moving to music..");
                //music(path)
            }
            Var => {
                cli::say("moving to var..");
                //var(path)
            }
            Quit => {
                cli::say("bye bye");
                unsafe { libc::exit(0 as libc::c_int); }
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
    else if ans == "s" { Skip }
    else if ans == "t" { Trash }
    else if ans == "m" { Movie }
    else if ans == "u" { Music }
    else if ans == "v" { Var }
    else if ans == "q" { Quit }
    else { Nothing }
}
