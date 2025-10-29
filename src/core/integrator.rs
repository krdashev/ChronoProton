//! Time evolution integrators for quantum systems

use ndarray::Array2;
use num_complex::Complex64;
use crate::core::{Hamiltonian, QuantumState};
use crate::utils::Result;

/// Types of integrators available
#[derive(Debug, Clone, Copy)]
pub enum IntegratorType {
    /// 4th order Runge-Kutta
    RK4,
    /// 2nd order Magnus expansion
    Magnus2,
    /// 4th order Magnus expansion
    Magnus4,
}

/// Trait for time evolution integrators
pub trait Integrator: Send + Sync {
    /// Perform one integration step
    fn step(
        &self,
        hamiltonian: &dyn Hamiltonian,
        state: &mut QuantumState,
        t: f64,
        dt: f64,
    ) -> Result<()>;

    /// Get the integrator type
    fn integrator_type(&self) -> IntegratorType;
}

/// Runge-Kutta 4th order integrator
pub struct RK4Integrator;

impl RK4Integrator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RK4Integrator {
    fn default() -> Self {
        Self::new()
    }
}

impl Integrator for RK4Integrator {
    fn step(
        &self,
        hamiltonian: &dyn Hamiltonian,
        state: &mut QuantumState,
        t: f64,
        dt: f64,
    ) -> Result<()> {
        let dim = hamiltonian.dim();
        let mut h = Array2::zeros((dim, dim));

        // k1 = -i H(t) |ψ⟩
        hamiltonian.compute(t, &mut h);
        let k1 = apply_hamiltonian(&h, state);

        // k2 = -i H(t + dt/2) |ψ + k1*dt/2⟩
        let mut temp_state = state.clone();
        add_scaled_to_state(&mut temp_state, &k1, dt / 2.0);
        hamiltonian.compute(t + dt / 2.0, &mut h);
        let k2 = apply_hamiltonian(&h, &temp_state);

        // k3 = -i H(t + dt/2) |ψ + k2*dt/2⟩
        let mut temp_state = state.clone();
        add_scaled_to_state(&mut temp_state, &k2, dt / 2.0);
        hamiltonian.compute(t + dt / 2.0, &mut h);
        let k3 = apply_hamiltonian(&h, &temp_state);

        // k4 = -i H(t + dt) |ψ + k3*dt⟩
        let mut temp_state = state.clone();
        add_scaled_to_state(&mut temp_state, &k3, dt);
        hamiltonian.compute(t + dt, &mut h);
        let k4 = apply_hamiltonian(&h, &temp_state);

        // |ψ⟩ += dt/6 * (k1 + 2*k2 + 2*k3 + k4)
        let data = state.data().to_owned();
        let new_data = &data
            + &(dt / 6.0 * (&k1 + &(2.0 * &k2) + &(2.0 * &k3) + &k4));

        // Renormalize
        let norm: f64 = new_data.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt();
        let normalized = new_data.mapv(|x| x / norm);

        *state = QuantumState::new(normalized)?;

        Ok(())
    }

    fn integrator_type(&self) -> IntegratorType {
        IntegratorType::RK4
    }
}

/// Helper function to apply Hamiltonian: -i H |ψ⟩
fn apply_hamiltonian(h: &Array2<Complex64>, state: &QuantumState) -> ndarray::Array1<Complex64> {
    let dim = h.nrows();
    let psi = state.data();
    let mut result = ndarray::Array1::zeros(dim);

    let i = Complex64::new(0.0, 1.0);

    for row in 0..dim {
        let mut sum = Complex64::new(0.0, 0.0);
        for col in 0..dim {
            sum += h[[row, col]] * psi[col];
        }
        result[row] = -i * sum;
    }

    result
}

/// Helper function to add scaled vector to state
fn add_scaled_to_state(
    state: &mut QuantumState,
    delta: &ndarray::Array1<Complex64>,
    scale: f64,
) {
    let data = state.data().to_owned() + &(scale * delta);
    *state = QuantumState::new(data).unwrap();
}

/// Create an integrator of the specified type
pub fn create_integrator(integrator_type: IntegratorType) -> Box<dyn Integrator> {
    match integrator_type {
        IntegratorType::RK4 => Box::new(RK4Integrator::new()),
        IntegratorType::Magnus2 | IntegratorType::Magnus4 => {
            // TODO: Implement Magnus integrators
            Box::new(RK4Integrator::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::hamiltonian::TimeIndependentHamiltonian;
    use approx::assert_relative_eq;

    #[test]
    fn test_rk4_conserves_norm() {
        // Simple 2-level Hamiltonian
        let mut h = Array2::zeros((2, 2));
        h[[0, 1]] = Complex64::new(1.0, 0.0);
        h[[1, 0]] = Complex64::new(1.0, 0.0);

        let hamiltonian = TimeIndependentHamiltonian::new(h);
        let mut state = QuantumState::ground_state(2);

        let integrator = RK4Integrator::new();
        integrator.step(&hamiltonian, &mut state, 0.0, 0.01).unwrap();

        let norm_sq: f64 = state.data().iter().map(|x| x.norm_sqr()).sum();
        assert_relative_eq!(norm_sq, 1.0, epsilon = 1e-10);
    }
}
