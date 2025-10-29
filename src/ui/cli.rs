//! Command-line interface

/// CLI application state
pub struct Cli;

impl Cli {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}
