
use crate::utils::Result;
use ndarray::Array2;
use num_complex::Complex64;

pub struct MatMulKernel;

impl MatMulKernel {
    pub fn execute(
        _a: &Array2<Complex64>,
        _b: &Array2<Complex64>,
    ) -> Result<Array2<Complex64>> {

        Err(crate::utils::Error::NotImplemented(
            "GPU matrix multiplication".to_string(),
        ))
    }
}

pub struct EvolveKernel;

impl EvolveKernel {
    pub fn execute_batch(
        _states: &[Array2<Complex64>],
        _hamiltonians: &[Array2<Complex64>],
        _dt: f64,
    ) -> Result<Vec<Array2<Complex64>>> {

        Err(crate::utils::Error::NotImplemented(
            "GPU batched evolution".to_string(),
        ))
    }
}
