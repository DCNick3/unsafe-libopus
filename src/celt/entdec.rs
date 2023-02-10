use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:33"]
pub mod opus_types_h {
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:35"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = opus_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: opus_uint32,
        pub end_offs: opus_uint32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: opus_uint32,
        pub rng: opus_uint32,
        pub val: opus_uint32,
        pub ext: opus_uint32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
    #[inline]
    #[c2rust::src_loc = "124:1"]
    pub unsafe extern "C" fn celt_udiv(mut n: opus_uint32, mut d: opus_uint32) -> opus_uint32 {
        return n.wrapping_div(d);
    }
    use super::opus_types_h::opus_uint32;
}
pub use self::entcode_h::{celt_udiv, ec_ctx, ec_dec, ec_window};
pub use self::opus_types_h::opus_uint32;
pub use self::stdint_uintn_h::uint32_t;
pub use self::types_h::__uint32_t;
#[c2rust::src_loc = "91:1"]
unsafe extern "C" fn ec_read_byte(mut _this: *mut ec_dec) -> libc::c_int {
    return if (*_this).offs < (*_this).storage {
        let fresh0 = (*_this).offs;
        (*_this).offs = ((*_this).offs).wrapping_add(1);
        *((*_this).buf).offset(fresh0 as isize) as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[c2rust::src_loc = "95:1"]
unsafe extern "C" fn ec_read_byte_from_end(mut _this: *mut ec_dec) -> libc::c_int {
    return if (*_this).end_offs < (*_this).storage {
        (*_this).end_offs = ((*_this).end_offs).wrapping_add(1);
        *((*_this).buf).offset(((*_this).storage).wrapping_sub((*_this).end_offs) as isize)
            as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn ec_dec_normalize(mut _this: *mut ec_dec) {
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
                - ((32 as libc::c_int - 2 as libc::c_int) % 8 as libc::c_int + 1 as libc::c_int);
        (*_this).val = ((*_this).val << 8 as libc::c_int).wrapping_add(
            ((1 as libc::c_uint) << 8 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                & !sym as libc::c_uint,
        ) & ((1 as libc::c_uint) << 32 as libc::c_int - 1 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
    }
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn ec_dec_init(
    mut _this: *mut ec_dec,
    mut _buf: *mut libc::c_uchar,
    mut _storage: opus_uint32,
) {
    (*_this).buf = _buf;
    (*_this).storage = _storage;
    (*_this).end_offs = 0 as libc::c_int as opus_uint32;
    (*_this).end_window = 0 as libc::c_int as ec_window;
    (*_this).nend_bits = 0 as libc::c_int;
    (*_this).nbits_total = 32 as libc::c_int + 1 as libc::c_int
        - (32 as libc::c_int
            - ((32 as libc::c_int - 2 as libc::c_int) % 8 as libc::c_int + 1 as libc::c_int))
            / 8 as libc::c_int
            * 8 as libc::c_int;
    (*_this).offs = 0 as libc::c_int as opus_uint32;
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
#[no_mangle]
#[c2rust::src_loc = "139:1"]
pub unsafe extern "C" fn ec_decode(mut _this: *mut ec_dec, mut _ft: libc::c_uint) -> libc::c_uint {
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
#[no_mangle]
#[c2rust::src_loc = "146:1"]
pub unsafe extern "C" fn ec_decode_bin(
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
#[no_mangle]
#[c2rust::src_loc = "153:1"]
pub unsafe extern "C" fn ec_dec_update(
    mut _this: *mut ec_dec,
    mut _fl: libc::c_uint,
    mut _fh: libc::c_uint,
    mut _ft: libc::c_uint,
) {
    let mut s: opus_uint32 = 0;
    s = ((*_this).ext).wrapping_mul(_ft.wrapping_sub(_fh));
    (*_this).val = ((*_this).val as libc::c_uint).wrapping_sub(s) as opus_uint32 as opus_uint32;
    (*_this).rng = if _fl > 0 as libc::c_int as libc::c_uint {
        ((*_this).ext).wrapping_mul(_fh.wrapping_sub(_fl))
    } else {
        ((*_this).rng).wrapping_sub(s)
    };
    ec_dec_normalize(_this);
}
#[no_mangle]
#[c2rust::src_loc = "162:1"]
pub unsafe extern "C" fn ec_dec_bit_logp(
    mut _this: *mut ec_dec,
    mut _logp: libc::c_uint,
) -> libc::c_int {
    let mut r: opus_uint32 = 0;
    let mut d: opus_uint32 = 0;
    let mut s: opus_uint32 = 0;
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
#[no_mangle]
#[c2rust::src_loc = "177:1"]
pub unsafe extern "C" fn ec_dec_icdf(
    mut _this: *mut ec_dec,
    mut _icdf: *const libc::c_uchar,
    mut _ftb: libc::c_uint,
) -> libc::c_int {
    let mut r: opus_uint32 = 0;
    let mut d: opus_uint32 = 0;
    let mut s: opus_uint32 = 0;
    let mut t: opus_uint32 = 0;
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
#[no_mangle]
#[c2rust::src_loc = "198:1"]
pub unsafe extern "C" fn ec_dec_uint(mut _this: *mut ec_dec, mut _ft: opus_uint32) -> opus_uint32 {
    let mut ft: libc::c_uint = 0;
    let mut s: libc::c_uint = 0;
    let mut ftb: libc::c_int = 0;
    _ft = _ft.wrapping_sub(1);
    ftb = ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * 8 as libc::c_int
        - _ft.leading_zeros() as i32;
    if ftb > 8 as libc::c_int {
        let mut t: opus_uint32 = 0;
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
#[no_mangle]
#[c2rust::src_loc = "225:1"]
pub unsafe extern "C" fn ec_dec_bits(
    mut _this: *mut ec_dec,
    mut _bits: libc::c_uint,
) -> opus_uint32 {
    let mut window: ec_window = 0;
    let mut available: libc::c_int = 0;
    let mut ret: opus_uint32 = 0;
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
    ret = window & ((1 as libc::c_int as opus_uint32) << _bits).wrapping_sub(1 as libc::c_uint);
    window >>= _bits;
    available = (available as libc::c_uint).wrapping_sub(_bits) as libc::c_int as libc::c_int;
    (*_this).end_window = window;
    (*_this).nend_bits = available;
    (*_this).nbits_total =
        ((*_this).nbits_total as libc::c_uint).wrapping_add(_bits) as libc::c_int as libc::c_int;
    return ret;
}
