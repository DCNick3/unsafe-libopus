use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:38"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:38"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:38"]
pub mod opus_types_h {
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    use super::stdint_uintn_h::uint32_t;
}
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
pub use self::opus_types_h::opus_uint32;
pub use self::stdint_uintn_h::uint32_t;
pub use self::types_h::__uint32_t;
#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn isqrt32(mut _val: opus_uint32) -> libc::c_uint {
    let mut b: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    let mut bshift: libc::c_int = 0;
    g = 0 as libc::c_int as libc::c_uint;
    bshift = EC_CLZ0 - _val.leading_zeros() as i32 - 1 as libc::c_int >> 1 as libc::c_int;
    b = (1 as libc::c_uint) << bshift;
    loop {
        let mut t: opus_uint32 = 0;
        t = (g << 1 as libc::c_int).wrapping_add(b) << bshift;
        if t <= _val {
            g = g.wrapping_add(b);
            _val = (_val as libc::c_uint).wrapping_sub(t) as opus_uint32 as opus_uint32;
        }
        b >>= 1 as libc::c_int;
        bshift -= 1;
        if !(bshift >= 0 as libc::c_int) {
            break;
        }
    }
    return g;
}
