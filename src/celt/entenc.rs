use crate::celt::entcode::{celt_udiv, ec_ctx, ec_window, EC_UINT_BITS, EC_WINDOW_SIZE};

pub type ec_enc = ec_ctx;

use crate::celt::entcode::{
    EC_CODE_BITS, EC_CODE_BOT, EC_CODE_SHIFT, EC_CODE_TOP, EC_SYM_BITS, EC_SYM_MAX,
};

use crate::externs::{memmove, memset};
use crate::silk::macros::EC_CLZ0;

unsafe fn ec_write_byte(mut _this: &mut ec_enc, mut _value: u32) -> i32 {
    if (_this.offs).wrapping_add(_this.end_offs) >= _this.storage {
        return -1;
    }
    let fresh0 = _this.offs;
    _this.offs = (_this.offs).wrapping_add(1);
    *(_this.buf).offset(fresh0 as isize) = _value as u8;
    return 0;
}
unsafe fn ec_write_byte_at_end(mut _this: &mut ec_enc, mut _value: u32) -> i32 {
    if (_this.offs).wrapping_add(_this.end_offs) >= _this.storage {
        return -1;
    }
    _this.end_offs = (_this.end_offs).wrapping_add(1);
    *(_this.buf).offset((_this.storage).wrapping_sub(_this.end_offs) as isize) = _value as u8;
    return 0;
}
unsafe fn ec_enc_carry_out(mut _this: &mut ec_enc, mut _c: i32) {
    if _c as u32 != EC_SYM_MAX {
        let mut carry: i32 = 0;
        carry = _c >> EC_SYM_BITS;
        if _this.rem >= 0 {
            _this.error |= ec_write_byte(_this, (_this.rem + carry) as u32);
        }
        if _this.ext > 0 {
            let mut sym: u32 = 0;
            sym = EC_SYM_MAX.wrapping_add(carry as u32) & EC_SYM_MAX;
            loop {
                _this.error |= ec_write_byte(_this, sym);
                _this.ext = (_this.ext).wrapping_sub(1);
                if !(_this.ext > 0) {
                    break;
                }
            }
        }
        _this.rem = (_c as u32 & EC_SYM_MAX) as i32;
    } else {
        _this.ext = (_this.ext).wrapping_add(1);
    };
}
#[inline]
unsafe fn ec_enc_normalize(mut _this: &mut ec_enc) {
    while _this.rng <= EC_CODE_BOT {
        ec_enc_carry_out(_this, (_this.val >> EC_CODE_SHIFT) as i32);
        _this.val = _this.val << EC_SYM_BITS & EC_CODE_TOP.wrapping_sub(1);
        _this.rng <<= EC_SYM_BITS;
        _this.nbits_total += EC_SYM_BITS;
    }
}
pub unsafe fn ec_enc_init(this: &mut ec_enc, buf: *mut u8, size: u32) {
    this.buf = buf;
    this.end_offs = 0;
    this.end_window = 0 as ec_window;
    this.nend_bits = 0;
    this.nbits_total = EC_CODE_BITS + 1;
    this.offs = 0;
    this.rng = EC_CODE_TOP;
    this.rem = -1;
    this.val = 0;
    this.ext = 0;
    this.storage = size;
    this.error = 0;
}
pub unsafe fn ec_encode(mut _this: &mut ec_enc, mut _fl: u32, mut _fh: u32, mut _ft: u32) {
    let mut r: u32 = 0;
    r = celt_udiv(_this.rng, _ft);
    if _fl > 0 {
        _this.val = (_this.val as u32)
            .wrapping_add((_this.rng).wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fl))))
            as u32 as u32;
        _this.rng = r.wrapping_mul(_fh.wrapping_sub(_fl));
    } else {
        _this.rng =
            (_this.rng as u32).wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fh))) as u32 as u32;
    }
    ec_enc_normalize(_this);
}
pub unsafe fn ec_encode_bin(mut _this: &mut ec_enc, mut _fl: u32, mut _fh: u32, mut _bits: u32) {
    let mut r: u32 = 0;
    r = _this.rng >> _bits;
    if _fl > 0 {
        _this.val = (_this.val as u32).wrapping_add(
            (_this.rng).wrapping_sub(r.wrapping_mul(((1 as u32) << _bits).wrapping_sub(_fl))),
        ) as u32 as u32;
        _this.rng = r.wrapping_mul(_fh.wrapping_sub(_fl));
    } else {
        _this.rng = (_this.rng as u32)
            .wrapping_sub(r.wrapping_mul(((1 as u32) << _bits).wrapping_sub(_fh)))
            as u32 as u32;
    }
    ec_enc_normalize(_this);
}
pub unsafe fn ec_enc_bit_logp(mut _this: &mut ec_enc, mut _val: i32, mut _logp: u32) {
    let mut r: u32 = 0;
    let mut s: u32 = 0;
    let mut l: u32 = 0;
    r = _this.rng;
    l = _this.val;
    s = r >> _logp;
    r = (r as u32).wrapping_sub(s) as u32 as u32;
    if _val != 0 {
        _this.val = l.wrapping_add(r);
    }
    _this.rng = if _val != 0 { s } else { r };
    ec_enc_normalize(_this);
}
pub unsafe fn ec_enc_icdf(
    mut _this: &mut ec_enc,
    mut _s: i32,
    mut _icdf: *const u8,
    mut _ftb: u32,
) {
    let mut r: u32 = 0;
    r = _this.rng >> _ftb;
    if _s > 0 {
        _this.val = (_this.val as u32).wrapping_add(
            (_this.rng).wrapping_sub(r.wrapping_mul(*_icdf.offset((_s - 1) as isize) as u32)),
        ) as u32 as u32;
        _this.rng = r.wrapping_mul(
            (*_icdf.offset((_s - 1) as isize) as i32 - *_icdf.offset(_s as isize) as i32) as u32,
        );
    } else {
        _this.rng = (_this.rng as u32)
            .wrapping_sub(r.wrapping_mul(*_icdf.offset(_s as isize) as u32))
            as u32 as u32;
    }
    ec_enc_normalize(_this);
}
pub unsafe fn ec_enc_uint(mut _this: &mut ec_enc, mut _fl: u32, mut _ft: u32) {
    let mut ft: u32 = 0;
    let mut fl: u32 = 0;
    let mut ftb: i32 = 0;
    assert!(_ft > 1);
    _ft = _ft.wrapping_sub(1);
    ftb = EC_CLZ0 - _ft.leading_zeros() as i32;
    if ftb > EC_UINT_BITS {
        ftb -= EC_UINT_BITS;
        ft = (_ft >> ftb).wrapping_add(1);
        fl = _fl >> ftb;
        ec_encode(_this, fl, fl.wrapping_add(1), ft);
        ec_enc_bits(_this, _fl & (1_u32 << ftb).wrapping_sub(1), ftb as u32);
    } else {
        ec_encode(_this, _fl, _fl.wrapping_add(1), _ft.wrapping_add(1));
    };
}
pub unsafe fn ec_enc_bits(mut _this: &mut ec_enc, mut _fl: u32, mut _bits: u32) {
    let mut window: ec_window = 0;
    let mut used: i32 = 0;
    window = _this.end_window;
    used = _this.nend_bits;
    assert!(_bits > 0);
    if (used as u32).wrapping_add(_bits) > EC_WINDOW_SIZE as u32 {
        loop {
            _this.error |= ec_write_byte_at_end(_this, window & EC_SYM_MAX);
            window >>= EC_SYM_BITS;
            used -= EC_SYM_BITS;
            if !(used >= EC_SYM_BITS) {
                break;
            }
        }
    }
    window |= _fl << used;
    used = (used as u32).wrapping_add(_bits) as i32 as i32;
    _this.end_window = window;
    _this.nend_bits = used;
    _this.nbits_total = (_this.nbits_total as u32).wrapping_add(_bits) as i32 as i32;
}
pub unsafe fn ec_enc_patch_initial_bits(mut _this: &mut ec_enc, mut _val: u32, mut _nbits: u32) {
    let mut shift: i32 = 0;
    let mut mask: u32 = 0;
    assert!(_nbits <= 8);
    shift = (EC_SYM_BITS as u32).wrapping_sub(_nbits) as i32;
    mask = ((((1) << _nbits) - 1) << shift) as u32;
    if _this.offs > 0 {
        *(_this.buf).offset(0 as isize) =
            (*(_this.buf).offset(0 as isize) as u32 & !mask | _val << shift) as u8;
    } else if _this.rem >= 0 {
        _this.rem = (_this.rem as u32 & !mask | _val << shift) as i32;
    } else if _this.rng <= EC_CODE_TOP >> _nbits {
        _this.val = _this.val & !(mask << EC_CODE_SHIFT) | _val << EC_CODE_SHIFT + shift;
    } else {
        _this.error = -1;
    };
}
pub unsafe fn ec_enc_shrink(mut _this: &mut ec_enc, mut _size: u32) {
    assert!((_this.offs).wrapping_add(_this.end_offs) <= _size);
    memmove(
        (_this.buf)
            .offset(_size as isize)
            .offset(-(_this.end_offs as isize)) as *mut core::ffi::c_void,
        (_this.buf)
            .offset(_this.storage as isize)
            .offset(-(_this.end_offs as isize)) as *const core::ffi::c_void,
        (_this.end_offs as u64)
            .wrapping_mul(::core::mem::size_of::<u8>() as u64)
            .wrapping_add(
                (0 * (_this.buf)
                    .offset(_size as isize)
                    .offset(-(_this.end_offs as isize))
                    .offset_from(
                        (_this.buf)
                            .offset(_this.storage as isize)
                            .offset(-(_this.end_offs as isize)),
                    ) as i64) as u64,
            ),
    );
    _this.storage = _size;
}
pub unsafe fn ec_enc_done(mut _this: &mut ec_enc) {
    let mut window: ec_window = 0;
    let mut used: i32 = 0;
    let mut msk: u32 = 0;
    let mut end: u32 = 0;
    let mut l: i32 = 0;
    l = EC_CODE_BITS - (EC_CLZ0 - (_this.rng).leading_zeros() as i32);
    msk = EC_CODE_TOP.wrapping_sub(1) >> l;
    end = (_this.val).wrapping_add(msk) & !msk;
    if end | msk >= (_this.val).wrapping_add(_this.rng) {
        l += 1;
        msk >>= 1;
        end = (_this.val).wrapping_add(msk) & !msk;
    }
    while l > 0 {
        ec_enc_carry_out(_this, (end >> EC_CODE_SHIFT) as i32);
        end = end << EC_SYM_BITS & EC_CODE_TOP.wrapping_sub(1);
        l -= EC_SYM_BITS;
    }
    if _this.rem >= 0 || _this.ext > 0 {
        ec_enc_carry_out(_this, 0);
    }
    window = _this.end_window;
    used = _this.nend_bits;
    while used >= EC_SYM_BITS {
        _this.error |= ec_write_byte_at_end(_this, window & EC_SYM_MAX);
        window >>= EC_SYM_BITS;
        used -= EC_SYM_BITS;
    }
    if _this.error == 0 {
        memset(
            (_this.buf).offset(_this.offs as isize) as *mut core::ffi::c_void,
            0,
            ((_this.storage)
                .wrapping_sub(_this.offs)
                .wrapping_sub(_this.end_offs) as u64)
                .wrapping_mul(::core::mem::size_of::<u8>() as u64),
        );
        if used > 0 {
            if _this.end_offs >= _this.storage {
                _this.error = -1;
            } else {
                l = -l;
                if (_this.offs).wrapping_add(_this.end_offs) >= _this.storage && l < used {
                    window &= (((1) << l) - 1) as u32;
                    _this.error = -1;
                }
                let ref mut fresh1 = *(_this.buf)
                    .offset((_this.storage).wrapping_sub(_this.end_offs).wrapping_sub(1) as isize);
                *fresh1 = (*fresh1 as i32 | window as u8 as i32) as u8;
            }
        }
    }
}
