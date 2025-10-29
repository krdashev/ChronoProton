//! Logging configuration for ChronoPhoton

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize the logger with default settings
pub fn init() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("chronophoton=info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();

    Ok(())
}

/// Initialize the logger with a custom filter
pub fn init_with_filter(filter: &str) -> anyhow::Result<()> {
    let filter = EnvFilter::try_new(filter)?;

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();

    Ok(())
}
