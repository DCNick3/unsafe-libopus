use crate::celt::celt::celt_fatal;
use crate::celt::entcode::{celt_udiv, ec_ctx, ec_window, EC_UINT_BITS, EC_WINDOW_SIZE};
use ::libc;

#[c2rust::src_loc = "47:1"]
pub type ec_enc = ec_ctx;
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:33"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:33"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int =
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mfrngcod.h:34"]
pub mod mfrngcod_h {
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
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}

pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
pub use self::mfrngcod_h::{
    EC_CODE_BITS, EC_CODE_BOT, EC_CODE_SHIFT, EC_CODE_TOP, EC_SYM_BITS, EC_SYM_MAX,
};

use crate::externs::{memmove, memset};
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn ec_write_byte(
    mut _this: *mut ec_enc,
    mut _value: libc::c_uint,
) -> libc::c_int {
    if ((*_this).offs).wrapping_add((*_this).end_offs) >= (*_this).storage {
        return -(1 as libc::c_int);
    }
    let fresh0 = (*_this).offs;
    (*_this).offs = ((*_this).offs).wrapping_add(1);
    *((*_this).buf).offset(fresh0 as isize) = _value as libc::c_uchar;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "66:1"]
unsafe extern "C" fn ec_write_byte_at_end(
    mut _this: *mut ec_enc,
    mut _value: libc::c_uint,
) -> libc::c_int {
    if ((*_this).offs).wrapping_add((*_this).end_offs) >= (*_this).storage {
        return -(1 as libc::c_int);
    }
    (*_this).end_offs = ((*_this).end_offs).wrapping_add(1);
    *((*_this).buf).offset(((*_this).storage).wrapping_sub((*_this).end_offs) as isize) =
        _value as libc::c_uchar;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn ec_enc_carry_out(mut _this: *mut ec_enc, mut _c: libc::c_int) {
    if _c as libc::c_uint != EC_SYM_MAX {
        let mut carry: libc::c_int = 0;
        carry = _c >> EC_SYM_BITS;
        if (*_this).rem >= 0 as libc::c_int {
            (*_this).error |= ec_write_byte(_this, ((*_this).rem + carry) as libc::c_uint);
        }
        if (*_this).ext > 0 as libc::c_int as libc::c_uint {
            let mut sym: libc::c_uint = 0;
            sym = EC_SYM_MAX.wrapping_add(carry as libc::c_uint) & EC_SYM_MAX;
            loop {
                (*_this).error |= ec_write_byte(_this, sym);
                (*_this).ext = ((*_this).ext).wrapping_sub(1);
                if !((*_this).ext > 0 as libc::c_int as libc::c_uint) {
                    break;
                }
            }
        }
        (*_this).rem = (_c as libc::c_uint & EC_SYM_MAX) as libc::c_int;
    } else {
        (*_this).ext = ((*_this).ext).wrapping_add(1);
    };
}
#[inline]
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn ec_enc_normalize(mut _this: *mut ec_enc) {
    while (*_this).rng <= EC_CODE_BOT {
        ec_enc_carry_out(_this, ((*_this).val >> EC_CODE_SHIFT) as libc::c_int);
        (*_this).val = (*_this).val << EC_SYM_BITS
            & EC_CODE_TOP.wrapping_sub(1 as libc::c_int as libc::c_uint);
        (*_this).rng <<= EC_SYM_BITS;
        (*_this).nbits_total += EC_SYM_BITS;
    }
}
#[no_mangle]
#[c2rust::src_loc = "112:1"]
pub unsafe extern "C" fn ec_enc_init(
    mut _this: *mut ec_enc,
    mut _buf: *mut libc::c_uchar,
    mut _size: u32,
) {
    (*_this).buf = _buf;
    (*_this).end_offs = 0 as libc::c_int as u32;
    (*_this).end_window = 0 as libc::c_int as ec_window;
    (*_this).nend_bits = 0 as libc::c_int;
    (*_this).nbits_total = EC_CODE_BITS + 1 as libc::c_int;
    (*_this).offs = 0 as libc::c_int as u32;
    (*_this).rng = EC_CODE_TOP;
    (*_this).rem = -(1 as libc::c_int);
    (*_this).val = 0 as libc::c_int as u32;
    (*_this).ext = 0 as libc::c_int as u32;
    (*_this).storage = _size;
    (*_this).error = 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "128:1"]
pub unsafe extern "C" fn ec_encode(
    mut _this: *mut ec_enc,
    mut _fl: libc::c_uint,
    mut _fh: libc::c_uint,
    mut _ft: libc::c_uint,
) {
    let mut r: u32 = 0;
    r = celt_udiv((*_this).rng, _ft);
    if _fl > 0 as libc::c_int as libc::c_uint {
        (*_this).val = ((*_this).val as libc::c_uint)
            .wrapping_add(((*_this).rng).wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fl))))
            as u32 as u32;
        (*_this).rng = r.wrapping_mul(_fh.wrapping_sub(_fl));
    } else {
        (*_this).rng = ((*_this).rng as libc::c_uint)
            .wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fh))) as u32
            as u32;
    }
    ec_enc_normalize(_this);
}
#[no_mangle]
#[c2rust::src_loc = "139:1"]
pub unsafe extern "C" fn ec_encode_bin(
    mut _this: *mut ec_enc,
    mut _fl: libc::c_uint,
    mut _fh: libc::c_uint,
    mut _bits: libc::c_uint,
) {
    let mut r: u32 = 0;
    r = (*_this).rng >> _bits;
    if _fl > 0 as libc::c_int as libc::c_uint {
        (*_this).val = ((*_this).val as libc::c_uint).wrapping_add(
            ((*_this).rng)
                .wrapping_sub(r.wrapping_mul(((1 as libc::c_uint) << _bits).wrapping_sub(_fl))),
        ) as u32 as u32;
        (*_this).rng = r.wrapping_mul(_fh.wrapping_sub(_fl));
    } else {
        (*_this).rng = ((*_this).rng as libc::c_uint)
            .wrapping_sub(r.wrapping_mul(((1 as libc::c_uint) << _bits).wrapping_sub(_fh)))
            as u32 as u32;
    }
    ec_enc_normalize(_this);
}
#[no_mangle]
#[c2rust::src_loc = "151:1"]
pub unsafe extern "C" fn ec_enc_bit_logp(
    mut _this: *mut ec_enc,
    mut _val: libc::c_int,
    mut _logp: libc::c_uint,
) {
    let mut r: u32 = 0;
    let mut s: u32 = 0;
    let mut l: u32 = 0;
    r = (*_this).rng;
    l = (*_this).val;
    s = r >> _logp;
    r = (r as libc::c_uint).wrapping_sub(s) as u32 as u32;
    if _val != 0 {
        (*_this).val = l.wrapping_add(r);
    }
    (*_this).rng = if _val != 0 { s } else { r };
    ec_enc_normalize(_this);
}
#[no_mangle]
#[c2rust::src_loc = "164:1"]
pub unsafe extern "C" fn ec_enc_icdf(
    mut _this: *mut ec_enc,
    mut _s: libc::c_int,
    mut _icdf: *const libc::c_uchar,
    mut _ftb: libc::c_uint,
) {
    let mut r: u32 = 0;
    r = (*_this).rng >> _ftb;
    if _s > 0 as libc::c_int {
        (*_this).val = ((*_this).val as libc::c_uint).wrapping_add(((*_this).rng).wrapping_sub(
            r.wrapping_mul(*_icdf.offset((_s - 1 as libc::c_int) as isize) as libc::c_uint),
        )) as u32 as u32;
        (*_this).rng = r.wrapping_mul(
            (*_icdf.offset((_s - 1 as libc::c_int) as isize) as libc::c_int
                - *_icdf.offset(_s as isize) as libc::c_int) as libc::c_uint,
        );
    } else {
        (*_this).rng = ((*_this).rng as libc::c_uint)
            .wrapping_sub(r.wrapping_mul(*_icdf.offset(_s as isize) as libc::c_uint))
            as u32 as u32;
    }
    ec_enc_normalize(_this);
}
#[no_mangle]
#[c2rust::src_loc = "175:1"]
pub unsafe extern "C" fn ec_enc_uint(mut _this: *mut ec_enc, mut _fl: u32, mut _ft: u32) {
    let mut ft: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    let mut ftb: libc::c_int = 0;
    if !(_ft > 1 as libc::c_int as libc::c_uint) {
        celt_fatal(
            b"assertion failed: _ft>1\0" as *const u8 as *const libc::c_char,
            b"celt/entenc.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
        );
    }
    _ft = _ft.wrapping_sub(1);
    ftb = EC_CLZ0 - _ft.leading_zeros() as i32;
    if ftb > EC_UINT_BITS {
        ftb -= EC_UINT_BITS;
        ft = (_ft >> ftb).wrapping_add(1 as libc::c_int as libc::c_uint);
        fl = _fl >> ftb;
        ec_encode(
            _this,
            fl,
            fl.wrapping_add(1 as libc::c_int as libc::c_uint),
            ft,
        );
        ec_enc_bits(
            _this,
            _fl & ((1 as libc::c_int as u32) << ftb).wrapping_sub(1 as libc::c_uint),
            ftb as libc::c_uint,
        );
    } else {
        ec_encode(
            _this,
            _fl,
            _fl.wrapping_add(1 as libc::c_int as libc::c_uint),
            _ft.wrapping_add(1 as libc::c_int as libc::c_uint),
        );
    };
}
#[no_mangle]
#[c2rust::src_loc = "193:1"]
pub unsafe extern "C" fn ec_enc_bits(
    mut _this: *mut ec_enc,
    mut _fl: u32,
    mut _bits: libc::c_uint,
) {
    let mut window: ec_window = 0;
    let mut used: libc::c_int = 0;
    window = (*_this).end_window;
    used = (*_this).nend_bits;
    if !(_bits > 0 as libc::c_int as libc::c_uint) {
        celt_fatal(
            b"assertion failed: _bits>0\0" as *const u8 as *const libc::c_char,
            b"celt/entenc.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
        );
    }
    if (used as libc::c_uint).wrapping_add(_bits) > EC_WINDOW_SIZE as libc::c_uint {
        loop {
            (*_this).error |= ec_write_byte_at_end(_this, window & EC_SYM_MAX);
            window >>= EC_SYM_BITS;
            used -= EC_SYM_BITS;
            if !(used >= EC_SYM_BITS) {
                break;
            }
        }
    }
    window |= _fl << used;
    used = (used as libc::c_uint).wrapping_add(_bits) as libc::c_int as libc::c_int;
    (*_this).end_window = window;
    (*_this).nend_bits = used;
    (*_this).nbits_total =
        ((*_this).nbits_total as libc::c_uint).wrapping_add(_bits) as libc::c_int as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "214:1"]
