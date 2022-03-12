use serde::Deserialize;
use std::{cmp::Ordering, convert::TryFrom};
use crate::error::ParsingError;

const DECIMAL_SIZE: usize = 4;
const UNIT_MULTIPLIER: AmountInner = (10 as AmountInner).pow(DECIMAL_SIZE as u32);
type AmountInner = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(try_from = "&str")]
pub struct Amount(AmountInner);

#[inline]
fn to_inner(value: &str) -> Result<AmountInner, Box<dyn std::error::Error>> {
  let parts: Vec<&str> = value.split(".").collect();
  if parts.len() == 1 {
    let units: AmountInner = parts.get(0).unwrap().parse()?;
    return Ok(units * UNIT_MULTIPLIER);
  }
  if parts.len() != 2 {
    return Err(Box::new(ParsingError()));
  }
  let units: AmountInner = parts.get(0).unwrap().parse()?;
  let decimals = parts.get(1).unwrap_or(&"0000");
  let decimals: AmountInner = match decimals.len().cmp(&DECIMAL_SIZE) {
    Ordering::Equal => decimals.parse()?,
    Ordering::Greater => decimals[0..DECIMAL_SIZE].parse()?,
    Ordering::Less => {
      let n_missing_zeros = (DECIMAL_SIZE - decimals.len()) as AmountInner;
      let parsed: AmountInner = decimals.parse()?;
      parsed * (10 as AmountInner).pow(n_missing_zeros as u32)
    }
  };
  Ok(units * UNIT_MULTIPLIER + decimals as AmountInner)
}

impl TryFrom<&str> for Amount {
  type Error = ParsingError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    to_inner(value)
      .map(|inner| Amount(inner))
      .map_err(|_| ParsingError())
  }
}

#[cfg(test)]
mod tests {
  use super::Amount;
  use super::ParsingError;

  #[test]
  fn simple() {
    assert_eq!(Amount(10000), Amount::try_from("1.0000").unwrap());
    assert_eq!(Amount(20000), Amount::try_from("2.0").unwrap());
    assert_eq!(Amount(1005000), Amount::try_from("100.50").unwrap());
    assert_eq!(Amount(415050), Amount::try_from("41.505").unwrap());
    assert_eq!(Amount(10001), Amount::try_from("1.0001012313").unwrap());
    assert_eq!(Amount(50000), Amount::try_from("5").unwrap());
  }

  #[test]
  fn errors() {
    assert_eq!(Err(ParsingError()), Amount::try_from("hi"));
    assert_eq!(Err(ParsingError()), Amount::try_from("3.0.0"));
  }

  #[test]
  fn deserialization() {
    //let a: Amount = Deserialize::deserialize("3.0").unwrap();
    assert_eq!(Err(ParsingError()), Amount::try_from("hi"));
  }
}
