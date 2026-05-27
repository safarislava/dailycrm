use sqlx::Encode;
use sqlx::Postgres;
use sqlx::Type;
use sqlx::encode::IsNull;
use sqlx::postgres::PgArgumentBuffer;

pub struct Username(pub String);

pub struct ValidUsername(Username);

impl ValidUsername {
    pub fn try_new(username: Username) -> Result<Self, UsernameError> {
        let len = username.0.len();
        if len < 3 {
            return Err(UsernameError::TooShort);
        }
        if len > 50 {
            return Err(UsernameError::TooLong);
        }
        if !username
            .0
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(UsernameError::InvalidChars);
        }
        Ok(Self(username))
    }
}

impl Type<Postgres> for ValidUsername {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }
}

impl<'q> Encode<'q, Postgres> for ValidUsername {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <&str as Encode<Postgres>>::encode_by_ref(&&*self.0.0, buf)
    }
}

pub enum UsernameError {
    TooShort,
    TooLong,
    InvalidChars,
}

impl UsernameError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::TooShort | Self::TooLong => "Username must be 3–50 characters",
            Self::InvalidChars => "Username may only contain letters, digits, _ or -",
        }
    }
}
