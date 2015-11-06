use std::process::{Command, Child};
use env::{Env, EnvList};
use super::Result;
use error::BenvError;

pub fn run(program_with_args: &str, env_list: EnvList) -> Result<Child> {
    let (program, args) = try!(split_program_and_args(program_with_args));
    let mut command = Command::new(&program);
    command.args(&args);
    for env in env_list {
        command.env(env.name, env.value);
    }
    let child = try!(command.spawn());

    Ok(child)
}

fn split_program_and_args(program_with_args: &str) -> Result<(String, Vec<&str>)> {
    // Life would have been good...
    // match program_with_args.split_whitespace() {
    //     [program, .. args] => (program.to_string(), args.to_string())
    // }
    let mut vec: Vec<&str> = program_with_args.split_whitespace().collect();
    if vec.len() == 0 {
        return Err(BenvError::MissingProgram);
    }
    let program = vec.remove(0).to_string();
    Ok((program, vec))
}

#[cfg(test)]
mod test {
    use super::*;
    use env::{Env, EnvList};

    #[test]
    fn test_simple_command() {
        // TODO
        //
        // With latest nightly, it seems impossible to write a proper test case
        // where stdout of the child process is captured.
        //
        // let envlist: EnvList = vec![Env::new("HELLO", "World")];
        // let child = run("echo $HELLO", envlist).unwrap().wait_with_output().unwrap();
        // println!("{:?}", child.stderr);
        // let result = String::from_utf8(child.stdout).unwrap();


        // assert_eq!(result, "world");
    }
}
