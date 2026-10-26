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
