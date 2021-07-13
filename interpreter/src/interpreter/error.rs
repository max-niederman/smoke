use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Error {
    TypeError { expected: String, found: String },
    ReferenceUndefinedError { name: String },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::TypeError { expected, found } => {
                write!(f, "expected value of type {} but found {}", expected, found)
            }
            Self::ReferenceUndefinedError { name } => {
                write!(f, "reference by name {} was not in scope", name)
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
