use crate::amount::Amount;
use crate::error::{ ParsingError, TransactionProcessingError};
use serde::Deserialize;
use std::convert::TryFrom;

pub type ClientId = u16;
pub type TxId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&str")]
pub enum TransactionType {
  Deposit,
  Withdrawal,
  Dispute,
  Resolve,
  Chargeback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct Transaction {
  #[serde(rename = "type")]
  pub type_: TransactionType,
  pub client: ClientId,
  pub tx: TxId,
  pub amount: Option<Amount>,
}

impl TryFrom<&str> for TransactionType {
  type Error = ParsingError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "deposit" => Ok(Self::Deposit),
      "withdrawal" => Ok(Self::Withdrawal),
      "dispute" => Ok(Self::Dispute),
      "resolve" => Ok(Self::Resolve),
      "chargeback" => Ok(Self::Chargeback),
      _ => Err(ParsingError()),
    }
  }
}

impl Transaction {
  pub fn can_be_disputed_by(&self, &dispute: &Self) -> Result<(), TransactionProcessingError> {
    if self.type_ != TransactionType::Deposit {
      return Err(TransactionProcessingError::DisputingWrongTransactionType);
    }
    if self.client != dispute.client {
      return Err(TransactionProcessingError::ClientDoesNotMatch);
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::transaction::TransactionType;

  use super::Amount;
  use super::Transaction;

  #[test]
  fn simple() {
    let transactions: Vec<Transaction> = csv::ReaderBuilder::new()
      .trim(csv::Trim::All)
      .from_path("./tests/samples/simple.csv")
      .unwrap()
      .deserialize()
      .map(|res| res.unwrap())
      .collect();
    assert_eq!(transactions.len(), 1);
    let t = transactions[0];
    assert_eq!(t.amount, Amount::try_from("1.0").ok());
    assert_eq!(t.client, 1);
    assert_eq!(t.tx, 1);
    assert_eq!(t.type_, TransactionType::Deposit);
  }
}
