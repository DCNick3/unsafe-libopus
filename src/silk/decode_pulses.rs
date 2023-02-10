use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:32"]
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
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:32"]
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
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.h:32"]
pub mod entdec_h {
    use super::entcode_h::ec_dec;
    extern "C" {
        #[c2rust::src_loc = "82:1"]
        pub fn ec_dec_icdf(
            _this: *mut ec_dec,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "50:26"]
        pub static silk_pulses_per_block_iCDF: [[u8; 18]; 10];
        #[c2rust::src_loc = "53:26"]
        pub static silk_rate_levels_iCDF: [[u8; 9]; 2];
        #[c2rust::src_loc = "64:26"]
        pub static silk_lsb_iCDF: [u8; 2];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use super::entcode_h::ec_dec;
    extern "C" {
        #[c2rust::src_loc = "125:1"]
        pub fn silk_decode_signs(
            psRangeDec: *mut ec_dec,
            pulses: *mut i16,
            length: libc::c_int,
            signalType: libc::c_int,
            quantOffsetType: libc::c_int,
            sum_pulses: *const libc::c_int,
        );
        #[c2rust::src_loc = "171:1"]
        pub fn silk_shell_decoder(pulses0: *mut i16, psRangeDec: *mut ec_dec, pulses4: libc::c_int);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "168:9"]
    pub const SHELL_CODEC_FRAME_LENGTH: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "173:9"]
    pub const N_RATE_LEVELS: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "176:9"]
    pub const SILK_MAX_PULSES: libc::c_int = 16 as libc::c_int;
}
use self::arch_h::celt_fatal;
pub use self::define_h::{N_RATE_LEVELS, SHELL_CODEC_FRAME_LENGTH, SILK_MAX_PULSES};
pub use self::entcode_h::{ec_ctx, ec_dec, ec_window};
use self::entdec_h::ec_dec_icdf;
use self::main_h::{silk_decode_signs, silk_shell_decoder};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::string_h::memset;
use self::tables_h::{silk_lsb_iCDF, silk_pulses_per_block_iCDF, silk_rate_levels_iCDF};
pub use self::types_h::{__int16_t, __int32_t, __uint32_t, __uint8_t};
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn silk_decode_pulses(
    psRangeDec: *mut ec_dec,
    pulses: *mut i16,
    signalType: libc::c_int,
    quantOffsetType: libc::c_int,
    frame_length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut abs_q: libc::c_int = 0;
    let mut nLS: libc::c_int = 0;
    let mut RateLevelIndex: libc::c_int = 0;
    let mut sum_pulses: [libc::c_int; 20] = [0; 20];
    let mut nLshifts: [libc::c_int; 20] = [0; 20];
    let mut pulses_ptr: *mut i16 = 0 as *mut i16;
    let mut cdf_ptr: *const u8 = 0 as *const u8;
    RateLevelIndex = ec_dec_icdf(
        psRangeDec,
        (silk_rate_levels_iCDF[(signalType >> 1 as libc::c_int) as usize]).as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
    iter = frame_length >> 4 as libc::c_int;
    if iter * SHELL_CODEC_FRAME_LENGTH < frame_length {
        if !(frame_length == 12 as libc::c_int * 10 as libc::c_int) {
            celt_fatal(
                b"assertion failed: frame_length == 12 * 10\0" as *const u8 as *const libc::c_char,
                b"silk/decode_pulses.c\0" as *const u8 as *const libc::c_char,
                59 as libc::c_int,
            );
        }
        iter += 1;
    }
    cdf_ptr = (silk_pulses_per_block_iCDF[RateLevelIndex as usize]).as_ptr();
    i = 0 as libc::c_int;
    while i < iter {
        nLshifts[i as usize] = 0 as libc::c_int;
        sum_pulses[i as usize] = ec_dec_icdf(psRangeDec, cdf_ptr, 8 as libc::c_int as libc::c_uint);
        while sum_pulses[i as usize] == SILK_MAX_PULSES + 1 as libc::c_int {
            nLshifts[i as usize] += 1;
            sum_pulses[i as usize] = ec_dec_icdf(
                psRangeDec,
                (silk_pulses_per_block_iCDF[(N_RATE_LEVELS - 1 as libc::c_int) as usize])
                    .as_ptr()
                    .offset((nLshifts[i as usize] == 10 as libc::c_int) as libc::c_int as isize),
                8 as libc::c_int as libc::c_uint,
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < iter {
        if sum_pulses[i as usize] > 0 as libc::c_int {
            silk_shell_decoder(
                &mut *pulses.offset((i as i16 as i32 * 16 as libc::c_int as i16 as i32) as isize),
                psRangeDec,
                sum_pulses[i as usize],
            );
        } else {
            memset(
                &mut *pulses.offset((i as i16 as i32 * 16 as libc::c_int as i16 as i32) as isize)
                    as *mut i16 as *mut libc::c_void,
                0 as libc::c_int,
                (16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < iter {
        if nLshifts[i as usize] > 0 as libc::c_int {
            nLS = nLshifts[i as usize];
            pulses_ptr = &mut *pulses
                .offset((i as i16 as i32 * 16 as libc::c_int as i16 as i32) as isize)
                as *mut i16;
            k = 0 as libc::c_int;
            while k < SHELL_CODEC_FRAME_LENGTH {
                abs_q = *pulses_ptr.offset(k as isize) as libc::c_int;
                j = 0 as libc::c_int;
                while j < nLS {
                    abs_q = ((abs_q as u32) << 1 as libc::c_int) as i32;
                    abs_q += ec_dec_icdf(
                        psRangeDec,
                        silk_lsb_iCDF.as_ptr(),
                        8 as libc::c_int as libc::c_uint,
                    );
                    j += 1;
                }
                *pulses_ptr.offset(k as isize) = abs_q as i16;
                k += 1;
            }
            sum_pulses[i as usize] |= nLS << 5 as libc::c_int;
        }
        i += 1;
    }
    silk_decode_signs(
        psRangeDec,
        pulses,
        frame_length,
        signalType,
        quantOffsetType,
        sum_pulses.as_mut_ptr() as *const libc::c_int,
    );
}
