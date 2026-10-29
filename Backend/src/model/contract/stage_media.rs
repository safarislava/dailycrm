use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StageCosts {
    pub advance_cost: Option<ValidCost>,
    pub advance_confirmed: bool,
    pub second_advance_cost: Option<ValidCost>,
    pub second_advance_confirmed: bool,
    pub final_cost: Option<ValidCost>,
    pub final_confirmed: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StageStatus {
    pub deadline: Option<DateTime<Utc>>,
    pub completed: bool,
    pub gip_confirmed: bool,
    pub has_act: bool,
}

pub trait StageMedia: Send + Sync + 'static {
    fn add_stage(&mut self, stage_id: StageId, title: &str, status: StageStatus, costs: StageCosts);
}
