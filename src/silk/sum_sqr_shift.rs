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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/macros.h:32"]
pub mod macros_h {
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn silk_CLZ32(mut in32: opus_int32) -> opus_int32 {
        return if in32 != 0 {
            32 as libc::c_int
                - (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }
    use super::opus_types_h::opus_int32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "572:1"]
    pub unsafe extern "C" fn silk_max_32(mut a: opus_int32, mut b: opus_int32) -> opus_int32 {
        return if a > b { a } else { b };
    }
    use super::opus_types_h::opus_int32;
}
pub use self::macros_h::silk_CLZ32;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_uint32};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::types_h::{__int16_t, __int32_t, __uint32_t};
pub use self::SigProc_FIX_h::silk_max_32;
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_sum_sqr_shift(
    mut energy: *mut opus_int32,
    mut shift: *mut libc::c_int,
    mut x: *const opus_int16,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shft: libc::c_int = 0;
    let mut nrg_tmp: opus_uint32 = 0;
    let mut nrg: opus_int32 = 0;
    shft = 31 as libc::c_int - silk_CLZ32(len);
    nrg = len;
    i = 0 as libc::c_int;
    while i < len - 1 as libc::c_int {
        nrg_tmp = (*x.offset(i as isize) as opus_int32 * *x.offset(i as isize) as opus_int32)
            as opus_uint32;
        nrg_tmp = nrg_tmp.wrapping_add(
            (*x.offset((i + 1 as libc::c_int) as isize) as opus_int32
                * *x.offset((i + 1 as libc::c_int) as isize) as opus_int32)
                as opus_uint32,
        ) as opus_int32 as opus_uint32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32;
        i += 2 as libc::c_int;
    }
    if i < len {
        nrg_tmp = (*x.offset(i as isize) as opus_int32 * *x.offset(i as isize) as opus_int32)
            as opus_uint32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32;
    }
    shft = silk_max_32(0 as libc::c_int, shft + 3 as libc::c_int - silk_CLZ32(nrg));
    nrg = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len - 1 as libc::c_int {
        nrg_tmp = (*x.offset(i as isize) as opus_int32 * *x.offset(i as isize) as opus_int32)
            as opus_uint32;
        nrg_tmp = nrg_tmp.wrapping_add(
            (*x.offset((i + 1 as libc::c_int) as isize) as opus_int32
                * *x.offset((i + 1 as libc::c_int) as isize) as opus_int32)
                as opus_uint32,
        ) as opus_int32 as opus_uint32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32;
        i += 2 as libc::c_int;
    }
    if i < len {
        nrg_tmp = (*x.offset(i as isize) as opus_int32 * *x.offset(i as isize) as opus_int32)
            as opus_uint32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32;
    }
    *shift = shft;
    *energy = nrg;
}
