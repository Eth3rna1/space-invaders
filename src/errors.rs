use std::convert::AsRef;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    OutOfBounds,
    InexistentSprite,
    OverlappingSprite,
    Other,
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    diagnosis: String,
}

impl Error {
    pub fn new<T: AsRef<str> + ToString>(kind: ErrorKind, diagnosis: T) -> Self {
        Self {
            kind,
            diagnosis: diagnosis.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {})", &self.kind, &self.diagnosis)
    }
}
