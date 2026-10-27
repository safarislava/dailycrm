use crate::model::project::stage::cost::valid_cost::ValidCost;
use rust_decimal::Decimal;

pub trait Cost: Send + Sync {
    fn as_decimal(&self) -> Result<Decimal, CostError>;
    fn clone_box(&self) -> Box<dyn Cost>;
}

impl Clone for Box<dyn Cost> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Cost for Box<dyn Cost> {
    fn as_decimal(&self) -> Result<Decimal, CostError> {
        (**self).as_decimal()
    }

    fn clone_box(&self) -> Box<dyn Cost> {
        (**self).clone_box()
    }
}

impl From<ValidCost> for Box<dyn Cost> {
    fn from(cost: ValidCost) -> Self {
        Box::new(cost)
    }
}

impl From<Decimal> for Box<dyn Cost> {
    fn from(decimal: Decimal) -> Self {
        Box::new(ValidCost::new(decimal))
    }
}

impl From<i32> for Box<dyn Cost> {
    fn from(n: i32) -> Self {
        Box::new(ValidCost::new(Decimal::from(n)))
    }
}

impl PartialEq for dyn Cost {
    fn eq(&self, other: &Self) -> bool {
        self.as_decimal() == other.as_decimal()
    }
}

impl Eq for dyn Cost {}

impl std::fmt::Debug for dyn Cost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cost({:?})", self.as_decimal())
    }
}

impl serde::Serialize for Box<dyn Cost> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let dec = self.as_decimal().map_err(serde::ser::Error::custom)?;
        serde::Serialize::serialize(&dec, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Box<dyn Cost> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let dec = <Decimal as serde::Deserialize>::deserialize(deserializer)?;
        let valid = ValidCost::new(dec);
        valid.as_decimal().map_err(serde::de::Error::custom)?;
        Ok(valid.into())
    }
}

impl sqlx::Type<sqlx::Postgres> for Box<dyn Cost> {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <Decimal as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Box<dyn Cost> {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let decimal = <Decimal as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(decimal.into())
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Box<dyn Cost> {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let decimal = self.as_decimal()?;
        <Decimal as sqlx::Encode<sqlx::Postgres>>::encode(decimal, buf)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CostError {
    TooManyDecimalPlaces,
}

impl std::fmt::Display for CostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyDecimalPlaces => {
                f.write_str("Cost cannot have more than 2 decimal places")
            }
        }
    }
}

impl std::error::Error for CostError {}
