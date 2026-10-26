use crate::model::contract::comment_text::CommentText;
use crate::model::project::stage::cost::contract::cost::Cost;
use crate::model::project::stage::cost::formatted_cost::FormattedCost;

pub struct AdvanceCostChangeText {
    old: Box<dyn Cost>,
    new: Option<Box<dyn Cost>>,
}

impl AdvanceCostChangeText {
    pub fn new(old: Box<dyn Cost>, new: Option<Box<dyn Cost>>) -> Self {
        Self { old, new }
    }
}

impl CommentText for AdvanceCostChangeText {
    fn text(&self) -> String {
        match &self.new {
            Some(new) => format!(
                "Аванс изменён: {} ₽ → {} ₽",
                FormattedCost(self.old.as_ref()),
                FormattedCost(new.as_ref())
            ),
            None => format!("Аванс удалён: {} ₽", FormattedCost(self.old.as_ref())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::project::stage::cost::valid_cost::ValidCost;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn formats_advance_cost_change_and_removal() {
        let change = AdvanceCostChangeText::new(
            Box::new(ValidCost::new(Decimal::from(100000))),
            Some(Box::new(ValidCost::new(Decimal::from(250000)))),
        );
        assert_eq!(change.text(), "Аванс изменён: 100 000 ₽ → 250 000 ₽");

        let removal =
            AdvanceCostChangeText::new(Box::new(ValidCost::new(Decimal::from(100000))), None);
        assert_eq!(removal.text(), "Аванс удалён: 100 000 ₽");

        let kopecks_change = AdvanceCostChangeText::new(
            Box::new(ValidCost::new(Decimal::from_str("100000.5").unwrap())),
            Some(Box::new(ValidCost::new(
                Decimal::from_str("250000.75").unwrap(),
            ))),
        );
        assert_eq!(
            kopecks_change.text(),
            "Аванс изменён: 100 000,50 ₽ → 250 000,75 ₽"
        );
    }
}
