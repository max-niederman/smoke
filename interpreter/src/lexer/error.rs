#[derive(Debug, Clone, Copy)]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;
