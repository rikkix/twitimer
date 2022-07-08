use std::convert::Infallible;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;

pub struct Error {
    pub code: Option<u8>,
    pub msg: String,
}

impl Error {
    pub fn new(code: Option<u8>, msg: String) -> Error {
        Error { code, msg }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Error {
        Error {
            code: Some(10),
            msg: e.to_string(),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error {
        Error {
            code: Some(10),
            msg: e.to_string(),
        }
    }
}

impl From<egg_mode::error::Error> for Error {
    fn from(e: egg_mode::error::Error) -> Self {
        Error {
            code: Some(32),
            msg: format!("Egg mode Error: {:#?}", e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(c) => write!(f, "Error: {}. (Code: {})", self.msg, c),
            None => write!(f, "Error: {}.", self.msg),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(c) => write!(f, "Error: {}. (Code: {})", self.msg, c),
            None => write!(f, "Error: {}.", self.msg),
        }
    }
}
