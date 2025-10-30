
use ndarray::Array2;
use num_complex::Complex64;
use crate::core::Hamiltonian;

pub struct DrivenCavity {
    pub omega_c: f64,
    pub omega_p: f64,
    pub g: f64,
    dim: usize,
}

impl DrivenCavity {

    pub fn new(omega_c: f64, omega_p: f64, g: f64, dim: usize) -> Self {
        Self {
            omega_c,
            omega_p,
            g,
            dim,
        }
    }
}

impl Hamiltonian for DrivenCavity {
    fn dim(&self) -> usize {
        self.dim
    }

    fn compute(&self, t: f64, out: &mut Array2<Complex64>) {
        out.fill(Complex64::new(0.0, 0.0));

        let drive = self.g * (self.omega_p * t).cos();

        for n in 0..self.dim {
            out[[n, n]] = Complex64::new(self.omega_c * n as f64, 0.0);
        }

        for n in 0..self.dim - 2 {
            let amp = ((n + 1) * (n + 2)) as f64;
            out[[n + 2, n]] += Complex64::new(drive * amp.sqrt(), 0.0);
            out[[n, n + 2]] += Complex64::new(drive * amp.sqrt(), 0.0);
        }
    }

    fn period(&self) -> Option<f64> {
        Some(2.0 * std::f64::consts::PI / self.omega_p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::math::is_hermitian;

    #[test]
    fn test_driven_cavity() {
        let cavity = DrivenCavity::new(10.0, 20.0, 0.3, 10);
        let mut h = Array2::zeros((10, 10));
        cavity.compute(0.0, &mut h);
        assert!(is_hermitian(&h.view(), 1e-10));
    }
}
