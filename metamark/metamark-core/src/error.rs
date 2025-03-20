use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parser error: {0}")]
    Parser(String),

    #[error("Lexer error: {0}")]
    Lexer(String),

    #[error("AST error: {0}")]
    Ast(String),

    #[error("Security error: {0}")]
    Security(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Version control error: {0}")]
    VersionControl(String),

    #[error("Invalid document format: {0}")]
    InvalidFormat(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Error {
    pub fn parser<T: ToString>(msg: T) -> Self {
        Self::Parser(msg.to_string())
    }

    pub fn lexer<T: ToString>(msg: T) -> Self {
        Self::Lexer(msg.to_string())
    }

    pub fn ast<T: ToString>(msg: T) -> Self {
        Self::Ast(msg.to_string())
    }

    pub fn security<T: ToString>(msg: T) -> Self {
        Self::Security(msg.to_string())
    }

    pub fn serialization<T: ToString>(msg: T) -> Self {
        Self::Serialization(msg.to_string())
    }
} 