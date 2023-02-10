use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:32"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = u32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: u32,
        pub end_offs: u32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: u32,
        pub rng: u32,
        pub val: u32,
        pub ext: u32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "57:10"]
    pub const BITRES: libc::c_int = 3 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:32"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int =
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * CHAR_BIT;

    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:32"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
pub use self::ecintrin_h::EC_CLZ0;
pub use self::entcode_h::{ec_ctx, ec_window, BITRES};
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;

#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn ec_tell_frac(mut _this: *mut ec_ctx) -> u32 {
    static mut correction: [libc::c_uint; 8] = [
        35733 as libc::c_int as libc::c_uint,
        38967 as libc::c_int as libc::c_uint,
        42495 as libc::c_int as libc::c_uint,
        46340 as libc::c_int as libc::c_uint,
        50535 as libc::c_int as libc::c_uint,
        55109 as libc::c_int as libc::c_uint,
        60097 as libc::c_int as libc::c_uint,
        65535 as libc::c_int as libc::c_uint,
    ];
    let mut nbits: u32 = 0;
    let mut r: u32 = 0;
    let mut l: libc::c_int = 0;
    let mut b: libc::c_uint = 0;
    nbits = ((*_this).nbits_total << BITRES) as u32;
    l = EC_CLZ0 - ((*_this).rng).leading_zeros() as i32;
    r = (*_this).rng >> l - 16 as libc::c_int;
    b = (r >> 12 as libc::c_int).wrapping_sub(8 as libc::c_int as libc::c_uint);
    b = b.wrapping_add((r > correction[b as usize]) as libc::c_int as libc::c_uint);
    l = ((l << 3 as libc::c_int) as libc::c_uint).wrapping_add(b) as libc::c_int;
    return nbits.wrapping_sub(l as libc::c_uint);
}
