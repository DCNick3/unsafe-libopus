use crate::silk::macros::EC_CLZ0;

pub const EC_SYM_BITS: i32 = 8;
pub const EC_CODE_BITS: i32 = 32;
pub const EC_SYM_MAX: u32 = ((1 as u32) << EC_SYM_BITS).wrapping_sub(1);
pub const EC_CODE_SHIFT: i32 = EC_CODE_BITS - EC_SYM_BITS - 1;
pub const EC_CODE_TOP: u32 = (1 as u32) << EC_CODE_BITS - 1;
pub const EC_CODE_BOT: u32 = EC_CODE_TOP >> EC_SYM_BITS;
pub const EC_CODE_EXTRA: i32 = (EC_CODE_BITS - 2) % EC_SYM_BITS + 1;

pub type ec_window = u32;
#[derive(Copy, Clone)]
#[repr(C)]
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

pub const EC_UINT_BITS: i32 = 8;
pub const EC_WINDOW_SIZE: i32 = ::core::mem::size_of::<ec_window>() as i32 * 8;
pub const BITRES: i32 = 3;

#[inline]
pub unsafe fn ec_get_error(mut _this: *mut ec_ctx) -> i32 {
    return (*_this).error;
}
#[inline]
pub unsafe fn ec_range_bytes(mut _this: *mut ec_ctx) -> u32 {
    return (*_this).offs;
}
#[inline]
pub unsafe fn ec_get_buffer(mut _this: *mut ec_ctx) -> *mut u8 {
    return (*_this).buf;
}
#[inline]
pub unsafe fn ec_tell(mut _this: *mut ec_ctx) -> i32 {
    return (*_this).nbits_total - (EC_CLZ0 - ((*_this).rng).leading_zeros() as i32);
}

#[inline]
pub unsafe fn celt_udiv(n: u32, d: u32) -> u32 {
    return n.wrapping_div(d);
}

#[inline]
pub unsafe fn celt_sudiv(n: i32, d: i32) -> i32 {
    return n / d;
}

pub unsafe fn ec_tell_frac(mut _this: *mut ec_ctx) -> u32 {
    static mut correction: [u32; 8] = [35733, 38967, 42495, 46340, 50535, 55109, 60097, 65535];
    let mut nbits: u32 = 0;
    let mut r: u32 = 0;
    let mut l: i32 = 0;
    let mut b: u32 = 0;
    nbits = ((*_this).nbits_total << BITRES) as u32;
    l = EC_CLZ0 - ((*_this).rng).leading_zeros() as i32;
    r = (*_this).rng >> l - 16;
    b = (r >> 12).wrapping_sub(8);
    b = b.wrapping_add((r > correction[b as usize]) as i32 as u32);
    l = ((l << 3) as u32).wrapping_add(b) as i32;
    return nbits.wrapping_sub(l as u32);
}
