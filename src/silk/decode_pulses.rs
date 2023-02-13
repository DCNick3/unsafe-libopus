use crate::celt::celt::celt_fatal;
use crate::celt::entdec::{ec_dec, ec_dec_icdf};
use crate::externs::memset;
use crate::silk::code_signs::silk_decode_signs;
use crate::silk::define::{N_RATE_LEVELS, SHELL_CODEC_FRAME_LENGTH, SILK_MAX_PULSES};
use crate::silk::shell_coder::silk_shell_decoder;
use crate::silk::tables_other::silk_lsb_iCDF;
use crate::silk::tables_pulses_per_block::{silk_pulses_per_block_iCDF, silk_rate_levels_iCDF};

#[c2rust::src_loc = "37:1"]
pub unsafe fn silk_decode_pulses(
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
