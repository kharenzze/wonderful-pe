use crate::amount::Amount;
use crate::error::ParsingError;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&str")]
enum TransactionType {
  Deposit,
  Withdrawal,
  Dispute,
  Resolve,
  Chargeback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
struct Transaction {
  #[serde(rename = "type")]
  type_: TransactionType,
  client: u16,
  tx: u32,
  amount: Amount,
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

#[cfg(test)]
mod tests {
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
  }
}
