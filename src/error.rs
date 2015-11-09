use std::io;
use std::fmt;

/// Generic error used accross the libary.
/// Wraps around other errors for genericity.
///
/// Used by Result in main.rs. `use` super::Result
/// and the rest should work like magic.
#[derive(Debug)]
pub enum BenvError {
    SplitError(&'static str),
    IO(io::Error),
    MissingProgram
}

impl From<io::Error> for BenvError {
    fn from(error: io::Error) -> BenvError {
        BenvError::IO(error)
    }
}

impl fmt::Display for BenvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BenvError::IO(ref err) => write!(f, "IO error: {}", err),
            BenvError::SplitError(ref err) => write!(f, "Split error: {}", err),
            BenvError::MissingProgram => write!(f, "Missing program")
        }
    }
}
