use crate::silk::define::MAX_PREDICTION_POWER_GAIN;

use crate::externs::memcpy;
#[c2rust::src_loc = "39:1"]
pub unsafe fn silk_LPC_inverse_pred_gain_FLP(A: *const f32, order: i32) -> f32 {
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut invGain: f64 = 0.;
    let mut rc: f64 = 0.;
    let mut rc_mult1: f64 = 0.;
    let mut rc_mult2: f64 = 0.;
    let mut tmp1: f64 = 0.;
    let mut tmp2: f64 = 0.;
    let mut Atmp: [f32; 24] = [0.; 24];
    memcpy(
        Atmp.as_mut_ptr() as *mut core::ffi::c_void,
        A as *const core::ffi::c_void,
        (order as u64).wrapping_mul(::core::mem::size_of::<f32>() as u64),
    );
    invGain = 1.0f64;
    k = order - 1 as i32;
    while k > 0 as i32 {
        rc = -Atmp[k as usize] as f64;
        rc_mult1 = 1.0f32 as f64 - rc * rc;
        invGain *= rc_mult1;
        if (invGain * MAX_PREDICTION_POWER_GAIN as f64) < 1.0f32 as f64 {
            return 0.0f32;
        }
        rc_mult2 = 1.0f32 as f64 / rc_mult1;
        n = 0 as i32;
        while n < k + 1 as i32 >> 1 as i32 {
            tmp1 = Atmp[n as usize] as f64;
            tmp2 = Atmp[(k - n - 1 as i32) as usize] as f64;
            Atmp[n as usize] = ((tmp1 - tmp2 * rc) * rc_mult2) as f32;
            Atmp[(k - n - 1 as i32) as usize] = ((tmp2 - tmp1 * rc) * rc_mult2) as f32;
            n += 1;
        }
        k -= 1;
    }
    rc = -Atmp[0 as i32 as usize] as f64;
    rc_mult1 = 1.0f32 as f64 - rc * rc;
    invGain *= rc_mult1;
    if (invGain * MAX_PREDICTION_POWER_GAIN as f64) < 1.0f32 as f64 {
        return 0.0f32;
    }
    return invGain as f32;
}
