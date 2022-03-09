enum TransactionType {
  Deposit,
  Withdrawal,
  Dispute,
  Resolve,
  Chargeback,
}

struct Amount(i64);

struct Transaction {
  type_: TransactionType,
  client: u16,
  tx: u32,
  amount: Amount,
}

struct EngineStatus {
  balances: Vec<ClientBalance>
}

struct ClientBalance {
  client: u16,
  available: Amount,
  held: Amount,
  total: Amount,
  locked: bool,
}
