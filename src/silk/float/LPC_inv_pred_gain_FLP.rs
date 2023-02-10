use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:34"]
pub mod define_h {
    #[c2rust::src_loc = "139:9"]
    pub const MAX_PREDICTION_POWER_GAIN: libc::c_float = 1e4f32;
}
pub use self::define_h::MAX_PREDICTION_POWER_GAIN;

use crate::externs::memcpy;
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_LPC_inverse_pred_gain_FLP(
    A: *const libc::c_float,
    order: i32,
) -> libc::c_float {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut invGain: libc::c_double = 0.;
    let mut rc: libc::c_double = 0.;
    let mut rc_mult1: libc::c_double = 0.;
    let mut rc_mult2: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut tmp2: libc::c_double = 0.;
    let mut Atmp: [libc::c_float; 24] = [0.; 24];
    memcpy(
        Atmp.as_mut_ptr() as *mut libc::c_void,
        A as *const libc::c_void,
        (order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    invGain = 1.0f64;
    k = order - 1 as libc::c_int;
    while k > 0 as libc::c_int {
        rc = -Atmp[k as usize] as libc::c_double;
        rc_mult1 = 1.0f32 as libc::c_double - rc * rc;
        invGain *= rc_mult1;
        if (invGain * MAX_PREDICTION_POWER_GAIN as libc::c_double) < 1.0f32 as libc::c_double {
            return 0.0f32;
        }
        rc_mult2 = 1.0f32 as libc::c_double / rc_mult1;
        n = 0 as libc::c_int;
        while n < k + 1 as libc::c_int >> 1 as libc::c_int {
            tmp1 = Atmp[n as usize] as libc::c_double;
            tmp2 = Atmp[(k - n - 1 as libc::c_int) as usize] as libc::c_double;
            Atmp[n as usize] = ((tmp1 - tmp2 * rc) * rc_mult2) as libc::c_float;
            Atmp[(k - n - 1 as libc::c_int) as usize] =
                ((tmp2 - tmp1 * rc) * rc_mult2) as libc::c_float;
            n += 1;
        }
        k -= 1;
    }
    rc = -Atmp[0 as libc::c_int as usize] as libc::c_double;
    rc_mult1 = 1.0f32 as libc::c_double - rc * rc;
    invGain *= rc_mult1;
    if (invGain * MAX_PREDICTION_POWER_GAIN as libc::c_double) < 1.0f32 as libc::c_double {
        return 0.0f32;
    }
    return invGain as libc::c_float;
}
