#[derive(Debug, Clone)]
pub enum Error {
    UnexpectedToken { expected: String, found: String },
}

pub type Result<T> = std::result::Result<T, Error>;
