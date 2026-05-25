use uuid::Uuid;

pub struct HashPassword(ValidPassword);

impl HashPassword {
    pub fn new(password: ValidPassword) -> Self {
        Self(password)
    }

    pub async fn hash(&self) -> Option<String> {
        let p = self.0.as_str().to_string();
        match actix_web::rt::task::spawn_blocking(move || bcrypt::hash(p, bcrypt::DEFAULT_COST))
            .await
        {
            Ok(Ok(hash)) => Some(hash),
            _ => None,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct User {
    id: Uuid,
    username: String,
    password_hash: String,
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
        // bcrypt silently truncates at 72 bytes
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