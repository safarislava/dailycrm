use crate::model::contract::comment_text::CommentText;
use crate::model::project::stage::cost::formatted_cost::FormattedCost;
use crate::model::project::stage::cost::valid_cost::ValidCost;

pub struct AdvanceCostChangeText {
    old: ValidCost,
    new: Option<ValidCost>,
}

impl AdvanceCostChangeText {
    pub fn new(old: ValidCost, new: Option<ValidCost>) -> Self {
        Self { old, new }
    }
}

impl CommentText for AdvanceCostChangeText {
    fn text(&self) -> String {
        match &self.new {
            Some(new) => format!(
                "Аванс изменён: {} ₽ → {} ₽",
                FormattedCost(self.old),
                FormattedCost(*new)
            ),
            None => format!("Аванс удалён: {} ₽", FormattedCost(self.old)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn formats_advance_cost_change_and_removal() {
        let change = AdvanceCostChangeText::new(100000.into(), Some(250000.into()));
        assert_eq!(change.text(), "Аванс изменён: 100 000 ₽ → 250 000 ₽");

        let removal = AdvanceCostChangeText::new(100000.into(), None);
        assert_eq!(removal.text(), "Аванс удалён: 100 000 ₽");

        let kopecks_change = AdvanceCostChangeText::new(
            Decimal::from_str("100000.5").unwrap().into(),
            Some(Decimal::from_str("250000.75").unwrap().into()),
        );
        assert_eq!(
            kopecks_change.text(),
            "Аванс изменён: 100 000,50 ₽ → 250 000,75 ₽"
        );
    }
}
