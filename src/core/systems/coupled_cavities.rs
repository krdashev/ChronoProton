//! Coupled cavity arrays (SSH model and generalizations)

use ndarray::Array2;
use num_complex::Complex64;
use crate::core::Hamiltonian;
use crate::utils::Result;

/// Coupled cavity array Hamiltonian
///
/// H = Σ_i ωc a†_i a_i + Σ_<i,j> J_ij (a†_i a_j + h.c.)
pub struct CoupledCavities {
    pub omega_c: f64,
    pub couplings: Vec<f64>,
    num_cavities: usize,
}

impl CoupledCavities {
    /// Create SSH-type coupled cavities (alternating couplings)
    pub fn ssh(omega_c: f64, j1: f64, j2: f64, num_cavities: usize) -> Self {
        let mut couplings = Vec::new();
        for i in 0..num_cavities - 1 {
            couplings.push(if i % 2 == 0 { j1 } else { j2 });
        }

        Self {
            omega_c,
            couplings,
            num_cavities,
        }
    }

    /// Create uniform coupling
    pub fn uniform(omega_c: f64, j: f64, num_cavities: usize) -> Self {
        Self {
            omega_c,
            couplings: vec![j; num_cavities - 1],
            num_cavities,
        }
    }
}

impl Hamiltonian for CoupledCavities {
    fn dim(&self) -> usize {
        // Simplified: single excitation subspace
        self.num_cavities + 1
    }

    fn compute(&self, _t: f64, out: &mut Array2<Complex64>) {
        out.fill(Complex64::new(0.0, 0.0));

        // Ground state
        out[[0, 0]] = Complex64::new(0.0, 0.0);

        // Single excitation states
        for i in 1..=self.num_cavities {
            out[[i, i]] = Complex64::new(self.omega_c, 0.0);
        }

        // Couplings
        for (idx, &j) in self.couplings.iter().enumerate() {
            let i = idx + 1;
            let j_next = i + 1;
            if j_next <= self.num_cavities {
                out[[i, j_next]] = Complex64::new(j, 0.0);
                out[[j_next, i]] = Complex64::new(j, 0.0);
            }
        }
    }

    fn is_time_independent(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_cavities() {
        let ssh = CoupledCavities::ssh(5.0, 1.0, 0.5, 4);
        assert_eq!(ssh.dim(), 5);
        assert_eq!(ssh.couplings.len(), 3);
    }
}
