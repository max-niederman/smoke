#[derive(Debug, Clone)]
pub enum Error {
    Internal(&'static str),

    UnexpectedToken { expected: String, found: String },
}

pub type Result<T> = std::result::Result<T, Error>;
