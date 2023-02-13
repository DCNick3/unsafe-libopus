#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = u64;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:35"]
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
    #[c2rust::src_loc = "124:1"]
    pub unsafe fn celt_udiv(mut n: u32, mut d: u32) -> u32 {
        return n.wrapping_div(d);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:32"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    {
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    {
        #[c2rust::src_loc = "454:1"]
        pub fn rand() -> i32;
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: u64) -> *mut core::ffi::c_void;
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut core::ffi::c_void);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.c:38"]
pub mod entenc_c {
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
    #[c2rust::src_loc = "60:1"]
    pub unsafe fn ec_write_byte(
        mut _this: *mut ec_enc,
        mut _value: u32,
    ) -> i32 {
        if ((*_this).offs).wrapping_add((*_this).end_offs) >= (*_this).storage {
            return -(1 as i32);
        }
        let fresh0 = (*_this).offs;
        (*_this).offs = ((*_this).offs).wrapping_add(1);
        *((*_this).buf).offset(fresh0 as isize) = _value as u8;
        return 0 as i32;
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
                    let ref mut fresh1 = *((*_this).buf).offset(
                        ((*_this).storage)
                            .wrapping_sub((*_this).end_offs)
                            .wrapping_sub(1 as i32 as u32)
                            as isize,
                    );
                    *fresh1 = (*fresh1 as i32 | window as u8 as i32)
                        as u8;
                }
            }
        }
    }
    use super::entcode_h::{celt_udiv, ec_enc, ec_window};
    use crate::externs::{memmove, memset};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.c:39"]
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
    use super::entcode_h::{celt_udiv, ec_dec, ec_window};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.c:40"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/laplace.c:41"]
pub mod laplace_c {
    #[c2rust::src_loc = "44:1"]
    pub unsafe fn ec_laplace_get_freq1(
        mut fs0: u32,
        mut decay: i32,
    ) -> u32 {
        let mut ft: u32 = 0;
        ft = ((32768 as i32
            - ((1 as i32) << 0 as i32) * (2 as i32 * 16 as i32))
            as u32)
            .wrapping_sub(fs0);
        return ft.wrapping_mul((16384 as i32 - decay) as u32)
            >> 15 as i32;
    }
    #[c2rust::src_loc = "51:1"]
    pub unsafe fn ec_laplace_encode(
        mut enc: *mut ec_enc,
        mut value: *mut i32,
        mut fs: u32,
        mut decay: i32,
    ) {
        let mut fl: u32 = 0;
        let mut val: i32 = *value;
        fl = 0 as i32 as u32;
        if val != 0 {
            let mut s: i32 = 0;
            let mut i: i32 = 0;
            s = -((val < 0 as i32) as i32);
            val = val + s ^ s;
            fl = fs;
            fs = ec_laplace_get_freq1(fs, decay);
            i = 1 as i32;
            while fs > 0 as i32 as u32 && i < val {
                fs = fs.wrapping_mul(2 as i32 as u32);
                fl = fl.wrapping_add(fs.wrapping_add(
                    (2 as i32 * ((1 as i32) << 0 as i32)) as u32,
                ));
                fs = fs.wrapping_mul(decay as u32) >> 15 as i32;
                i += 1;
            }
            if fs == 0 {
                let mut di: i32 = 0;
                let mut ndi_max: i32 = 0;
                ndi_max = ((32768 as i32 as u32)
                    .wrapping_sub(fl)
                    .wrapping_add(((1 as i32) << 0 as i32) as u32)
                    .wrapping_sub(1 as i32 as u32)
                    >> 0 as i32) as i32;
                ndi_max = ndi_max - s >> 1 as i32;
                di = if val - i < ndi_max - 1 as i32 {
                    val - i
                } else {
                    ndi_max - 1 as i32
                };
                fl = fl.wrapping_add(
                    ((2 as i32 * di + 1 as i32 + s)
                        * ((1 as i32) << 0 as i32))
                        as u32,
                );
                fs = if (((1 as i32) << 0 as i32) as u32)
                    < (32768 as i32 as u32).wrapping_sub(fl)
                {
                    ((1 as i32) << 0 as i32) as u32
                } else {
                    (32768 as i32 as u32).wrapping_sub(fl)
                };
                *value = i + di + s ^ s;
            } else {
                fs = fs.wrapping_add(((1 as i32) << 0 as i32) as u32);
                fl = fl.wrapping_add(fs & !s as u32);
            }
        }
        ec_encode_bin(
            enc,
            fl,
            fl.wrapping_add(fs),
            15 as i32 as u32,
        );
    }
    #[c2rust::src_loc = "94:1"]
    pub unsafe fn ec_laplace_decode(
        mut dec: *mut ec_dec,
        mut fs: u32,
        mut decay: i32,
    ) -> i32 {
        let mut val: i32 = 0 as i32;
        let mut fl: u32 = 0;
        let mut fm: u32 = 0;
        fm = ec_decode_bin(dec, 15 as i32 as u32);
        fl = 0 as i32 as u32;
        if fm >= fs {
            val += 1;
            fl = fs;
            fs = (ec_laplace_get_freq1(fs, decay))
                .wrapping_add(((1 as i32) << 0 as i32) as u32);
            while fs > ((1 as i32) << 0 as i32) as u32
                && fm >= fl.wrapping_add((2 as i32 as u32).wrapping_mul(fs))
            {
                fs = fs.wrapping_mul(2 as i32 as u32);
                fl = fl.wrapping_add(fs);
                fs = fs
                    .wrapping_sub(
                        (2 as i32 * ((1 as i32) << 0 as i32))
                            as u32,
                    )
                    .wrapping_mul(decay as u32)
                    >> 15 as i32;
                fs = fs.wrapping_add(((1 as i32) << 0 as i32) as u32);
                val += 1;
            }
            if fs <= ((1 as i32) << 0 as i32) as u32 {
                let mut di: i32 = 0;
                di = (fm.wrapping_sub(fl) >> 0 as i32 + 1 as i32) as i32;
                val += di;
                fl = fl.wrapping_add(
                    (2 as i32 * di * ((1 as i32) << 0 as i32))
                        as u32,
                );
            }
            if fm < fl.wrapping_add(fs) {
                val = -val;
            } else {
                fl = fl.wrapping_add(fs);
            }
        }
        ec_dec_update(
            dec,
            fl,
            if fl.wrapping_add(fs) < 32768 as i32 as u32 {
                fl.wrapping_add(fs)
            } else {
                32768 as i32 as u32
            },
            32768 as i32 as u32,
        );
        return val;
    }
    use super::entcode_h::{ec_dec, ec_enc};
    use super::entdec_c::{ec_dec_update, ec_decode_bin};
    use super::entenc_c::ec_encode_bin;
}
pub use self::entcode_c::ec_tell_frac;
pub use self::entcode_h::{
    celt_udiv, ec_ctx, ec_dec, ec_enc, ec_get_buffer, ec_range_bytes, ec_window,
};
pub use self::entdec_c::{
    ec_dec_bit_logp, ec_dec_bits, ec_dec_icdf, ec_dec_init, ec_dec_normalize, ec_dec_uint,
    ec_dec_update, ec_decode, ec_decode_bin, ec_read_byte, ec_read_byte_from_end,
};
pub use self::entenc_c::{
    ec_enc_bit_logp, ec_enc_bits, ec_enc_carry_out, ec_enc_done, ec_enc_icdf, ec_enc_init,
    ec_enc_normalize, ec_enc_patch_initial_bits, ec_enc_shrink, ec_enc_uint, ec_encode,
    ec_encode_bin, ec_write_byte, ec_write_byte_at_end,
};
pub use self::laplace_c::{ec_laplace_decode, ec_laplace_encode, ec_laplace_get_freq1};
pub use self::stddef_h::size_t;

