use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:38"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:38"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int =
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;

#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn isqrt32(mut _val: u32) -> libc::c_uint {
    let mut b: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    let mut bshift: libc::c_int = 0;
    g = 0 as libc::c_int as libc::c_uint;
    bshift = EC_CLZ0 - _val.leading_zeros() as i32 - 1 as libc::c_int >> 1 as libc::c_int;
    b = (1 as libc::c_uint) << bshift;
    loop {
        let mut t: u32 = 0;
        t = (g << 1 as libc::c_int).wrapping_add(b) << bshift;
        if t <= _val {
            g = g.wrapping_add(b);
            _val = (_val as libc::c_uint).wrapping_sub(t) as u32 as u32;
        }
        b >>= 1 as libc::c_int;
        bshift -= 1;
        if !(bshift >= 0 as libc::c_int) {
            break;
        }
    }
    return g;
}
