use crate::common::BoxError;
use crate::model::notification::burning_deadline::BurningDeadline;
use crate::model::notification::contract::digest::Digest;
use crate::model::notification::contract::message::Message;

pub struct DeadlineDigest {
    deadlines: Vec<BurningDeadline>,
}

impl DeadlineDigest {
    pub fn new(deadlines: Vec<BurningDeadline>) -> Self {
        Self { deadlines }
    }
}

#[async_trait::async_trait]
impl Message for DeadlineDigest {
    async fn text(&self) -> Result<String, BoxError> {
        let mut body = String::from("Дедлайны, которые сгорают завтра:\n\n");
        for deadline in &self.deadlines {
            body.push_str(&deadline.text().await?);
            body.push('\n');
        }
        Ok(body)
    }
}

#[async_trait::async_trait]
impl Digest for DeadlineDigest {
    fn is_empty(&self) -> bool {
        self.deadlines.is_empty()
    }
}