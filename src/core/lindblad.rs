use crate::core::{DensityMatrix, Hamiltonian};
use crate::utils::Result;
use ndarray::Array2;
use num_complex::Complex64;

pub struct LindbladOperator {
    pub operator: Array2<Complex64>,

    pub rate: f64,
}

impl LindbladOperator {
    pub fn new(operator: Array2<Complex64>, rate: f64) -> Result<Self> {
        if rate < 0.0 {
            return Err(crate::utils::Error::InvalidParameter(
                "Lindblad rate must be non-negative".to_string(),
            ));
        }
        Ok(Self { operator, rate })
    }

    pub fn annihilation(dim: usize, rate: f64) -> Result<Self> {
        let mut op = Array2::zeros((dim, dim));
        for n in 1..dim {
            op[[n - 1, n]] = Complex64::new((n as f64).sqrt(), 0.0);
        }
        Self::new(op, rate)
    }

    pub fn dephasing(dim: usize, rate: f64) -> Result<Self> {
        let mut op = Array2::zeros((dim, dim));
        for n in 0..dim {
            op[[n, n]] = Complex64::new(n as f64, 0.0);
        }
        Self::new(op, rate)
    }
}

pub struct LindbladSolver {
    hamiltonian: Box<dyn Hamiltonian>,
    lindblad_ops: Vec<LindbladOperator>,
    dim: usize,
}

impl LindbladSolver {
    pub fn new(
        hamiltonian: Box<dyn Hamiltonian>,
        lindblad_ops: Vec<LindbladOperator>,
    ) -> Result<Self> {
        let dim = hamiltonian.dim();

        for op in &lindblad_ops {
            if op.operator.nrows() != dim || op.operator.ncols() != dim {
                return Err(crate::utils::Error::DimensionMismatch {
                    expected: dim,
                    actual: op.operator.nrows(),
                });
            }
        }

        Ok(Self {
            hamiltonian,
            lindblad_ops,
            dim,
        })
    }

    pub fn compute_derivative(&self, rho: &DensityMatrix, t: f64) -> Result<Array2<Complex64>> {
        let mut drho_dt = Array2::zeros((self.dim, self.dim));

        let mut h = Array2::zeros((self.dim, self.dim));
        self.hamiltonian.compute(t, &mut h);

        let i = Complex64::new(0.0, 1.0);
        let rho_data = rho.data();

        let mut h_rho = Array2::zeros((self.dim, self.dim));
        let mut rho_h = Array2::zeros((self.dim, self.dim));

        for row in 0..self.dim {
            for col in 0..self.dim {
                let mut sum1 = Complex64::new(0.0, 0.0);
                let mut sum2 = Complex64::new(0.0, 0.0);
                for k in 0..self.dim {
                    sum1 += h[[row, k]] * rho_data[[k, col]];
                    sum2 += rho_data[[row, k]] * h[[k, col]];
                }
                h_rho[[row, col]] = sum1;
                rho_h[[row, col]] = sum2;
            }
        }

        drho_dt = -i * (h_rho - rho_h);

        for lindblad_op in &self.lindblad_ops {
            let l = &lindblad_op.operator;
            let gamma = lindblad_op.rate;

            let mut l_rho = Array2::zeros((self.dim, self.dim));
            for i in 0..self.dim {
                for j in 0..self.dim {
                    let mut sum = Complex64::new(0.0, 0.0);
                    for k in 0..self.dim {
                        sum += l[[i, k]] * rho_data[[k, j]];
                    }
                    l_rho[[i, j]] = sum;
                }
            }

            let mut l_rho_ldag = Array2::zeros((self.dim, self.dim));
            for i in 0..self.dim {
                for j in 0..self.dim {
                    let mut sum = Complex64::new(0.0, 0.0);
                    for k in 0..self.dim {
                        sum += l_rho[[i, k]] * l[[j, k]].conj();
                    }
                    l_rho_ldag[[i, j]] = sum;
                }
            }

            let mut ldag_l = Array2::zeros((self.dim, self.dim));
            for i in 0..self.dim {
                for j in 0..self.dim {
                    let mut sum = Complex64::new(0.0, 0.0);
                    for k in 0..self.dim {
                        sum += l[[k, i]].conj() * l[[k, j]];
                    }
                    ldag_l[[i, j]] = sum;
                }
            }

            let mut ldag_l_rho = Array2::zeros((self.dim, self.dim));
            let mut rho_ldag_l = Array2::zeros((self.dim, self.dim));

            for i in 0..self.dim {
                for j in 0..self.dim {
                    let mut sum1 = Complex64::new(0.0, 0.0);
                    let mut sum2 = Complex64::new(0.0, 0.0);
                    for k in 0..self.dim {
                        sum1 += ldag_l[[i, k]] * rho_data[[k, j]];
                        sum2 += rho_data[[i, k]] * ldag_l[[k, j]];
                    }
                    ldag_l_rho[[i, j]] = sum1;
                    rho_ldag_l[[i, j]] = sum2;
                }
            }

            let anticommutator = ldag_l_rho + rho_ldag_l;

            let term = l_rho_ldag - anticommutator.mapv(|x| x * 0.5);
            drho_dt = drho_dt + term.mapv(|x| x * gamma);
        }

        Ok(drho_dt)
    }

    pub fn step(&self, rho: &mut DensityMatrix, t: f64, dt: f64) -> Result<()> {
        let k1 = self.compute_derivative(rho, t)?;

        let mut rho2_data = rho.data().clone() + &k1.mapv(|x| x * Complex64::new(dt / 2.0, 0.0));
        let rho2 = DensityMatrix::new_unchecked(rho2_data.clone());
        let k2 = self.compute_derivative(&rho2, t + dt / 2.0)?;

        let mut rho3_data = rho.data().clone() + &k2.mapv(|x| x * Complex64::new(dt / 2.0, 0.0));
        let rho3 = DensityMatrix::new_unchecked(rho3_data.clone());
        let k3 = self.compute_derivative(&rho3, t + dt / 2.0)?;

        let mut rho4_data = rho.data().clone() + &k3.mapv(|x| x * Complex64::new(dt, 0.0));
        let rho4 = DensityMatrix::new_unchecked(rho4_data.clone());
        let k4 = self.compute_derivative(&rho4, t + dt)?;

        let increment = k1 + k2.mapv(|x| x * 2.0) + k3.mapv(|x| x * 2.0) + k4;
        let new_data = rho.data().clone() + &increment.mapv(|x| x * Complex64::new(dt / 6.0, 0.0));

        *rho = DensityMatrix::new_unchecked(new_data);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::hamiltonian::TimeIndependentHamiltonian;

    #[test]
    fn test_annihilation_operator() {
        let op = LindbladOperator::annihilation(3, 0.1).unwrap();
        assert_eq!(op.operator.nrows(), 3);
        assert_eq!(op.rate, 0.1);
    }

    #[test]
    fn test_lindblad_solver_creation() {
        let h = Array2::zeros((2, 2));
        let ham = TimeIndependentHamiltonian::new(h);
        let lindblad_ops = vec![LindbladOperator::annihilation(2, 0.01).unwrap()];

        let solver = LindbladSolver::new(Box::new(ham), lindblad_ops);
        assert!(solver.is_ok());
    }
}
