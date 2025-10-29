//! Simulation checkpoint management

use crate::utils::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Simulation checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub time: f64,
    pub step: usize,
    // TODO: Add state data
}

impl Checkpoint {
    /// Save checkpoint to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let data = bincode::serialize(self)
            .map_err(|e| crate::utils::Error::Serialization(e.to_string()))?;
        std::fs::write(path, data)?;
        Ok(())
    }

    /// Load checkpoint from file
    pub fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read(path)?;
        let checkpoint = bincode::deserialize(&data)
            .map_err(|e| crate::utils::Error::Serialization(e.to_string()))?;
        Ok(checkpoint)
    }
}