pub unsafe extern "C" fn ec_enc_patch_initial_bits(
    mut _this: *mut ec_enc,
    mut _val: libc::c_uint,
    mut _nbits: libc::c_uint,
) {
    let mut shift: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    if !(_nbits <= 8 as libc::c_int as libc::c_uint) {
        celt_fatal(
            b"assertion failed: _nbits<=EC_SYM_BITS\0" as *const u8 as *const libc::c_char,
            b"celt/entenc.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
        );
    }
    shift = (EC_SYM_BITS as libc::c_uint).wrapping_sub(_nbits) as libc::c_int;
    mask = ((((1 as libc::c_int) << _nbits) - 1 as libc::c_int) << shift) as libc::c_uint;
    if (*_this).offs > 0 as libc::c_int as libc::c_uint {
        *((*_this).buf).offset(0 as libc::c_int as isize) =
            (*((*_this).buf).offset(0 as libc::c_int as isize) as libc::c_uint & !mask
                | _val << shift) as libc::c_uchar;
    } else if (*_this).rem >= 0 as libc::c_int {
        (*_this).rem = ((*_this).rem as libc::c_uint & !mask | _val << shift) as libc::c_int;
    } else if (*_this).rng <= EC_CODE_TOP >> _nbits {
        (*_this).val = (*_this).val & !(mask << EC_CODE_SHIFT) | _val << EC_CODE_SHIFT + shift;
    } else {
        (*_this).error = -(1 as libc::c_int);
    };
}
#[no_mangle]
#[c2rust::src_loc = "237:1"]
pub unsafe extern "C" fn ec_enc_shrink(mut _this: *mut ec_enc, mut _size: u32) {
    if !(((*_this).offs).wrapping_add((*_this).end_offs) <= _size) {
        celt_fatal(
            b"assertion failed: _this->offs+_this->end_offs<=_size\0" as *const u8
                as *const libc::c_char,
            b"celt/entenc.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int,
        );
    }
    memmove(
        ((*_this).buf)
            .offset(_size as isize)
            .offset(-((*_this).end_offs as isize)) as *mut libc::c_void,
        ((*_this).buf)
            .offset((*_this).storage as isize)
            .offset(-((*_this).end_offs as isize)) as *const libc::c_void,
        ((*_this).end_offs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * ((*_this).buf)
                        .offset(_size as isize)
                        .offset(-((*_this).end_offs as isize))
                        .offset_from(
                            ((*_this).buf)
                                .offset((*_this).storage as isize)
                                .offset(-((*_this).end_offs as isize)),
                        ) as libc::c_long) as libc::c_ulong,
            ),
    );
    (*_this).storage = _size;
}
#[no_mangle]
#[c2rust::src_loc = "244:1"]
pub unsafe extern "C" fn ec_enc_done(mut _this: *mut ec_enc) {
    let mut window: ec_window = 0;
    let mut used: libc::c_int = 0;
    let mut msk: u32 = 0;
    let mut end: u32 = 0;
    let mut l: libc::c_int = 0;
    l = EC_CODE_BITS - (EC_CLZ0 - ((*_this).rng).leading_zeros() as i32);
    msk = EC_CODE_TOP.wrapping_sub(1 as libc::c_int as libc::c_uint) >> l;
    end = ((*_this).val).wrapping_add(msk) & !msk;
    if end | msk >= ((*_this).val).wrapping_add((*_this).rng) {
        l += 1;
        msk >>= 1 as libc::c_int;
        end = ((*_this).val).wrapping_add(msk) & !msk;
    }
    while l > 0 as libc::c_int {
        ec_enc_carry_out(_this, (end >> EC_CODE_SHIFT) as libc::c_int);
        end = end << EC_SYM_BITS & EC_CODE_TOP.wrapping_sub(1 as libc::c_int as libc::c_uint);
        l -= EC_SYM_BITS;
    }
    if (*_this).rem >= 0 as libc::c_int || (*_this).ext > 0 as libc::c_int as libc::c_uint {
        ec_enc_carry_out(_this, 0 as libc::c_int);
    }
    window = (*_this).end_window;
    used = (*_this).nend_bits;
    while used >= EC_SYM_BITS {
        (*_this).error |= ec_write_byte_at_end(_this, window & EC_SYM_MAX);
        window >>= EC_SYM_BITS;
        used -= EC_SYM_BITS;
    }
    if (*_this).error == 0 {
        memset(
            ((*_this).buf).offset((*_this).offs as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (((*_this).storage)
                .wrapping_sub((*_this).offs)
                .wrapping_sub((*_this).end_offs) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        );
        if used > 0 as libc::c_int {
            if (*_this).end_offs >= (*_this).storage {
                (*_this).error = -(1 as libc::c_int);
            } else {
                l = -l;
                if ((*_this).offs).wrapping_add((*_this).end_offs) >= (*_this).storage && l < used {
                    window &= (((1 as libc::c_int) << l) - 1 as libc::c_int) as libc::c_uint;
                    (*_this).error = -(1 as libc::c_int);
                }
                let ref mut fresh1 = *((*_this).buf).offset(
                    ((*_this).storage)
                        .wrapping_sub((*_this).end_offs)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as isize,
                );
                *fresh1 = (*fresh1 as libc::c_int | window as libc::c_uchar as libc::c_int)
                    as libc::c_uchar;
            }
        }
    }
}
