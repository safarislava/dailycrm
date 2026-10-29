use crate::model::project::stage::cost::formatted_cost::FormattedCost;
use crate::model::project::stage::cost::valid_cost::ValidCost;

pub trait PaymentCostText: Send + Sync {
    fn text(&self, old: ValidCost, new: Option<ValidCost>) -> String;
}

pub trait PaymentConfirmationText: Send + Sync {
    fn text(&self, confirmed: bool) -> String;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AdvanceCostText;

impl PaymentCostText for AdvanceCostText {
    fn text(&self, old: ValidCost, new: Option<ValidCost>) -> String {
        match new {
            Some(new) => format!(
                "Аванс изменён: {} ₽ → {} ₽",
                FormattedCost(old),
                FormattedCost(new)
            ),
            None => format!("Аванс удалён: {} ₽", FormattedCost(old)),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AdvanceConfirmationText;

impl PaymentConfirmationText for AdvanceConfirmationText {
    fn text(&self, confirmed: bool) -> String {
        if confirmed {
            "Аванс подтверждён".to_string()
        } else {
            "Подтверждение аванса снято".to_string()
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SecondAdvanceCostText;

impl PaymentCostText for SecondAdvanceCostText {
    fn text(&self, old: ValidCost, new: Option<ValidCost>) -> String {
        match new {
            Some(new) => format!(
                "Второй аванс изменён: {} ₽ → {} ₽",
                FormattedCost(old),
                FormattedCost(new)
            ),
            None => format!("Второй аванс удалён: {} ₽", FormattedCost(old)),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SecondAdvanceConfirmationText;

impl PaymentConfirmationText for SecondAdvanceConfirmationText {
    fn text(&self, confirmed: bool) -> String {
        if confirmed {
            "Второй аванс подтверждён".to_string()
        } else {
            "Подтверждение второго аванса снято".to_string()
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FinalCostText;

impl PaymentCostText for FinalCostText {
    fn text(&self, old: ValidCost, new: Option<ValidCost>) -> String {
        match new {
            Some(new) => format!(
                "Окончательная оплата изменена: {} ₽ → {} ₽",
                FormattedCost(old),
                FormattedCost(new)
            ),
            None => format!("Окончательная оплата удалена: {} ₽", FormattedCost(old)),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FinalConfirmationText;

impl PaymentConfirmationText for FinalConfirmationText {
    fn text(&self, confirmed: bool) -> String {
        if confirmed {
            "Окончательная оплата подтверждена".to_string()
        } else {
            "Подтверждение окончательной оплаты снято".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn formats_advance_cost_change_and_removal() {
        let formatter = AdvanceCostText;
        assert_eq!(
            formatter.text(100000.into(), Some(250000.into())),
            "Аванс изменён: 100 000 ₽ → 250 000 ₽"
        );
        assert_eq!(
            formatter.text(100000.into(), None),
            "Аванс удалён: 100 000 ₽"
        );
        let kopecks_change = formatter.text(
            Decimal::from_str("100000.5").unwrap().into(),
            Some(Decimal::from_str("250000.75").unwrap().into()),
        );
        assert_eq!(kopecks_change, "Аванс изменён: 100 000,50 ₽ → 250 000,75 ₽");
    }

    #[test]
    fn formats_advance_payment_confirmation() {
        let formatter = AdvanceConfirmationText;
        assert_eq!(formatter.text(true), "Аванс подтверждён");
        assert_eq!(formatter.text(false), "Подтверждение аванса снято");
    }

    #[test]
    fn formats_second_advance_cost_change_and_removal() {
        let formatter = SecondAdvanceCostText;
        assert_eq!(
            formatter.text(100000.into(), Some(250000.into())),
            "Второй аванс изменён: 100 000 ₽ → 250 000 ₽"
        );
        assert_eq!(
            formatter.text(100000.into(), None),
            "Второй аванс удалён: 100 000 ₽"
        );
        let kopecks_change = formatter.text(
            Decimal::from_str("100000.5").unwrap().into(),
            Some(Decimal::from_str("250000.75").unwrap().into()),
        );
        assert_eq!(
            kopecks_change,
            "Второй аванс изменён: 100 000,50 ₽ → 250 000,75 ₽"
        );
    }

    #[test]
    fn formats_second_advance_payment_confirmation() {
        let formatter = SecondAdvanceConfirmationText;
        assert_eq!(formatter.text(true), "Второй аванс подтверждён");
        assert_eq!(formatter.text(false), "Подтверждение второго аванса снято");
    }

    #[test]
    fn formats_final_cost_change_and_removal() {
        let formatter = FinalCostText;
        assert_eq!(
            formatter.text(100000.into(), Some(250000.into())),
            "Окончательная оплата изменена: 100 000 ₽ → 250 000 ₽"
        );
        assert_eq!(
            formatter.text(100000.into(), None),
            "Окончательная оплата удалена: 100 000 ₽"
        );
        let kopecks_change = formatter.text(
            Decimal::from_str("100000.5").unwrap().into(),
            Some(Decimal::from_str("250000.75").unwrap().into()),
        );
        assert_eq!(
            kopecks_change,
            "Окончательная оплата изменена: 100 000,50 ₽ → 250 000,75 ₽"
        );
    }

    #[test]
    fn formats_final_payment_confirmation() {
        let formatter = FinalConfirmationText;
        assert_eq!(formatter.text(true), "Окончательная оплата подтверждена");
        assert_eq!(
            formatter.text(false),
            "Подтверждение окончательной оплаты снято"
        );
    }
}
