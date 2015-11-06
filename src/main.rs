mod load;
mod error;
mod env;
mod run;

use std::result;
use run::run;
use env::Env;

pub use error::BenvError;

/// Generic Result type. Never return internal results.
pub type Result<T> = result::Result<T, BenvError>;

#[cfg(not(test))]
fn main() {
    run("env", vec![Env::new("HELLO", "World")]);
}
