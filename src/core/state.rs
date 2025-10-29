//! Quantum state representations

use ndarray::{Array1, Array2};
use num_complex::Complex64;
use crate::utils::{Error, Result};

/// Quantum state vector (pure state)
#[derive(Clone, Debug)]
pub struct QuantumState {
    data: Array1<Complex64>,
}

impl QuantumState {
    /// Create a new quantum state from a vector
    pub fn new(data: Array1<Complex64>) -> Result<Self> {
        let norm_sq: f64 = data.iter().map(|x| x.norm_sqr()).sum();
        if (norm_sq - 1.0).abs() > 1e-10 {
            return Err(Error::InvalidParameter(format!(
                "State must be normalized, got norm^2 = {}",
                norm_sq
            )));
        }
        Ok(Self { data })
    }

    /// Create a ground state (all zeros except first element)
    pub fn ground_state(dim: usize) -> Self {
        let mut data = Array1::zeros(dim);
        data[0] = Complex64::new(1.0, 0.0);
        Self { data }
    }

    /// Create a random normalized state
    pub fn random(dim: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut data = Array1::zeros(dim);
        for i in 0..dim {
            data[i] = Complex64::new(rng.gen(), rng.gen());
        }

        // Normalize
        let norm: f64 = data.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt();
        data.mapv_inplace(|x| x / norm);

        Self { data }
    }

    /// Get the dimension
    pub fn dim(&self) -> usize {
        self.data.len()
    }

    /// Get the underlying data
    pub fn data(&self) -> &Array1<Complex64> {
        &self.data
    }

    /// Convert to density matrix
    pub fn to_density_matrix(&self) -> DensityMatrix {
        let dim = self.dim();
        let mut rho = Array2::zeros((dim, dim));

        for i in 0..dim {
            for j in 0..dim {
                rho[[i, j]] = self.data[i] * self.data[j].conj();
            }
        }

        DensityMatrix::new_unchecked(rho)
    }
}

/// Density matrix (mixed state)
#[derive(Clone, Debug)]
pub struct DensityMatrix {
    data: Array2<Complex64>,
}

impl DensityMatrix {
    /// Create a new density matrix with validation
    pub fn new(data: Array2<Complex64>) -> Result<Self> {
        use crate::utils::math::{is_hermitian, trace};

        if data.nrows() != data.ncols() {
            return Err(Error::DimensionMismatch {
                expected: data.nrows(),
                actual: data.ncols(),
            });
        }

        if !is_hermitian(&data.view(), 1e-10) {
            return Err(Error::InvalidParameter(
                "Density matrix must be Hermitian".to_string(),
            ));
        }

        let tr = trace(&data.view());
        if (tr.re - 1.0).abs() > 1e-10 || tr.im.abs() > 1e-10 {
            return Err(Error::InvalidParameter(format!(
                "Density matrix must have trace 1, got {}",
                tr
            )));
        }

        Ok(Self { data })
    }

    /// Create a density matrix without validation (for performance)
    pub fn new_unchecked(data: Array2<Complex64>) -> Self {
        Self { data }
    }

    /// Create a maximally mixed state
    pub fn maximally_mixed(dim: usize) -> Self {
        let mut data = Array2::zeros((dim, dim));
        let weight = 1.0 / dim as f64;

        for i in 0..dim {
            data[[i, i]] = Complex64::new(weight, 0.0);
        }

        Self::new_unchecked(data)
    }

    /// Get the dimension
    pub fn dim(&self) -> usize {
        self.data.nrows()
    }

    /// Get the underlying data
    pub fn data(&self) -> &Array2<Complex64> {
        &self.data
    }

    /// Get mutable access to underlying data
    pub fn data_mut(&mut self) -> &mut Array2<Complex64> {
        &mut self.data
    }

    /// Compute purity Tr(ρ²)
    pub fn purity(&self) -> f64 {
        use crate::utils::math::trace;

        let dim = self.dim();
        let mut rho_sq = Array2::zeros((dim, dim));

        for i in 0..dim {
            for j in 0..dim {
                let mut sum = Complex64::new(0.0, 0.0);
                for k in 0..dim {
                    sum += self.data[[i, k]] * self.data[[k, j]];
                }
                rho_sq[[i, j]] = sum;
            }
        }

        trace(&rho_sq.view()).re
    }

    /// Compute von Neumann entropy -Tr(ρ ln ρ)
    pub fn von_neumann_entropy(&self) -> Result<f64> {
        // TODO: Implement eigenvalue decomposition
        Err(Error::NotImplemented("von_neumann_entropy".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_ground_state() {
        let psi = QuantumState::ground_state(3);
        assert_eq!(psi.dim(), 3);
        assert_relative_eq!(psi.data()[0].norm_sqr(), 1.0);
        assert_relative_eq!(psi.data()[1].norm_sqr(), 0.0);
    }

    #[test]
    fn test_density_matrix_from_pure_state() {
        let psi = QuantumState::ground_state(2);
        let rho = psi.to_density_matrix();

        assert_relative_eq!(rho.data()[[0, 0]].re, 1.0);
        assert_relative_eq!(rho.data()[[1, 1]].re, 0.0);
        assert_relative_eq!(rho.purity(), 1.0);
    }

    #[test]
    fn test_maximally_mixed() {
        let rho = DensityMatrix::maximally_mixed(2);
        assert_relative_eq!(rho.purity(), 0.5);
    }
}
