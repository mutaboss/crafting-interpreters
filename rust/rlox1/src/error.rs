use std::fmt;

#[derive(Debug, Clone)]
pub struct LoxError {
    message: String,
}

impl LoxError {
    pub fn new(message: &str) -> LoxError {
        LoxError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.", self.message)
    }
}

impl std::error::Error for LoxError {}

impl From<std::io::Error> for LoxError {
    fn from(other: std::io::Error) -> Self {
        LoxError {
            message: format!("{}", other),
        }
    }
}
