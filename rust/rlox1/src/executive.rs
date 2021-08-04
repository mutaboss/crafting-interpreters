use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use crate::error::{self, LoxError};

pub struct Executor;

impl Executor {
    // display_prompt: Display a prompt and flush to stdout.
    fn display_prompt(&self, prompt: &str) {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to write to stdout!");
    }

    // read_file: Read lines from a file. Line termination is stripped.
    fn read_file(&self, filename: &str) -> Result<Vec<String>, LoxError> {
        // TODO: Check file size before opening.
        let f = File::open(filename)?;
        let reader = BufReader::new(f);
        let mut lines = Vec::new();
        for line in reader.lines() {
            lines.push(line?);
        }
        Ok(lines)
    }

    // run_line: Run a single line of Lox code. This is where the magic happens.
    fn run_line(&self, buffer: &str) -> Result<(), LoxError> {
        if buffer.starts_with("print") || buffer.starts_with("var") {
            println!("{}", buffer);
            Ok(())
        } else {
            Err(error::new(&format!("Bad input: {}", buffer)))
        }
    }

    // run_file: Run the supplied file based on filename.
    // We iterate through each line of the file and attempt to execute it.
    // TODO: collect errors from execution, so we can see if multiple errors are encountered.
    pub fn run_file(&self, filename: &str) -> Result<(), LoxError> {
        for line in self.read_file(filename)? {
            self.run_line(&line)?;
        }
        Ok(())
    }

    // run_repl: Read a line, execute it, repeat.
    pub fn run_repl(&self) -> Result<(), LoxError> {
        let mut line = String::new();
        loop {
            line.clear();
            self.display_prompt("> ");
            if io::stdin().read_line(&mut line).expect("Error on stdin!") == 0 {
                break; // EOF reached.
            } else {
                let line = line.trim();
                // Skip empty lines. Display and continue on error.
                if !line.is_empty() {
                    if let Err(err) = self.run_line(line) {
                        eprintln!("{}", err);
                    }
                }
            }
        }
        return Ok(());
    }
}
