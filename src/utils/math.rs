//! Mathematical utilities and helper functions

use ndarray::{Array2, ArrayView2};
use num_complex::Complex64;

/// Check if a matrix is Hermitian within a tolerance
pub fn is_hermitian(matrix: &ArrayView2<Complex64>, tol: f64) -> bool {
    if matrix.nrows() != matrix.ncols() {
        return false;
    }

    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            let diff = (matrix[[i, j]] - matrix[[j, i]].conj()).norm();
            if diff > tol {
                return false;
            }
        }
    }

    true
}

/// Check if a matrix is unitary within a tolerance
pub fn is_unitary(matrix: &ArrayView2<Complex64>, tol: f64) -> bool {
    let dim = matrix.nrows();
    if dim != matrix.ncols() {
        return false;
    }

    // Compute U^† U
    let mut result = Array2::zeros((dim, dim));
    for i in 0..dim {
        for j in 0..dim {
            let mut sum = Complex64::new(0.0, 0.0);
            for k in 0..dim {
                sum += matrix[[k, i]].conj() * matrix[[k, j]];
            }
            result[[i, j]] = sum;
        }
    }

    // Check if result is identity
    for i in 0..dim {
        for j in 0..dim {
            let expected = if i == j { Complex64::new(1.0, 0.0) } else { Complex64::new(0.0, 0.0) };
            if (result[[i, j]] - expected).norm() > tol {
                return false;
            }
        }
    }

    true
}

/// Compute the trace of a matrix
pub fn trace(matrix: &ArrayView2<Complex64>) -> Complex64 {
    let mut sum = Complex64::new(0.0, 0.0);
    for i in 0..matrix.nrows().min(matrix.ncols()) {
        sum += matrix[[i, i]];
    }
    sum
}

/// Compute the Frobenius norm of a matrix
pub fn frobenius_norm(matrix: &ArrayView2<Complex64>) -> f64 {
    matrix.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt()
}

/// Generate an identity matrix of given dimension
pub fn identity(dim: usize) -> Array2<Complex64> {
    let mut result = Array2::zeros((dim, dim));
    for i in 0..dim {
        result[[i, i]] = Complex64::new(1.0, 0.0);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_is_hermitian() {
        // Pauli X matrix
        let mut pauli_x = Array2::zeros((2, 2));
        pauli_x[[0, 1]] = Complex64::new(1.0, 0.0);
        pauli_x[[1, 0]] = Complex64::new(1.0, 0.0);

        assert!(is_hermitian(&pauli_x.view(), 1e-10));
    }

    #[test]
    fn test_trace() {
        let mut matrix = Array2::zeros((3, 3));
        matrix[[0, 0]] = Complex64::new(1.0, 0.0);
        matrix[[1, 1]] = Complex64::new(2.0, 0.0);
        matrix[[2, 2]] = Complex64::new(3.0, 0.0);

        let tr = trace(&matrix.view());
        assert_relative_eq!(tr.re, 6.0);
        assert_relative_eq!(tr.im, 0.0);
    }

    #[test]
    fn test_identity() {
        let id = identity(3);
        assert!(is_unitary(&id.view(), 1e-10));
        let tr = trace(&id.view());
        assert_relative_eq!(tr.re, 3.0);
    }
}
