use crate::model::project::stage::cost::valid_cost::ValidCost;
use rust_decimal::Decimal;
use std::fmt;

pub struct FormattedCost(pub ValidCost);

impl fmt::Display for FormattedCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let decimal = self.0.as_decimal().unwrap_or(Decimal::ZERO);
        let is_negative = decimal.is_sign_negative();
        let abs_val = decimal.abs();
        let rubles = abs_val.trunc();
        let fract = (abs_val.fract() * Decimal::from(100)).trunc();

        let s = rubles.to_string();
        let bytes = s.as_bytes();
        let len = bytes.len();
        if is_negative {
            write!(f, "-")?;
        }
        for (i, &b) in bytes.iter().enumerate() {
            if i > 0 && (len - i).is_multiple_of(3) {
                write!(f, " ")?;
            }
            write!(f, "{}", b as char)?;
        }
        if !fract.is_zero() {
            let kopecks: u32 = fract.to_string().parse().unwrap_or(0);
            write!(f, ",{:02}", kopecks)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn formats_cost_with_thousand_separators() {
        let cost_1000: ValidCost = 1000.into();
        assert_eq!(FormattedCost(cost_1000).to_string(), "1 000");

        let cost_mil: ValidCost = 1000000.into();
        assert_eq!(FormattedCost(cost_mil).to_string(), "1 000 000");

        let cost_500: ValidCost = 500.into();
        assert_eq!(FormattedCost(cost_500).to_string(), "500");

        let cost_0: ValidCost = 0.into();
        assert_eq!(FormattedCost(cost_0).to_string(), "0");

        let cost_neg: ValidCost = (-1000).into();
        assert_eq!(FormattedCost(cost_neg).to_string(), "-1 000");

        let cost_dec1: ValidCost = Decimal::from_str("1000.5").unwrap().into();
        assert_eq!(FormattedCost(cost_dec1).to_string(), "1 000,50");

        let cost_dec2: ValidCost = Decimal::from_str("1000.05").unwrap().into();
        assert_eq!(FormattedCost(cost_dec2).to_string(), "1 000,05");
    }
}
