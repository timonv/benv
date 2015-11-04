use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, self};
use error::BenvError;

use super::Result;

#[derive(Debug, Eq, PartialEq)]
pub struct Env {
    name: String,
    value: String

}
pub type EnvList = Vec<Env>;

pub fn load_file(path: &Path) -> Result<EnvList> {
   let file = try!(File::open(path));

   let mut list: EnvList = vec![];
   for line in BufReader::new(file).lines() {
      let line = try!(line);
      let parsed = try!(parse(line));
      list.push(parsed);
   }

   Ok(list)
}

fn parse(data: String) -> Result<Env> {
   let (name,value) = try!(split(&data));
   Ok(Env { name: name, value: value})
}

fn split(data: &str) -> Result<(String, String)> {
   let vals: Vec<&str> = data.splitn(2,"=").collect();
   if vals.len() != 2 { return Err(BenvError::SplitError("More than two elements on split")) }

   Ok((vals[0].to_string(), vals[1].to_string()))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_single_var() {
        let path = Path::new("fixtures/single_var");
        let result = load_file(&path);
        let expected = Env {
            name: "HELLO".to_string(),
            value: "world".to_string()
        };

        assert_eq!(result.unwrap()[0], expected);
    }

    #[test]
    fn test_two_vars() {
       let path = Path::new("fixtures/two_vars");
       let result = load_file(&path);
       let expected = vec![
          Env {
             name: "ONE".to_string(),
             value: "one".to_string(),
          },
          Env {
             name: "TWO".to_string(),
             value: "two".to_string()
          }
       ];

       assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_gibberish() {
        let path = Path::new("fixtures/gibberish");
        let result = load_file(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_eq() {
       let path= Path::new("fixtures/multi_eq");
       let result = load_file(&path);
       let expected = Env {
          name: "HELLO".to_string(),
          value: "wor=ld".to_string()
       };
       assert_eq!(result.unwrap()[0], expected);
    }

    #[test]
    fn test_uri_with_quotes() {
       let path = Path::new("fixtures/uri");
       let result = load_file(&path);
       let expected = Env {
          name: "DATABASE".to_string(),
          value: "\"postgres://my_favourite_database.com/abc\"".to_string()
       };
       assert_eq!(result.unwrap()[0], expected);
    }
    // Test URI
    // Test overwrite on double occurrence
}
