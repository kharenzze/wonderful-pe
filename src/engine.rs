use crate::amount::Amount;

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