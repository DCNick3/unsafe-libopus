#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = u64;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:44"]
pub mod arch_h {
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = f32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:44"]
pub mod entcode_h {
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
    #[c2rust::src_loc = "47:1"]
    pub type ec_enc = ec_ctx;
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
    #[inline]
    #[c2rust::src_loc = "124:1"]
    pub unsafe fn celt_udiv(mut n: u32, mut d: u32) -> u32 {
        return n.wrapping_div(d);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    {
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
        #[c2rust::src_loc = "356:12"]
        pub fn printf(_: *const i8, _: ...) -> i32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.c:44"]
pub mod entenc_c {
    #[c2rust::src_loc = "244:1"]
    pub unsafe fn ec_enc_done(mut _this: *mut ec_enc) {
        let mut window: ec_window = 0;
        let mut used: i32 = 0;
        let mut msk: u32 = 0;
        let mut end: u32 = 0;
        let mut l: i32 = 0;
        l = 32 as i32
            - (::core::mem::size_of::<u32>() as u64 as i32
                * 8 as i32
                - ((*_this).rng).leading_zeros() as i32);
        msk = ((1 as u32) << 32 as i32 - 1 as i32)
            .wrapping_sub(1 as i32 as u32)
            >> l;
        end = ((*_this).val).wrapping_add(msk) & !msk;
        if end | msk >= ((*_this).val).wrapping_add((*_this).rng) {
            l += 1;
            msk >>= 1 as i32;
            end = ((*_this).val).wrapping_add(msk) & !msk;
        }
        while l > 0 as i32 {
            ec_enc_carry_out(
                _this,
                (end >> 32 as i32 - 8 as i32 - 1 as i32) as i32,
            );
            end = end << 8 as i32
                & ((1 as u32) << 32 as i32 - 1 as i32)
                    .wrapping_sub(1 as i32 as u32);
            l -= 8 as i32;
        }
        if (*_this).rem >= 0 as i32 || (*_this).ext > 0 as i32 as u32 {
            ec_enc_carry_out(_this, 0 as i32);
        }
        window = (*_this).end_window;
        used = (*_this).nend_bits;
        while used >= 8 as i32 {
            (*_this).error |= ec_write_byte_at_end(
                _this,
                window
                    & ((1 as u32) << 8 as i32)
                        .wrapping_sub(1 as i32 as u32),
            );
            window >>= 8 as i32;
            used -= 8 as i32;
        }
        if (*_this).error == 0 {
            memset(
                ((*_this).buf).offset((*_this).offs as isize) as *mut core::ffi::c_void,
                0 as i32,
                (((*_this).storage)
                    .wrapping_sub((*_this).offs)
                    .wrapping_sub((*_this).end_offs) as u64)
                    .wrapping_mul(::core::mem::size_of::<u8>() as u64),
            );
            if used > 0 as i32 {
                if (*_this).end_offs >= (*_this).storage {
                    (*_this).error = -(1 as i32);
                } else {
                    l = -l;
                    if ((*_this).offs).wrapping_add((*_this).end_offs) >= (*_this).storage
                        && l < used
                    {
                        window &= (((1 as i32) << l) - 1 as i32) as u32;
                        (*_this).error = -(1 as i32);
                    }
                    let ref mut fresh0 = *((*_this).buf).offset(
                        ((*_this).storage)
                            .wrapping_sub((*_this).end_offs)
                            .wrapping_sub(1 as i32 as u32)
                            as isize,
                    );
                    *fresh0 = (*fresh0 as i32 | window as u8 as i32)
                        as u8;
                }
            }
        }
    }
    #[c2rust::src_loc = "237:1"]
    pub unsafe fn ec_enc_shrink(mut _this: *mut ec_enc, mut _size: u32) {
        memmove(
            ((*_this).buf)
                .offset(_size as isize)
                .offset(-((*_this).end_offs as isize)) as *mut core::ffi::c_void,
            ((*_this).buf)
                .offset((*_this).storage as isize)
                .offset(-((*_this).end_offs as isize)) as *const core::ffi::c_void,
            ((*_this).end_offs as u64)
                .wrapping_mul(::core::mem::size_of::<u8>() as u64)
                .wrapping_add(
                    (0 as i32 as i64
                        * ((*_this).buf)
                            .offset(_size as isize)
                            .offset(-((*_this).end_offs as isize))
                            .offset_from(
                                ((*_this).buf)
                                    .offset((*_this).storage as isize)
                                    .offset(-((*_this).end_offs as isize)),
                            ) as i64) as u64,
                ),
        );
        (*_this).storage = _size;
    }
    #[c2rust::src_loc = "214:1"]
    pub unsafe fn ec_enc_patch_initial_bits(
        mut _this: *mut ec_enc,
        mut _val: u32,
        mut _nbits: u32,
    ) {
        let mut shift: i32 = 0;
        let mut mask: u32 = 0;
        shift = (8 as i32 as u32).wrapping_sub(_nbits) as i32;
        mask = ((((1 as i32) << _nbits) - 1 as i32) << shift) as u32;
        if (*_this).offs > 0 as i32 as u32 {
            *((*_this).buf).offset(0 as i32 as isize) =
                (*((*_this).buf).offset(0 as i32 as isize) as u32 & !mask
                    | _val << shift) as u8;
        } else if (*_this).rem >= 0 as i32 {
            (*_this).rem = ((*_this).rem as u32 & !mask | _val << shift) as i32;
        } else if (*_this).rng
            <= (1 as u32) << 32 as i32 - 1 as i32 >> _nbits
        {
            (*_this).val = (*_this).val
                & !(mask << 32 as i32 - 8 as i32 - 1 as i32)
                | _val << 32 as i32 - 8 as i32 - 1 as i32 + shift;
        } else {
            (*_this).error = -(1 as i32);
        };
    }
    #[c2rust::src_loc = "66:1"]
    pub unsafe fn ec_write_byte_at_end(
        mut _this: *mut ec_enc,
        mut _value: u32,
    ) -> i32 {
        if ((*_this).offs).wrapping_add((*_this).end_offs) >= (*_this).storage {
            return -(1 as i32);
        }
        (*_this).end_offs = ((*_this).end_offs).wrapping_add(1);
        *((*_this).buf).offset(((*_this).storage).wrapping_sub((*_this).end_offs) as isize) =
            _value as u8;
        return 0 as i32;
    }
    #[c2rust::src_loc = "193:1"]
    pub unsafe fn ec_enc_bits(
        mut _this: *mut ec_enc,
        mut _fl: u32,
        mut _bits: u32,
    ) {
        let mut window: ec_window = 0;
        let mut used: i32 = 0;
        window = (*_this).end_window;
        used = (*_this).nend_bits;
        if (used as u32).wrapping_add(_bits)
            > (::core::mem::size_of::<ec_window>() as u64 as i32
                * 8 as i32) as u32
        {
            loop {
                (*_this).error |= ec_write_byte_at_end(
                    _this,
                    window
                        & ((1 as u32) << 8 as i32)
                            .wrapping_sub(1 as i32 as u32),
                );
                window >>= 8 as i32;
                used -= 8 as i32;
                if !(used >= 8 as i32) {
                    break;
                }
            }
        }
        window |= _fl << used;
        used = (used as u32).wrapping_add(_bits) as i32 as i32;
        (*_this).end_window = window;
        (*_this).nend_bits = used;
        (*_this).nbits_total = ((*_this).nbits_total as u32).wrapping_add(_bits)
            as i32 as i32;
    }
    #[c2rust::src_loc = "175:1"]
    pub unsafe fn ec_enc_uint(mut _this: *mut ec_enc, mut _fl: u32, mut _ft: u32) {
        let mut ft: u32 = 0;
        let mut fl: u32 = 0;
        let mut ftb: i32 = 0;
        _ft = _ft.wrapping_sub(1);
        ftb = ::core::mem::size_of::<u32>() as u64 as i32
            * 8 as i32
            - _ft.leading_zeros() as i32;
        if ftb > 8 as i32 {
            ftb -= 8 as i32;
            ft = (_ft >> ftb).wrapping_add(1 as i32 as u32);
            fl = _fl >> ftb;
            ec_encode(
                _this,
                fl,
                fl.wrapping_add(1 as i32 as u32),
                ft,
            );
            ec_enc_bits(
                _this,
                _fl & ((1 as i32 as u32) << ftb).wrapping_sub(1 as u32),
                ftb as u32,
            );
        } else {
            ec_encode(
                _this,
                _fl,
                _fl.wrapping_add(1 as i32 as u32),
                _ft.wrapping_add(1 as i32 as u32),
            );
        };
    }
    #[c2rust::src_loc = "164:1"]
    pub unsafe fn ec_enc_icdf(
        mut _this: *mut ec_enc,
        mut _s: i32,
        mut _icdf: *const u8,
        mut _ftb: u32,
    ) {
        let mut r: u32 = 0;
        r = (*_this).rng >> _ftb;
        if _s > 0 as i32 {
            (*_this).val = ((*_this).val as u32).wrapping_add(((*_this).rng).wrapping_sub(
                r.wrapping_mul(*_icdf.offset((_s - 1 as i32) as isize) as u32),
            )) as u32 as u32;
            (*_this).rng = r.wrapping_mul(
                (*_icdf.offset((_s - 1 as i32) as isize) as i32
                    - *_icdf.offset(_s as isize) as i32) as u32,
            );
        } else {
            (*_this).rng = ((*_this).rng as u32)
                .wrapping_sub(r.wrapping_mul(*_icdf.offset(_s as isize) as u32))
                as u32 as u32;
        }
        ec_enc_normalize(_this);
    }
    #[c2rust::src_loc = "151:1"]
    pub unsafe fn ec_enc_bit_logp(
        mut _this: *mut ec_enc,
        mut _val: i32,
        mut _logp: u32,
    ) {
        let mut r: u32 = 0;
        let mut s: u32 = 0;
        let mut l: u32 = 0;
        r = (*_this).rng;
        l = (*_this).val;
        s = r >> _logp;
        r = (r as u32).wrapping_sub(s) as u32 as u32;
        if _val != 0 {
            (*_this).val = l.wrapping_add(r);
        }
        (*_this).rng = if _val != 0 { s } else { r };
        ec_enc_normalize(_this);
    }
    #[c2rust::src_loc = "139:1"]
    pub unsafe fn ec_encode_bin(
        mut _this: *mut ec_enc,
        mut _fl: u32,
        mut _fh: u32,
        mut _bits: u32,
    ) {
        let mut r: u32 = 0;
        r = (*_this).rng >> _bits;
        if _fl > 0 as i32 as u32 {
            (*_this).val = ((*_this).val as u32).wrapping_add(
                ((*_this).rng)
                    .wrapping_sub(r.wrapping_mul(((1 as u32) << _bits).wrapping_sub(_fl))),
            ) as u32 as u32;
            (*_this).rng = r.wrapping_mul(_fh.wrapping_sub(_fl));
        } else {
            (*_this).rng = ((*_this).rng as u32)
                .wrapping_sub(r.wrapping_mul(((1 as u32) << _bits).wrapping_sub(_fh)))
                as u32 as u32;
        }
        ec_enc_normalize(_this);
    }
    #[c2rust::src_loc = "60:1"]
    pub unsafe fn ec_write_byte(
        mut _this: *mut ec_enc,
        mut _value: u32,
    ) -> i32 {
        if ((*_this).offs).wrapping_add((*_this).end_offs) >= (*_this).storage {
            return -(1 as i32);
        }
        let fresh1 = (*_this).offs;
        (*_this).offs = ((*_this).offs).wrapping_add(1);
        *((*_this).buf).offset(fresh1 as isize) = _value as u8;
        return 0 as i32;
    }
    #[c2rust::src_loc = "82:1"]
    pub unsafe fn ec_enc_carry_out(mut _this: *mut ec_enc, mut _c: i32) {
        if _c as u32
            != ((1 as u32) << 8 as i32)
                .wrapping_sub(1 as i32 as u32)
        {
            let mut carry: i32 = 0;
            carry = _c >> 8 as i32;
            if (*_this).rem >= 0 as i32 {
                (*_this).error |= ec_write_byte(_this, ((*_this).rem + carry) as u32);
            }
            if (*_this).ext > 0 as i32 as u32 {
                let mut sym: u32 = 0;
                sym = ((1 as u32) << 8 as i32)
                    .wrapping_sub(1 as i32 as u32)
                    .wrapping_add(carry as u32)
                    & ((1 as u32) << 8 as i32)
                        .wrapping_sub(1 as i32 as u32);
                loop {
                    (*_this).error |= ec_write_byte(_this, sym);
                    (*_this).ext = ((*_this).ext).wrapping_sub(1);
                    if !((*_this).ext > 0 as i32 as u32) {
                        break;
                    }
                }
            }
            (*_this).rem = (_c as u32
                & ((1 as u32) << 8 as i32)
                    .wrapping_sub(1 as i32 as u32))
                as i32;
        } else {
            (*_this).ext = ((*_this).ext).wrapping_add(1);
        };
    }
    #[inline]
    #[c2rust::src_loc = "101:1"]
    pub unsafe fn ec_enc_normalize(mut _this: *mut ec_enc) {
        while (*_this).rng
            <= (1 as u32) << 32 as i32 - 1 as i32 >> 8 as i32
        {
            ec_enc_carry_out(
                _this,
                ((*_this).val >> 32 as i32 - 8 as i32 - 1 as i32)
                    as i32,
            );
            (*_this).val = (*_this).val << 8 as i32
                & ((1 as u32) << 32 as i32 - 1 as i32)
                    .wrapping_sub(1 as i32 as u32);
            (*_this).rng <<= 8 as i32;
            (*_this).nbits_total += 8 as i32;
        }
    }
    #[c2rust::src_loc = "128:1"]
    pub unsafe fn ec_encode(
        mut _this: *mut ec_enc,
        mut _fl: u32,
        mut _fh: u32,
        mut _ft: u32,
    ) {
        let mut r: u32 = 0;
        r = celt_udiv((*_this).rng, _ft);
        if _fl > 0 as i32 as u32 {
            (*_this).val = ((*_this).val as u32)
                .wrapping_add(((*_this).rng).wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fl))))
                as u32 as u32;
            (*_this).rng = r.wrapping_mul(_fh.wrapping_sub(_fl));
        } else {
            (*_this).rng = ((*_this).rng as u32)
                .wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fh)))
                as u32 as u32;
        }
        ec_enc_normalize(_this);
    }
    #[c2rust::src_loc = "112:1"]
    pub unsafe fn ec_enc_init(
        mut _this: *mut ec_enc,
        mut _buf: *mut u8,
        mut _size: u32,
    ) {
        (*_this).buf = _buf;
        (*_this).end_offs = 0 as i32 as u32;
        (*_this).end_window = 0 as i32 as ec_window;
        (*_this).nend_bits = 0 as i32;
        (*_this).nbits_total = 32 as i32 + 1 as i32;
        (*_this).offs = 0 as i32 as u32;
        (*_this).rng = (1 as u32) << 32 as i32 - 1 as i32;
        (*_this).rem = -(1 as i32);
        (*_this).val = 0 as i32 as u32;
        (*_this).ext = 0 as i32 as u32;
        (*_this).storage = _size;
        (*_this).error = 0 as i32;
    }
    use super::entcode_h::{celt_udiv, ec_enc, ec_window};
    use crate::externs::{memmove, memset};
}
#[c2rust::header_src = "/usr/include/stdlib.h:44"]
pub mod stdlib_h {
    {
        #[c2rust::src_loc = "861:12"]
        pub fn abs(_: i32) -> i32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.c:45"]
pub mod entdec_c {
    #[c2rust::src_loc = "91:1"]
    pub unsafe fn ec_read_byte(mut _this: *mut ec_dec) -> i32 {
        return if (*_this).offs < (*_this).storage {
            let fresh2 = (*_this).offs;
            (*_this).offs = ((*_this).offs).wrapping_add(1);
            *((*_this).buf).offset(fresh2 as isize) as i32
        } else {
            0 as i32
        };
    }
    #[c2rust::src_loc = "95:1"]
    pub unsafe fn ec_read_byte_from_end(mut _this: *mut ec_dec) -> i32 {
        return if (*_this).end_offs < (*_this).storage {
            (*_this).end_offs = ((*_this).end_offs).wrapping_add(1);
            *((*_this).buf).offset(((*_this).storage).wrapping_sub((*_this).end_offs) as isize)
                as i32
        } else {
            0 as i32
        };
    }
    #[c2rust::src_loc = "102:1"]
    pub unsafe fn ec_dec_normalize(mut _this: *mut ec_dec) {
        while (*_this).rng
            <= (1 as u32) << 32 as i32 - 1 as i32 >> 8 as i32
        {
            let mut sym: i32 = 0;
            (*_this).nbits_total += 8 as i32;
            (*_this).rng <<= 8 as i32;
            sym = (*_this).rem;
            (*_this).rem = ec_read_byte(_this);
            sym = (sym << 8 as i32 | (*_this).rem)
                >> 8 as i32
                    - ((32 as i32 - 2 as i32) % 8 as i32
                        + 1 as i32);
            (*_this).val = ((*_this).val << 8 as i32).wrapping_add(
                ((1 as u32) << 8 as i32)
                    .wrapping_sub(1 as i32 as u32)
                    & !sym as u32,
            ) & ((1 as u32) << 32 as i32 - 1 as i32)
                .wrapping_sub(1 as i32 as u32);
        }
    }
    #[c2rust::src_loc = "146:1"]
    pub unsafe fn ec_decode_bin(
        mut _this: *mut ec_dec,
        mut _bits: u32,
    ) -> u32 {
        let mut s: u32 = 0;
        (*_this).ext = (*_this).rng >> _bits;
        s = ((*_this).val).wrapping_div((*_this).ext);
        return ((1 as u32) << _bits).wrapping_sub(
            s.wrapping_add(1 as u32).wrapping_add(
                ((1 as u32) << _bits).wrapping_sub(s.wrapping_add(1 as u32))
                    & -(((1 as u32) << _bits < s.wrapping_add(1 as u32))
                        as i32) as u32,
            ),
        );
    }
    #[c2rust::src_loc = "153:1"]
    pub unsafe fn ec_dec_update(
        mut _this: *mut ec_dec,
        mut _fl: u32,
        mut _fh: u32,
        mut _ft: u32,
    ) {
        let mut s: u32 = 0;
        s = ((*_this).ext).wrapping_mul(_ft.wrapping_sub(_fh));
        (*_this).val = ((*_this).val as u32).wrapping_sub(s) as u32 as u32;
        (*_this).rng = if _fl > 0 as i32 as u32 {
            ((*_this).ext).wrapping_mul(_fh.wrapping_sub(_fl))
        } else {
            ((*_this).rng).wrapping_sub(s)
        };
        ec_dec_normalize(_this);
    }
    #[c2rust::src_loc = "162:1"]
    pub unsafe fn ec_dec_bit_logp(
        mut _this: *mut ec_dec,
        mut _logp: u32,
    ) -> i32 {
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
    #[c2rust::src_loc = "177:1"]
    pub unsafe fn ec_dec_icdf(
        mut _this: *mut ec_dec,
        mut _icdf: *const u8,
        mut _ftb: u32,
    ) -> i32 {
        let mut r: u32 = 0;
        let mut d: u32 = 0;
        let mut s: u32 = 0;
        let mut t: u32 = 0;
        let mut ret: i32 = 0;
        s = (*_this).rng;
        d = (*_this).val;
        r = s >> _ftb;
        ret = -(1 as i32);
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
    #[c2rust::src_loc = "198:1"]
    pub unsafe fn ec_dec_uint(mut _this: *mut ec_dec, mut _ft: u32) -> u32 {
        let mut ft: u32 = 0;
        let mut s: u32 = 0;
        let mut ftb: i32 = 0;
        _ft = _ft.wrapping_sub(1);
        ftb = ::core::mem::size_of::<u32>() as u64 as i32
            * 8 as i32
            - _ft.leading_zeros() as i32;
        if ftb > 8 as i32 {
            let mut t: u32 = 0;
            ftb -= 8 as i32;
            ft = (_ft >> ftb).wrapping_add(1 as i32 as u32);
            s = ec_decode(_this, ft);
            ec_dec_update(
                _this,
                s,
                s.wrapping_add(1 as i32 as u32),
                ft,
            );
            t = s << ftb | ec_dec_bits(_this, ftb as u32);
            if t <= _ft {
                return t;
            }
            (*_this).error = 1 as i32;
            return _ft;
        } else {
            _ft = _ft.wrapping_add(1);
            s = ec_decode(_this, _ft);
            ec_dec_update(
                _this,
                s,
                s.wrapping_add(1 as i32 as u32),
                _ft,
            );
            return s;
        };
    }
    #[c2rust::src_loc = "225:1"]
    pub unsafe fn ec_dec_bits(mut _this: *mut ec_dec, mut _bits: u32) -> u32 {
        let mut window: ec_window = 0;
        let mut available: i32 = 0;
        let mut ret: u32 = 0;
        window = (*_this).end_window;
        available = (*_this).nend_bits;
        if (available as u32) < _bits {
            loop {
                window |= (ec_read_byte_from_end(_this) as ec_window) << available;
                available += 8 as i32;
                if !(available
                    <= ::core::mem::size_of::<ec_window>() as u64 as i32
                        * 8 as i32
                        - 8 as i32)
                {
                    break;
                }
            }
        }
        ret = window & ((1 as i32 as u32) << _bits).wrapping_sub(1 as u32);
        window >>= _bits;
        available = (available as u32).wrapping_sub(_bits) as i32 as i32;
        (*_this).end_window = window;
        (*_this).nend_bits = available;
        (*_this).nbits_total = ((*_this).nbits_total as u32).wrapping_add(_bits)
            as i32 as i32;
        return ret;
    }
    #[c2rust::src_loc = "139:1"]
    pub unsafe fn ec_decode(
        mut _this: *mut ec_dec,
        mut _ft: u32,
    ) -> u32 {
        let mut s: u32 = 0;
        (*_this).ext = celt_udiv((*_this).rng, _ft);
        s = ((*_this).val).wrapping_div((*_this).ext);
        return _ft.wrapping_sub(
            s.wrapping_add(1 as i32 as u32)
                .wrapping_add(
                    _ft.wrapping_sub(s.wrapping_add(1 as i32 as u32))
                        & -((_ft < s.wrapping_add(1 as i32 as u32)) as i32)
                            as u32,
                ),
        );
    }
    #[c2rust::src_loc = "119:1"]
    pub unsafe fn ec_dec_init(
        mut _this: *mut ec_dec,
        mut _buf: *mut u8,
        mut _storage: u32,
    ) {
        (*_this).buf = _buf;
        (*_this).storage = _storage;
        (*_this).end_offs = 0 as i32 as u32;
        (*_this).end_window = 0 as i32 as ec_window;
        (*_this).nend_bits = 0 as i32;
        (*_this).nbits_total = 32 as i32 + 1 as i32
            - (32 as i32
                - ((32 as i32 - 2 as i32) % 8 as i32 + 1 as i32))
                / 8 as i32
                * 8 as i32;
        (*_this).offs = 0 as i32 as u32;
        (*_this).rng = (1 as u32)
            << (32 as i32 - 2 as i32) % 8 as i32 + 1 as i32;
        (*_this).rem = ec_read_byte(_this);
        (*_this).val = ((*_this).rng)
            .wrapping_sub(1 as i32 as u32)
            .wrapping_sub(
                ((*_this).rem
                    >> 8 as i32
                        - ((32 as i32 - 2 as i32) % 8 as i32
                            + 1 as i32)) as u32,
            );
        (*_this).error = 0 as i32;
        ec_dec_normalize(_this);
    }
    use super::entcode_h::{celt_udiv, ec_dec, ec_window};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.c:46"]
pub mod entcode_c {
    #[c2rust::src_loc = "69:1"]
    pub unsafe fn ec_tell_frac(mut _this: *mut ec_ctx) -> u32 {
        pub static mut correction: [u32; 8] = [
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
        nbits = ((*_this).nbits_total << 3 as i32) as u32;
        l = ::core::mem::size_of::<u32>() as u64 as i32
            * 8 as i32
            - ((*_this).rng).leading_zeros() as i32;
        r = (*_this).rng >> l - 16 as i32;
        b = (r >> 12 as i32).wrapping_sub(8 as i32 as u32);
        b = b.wrapping_add((r > correction[b as usize]) as i32 as u32);
        l = ((l << 3 as i32) as u32).wrapping_add(b) as i32;
        return nbits.wrapping_sub(l as u32);
    }
    use super::entcode_h::ec_ctx;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cwrs.c:47"]
pub mod cwrs_c {
    #[c2rust::src_loc = "45:1"]
    pub unsafe fn log2_frac(mut val: u32, mut frac: i32) -> i32 {
        let mut l: i32 = 0;
        l = ::core::mem::size_of::<u32>() as u64 as i32
            * 8 as i32
            - val.leading_zeros() as i32;
        if val & val.wrapping_sub(1 as i32 as u32) != 0 {
            if l > 16 as i32 {
                val = (val.wrapping_sub(1 as i32 as u32) >> l - 16 as i32)
                    .wrapping_add(1 as i32 as u32);
            } else {
                val <<= 16 as i32 - l;
            }
            l = (l - 1 as i32) << frac;
            loop {
                let mut b: i32 = 0;
                b = (val >> 16 as i32) as i32;
                l += b << frac;
                val = val.wrapping_add(b as u32) >> b;
                val = val
                    .wrapping_mul(val)
                    .wrapping_add(0x7fff as i32 as u32)
                    >> 15 as i32;
                let fresh3 = frac;
                frac = frac - 1;
                if !(fresh3 > 0 as i32) {
                    break;
                }
            }
            return l + (val > 0x8000 as i32 as u32) as i32;
        } else {
            return (l - 1 as i32) << frac;
        };
    }
    #[c2rust::src_loc = "211:26"]
    pub static mut CELT_PVQ_U_DATA: [u32; 1488] = [
        1 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        0 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        1 as i32 as u32,
        3 as i32 as u32,
        5 as i32 as u32,
        7 as i32 as u32,
        9 as i32 as u32,
        11 as i32 as u32,
        13 as i32 as u32,
        15 as i32 as u32,
        17 as i32 as u32,
        19 as i32 as u32,
        21 as i32 as u32,
        23 as i32 as u32,
        25 as i32 as u32,
        27 as i32 as u32,
        29 as i32 as u32,
        31 as i32 as u32,
        33 as i32 as u32,
        35 as i32 as u32,
        37 as i32 as u32,
        39 as i32 as u32,
        41 as i32 as u32,
        43 as i32 as u32,
        45 as i32 as u32,
        47 as i32 as u32,
        49 as i32 as u32,
        51 as i32 as u32,
        53 as i32 as u32,
        55 as i32 as u32,
        57 as i32 as u32,
        59 as i32 as u32,
        61 as i32 as u32,
        63 as i32 as u32,
        65 as i32 as u32,
        67 as i32 as u32,
        69 as i32 as u32,
        71 as i32 as u32,
        73 as i32 as u32,
        75 as i32 as u32,
        77 as i32 as u32,
        79 as i32 as u32,
        81 as i32 as u32,
        83 as i32 as u32,
        85 as i32 as u32,
        87 as i32 as u32,
        89 as i32 as u32,
        91 as i32 as u32,
        93 as i32 as u32,
        95 as i32 as u32,
        97 as i32 as u32,
        99 as i32 as u32,
        101 as i32 as u32,
        103 as i32 as u32,
        105 as i32 as u32,
        107 as i32 as u32,
        109 as i32 as u32,
        111 as i32 as u32,
        113 as i32 as u32,
        115 as i32 as u32,
        117 as i32 as u32,
        119 as i32 as u32,
        121 as i32 as u32,
        123 as i32 as u32,
        125 as i32 as u32,
        127 as i32 as u32,
        129 as i32 as u32,
        131 as i32 as u32,
        133 as i32 as u32,
        135 as i32 as u32,
        137 as i32 as u32,
        139 as i32 as u32,
        141 as i32 as u32,
        143 as i32 as u32,
        145 as i32 as u32,
        147 as i32 as u32,
        149 as i32 as u32,
        151 as i32 as u32,
        153 as i32 as u32,
        155 as i32 as u32,
        157 as i32 as u32,
        159 as i32 as u32,
        161 as i32 as u32,
        163 as i32 as u32,
        165 as i32 as u32,
        167 as i32 as u32,
        169 as i32 as u32,
        171 as i32 as u32,
        173 as i32 as u32,
        175 as i32 as u32,
        177 as i32 as u32,
        179 as i32 as u32,
        181 as i32 as u32,
        183 as i32 as u32,
        185 as i32 as u32,
        187 as i32 as u32,
        189 as i32 as u32,
        191 as i32 as u32,
        193 as i32 as u32,
        195 as i32 as u32,
        197 as i32 as u32,
        199 as i32 as u32,
        201 as i32 as u32,
        203 as i32 as u32,
        205 as i32 as u32,
        207 as i32 as u32,
        209 as i32 as u32,
        211 as i32 as u32,
        213 as i32 as u32,
        215 as i32 as u32,
        217 as i32 as u32,
        219 as i32 as u32,
        221 as i32 as u32,
        223 as i32 as u32,
        225 as i32 as u32,
        227 as i32 as u32,
        229 as i32 as u32,
        231 as i32 as u32,
        233 as i32 as u32,
        235 as i32 as u32,
        237 as i32 as u32,
        239 as i32 as u32,
        241 as i32 as u32,
        243 as i32 as u32,
        245 as i32 as u32,
        247 as i32 as u32,
        249 as i32 as u32,
        251 as i32 as u32,
        253 as i32 as u32,
        255 as i32 as u32,
        257 as i32 as u32,
        259 as i32 as u32,
        261 as i32 as u32,
        263 as i32 as u32,
        265 as i32 as u32,
        267 as i32 as u32,
        269 as i32 as u32,
        271 as i32 as u32,
        273 as i32 as u32,
        275 as i32 as u32,
        277 as i32 as u32,
        279 as i32 as u32,
        281 as i32 as u32,
        283 as i32 as u32,
        285 as i32 as u32,
        287 as i32 as u32,
        289 as i32 as u32,
        291 as i32 as u32,
        293 as i32 as u32,
        295 as i32 as u32,
        297 as i32 as u32,
        299 as i32 as u32,
        301 as i32 as u32,
        303 as i32 as u32,
        305 as i32 as u32,
        307 as i32 as u32,
        309 as i32 as u32,
        311 as i32 as u32,
        313 as i32 as u32,
        315 as i32 as u32,
        317 as i32 as u32,
        319 as i32 as u32,
        321 as i32 as u32,
        323 as i32 as u32,
        325 as i32 as u32,
        327 as i32 as u32,
        329 as i32 as u32,
        331 as i32 as u32,
        333 as i32 as u32,
        335 as i32 as u32,
        337 as i32 as u32,
        339 as i32 as u32,
        341 as i32 as u32,
        343 as i32 as u32,
        345 as i32 as u32,
        347 as i32 as u32,
        349 as i32 as u32,
        351 as i32 as u32,
        353 as i32 as u32,
        355 as i32 as u32,
        357 as i32 as u32,
        359 as i32 as u32,
        361 as i32 as u32,
        363 as i32 as u32,
        365 as i32 as u32,
        367 as i32 as u32,
        369 as i32 as u32,
        371 as i32 as u32,
        373 as i32 as u32,
        375 as i32 as u32,
        377 as i32 as u32,
        379 as i32 as u32,
        381 as i32 as u32,
        383 as i32 as u32,
        385 as i32 as u32,
        387 as i32 as u32,
        389 as i32 as u32,
        391 as i32 as u32,
        393 as i32 as u32,
        395 as i32 as u32,
        397 as i32 as u32,
        399 as i32 as u32,
        401 as i32 as u32,
        403 as i32 as u32,
        405 as i32 as u32,
        407 as i32 as u32,
        409 as i32 as u32,
        411 as i32 as u32,
        413 as i32 as u32,
        415 as i32 as u32,
        13 as i32 as u32,
        25 as i32 as u32,
        41 as i32 as u32,
        61 as i32 as u32,
        85 as i32 as u32,
        113 as i32 as u32,
        145 as i32 as u32,
        181 as i32 as u32,
        221 as i32 as u32,
        265 as i32 as u32,
        313 as i32 as u32,
        365 as i32 as u32,
        421 as i32 as u32,
        481 as i32 as u32,
        545 as i32 as u32,
        613 as i32 as u32,
        685 as i32 as u32,
        761 as i32 as u32,
        841 as i32 as u32,
        925 as i32 as u32,
        1013 as i32 as u32,
        1105 as i32 as u32,
        1201 as i32 as u32,
        1301 as i32 as u32,
        1405 as i32 as u32,
        1513 as i32 as u32,
        1625 as i32 as u32,
        1741 as i32 as u32,
        1861 as i32 as u32,
        1985 as i32 as u32,
        2113 as i32 as u32,
        2245 as i32 as u32,
        2381 as i32 as u32,
        2521 as i32 as u32,
        2665 as i32 as u32,
        2813 as i32 as u32,
        2965 as i32 as u32,
        3121 as i32 as u32,
        3281 as i32 as u32,
        3445 as i32 as u32,
        3613 as i32 as u32,
        3785 as i32 as u32,
        3961 as i32 as u32,
        4141 as i32 as u32,
        4325 as i32 as u32,
        4513 as i32 as u32,
        4705 as i32 as u32,
        4901 as i32 as u32,
        5101 as i32 as u32,
        5305 as i32 as u32,
        5513 as i32 as u32,
        5725 as i32 as u32,
        5941 as i32 as u32,
        6161 as i32 as u32,
        6385 as i32 as u32,
        6613 as i32 as u32,
        6845 as i32 as u32,
        7081 as i32 as u32,
        7321 as i32 as u32,
        7565 as i32 as u32,
        7813 as i32 as u32,
        8065 as i32 as u32,
        8321 as i32 as u32,
        8581 as i32 as u32,
        8845 as i32 as u32,
        9113 as i32 as u32,
        9385 as i32 as u32,
        9661 as i32 as u32,
        9941 as i32 as u32,
        10225 as i32 as u32,
        10513 as i32 as u32,
        10805 as i32 as u32,
        11101 as i32 as u32,
        11401 as i32 as u32,
        11705 as i32 as u32,
        12013 as i32 as u32,
        12325 as i32 as u32,
        12641 as i32 as u32,
        12961 as i32 as u32,
        13285 as i32 as u32,
        13613 as i32 as u32,
        13945 as i32 as u32,
        14281 as i32 as u32,
        14621 as i32 as u32,
        14965 as i32 as u32,
        15313 as i32 as u32,
        15665 as i32 as u32,
        16021 as i32 as u32,
        16381 as i32 as u32,
        16745 as i32 as u32,
        17113 as i32 as u32,
        17485 as i32 as u32,
        17861 as i32 as u32,
        18241 as i32 as u32,
        18625 as i32 as u32,
        19013 as i32 as u32,
        19405 as i32 as u32,
        19801 as i32 as u32,
        20201 as i32 as u32,
        20605 as i32 as u32,
        21013 as i32 as u32,
        21425 as i32 as u32,
        21841 as i32 as u32,
        22261 as i32 as u32,
        22685 as i32 as u32,
        23113 as i32 as u32,
        23545 as i32 as u32,
        23981 as i32 as u32,
        24421 as i32 as u32,
        24865 as i32 as u32,
        25313 as i32 as u32,
        25765 as i32 as u32,
        26221 as i32 as u32,
        26681 as i32 as u32,
        27145 as i32 as u32,
        27613 as i32 as u32,
        28085 as i32 as u32,
        28561 as i32 as u32,
        29041 as i32 as u32,
        29525 as i32 as u32,
        30013 as i32 as u32,
        30505 as i32 as u32,
        31001 as i32 as u32,
        31501 as i32 as u32,
        32005 as i32 as u32,
        32513 as i32 as u32,
        33025 as i32 as u32,
        33541 as i32 as u32,
        34061 as i32 as u32,
        34585 as i32 as u32,
        35113 as i32 as u32,
        35645 as i32 as u32,
        36181 as i32 as u32,
        36721 as i32 as u32,
        37265 as i32 as u32,
        37813 as i32 as u32,
        38365 as i32 as u32,
        38921 as i32 as u32,
        39481 as i32 as u32,
        40045 as i32 as u32,
        40613 as i32 as u32,
        41185 as i32 as u32,
        41761 as i32 as u32,
        42341 as i32 as u32,
        42925 as i32 as u32,
        43513 as i32 as u32,
        44105 as i32 as u32,
        44701 as i32 as u32,
        45301 as i32 as u32,
        45905 as i32 as u32,
        46513 as i32 as u32,
        47125 as i32 as u32,
        47741 as i32 as u32,
        48361 as i32 as u32,
        48985 as i32 as u32,
        49613 as i32 as u32,
        50245 as i32 as u32,
        50881 as i32 as u32,
        51521 as i32 as u32,
        52165 as i32 as u32,
        52813 as i32 as u32,
        53465 as i32 as u32,
        54121 as i32 as u32,
        54781 as i32 as u32,
        55445 as i32 as u32,
        56113 as i32 as u32,
        56785 as i32 as u32,
        57461 as i32 as u32,
        58141 as i32 as u32,
        58825 as i32 as u32,
        59513 as i32 as u32,
        60205 as i32 as u32,
        60901 as i32 as u32,
        61601 as i32 as u32,
        62305 as i32 as u32,
        63013 as i32 as u32,
        63725 as i32 as u32,
        64441 as i32 as u32,
        65161 as i32 as u32,
        65885 as i32 as u32,
        66613 as i32 as u32,
        67345 as i32 as u32,
        68081 as i32 as u32,
        68821 as i32 as u32,
        69565 as i32 as u32,
        70313 as i32 as u32,
        71065 as i32 as u32,
        71821 as i32 as u32,
        72581 as i32 as u32,
        73345 as i32 as u32,
        74113 as i32 as u32,
        74885 as i32 as u32,
        75661 as i32 as u32,
        76441 as i32 as u32,
        77225 as i32 as u32,
        78013 as i32 as u32,
        78805 as i32 as u32,
        79601 as i32 as u32,
        80401 as i32 as u32,
        81205 as i32 as u32,
        82013 as i32 as u32,
        82825 as i32 as u32,
        83641 as i32 as u32,
        84461 as i32 as u32,
        85285 as i32 as u32,
        86113 as i32 as u32,
        63 as i32 as u32,
        129 as i32 as u32,
        231 as i32 as u32,
        377 as i32 as u32,
        575 as i32 as u32,
        833 as i32 as u32,
        1159 as i32 as u32,
        1561 as i32 as u32,
        2047 as i32 as u32,
        2625 as i32 as u32,
        3303 as i32 as u32,
        4089 as i32 as u32,
        4991 as i32 as u32,
        6017 as i32 as u32,
        7175 as i32 as u32,
        8473 as i32 as u32,
        9919 as i32 as u32,
        11521 as i32 as u32,
        13287 as i32 as u32,
        15225 as i32 as u32,
        17343 as i32 as u32,
        19649 as i32 as u32,
        22151 as i32 as u32,
        24857 as i32 as u32,
        27775 as i32 as u32,
        30913 as i32 as u32,
        34279 as i32 as u32,
        37881 as i32 as u32,
        41727 as i32 as u32,
        45825 as i32 as u32,
        50183 as i32 as u32,
        54809 as i32 as u32,
        59711 as i32 as u32,
        64897 as i32 as u32,
        70375 as i32 as u32,
        76153 as i32 as u32,
        82239 as i32 as u32,
        88641 as i32 as u32,
        95367 as i32 as u32,
        102425 as i32 as u32,
        109823 as i32 as u32,
        117569 as i32 as u32,
        125671 as i32 as u32,
        134137 as i32 as u32,
        142975 as i32 as u32,
        152193 as i32 as u32,
        161799 as i32 as u32,
        171801 as i32 as u32,
        182207 as i32 as u32,
        193025 as i32 as u32,
        204263 as i32 as u32,
        215929 as i32 as u32,
        228031 as i32 as u32,
        240577 as i32 as u32,
        253575 as i32 as u32,
        267033 as i32 as u32,
        280959 as i32 as u32,
        295361 as i32 as u32,
        310247 as i32 as u32,
        325625 as i32 as u32,
        341503 as i32 as u32,
        357889 as i32 as u32,
        374791 as i32 as u32,
        392217 as i32 as u32,
        410175 as i32 as u32,
        428673 as i32 as u32,
        447719 as i32 as u32,
        467321 as i32 as u32,
        487487 as i32 as u32,
        508225 as i32 as u32,
        529543 as i32 as u32,
        551449 as i32 as u32,
        573951 as i32 as u32,
        597057 as i32 as u32,
        620775 as i32 as u32,
        645113 as i32 as u32,
        670079 as i32 as u32,
        695681 as i32 as u32,
        721927 as i32 as u32,
        748825 as i32 as u32,
        776383 as i32 as u32,
        804609 as i32 as u32,
        833511 as i32 as u32,
        863097 as i32 as u32,
        893375 as i32 as u32,
        924353 as i32 as u32,
        956039 as i32 as u32,
        988441 as i32 as u32,
        1021567 as i32 as u32,
        1055425 as i32 as u32,
        1090023 as i32 as u32,
        1125369 as i32 as u32,
        1161471 as i32 as u32,
        1198337 as i32 as u32,
        1235975 as i32 as u32,
        1274393 as i32 as u32,
        1313599 as i32 as u32,
        1353601 as i32 as u32,
        1394407 as i32 as u32,
        1436025 as i32 as u32,
        1478463 as i32 as u32,
        1521729 as i32 as u32,
        1565831 as i32 as u32,
        1610777 as i32 as u32,
        1656575 as i32 as u32,
        1703233 as i32 as u32,
        1750759 as i32 as u32,
        1799161 as i32 as u32,
        1848447 as i32 as u32,
        1898625 as i32 as u32,
        1949703 as i32 as u32,
        2001689 as i32 as u32,
        2054591 as i32 as u32,
        2108417 as i32 as u32,
        2163175 as i32 as u32,
        2218873 as i32 as u32,
        2275519 as i32 as u32,
        2333121 as i32 as u32,
        2391687 as i32 as u32,
        2451225 as i32 as u32,
        2511743 as i32 as u32,
        2573249 as i32 as u32,
        2635751 as i32 as u32,
        2699257 as i32 as u32,
        2763775 as i32 as u32,
        2829313 as i32 as u32,
        2895879 as i32 as u32,
        2963481 as i32 as u32,
        3032127 as i32 as u32,
        3101825 as i32 as u32,
        3172583 as i32 as u32,
        3244409 as i32 as u32,
        3317311 as i32 as u32,
        3391297 as i32 as u32,
        3466375 as i32 as u32,
        3542553 as i32 as u32,
        3619839 as i32 as u32,
        3698241 as i32 as u32,
        3777767 as i32 as u32,
        3858425 as i32 as u32,
        3940223 as i32 as u32,
        4023169 as i32 as u32,
        4107271 as i32 as u32,
        4192537 as i32 as u32,
        4278975 as i32 as u32,
        4366593 as i32 as u32,
        4455399 as i32 as u32,
        4545401 as i32 as u32,
        4636607 as i32 as u32,
        4729025 as i32 as u32,
        4822663 as i32 as u32,
        4917529 as i32 as u32,
        5013631 as i32 as u32,
        5110977 as i32 as u32,
        5209575 as i32 as u32,
        5309433 as i32 as u32,
        5410559 as i32 as u32,
        5512961 as i32 as u32,
        5616647 as i32 as u32,
        5721625 as i32 as u32,
        5827903 as i32 as u32,
        5935489 as i32 as u32,
        6044391 as i32 as u32,
        6154617 as i32 as u32,
        6266175 as i32 as u32,
        6379073 as i32 as u32,
        6493319 as i32 as u32,
        6608921 as i32 as u32,
        6725887 as i32 as u32,
        6844225 as i32 as u32,
        6963943 as i32 as u32,
        7085049 as i32 as u32,
        7207551 as i32 as u32,
        7331457 as i32 as u32,
        7456775 as i32 as u32,
        7583513 as i32 as u32,
        7711679 as i32 as u32,
        7841281 as i32 as u32,
        7972327 as i32 as u32,
        8104825 as i32 as u32,
        8238783 as i32 as u32,
        8374209 as i32 as u32,
        8511111 as i32 as u32,
        8649497 as i32 as u32,
        8789375 as i32 as u32,
        8930753 as i32 as u32,
        9073639 as i32 as u32,
        9218041 as i32 as u32,
        9363967 as i32 as u32,
        9511425 as i32 as u32,
        9660423 as i32 as u32,
        9810969 as i32 as u32,
        9963071 as i32 as u32,
        10116737 as i32 as u32,
        10271975 as i32 as u32,
        10428793 as i32 as u32,
        10587199 as i32 as u32,
        10747201 as i32 as u32,
        10908807 as i32 as u32,
        11072025 as i32 as u32,
        11236863 as i32 as u32,
        11403329 as i32 as u32,
        11571431 as i32 as u32,
        11741177 as i32 as u32,
        11912575 as i32 as u32,
        321 as i32 as u32,
        681 as i32 as u32,
        1289 as i32 as u32,
        2241 as i32 as u32,
        3649 as i32 as u32,
        5641 as i32 as u32,
        8361 as i32 as u32,
        11969 as i32 as u32,
        16641 as i32 as u32,
        22569 as i32 as u32,
        29961 as i32 as u32,
        39041 as i32 as u32,
        50049 as i32 as u32,
        63241 as i32 as u32,
        78889 as i32 as u32,
        97281 as i32 as u32,
        118721 as i32 as u32,
        143529 as i32 as u32,
        172041 as i32 as u32,
        204609 as i32 as u32,
        241601 as i32 as u32,
        283401 as i32 as u32,
        330409 as i32 as u32,
        383041 as i32 as u32,
        441729 as i32 as u32,
        506921 as i32 as u32,
        579081 as i32 as u32,
        658689 as i32 as u32,
        746241 as i32 as u32,
        842249 as i32 as u32,
        947241 as i32 as u32,
        1061761 as i32 as u32,
        1186369 as i32 as u32,
        1321641 as i32 as u32,
        1468169 as i32 as u32,
        1626561 as i32 as u32,
        1797441 as i32 as u32,
        1981449 as i32 as u32,
        2179241 as i32 as u32,
        2391489 as i32 as u32,
        2618881 as i32 as u32,
        2862121 as i32 as u32,
        3121929 as i32 as u32,
        3399041 as i32 as u32,
        3694209 as i32 as u32,
        4008201 as i32 as u32,
        4341801 as i32 as u32,
        4695809 as i32 as u32,
        5071041 as i32 as u32,
        5468329 as i32 as u32,
        5888521 as i32 as u32,
        6332481 as i32 as u32,
        6801089 as i32 as u32,
        7295241 as i32 as u32,
        7815849 as i32 as u32,
        8363841 as i32 as u32,
        8940161 as i32 as u32,
        9545769 as i32 as u32,
        10181641 as i32 as u32,
        10848769 as i32 as u32,
        11548161 as i32 as u32,
        12280841 as i32 as u32,
        13047849 as i32 as u32,
        13850241 as i32 as u32,
        14689089 as i32 as u32,
        15565481 as i32 as u32,
        16480521 as i32 as u32,
        17435329 as i32 as u32,
        18431041 as i32 as u32,
        19468809 as i32 as u32,
        20549801 as i32 as u32,
        21675201 as i32 as u32,
        22846209 as i32 as u32,
        24064041 as i32 as u32,
        25329929 as i32 as u32,
        26645121 as i32 as u32,
        28010881 as i32 as u32,
        29428489 as i32 as u32,
        30899241 as i32 as u32,
        32424449 as i32 as u32,
        34005441 as i32 as u32,
        35643561 as i32 as u32,
        37340169 as i32 as u32,
        39096641 as i32 as u32,
        40914369 as i32 as u32,
        42794761 as i32 as u32,
        44739241 as i32 as u32,
        46749249 as i32 as u32,
        48826241 as i32 as u32,
        50971689 as i32 as u32,
        53187081 as i32 as u32,
        55473921 as i32 as u32,
        57833729 as i32 as u32,
        60268041 as i32 as u32,
        62778409 as i32 as u32,
        65366401 as i32 as u32,
        68033601 as i32 as u32,
        70781609 as i32 as u32,
        73612041 as i32 as u32,
        76526529 as i32 as u32,
        79526721 as i32 as u32,
        82614281 as i32 as u32,
        85790889 as i32 as u32,
        89058241 as i32 as u32,
        92418049 as i32 as u32,
        95872041 as i32 as u32,
        99421961 as i32 as u32,
        103069569 as i32 as u32,
        106816641 as i32 as u32,
        110664969 as i32 as u32,
        114616361 as i32 as u32,
        118672641 as i32 as u32,
        122835649 as i32 as u32,
        127107241 as i32 as u32,
        131489289 as i32 as u32,
        135983681 as i32 as u32,
        140592321 as i32 as u32,
        145317129 as i32 as u32,
        150160041 as i32 as u32,
        155123009 as i32 as u32,
        160208001 as i32 as u32,
        165417001 as i32 as u32,
        170752009 as i32 as u32,
        176215041 as i32 as u32,
        181808129 as i32 as u32,
        187533321 as i32 as u32,
        193392681 as i32 as u32,
        199388289 as i32 as u32,
        205522241 as i32 as u32,
        211796649 as i32 as u32,
        218213641 as i32 as u32,
        224775361 as i32 as u32,
        231483969 as i32 as u32,
        238341641 as i32 as u32,
        245350569 as i32 as u32,
        252512961 as i32 as u32,
        259831041 as i32 as u32,
        267307049 as i32 as u32,
        274943241 as i32 as u32,
        282741889 as i32 as u32,
        290705281 as i32 as u32,
        298835721 as i32 as u32,
        307135529 as i32 as u32,
        315607041 as i32 as u32,
        324252609 as i32 as u32,
        333074601 as i32 as u32,
        342075401 as i32 as u32,
        351257409 as i32 as u32,
        360623041 as i32 as u32,
        370174729 as i32 as u32,
        379914921 as i32 as u32,
        389846081 as i32 as u32,
        399970689 as i32 as u32,
        410291241 as i32 as u32,
        420810249 as i32 as u32,
        431530241 as i32 as u32,
        442453761 as i32 as u32,
        453583369 as i32 as u32,
        464921641 as i32 as u32,
        476471169 as i32 as u32,
        488234561 as i32 as u32,
        500214441 as i32 as u32,
        512413449 as i32 as u32,
        524834241 as i32 as u32,
        537479489 as i32 as u32,
        550351881 as i32 as u32,
        563454121 as i32 as u32,
        576788929 as i32 as u32,
        590359041 as i32 as u32,
        604167209 as i32 as u32,
        618216201 as i32 as u32,
        632508801 as i32 as u32,
        647047809 as i32 as u32,
        661836041 as i32 as u32,
        676876329 as i32 as u32,
        692171521 as i32 as u32,
        707724481 as i32 as u32,
        723538089 as i32 as u32,
        739615241 as i32 as u32,
        755958849 as i32 as u32,
        772571841 as i32 as u32,
        789457161 as i32 as u32,
        806617769 as i32 as u32,
        824056641 as i32 as u32,
        841776769 as i32 as u32,
        859781161 as i32 as u32,
        878072841 as i32 as u32,
        896654849 as i32 as u32,
        915530241 as i32 as u32,
        934702089 as i32 as u32,
        954173481 as i32 as u32,
        973947521 as i32 as u32,
        994027329 as i32 as u32,
        1014416041 as i32 as u32,
        1035116809 as i32 as u32,
        1056132801 as i32 as u32,
        1077467201 as i32 as u32,
        1099123209 as i32 as u32,
        1121104041 as i32 as u32,
        1143412929 as i32 as u32,
        1166053121 as i32 as u32,
        1189027881 as i32 as u32,
        1212340489 as i32 as u32,
        1235994241 as i32 as u32,
        1683 as i32 as u32,
        3653 as i32 as u32,
        7183 as i32 as u32,
        13073 as i32 as u32,
        22363 as i32 as u32,
        36365 as i32 as u32,
        56695 as i32 as u32,
        85305 as i32 as u32,
        124515 as i32 as u32,
        177045 as i32 as u32,
        246047 as i32 as u32,
        335137 as i32 as u32,
        448427 as i32 as u32,
        590557 as i32 as u32,
        766727 as i32 as u32,
        982729 as i32 as u32,
        1244979 as i32 as u32,
        1560549 as i32 as u32,
        1937199 as i32 as u32,
        2383409 as i32 as u32,
        2908411 as i32 as u32,
        3522221 as i32 as u32,
        4235671 as i32 as u32,
        5060441 as i32 as u32,
        6009091 as i32 as u32,
        7095093 as i32 as u32,
        8332863 as i32 as u32,
        9737793 as i32 as u32,
        11326283 as i32 as u32,
        13115773 as i32 as u32,
        15124775 as i32 as u32,
        17372905 as i32 as u32,
        19880915 as i32 as u32,
        22670725 as i32 as u32,
        25765455 as i32 as u32,
        29189457 as i32 as u32,
        32968347 as i32 as u32,
        37129037 as i32 as u32,
        41699767 as i32 as u32,
        46710137 as i32 as u32,
        52191139 as i32 as u32,
        58175189 as i32 as u32,
        64696159 as i32 as u32,
        71789409 as i32 as u32,
        79491819 as i32 as u32,
        87841821 as i32 as u32,
        96879431 as i32 as u32,
        106646281 as i32 as u32,
        117185651 as i32 as u32,
        128542501 as i32 as u32,
        140763503 as i32 as u32,
        153897073 as i32 as u32,
        167993403 as i32 as u32,
        183104493 as i32 as u32,
        199284183 as i32 as u32,
        216588185 as i32 as u32,
        235074115 as i32 as u32,
        254801525 as i32 as u32,
        275831935 as i32 as u32,
        298228865 as i32 as u32,
        322057867 as i32 as u32,
        347386557 as i32 as u32,
        374284647 as i32 as u32,
        402823977 as i32 as u32,
        433078547 as i32 as u32,
        465124549 as i32 as u32,
        499040399 as i32 as u32,
        534906769 as i32 as u32,
        572806619 as i32 as u32,
        612825229 as i32 as u32,
        655050231 as i32 as u32,
        699571641 as i32 as u32,
        746481891 as i32 as u32,
        795875861 as i32 as u32,
        847850911 as i32 as u32,
        902506913 as i32 as u32,
        959946283 as i32 as u32,
        1020274013 as i32 as u32,
        1083597703 as i32 as u32,
        1150027593 as i32 as u32,
        1219676595 as i32 as u32,
        1292660325 as i32 as u32,
        1369097135 as i32 as u32,
        1449108145 as i32 as u32,
        1532817275 as i32 as u32,
        1620351277 as i32 as u32,
        1711839767 as i32 as u32,
        1807415257 as i32 as u32,
        1907213187 as i32 as u32,
        2011371957 as i32 as u32,
        2120032959 as i32 as u32,
        2233340609 as u32,
        2351442379 as u32,
        2474488829 as u32,
        2602633639 as u32,
        2736033641 as u32,
        2874848851 as u32,
        3019242501 as u32,
        3169381071 as u32,
        3325434321 as u32,
        3487575323 as u32,
        3655980493 as u32,
        3830829623 as u32,
        4012305913 as u32,
        8989 as i32 as u32,
        19825 as i32 as u32,
        40081 as i32 as u32,
        75517 as i32 as u32,
        134245 as i32 as u32,
        227305 as i32 as u32,
        369305 as i32 as u32,
        579125 as i32 as u32,
        880685 as i32 as u32,
        1303777 as i32 as u32,
        1884961 as i32 as u32,
        2668525 as i32 as u32,
        3707509 as i32 as u32,
        5064793 as i32 as u32,
        6814249 as i32 as u32,
        9041957 as i32 as u32,
        11847485 as i32 as u32,
        15345233 as i32 as u32,
        19665841 as i32 as u32,
        24957661 as i32 as u32,
        31388293 as i32 as u32,
        39146185 as i32 as u32,
        48442297 as i32 as u32,
        59511829 as i32 as u32,
        72616013 as i32 as u32,
        88043969 as i32 as u32,
        106114625 as i32 as u32,
        127178701 as i32 as u32,
        151620757 as i32 as u32,
        179861305 as i32 as u32,
        212358985 as i32 as u32,
        249612805 as i32 as u32,
        292164445 as i32 as u32,
        340600625 as i32 as u32,
        395555537 as i32 as u32,
        457713341 as i32 as u32,
        527810725 as i32 as u32,
        606639529 as i32 as u32,
        695049433 as i32 as u32,
        793950709 as i32 as u32,
        904317037 as i32 as u32,
        1027188385 as i32 as u32,
        1163673953 as i32 as u32,
        1314955181 as i32 as u32,
        1482288821 as i32 as u32,
        1667010073 as i32 as u32,
        1870535785 as i32 as u32,
        2094367717 as i32 as u32,
        2340095869 as u32,
        2609401873 as u32,
        2904062449 as u32,
        3225952925 as u32,
        3577050821 as u32,
        3959439497 as u32,
        48639 as i32 as u32,
        108545 as i32 as u32,
        224143 as i32 as u32,
        433905 as i32 as u32,
        795455 as i32 as u32,
        1392065 as i32 as u32,
        2340495 as i32 as u32,
        3800305 as i32 as u32,
        5984767 as i32 as u32,
        9173505 as i32 as u32,
        13726991 as i32 as u32,
        20103025 as i32 as u32,
        28875327 as i32 as u32,
        40754369 as i32 as u32,
        56610575 as i32 as u32,
        77500017 as i32 as u32,
        104692735 as i32 as u32,
        139703809 as i32 as u32,
        184327311 as i32 as u32,
        240673265 as i32 as u32,
        311207743 as i32 as u32,
        398796225 as i32 as u32,
        506750351 as i32 as u32,
        638878193 as i32 as u32,
        799538175 as i32 as u32,
        993696769 as i32 as u32,
        1226990095 as i32 as u32,
        1505789553 as i32 as u32,
        1837271615 as i32 as u32,
        2229491905 as u32,
        2691463695 as u32,
        3233240945 as u32,
        3866006015 as u32,
        265729 as i32 as u32,
        598417 as i32 as u32,
        1256465 as i32 as u32,
        2485825 as i32 as u32,
        4673345 as i32 as u32,
        8405905 as i32 as u32,
        14546705 as i32 as u32,
        24331777 as i32 as u32,
        39490049 as i32 as u32,
        62390545 as i32 as u32,
        96220561 as i32 as u32,
        145198913 as i32 as u32,
        214828609 as i32 as u32,
        312193553 as i32 as u32,
        446304145 as i32 as u32,
        628496897 as i32 as u32,
        872893441 as i32 as u32,
        1196924561 as i32 as u32,
        1621925137 as i32 as u32,
        2173806145 as u32,
        2883810113 as u32,
        1462563 as i32 as u32,
        3317445 as i32 as u32,
        7059735 as i32 as u32,
        14218905 as i32 as u32,
        27298155 as i32 as u32,
        50250765 as i32 as u32,
        89129247 as i32 as u32,
        152951073 as i32 as u32,
        254831667 as i32 as u32,
        413442773 as i32 as u32,
        654862247 as i32 as u32,
        1014889769 as i32 as u32,
        1541911931 as i32 as u32,
        2300409629 as u32,
        3375210671 as u32,
        8097453 as i32 as u32,
        18474633 as i32 as u32,
        39753273 as i32 as u32,
        81270333 as i32 as u32,
        158819253 as i32 as u32,
        298199265 as i32 as u32,
        540279585 as i32 as u32,
        948062325 as i32 as u32,
        1616336765 as i32 as u32,
        2684641785 as u32,
        45046719 as i32 as u32,
        103274625 as i32 as u32,
        224298231 as i32 as u32,
        464387817 as i32 as u32,
        921406335 as i32 as u32,
        1759885185 as i32 as u32,
        3248227095 as u32,
        251595969 as i32 as u32,
        579168825 as i32 as u32,
        1267854873 as i32 as u32,
        2653649025 as u32,
        1409933619 as i32 as u32,
    ];
    #[c2rust::src_loc = "413:33"]
    pub static mut CELT_PVQ_U_ROW: [*const u32; 15] = [0 as *const u32; 15];
    #[c2rust::src_loc = "431:1"]
    pub unsafe fn get_required_bits(
        mut _bits: *mut i16,
        mut _n: i32,
        mut _maxk: i32,
        mut _frac: i32,
    ) {
        let mut k: i32 = 0;
        *_bits.offset(0 as i32 as isize) = 0 as i32 as i16;
        k = 1 as i32;
        while k <= _maxk {
            *_bits.offset(k as isize) = log2_frac(
                (*(CELT_PVQ_U_ROW[(if _n < k { _n } else { k }) as usize])
                    .offset((if _n > k { _n } else { k }) as isize))
                .wrapping_add(
                    *(CELT_PVQ_U_ROW[(if _n < k + 1 as i32 {
                        _n
                    } else {
                        k + 1 as i32
                    }) as usize])
                        .offset(
                            (if _n > k + 1 as i32 {
                                _n
                            } else {
                                k + 1 as i32
                            }) as isize,
                        ),
                ),
                _frac,
            ) as i16;
            k += 1;
        }
    }
    #[c2rust::src_loc = "440:1"]
    pub unsafe fn icwrs(mut _n: i32, mut _y: *const i32) -> u32 {
        let mut i: u32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        j = _n - 1 as i32;
        i = (*_y.offset(j as isize) < 0 as i32) as i32 as u32;
        k = abs(*_y.offset(j as isize));
        loop {
            j -= 1;
            i = (i as u32).wrapping_add(
                *(CELT_PVQ_U_ROW[(if _n - j < k { _n - j } else { k }) as usize])
                    .offset((if _n - j > k { _n - j } else { k }) as isize),
            ) as u32 as u32;
            k += abs(*_y.offset(j as isize));
            if *_y.offset(j as isize) < 0 as i32 {
                i = (i as u32).wrapping_add(
                    *(CELT_PVQ_U_ROW[(if _n - j < k + 1 as i32 {
                        _n - j
                    } else {
                        k + 1 as i32
                    }) as usize])
                        .offset(
                            (if _n - j > k + 1 as i32 {
                                _n - j
                            } else {
                                k + 1 as i32
                            }) as isize,
                        ),
                ) as u32 as u32;
            }
            if !(j > 0 as i32) {
                break;
            }
        }
        return i;
    }
    #[c2rust::src_loc = "458:1"]
    pub unsafe fn encode_pulses(
        mut _y: *const i32,
        mut _n: i32,
        mut _k: i32,
        mut _enc: *mut ec_enc,
    ) {
        ec_enc_uint(
            _enc,
            icwrs(_n, _y),
            (*(CELT_PVQ_U_ROW[(if _n < _k { _n } else { _k }) as usize])
                .offset((if _n > _k { _n } else { _k }) as isize))
            .wrapping_add(
                *(CELT_PVQ_U_ROW[(if _n < _k + 1 as i32 {
                    _n
                } else {
                    _k + 1 as i32
                }) as usize])
                    .offset(
                        (if _n > _k + 1 as i32 {
                            _n
                        } else {
                            _k + 1 as i32
                        }) as isize,
                    ),
            ),
        );
    }
    #[c2rust::src_loc = "463:1"]
    pub unsafe fn cwrsi(
        mut _n: i32,
        mut _k: i32,
        mut _i: u32,
        mut _y: *mut i32,
    ) -> opus_val32 {
        let mut p: u32 = 0;
        let mut s: i32 = 0;
        let mut k0: i32 = 0;
        let mut val: i16 = 0;
        let mut yy: opus_val32 = 0 as i32 as opus_val32;
        while _n > 2 as i32 {
            let mut q: u32 = 0;
            if _k >= _n {
                let mut row: *const u32 = 0 as *const u32;
                row = CELT_PVQ_U_ROW[_n as usize];
                p = *row.offset((_k + 1 as i32) as isize);
                s = -((_i >= p) as i32);
                _i = (_i as u32).wrapping_sub(p & s as u32) as u32 as u32;
                k0 = _k;
                q = *row.offset(_n as isize);
                if q > _i {
                    _k = _n;
                    loop {
                        _k -= 1;
                        p = *(CELT_PVQ_U_ROW[_k as usize]).offset(_n as isize);
                        if !(p > _i) {
                            break;
                        }
                    }
                } else {
                    p = *row.offset(_k as isize);
                    while p > _i {
                        _k -= 1;
                        p = *row.offset(_k as isize);
                    }
                }
                _i = (_i as u32).wrapping_sub(p) as u32 as u32;
                val = (k0 - _k + s ^ s) as i16;
                let fresh4 = _y;
                _y = _y.offset(1);
                *fresh4 = val as i32;
                yy = yy + val as opus_val32 * val as opus_val32;
            } else {
                p = *(CELT_PVQ_U_ROW[_k as usize]).offset(_n as isize);
                q = *(CELT_PVQ_U_ROW[(_k + 1 as i32) as usize]).offset(_n as isize);
                if p <= _i && _i < q {
                    _i = (_i as u32).wrapping_sub(p) as u32 as u32;
                    let fresh5 = _y;
                    _y = _y.offset(1);
                    *fresh5 = 0 as i32;
                } else {
                    s = -((_i >= q) as i32);
                    _i = (_i as u32).wrapping_sub(q & s as u32) as u32 as u32;
                    k0 = _k;
                    loop {
                        _k -= 1;
                        p = *(CELT_PVQ_U_ROW[_k as usize]).offset(_n as isize);
                        if !(p > _i) {
                            break;
                        }
                    }
                    _i = (_i as u32).wrapping_sub(p) as u32 as u32;
                    val = (k0 - _k + s ^ s) as i16;
                    let fresh6 = _y;
                    _y = _y.offset(1);
                    *fresh6 = val as i32;
                    yy = yy + val as opus_val32 * val as opus_val32;
                }
            }
            _n -= 1;
        }
        p = (2 as i32 * _k + 1 as i32) as u32;
        s = -((_i >= p) as i32);
        _i = (_i as u32).wrapping_sub(p & s as u32) as u32 as u32;
        k0 = _k;
        _k = (_i.wrapping_add(1 as i32 as u32) >> 1 as i32) as i32;
        if _k != 0 {
            _i = (_i as u32)
                .wrapping_sub((2 as i32 * _k - 1 as i32) as u32)
                as u32 as u32;
        }
        val = (k0 - _k + s ^ s) as i16;
        let fresh7 = _y;
        _y = _y.offset(1);
        *fresh7 = val as i32;
        yy = yy + val as opus_val32 * val as opus_val32;
        s = -(_i as i32);
        val = (_k + s ^ s) as i16;
        *_y = val as i32;
        yy = yy + val as opus_val32 * val as opus_val32;
        return yy;
    }
    #[c2rust::src_loc = "539:1"]
    pub unsafe fn decode_pulses(
        mut _y: *mut i32,
        mut _n: i32,
        mut _k: i32,
        mut _dec: *mut ec_dec,
    ) -> opus_val32 {
        return cwrsi(
            _n,
            _k,
            ec_dec_uint(
                _dec,
                (*(CELT_PVQ_U_ROW[(if _n < _k { _n } else { _k }) as usize])
                    .offset((if _n > _k { _n } else { _k }) as isize))
                .wrapping_add(
                    *(CELT_PVQ_U_ROW[(if _n < _k + 1 as i32 {
                        _n
                    } else {
                        _k + 1 as i32
                    }) as usize])
                        .offset(
                            (if _n > _k + 1 as i32 {
                                _n
                            } else {
                                _k + 1 as i32
                            }) as isize,
                        ),
                ),
            ),
            _y,
        );
    }
    use super::arch_h::opus_val32;
    use super::entcode_h::{ec_dec, ec_enc};
    use super::entdec_c::ec_dec_uint;
    use super::entenc_c::ec_enc_uint;
    use super::stdlib_h::abs;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mathops.c:48"]
pub mod mathops_c {
    #[c2rust::src_loc = "43:1"]
    pub unsafe fn isqrt32(mut _val: u32) -> u32 {
        let mut b: u32 = 0;
        let mut g: u32 = 0;
        let mut bshift: i32 = 0;
        g = 0 as i32 as u32;
        bshift = ::core::mem::size_of::<u32>() as u64 as i32
            * 8 as i32
            - _val.leading_zeros() as i32
            - 1 as i32
            >> 1 as i32;
        b = (1 as u32) << bshift;
        loop {
            let mut t: u32 = 0;
            t = (g << 1 as i32).wrapping_add(b) << bshift;
            if t <= _val {
                g = g.wrapping_add(b);
                _val = (_val as u32).wrapping_sub(t) as u32 as u32;
            }
            b >>= 1 as i32;
            bshift -= 1;
            if !(bshift >= 0 as i32) {
                break;
            }
        }
        return g;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/rate.h:49"]
pub mod rate_h {
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe fn get_pulses(mut i: i32) -> i32 {
        return if i < 8 as i32 {
            i
        } else {
            8 as i32 + (i & 7 as i32) << (i >> 3 as i32) - 1 as i32
        };
    }
}
pub use self::arch_h::opus_val32;
pub use self::cwrs_c::{
    cwrsi, decode_pulses, encode_pulses, get_required_bits, icwrs, log2_frac, CELT_PVQ_U_DATA,
    CELT_PVQ_U_ROW,
};
pub use self::entcode_c::ec_tell_frac;
pub use self::entcode_h::{celt_udiv, ec_ctx, ec_dec, ec_enc, ec_window};
pub use self::entdec_c::{
    ec_dec_bit_logp, ec_dec_bits, ec_dec_icdf, ec_dec_init, ec_dec_normalize, ec_dec_uint,
    ec_dec_update, ec_decode, ec_decode_bin, ec_read_byte, ec_read_byte_from_end,
};
pub use self::entenc_c::{
    ec_enc_bit_logp, ec_enc_bits, ec_enc_carry_out, ec_enc_done, ec_enc_icdf, ec_enc_init,
    ec_enc_normalize, ec_enc_patch_initial_bits, ec_enc_shrink, ec_enc_uint, ec_encode,
    ec_encode_bin, ec_write_byte, ec_write_byte_at_end,
};
pub use self::mathops_c::isqrt32;
pub use self::rate_h::get_pulses;
pub use self::stddef_h::size_t;

use self::stdio_h::{fprintf, printf, stderr};
use self::stdlib_h::abs;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};

pub use self::FILE_h::FILE;
use crate::externs::{memmove, memset};
#[c2rust::src_loc = "75:18"]
static mut pn: [i32; 22] = [
    2 as i32,
    3 as i32,
    4 as i32,
    6 as i32,
    8 as i32,
    9 as i32,
    11 as i32,
    12 as i32,
    16 as i32,
    18 as i32,
    22 as i32,
    24 as i32,
    32 as i32,
    36 as i32,
    44 as i32,
    48 as i32,
    64 as i32,
    72 as i32,
    88 as i32,
    96 as i32,
    144 as i32,
    176 as i32,
];
#[c2rust::src_loc = "80:18"]
static mut pkmax: [i32; 22] = [
    128 as i32,
    128 as i32,
    128 as i32,
    88 as i32,
    36 as i32,
    26 as i32,
    18 as i32,
    16 as i32,
    12 as i32,
    11 as i32,
    9 as i32,
    9 as i32,
    7 as i32,
    7 as i32,
    6 as i32,
    6 as i32,
    5 as i32,
    5 as i32,
    5 as i32,
    5 as i32,
    4 as i32,
    4 as i32,
];
#[c2rust::src_loc = "88:1"]
unsafe fn main_0() -> i32 {
    let mut t: i32 = 0;
    let mut n: i32 = 0;
    t = 0 as i32;
    while t < 22 as i32 {
        let mut pseudo: i32 = 0;
        n = pn[t as usize];
        pseudo = 1 as i32;
        while pseudo < 41 as i32 {
            let mut k: i32 = 0;
            let mut inc: u32 = 0;
            let mut nc: u32 = 0;
            let mut i: u32 = 0;
            k = get_pulses(pseudo);
            if k > pkmax[t as usize] {
                break;
            }
            printf(
                b"Testing CWRS with N=%i, K=%i...\n\0" as *const u8 as *const i8,
                n,
                k,
            );
            nc = (*(CELT_PVQ_U_ROW[(if n < k { n } else { k }) as usize])
                .offset((if n > k { n } else { k }) as isize))
            .wrapping_add(
                *(CELT_PVQ_U_ROW[(if n < k + 1 as i32 {
                    n
                } else {
                    k + 1 as i32
                }) as usize])
                    .offset(
                        (if n > k + 1 as i32 {
                            n
                        } else {
                            k + 1 as i32
                        }) as isize,
                    ),
            );
            inc = nc.wrapping_div(20000 as i32 as u32);
            if inc < 1 as i32 as u32 {
                inc = 1 as i32 as u32;
            }
            i = 0 as i32 as u32;
            while i < nc {
                let mut y: [i32; 240] = [0; 240];
                let mut sy: i32 = 0;
                let mut v: u32 = 0;
                let mut ii: u32 = 0;
                let mut j: i32 = 0;
                cwrsi(n, k, i, y.as_mut_ptr());
                sy = 0 as i32;
                j = 0 as i32;
                while j < n {
                    sy += abs(y[j as usize]);
                    j += 1;
                }
                if sy != k {
                    fprintf(
                        stderr(),
                        b"N=%d Pulse count mismatch in cwrsi (%d!=%d).\n\0" as *const u8
                            as *const i8,
                        n,
                        sy,
                        k,
                    );
                    return 99 as i32;
                }
                ii = icwrs(n, y.as_mut_ptr());
                v = (*(CELT_PVQ_U_ROW[(if n < k { n } else { k }) as usize])
                    .offset((if n > k { n } else { k }) as isize))
                .wrapping_add(
                    *(CELT_PVQ_U_ROW[(if n < k + 1 as i32 {
                        n
                    } else {
                        k + 1 as i32
                    }) as usize])
                        .offset(
                            (if n > k + 1 as i32 {
                                n
                            } else {
                                k + 1 as i32
                            }) as isize,
                        ),
                );
                if ii != i {
                    fprintf(
                        stderr(),
                        b"Combination-index mismatch (%lu!=%lu).\n\0" as *const u8
                            as *const i8,
                        ii as i64,
                        i as i64,
                    );
                    return 1 as i32;
                }
                if v != nc {
                    fprintf(
                        stderr(),
                        b"Combination count mismatch (%lu!=%lu).\n\0" as *const u8
                            as *const i8,
                        v as i64,
                        nc as i64,
                    );
                    return 2 as i32;
                }
                i = (i as u32).wrapping_add(inc) as u32 as u32;
            }
            pseudo += 1;
        }
        t += 1;
    }
    return 0 as i32;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
unsafe fn run_static_initializers() {
    CELT_PVQ_U_ROW = [
        CELT_PVQ_U_DATA.as_ptr().offset(0 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(208 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(415 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(621 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(826 as i32 as isize),
        CELT_PVQ_U_DATA
            .as_ptr()
            .offset(1030 as i32 as isize),
        CELT_PVQ_U_DATA
            .as_ptr()
            .offset(1233 as i32 as isize),
        CELT_PVQ_U_DATA
            .as_ptr()
            .offset(1336 as i32 as isize),
        CELT_PVQ_U_DATA
            .as_ptr()
            .offset(1389 as i32 as isize),
        CELT_PVQ_U_DATA
            .as_ptr()
            .offset(1421 as i32 as isize),
        CELT_PVQ_U_DATA
            .as_ptr()
            .offset(1441 as i32 as isize),
        CELT_PVQ_U_DATA
            .as_ptr()
            .offset(1455 as i32 as isize),
        CELT_PVQ_U_DATA
            .as_ptr()
            .offset(1464 as i32 as isize),
        CELT_PVQ_U_DATA
            .as_ptr()
            .offset(1470 as i32 as isize),
        CELT_PVQ_U_DATA
            .as_ptr()
            .offset(1473 as i32 as isize),
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
