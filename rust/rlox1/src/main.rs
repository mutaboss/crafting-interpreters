use clap::{App, Arg};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone)]
struct LoxError;

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error executing this code.")
    }
}

// read_file: Read lines from a file. Line termination is stripped.
// If we encounter an error, we print a message and exit.
// It seems pointless to return an error from here if we simply cannot read the file.
fn read_file(filename: &str) -> Vec<String> {
    let f = match File::open(filename) {
        Ok(fh) => fh,
        Err(err) => {
            println!("Error encountered opening {}: {}.", filename, err);
            std::process::exit(2);
        }
    };
    let reader = BufReader::new(f);
    return reader
        .lines()
        .map(|l| match l {
            Ok(line) => line,
            Err(err) => {
                println!("Error reading from {}: {}.", filename, err);
                std::process::exit(3);
            }
        })
        .collect();
}

// run_file: Run the supplied file based on filename.
// We iterate through each line of the file and attempt to execute it.
// TODO: collect errors from execution, so we can see if multiple errors are encountered.
fn run_file(filename: &str) -> Option<LoxError> {
    for line in read_file(filename) {
        match run_line(line) {
            None => (),
            Some(err) => return Some(err),
        }
    }
    return None;
}

// run_repl: Read a line, execute it, repeat.
// TODO: Implement reading from stdin.
fn run_repl() -> Option<LoxError> {
    return Some(LoxError {});
}

// run_line: Run a single line of Lox code.
// This is where the magic happens.
fn run_line(buffer: String) -> Option<LoxError> {
    println!("{}", buffer);
    return None;
}

fn main() {
    let matches = App::new("rlox1: Lox in Rust.")
        .version("0.1.0")
        .author("Brian King <brian@jenashcal.net>")
        .about("Implementation of Lox from Part II of Crafting Interpreters by Robert Nystrum.")
        .arg(Arg::with_name("script").index(1))
        .get_matches();
    let result = match matches.value_of("script") {
        None => run_repl(),
        Some(script) => run_file(script),
    };
    match result {
        None => (),
        Some(err) => println!("ERROR: {}", err),
    };
}
