use crate::model::project::stage::cost::contract::cost::{Cost, CostError};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct ValidCost(Decimal);

impl ValidCost {
    pub fn new(decimal: Decimal) -> Self {
        Self(decimal)
    }
}

impl Cost for ValidCost {
    fn as_decimal(&self) -> Result<Decimal, CostError> {
        if self.0.scale() > 2 {
            return Err(CostError::TooManyDecimalPlaces);
        }
        Ok(self.0)
    }

    fn clone_box(&self) -> Box<dyn Cost> {
        Box::new(*self)
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
}
