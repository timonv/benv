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
      exit(1);
    }

    if args.arg_program == "" {
      println!("Missing program");
      exit(1);
    }

    let envlist = load_file(&Path::new(&args.arg_dotenv)).unwrap();
    run(&args.arg_program, envlist).unwrap().wait().unwrap();
}
