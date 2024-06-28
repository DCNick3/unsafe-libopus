use crate::celt::float_cast::float2int;
use crate::silk::SigProc_FIX::silk_SAT16;

#[inline]
pub fn silk_float2int(x: f32) -> i32 {
    float2int(x)
}

#[inline]
pub fn silk_float2short_array(out: &mut [i16], input: &[f32]) {
    let length = out.len();
    assert_eq!(length, input.len());

    for k in (0..length).rev() {
        out[k] = silk_SAT16(float2int(input[k])) as i16;
    }
}

#[inline]
pub fn silk_short2float_array(out: &mut [f32], input: &[i16]) {
    let length = out.len();
    assert_eq!(length, input.len());

    for k in (0..length).rev() {
        out[k] = input[k] as f32;
    }
}

#[inline]
pub fn silk_log2(x: f64) -> f32 {
    return (std::f64::consts::LOG2_10 * x.log10()) as f32;
}

#[inline]
pub fn silk_sigmoid(x: f32) -> f32 {
    return (1.0f64 / (1.0f64 + (-x as f64).exp())) as f32;
}
