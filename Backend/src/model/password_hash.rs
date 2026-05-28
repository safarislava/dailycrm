use sqlx::Encode;
use sqlx::Postgres;
use sqlx::Type;
use sqlx::encode::IsNull;
use sqlx::postgres::PgArgumentBuffer;

#[derive(Clone)]
pub struct PasswordHash(String);

pub enum HashError {
    Bcrypt,
    Task,
}

pub enum VerifyError {
    WrongPassword,
    Internal,
}

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        PasswordHash(hash)
    }

    pub async fn verification(&self, password: &str) -> Result<(), VerifyError> {
        let raw = self.0.clone();
        let password = password.to_owned();
        match actix_web::rt::task::spawn_blocking(move || bcrypt::verify(&password, &raw)).await {
            Ok(Ok(true)) => Ok(()),
            Ok(Ok(false)) => Err(VerifyError::WrongPassword),
            _ => Err(VerifyError::Internal),
        }
    }
}

impl Type<Postgres> for PasswordHash {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }
}

impl<'q> Encode<'q, Postgres> for PasswordHash {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <&str as Encode<Postgres>>::encode_by_ref(&&*self.0, buf)
    }
}