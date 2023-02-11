use ::libc;
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:32"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:32"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int =
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
use crate::silk::macros::silk_CLZ32;
use crate::silk::SigProc_FIX::silk_max_32;

#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_sum_sqr_shift(
    energy: *mut i32,
    shift: *mut libc::c_int,
    x: *const i16,
    len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shft: libc::c_int = 0;
    let mut nrg_tmp: u32 = 0;
    let mut nrg: i32 = 0;
    shft = 31 as libc::c_int - silk_CLZ32(len);
    nrg = len;
    i = 0 as libc::c_int;
    while i < len - 1 as libc::c_int {
        nrg_tmp = (*x.offset(i as isize) as i32 * *x.offset(i as isize) as i32) as u32;
        nrg_tmp = nrg_tmp.wrapping_add(
            (*x.offset((i + 1 as libc::c_int) as isize) as i32
                * *x.offset((i + 1 as libc::c_int) as isize) as i32) as u32,
        ) as i32 as u32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as i32;
        i += 2 as libc::c_int;
    }
    if i < len {
        nrg_tmp = (*x.offset(i as isize) as i32 * *x.offset(i as isize) as i32) as u32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as i32;
    }
    shft = silk_max_32(0 as libc::c_int, shft + 3 as libc::c_int - silk_CLZ32(nrg));
    nrg = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len - 1 as libc::c_int {
        nrg_tmp = (*x.offset(i as isize) as i32 * *x.offset(i as isize) as i32) as u32;
        nrg_tmp = nrg_tmp.wrapping_add(
            (*x.offset((i + 1 as libc::c_int) as isize) as i32
                * *x.offset((i + 1 as libc::c_int) as isize) as i32) as u32,
        ) as i32 as u32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as i32;
        i += 2 as libc::c_int;
    }
    if i < len {
        nrg_tmp = (*x.offset(i as isize) as i32 * *x.offset(i as isize) as i32) as u32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as i32;
    }
    *shift = shft;
    *energy = nrg;
}
