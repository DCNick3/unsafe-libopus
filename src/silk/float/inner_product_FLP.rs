use ndarray::{aview1, azip};

/// Inner product of two silk_float arrays, with result as double
pub fn silk_inner_product_FLP(data1: &[f32], data2: &[f32]) -> f64 {
    let data1 = aview1(data1);
    let data2 = aview1(data2);

    let mut result = 0f64;
    azip!((&x1 in data1, &x2 in data2) {
        result += x1 as f64 * x2 as f64;
    });

    result
}
