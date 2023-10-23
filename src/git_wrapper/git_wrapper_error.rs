use std::fmt;

pub struct GitWrapperError {
    message: String,
}

impl GitWrapperError {
    pub fn new(message: String) -> GitWrapperError {
        GitWrapperError { message }
    }

    pub fn message(&self) -> &str { self.message.as_str() }
}

impl fmt::Display for GitWrapperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",  self.message) // user-facing output
    }
}

impl fmt::Debug for GitWrapperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {{ file: {}, line: {} }}", self.message, file!(), line!()) // programmer-facing output
    }
}
