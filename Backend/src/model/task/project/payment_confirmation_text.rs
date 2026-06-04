use crate::model::task::project::comment_text::CommentText;

pub struct PaymentConfirmationText {
    confirmed: bool,
}

impl PaymentConfirmationText {
    pub fn new(confirmed: bool) -> Self {
        Self { confirmed }
    }
}

impl CommentText for PaymentConfirmationText {
    fn text(&self) -> String {
        if self.confirmed {
            "Оплата подтверждена".to_string()
        } else {
            "Подтверждение оплаты снято".to_string()
        }
    }
}