mod amount;
mod transaction;

use self::amount::Amount;


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
