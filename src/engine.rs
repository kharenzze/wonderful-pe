use crate::amount::Amount;
use crate::error::DynResult;
use crate::transaction::{Transaction, TransactionType};
use std::collections::HashMap;

type ClientId = u16;

#[derive(Debug, Default, Clone)]
pub struct Engine {
  balances: HashMap<ClientId, ClientBalance>,
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
        println!("{:?}", &transaction);
      });
    Ok(())
  }

  fn apply_transaction(&mut self, transaction: &Transaction) -> DynResult<()> {
    let mut balance = self.get_or_create_mut_balance(transaction.client);
    match transaction.type_ {
      TransactionType::Deposit => {
        balance.total += transaction.amount;
        balance.available += transaction.amount;
      },
      TransactionType::Withdrawal => {
        balance.available = balance.available.checked_sub(transaction.amount).ok_or("Not enough available")?;
        balance.total = balance.total.checked_sub(transaction.amount).unwrap();
      },
      TransactionType::Dispute => {
        unimplemented!();
      },
      TransactionType::Resolve => {
        unimplemented!();
      },
      TransactionType::Chargeback => {
        unimplemented!();
      },
      _ => unimplemented!()
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
