use ::libc;
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:35"]
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
    pub unsafe fn ec_get_buffer(mut _this: *mut ec_ctx) -> *mut libc::c_uchar {
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
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    {
        #[c2rust::src_loc = "454:1"]
        pub fn rand() -> libc::c_int;
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.c:38"]
pub mod entenc_c {
    #[c2rust::src_loc = "82:1"]
    pub unsafe fn ec_enc_carry_out(mut _this: *mut ec_enc, mut _c: libc::c_int) {
        if _c as libc::c_uint
            != ((1 as libc::c_uint) << 8 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            let mut carry: libc::c_int = 0;
            carry = _c >> 8 as libc::c_int;
            if (*_this).rem >= 0 as libc::c_int {
                (*_this).error |= ec_write_byte(_this, ((*_this).rem + carry) as libc::c_uint);
            }
            if (*_this).ext > 0 as libc::c_int as libc::c_uint {
                let mut sym: libc::c_uint = 0;
                sym = ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(carry as libc::c_uint)
                    & ((1 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint);
                loop {
                    (*_this).error |= ec_write_byte(_this, sym);
                    (*_this).ext = ((*_this).ext).wrapping_sub(1);
                    if !((*_this).ext > 0 as libc::c_int as libc::c_uint) {
                        break;
                    }
                }
            }
            (*_this).rem = (_c as libc::c_uint
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                as libc::c_int;
        } else {
            (*_this).ext = ((*_this).ext).wrapping_add(1);
        };
    }
    #[c2rust::src_loc = "60:1"]
    pub unsafe fn ec_write_byte(
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
    pub unsafe fn ec_write_byte_at_end(
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
    #[inline]
    #[c2rust::src_loc = "101:1"]
    pub unsafe fn ec_enc_normalize(mut _this: *mut ec_enc) {
        while (*_this).rng
            <= (1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int >> 8 as libc::c_int
        {
            ec_enc_carry_out(
                _this,
                ((*_this).val >> 32 as libc::c_int - 8 as libc::c_int - 1 as libc::c_int)
                    as libc::c_int,
            );
            (*_this).val = (*_this).val << 8 as libc::c_int
                & ((1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint);
            (*_this).rng <<= 8 as libc::c_int;
            (*_this).nbits_total += 8 as libc::c_int;
        }
    }
    #[c2rust::src_loc = "112:1"]
    pub unsafe fn ec_enc_init(
        mut _this: *mut ec_enc,
        mut _buf: *mut libc::c_uchar,
        mut _size: u32,
    ) {
        (*_this).buf = _buf;
        (*_this).end_offs = 0 as libc::c_int as u32;
        (*_this).end_window = 0 as libc::c_int as ec_window;
        (*_this).nend_bits = 0 as libc::c_int;
        (*_this).nbits_total = 32 as libc::c_int + 1 as libc::c_int;
        (*_this).offs = 0 as libc::c_int as u32;
        (*_this).rng = (1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int;
        (*_this).rem = -(1 as libc::c_int);
        (*_this).val = 0 as libc::c_int as u32;
        (*_this).ext = 0 as libc::c_int as u32;
        (*_this).storage = _size;
        (*_this).error = 0 as libc::c_int;
    }
    #[c2rust::src_loc = "128:1"]
    pub unsafe fn ec_encode(
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
                .wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fh)))
                as u32 as u32;
        }
        ec_enc_normalize(_this);
    }
    #[c2rust::src_loc = "139:1"]
    pub unsafe fn ec_encode_bin(
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
    #[c2rust::src_loc = "151:1"]
    pub unsafe fn ec_enc_bit_logp(
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
    #[c2rust::src_loc = "164:1"]
    pub unsafe fn ec_enc_icdf(
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
    #[c2rust::src_loc = "175:1"]
    pub unsafe fn ec_enc_uint(mut _this: *mut ec_enc, mut _fl: u32, mut _ft: u32) {
        let mut ft: libc::c_uint = 0;
        let mut fl: libc::c_uint = 0;
        let mut ftb: libc::c_int = 0;
        _ft = _ft.wrapping_sub(1);
        ftb = ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
            * 8 as libc::c_int
            - _ft.leading_zeros() as i32;
        if ftb > 8 as libc::c_int {
            ftb -= 8 as libc::c_int;
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
    #[c2rust::src_loc = "193:1"]
    pub unsafe fn ec_enc_bits(
        mut _this: *mut ec_enc,
        mut _fl: u32,
        mut _bits: libc::c_uint,
    ) {
        let mut window: ec_window = 0;
        let mut used: libc::c_int = 0;
        window = (*_this).end_window;
        used = (*_this).nend_bits;
        if (used as libc::c_uint).wrapping_add(_bits)
            > (::core::mem::size_of::<ec_window>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int) as libc::c_uint
        {
            loop {
                (*_this).error |= ec_write_byte_at_end(
                    _this,
                    window
                        & ((1 as libc::c_uint) << 8 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
                window >>= 8 as libc::c_int;
                used -= 8 as libc::c_int;
                if !(used >= 8 as libc::c_int) {
                    break;
                }
            }
        }
        window |= _fl << used;
        used = (used as libc::c_uint).wrapping_add(_bits) as libc::c_int as libc::c_int;
        (*_this).end_window = window;
        (*_this).nend_bits = used;
        (*_this).nbits_total = ((*_this).nbits_total as libc::c_uint).wrapping_add(_bits)
            as libc::c_int as libc::c_int;
    }
    #[c2rust::src_loc = "214:1"]
    pub unsafe fn ec_enc_patch_initial_bits(
        mut _this: *mut ec_enc,
        mut _val: libc::c_uint,
        mut _nbits: libc::c_uint,
    ) {
        let mut shift: libc::c_int = 0;
        let mut mask: libc::c_uint = 0;
        shift = (8 as libc::c_int as libc::c_uint).wrapping_sub(_nbits) as libc::c_int;
        mask = ((((1 as libc::c_int) << _nbits) - 1 as libc::c_int) << shift) as libc::c_uint;
        if (*_this).offs > 0 as libc::c_int as libc::c_uint {
            *((*_this).buf).offset(0 as libc::c_int as isize) =
                (*((*_this).buf).offset(0 as libc::c_int as isize) as libc::c_uint & !mask
                    | _val << shift) as libc::c_uchar;
        } else if (*_this).rem >= 0 as libc::c_int {
            (*_this).rem = ((*_this).rem as libc::c_uint & !mask | _val << shift) as libc::c_int;
        } else if (*_this).rng
            <= (1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int >> _nbits
        {
            (*_this).val = (*_this).val
                & !(mask << 32 as libc::c_int - 8 as libc::c_int - 1 as libc::c_int)
                | _val << 32 as libc::c_int - 8 as libc::c_int - 1 as libc::c_int + shift;
        } else {
            (*_this).error = -(1 as libc::c_int);
        };
    }
    #[c2rust::src_loc = "237:1"]
    pub unsafe fn ec_enc_shrink(mut _this: *mut ec_enc, mut _size: u32) {
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
    #[c2rust::src_loc = "244:1"]
    pub unsafe fn ec_enc_done(mut _this: *mut ec_enc) {
        let mut window: ec_window = 0;
        let mut used: libc::c_int = 0;
        let mut msk: u32 = 0;
        let mut end: u32 = 0;
        let mut l: libc::c_int = 0;
        l = 32 as libc::c_int
            - (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
                - ((*_this).rng).leading_zeros() as i32);
        msk = ((1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            >> l;
        end = ((*_this).val).wrapping_add(msk) & !msk;
        if end | msk >= ((*_this).val).wrapping_add((*_this).rng) {
            l += 1;
            msk >>= 1 as libc::c_int;
            end = ((*_this).val).wrapping_add(msk) & !msk;
        }
        while l > 0 as libc::c_int {
            ec_enc_carry_out(
                _this,
                (end >> 32 as libc::c_int - 8 as libc::c_int - 1 as libc::c_int) as libc::c_int,
            );
            end = end << 8 as libc::c_int
                & ((1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint);
            l -= 8 as libc::c_int;
        }
        if (*_this).rem >= 0 as libc::c_int || (*_this).ext > 0 as libc::c_int as libc::c_uint {
            ec_enc_carry_out(_this, 0 as libc::c_int);
        }
        window = (*_this).end_window;
        used = (*_this).nend_bits;
        while used >= 8 as libc::c_int {
            (*_this).error |= ec_write_byte_at_end(
                _this,
                window
                    & ((1 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
            window >>= 8 as libc::c_int;
            used -= 8 as libc::c_int;
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
                    if ((*_this).offs).wrapping_add((*_this).end_offs) >= (*_this).storage
                        && l < used
                    {
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
    use super::entcode_h::{celt_udiv, ec_enc, ec_window};
    use crate::externs::{memmove, memset};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.c:39"]
pub mod entdec_c {
    #[c2rust::src_loc = "91:1"]
    pub unsafe fn ec_read_byte(mut _this: *mut ec_dec) -> libc::c_int {
        return if (*_this).offs < (*_this).storage {
            let fresh2 = (*_this).offs;
            (*_this).offs = ((*_this).offs).wrapping_add(1);
            *((*_this).buf).offset(fresh2 as isize) as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    #[c2rust::src_loc = "95:1"]
    pub unsafe fn ec_read_byte_from_end(mut _this: *mut ec_dec) -> libc::c_int {
        return if (*_this).end_offs < (*_this).storage {
            (*_this).end_offs = ((*_this).end_offs).wrapping_add(1);
            *((*_this).buf).offset(((*_this).storage).wrapping_sub((*_this).end_offs) as isize)
                as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    #[c2rust::src_loc = "102:1"]
    pub unsafe fn ec_dec_normalize(mut _this: *mut ec_dec) {
        while (*_this).rng
            <= (1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int >> 8 as libc::c_int
        {
            let mut sym: libc::c_int = 0;
            (*_this).nbits_total += 8 as libc::c_int;
            (*_this).rng <<= 8 as libc::c_int;
            sym = (*_this).rem;
            (*_this).rem = ec_read_byte(_this);
            sym = (sym << 8 as libc::c_int | (*_this).rem)
                >> 8 as libc::c_int
                    - ((32 as libc::c_int - 2 as libc::c_int) % 8 as libc::c_int
                        + 1 as libc::c_int);
            (*_this).val = ((*_this).val << 8 as libc::c_int).wrapping_add(
                ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    & !sym as libc::c_uint,
            ) & ((1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        }
    }
    #[c2rust::src_loc = "119:1"]
    pub unsafe fn ec_dec_init(
        mut _this: *mut ec_dec,
        mut _buf: *mut libc::c_uchar,
        mut _storage: u32,
    ) {
        (*_this).buf = _buf;
        (*_this).storage = _storage;
        (*_this).end_offs = 0 as libc::c_int as u32;
        (*_this).end_window = 0 as libc::c_int as ec_window;
        (*_this).nend_bits = 0 as libc::c_int;
        (*_this).nbits_total = 32 as libc::c_int + 1 as libc::c_int
            - (32 as libc::c_int
                - ((32 as libc::c_int - 2 as libc::c_int) % 8 as libc::c_int + 1 as libc::c_int))
                / 8 as libc::c_int
                * 8 as libc::c_int;
        (*_this).offs = 0 as libc::c_int as u32;
        (*_this).rng = (1 as libc::c_uint)
            << (32 as libc::c_int - 2 as libc::c_int) % 8 as libc::c_int + 1 as libc::c_int;
        (*_this).rem = ec_read_byte(_this);
        (*_this).val = ((*_this).rng)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(
                ((*_this).rem
                    >> 8 as libc::c_int
                        - ((32 as libc::c_int - 2 as libc::c_int) % 8 as libc::c_int
                            + 1 as libc::c_int)) as libc::c_uint,
            );
        (*_this).error = 0 as libc::c_int;
        ec_dec_normalize(_this);
    }
    #[c2rust::src_loc = "139:1"]
    pub unsafe fn ec_decode(
        mut _this: *mut ec_dec,
        mut _ft: libc::c_uint,
    ) -> libc::c_uint {
        let mut s: libc::c_uint = 0;
        (*_this).ext = celt_udiv((*_this).rng, _ft);
        s = ((*_this).val).wrapping_div((*_this).ext);
        return _ft.wrapping_sub(
            s.wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    _ft.wrapping_sub(s.wrapping_add(1 as libc::c_int as libc::c_uint))
                        & -((_ft < s.wrapping_add(1 as libc::c_int as libc::c_uint)) as libc::c_int)
                            as libc::c_uint,
                ),
        );
    }
    #[c2rust::src_loc = "146:1"]
    pub unsafe fn ec_decode_bin(
        mut _this: *mut ec_dec,
        mut _bits: libc::c_uint,
    ) -> libc::c_uint {
        let mut s: libc::c_uint = 0;
        (*_this).ext = (*_this).rng >> _bits;
        s = ((*_this).val).wrapping_div((*_this).ext);
        return ((1 as libc::c_uint) << _bits).wrapping_sub(
            s.wrapping_add(1 as libc::c_uint).wrapping_add(
                ((1 as libc::c_uint) << _bits).wrapping_sub(s.wrapping_add(1 as libc::c_uint))
                    & -(((1 as libc::c_uint) << _bits < s.wrapping_add(1 as libc::c_uint))
                        as libc::c_int) as libc::c_uint,
            ),
        );
    }
    #[c2rust::src_loc = "153:1"]
    pub unsafe fn ec_dec_update(
        mut _this: *mut ec_dec,
        mut _fl: libc::c_uint,
        mut _fh: libc::c_uint,
        mut _ft: libc::c_uint,
    ) {
        let mut s: u32 = 0;
        s = ((*_this).ext).wrapping_mul(_ft.wrapping_sub(_fh));
        (*_this).val = ((*_this).val as libc::c_uint).wrapping_sub(s) as u32 as u32;
        (*_this).rng = if _fl > 0 as libc::c_int as libc::c_uint {
            ((*_this).ext).wrapping_mul(_fh.wrapping_sub(_fl))
        } else {
            ((*_this).rng).wrapping_sub(s)
        };
        ec_dec_normalize(_this);
    }
    #[c2rust::src_loc = "162:1"]
    pub unsafe fn ec_dec_bit_logp(
        mut _this: *mut ec_dec,
        mut _logp: libc::c_uint,
    ) -> libc::c_int {
        let mut r: u32 = 0;
        let mut d: u32 = 0;
        let mut s: u32 = 0;
        let mut ret: libc::c_int = 0;
        r = (*_this).rng;
        d = (*_this).val;
        s = r >> _logp;
        ret = (d < s) as libc::c_int;
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
        mut _icdf: *const libc::c_uchar,
        mut _ftb: libc::c_uint,
    ) -> libc::c_int {
        let mut r: u32 = 0;
        let mut d: u32 = 0;
        let mut s: u32 = 0;
        let mut t: u32 = 0;
        let mut ret: libc::c_int = 0;
        s = (*_this).rng;
        d = (*_this).val;
        r = s >> _ftb;
        ret = -(1 as libc::c_int);
        loop {
            t = s;
            ret += 1;
            s = r.wrapping_mul(*_icdf.offset(ret as isize) as libc::c_uint);
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
        let mut ft: libc::c_uint = 0;
        let mut s: libc::c_uint = 0;
        let mut ftb: libc::c_int = 0;
        _ft = _ft.wrapping_sub(1);
        ftb = ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
            * 8 as libc::c_int
            - _ft.leading_zeros() as i32;
        if ftb > 8 as libc::c_int {
            let mut t: u32 = 0;
            ftb -= 8 as libc::c_int;
            ft = (_ft >> ftb).wrapping_add(1 as libc::c_int as libc::c_uint);
            s = ec_decode(_this, ft);
            ec_dec_update(
                _this,
                s,
                s.wrapping_add(1 as libc::c_int as libc::c_uint),
                ft,
            );
            t = s << ftb | ec_dec_bits(_this, ftb as libc::c_uint);
            if t <= _ft {
                return t;
            }
            (*_this).error = 1 as libc::c_int;
            return _ft;
        } else {
            _ft = _ft.wrapping_add(1);
            s = ec_decode(_this, _ft);
            ec_dec_update(
                _this,
                s,
                s.wrapping_add(1 as libc::c_int as libc::c_uint),
                _ft,
            );
            return s;
        };
    }
    #[c2rust::src_loc = "225:1"]
    pub unsafe fn ec_dec_bits(mut _this: *mut ec_dec, mut _bits: libc::c_uint) -> u32 {
        let mut window: ec_window = 0;
        let mut available: libc::c_int = 0;
        let mut ret: u32 = 0;
        window = (*_this).end_window;
        available = (*_this).nend_bits;
        if (available as libc::c_uint) < _bits {
            loop {
                window |= (ec_read_byte_from_end(_this) as ec_window) << available;
                available += 8 as libc::c_int;
                if !(available
                    <= ::core::mem::size_of::<ec_window>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int
                        - 8 as libc::c_int)
                {
                    break;
                }
            }
        }
        ret = window & ((1 as libc::c_int as u32) << _bits).wrapping_sub(1 as libc::c_uint);
        window >>= _bits;
        available = (available as libc::c_uint).wrapping_sub(_bits) as libc::c_int as libc::c_int;
        (*_this).end_window = window;
        (*_this).nend_bits = available;
        (*_this).nbits_total = ((*_this).nbits_total as libc::c_uint).wrapping_add(_bits)
            as libc::c_int as libc::c_int;
        return ret;
    }
    use super::entcode_h::{celt_udiv, ec_dec, ec_window};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.c:40"]
pub mod entcode_c {
    #[c2rust::src_loc = "69:1"]
    pub unsafe fn ec_tell_frac(mut _this: *mut ec_ctx) -> u32 {
        pub static mut correction: [libc::c_uint; 8] = [
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
        nbits = ((*_this).nbits_total << 3 as libc::c_int) as u32;
        l = ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
            * 8 as libc::c_int
            - ((*_this).rng).leading_zeros() as i32;
        r = (*_this).rng >> l - 16 as libc::c_int;
        b = (r >> 12 as libc::c_int).wrapping_sub(8 as libc::c_int as libc::c_uint);
        b = b.wrapping_add((r > correction[b as usize]) as libc::c_int as libc::c_uint);
        l = ((l << 3 as libc::c_int) as libc::c_uint).wrapping_add(b) as libc::c_int;
        return nbits.wrapping_sub(l as libc::c_uint);
    }
    use super::entcode_h::ec_ctx;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/laplace.c:41"]
pub mod laplace_c {
    #[c2rust::src_loc = "44:1"]
    pub unsafe fn ec_laplace_get_freq1(
        mut fs0: libc::c_uint,
        mut decay: libc::c_int,
    ) -> libc::c_uint {
        let mut ft: libc::c_uint = 0;
        ft = ((32768 as libc::c_int
            - ((1 as libc::c_int) << 0 as libc::c_int) * (2 as libc::c_int * 16 as libc::c_int))
            as libc::c_uint)
            .wrapping_sub(fs0);
        return ft.wrapping_mul((16384 as libc::c_int - decay) as libc::c_uint)
            >> 15 as libc::c_int;
    }
    #[c2rust::src_loc = "51:1"]
    pub unsafe fn ec_laplace_encode(
        mut enc: *mut ec_enc,
        mut value: *mut libc::c_int,
        mut fs: libc::c_uint,
        mut decay: libc::c_int,
    ) {
        let mut fl: libc::c_uint = 0;
        let mut val: libc::c_int = *value;
        fl = 0 as libc::c_int as libc::c_uint;
        if val != 0 {
            let mut s: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            s = -((val < 0 as libc::c_int) as libc::c_int);
            val = val + s ^ s;
            fl = fs;
            fs = ec_laplace_get_freq1(fs, decay);
            i = 1 as libc::c_int;
            while fs > 0 as libc::c_int as libc::c_uint && i < val {
                fs = fs.wrapping_mul(2 as libc::c_int as libc::c_uint);
                fl = fl.wrapping_add(fs.wrapping_add(
                    (2 as libc::c_int * ((1 as libc::c_int) << 0 as libc::c_int)) as libc::c_uint,
                ));
                fs = fs.wrapping_mul(decay as libc::c_uint) >> 15 as libc::c_int;
                i += 1;
            }
            if fs == 0 {
                let mut di: libc::c_int = 0;
                let mut ndi_max: libc::c_int = 0;
                ndi_max = ((32768 as libc::c_int as libc::c_uint)
                    .wrapping_sub(fl)
                    .wrapping_add(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    >> 0 as libc::c_int) as libc::c_int;
                ndi_max = ndi_max - s >> 1 as libc::c_int;
                di = if val - i < ndi_max - 1 as libc::c_int {
                    val - i
                } else {
                    ndi_max - 1 as libc::c_int
                };
                fl = fl.wrapping_add(
                    ((2 as libc::c_int * di + 1 as libc::c_int + s)
                        * ((1 as libc::c_int) << 0 as libc::c_int))
                        as libc::c_uint,
                );
                fs = if (((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                    < (32768 as libc::c_int as libc::c_uint).wrapping_sub(fl)
                {
                    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
                } else {
                    (32768 as libc::c_int as libc::c_uint).wrapping_sub(fl)
                };
                *value = i + di + s ^ s;
            } else {
                fs = fs.wrapping_add(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint);
                fl = fl.wrapping_add(fs & !s as libc::c_uint);
            }
        }
        ec_encode_bin(
            enc,
            fl,
            fl.wrapping_add(fs),
            15 as libc::c_int as libc::c_uint,
        );
    }
    #[c2rust::src_loc = "94:1"]
    pub unsafe fn ec_laplace_decode(
        mut dec: *mut ec_dec,
        mut fs: libc::c_uint,
        mut decay: libc::c_int,
    ) -> libc::c_int {
        let mut val: libc::c_int = 0 as libc::c_int;
        let mut fl: libc::c_uint = 0;
        let mut fm: libc::c_uint = 0;
        fm = ec_decode_bin(dec, 15 as libc::c_int as libc::c_uint);
        fl = 0 as libc::c_int as libc::c_uint;
        if fm >= fs {
            val += 1;
            fl = fs;
            fs = (ec_laplace_get_freq1(fs, decay))
                .wrapping_add(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint);
            while fs > ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
                && fm >= fl.wrapping_add((2 as libc::c_int as libc::c_uint).wrapping_mul(fs))
            {
                fs = fs.wrapping_mul(2 as libc::c_int as libc::c_uint);
                fl = fl.wrapping_add(fs);
                fs = fs
                    .wrapping_sub(
                        (2 as libc::c_int * ((1 as libc::c_int) << 0 as libc::c_int))
                            as libc::c_uint,
                    )
                    .wrapping_mul(decay as libc::c_uint)
                    >> 15 as libc::c_int;
                fs = fs.wrapping_add(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint);
                val += 1;
            }
            if fs <= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint {
                let mut di: libc::c_int = 0;
                di = (fm.wrapping_sub(fl) >> 0 as libc::c_int + 1 as libc::c_int) as libc::c_int;
                val += di;
                fl = fl.wrapping_add(
                    (2 as libc::c_int * di * ((1 as libc::c_int) << 0 as libc::c_int))
                        as libc::c_uint,
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
            if fl.wrapping_add(fs) < 32768 as libc::c_int as libc::c_uint {
                fl.wrapping_add(fs)
            } else {
                32768 as libc::c_int as libc::c_uint
            },
            32768 as libc::c_int as libc::c_uint,
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
pub unsafe fn ec_laplace_get_start_freq(mut decay: libc::c_int) -> libc::c_int {
    let mut ft: u32 = (32768 as libc::c_int
        - ((1 as libc::c_int) << 0 as libc::c_int)
            * (2 as libc::c_int * 16 as libc::c_int + 1 as libc::c_int))
        as u32;
    let mut fs: libc::c_int =
        ft.wrapping_mul((16384 as libc::c_int - decay) as libc::c_uint)
            .wrapping_div((16384 as libc::c_int + decay) as libc::c_uint) as libc::c_int;
    return fs + ((1 as libc::c_int) << 0 as libc::c_int);
}
#[c2rust::src_loc = "52:1"]
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut enc: ec_enc = ec_enc {
        buf: 0 as *mut libc::c_uchar,
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
        buf: 0 as *mut libc::c_uchar,
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
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut val: [libc::c_int; 10000] = [0; 10000];
    let mut decay: [libc::c_int; 10000] = [0; 10000];
    ptr = malloc(40000 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    ec_enc_init(&mut enc, ptr, 40000 as libc::c_int as u32);
    val[0 as libc::c_int as usize] = 3 as libc::c_int;
    decay[0 as libc::c_int as usize] = 6000 as libc::c_int;
    val[1 as libc::c_int as usize] = 0 as libc::c_int;
    decay[1 as libc::c_int as usize] = 5800 as libc::c_int;
    val[2 as libc::c_int as usize] = -(1 as libc::c_int);
    decay[2 as libc::c_int as usize] = 5600 as libc::c_int;
    i = 3 as libc::c_int;
    while i < 10000 as libc::c_int {
        val[i as usize] = rand() % 15 as libc::c_int - 7 as libc::c_int;
        decay[i as usize] = rand() % 11000 as libc::c_int + 5000 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        ec_laplace_encode(
            &mut enc,
            &mut *val.as_mut_ptr().offset(i as isize),
            ec_laplace_get_start_freq(decay[i as usize]) as libc::c_uint,
            decay[i as usize],
        );
        i += 1;
    }
    ec_enc_done(&mut enc);
    ec_dec_init(&mut dec, ec_get_buffer(&mut enc), ec_range_bytes(&mut enc));
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        let mut d: libc::c_int = ec_laplace_decode(
            &mut dec,
            ec_laplace_get_start_freq(decay[i as usize]) as libc::c_uint,
            decay[i as usize],
        );
        if d != val[i as usize] {
            fprintf(
                stderr(),
                b"Got %d instead of %d\n\0" as *const u8 as *const libc::c_char,
                d,
                val[i as usize],
            );
            ret = 1 as libc::c_int;
        }
        i += 1;
    }
    free(ptr as *mut libc::c_void);
    return ret;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
