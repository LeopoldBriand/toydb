use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ParserError {
    msg: String,
}

impl ParserError {
    pub fn new(msg: &str) -> Self {
        ParserError { msg: msg.to_owned() }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        &self.msg
    }
}
