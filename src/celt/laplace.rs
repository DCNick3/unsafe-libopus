use crate::celt::celt::celt_fatal;

use crate::celt::entdec::{ec_dec, ec_dec_update, ec_decode_bin};
use crate::celt::entenc::{ec_enc, ec_encode_bin};

#[c2rust::src_loc = "37:9"]
pub const LAPLACE_LOG_MINP: i32 = 0 as i32;
#[c2rust::src_loc = "38:9"]
pub const LAPLACE_MINP: i32 = (1 as i32) << LAPLACE_LOG_MINP;
#[c2rust::src_loc = "41:9"]
pub const LAPLACE_NMIN: i32 = 16 as i32;
#[c2rust::src_loc = "44:1"]
unsafe fn ec_laplace_get_freq1(fs0: u32, decay: i32) -> u32 {
    let mut ft: u32 = 0;
    ft = ((32768 as i32 - LAPLACE_MINP * (2 as i32 * LAPLACE_NMIN)) as u32).wrapping_sub(fs0);
    return ft.wrapping_mul((16384 as i32 - decay) as u32) >> 15 as i32;
}
#[c2rust::src_loc = "51:1"]
pub unsafe fn ec_laplace_encode(enc: *mut ec_enc, value: *mut i32, mut fs: u32, decay: i32) {
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
            fl = fl.wrapping_add(fs.wrapping_add((2 as i32 * LAPLACE_MINP) as u32));
            fs = fs.wrapping_mul(decay as u32) >> 15 as i32;
            i += 1;
        }
        if fs == 0 {
            let mut di: i32 = 0;
            let mut ndi_max: i32 = 0;
            ndi_max = ((32768 as i32 as u32)
                .wrapping_sub(fl)
                .wrapping_add(LAPLACE_MINP as u32)
                .wrapping_sub(1 as i32 as u32)
                >> LAPLACE_LOG_MINP) as i32;
            ndi_max = ndi_max - s >> 1 as i32;
            di = if val - i < ndi_max - 1 as i32 {
                val - i
            } else {
                ndi_max - 1 as i32
            };
            fl = fl.wrapping_add(((2 as i32 * di + 1 as i32 + s) * LAPLACE_MINP) as u32);
            fs = if (((1 as i32) << 0 as i32) as u32) < (32768 as i32 as u32).wrapping_sub(fl) {
                ((1 as i32) << 0 as i32) as u32
            } else {
                (32768 as i32 as u32).wrapping_sub(fl)
            };
            *value = i + di + s ^ s;
        } else {
            fs = fs.wrapping_add(LAPLACE_MINP as u32);
            fl = fl.wrapping_add(fs & !s as u32);
        }
        if !(fl.wrapping_add(fs) <= 32768 as i32 as u32) {
            celt_fatal(
                b"assertion failed: fl+fs<=32768\0" as *const u8 as *const i8,
                b"celt/laplace.c\0" as *const u8 as *const i8,
                88 as i32,
            );
        }
        if !(fs > 0 as i32 as u32) {
            celt_fatal(
                b"assertion failed: fs>0\0" as *const u8 as *const i8,
                b"celt/laplace.c\0" as *const u8 as *const i8,
                89 as i32,
            );
        }
    }
    ec_encode_bin(enc, fl, fl.wrapping_add(fs), 15 as i32 as u32);
}
#[c2rust::src_loc = "94:1"]
pub unsafe fn ec_laplace_decode(dec: *mut ec_dec, mut fs: u32, decay: i32) -> i32 {
    let mut val: i32 = 0 as i32;
    let mut fl: u32 = 0;
    let mut fm: u32 = 0;
    fm = ec_decode_bin(dec, 15 as i32 as u32);
    fl = 0 as i32 as u32;
    if fm >= fs {
        val += 1;
        fl = fs;
        fs = (ec_laplace_get_freq1(fs, decay)).wrapping_add(LAPLACE_MINP as u32);
        while fs > LAPLACE_MINP as u32 && fm >= fl.wrapping_add((2 as i32 as u32).wrapping_mul(fs))
        {
            fs = fs.wrapping_mul(2 as i32 as u32);
            fl = fl.wrapping_add(fs);
            fs = fs
                .wrapping_sub((2 as i32 * LAPLACE_MINP) as u32)
                .wrapping_mul(decay as u32)
                >> 15 as i32;
            fs = fs.wrapping_add(LAPLACE_MINP as u32);
            val += 1;
        }
        if fs <= LAPLACE_MINP as u32 {
            let mut di: i32 = 0;
            di = (fm.wrapping_sub(fl) >> LAPLACE_LOG_MINP + 1 as i32) as i32;
            val += di;
            fl = fl.wrapping_add((2 as i32 * di * LAPLACE_MINP) as u32);
        }
        if fm < fl.wrapping_add(fs) {
            val = -val;
        } else {
            fl = fl.wrapping_add(fs);
        }
    }
    if !(fl < 32768 as i32 as u32) {
        celt_fatal(
            b"assertion failed: fl<32768\0" as *const u8 as *const i8,
            b"celt/laplace.c\0" as *const u8 as *const i8,
            128 as i32,
        );
    }
    if !(fs > 0 as i32 as u32) {
        celt_fatal(
            b"assertion failed: fs>0\0" as *const u8 as *const i8,
            b"celt/laplace.c\0" as *const u8 as *const i8,
            129 as i32,
        );
    }
    if !(fl <= fm) {
        celt_fatal(
            b"assertion failed: fl<=fm\0" as *const u8 as *const i8,
            b"celt/laplace.c\0" as *const u8 as *const i8,
            130 as i32,
        );
    }
    if !(fm
        < (if fl.wrapping_add(fs) < 32768 as i32 as u32 {
            fl.wrapping_add(fs)
        } else {
            32768 as i32 as u32
        }))
    {
        celt_fatal(
            b"assertion failed: fm<IMIN(fl+fs,32768)\0" as *const u8 as *const i8,
            b"celt/laplace.c\0" as *const u8 as *const i8,
            131 as i32,
        );
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
