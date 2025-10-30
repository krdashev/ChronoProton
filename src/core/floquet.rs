use crate::core::Hamiltonian;
use crate::utils::{Error, Result};
use ndarray::Array2;
use num_complex::Complex64;

pub struct FloquetSpectrum {
    pub quasi_energies: Vec<f64>,

    pub modes: Array2<Complex64>,

    pub period: f64,
}

impl FloquetSpectrum {
    pub fn compute(hamiltonian: &dyn Hamiltonian, period: f64, num_steps: usize) -> Result<Self> {
        if !hamiltonian.is_time_independent() && hamiltonian.period().is_none() {
            return Err(Error::InvalidParameter(
                "Hamiltonian must be time-periodic for Floquet analysis".to_string(),
            ));
        }

        Err(Error::NotImplemented(
            "FloquetSpectrum::compute".to_string(),
        ))
    }

    pub fn num_levels(&self) -> usize {
        self.quasi_energies.len()
    }

    pub fn level_spacing(&self, n: usize) -> Option<f64> {
        if n + 1 < self.quasi_energies.len() {
            Some(self.quasi_energies[n + 1] - self.quasi_energies[n])
        } else {
            None
        }
    }
}

pub struct FloquetHamiltonian {
    pub n_fourier: usize,

    hamiltonian: Box<dyn Hamiltonian>,

    pub omega: f64,
}

impl FloquetHamiltonian {
    pub fn new(hamiltonian: Box<dyn Hamiltonian>, omega: f64, n_fourier: usize) -> Self {
        Self {
            n_fourier,
            hamiltonian,
            omega,
        }
    }

    pub fn extended_dim(&self) -> usize {
        self.hamiltonian.dim() * (2 * self.n_fourier + 1)
    }

    pub fn compute_extended(&self) -> Result<Array2<Complex64>> {
        Err(Error::NotImplemented(
            "FloquetHamiltonian::compute_extended".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floquet_hamiltonian_creation() {
        use crate::core::hamiltonian::TimeIndependentHamiltonian;

        let h = Array2::zeros((2, 2));
        let ham = TimeIndependentHamiltonian::new(h);
        let floquet = FloquetHamiltonian::new(Box::new(ham), 1.0, 5);

        assert_eq!(floquet.extended_dim(), 2 * 11);
    }
}
