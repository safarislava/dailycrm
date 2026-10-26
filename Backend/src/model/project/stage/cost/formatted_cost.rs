use std::fmt;

pub struct FormattedCost(pub i32);

impl fmt::Display for FormattedCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_negative = self.0 < 0;
        let abs_val = self.0.unsigned_abs();
        let s = abs_val.to_string();
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_cost_with_thousand_separators() {
        assert_eq!(FormattedCost(1000).to_string(), "1 000");
        assert_eq!(FormattedCost(1000000).to_string(), "1 000 000");
        assert_eq!(FormattedCost(500).to_string(), "500");
        assert_eq!(FormattedCost(0).to_string(), "0");
        assert_eq!(FormattedCost(-1000).to_string(), "-1 000");
    }
}
