//! GPU kernel implementations

use crate::utils::Result;
use ndarray::Array2;
use num_complex::Complex64;

/// Matrix multiplication kernel
pub struct MatMulKernel;

impl MatMulKernel {
    pub fn execute(
        _a: &Array2<Complex64>,
        _b: &Array2<Complex64>,
    ) -> Result<Array2<Complex64>> {
        // TODO: Implement GPU matrix multiplication
        Err(crate::utils::Error::NotImplemented(
            "GPU matrix multiplication".to_string(),
        ))
    }
}

/// Time evolution kernel
pub struct EvolveKernel;

impl EvolveKernel {
    pub fn execute_batch(
        _states: &[Array2<Complex64>],
        _hamiltonians: &[Array2<Complex64>],
        _dt: f64,
    ) -> Result<Vec<Array2<Complex64>>> {
        // TODO: Implement batched evolution on GPU
        Err(crate::utils::Error::NotImplemented(
            "GPU batched evolution".to_string(),
        ))
    }
}
