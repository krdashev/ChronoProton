//! Driven two-level system (Rabi oscillations)

use ndarray::Array2;
use num_complex::Complex64;
use crate::core::Hamiltonian;

/// Driven two-level system Hamiltonian
///
/// H(t) = ω₀/2 σz + Ω(t) σx
///
/// where Ω(t) = Ω_R cos(ωd t + φ)
///
/// # Parameters
/// - `omega_0`: Transition frequency
/// - `omega_d`: Drive frequency
/// - `rabi_freq`: Rabi frequency (drive amplitude)
/// - `phase`: Drive phase
pub struct DrivenTLS {
    pub omega_0: f64,
    pub omega_d: f64,
    pub rabi_freq: f64,
    pub phase: f64,
}

impl DrivenTLS {
    /// Create a new driven two-level system
    pub fn new(omega_0: f64, omega_d: f64, rabi_freq: f64) -> Self {
        Self {
            omega_0,
            omega_d,
            rabi_freq,
            phase: 0.0,
        }
    }

    /// Create with specified phase
    pub fn with_phase(omega_0: f64, omega_d: f64, rabi_freq: f64, phase: f64) -> Self {
        Self {
            omega_0,
            omega_d,
            rabi_freq,
            phase,
        }
    }

    /// Get the detuning Δ = ω₀ - ωd
    pub fn detuning(&self) -> f64 {
        self.omega_0 - self.omega_d
    }
}

impl Hamiltonian for DrivenTLS {
    fn dim(&self) -> usize {
        2
    }

    fn compute(&self, t: f64, out: &mut Array2<Complex64>) {
        let omega_eff = self.rabi_freq * (self.omega_d * t + self.phase).cos();

        // H = ω₀/2 σz + Ω(t) σx
        out[[0, 0]] = Complex64::new(self.omega_0 / 2.0, 0.0);
        out[[1, 1]] = Complex64::new(-self.omega_0 / 2.0, 0.0);
        out[[0, 1]] = Complex64::new(omega_eff, 0.0);
        out[[1, 0]] = Complex64::new(omega_eff, 0.0);
    }

    fn period(&self) -> Option<f64> {
        Some(2.0 * std::f64::consts::PI / self.omega_d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::math::is_hermitian;

    #[test]
    fn test_driven_tls() {
        let tls = DrivenTLS::new(5.0, 5.0, 0.5);
        assert_eq!(tls.dim(), 2);
        assert_eq!(tls.detuning(), 0.0);

        let mut h = Array2::zeros((2, 2));
        tls.compute(0.0, &mut h);

        assert!(is_hermitian(&h.view(), 1e-10));
    }
}
