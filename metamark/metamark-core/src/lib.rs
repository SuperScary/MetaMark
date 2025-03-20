pub mod ast;
pub mod lexer;
pub mod parser;
pub mod security;
pub mod error;
pub mod document;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

/// The MetaMark document format version
pub const VERSION: &str = "1.0.0";

/// Core functionality for MetaMark document processing
#[derive(Debug)]
pub struct MetaMark {
    config: Config,
}

#[derive(Debug, Clone)]
pub struct Config {
    encryption_enabled: bool,
    version_control_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            encryption_enabled: true,
            version_control_enabled: true,
        }
    }
}

impl MetaMark {
    /// Create a new MetaMark instance with default configuration
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// Create a new MetaMark instance with custom configuration
    pub fn with_config(config: Config) -> Self {
        Self { config }
    }
} 