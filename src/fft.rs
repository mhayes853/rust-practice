use std::f64::consts::PI;

/// Computes the fast Fourier transform of the provided real-valued samples.
///
/// Each result is represented as `(real, imaginary)`.
pub fn fft(input: &[f64]) -> Vec<(f64, f64)> {
    if input.is_empty() {
        return vec![];
    }
    if input.len() == 1 {
        return vec![(input[0], 0.0)];
    }

    let N = input.len();
    let mut evens = Vec::<f64>::new();
    let mut odds = Vec::<f64>::new();
    for i in 0..N {
        if i.is_multiple_of(2) {
            evens.push(input[i]);
        } else {
            odds.push(input[i]);
        }
    }
    let e = fft(&evens);
    let o = fft(&odds);

    let mut x = vec![(0.0, 0.0); input.len()];

    for i in 0..(N / 2) {
        let k = i as f64;
        let theta = (2.0 * PI * k) / N as f64;
        let w = (theta.cos(), -theta.sin());
        let t = (w.0 * o[i].0 - w.1 * o[i].1, w.0 * o[i].1 + w.1 * o[i].0);
        x[i] = (e[i].0 + t.0, e[i].1 + t.1);
        x[i + N / 2] = (e[i].0 - t.0, e[i].1 - t.1);
    }

    x
}

#[cfg(test)]
mod tests {
    use super::fft;

    #[test]
    fn empty_input_has_no_frequency_bins() {
        assert!(fft(&[]).is_empty());
    }

    #[test]
    fn a_single_sample_is_its_own_zero_frequency_bin() {
        assert_complexes_close(&fft(&[3.0]), &[(3.0, 0.0)]);
    }

    #[test]
    fn an_impulse_has_equal_energy_in_every_bin() {
        assert_complexes_close(
            &fft(&[1.0, 0.0, 0.0, 0.0]),
            &[(1.0, 0.0), (1.0, 0.0), (1.0, 0.0), (1.0, 0.0)],
        );
    }

    #[test]
    fn a_constant_signal_only_has_a_zero_frequency_component() {
        assert_complexes_close(
            &fft(&[2.0, 2.0, 2.0, 2.0]),
            &[(8.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0)],
        );
    }

    #[test]
    fn four_point_transform_matches_known_values() {
        assert_complexes_close(
            &fft(&[1.0, 2.0, 3.0, 4.0]),
            &[(10.0, 0.0), (-2.0, 2.0), (-2.0, 0.0), (-2.0, -2.0)],
        );
    }

    #[test]
    fn real_input_has_conjugate_symmetric_output() {
        let output = fft(&[1.0, 2.0, 3.0, 4.0]);

        assert_complex_close(output[1], (output[3].0, -output[3].1));
        assert!(output[0].1.abs() < 1e-10);
        assert!(output[2].1.abs() < 1e-10);
    }

    fn assert_complexes_close(actual: &[(f64, f64)], expected: &[(f64, f64)]) {
        assert_eq!(actual.len(), expected.len());

        for (actual, expected) in actual.iter().zip(expected) {
            assert_complex_close(*actual, *expected);
        }
    }

    fn assert_complex_close(actual: (f64, f64), expected: (f64, f64)) {
        assert!(
            (actual.0 - expected.0).abs() < 1e-10,
            "real: {actual:?} != {expected:?}"
        );
        assert!(
            (actual.1 - expected.1).abs() < 1e-10,
            "imaginary: {actual:?} != {expected:?}"
        );
    }
}
