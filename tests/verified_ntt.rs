#![cfg(test)]

use polynomial_ring::Polynomial;
use ring_lwe::utils::polymul_fast;

fn polynomial_modulus(degree: usize) -> Polynomial<i64> {
    let mut f = vec![0; degree + 1];
    f[0] = 1;
    f[degree] = 1;
    Polynomial::new(f)
}

// Independent schoolbook negacyclic multiplication with wide accumulators.
fn reference(a: &[i64], b: &[i64], degree: usize, q: i64) -> Vec<i64> {
    let mut product = vec![0i128; degree];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            let term = i128::from(x.rem_euclid(q)) * i128::from(y.rem_euclid(q));
            if i + j < degree {
                product[i + j] += term;
            } else {
                product[i + j - degree] -= term;
            }
        }
    }
    product
        .into_iter()
        .map(|x| {
            let canonical = x.rem_euclid(i128::from(q)) as i64;
            if canonical > q / 2 {
                canonical - q
            } else {
                canonical
            }
        })
        .collect()
}

fn check(a: Vec<i64>, b: Vec<i64>, degree: usize, q: i64, root: i64) {
    let expected = reference(&a, &b, degree, q);
    let actual = polymul_fast(
        &Polynomial::new(a),
        &Polynomial::new(b),
        q,
        &polynomial_modulus(degree),
        root,
    );
    let mut coefficients = actual.coeffs().to_vec();
    assert!(coefficients.len() <= degree);
    coefficients.resize(degree, 0);
    assert_eq!(coefficients, expected, "degree={degree}, q={q}");
}

#[test]
fn actual_adapter_matches_schoolbook_at_all_three_fixed_tuples() {
    let ring = ring_lwe::utils::Parameters::default();
    for (degree, q, root) in [
        (512, 12289, 10302),
        (ring.n, ring.q, ring.omega),
        (1024, 151019521, 51689479),
    ] {
        check(vec![], vec![], degree, q, root);
        check(vec![], vec![1, -2, 3], degree, q, root);
        check(vec![i64::MIN], vec![i64::MAX], degree, q, root);
        check(vec![1, -2, 3, 0, 0], vec![4, 5, -6], degree, q, root);
        let mut highest = vec![0; degree];
        highest[degree - 1] = 1;
        check(highest, vec![0, 1], degree, q, root);
        let mut a: Vec<_> = (0..degree).map(|i| i as i64 * 7919 - q).collect();
        let mut b: Vec<_> = (0..degree).map(|i| q - i as i64 * 104729).collect();
        a[0] = i64::MIN;
        b[0] = i64::MAX;
        check(a, b, degree, q, root);
    }
}

#[test]
fn toy_ring_uses_its_own_degree_and_root() {
    check(vec![1, 2, 3, 4], vec![5, 6, 7, 8], 4, 17, 9);
}

#[test]
fn malformed_rings_oversized_inputs_and_mismatched_roots_are_rejected() {
    let one = Polynomial::new(vec![1]);
    for f in [
        vec![],
        vec![1],
        vec![1, 0, 0, 1],
        vec![2, 0, 0, 0, 1],
        vec![1, 1, 0, 0, 1],
    ] {
        assert!(
            std::panic::catch_unwind(|| polymul_fast(&one, &one, 17, &Polynomial::new(f), 9))
                .is_err()
        );
    }
    let f = polynomial_modulus(4);
    let oversized = Polynomial::new(vec![0, 0, 0, 0, 1]);
    assert!(std::panic::catch_unwind(|| polymul_fast(&oversized, &one, 17, &f, 9)).is_err());
    assert!(std::panic::catch_unwind(|| polymul_fast(&one, &oversized, 17, &f, 9)).is_err());
    assert!(std::panic::catch_unwind(|| polymul_fast(
        &one,
        &one,
        12289,
        &polynomial_modulus(512),
        1945
    ))
    .is_err());
}
