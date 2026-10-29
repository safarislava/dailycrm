use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum CostError {
    TooManyDecimalPlaces,
}

impl std::fmt::Display for CostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyDecimalPlaces => {
                f.write_str("Cost cannot have more than 2 decimal places")
            }
        }
    }
}

impl std::error::Error for CostError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct ValidCost(Decimal);

impl ValidCost {
    pub fn new(decimal: Decimal) -> Self {
        Self(decimal)
    }

    pub fn as_decimal(&self) -> Result<Decimal, CostError> {
        if self.0.scale() > 2 {
            return Err(CostError::TooManyDecimalPlaces);
        }
        Ok(self.0)
    }
}

impl From<ValidCost> for Decimal {
    fn from(cost: ValidCost) -> Self {
        cost.0
    }
}

impl From<Decimal> for ValidCost {
    fn from(decimal: Decimal) -> Self {
        Self::new(decimal)
    }
}

impl From<i32> for ValidCost {
    fn from(n: i32) -> Self {
        Self::new(Decimal::from(n))
    }
}

impl<'de> Deserialize<'de> for ValidCost {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let dec = <Decimal as Deserialize>::deserialize(deserializer)?;
        let valid = ValidCost::new(dec);
        valid.as_decimal().map_err(serde::de::Error::custom)?;
        Ok(valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn accepts_valid_integer_cost() {
        assert_eq!(
            ValidCost::new(Decimal::from(1000)).as_decimal(),
            Ok(Decimal::from(1000))
        );
    }

    #[test]
    fn accepts_valid_cost_with_two_decimals() {
        assert_eq!(
            ValidCost::new(Decimal::from_str("1000.50").unwrap()).as_decimal(),
            Ok(Decimal::from_str("1000.50").unwrap())
        );
    }

    #[test]
    fn accepts_valid_cost_with_one_decimal() {
        assert_eq!(
            ValidCost::new(Decimal::from_str("1000.5").unwrap()).as_decimal(),
            Ok(Decimal::from_str("1000.5").unwrap())
        );
    }

    #[test]
    fn accepts_zero_cost() {
        assert_eq!(
            ValidCost::new(Decimal::ZERO).as_decimal(),
            Ok(Decimal::ZERO)
        );
    }

    #[test]
    fn rejects_cost_with_more_than_two_decimal_places() {
        assert_eq!(
            ValidCost::new(Decimal::from_str("1000.555").unwrap()).as_decimal(),
            Err(CostError::TooManyDecimalPlaces)
        );
    }

    #[test]
    fn deserializes_valid_cost_json() {
        let cost: ValidCost = serde_json::from_str("100.50").unwrap();
        assert_eq!(cost.as_decimal().unwrap(), Decimal::from_str("100.50").unwrap());
    }

    #[test]
    fn rejects_deserializing_too_many_decimals() {
        let res: Result<ValidCost, _> = serde_json::from_str("100.555");
        assert!(res.is_err());
    }
}
