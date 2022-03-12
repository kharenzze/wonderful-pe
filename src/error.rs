use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[error("Could not parse string")]
pub struct ParsingError();