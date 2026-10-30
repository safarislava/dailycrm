use crate::model::project::stage::cost::tranche::StagePaymentTranche;
use crate::model::project::stage::cost::valid_cost::ValidCost;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct StagePayment {
    pub kind: String,
    pub cost: Option<ValidCost>,
    pub confirmed: bool,
    #[serde(default)]
    pub tranches: Vec<StagePaymentTranche>,
}

impl StagePayment {
    pub fn new(kind: impl Into<String>, cost: Option<ValidCost>, confirmed: bool) -> Self {
        Self {
            kind: kind.into(),
            cost,
            confirmed,
            tranches: Vec::new(),
        }
    }

    pub fn with_tranches(
        kind: impl Into<String>,
        cost: Option<ValidCost>,
        confirmed: bool,
        tranches: Vec<StagePaymentTranche>,
    ) -> Self {
        Self {
            kind: kind.into(),
            cost,
            confirmed,
            tranches,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_serializes_stage_payment() {
        let payment = StagePayment::new("first_advance", Some(100.into()), true);
        assert_eq!(
            serde_json::to_string(&payment).unwrap(),
            r#"{"kind":"first_advance","cost":"100","confirmed":true,"tranches":[]}"#
        );
    }
}
