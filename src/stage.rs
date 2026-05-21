use chrono::{DateTime, Local};

pub struct Stage {
    title: String,
    description: String,
    deadline: DateTime<Local>,
    cost: u64,
}