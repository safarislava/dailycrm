use crate::model::task::project::comment_text::CommentText;

pub struct RenameText {
    old: String,
    new: String,
}

impl RenameText {
    pub fn new(old: String, new: String) -> Self {
        Self { old, new }
    }
}

impl CommentText for RenameText {
    fn text(&self) -> String {
        format!("Название изменено: «{}» → «{}»", self.old, self.new)
    }
}