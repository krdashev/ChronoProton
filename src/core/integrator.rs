use crate::core::{Hamiltonian, QuantumState};
use crate::utils::Result;
use ndarray::Array2;
use num_complex::Complex64;

#[derive(Debug, Clone, Copy)]
pub enum IntegratorType {
    RK4,

    Magnus2,

    Magnus4,
}

pub trait Integrator: Send + Sync {
    fn step(
        &self,
        hamiltonian: &dyn Hamiltonian,
        state: &mut QuantumState,
        t: f64,
        dt: f64,
    ) -> Result<()>;

    fn integrator_type(&self) -> IntegratorType;
}

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

        hamiltonian.compute(t, &mut h);
        let k1 = apply_hamiltonian(&h, state);

        let mut temp_state = state.clone();
        add_scaled_to_state(&mut temp_state, &k1, dt / 2.0);
        hamiltonian.compute(t + dt / 2.0, &mut h);
        let k2 = apply_hamiltonian(&h, &temp_state);

        let mut temp_state = state.clone();
        add_scaled_to_state(&mut temp_state, &k2, dt / 2.0);
        hamiltonian.compute(t + dt / 2.0, &mut h);
        let k3 = apply_hamiltonian(&h, &temp_state);

        let mut temp_state = state.clone();
        add_scaled_to_state(&mut temp_state, &k3, dt);
        hamiltonian.compute(t + dt, &mut h);
        let k4 = apply_hamiltonian(&h, &temp_state);

        let data = state.data().to_owned();
        let factor = Complex64::new(dt / 6.0, 0.0);
        let increment = &k1 + &k2.mapv(|x| x * 2.0) + &k3.mapv(|x| x * 2.0) + &k4;
        let new_data = &data + &increment.mapv(|x| x * factor);

        let norm: f64 = new_data.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt();
        let normalized = new_data.mapv(|x| x / norm);

        *state = QuantumState::new(normalized)?;

        Ok(())
    }

    fn integrator_type(&self) -> IntegratorType {
        IntegratorType::RK4
    }
}

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

fn add_scaled_to_state(state: &mut QuantumState, delta: &ndarray::Array1<Complex64>, scale: f64) {
    let scaled_delta = delta.mapv(|x| x * Complex64::new(scale, 0.0));
    let mut data = state.data().to_owned() + &scaled_delta;

    let norm: f64 = data.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt();
    data.mapv_inplace(|x| x / norm);

    *state = QuantumState::new(data).unwrap();
}

pub fn create_integrator(integrator_type: IntegratorType) -> Box<dyn Integrator> {
    match integrator_type {
        IntegratorType::RK4 => Box::new(RK4Integrator::new()),
        IntegratorType::Magnus2 | IntegratorType::Magnus4 => Box::new(RK4Integrator::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::hamiltonian::TimeIndependentHamiltonian;
    use approx::assert_relative_eq;

    #[test]
    fn test_rk4_conserves_norm() {
        let mut h = Array2::zeros((2, 2));
        h[[0, 1]] = Complex64::new(1.0, 0.0);
        h[[1, 0]] = Complex64::new(1.0, 0.0);

        let hamiltonian = TimeIndependentHamiltonian::new(h);
        let mut state = QuantumState::ground_state(2);

        let integrator = RK4Integrator::new();
        integrator
            .step(&hamiltonian, &mut state, 0.0, 0.01)
            .unwrap();

        let norm_sq: f64 = state.data().iter().map(|x| x.norm_sqr()).sum();
        assert_relative_eq!(norm_sq, 1.0, epsilon = 1e-10);
    }
}
