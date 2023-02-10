use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::int32_t;
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
pub use self::opus_types_h::opus_int32;
pub use self::stdint_intn_h::int32_t;
use self::string_h::memcpy;
pub use self::types_h::__int32_t;
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_LPC_inverse_pred_gain_FLP(
    mut A: *const libc::c_float,
    mut order: opus_int32,
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
        if (invGain * 1e4f32 as libc::c_double) < 1.0f32 as libc::c_double {
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
    if (invGain * 1e4f32 as libc::c_double) < 1.0f32 as libc::c_double {
        return 0.0f32;
    }
    return invGain as libc::c_float;
}
