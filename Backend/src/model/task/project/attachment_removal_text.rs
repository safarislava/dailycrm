use crate::model::task::project::comment_text::CommentText;

pub struct AttachmentRemovalText {
    filename: String,
    is_act: bool,
}

impl AttachmentRemovalText {
    pub fn new(filename: String, is_act: bool) -> Self {
        Self { filename, is_act }
    }
}

impl CommentText for AttachmentRemovalText {
    fn text(&self) -> String {
        if self.is_act {
            format!("Удалён акт: {}", self.filename)
        } else {
            format!("Удалён файл: {}", self.filename)
        }
    }
}