/// Reduce a canonical length-2d cyclic product modulo x^d + 1, in place.
/// The first half becomes centered coefficients; the second half is unchanged.
/// The caller supplies q > 1 and coefficients in 0..q, as returned by NTT.
pub fn fold_center(product: &mut [i64], q: i64) {
    assert!(!product.is_empty() && product.len() % 2 == 0);
    let degree = product.len() / 2;
    let mut start = 0;
    while start < degree {
        let end = (start + 64).min(degree);
        fold_center_block(product, q, start, end);
        start = end;
    }
}

fn fold_center_block(product: &mut [i64], q: i64, start: usize, end: usize) {
    let degree = product.len() / 2;
    assert!(start <= end && end <= degree);
    for i in start..end {
        product[i] = center_difference(product[i], product[i + degree], q);
    }
}

fn center_difference(low: i64, high: i64, q: i64) -> i64 {
    let residue = (low - high).rem_euclid(q);
    if residue > q / 2 { residue - q } else { residue }
}
