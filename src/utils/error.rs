
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

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

    pub fn config(msg: impl Into<String>) -> Self {
        Error::Config(msg.into())
    }

    pub fn gpu(msg: impl Into<String>) -> Self {
        Error::Gpu(msg.into())
    }

    pub fn numerical(msg: impl Into<String>) -> Self {
        Error::Numerical(msg.into())
    }

    pub fn dimension_mismatch(expected: usize, actual: usize) -> Self {
        Error::DimensionMismatch { expected, actual }
    }
}
