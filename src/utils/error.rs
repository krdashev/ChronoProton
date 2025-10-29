//! Error types for ChronoPhoton

use thiserror::Error;

/// Result type alias using ChronoPhoton error type
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for ChronoPhoton
#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Hamiltonian error: {0}")]
    Hamiltonian(String),

    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("GPU error: {0}")]
    Gpu(String),

    #[error("Integration error: {0}")]
    Integration(String),

    #[error("Numerical error: {0}")]
    Numerical(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    /// Create a configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Error::Config(msg.into())
    }

    /// Create a GPU error
    pub fn gpu(msg: impl Into<String>) -> Self {
        Error::Gpu(msg.into())
    }

    /// Create a numerical error
    pub fn numerical(msg: impl Into<String>) -> Self {
        Error::Numerical(msg.into())
    }

    /// Create a dimension mismatch error
    pub fn dimension_mismatch(expected: usize, actual: usize) -> Self {
        Error::DimensionMismatch { expected, actual }
    }
}
