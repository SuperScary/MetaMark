//! Error types for the MetaMark parser.
//!
//! This module defines the error types that can occur during parsing and processing
//! of MetaMark documents, including lexer errors, parser errors, and metadata errors.

use thiserror::Error;

/// Errors that can occur while processing MetaMark documents.
#[derive(Error, Debug)]
pub enum MetaMarkError {
    /// Error during lexical analysis
    #[error("Lexer error at line {line}, column {column}: {message}")]
    LexerError {
        /// Line number where the error occurred (1-based)
        line: usize,
        /// Column number where the error occurred (1-based)
        column: usize,
        /// Description of the error
        message: String,
    },

    /// Error during parsing
    #[error("Parser error at line {line}, column {column}: {message}")]
    ParserError {
        /// Line number where the error occurred (1-based)
        line: usize,
        /// Column number where the error occurred (1-based)
        column: usize,
        /// Description of the error
        message: String,
    },

    /// Error parsing metadata section
    #[error("Invalid metadata: {0}")]
    MetadataError(String),

    /// Error in component block
    #[error("Invalid component block: {0}")]
    ComponentError(String),

    /// I/O error during file operations
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error parsing YAML metadata
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// Error parsing TOML metadata
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
}

/// Result type for MetaMark operations that can fail.
pub type MetaMarkResult<T> = Result<T, MetaMarkError>; 