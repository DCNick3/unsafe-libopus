use crate::celt::celt::celt_fatal;
use ::libc;

use crate::celt::entdec::{ec_dec, ec_dec_update, ec_decode_bin};
use crate::celt::entenc::{ec_enc, ec_encode_bin};

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
