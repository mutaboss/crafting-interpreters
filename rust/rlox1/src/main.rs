use clap::{App, Arg};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

// TODO: Add documentation.

// ------------------------------------------------------------------------------------------------
// Error Handling
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct LoxError {
    message: String,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LoxError encountered: {}.", self.message)
    }
}

// ------------------------------------------------------------------------------------------------
// Utility Functions
// ------------------------------------------------------------------------------------------------

fn display_prompt(prompt: &str) {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to write to stdout!");
}

// read_file: Read lines from a file. Line termination is stripped.
fn read_file(filename: &str) -> Result<Vec<String>, LoxError> {
    let f = match File::open(filename) {
        Ok(fh) => fh,
        Err(err) => {
            return Err(LoxError {
                message: format!("{}", err),
            });
        }
    };
    let reader = BufReader::new(f);
    let mut lines = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(err) => {
                return Err(LoxError {
                    message: format!("{}", err),
                });
            }
        }
    }
    return Ok(lines);
}

// ------------------------------------------------------------------------------------------------
// Execution
// ------------------------------------------------------------------------------------------------

struct Executor;

impl Executor {
    // run_line: Run a single line of Lox code.
    // This is where the magic happens.
    fn run_line(&self, buffer: &str) -> Result<(), LoxError> {
        if buffer.starts_with("print") || buffer.starts_with("var") {
            println!("{}", buffer);
            return Ok(());
        } else {
            return Err(LoxError {
                message: format!("Bad input: {}", buffer),
            });
        }
    }

    // run_file: Run the supplied file based on filename.
    // We iterate through each line of the file and attempt to execute it.
    // TODO: collect errors from execution, so we can see if multiple errors are encountered.
    pub fn run_file(&self, filename: &str) -> Result<(), LoxError> {
        match read_file(filename) {
            Ok(lines) => {
                for line in lines {
                    match self.run_line(&line) {
                        Ok(_) => (),
                        Err(err) => return Err(err),
                    }
                }
            }
            Err(err) => return Err(err),
        }
        return Ok(());
    }

    // run_repl: Read a line, execute it, repeat.
    pub fn run_repl(&self) -> Result<(), LoxError> {
        let mut line = String::new();
        loop {
            display_prompt("> ");
            line.clear();
            match io::stdin().read_line(&mut line) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    } else {
                        if !line.trim().is_empty() {
                            match self.run_line(&line.trim()) {
                                Ok(()) => (),
                                Err(err) => eprintln!("{}", err),
                            }
                        }
                    }
                }
                Err(err) => {
                    return Err(LoxError {
                        message: format!("{}", err),
                    });
                }
            }
        }
        return Ok(());
    }
}

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
