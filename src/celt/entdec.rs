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
    #[c2rust::src_loc = "53:10"]
    pub const EC_UINT_BITS: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "50:10"]
    pub const EC_WINDOW_SIZE: libc::c_int = ::core::mem::size_of::<ec_window>()
        as libc::c_ulong as libc::c_int * CHAR_BIT;
    #[inline]
    #[c2rust::src_loc = "124:1"]
    pub unsafe extern "C" fn celt_udiv(
        mut n: opus_uint32,
        mut d: opus_uint32,
    ) -> opus_uint32 {
        return n.wrapping_div(d);
    }
    use super::opus_types_h::opus_uint32;
    use super::internal::__CHAR_BIT__;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:34"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:35"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int = ::core::mem::size_of::<libc::c_uint>()
        as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:35"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mfrngcod.h:36"]
pub mod mfrngcod_h {
    #[c2rust::src_loc = "35:10"]
    pub const EC_SYM_BITS: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "37:10"]
    pub const EC_CODE_BITS: libc::c_int = 32 as libc::c_int;
    #[c2rust::src_loc = "39:10"]
    pub const EC_SYM_MAX: libc::c_uint = ((1 as libc::c_uint) << EC_SYM_BITS)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    #[c2rust::src_loc = "43:10"]
    pub const EC_CODE_TOP: opus_uint32 = (1 as libc::c_uint)
        << EC_CODE_BITS - 1 as libc::c_int;
    #[c2rust::src_loc = "45:10"]
    pub const EC_CODE_BOT: opus_uint32 = EC_CODE_TOP >> EC_SYM_BITS;
    #[c2rust::src_loc = "47:10"]
    pub const EC_CODE_EXTRA: libc::c_int = (EC_CODE_BITS - 2 as libc::c_int)
        % EC_SYM_BITS + 1 as libc::c_int;
    use super::opus_types_h::opus_uint32;
}
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::opus_types_h::opus_uint32;
pub use self::entcode_h::{
    ec_window, ec_ctx, ec_dec, EC_UINT_BITS, EC_WINDOW_SIZE, celt_udiv,
};
use self::arch_h::celt_fatal;
pub use self::ecintrin_h::EC_CLZ0;
pub use self::limits_h::CHAR_BIT;
pub use self::internal::__CHAR_BIT__;
pub use self::mfrngcod_h::{
    EC_SYM_BITS, EC_CODE_BITS, EC_SYM_MAX, EC_CODE_TOP, EC_CODE_BOT, EC_CODE_EXTRA,
};
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
        *((*_this).buf)
            .offset(((*_this).storage).wrapping_sub((*_this).end_offs) as isize)
            as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn ec_dec_normalize(mut _this: *mut ec_dec) {
    while (*_this).rng <= EC_CODE_BOT {
        let mut sym: libc::c_int = 0;
        (*_this).nbits_total += EC_SYM_BITS;
        (*_this).rng <<= EC_SYM_BITS;
        sym = (*_this).rem;
        (*_this).rem = ec_read_byte(_this);
        sym = (sym << EC_SYM_BITS | (*_this).rem) >> EC_SYM_BITS - EC_CODE_EXTRA;
        (*_this)
            .val = ((*_this).val << EC_SYM_BITS)
            .wrapping_add(EC_SYM_MAX & !sym as libc::c_uint)
            & EC_CODE_TOP.wrapping_sub(1 as libc::c_int as libc::c_uint);
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
    (*_this)
        .nbits_total = EC_CODE_BITS + 1 as libc::c_int
        - (EC_CODE_BITS - EC_CODE_EXTRA) / EC_SYM_BITS * EC_SYM_BITS;
    (*_this).offs = 0 as libc::c_int as opus_uint32;
    (*_this).rng = (1 as libc::c_uint) << EC_CODE_EXTRA;
    (*_this).rem = ec_read_byte(_this);
    (*_this)
        .val = ((*_this).rng)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_sub(((*_this).rem >> EC_SYM_BITS - EC_CODE_EXTRA) as libc::c_uint);
    (*_this).error = 0 as libc::c_int;
    ec_dec_normalize(_this);
}
#[no_mangle]
#[c2rust::src_loc = "139:1"]
pub unsafe extern "C" fn ec_decode(
    mut _this: *mut ec_dec,
    mut _ft: libc::c_uint,
) -> libc::c_uint {
    let mut s: libc::c_uint = 0;
    (*_this).ext = celt_udiv((*_this).rng, _ft);
    s = ((*_this).val).wrapping_div((*_this).ext);
    return _ft
        .wrapping_sub(
            s
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    _ft.wrapping_sub(s.wrapping_add(1 as libc::c_int as libc::c_uint))
                        & -((_ft < s.wrapping_add(1 as libc::c_int as libc::c_uint))
                            as libc::c_int) as libc::c_uint,
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
    return ((1 as libc::c_uint) << _bits)
        .wrapping_sub(
            s
                .wrapping_add(1 as libc::c_uint)
                .wrapping_add(
                    ((1 as libc::c_uint) << _bits)
                        .wrapping_sub(s.wrapping_add(1 as libc::c_uint))
                        & -(((1 as libc::c_uint) << _bits
                            < s.wrapping_add(1 as libc::c_uint)) as libc::c_int)
                            as libc::c_uint,
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
    (*_this)
        .val = ((*_this).val as libc::c_uint).wrapping_sub(s) as opus_uint32
        as opus_uint32;
    (*_this)
        .rng = if _fl > 0 as libc::c_int as libc::c_uint {
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
pub unsafe extern "C" fn ec_dec_uint(
    mut _this: *mut ec_dec,
    mut _ft: opus_uint32,
) -> opus_uint32 {
    let mut ft: libc::c_uint = 0;
    let mut s: libc::c_uint = 0;
    let mut ftb: libc::c_int = 0;
    if !(_ft > 1 as libc::c_int as libc::c_uint) {
        celt_fatal(
            b"assertion failed: _ft>1\0" as *const u8 as *const libc::c_char,
            b"celt/entdec.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int,
        );
    }
    _ft = _ft.wrapping_sub(1);
    ftb = EC_CLZ0 - _ft.leading_zeros() as i32;
    if ftb > EC_UINT_BITS {
        let mut t: opus_uint32 = 0;
        ftb -= EC_UINT_BITS;
        ft = (_ft >> ftb).wrapping_add(1 as libc::c_int as libc::c_uint);
        s = ec_decode(_this, ft);
        ec_dec_update(_this, s, s.wrapping_add(1 as libc::c_int as libc::c_uint), ft);
        t = s << ftb | ec_dec_bits(_this, ftb as libc::c_uint);
        if t <= _ft {
            return t;
        }
        (*_this).error = 1 as libc::c_int;
        return _ft;
    } else {
        _ft = _ft.wrapping_add(1);
        s = ec_decode(_this, _ft);
        ec_dec_update(_this, s, s.wrapping_add(1 as libc::c_int as libc::c_uint), _ft);
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
            available += EC_SYM_BITS;
            if !(available <= EC_WINDOW_SIZE - EC_SYM_BITS) {
                break;
            }
        }
    }
    ret = window
        & ((1 as libc::c_int as opus_uint32) << _bits).wrapping_sub(1 as libc::c_uint);
    window >>= _bits;
    available = (available as libc::c_uint).wrapping_sub(_bits) as libc::c_int
        as libc::c_int;
    (*_this).end_window = window;
    (*_this).nend_bits = available;
    (*_this)
        .nbits_total = ((*_this).nbits_total as libc::c_uint).wrapping_add(_bits)
        as libc::c_int as libc::c_int;
    return ret;
}
