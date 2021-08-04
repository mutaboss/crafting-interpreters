use clap::{App, Arg};

use crate::executive::Executor;

// TODO: Add documentation.

mod error;
mod executive;

// ------------------------------------------------------------------------------------------------
// Main
// ------------------------------------------------------------------------------------------------

fn main() {
    let matches = App::new("rlox1: Lox in Rust.")
        .version("v0.1.0")
        .author("Brian King <brian@jenashcal.net>")
        .about("Implementation of Lox from Part II of Crafting Interpreters by Robert Nystrum.")
        .arg(Arg::with_name("script").index(1))
        .get_matches();
    let exec = Executor {};
    let result = match matches.value_of("script") {
        None => exec.run_repl(),
        Some(script) => exec.run_file(script),
    };
    match result {
        Ok(()) => (),
        Err(err) => eprintln!("ERROR: {}", err),
    };
}
