use crate::model::task::project::comment_text::CommentText;

pub struct ActUploadText {
    filename: String,
}

impl ActUploadText {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }
}

impl CommentText for ActUploadText {
    fn text(&self) -> String {
        format!("Загружен акт: {}", self.filename)
    }
}