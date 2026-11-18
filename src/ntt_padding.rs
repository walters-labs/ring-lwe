use polynomial_ring::Polynomial;

/// Copy coefficients into a fixed-length, zero-padded transform buffer.
pub fn padded(polynomial: &Polynomial<i64>, n: usize) -> Vec<i64> {
    assert!(polynomial.coeffs().len() <= n);
    let mut output = vec![0; n];
    output[..polynomial.coeffs().len()].copy_from_slice(polynomial.coeffs());
    output
}
