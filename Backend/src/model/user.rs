use uuid::Uuid;

pub struct PasswordHash(String);

pub enum HashError {
    Bcrypt,
    Task,
}

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        PasswordHash(hash)
    }

    pub async fn new_from_password(password: ValidPassword) -> Result<Self, HashError> {
        match actix_web::rt::task::spawn_blocking(move || {
            bcrypt::hash(password.as_str(), bcrypt::DEFAULT_COST)
        })
        .await
        {
            Ok(Ok(hash)) => Ok(Self(hash)),
            Ok(Err(_)) => Err(HashError::Bcrypt),
            Err(_) => Err(HashError::Task),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[allow(dead_code)]
pub struct ValidPasswordHash(PasswordHash);

pub enum VerifyError {
    WrongPassword,
    Internal,
}

impl ValidPasswordHash {
    pub async fn try_new(hash: PasswordHash, password: &str) -> Result<Self, VerifyError> {
        let raw = hash.0.clone();
        let password = password.to_owned();
        match actix_web::rt::task::spawn_blocking(move || bcrypt::verify(&password, &raw)).await {
            Ok(Ok(true)) => Ok(Self(hash)),
            Ok(Ok(false)) => Err(VerifyError::WrongPassword),
            _ => Err(VerifyError::Internal),
        }
    }
}

#[allow(dead_code)]
pub struct User {
    id: Uuid,
    username: ValidUsername,
    password_hash: PasswordHash,
}

pub struct Username(pub String);
pub struct Password(pub String);

pub struct ValidUsername(Username);
pub struct ValidPassword(Password);

pub enum UsernameError {
    TooShort,
    TooLong,
    InvalidChars,
}

pub enum PasswordError {
    TooShort,
    TooLong,
}

impl ValidUsername {
    pub fn try_new(username: Username) -> Result<Self, UsernameError> {
        let len = username.0.len();
        if len < 3 {
            return Err(UsernameError::TooShort);
        }
        if len > 50 {
            return Err(UsernameError::TooLong);
        }
        if !username.0.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            return Err(UsernameError::InvalidChars);
        }
        Ok(Self(username))
    }

    pub fn as_str(&self) -> &str {
        &self.0.0
    }
}

impl ValidPassword {
    pub fn try_new(password: Password) -> Result<Self, PasswordError> {
        let len = password.0.len();
        if len < 8 {
            return Err(PasswordError::TooShort);
        }
        if len > 72 {
            return Err(PasswordError::TooLong);
        }
        Ok(Self(password))
    }

    pub fn as_str(&self) -> &str {
        &self.0.0
    }
}

impl UsernameError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::TooShort | Self::TooLong => "Username must be 3–50 characters",
            Self::InvalidChars => "Username may only contain letters, digits, _ or -",
        }
    }
}

impl PasswordError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::TooShort | Self::TooLong => "Password must be 8–72 characters",
        }
    }
}