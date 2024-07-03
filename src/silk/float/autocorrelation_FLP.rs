use crate::silk::float::inner_product_FLP::silk_inner_product_FLP;

/// Compute autocorrelation
///
/// ```text
/// results          O  result (length correlationCount)
/// inputData        I  input data to correlate
/// inputDataSize    I  length of input
/// correlationCount I  number of correlation taps to compute
/// ```
pub fn silk_autocorrelation_FLP(results: &mut [f32], input: &[f32]) {
    let results = if results.len() > input.len() {
        &mut results[0..input.len()]
    } else {
        results
    };

    for (i, y) in (0..).zip(results.iter_mut()) {
        let tail = &input[i..];
        let head = &input[..tail.len()];
        *y = silk_inner_product_FLP(head, tail) as f32;
    }
}
