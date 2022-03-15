use crate::amount::Amount;
use crate::error::{DynResult, TransactionProcessingError};
use crate::transaction::{ClientId, Transaction, TransactionType, TxId};
use std::collections::HashMap;

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
        let balance = self.get_or_create_mut_balance(transaction.client);
        if balance.locked {
          return Err(TransactionProcessingError::TargetAccountLocked.into());
        }
        balance.total += transaction.amount;
        balance.available += transaction.amount;

        self
          .transaction_history
          .insert(transaction.tx, TransactionRecord::from(transaction));
      }
      TransactionType::Withdrawal => {
        if self.transaction_history.get(&transaction.tx).is_some() {
          return Err(TransactionProcessingError::Duplicated.into());
        }
        let mut balance = self.get_or_create_mut_balance(transaction.client);
        if balance.locked {
          return Err(TransactionProcessingError::TargetAccountLocked.into());
        }
        balance.available = balance
          .available
          .checked_sub(transaction.amount)
          .ok_or_else(|| TransactionProcessingError::NotEnoughAvailable)?;
        balance.total = balance.total.checked_sub(transaction.amount).unwrap();

        self
          .transaction_history
          .insert(transaction.tx, TransactionRecord::from(transaction));
      }
      TransactionType::Dispute => {
        let record = self
          .transaction_history
          .get_mut(&transaction.tx)
          .ok_or_else(|| TransactionProcessingError::MissingTargetTransaction)?;
        if record.dispute_status.is_some() {
          return Err(TransactionProcessingError::AlreadyDisputed.into());
        }
        
      }
      TransactionType::Resolve => {
        unimplemented!();
      }
      TransactionType::Chargeback => {
        unimplemented!();
      }
    }
    Ok(())
  }

  fn get_or_create_mut_balance(&mut self, client: ClientId) -> &mut ClientBalance {
    if self.balances.get(&client).is_none() {
      self.balances.insert(client, ClientBalance::new(client));
    }
    self.balances.get_mut(&client).unwrap()
  }
}
