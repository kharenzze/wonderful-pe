use crate::amount::Amount;
use crate::error::DynResult;
use crate::transaction::Transaction;
use std::collections::HashMap;

type ClientId = u16;

#[derive(Debug, Default, Clone)]
struct Engine {
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

impl Engine {
  fn ingest_csv(&mut self, filename: &str) -> DynResult<()> {
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
}
