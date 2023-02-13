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
    signalType: i32,
    quantOffsetType: i32,
    frame_length: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut iter: i32 = 0;
    let mut abs_q: i32 = 0;
    let mut nLS: i32 = 0;
    let mut RateLevelIndex: i32 = 0;
    let mut sum_pulses: [i32; 20] = [0; 20];
    let mut nLshifts: [i32; 20] = [0; 20];
    let mut pulses_ptr: *mut i16 = 0 as *mut i16;
    let mut cdf_ptr: *const u8 = 0 as *const u8;
    RateLevelIndex = ec_dec_icdf(
        psRangeDec,
        (silk_rate_levels_iCDF[(signalType >> 1 as i32) as usize]).as_ptr(),
        8 as i32 as u32,
    );
    iter = frame_length >> 4 as i32;
    if iter * SHELL_CODEC_FRAME_LENGTH < frame_length {
        if !(frame_length == 12 as i32 * 10 as i32) {
            celt_fatal(
                b"assertion failed: frame_length == 12 * 10\0" as *const u8 as *const i8,
                b"silk/decode_pulses.c\0" as *const u8 as *const i8,
                59 as i32,
            );
        }
        iter += 1;
    }
    cdf_ptr = (silk_pulses_per_block_iCDF[RateLevelIndex as usize]).as_ptr();
    i = 0 as i32;
    while i < iter {
        nLshifts[i as usize] = 0 as i32;
        sum_pulses[i as usize] = ec_dec_icdf(psRangeDec, cdf_ptr, 8 as i32 as u32);
        while sum_pulses[i as usize] == SILK_MAX_PULSES + 1 as i32 {
            nLshifts[i as usize] += 1;
            sum_pulses[i as usize] = ec_dec_icdf(
                psRangeDec,
                (silk_pulses_per_block_iCDF[(N_RATE_LEVELS - 1 as i32) as usize])
                    .as_ptr()
                    .offset((nLshifts[i as usize] == 10 as i32) as i32 as isize),
                8 as i32 as u32,
            );
        }
        i += 1;
    }
    i = 0 as i32;
    while i < iter {
        if sum_pulses[i as usize] > 0 as i32 {
            silk_shell_decoder(
                &mut *pulses.offset((i as i16 as i32 * 16 as i32 as i16 as i32) as isize),
                psRangeDec,
                sum_pulses[i as usize],
            );
        } else {
            memset(
                &mut *pulses.offset((i as i16 as i32 * 16 as i32 as i16 as i32) as isize)
                    as *mut i16 as *mut core::ffi::c_void,
                0 as i32,
                (16 as i32 as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64),
            );
        }
        i += 1;
    }
    i = 0 as i32;
    while i < iter {
        if nLshifts[i as usize] > 0 as i32 {
            nLS = nLshifts[i as usize];
            pulses_ptr = &mut *pulses.offset((i as i16 as i32 * 16 as i32 as i16 as i32) as isize)
                as *mut i16;
            k = 0 as i32;
            while k < SHELL_CODEC_FRAME_LENGTH {
                abs_q = *pulses_ptr.offset(k as isize) as i32;
                j = 0 as i32;
                while j < nLS {
                    abs_q = ((abs_q as u32) << 1 as i32) as i32;
                    abs_q += ec_dec_icdf(psRangeDec, silk_lsb_iCDF.as_ptr(), 8 as i32 as u32);
                    j += 1;
                }
                *pulses_ptr.offset(k as isize) = abs_q as i16;
                k += 1;
            }
            sum_pulses[i as usize] |= nLS << 5 as i32;
        }
        i += 1;
    }
    silk_decode_signs(
        psRangeDec,
        pulses,
        frame_length,
        signalType,
        quantOffsetType,
        sum_pulses.as_mut_ptr() as *const i32,
    );
}
