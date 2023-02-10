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
pub use self::opus_types_h::opus_int32;
pub use self::stdint_intn_h::int32_t;
pub use self::types_h::__int32_t;
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_k2a_FLP(
    mut A: *mut libc::c_float,
    mut rc: *const libc::c_float,
    mut order: opus_int32,
) {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut rck: libc::c_float = 0.;
    let mut tmp1: libc::c_float = 0.;
    let mut tmp2: libc::c_float = 0.;
    k = 0 as libc::c_int;
    while k < order {
        rck = *rc.offset(k as isize);
        n = 0 as libc::c_int;
        while n < k + 1 as libc::c_int >> 1 as libc::c_int {
            tmp1 = *A.offset(n as isize);
            tmp2 = *A.offset((k - n - 1 as libc::c_int) as isize);
            *A.offset(n as isize) = tmp1 + tmp2 * rck;
            *A.offset((k - n - 1 as libc::c_int) as isize) = tmp2 + tmp1 * rck;
            n += 1;
        }
        *A.offset(k as isize) = -rck;
        k += 1;
    }
}
