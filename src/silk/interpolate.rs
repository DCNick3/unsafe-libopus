use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
}
pub use self::opus_types_h::{opus_int16, opus_int32};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::types_h::{__int16_t, __int32_t};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_interpolate(
    mut xi: *mut opus_int16,
    mut x0: *const opus_int16,
    mut x1: *const opus_int16,
    ifact_Q2: libc::c_int,
    d: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < d {
        *xi.offset(i as isize) = (*x0.offset(i as isize) as libc::c_int
            + ((*x1.offset(i as isize) as libc::c_int - *x0.offset(i as isize) as libc::c_int)
                as opus_int16 as opus_int32
                * ifact_Q2 as opus_int16 as opus_int32
                >> 2 as libc::c_int)) as opus_int16;
        i += 1;
    }
}
