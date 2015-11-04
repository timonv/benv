use std::io;

/// Generic error used accross the libary.
/// Wraps around other errors for genericity.
///
/// Used by Result in main.rs. `use` super::Result
/// and the rest should work like magic.
#[derive(Debug)]
pub enum BenvError {
    ParseError,
    SplitError(&'static str),
    IO(io::Error)
}

impl From<io::Error> for BenvError {
    fn from(error: io::Error) -> BenvError {
        BenvError::IO(error)
    }
}

// impl error::Error for BenvError {
//     fn description(&self) -> &str {
//         match *self {
//         }
//     }

//     fn cause(&self) -> Option<&error::Error> {
//         match *self {
//         }
//     }
// }
