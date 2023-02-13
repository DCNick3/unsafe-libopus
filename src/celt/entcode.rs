use crate::silk::macros::EC_CLZ0;

#[c2rust::src_loc = "35:10"]
pub const EC_SYM_BITS: i32 = 8 as i32;
#[c2rust::src_loc = "37:10"]
pub const EC_CODE_BITS: i32 = 32 as i32;
#[c2rust::src_loc = "39:10"]
pub const EC_SYM_MAX: u32 = ((1 as u32) << EC_SYM_BITS).wrapping_sub(1 as i32 as u32);
#[c2rust::src_loc = "41:10"]
pub const EC_CODE_SHIFT: i32 = EC_CODE_BITS - EC_SYM_BITS - 1 as i32;
#[c2rust::src_loc = "43:10"]
pub const EC_CODE_TOP: u32 = (1 as u32) << EC_CODE_BITS - 1 as i32;
#[c2rust::src_loc = "45:10"]
pub const EC_CODE_BOT: u32 = EC_CODE_TOP >> EC_SYM_BITS;
#[c2rust::src_loc = "47:10"]
pub const EC_CODE_EXTRA: i32 = (EC_CODE_BITS - 2 as i32) % EC_SYM_BITS + 1 as i32;

#[c2rust::src_loc = "45:1"]
pub type ec_window = u32;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "62:8"]
pub struct ec_ctx {
    pub buf: *mut u8,
    pub storage: u32,
    pub end_offs: u32,
    pub end_window: ec_window,
    pub nend_bits: i32,
    pub nbits_total: i32,
    pub offs: u32,
    pub rng: u32,
    pub val: u32,
    pub ext: u32,
    pub rem: i32,
    pub error: i32,
}

#[c2rust::src_loc = "53:10"]
pub const EC_UINT_BITS: i32 = 8 as i32;
#[c2rust::src_loc = "50:10"]
pub const EC_WINDOW_SIZE: i32 = ::core::mem::size_of::<ec_window>() as i32 * 8;
#[c2rust::src_loc = "57:10"]
pub const BITRES: i32 = 3 as i32;

#[inline]
#[c2rust::src_loc = "101:1"]
pub unsafe fn ec_get_error(mut _this: *mut ec_ctx) -> i32 {
    return (*_this).error;
}
#[inline]
#[c2rust::src_loc = "93:1"]
pub unsafe fn ec_range_bytes(mut _this: *mut ec_ctx) -> u32 {
    return (*_this).offs;
}
#[inline]
#[c2rust::src_loc = "97:1"]
pub unsafe fn ec_get_buffer(mut _this: *mut ec_ctx) -> *mut u8 {
    return (*_this).buf;
}
#[inline]
#[c2rust::src_loc = "111:1"]
pub unsafe fn ec_tell(mut _this: *mut ec_ctx) -> i32 {
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
    static mut correction: [u32; 8] = [
        35733 as i32 as u32,
        38967 as i32 as u32,
        42495 as i32 as u32,
        46340 as i32 as u32,
        50535 as i32 as u32,
        55109 as i32 as u32,
        60097 as i32 as u32,
        65535 as i32 as u32,
    ];
    let mut nbits: u32 = 0;
    let mut r: u32 = 0;
    let mut l: i32 = 0;
    let mut b: u32 = 0;
    nbits = ((*_this).nbits_total << BITRES) as u32;
    l = EC_CLZ0 - ((*_this).rng).leading_zeros() as i32;
    r = (*_this).rng >> l - 16 as i32;
    b = (r >> 12 as i32).wrapping_sub(8 as i32 as u32);
    b = b.wrapping_add((r > correction[b as usize]) as i32 as u32);
    l = ((l << 3 as i32) as u32).wrapping_add(b) as i32;
    return nbits.wrapping_sub(l as u32);
}
