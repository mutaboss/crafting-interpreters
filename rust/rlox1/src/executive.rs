use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader};

use crate::error::{LoxError};

const MAX_SOURCE_FILE_SIZE: u64 = 65535;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Executor {}
    }
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
            return Err(LoxError::new(&format!("Path {} is not a file.", filename)));
        } else if attr.len() > MAX_SOURCE_FILE_SIZE {
            return Err(LoxError::new(&format!(
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
            Err(LoxError::new(&format!("Bad input: {}", buffer)))
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
    use crate::error::{LoxError};
    use crate::executive::Executor;
    use std::path::PathBuf;

    macro_rules! assert_error_contains {
        ( $er:expr, $ct:expr ) => {
            match $er {
                Ok(()) => Err(LoxError::new("expected error")),
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

    macro_rules! assert_run_file {
        ( $fn:expr, $ct:expr ) => {
            {
                let e = Executor::new();
                assert_error_contains!(e.run_file(&get_resource($fn)), $ct)
            }
        };
    }

    fn get_resource(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/");
        d.push(name);
        format!("{}", d.display())
    }

    #[test]
    fn create_default_executor() {
        let _ = Executor::new();
    }

    #[test]
    fn load_non_existent_file() -> Result<(), LoxError> {
        assert_run_file!("not-a-file.file", "No such file")
    }

    #[test]
    fn load_too_big_file() -> Result<(), LoxError> {
        assert_run_file!("large.file", "is too large")
    }

    #[test]
    fn load_directory() -> Result<(), LoxError> {
        assert_run_file!(".", "is not a file")
    }

    #[test]
    fn load_file_with_bad_statement() -> Result<(), LoxError> {
        assert_run_file!("test-bad.lox", "Bad input")
    }
}
