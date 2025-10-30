
use ndarray::Array2;
use num_complex::Complex64;
use crate::utils::Result;

pub trait Hamiltonian: Send + Sync {

    fn dim(&self) -> usize;

    fn compute(&self, t: f64, out: &mut Array2<Complex64>);

    fn is_time_independent(&self) -> bool {
        false
    }

    fn period(&self) -> Option<f64> {
        None
    }

    fn validate(&self) -> Result<()> {
        use crate::utils::math::is_hermitian;

        let mut h = Array2::zeros((self.dim(), self.dim()));
        self.compute(0.0, &mut h);

        if !is_hermitian(&h.view(), 1e-10) {
            return Err(crate::utils::Error::Hamiltonian(
                "Hamiltonian is not Hermitian".to_string(),
            ));
        }

        Ok(())
    }
}

pub struct TimeIndependentHamiltonian {
    matrix: Array2<Complex64>,
}

impl TimeIndependentHamiltonian {

    pub fn new(matrix: Array2<Complex64>) -> Self {
        Self { matrix }
    }
}

impl Hamiltonian for TimeIndependentHamiltonian {
    fn dim(&self) -> usize {
        self.matrix.nrows()
    }

    fn compute(&self, _t: f64, out: &mut Array2<Complex64>) {
        out.assign(&self.matrix);
    }

    fn is_time_independent(&self) -> bool {
        true
    }
}

pub struct CompositeHamiltonian {
    terms: Vec<Box<dyn Hamiltonian>>,
    dim: usize,
}

impl CompositeHamiltonian {

    pub fn new(terms: Vec<Box<dyn Hamiltonian>>) -> Result<Self> {
        if terms.is_empty() {
            return Err(crate::utils::Error::Hamiltonian(
                "Composite Hamiltonian must have at least one term".to_string(),
            ));
        }

        let dim = terms[0].dim();
        for term in &terms {
            if term.dim() != dim {
                return Err(crate::utils::Error::DimensionMismatch {
                    expected: dim,
                    actual: term.dim(),
                });
            }
        }

        Ok(Self { terms, dim })
    }
}

impl Hamiltonian for CompositeHamiltonian {
    fn dim(&self) -> usize {
        self.dim
    }

    fn compute(&self, t: f64, out: &mut Array2<Complex64>) {
        out.fill(Complex64::new(0.0, 0.0));

        let mut temp = Array2::zeros((self.dim, self.dim));
        for term in &self.terms {
            term.compute(t, &mut temp);
            *out += &temp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_time_independent_hamiltonian() {
        let mut h = Array2::zeros((2, 2));
        h[[0, 0]] = Complex64::new(1.0, 0.0);
        h[[1, 1]] = Complex64::new(-1.0, 0.0);

        let hamiltonian = TimeIndependentHamiltonian::new(h);
        assert_eq!(hamiltonian.dim(), 2);
        assert!(hamiltonian.is_time_independent());

        let mut out = Array2::zeros((2, 2));
        hamiltonian.compute(5.0, &mut out);
        assert_relative_eq!(out[[0, 0]].re, 1.0);
        assert_relative_eq!(out[[1, 1]].re, -1.0);
    }
}
