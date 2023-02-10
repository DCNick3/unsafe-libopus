use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
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
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int16_t, int32_t, int64_t};
    use super::stdint_uintn_h::uint32_t;
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
    pub const EC_CLZ0: libc::c_int = ::core::mem::size_of::<libc::c_uint>()
        as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/macros.h:32"]
pub mod macros_h {
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn silk_CLZ32(mut in32: opus_int32) -> opus_int32 {
        return if in32 != 0 {
            32 as libc::c_int - (EC_CLZ0 - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }
    use super::opus_types_h::opus_int32;
    use super::ecintrin_h::EC_CLZ0;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/Inlines.h:32"]
pub mod Inlines_h {
    #[inline]
    #[c2rust::src_loc = "56:1"]
    pub unsafe extern "C" fn silk_CLZ_FRAC(
        mut in_0: opus_int32,
        mut lz: *mut opus_int32,
        mut frac_Q7: *mut opus_int32,
    ) {
        let mut lzeros: opus_int32 = silk_CLZ32(in_0);
        *lz = lzeros;
        *frac_Q7 = silk_ROR32(in_0, 24 as libc::c_int - lzeros) & 0x7f as libc::c_int;
    }
    use super::opus_types_h::opus_int32;
    use super::macros_h::silk_CLZ32;
    use super::SigProc_FIX_h::silk_ROR32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "398:1"]
    pub unsafe extern "C" fn silk_ROR32(
        mut a32: opus_int32,
        mut rot: libc::c_int,
    ) -> opus_int32 {
        let mut x: opus_uint32 = a32 as opus_uint32;
        let mut r: opus_uint32 = rot as opus_uint32;
        let mut m: opus_uint32 = -rot as opus_uint32;
        if rot == 0 as libc::c_int {
            return a32
        } else if rot < 0 as libc::c_int {
            return (x << m | x >> (32 as libc::c_int as libc::c_uint).wrapping_sub(m))
                as opus_int32
        } else {
            return (x << (32 as libc::c_int as libc::c_uint).wrapping_sub(r) | x >> r)
                as opus_int32
        };
    }
    use super::opus_types_h::{opus_int32, opus_uint32};
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
pub use self::types_h::{__int16_t, __int32_t, __uint32_t, __int64_t};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_uint32, opus_int64};
pub use self::limits_h::CHAR_BIT;
pub use self::ecintrin_h::EC_CLZ0;
pub use self::macros_h::silk_CLZ32;
pub use self::Inlines_h::silk_CLZ_FRAC;
pub use self::SigProc_FIX_h::silk_ROR32;
pub use self::internal::__CHAR_BIT__;
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_lin2log(inLin: opus_int32) -> opus_int32 {
    let mut lz: opus_int32 = 0;
    let mut frac_Q7: opus_int32 = 0;
    silk_CLZ_FRAC(inLin, &mut lz, &mut frac_Q7);
    return (frac_Q7 as libc::c_long
        + ((frac_Q7 * (128 as libc::c_int - frac_Q7)) as libc::c_long
            * 179 as libc::c_int as opus_int16 as opus_int64 >> 16 as libc::c_int))
        as opus_int32
        + (((31 as libc::c_int - lz) as opus_uint32) << 7 as libc::c_int) as opus_int32;
}
