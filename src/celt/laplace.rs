use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:33"]
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
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.h:33"]
pub mod entdec_h {
    use super::entcode_h::ec_dec;
    extern "C" {
        #[c2rust::src_loc = "69:1"]
        pub fn ec_dec_update(
            _this: *mut ec_dec,
            _fl: libc::c_uint,
            _fh: libc::c_uint,
            _ft: libc::c_uint,
        );
        #[c2rust::src_loc = "54:1"]
        pub fn ec_decode_bin(_this: *mut ec_dec, _bits: libc::c_uint) -> libc::c_uint;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.h:33"]
pub mod entenc_h {
    use super::entcode_h::ec_enc;
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub fn ec_encode_bin(
            _this: *mut ec_enc,
            _fl: libc::c_uint,
            _fh: libc::c_uint,
            _bits: libc::c_uint,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
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
use self::arch_h::celt_fatal;
pub use self::entcode_h::{ec_ctx, ec_dec, ec_enc, ec_window};
use self::entdec_h::{ec_dec_update, ec_decode_bin};
use self::entenc_h::ec_encode_bin;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::types_h::{__int32_t, __uint32_t};
#[c2rust::src_loc = "37:9"]
pub const LAPLACE_LOG_MINP: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "38:9"]
pub const LAPLACE_MINP: libc::c_int = (1 as libc::c_int) << LAPLACE_LOG_MINP;
#[c2rust::src_loc = "41:9"]
pub const LAPLACE_NMIN: libc::c_int = 16 as libc::c_int;
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn ec_laplace_get_freq1(fs0: libc::c_uint, decay: libc::c_int) -> libc::c_uint {
    let mut ft: libc::c_uint = 0;
    ft = ((32768 as libc::c_int - LAPLACE_MINP * (2 as libc::c_int * LAPLACE_NMIN))
        as libc::c_uint)
        .wrapping_sub(fs0);
    return ft.wrapping_mul((16384 as libc::c_int - decay) as libc::c_uint) >> 15 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "51:1"]
pub unsafe extern "C" fn ec_laplace_encode(
    enc: *mut ec_enc,
    value: *mut libc::c_int,
    mut fs: libc::c_uint,
    decay: libc::c_int,
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
            fl =
                fl.wrapping_add(fs.wrapping_add((2 as libc::c_int * LAPLACE_MINP) as libc::c_uint));
            fs = fs.wrapping_mul(decay as libc::c_uint) >> 15 as libc::c_int;
            i += 1;
        }
        if fs == 0 {
            let mut di: libc::c_int = 0;
            let mut ndi_max: libc::c_int = 0;
            ndi_max = ((32768 as libc::c_int as libc::c_uint)
                .wrapping_sub(fl)
                .wrapping_add(LAPLACE_MINP as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                >> LAPLACE_LOG_MINP) as libc::c_int;
            ndi_max = ndi_max - s >> 1 as libc::c_int;
            di = if val - i < ndi_max - 1 as libc::c_int {
                val - i
            } else {
                ndi_max - 1 as libc::c_int
            };
            fl = fl.wrapping_add(
                ((2 as libc::c_int * di + 1 as libc::c_int + s) * LAPLACE_MINP) as libc::c_uint,
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
            fs = fs.wrapping_add(LAPLACE_MINP as libc::c_uint);
            fl = fl.wrapping_add(fs & !s as libc::c_uint);
        }
        if !(fl.wrapping_add(fs) <= 32768 as libc::c_int as libc::c_uint) {
            celt_fatal(
                b"assertion failed: fl+fs<=32768\0" as *const u8 as *const libc::c_char,
                b"celt/laplace.c\0" as *const u8 as *const libc::c_char,
                88 as libc::c_int,
            );
        }
        if !(fs > 0 as libc::c_int as libc::c_uint) {
            celt_fatal(
                b"assertion failed: fs>0\0" as *const u8 as *const libc::c_char,
                b"celt/laplace.c\0" as *const u8 as *const libc::c_char,
                89 as libc::c_int,
            );
        }
    }
    ec_encode_bin(
        enc,
        fl,
        fl.wrapping_add(fs),
        15 as libc::c_int as libc::c_uint,
    );
}
#[no_mangle]
#[c2rust::src_loc = "94:1"]
pub unsafe extern "C" fn ec_laplace_decode(
    dec: *mut ec_dec,
    mut fs: libc::c_uint,
    decay: libc::c_int,
) -> libc::c_int {
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut fl: libc::c_uint = 0;
    let mut fm: libc::c_uint = 0;
    fm = ec_decode_bin(dec, 15 as libc::c_int as libc::c_uint);
    fl = 0 as libc::c_int as libc::c_uint;
    if fm >= fs {
        val += 1;
        fl = fs;
        fs = (ec_laplace_get_freq1(fs, decay)).wrapping_add(LAPLACE_MINP as libc::c_uint);
        while fs > LAPLACE_MINP as libc::c_uint
            && fm >= fl.wrapping_add((2 as libc::c_int as libc::c_uint).wrapping_mul(fs))
        {
            fs = fs.wrapping_mul(2 as libc::c_int as libc::c_uint);
            fl = fl.wrapping_add(fs);
            fs = fs
                .wrapping_sub((2 as libc::c_int * LAPLACE_MINP) as libc::c_uint)
                .wrapping_mul(decay as libc::c_uint)
                >> 15 as libc::c_int;
            fs = fs.wrapping_add(LAPLACE_MINP as libc::c_uint);
            val += 1;
        }
        if fs <= LAPLACE_MINP as libc::c_uint {
            let mut di: libc::c_int = 0;
            di = (fm.wrapping_sub(fl) >> LAPLACE_LOG_MINP + 1 as libc::c_int) as libc::c_int;
            val += di;
            fl = fl.wrapping_add((2 as libc::c_int * di * LAPLACE_MINP) as libc::c_uint);
        }
        if fm < fl.wrapping_add(fs) {
            val = -val;
        } else {
            fl = fl.wrapping_add(fs);
        }
    }
    if !(fl < 32768 as libc::c_int as libc::c_uint) {
        celt_fatal(
            b"assertion failed: fl<32768\0" as *const u8 as *const libc::c_char,
            b"celt/laplace.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
        );
    }
    if !(fs > 0 as libc::c_int as libc::c_uint) {
        celt_fatal(
            b"assertion failed: fs>0\0" as *const u8 as *const libc::c_char,
            b"celt/laplace.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int,
        );
    }
    if !(fl <= fm) {
        celt_fatal(
            b"assertion failed: fl<=fm\0" as *const u8 as *const libc::c_char,
            b"celt/laplace.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int,
        );
    }
    if !(fm
        < (if fl.wrapping_add(fs) < 32768 as libc::c_int as libc::c_uint {
            fl.wrapping_add(fs)
        } else {
            32768 as libc::c_int as libc::c_uint
        }))
    {
        celt_fatal(
            b"assertion failed: fm<IMIN(fl+fs,32768)\0" as *const u8 as *const libc::c_char,
            b"celt/laplace.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
        );
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
