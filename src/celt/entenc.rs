use crate::celt::entcode::{celt_udiv, ec_ctx, ec_window, EC_UINT_BITS, EC_WINDOW_SIZE};

pub type ec_enc = ec_ctx;

use crate::celt::entcode::{
    EC_CODE_BITS, EC_CODE_BOT, EC_CODE_SHIFT, EC_CODE_TOP, EC_SYM_BITS, EC_SYM_MAX,
};

use crate::externs::{memmove, memset};
use crate::silk::macros::EC_CLZ0;

unsafe fn ec_write_byte(this: &mut ec_enc, value: u32) -> i32 {
    if this.offs + this.end_offs >= this.storage {
        return -1;
    }

    *(this.buf).offset(this.offs as isize) = value as u8;
    this.offs += 1;

    0
}

unsafe fn ec_write_byte_at_end(this: &mut ec_enc, value: u32) -> i32 {
    if this.offs + this.end_offs >= this.storage {
        return -1;
    }

    this.end_offs += 1;
    *(this.buf).offset((this.storage - this.end_offs) as isize) = value as u8;

    0
}

unsafe fn ec_enc_carry_out(this: &mut ec_enc, c: i32) {
    if c as u32 != EC_SYM_MAX {
        let mut carry: i32 = 0;
        carry = c >> EC_SYM_BITS;
        if this.rem >= 0 {
            this.error |= ec_write_byte(this, (this.rem + carry) as u32);
        }
        if this.ext > 0 {
            let mut sym: u32 = 0;
            sym = EC_SYM_MAX.wrapping_add(carry as u32) & EC_SYM_MAX;
            loop {
                this.error |= ec_write_byte(this, sym);
                this.ext -= 1;
                if this.ext == 0 {
                    break;
                }
            }
        }
        this.rem = (c as u32 & EC_SYM_MAX) as i32;
    } else {
        this.ext += 1;
    };
}

#[inline]
unsafe fn ec_enc_normalize(this: &mut ec_enc) {
    while this.rng <= EC_CODE_BOT {
        ec_enc_carry_out(this, (this.val >> EC_CODE_SHIFT) as i32);
        this.val = this.val << EC_SYM_BITS & EC_CODE_TOP.wrapping_sub(1);
        this.rng <<= EC_SYM_BITS;
        this.nbits_total += EC_SYM_BITS;
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

pub unsafe fn ec_encode(this: &mut ec_enc, mut _fl: u32, mut _fh: u32, mut _ft: u32) {
    let mut r: u32 = 0;
    r = celt_udiv(this.rng, _ft);
    if _fl > 0 {
        this.val = this
            .val
            .wrapping_add((this.rng).wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fl))));
        this.rng = r.wrapping_mul(_fh.wrapping_sub(_fl));
    } else {
        this.rng = this.rng.wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fh)));
    }
    ec_enc_normalize(this);
}

pub unsafe fn ec_encode_bin(this: &mut ec_enc, mut _fl: u32, mut _fh: u32, mut _bits: u32) {
    let mut r: u32 = 0;
    r = this.rng >> _bits;
    if _fl > 0 {
        this.val = this.val.wrapping_add(
            this.rng
                .wrapping_sub(r.wrapping_mul((1_u32 << _bits).wrapping_sub(_fl))),
        );
        this.rng = r.wrapping_mul(_fh.wrapping_sub(_fl));
    } else {
        this.rng = this
            .rng
            .wrapping_sub(r.wrapping_mul((1_u32 << _bits).wrapping_sub(_fh)));
    }
    ec_enc_normalize(this);
}

pub unsafe fn ec_enc_bit_logp(this: &mut ec_enc, mut _val: i32, mut _logp: u32) {
    let mut r: u32 = 0;
    let mut s: u32 = 0;
    let mut l: u32 = 0;
    r = this.rng;
    l = this.val;
    s = r >> _logp;
    r = r.wrapping_sub(s);
    if _val != 0 {
        this.val = l.wrapping_add(r);
    }
    this.rng = if _val != 0 { s } else { r };
    ec_enc_normalize(this);
}

