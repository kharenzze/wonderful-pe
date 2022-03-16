use crate::amount::Amount;
use crate::error::{DynResult, TransactionProcessingError};
use crate::transaction::{ClientId, Transaction, TransactionType, TxId};
use std::collections::HashMap;

type TransResult<T> = Result<T, TransactionProcessingError>;

#[derive(Debug, Default, Clone)]
pub struct Engine {
  balances: HashMap<ClientId, ClientBalance>,
  transaction_history: HashMap<TxId, TransactionRecord>,
}

#[derive(Debug, Clone)]
struct TransactionRecord {
  transaction: Transaction,
  dispute_status: Option<TransactionType>,
}

impl From<&Transaction> for TransactionRecord {
  fn from(t: &Transaction) -> Self {
    Self {
      transaction: t.clone(),
      dispute_status: None,
    }
  }
}

#[derive(Debug, Default, Clone, Copy)]
struct ClientBalance {
  client: ClientId,
  available: Amount,
  held: Amount,
  total: Amount,
  locked: bool,
}

impl ClientBalance {
  fn new(client: ClientId) -> Self {
    Self {
      client,
      available: Default::default(),
      held: Default::default(),
      total: Default::default(),
      locked: false,
    }
  }
}

impl Engine {
  pub fn ingest_csv(&mut self, filename: &str) -> DynResult<()> {
    csv::ReaderBuilder::new()
      .trim(csv::Trim::All)
      .from_path(filename)?
      .deserialize()
      .filter(|res| res.is_ok())
      .for_each(|res| {
        let transaction: Transaction = res.unwrap();
        let processed = self.apply_transaction(&transaction);
      });
    Ok(())
  }

  fn apply_transaction(&mut self, transaction: &Transaction) -> DynResult<()> {
    match transaction.type_ {
      TransactionType::Deposit => {
        if self.transaction_history.get(&transaction.tx).is_some() {
          return Err(TransactionProcessingError::Duplicated.into());
        }
        let balance = self.get_or_create_balance_mut(transaction.client)?;
        let amount = transaction
          .amount
          .ok_or_else(|| TransactionProcessingError::MissingTransactionAmount)?;
        balance.total += amount;
        balance.available += amount;

        self
          .transaction_history
          .insert(transaction.tx, TransactionRecord::from(transaction));
      }
      TransactionType::Withdrawal => {
        if self.transaction_history.get(&transaction.tx).is_some() {
          return Err(TransactionProcessingError::Duplicated.into());
        }
        let mut balance = self.get_or_create_balance_mut(transaction.client)?;
        let amount = transaction
          .amount
          .ok_or_else(|| TransactionProcessingError::MissingTransactionAmount)?;
        balance.available = balance
          .available
          .checked_sub(amount)
          .ok_or_else(|| TransactionProcessingError::NotEnoughAvailable)?;
        balance.total = balance.total.checked_sub(amount).unwrap();

        self
          .transaction_history
          .insert(transaction.tx, TransactionRecord::from(transaction));
      }
      TransactionType::Dispute => {
        let record = self.get_tx_record(transaction.tx)?;
        if record.dispute_status.is_some() {
          return Err(TransactionProcessingError::AlreadyDisputed.into());
        }
        record.transaction.can_be_disputed_by(transaction)?;

        let amount = record.transaction.amount.unwrap();
        let mut balance = self.get_or_create_balance_mut(transaction.client)?;
        balance.available = balance
          .available
          .checked_sub(amount)
          .ok_or_else(|| TransactionProcessingError::NotEnoughAvailable)?;
        balance.held += amount;

        let record = self.get_tx_record_mut(transaction.tx)?;
        record.dispute_status = Some(TransactionType::Dispute);
      }
      TransactionType::Resolve => {
        let record = self.get_tx_record(transaction.tx)?;
        if !record.dispute_status.eq(&Some(TransactionType::Dispute)) {
          return Err(TransactionProcessingError::NotDisputed.into());
        }
        record.transaction.can_be_disputed_by(transaction)?;

        let amount = record.transaction.amount.unwrap();
        let mut balance = self.get_or_create_balance_mut(transaction.client)?;
        balance.held = balance
          .held
          .checked_sub(amount)
          .ok_or_else(|| TransactionProcessingError::NotEnoughHeld)?;
        balance.available += amount;

        let record = self.get_tx_record_mut(transaction.tx)?;
        record.dispute_status = Some(TransactionType::Resolve);
      }
      TransactionType::Chargeback => {
        let record = self.get_tx_record(transaction.tx)?;
        if !record.dispute_status.eq(&Some(TransactionType::Dispute)) {
          return Err(TransactionProcessingError::NotDisputed.into());
        }
        record.transaction.can_be_disputed_by(transaction)?;

        let amount = record.transaction.amount.unwrap();
        let mut balance = self.get_or_create_balance_mut(transaction.client)?;
        let held = balance
          .held
          .checked_sub(amount)
          .ok_or_else(|| TransactionProcessingError::NotEnoughHeld)?;
        let total = balance
          .total
          .checked_sub(amount)
          .ok_or_else(|| TransactionProcessingError::NotEnoughTotal)?;

        balance.held = held;
        balance.total = total;
        balance.locked = true;

        let record = self.get_tx_record_mut(transaction.tx)?;
        record.dispute_status = Some(TransactionType::Chargeback);
      }
    }
    Ok(())
  }

  #[inline]
  fn get_tx_record(&self, tx: TxId) -> TransResult<&TransactionRecord> {
    self
      .transaction_history
      .get(&tx)
      .ok_or_else(|| TransactionProcessingError::MissingTargetTransaction)
  }

  #[inline]
  fn get_tx_record_mut(&mut self, tx: TxId) -> TransResult<&mut TransactionRecord> {
    self
      .transaction_history
      .get_mut(&tx)
      .ok_or_else(|| TransactionProcessingError::MissingTargetTransaction)
  }

  #[inline]
  fn get_or_create_balance_mut(&mut self, client: ClientId) -> TransResult<&mut ClientBalance> {
    if self.balances.get(&client).is_none() {
      self.balances.insert(client, ClientBalance::new(client));
    }
    let b = self.balances.get_mut(&client).unwrap();
    if b.locked {
      return Err(TransactionProcessingError::TargetAccountLocked);
    }
    Ok(b)
  }
}

#[cfg(test)]
mod tests {
  use super::Engine;

  #[test]
  fn little() {
    let mut engine: Engine = Default::default();
    let ingest_result = engine.ingest_csv("./tests/samples/little.csv");
    assert_eq!(ingest_result.is_ok(), true);
  }
}
