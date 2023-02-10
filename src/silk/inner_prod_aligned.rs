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
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::types_h::{__int16_t, __int32_t};
#[no_mangle]
#[c2rust::src_loc = "34:1"]
pub unsafe extern "C" fn silk_inner_prod_aligned_scale(
    inVec1: *const i16,
    inVec2: *const i16,
    scale: libc::c_int,
    len: libc::c_int,
) -> i32 {
    let mut i: libc::c_int = 0;
    let mut sum: i32 = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        sum =
            sum + (*inVec1.offset(i as isize) as i32 * *inVec2.offset(i as isize) as i32 >> scale);
        i += 1;
    }
    return sum;
}
