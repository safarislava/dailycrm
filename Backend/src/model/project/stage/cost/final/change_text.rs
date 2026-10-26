use crate::model::contract::comment_text::CommentText;
use crate::model::project::stage::cost::contract::cost::Cost;
use crate::model::project::stage::cost::formatted_cost::FormattedCost;

pub struct FinalCostChangeText {
    old: Box<dyn Cost>,
    new: Option<Box<dyn Cost>>,
}

impl FinalCostChangeText {
    pub fn new(old: Box<dyn Cost>, new: Option<Box<dyn Cost>>) -> Self {
        Self { old, new }
    }
}

impl CommentText for FinalCostChangeText {
    fn text(&self) -> String {
        match &self.new {
            Some(new) => format!(
                "Окончательная оплата изменена: {} ₽ → {} ₽",
                FormattedCost(self.old.as_ref()),
                FormattedCost(new.as_ref())
            ),
            None => format!(
                "Окончательная оплата удалена: {} ₽",
                FormattedCost(self.old.as_ref())
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn formats_final_cost_change_and_removal() {
        let change = FinalCostChangeText::new(100000.into(), Some(250000.into()));
        assert_eq!(
            change.text(),
            "Окончательная оплата изменена: 100 000 ₽ → 250 000 ₽"
        );

        let removal = FinalCostChangeText::new(100000.into(), None);
        assert_eq!(removal.text(), "Окончательная оплата удалена: 100 000 ₽");

        let kopecks_change = FinalCostChangeText::new(
            Decimal::from_str("100000.5").unwrap().into(),
            Some(Decimal::from_str("250000.75").unwrap().into()),
        );
        assert_eq!(
            kopecks_change.text(),
            "Окончательная оплата изменена: 100 000,50 ₽ → 250 000,75 ₽"
        );
    }
}
