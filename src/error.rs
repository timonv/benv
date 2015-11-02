use std::io;

#[derive(Debug)]
pub enum BenvError {
    ParseError,
    IO(io::Error)
}

impl From<io::Error> for BenvError {
    fn from(error: io::Error) -> BenvError {
        BenvError::IO(error)
    }
}

