use crate::common::BoxError;
use crate::model::schedule::contract::scheduled::Scheduled;
use crate::model::schedule::schedule::Schedule;
use futures_util::future::try_join_all;

pub struct Timetable {
    schedules: Vec<Schedule>,
}

impl Timetable {
    pub fn new(schedules: Vec<Schedule>) -> Self {
        Self { schedules }
    }
}

#[async_trait::async_trait]
impl Scheduled for Timetable {
    async fn run(&self) -> Result<(), BoxError> {
        try_join_all(self.schedules.iter().map(|s| s.run())).await?;
        Ok(())
    }
}
