use crate::model::task::project::comment_text::CommentText;

pub struct FinalPaymentConfirmationText {
    confirmed: bool,
}

impl FinalPaymentConfirmationText {
    pub fn new(confirmed: bool) -> Self {
        Self { confirmed }
    }
}

impl CommentText for FinalPaymentConfirmationText {
    fn text(&self) -> String {
        if self.confirmed {
            "Окончательная оплата подтверждена".to_string()
        } else {
            "Подтверждение окончательной оплаты снято".to_string()
        }
    }
}