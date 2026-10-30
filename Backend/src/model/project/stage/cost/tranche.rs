use crate::model::project::stage::cost::valid_cost::ValidCost;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub enum TrancheError {
    CostNotSet,
    OverpaymentForbidden,
    CostBelowPaid,
    TrancheNotFound,
}

impl fmt::Display for TrancheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CostNotSet => f.write_str("Плановая стоимость платежа не задана"),
            Self::OverpaymentForbidden => {
                f.write_str("Сумма частичной оплаты превышает оставшуюся стоимость")
            }
            Self::CostBelowPaid => {
                f.write_str("Плановая стоимость не может быть меньше уже оплаченной суммы")
            }
            Self::TrancheNotFound => f.write_str("Транш не найден"),
        }
    }
}

impl std::error::Error for TrancheError {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct StagePaymentTranche {
    pub id: Uuid,
    pub amount: ValidCost,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

impl StagePaymentTranche {
    pub fn new(
        id: Uuid,
        amount: ValidCost,
        created_at: DateTime<Utc>,
        created_by: Option<Uuid>,
    ) -> Self {
        Self {
            id,
            amount,
            created_at,
            created_by,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_serializes_stage_payment_tranche() {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let tranche = StagePaymentTranche::new(id, 500.into(), now, None);
        assert_eq!(tranche.id, id);
        assert_eq!(tranche.amount, 500.into());
    }

    #[test]
    fn displays_tranche_errors() {
        assert_eq!(
            TrancheError::OverpaymentForbidden.to_string(),
            "Сумма частичной оплаты превышает оставшуюся стоимость"
        );
        assert_eq!(
            TrancheError::CostBelowPaid.to_string(),
            "Плановая стоимость не может быть меньше уже оплаченной суммы"
        );
    }
}
