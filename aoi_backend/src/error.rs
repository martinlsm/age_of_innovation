use std::fmt;

#[derive(Debug, Clone)]
struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

pub fn create_error(msg: &str) -> Box<dyn std::error::Error> {
    Box::new(Error(String::from(msg)))
}
