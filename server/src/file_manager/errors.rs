use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct FileManagerError {
    msg: String,
}

impl FileManagerError {
    pub fn new(msg: &str) -> Self {
        FileManagerError { msg: msg.to_owned() }
    }
}

impl fmt::Display for FileManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

impl Error for FileManagerError {
    fn description(&self) -> &str {
        &self.msg
    }
}
