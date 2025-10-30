
use crate::utils::Result;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Checkpoint {
    pub time: f64,
    pub step: usize,

}

impl Checkpoint {

    pub fn save(&self, path: &Path) -> Result<()> {
        let config = bincode::config::standard();
        let data = bincode::encode_to_vec(self, config)
            .map_err(|e| crate::utils::Error::Serialization(e.to_string()))?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read(path)?;
        let config = bincode::config::standard();
        let (checkpoint, _) = bincode::decode_from_slice(&data, config)
            .map_err(|e| crate::utils::Error::Serialization(e.to_string()))?;
        Ok(checkpoint)
    }
}
