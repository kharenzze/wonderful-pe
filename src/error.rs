use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[error("Could not parse string")]
pub struct ParsingError();

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TransactionProcessingError {
  #[error("Duplicated. Tx already known")]
  Duplicated,
  #[error("Target account is locked")]
  TargetAccountLocked,
  #[error("Available money is less than withdraw amount")]
  NotEnoughAvailable,
  #[error("Target transaction does not exist")]
  MissingTargetTransaction,
  #[error("TransactionAlreadyDisputed")]
  AlreadyDisputed
} 

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;