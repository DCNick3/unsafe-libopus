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
use crate::silk::Inlines::silk_CLZ_FRAC;

#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_lin2log(inLin: i32) -> i32 {
    let mut lz: i32 = 0;
    let mut frac_Q7: i32 = 0;
    silk_CLZ_FRAC(inLin, &mut lz, &mut frac_Q7);
    return (frac_Q7 as libc::c_long
        + ((frac_Q7 * (128 as libc::c_int - frac_Q7)) as libc::c_long
            * 179 as libc::c_int as i16 as i64
            >> 16 as libc::c_int)) as i32
        + (((31 as libc::c_int - lz) as u32) << 7 as libc::c_int) as i32;
}
