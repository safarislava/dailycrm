pub trait Username: Send + Sync + 'static {
    fn value(&self) -> Result<String, UsernameError>;
}

#[derive(Debug)]
pub enum UsernameError {
    TooShort,
    TooLong,
    InvalidChars,
}

impl std::fmt::Display for UsernameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            Self::TooShort | Self::TooLong => "Username must be 3–50 characters",
            Self::InvalidChars => "Username may only contain letters, digits, spaces, _ or -",
        })
    }
}

impl std::error::Error for UsernameError {}
