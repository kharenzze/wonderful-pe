use std::convert::TryFrom;
use thiserror::Error;

type AmountInner = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount(AmountInner);

#[derive(Debug, Error)]
#[error("Could not convert string into Amount")]
pub struct ConversionError();

impl TryFrom<&str> for Amount {
  type Error = ConversionError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let n: AmountInner = 0;
    Ok(Amount(n))
  }
}

#[cfg(test)]
mod tests {
  use super::Amount;

  #[test]
  fn simple() {
    assert_eq!(Amount(1), Amount::try_from("1.0000").unwrap());
  }
}
