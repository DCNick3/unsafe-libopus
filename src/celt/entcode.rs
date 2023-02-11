use crate::silk::macros::EC_CLZ0;
use ::libc;

#[c2rust::src_loc = "35:10"]
pub const EC_SYM_BITS: libc::c_int = 8 as libc::c_int;
#[c2rust::src_loc = "37:10"]
pub const EC_CODE_BITS: libc::c_int = 32 as libc::c_int;
#[c2rust::src_loc = "39:10"]
pub const EC_SYM_MAX: libc::c_uint =
    ((1 as libc::c_uint) << EC_SYM_BITS).wrapping_sub(1 as libc::c_int as libc::c_uint);
#[c2rust::src_loc = "41:10"]
pub const EC_CODE_SHIFT: libc::c_int = EC_CODE_BITS - EC_SYM_BITS - 1 as libc::c_int;
#[c2rust::src_loc = "43:10"]
pub const EC_CODE_TOP: u32 = (1 as libc::c_uint) << EC_CODE_BITS - 1 as libc::c_int;
#[c2rust::src_loc = "45:10"]
pub const EC_CODE_BOT: u32 = EC_CODE_TOP >> EC_SYM_BITS;
#[c2rust::src_loc = "47:10"]
pub const EC_CODE_EXTRA: libc::c_int =
    (EC_CODE_BITS - 2 as libc::c_int) % EC_SYM_BITS + 1 as libc::c_int;

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

#[c2rust::src_loc = "53:10"]
pub const EC_UINT_BITS: libc::c_int = 8 as libc::c_int;
#[c2rust::src_loc = "50:10"]
pub const EC_WINDOW_SIZE: libc::c_int = ::core::mem::size_of::<ec_window>() as libc::c_int * 8;
#[c2rust::src_loc = "57:10"]
pub const BITRES: libc::c_int = 3 as libc::c_int;

#[inline]
#[c2rust::src_loc = "101:1"]
pub unsafe fn ec_get_error(mut _this: *mut ec_ctx) -> libc::c_int {
    return (*_this).error;
}
#[inline]
#[c2rust::src_loc = "93:1"]
pub unsafe fn ec_range_bytes(mut _this: *mut ec_ctx) -> u32 {
    return (*_this).offs;
}
#[inline]
#[c2rust::src_loc = "97:1"]
pub unsafe fn ec_get_buffer(mut _this: *mut ec_ctx) -> *mut libc::c_uchar {
    return (*_this).buf;
}
#[inline]
#[c2rust::src_loc = "111:1"]
pub unsafe fn ec_tell(mut _this: *mut ec_ctx) -> libc::c_int {
    return (*_this).nbits_total - (EC_CLZ0 - ((*_this).rng).leading_zeros() as i32);
}

#[inline]
#[c2rust::src_loc = "124:1"]
pub unsafe fn celt_udiv(n: u32, d: u32) -> u32 {
    return n.wrapping_div(d);
}

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:35"]
#[inline]
#[c2rust::src_loc = "140:1"]
pub unsafe fn celt_sudiv(n: i32, d: i32) -> i32 {
    return n / d;
}

#[c2rust::src_loc = "69:1"]
pub unsafe fn ec_tell_frac(mut _this: *mut ec_ctx) -> u32 {
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
