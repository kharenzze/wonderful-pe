use crate::amount::Amount;
enum TransactionType {
  Deposit,
  Withdrawal,
  Dispute,
  Resolve,
  Chargeback,
}

struct Transaction {
  type_: TransactionType,
  client: u16,
  tx: u32,
  amount: Amount,
}