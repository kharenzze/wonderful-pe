use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[error("Could not parse string")]
pub struct ParsingError();

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TransactionProcessingError {
  #[error("Duplicated. Tx already known")]
  Duplicated
} 

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;