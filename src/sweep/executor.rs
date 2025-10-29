//! Parameter sweep executor

use crate::utils::Result;

/// Parameter sweep configuration
pub struct ParameterSweep {
    batch_size: usize,
}

impl ParameterSweep {
    pub fn new() -> Self {
        Self { batch_size: 256 }
    }
}

impl Default for ParameterSweep {
    fn default() -> Self {
        Self::new()
    }
}
