use crate::core::{DensityMatrix, QuantumState};
use crate::utils::Result;
use ndarray::Array2;
use num_complex::Complex64;

pub trait Observable: Send + Sync {
    fn dim(&self) -> usize;

    fn matrix(&self) -> &Array2<Complex64>;

    fn expectation_pure(&self, state: &QuantumState) -> Complex64 {
        let psi = state.data();
        let op = self.matrix();
        let dim = self.dim();

        let mut sum = Complex64::new(0.0, 0.0);
        for i in 0..dim {
            for j in 0..dim {
                sum += psi[i].conj() * op[[i, j]] * psi[j];
            }
        }
        sum
    }

    fn expectation_mixed(&self, state: &DensityMatrix) -> Complex64 {
        let rho = state.data();
        let op = self.matrix();
        let dim = self.dim();

        let mut sum = Complex64::new(0.0, 0.0);
        for i in 0..dim {
            for j in 0..dim {
                sum += rho[[i, j]] * op[[j, i]];
            }
        }
        sum
    }
}

#[derive(Debug, Clone)]
pub struct ExpectationValue {
    pub time: f64,
    pub value: Complex64,
}

pub struct MatrixObservable {
    matrix: Array2<Complex64>,
}

impl MatrixObservable {
    pub fn new(matrix: Array2<Complex64>) -> Self {
        Self { matrix }
    }
}

impl Observable for MatrixObservable {
    fn dim(&self) -> usize {
        self.matrix.nrows()
    }

    fn matrix(&self) -> &Array2<Complex64> {
        &self.matrix
    }
}

pub struct NumberOperator {
    matrix: Array2<Complex64>,
}

impl NumberOperator {
    pub fn new(dim: usize) -> Self {
        let mut matrix = Array2::zeros((dim, dim));
        for n in 0..dim {
            matrix[[n, n]] = Complex64::new(n as f64, 0.0);
        }
        Self { matrix }
    }
}

impl Observable for NumberOperator {
    fn dim(&self) -> usize {
        self.matrix.nrows()
    }

    fn matrix(&self) -> &Array2<Complex64> {
        &self.matrix
    }
}

pub struct PopulationOperator {
    matrix: Array2<Complex64>,
    level: usize,
}

impl PopulationOperator {
    pub fn new(dim: usize, level: usize) -> Result<Self> {
        if level >= dim {
            return Err(crate::utils::Error::InvalidParameter(format!(
                "Level {} out of bounds for dimension {}",
                level, dim
            )));
        }

        let mut matrix = Array2::zeros((dim, dim));
        matrix[[level, level]] = Complex64::new(1.0, 0.0);

        Ok(Self { matrix, level })
    }
}

impl Observable for PopulationOperator {
    fn dim(&self) -> usize {
        self.matrix.nrows()
    }

    fn matrix(&self) -> &Array2<Complex64> {
        &self.matrix
    }
}

pub struct CoherenceOperator {
    matrix: Array2<Complex64>,
}

impl CoherenceOperator {
    pub fn new(dim: usize, i: usize, j: usize) -> Result<Self> {
        if i >= dim || j >= dim {
            return Err(crate::utils::Error::InvalidParameter(
                "Indices out of bounds".to_string(),
            ));
        }

        let mut matrix = Array2::zeros((dim, dim));
        matrix[[i, j]] = Complex64::new(1.0, 0.0);

        Ok(Self { matrix })
    }
}

impl Observable for CoherenceOperator {
    fn dim(&self) -> usize {
        self.matrix.nrows()
    }

    fn matrix(&self) -> &Array2<Complex64> {
        &self.matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_population_operator() {
        let pop = PopulationOperator::new(2, 0).unwrap();
        let ground = QuantumState::ground_state(2);

        let exp_val = pop.expectation_pure(&ground);
        assert_relative_eq!(exp_val.re, 1.0);
        assert_relative_eq!(exp_val.im, 0.0);
    }

    #[test]
    fn test_number_operator() {
        let num_op = NumberOperator::new(3);
        let ground = QuantumState::ground_state(3);

        let exp_val = num_op.expectation_pure(&ground);
        assert_relative_eq!(exp_val.re, 0.0);
    }
}
