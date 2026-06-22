use crate::model::task::project::comment_text::CommentText;

pub struct AdvancePaymentConfirmationText {
    confirmed: bool,
}

impl AdvancePaymentConfirmationText {
    pub fn new(confirmed: bool) -> Self {
        Self { confirmed }
    }
}

impl CommentText for AdvancePaymentConfirmationText {
    fn text(&self) -> String {
        if self.confirmed {
            "Аванс подтверждён".to_string()
        } else {
            "Подтверждение аванса снято".to_string()
        }
    }
}