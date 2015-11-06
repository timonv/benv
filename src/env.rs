/// Env wraps environment variables (NAME=VALUE)
#[derive(Debug, Eq, PartialEq)]
pub struct Env {
    pub name: String,
    pub value: String

}

/// EnvList is a wrapper for a list of Env
pub type EnvList = Vec<Env>;

impl Env {
    pub fn new(name: &str, value: &str) -> Env {
        Env { name: name.to_string(), value: value.to_string()}
    }
}
