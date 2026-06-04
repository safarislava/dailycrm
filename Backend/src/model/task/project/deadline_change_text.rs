use crate::model::task::project::comment_text::CommentText;
use chrono::{DateTime, Utc};

pub struct DeadlineChangeText {
    old: DateTime<Utc>,
    new: Option<DateTime<Utc>>,
}

impl DeadlineChangeText {
    pub fn new(old: DateTime<Utc>, new: Option<DateTime<Utc>>) -> Self {
        Self { old, new }
    }
}

impl CommentText for DeadlineChangeText {
    fn text(&self) -> String {
        match self.new {
            Some(new) => format!(
                "Дедлайн изменён: {} → {}",
                self.old.format("%d.%m.%Y"),
                new.format("%d.%m.%Y")
            ),
            None => format!("Дедлайн удалён: {}", self.old.format("%d.%m.%Y")),
        }
    }
}