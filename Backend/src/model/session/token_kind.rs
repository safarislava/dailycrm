#[derive(Clone, Copy)]
pub enum TokenKind {
    Access,
    Refresh,
}

impl TokenKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenKind::Access => "access",
            TokenKind::Refresh => "refresh",
        }
    }
}
