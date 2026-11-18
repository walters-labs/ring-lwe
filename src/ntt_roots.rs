/// Roots used by the three fixed, verified NTT configurations.
///
/// This file is copied unchanged to `src/ntt_roots.rs` in both LWE consumers.
/// Other parameter pairs retain the existing root-generation behavior; the
/// formal contracts cover only the three explicit branches below.
pub fn root_for_parameters(modulus: i64, length: usize) -> i64 {
    match (modulus, length) {
        (12289, 1024) => 10302,
        (12289, 2048) => 1945,
        (151019521, 2048) => 51689479,
        _ => ntt::omega(modulus, length),
    }
}