use self::stdio_h::{fprintf, stderr};
use self::stdlib_h::{free, malloc, rand};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};

pub use self::FILE_h::FILE;
use crate::externs::{memmove, memset};
#[c2rust::src_loc = "45:1"]
pub unsafe fn ec_laplace_get_start_freq(mut decay: i32) -> i32 {
    let mut ft: u32 = (32768 as i32
        - ((1 as i32) << 0 as i32)
            * (2 as i32 * 16 as i32 + 1 as i32))
        as u32;
    let mut fs: i32 =
        ft.wrapping_mul((16384 as i32 - decay) as u32)
            .wrapping_div((16384 as i32 + decay) as u32) as i32;
    return fs + ((1 as i32) << 0 as i32);
}
#[c2rust::src_loc = "52:1"]
unsafe fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut ret: i32 = 0 as i32;
    let mut enc: ec_enc = ec_enc {
        buf: 0 as *mut u8,
        storage: 0,
        end_offs: 0,
        end_window: 0,
        nend_bits: 0,
        nbits_total: 0,
        offs: 0,
        rng: 0,
        val: 0,
        ext: 0,
        rem: 0,
        error: 0,
    };
    let mut dec: ec_dec = ec_enc {
        buf: 0 as *mut u8,
        storage: 0,
        end_offs: 0,
        end_window: 0,
        nend_bits: 0,
        nbits_total: 0,
        offs: 0,
        rng: 0,
        val: 0,
        ext: 0,
        rem: 0,
        error: 0,
    };
    let mut ptr: *mut u8 = 0 as *mut u8;
    let mut val: [i32; 10000] = [0; 10000];
    let mut decay: [i32; 10000] = [0; 10000];
    ptr = malloc(40000 as i32 as u64) as *mut u8;
    ec_enc_init(&mut enc, ptr, 40000 as i32 as u32);
    val[0 as i32 as usize] = 3 as i32;
    decay[0 as i32 as usize] = 6000 as i32;
    val[1 as i32 as usize] = 0 as i32;
    decay[1 as i32 as usize] = 5800 as i32;
    val[2 as i32 as usize] = -(1 as i32);
    decay[2 as i32 as usize] = 5600 as i32;
    i = 3 as i32;
    while i < 10000 as i32 {
        val[i as usize] = rand() % 15 as i32 - 7 as i32;
        decay[i as usize] = rand() % 11000 as i32 + 5000 as i32;
        i += 1;
    }
    i = 0 as i32;
    while i < 10000 as i32 {
        ec_laplace_encode(
            &mut enc,
            &mut *val.as_mut_ptr().offset(i as isize),
            ec_laplace_get_start_freq(decay[i as usize]) as u32,
            decay[i as usize],
        );
        i += 1;
    }
    ec_enc_done(&mut enc);
    ec_dec_init(&mut dec, ec_get_buffer(&mut enc), ec_range_bytes(&mut enc));
    i = 0 as i32;
    while i < 10000 as i32 {
        let mut d: i32 = ec_laplace_decode(
            &mut dec,
            ec_laplace_get_start_freq(decay[i as usize]) as u32,
            decay[i as usize],
        );
        if d != val[i as usize] {
            fprintf(
                stderr(),
                b"Got %d instead of %d\n\0" as *const u8 as *const i8,
                d,
                val[i as usize],
            );
            ret = 1 as i32;
        }
        i += 1;
    }
    free(ptr as *mut core::ffi::c_void);
    return ret;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
