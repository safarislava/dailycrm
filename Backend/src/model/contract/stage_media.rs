use crate::model::project::stage::cost::payment::StagePayment;
use crate::model::project::stage::id::StageId;
use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug, Default)]
pub struct StageStatus {
    pub deadline: Option<DateTime<Utc>>,
    pub completed: bool,
    pub gip_confirmed: bool,
    pub has_act: bool,
}

pub trait StageMedia: Send + Sync + 'static {
    fn add_stage(
        &mut self,
        stage_id: StageId,
        title: &str,
        status: StageStatus,
        payments: Vec<StagePayment>,
    );
}
