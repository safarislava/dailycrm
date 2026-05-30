use crate::common::BoxError;
use crate::model::schedule::contract::event::Event;
use chrono::{Duration, NaiveTime, Utc};

pub struct TimeOfDay {
    time: NaiveTime,
}

impl TimeOfDay {
    pub fn new(time: NaiveTime) -> Self {
        Self { time }
    }
}

#[async_trait::async_trait]
impl Event for TimeOfDay {
    async fn fired(&self) -> Result<(), BoxError> {
        let now = Utc::now();
        let today = now.date_naive().and_time(self.time).and_utc();
        let target = if today > now {
            today
        } else {
            today + Duration::days(1)
        };
        actix_web::rt::time::sleep((target - now).to_std()?).await;
        Ok(())
    }
}
