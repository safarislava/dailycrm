pub trait Password: Send + Sync + 'static {
    fn value(&self) -> Result<String, PasswordError>;
}

#[derive(Debug)]
pub enum PasswordError {
    TooShort,
    TooLong,
}

impl std::fmt::Display for PasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::TooShort | Self::TooLong => "Password must be 6–72 characters",
        })
    }
}

impl std::error::Error for PasswordError {}
