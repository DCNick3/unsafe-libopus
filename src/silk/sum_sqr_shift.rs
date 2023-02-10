use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/macros.h:32"]
pub mod macros_h {
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn silk_CLZ32(in32: i32) -> i32 {
        return if in32 != 0 {
            32 as libc::c_int - (EC_CLZ0 - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }
    use super::ecintrin_h::EC_CLZ0;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "572:1"]
    pub unsafe extern "C" fn silk_max_32(a: i32, b: i32) -> i32 {
        return if a > b { a } else { b };
    }
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
pub use self::macros_h::silk_CLZ32;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::types_h::{__int16_t, __int32_t, __uint32_t};
pub use self::SigProc_FIX_h::silk_max_32;
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
