mod load;
mod error;

use std::result;

pub use error::BenvError;

/// Generic Result type. Never return internal results.
pub type Result<T> = result::Result<T, BenvError>;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