pub unsafe fn ec_enc_icdf(this: &mut ec_enc, mut _s: i32, mut _icdf: *const u8, mut _ftb: u32) {
    let mut r: u32 = 0;
    r = this.rng >> _ftb;
    if _s > 0 {
        this.val = this.val.wrapping_add(
            this.rng
                .wrapping_sub(r.wrapping_mul(*_icdf.offset((_s - 1) as isize) as u32)),
        );
        this.rng = r.wrapping_mul(
            (*_icdf.offset((_s - 1) as isize) as i32 - *_icdf.offset(_s as isize) as i32) as u32,
        );
    } else {
        this.rng = this
            .rng
            .wrapping_sub(r.wrapping_mul(*_icdf.offset(_s as isize) as u32));
    }
    ec_enc_normalize(this);
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

pub unsafe fn ec_enc_bits(this: &mut ec_enc, mut _fl: u32, mut _bits: u32) {
    let mut window: ec_window = 0;
    let mut used: i32 = 0;
    window = this.end_window;
    used = this.nend_bits;
    assert!(_bits > 0);
    if (used as u32).wrapping_add(_bits) > EC_WINDOW_SIZE as u32 {
        loop {
            this.error |= ec_write_byte_at_end(this, window & EC_SYM_MAX);
            window >>= EC_SYM_BITS;
            used -= EC_SYM_BITS;
            if !(used >= EC_SYM_BITS) {
                break;
            }
        }
    }
    window |= _fl << used;
    used = (used as u32).wrapping_add(_bits) as i32;
    this.end_window = window;
    this.nend_bits = used;
    this.nbits_total = (this.nbits_total as u32).wrapping_add(_bits) as i32;
}

pub unsafe fn ec_enc_patch_initial_bits(mut this: &mut ec_enc, mut _val: u32, mut _nbits: u32) {
    let mut shift: i32 = 0;
    let mut mask: u32 = 0;
    assert!(_nbits <= 8);
    shift = (EC_SYM_BITS as u32).wrapping_sub(_nbits) as i32;
    mask = ((((1) << _nbits) - 1) << shift) as u32;
    if this.offs > 0 {
        *(this.buf).offset(0 as isize) =
            (*(this.buf).offset(0 as isize) as u32 & !mask | _val << shift) as u8;
    } else if this.rem >= 0 {
        this.rem = (this.rem as u32 & !mask | _val << shift) as i32;
    } else if this.rng <= EC_CODE_TOP >> _nbits {
        this.val = this.val & !(mask << EC_CODE_SHIFT) | _val << (EC_CODE_SHIFT + shift);
    } else {
        this.error = -1;
    };
}

pub unsafe fn ec_enc_shrink(this: &mut ec_enc, mut _size: u32) {
    assert!((this.offs).wrapping_add(this.end_offs) <= _size);
    memmove(
        (this.buf)
            .offset(_size as isize)
            .offset(-(this.end_offs as isize)) as *mut core::ffi::c_void,
        (this.buf)
            .offset(this.storage as isize)
            .offset(-(this.end_offs as isize)) as *const core::ffi::c_void,
        (this.end_offs as u64)
            .wrapping_mul(::core::mem::size_of::<u8>() as u64)
            .wrapping_add(
                (0 * (this.buf)
                    .offset(_size as isize)
                    .offset(-(this.end_offs as isize))
                    .offset_from(
                        (this.buf)
                            .offset(this.storage as isize)
                            .offset(-(this.end_offs as isize)),
                    ) as i64) as u64,
            ),
    );
    this.storage = _size;
}

pub unsafe fn ec_enc_done(this: &mut ec_enc) {
    let mut window: ec_window = 0;
    let mut used: i32 = 0;
    let mut msk: u32 = 0;
    let mut end: u32 = 0;
    let mut l: i32 = 0;
    l = EC_CODE_BITS - (EC_CLZ0 - (this.rng).leading_zeros() as i32);
    msk = EC_CODE_TOP.wrapping_sub(1) >> l;
    end = (this.val).wrapping_add(msk) & !msk;
    if end | msk >= (this.val).wrapping_add(this.rng) {
        l += 1;
        msk >>= 1;
        end = (this.val).wrapping_add(msk) & !msk;
    }
    while l > 0 {
        ec_enc_carry_out(this, (end >> EC_CODE_SHIFT) as i32);
        end = end << EC_SYM_BITS & EC_CODE_TOP.wrapping_sub(1);
        l -= EC_SYM_BITS;
    }
    if this.rem >= 0 || this.ext > 0 {
        ec_enc_carry_out(this, 0);
    }
    window = this.end_window;
    used = this.nend_bits;
    while used >= EC_SYM_BITS {
        this.error |= ec_write_byte_at_end(this, window & EC_SYM_MAX);
        window >>= EC_SYM_BITS;
        used -= EC_SYM_BITS;
    }
    if this.error == 0 {
        memset(
            (this.buf).offset(this.offs as isize) as *mut core::ffi::c_void,
            0,
            ((this.storage)
                .wrapping_sub(this.offs)
                .wrapping_sub(this.end_offs) as u64)
                .wrapping_mul(::core::mem::size_of::<u8>() as u64),
        );
        if used > 0 {
            if this.end_offs >= this.storage {
                this.error = -1;
            } else {
                l = -l;
                if (this.offs).wrapping_add(this.end_offs) >= this.storage && l < used {
                    window &= (((1) << l) - 1) as u32;
                    this.error = -1;
                }
                let fresh1 = &mut (*(this.buf)
                    .offset((this.storage).wrapping_sub(this.end_offs).wrapping_sub(1) as isize));
                *fresh1 = (*fresh1 as i32 | window as u8 as i32) as u8;
            }
        }
    }
}
