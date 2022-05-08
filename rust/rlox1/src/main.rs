use clap::{App, Arg};

// TODO: Add documentation.

#[macro_use]
mod error;
mod executive;
mod scanner;
mod parser;

use executive::Executor;

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
    let exec = Executor::new();
    let result = match matches.value_of("script") {
        None => exec.run_repl(),
        Some(script) => exec.run_file(script),
    };
    if let Err(err) = result {
        eprintln!("ERROR: {}", err);
    };
}
