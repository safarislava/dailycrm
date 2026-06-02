use crate::model::task::project::comment_text::CommentText;
use std::fmt;

pub struct CostChangeText {
    old: i32,
    new: Option<i32>,
}

impl CostChangeText {
    pub fn new(old: i32, new: Option<i32>) -> Self {
        Self { old, new }
    }
}

impl CommentText for CostChangeText {
    fn text(&self) -> String {
        match self.new {
            Some(new) => format!(
                "Стоимость изменена: {} ₽ → {} ₽",
                FormattedCost(self.old),
                FormattedCost(new)
            ),
            None => format!("Стоимость удалена: {} ₽", FormattedCost(self.old)),
        }
    }
}

struct FormattedCost(i32);

impl fmt::Display for FormattedCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.to_string();
        let bytes = s.as_bytes();
        let len = bytes.len();
        for (i, &b) in bytes.iter().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", b as char)?;
        }
        Ok(())
    }
}