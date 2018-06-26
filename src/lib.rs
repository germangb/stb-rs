use std::io;
use std::error::Error as StdError;

use std::fmt;

pub mod image;

#[derive(Debug)]
pub enum Error {
    Stb(String),
    Io(io::Error),
}

#[derive(Debug)]
pub struct Raw<T> {
    ptr: *const T,
}

impl<T> Raw<T> {
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Stb(_) => write!(f, "Error::Stb"),
            &Error::Io(_) => write!(f, "Error::Io"),
        }
    } 
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Stb(ref e) => e.as_str(),
            Error::Io(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref e) => e.cause(),
            _ => None,
        }
    }
}

/// Result type that reports STB errors
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
