use crate::utils::Result;
use std::path::Path;

pub struct Exporter;

impl Exporter {
    pub fn to_hdf5(_data: &[f64], _path: &Path) -> Result<()> {
        Err(crate::utils::Error::NotImplemented(
            "HDF5 export".to_string(),
        ))
    }

    pub fn to_csv(_data: &[f64], _path: &Path) -> Result<()> {
        Err(crate::utils::Error::NotImplemented(
            "CSV export".to_string(),
        ))
    }
}
