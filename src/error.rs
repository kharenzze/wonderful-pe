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
  #[error("Held money is less than expected")]
  NotEnoughHeld,
  #[error("Total money is less than expected")]
  NotEnoughTotal,
  #[error("Target transaction does not exist")]
  MissingTargetTransaction,
  #[error("Transaction already disputed")]
  AlreadyDisputed,
  #[error("Can only dispute Deposit transactions")]
  DisputingWrongTransactionType,
  #[error("Missing transaction amount")]
  MissingTransactionAmount,
  #[error("Client does not match")]
  ClientDoesNotMatch,
  #[error("Transaction is not being disputed")]
  NotDisputed,
}

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;
