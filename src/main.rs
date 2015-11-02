mod load;
mod error;

use std::result;

pub use error::BenvError;
pub type Result<T> = result::Result<T, error::BenvError>;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
