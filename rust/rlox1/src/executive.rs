use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader};

use crate::error::{self, LoxError};

const MAX_SOURCE_FILE_SIZE: u64 = 65535;

pub struct Executor;

pub fn new() -> Executor {
    Executor {}
}

impl Executor {
    // display_prompt: Display a prompt and flush to stdout.
    fn display_prompt(&self, prompt: &str) {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to write to stdout!");
    }

    // read_file: Read lines from a file. Line termination is stripped.
    fn read_file(&self, filename: &str) -> Result<Vec<String>, LoxError> {
        // Confirm the file isn't too big before opening.
        let attr = fs::metadata(filename)?;
        if !attr.is_file() {
            return Err(error::new(&format!("Path {} is not a file.", filename)));
        } else if attr.len() > MAX_SOURCE_FILE_SIZE {
            return Err(error::new(&format!(
                "File {} is too large ({} > {}).",
                filename,
                attr.len(),
                MAX_SOURCE_FILE_SIZE
            )));
        }
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

#[cfg(test)]
mod tests {
    use crate::error::{self, LoxError};
    use crate::executive;
    use std::path::PathBuf;

    macro_rules! assert_error_contains {
        ( $er:expr, $ct:expr ) => {
            match $er {
                Ok(()) => Err(error::new("expected error")),
                Err(err) => {
                    if !format!("{}", err).contains($ct) {
                        Err(err)
                    } else {
                        Ok(())
                    }
                }
            }
        };
    }

    macro_rules! dp {
        ( $p:expr ) => {
            format!("{}", $p.display())
        };
    }

    #[test]
    fn create_default_executor() {
        let _ = executive::new();
    }

    #[test]
    fn load_non_existent_file() -> Result<(), LoxError> {
        let e = executive::new();
        assert_error_contains!(e.run_file("test/not-a-file.file"), "No such file")
    }

    #[test]
    fn load_too_big_file() -> Result<(), LoxError> {
        let e = executive::new();
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/large.file");
        assert_error_contains!(e.run_file(&dp!(d)), "is too large")
    }

    #[test]
    fn load_directory() -> Result<(), LoxError> {
        let e = executive::new();
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        assert_error_contains!(e.run_file(&dp!(d)), "is not a file")
    }

    #[test]
    fn load_file_with_bad_statement() -> Result<(), LoxError> {
        let e = executive::new();
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/test-bad.lox");
        assert_error_contains!(e.run_file(&dp!(d)), "Bad input")
    }
}