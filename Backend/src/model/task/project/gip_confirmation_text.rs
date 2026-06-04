use crate::model::task::project::comment_text::CommentText;

pub struct GipConfirmationText {
    confirmed: bool,
}

impl GipConfirmationText {
    pub fn new(confirmed: bool) -> Self {
        Self { confirmed }
    }
}

impl CommentText for GipConfirmationText {
    fn text(&self) -> String {
        if self.confirmed {
            "ГИП подтвердил выполнение".to_string()
        } else {
            "ГИП снял подтверждение выполнения".to_string()
        }
    }
}