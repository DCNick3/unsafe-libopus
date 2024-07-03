use ndarray::aview1;

/// Sum of squares of a float array, with result as a double
pub fn silk_energy_FLP(data: &[f32]) -> f64 {
    let data_view = aview1(data);

    // opus sources unfold it manually, but LLVM seems to be able to 4x unfold it by itself
    // SIMD might still be nice idk
    data_view.fold(0.0f64, |acc, &x| acc + x as f64 * x as f64)
}
