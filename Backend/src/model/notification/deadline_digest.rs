use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::notification::burning_deadline::BurningDeadline;

pub struct DeadlineDigest {
    deadlines: Vec<BurningDeadline>,
}

impl DeadlineDigest {
    pub fn new(deadlines: Vec<BurningDeadline>) -> Self {
        Self { deadlines }
    }

    pub fn is_empty(&self) -> bool {
        self.deadlines.is_empty()
    }
}

#[async_trait::async_trait]
impl Contentable for DeadlineDigest {
    type Output = String;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        let mut body = String::from("Дедлайны, которые сгорают завтра:\n\n");
        for deadline in &self.deadlines {
            body.push_str(&deadline.content().await?);
            body.push('\n');
        }
        Ok(body)
    }
}
