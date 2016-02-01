#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(test, allow(unused_imports))]

extern crate rustc_serialize;
extern crate docopt;

mod env;
mod error;
mod load;
mod run;

use docopt::Docopt;
use error::BenvError;
use load::load_file;
use run::run;
use std::path::Path;
use std::process::exit;
use std::result;

/// Generic Result type. Never return internal results.
pub type Result<T> = result::Result<T, BenvError>;

const USAGE: &'static str = "
Benv

A tool to load .env ('dotenv') files and set it as
the environment for a program to run.

Usage: benv <dotenv> <program>...
       benv [--help | --version]

Options:
  -h --help    Show this screen.
  --version    Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_dotenv: String,
    arg_program: Vec<String>
}

#[cfg(not(test))]
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    load_file(&Path::new(&args.arg_dotenv))
      .and_then(|env_list| run(&args.arg_program.join(" "), env_list))
      .and_then(|mut process| process.wait().map_err(BenvError::IO))
      .map_err(print_user_friendly_error_and_exit).unwrap();
}

#[cfg(not(test))]
fn print_user_friendly_error_and_exit(error: BenvError) {
  println!("{}", error);
  println!("{}", USAGE);
  exit(1);
}

