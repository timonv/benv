use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

use super::Result;

#[derive(Debug, Eq, PartialEq)]
pub struct Env {
    name: String,
    value: String

}
pub type EnvList = Vec<Env>;

pub fn load_file(path: &Path) -> Result<EnvList> {
   let mut line = String::new();
   let file = try!(File::open(path));
   try!(BufReader::new(file).read_line(&mut line));

   let vals: Vec<&str> = line.split("=").collect();
   let env = Env {
       name: vals[0].trim().to_string(),
       value: vals[1].trim().to_string()
   };

   Ok(vec![env])

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
}
