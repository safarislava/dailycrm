use crate::model::task::project::comment_text::CommentText;

pub struct AttachmentUploadText {
    filename: String,
}

impl AttachmentUploadText {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }
}

impl CommentText for AttachmentUploadText {
    fn text(&self) -> String {
        format!("Загружен файл: {}", self.filename)
    }
}