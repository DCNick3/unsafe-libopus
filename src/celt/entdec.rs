use crate::celt::entcode::{celt_udiv, ec_ctx, ec_window, EC_UINT_BITS, EC_WINDOW_SIZE};
use crate::silk::macros::EC_CLZ0;

pub type ec_dec = ec_ctx;

use crate::celt::entcode::{
    EC_CODE_BITS, EC_CODE_BOT, EC_CODE_EXTRA, EC_CODE_TOP, EC_SYM_BITS, EC_SYM_MAX,
};

unsafe fn ec_read_byte(mut _this: *mut ec_dec) -> i32 {
    return if (*_this).offs < (*_this).storage {
        let fresh0 = (*_this).offs;
        (*_this).offs = ((*_this).offs).wrapping_add(1);
        *((*_this).buf).offset(fresh0 as isize) as i32
    } else {
        0
    };
}
unsafe fn ec_read_byte_from_end(mut _this: *mut ec_dec) -> i32 {
    return if (*_this).end_offs < (*_this).storage {
        (*_this).end_offs = ((*_this).end_offs).wrapping_add(1);
        *((*_this).buf).offset(((*_this).storage).wrapping_sub((*_this).end_offs) as isize) as i32
    } else {
        0
    };
}
unsafe fn ec_dec_normalize(mut _this: *mut ec_dec) {
    while (*_this).rng <= EC_CODE_BOT {
        let mut sym: i32 = 0;
        (*_this).nbits_total += EC_SYM_BITS;
        (*_this).rng <<= EC_SYM_BITS;
        sym = (*_this).rem;
        (*_this).rem = ec_read_byte(_this);
        sym = (sym << EC_SYM_BITS | (*_this).rem) >> EC_SYM_BITS - EC_CODE_EXTRA;
        (*_this).val = ((*_this).val << EC_SYM_BITS).wrapping_add(EC_SYM_MAX & !sym as u32)
            & EC_CODE_TOP.wrapping_sub(1);
    }
}
pub unsafe fn ec_dec_init(mut _this: *mut ec_dec, mut _buf: *mut u8, mut _storage: u32) {
    (*_this).buf = _buf;
    (*_this).storage = _storage;
    (*_this).end_offs = 0;
    (*_this).end_window = 0 as ec_window;
    (*_this).nend_bits = 0;
    (*_this).nbits_total =
        EC_CODE_BITS + 1 - (EC_CODE_BITS - EC_CODE_EXTRA) / EC_SYM_BITS * EC_SYM_BITS;
    (*_this).offs = 0;
    (*_this).rng = (1 as u32) << EC_CODE_EXTRA;
    (*_this).rem = ec_read_byte(_this);
    (*_this).val = ((*_this).rng)
        .wrapping_sub(1)
        .wrapping_sub(((*_this).rem >> EC_SYM_BITS - EC_CODE_EXTRA) as u32);
    (*_this).error = 0;
    ec_dec_normalize(_this);
}
pub unsafe fn ec_decode(mut _this: *mut ec_dec, mut _ft: u32) -> u32 {
    let mut s: u32 = 0;
    (*_this).ext = celt_udiv((*_this).rng, _ft);
    s = ((*_this).val).wrapping_div((*_this).ext);
    return _ft.wrapping_sub(s.wrapping_add(1).wrapping_add(
        _ft.wrapping_sub(s.wrapping_add(1)) & -((_ft < s.wrapping_add(1)) as i32) as u32,
    ));
}
pub unsafe fn ec_decode_bin(mut _this: *mut ec_dec, mut _bits: u32) -> u32 {
    let mut s: u32 = 0;
    (*_this).ext = (*_this).rng >> _bits;
    s = ((*_this).val).wrapping_div((*_this).ext);
    return ((1 as u32) << _bits).wrapping_sub(s.wrapping_add(1 as u32).wrapping_add(
        ((1 as u32) << _bits).wrapping_sub(s.wrapping_add(1 as u32))
            & -(((1 as u32) << _bits < s.wrapping_add(1 as u32)) as i32) as u32,
    ));
}
pub unsafe fn ec_dec_update(mut _this: *mut ec_dec, mut _fl: u32, mut _fh: u32, mut _ft: u32) {
    let mut s: u32 = 0;
    s = ((*_this).ext).wrapping_mul(_ft.wrapping_sub(_fh));
    (*_this).val = ((*_this).val as u32).wrapping_sub(s) as u32 as u32;
    (*_this).rng = if _fl > 0 {
        ((*_this).ext).wrapping_mul(_fh.wrapping_sub(_fl))
    } else {
        ((*_this).rng).wrapping_sub(s)
    };
    ec_dec_normalize(_this);
}
pub unsafe fn ec_dec_bit_logp(mut _this: *mut ec_dec, mut _logp: u32) -> i32 {
    let mut r: u32 = 0;
    let mut d: u32 = 0;
    let mut s: u32 = 0;
    let mut ret: i32 = 0;
    r = (*_this).rng;
    d = (*_this).val;
    s = r >> _logp;
    ret = (d < s) as i32;
    if ret == 0 {
        (*_this).val = d.wrapping_sub(s);
    }
    (*_this).rng = if ret != 0 { s } else { r.wrapping_sub(s) };
    ec_dec_normalize(_this);
    return ret;
}
pub unsafe fn ec_dec_icdf(mut _this: *mut ec_dec, mut _icdf: *const u8, mut _ftb: u32) -> i32 {
    let mut r: u32 = 0;
    let mut d: u32 = 0;
    let mut s: u32 = 0;
    let mut t: u32 = 0;
    let mut ret: i32 = 0;
    s = (*_this).rng;
    d = (*_this).val;
    r = s >> _ftb;
    ret = -1;
    loop {
        t = s;
        ret += 1;
        s = r.wrapping_mul(*_icdf.offset(ret as isize) as u32);
        if !(d < s) {
            break;
        }
    }
    (*_this).val = d.wrapping_sub(s);
    (*_this).rng = t.wrapping_sub(s);
    ec_dec_normalize(_this);
    return ret;
}
pub unsafe fn ec_dec_uint(mut _this: *mut ec_dec, mut _ft: u32) -> u32 {
    let mut ft: u32 = 0;
    let mut s: u32 = 0;
    let mut ftb: i32 = 0;
    assert!(_ft > 1);
    _ft = _ft.wrapping_sub(1);
    ftb = EC_CLZ0 - _ft.leading_zeros() as i32;
    if ftb > EC_UINT_BITS {
        let mut t: u32 = 0;
        ftb -= EC_UINT_BITS;
        ft = (_ft >> ftb).wrapping_add(1);
        s = ec_decode(_this, ft);
        ec_dec_update(_this, s, s.wrapping_add(1), ft);
        t = s << ftb | ec_dec_bits(_this, ftb as u32);
        if t <= _ft {
            return t;
        }
        (*_this).error = 1;
        return _ft;
    } else {
        _ft = _ft.wrapping_add(1);
        s = ec_decode(_this, _ft);
        ec_dec_update(_this, s, s.wrapping_add(1), _ft);
        return s;
    };
}
pub unsafe fn ec_dec_bits(mut _this: *mut ec_dec, mut _bits: u32) -> u32 {
    let mut window: ec_window = 0;
    let mut available: i32 = 0;
    let mut ret: u32 = 0;
    window = (*_this).end_window;
    available = (*_this).nend_bits;
    if (available as u32) < _bits {
        loop {
            window |= (ec_read_byte_from_end(_this) as ec_window) << available;
            available += EC_SYM_BITS;
            if !(available <= EC_WINDOW_SIZE - EC_SYM_BITS) {
                break;
            }
        }
    }
    ret = window & (1_u32 << _bits).wrapping_sub(1);
    window >>= _bits;
    available = (available as u32).wrapping_sub(_bits) as i32 as i32;
    (*_this).end_window = window;
    (*_this).nend_bits = available;
    (*_this).nbits_total = ((*_this).nbits_total as u32).wrapping_add(_bits) as i32 as i32;
    return ret;
}
