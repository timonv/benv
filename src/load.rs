use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use error::BenvError;

use super::Result;
use env::{Env, EnvList};

/// Loads an environment file and returns a Result<EnvList>
///
/// A file can look like this:
///
/// DATASASE=postgres://myserver/mydatabase
/// # A comment
/// // Another comment
/// SECRET_KEY=qwerty
pub fn load_file(path: &Path) -> Result<EnvList> {
   let lines = try!(read_lines_reverse(path));

   let mut list: EnvList = vec![];
   let mut seen: Vec<String> = vec![];

   for line in lines.iter() {
      let parsed = try!(split(line).and_then(parse));
      if !seen.iter().by_ref().any(|e| e == &parsed.name) {
         seen.push(parsed.name.clone());
         list.push(parsed);
      }
   }

   list.reverse();

   Ok(list)
}

fn read_lines_reverse(path: &Path) -> Result<Vec<String>> {
   let file = try!(File::open(path));
   let mut lines = vec![];
   for line in BufReader::new(file).lines() {
      let line = try!(line);
      if !is_comment(&line) {
         lines.insert(0, line);
      }
   }

   Ok(lines)
}

fn parse((name, value): (String, String)) -> Result<Env> {
   Ok(Env::new(&name, &value))
}

fn split(data: &str) -> Result<(String, String)> {
   let vals: Vec<&str> = data.splitn(2,"=").collect();
   if vals.len() != 2 { return Err(BenvError::SplitError("Not two elements on split")) }

   Ok((vals[0].to_string(), vals[1].to_string()))
}

fn is_comment(line: &str) -> bool {
   line.starts_with("#") || line.starts_with("//")
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;
    use env::Env;

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
       let path = Path::new("fixtures/multi_eq");
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

    #[test]
    fn test_overwrite_on_double() {
       let path = Path::new("fixtures/overwrite");
       let result = load_file(&path);
       let expected = vec![Env {
          name: "HELLO".to_string(),
          value: "world2".to_string()
       }];
       assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_ignore_comments() {
       let path = Path::new("fixtures/comments");
       let result = load_file(&path);
       let expected = Env {
          name: "HELLO".to_string(),
          value: "world".to_string()
       };

       assert_eq!(result.unwrap(), vec![expected]);
    }

    // Test URI
    // Test overwrite on double occurrence
}
