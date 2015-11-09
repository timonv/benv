extern crate rustc_serialize;
extern crate docopt;


mod load;
mod error;
mod env;
mod run;

use run::run;
use load::load_file;
use docopt::Docopt;
use std::result;
use std::path::Path;
use std::process::exit;

pub use error::BenvError;

/// Generic Result type. Never return internal results.
pub type Result<T> = result::Result<T, BenvError>;

const USAGE: &'static str = "
Benv

A tool to load .env ('dotenv') files and set it as
the environment for a program to run.

Usage: benv <dotenv> <program>
       benv [--help | --version]

Options:
  -h --help    Show this screen.
  --version    Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_dotenv: String,
    arg_program: String
}

#[cfg(not(test))]
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.arg_dotenv == "" {
      println!("Missing env file");
      println!("{}", USAGE);
      exit(1);
    }

    if args.arg_program == "" {
      println!("Missing program");
      println!("{}", USAGE);
      exit(1);
    }

    load_file(&Path::new(&args.arg_dotenv))
      .and_then(|env_list| run(&args.arg_program, env_list))
      .and_then(|mut process| process.wait().map_err(BenvError::IO))
      .map_err(print_user_friendly_error_and_exit).unwrap();
}

#[cfg(not(test))]
fn print_user_friendly_error_and_exit(error: BenvError) {
  println!("{}", error);
  println!("{}", USAGE);
  exit(1);
}

