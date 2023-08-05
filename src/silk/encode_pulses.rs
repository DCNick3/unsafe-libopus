pub mod typedef_h {
    pub const silk_int32_MAX: i32 = i32::MAX;
}
pub use self::typedef_h::silk_int32_MAX;
use crate::celt::entenc::{ec_enc, ec_enc_icdf};
use crate::externs::memset;
use crate::silk::code_signs::silk_encode_signs;
use crate::silk::define::{N_RATE_LEVELS, SHELL_CODEC_FRAME_LENGTH, SILK_MAX_PULSES};
use crate::silk::shell_coder::silk_shell_encoder;
use crate::silk::tables_other::silk_lsb_iCDF;
use crate::silk::tables_pulses_per_block::{
    silk_max_pulses_table, silk_pulses_per_block_BITS_Q5, silk_pulses_per_block_iCDF,
    silk_rate_levels_BITS_Q5, silk_rate_levels_iCDF,
};

#[inline]
unsafe fn combine_and_check(
    pulses_comb: *mut i32,
    pulses_in: *const i32,
    max_pulses: i32,
    len: i32,
) -> i32 {
    let mut k: i32 = 0;
    let mut sum: i32 = 0;
    k = 0 as i32;
    while k < len {
        sum = *pulses_in.offset((2 as i32 * k) as isize)
            + *pulses_in.offset((2 as i32 * k + 1 as i32) as isize);
        if sum > max_pulses {
            return 1 as i32;
        }
        *pulses_comb.offset(k as isize) = sum;
        k += 1;
    }
    return 0 as i32;
}
pub unsafe fn silk_encode_pulses(
    psRangeEnc: *mut ec_enc,
    signalType: i32,
    quantOffsetType: i32,
    pulses: *mut i8,
    frame_length: i32,
) {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut j: i32 = 0;
    let mut iter: i32 = 0;
    let mut bit: i32 = 0;
    let mut nLS: i32 = 0;
    let mut scale_down: i32 = 0;
    let mut RateLevelIndex: i32 = 0 as i32;
    let mut abs_q: i32 = 0;
    let mut minSumBits_Q5: i32 = 0;
    let mut sumBits_Q5: i32 = 0;
    let mut pulses_comb: [i32; 8] = [0; 8];
    let mut abs_pulses_ptr: *mut i32 = 0 as *mut i32;
    let mut pulses_ptr: *const i8 = 0 as *const i8;
    let mut cdf_ptr: *const u8 = 0 as *const u8;
    let mut nBits_ptr: *const u8 = 0 as *const u8;
    memset(
        pulses_comb.as_mut_ptr() as *mut core::ffi::c_void,
        0 as i32,
        (8 as i32 as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    iter = frame_length >> 4 as i32;
    if iter * SHELL_CODEC_FRAME_LENGTH < frame_length {
        assert!(frame_length == 12 as i32 * 10 as i32);
        iter += 1;
        memset(
            &mut *pulses.offset(frame_length as isize) as *mut i8 as *mut core::ffi::c_void,
            0 as i32,
            (16 as i32 as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
    }
    let vla = (iter * 16 as i32) as usize;
    let mut abs_pulses: Vec<i32> = ::std::vec::from_elem(0, vla);
    i = 0 as i32;
    while i < iter * SHELL_CODEC_FRAME_LENGTH {
        *abs_pulses.as_mut_ptr().offset((i + 0 as i32) as isize) =
            if *pulses.offset((i + 0 as i32) as isize) as i32 > 0 as i32 {
                *pulses.offset((i + 0 as i32) as isize) as i32
            } else {
                -(*pulses.offset((i + 0 as i32) as isize) as i32)
            };
        *abs_pulses.as_mut_ptr().offset((i + 1 as i32) as isize) =
            if *pulses.offset((i + 1 as i32) as isize) as i32 > 0 as i32 {
                *pulses.offset((i + 1 as i32) as isize) as i32
            } else {
                -(*pulses.offset((i + 1 as i32) as isize) as i32)
            };
        *abs_pulses.as_mut_ptr().offset((i + 2 as i32) as isize) =
            if *pulses.offset((i + 2 as i32) as isize) as i32 > 0 as i32 {
                *pulses.offset((i + 2 as i32) as isize) as i32
            } else {
                -(*pulses.offset((i + 2 as i32) as isize) as i32)
            };
        *abs_pulses.as_mut_ptr().offset((i + 3 as i32) as isize) =
            if *pulses.offset((i + 3 as i32) as isize) as i32 > 0 as i32 {
                *pulses.offset((i + 3 as i32) as isize) as i32
            } else {
                -(*pulses.offset((i + 3 as i32) as isize) as i32)
            };
        i += 4 as i32;
    }
    let vla_0 = iter as usize;
    let mut sum_pulses: Vec<i32> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = iter as usize;
    let mut nRshifts: Vec<i32> = ::std::vec::from_elem(0, vla_1);
    abs_pulses_ptr = abs_pulses.as_mut_ptr();
    i = 0 as i32;
    while i < iter {
        *nRshifts.as_mut_ptr().offset(i as isize) = 0 as i32;
        loop {
            scale_down = combine_and_check(
                pulses_comb.as_mut_ptr(),
                abs_pulses_ptr,
                silk_max_pulses_table[0 as i32 as usize] as i32,
                8 as i32,
            );
            scale_down += combine_and_check(
                pulses_comb.as_mut_ptr(),
                pulses_comb.as_mut_ptr(),
                silk_max_pulses_table[1 as i32 as usize] as i32,
                4 as i32,
            );
            scale_down += combine_and_check(
                pulses_comb.as_mut_ptr(),
                pulses_comb.as_mut_ptr(),
                silk_max_pulses_table[2 as i32 as usize] as i32,
                2 as i32,
            );
            scale_down += combine_and_check(
                &mut *sum_pulses.as_mut_ptr().offset(i as isize),
                pulses_comb.as_mut_ptr(),
                silk_max_pulses_table[3 as i32 as usize] as i32,
                1 as i32,
            );
            if !(scale_down != 0) {
                break;
            }
            let ref mut fresh0 = *nRshifts.as_mut_ptr().offset(i as isize);
            *fresh0 += 1;
            k = 0 as i32;
            while k < SHELL_CODEC_FRAME_LENGTH {
                *abs_pulses_ptr.offset(k as isize) = *abs_pulses_ptr.offset(k as isize) >> 1 as i32;
                k += 1;
            }
        }
        abs_pulses_ptr = abs_pulses_ptr.offset(SHELL_CODEC_FRAME_LENGTH as isize);
        i += 1;
    }
    minSumBits_Q5 = silk_int32_MAX;
    k = 0 as i32;
    while k < N_RATE_LEVELS - 1 as i32 {
        nBits_ptr = (silk_pulses_per_block_BITS_Q5[k as usize]).as_ptr();
        sumBits_Q5 = silk_rate_levels_BITS_Q5[(signalType >> 1 as i32) as usize][k as usize] as i32;
        i = 0 as i32;
        while i < iter {
            if *nRshifts.as_mut_ptr().offset(i as isize) > 0 as i32 {
                sumBits_Q5 += *nBits_ptr.offset((SILK_MAX_PULSES + 1 as i32) as isize) as i32;
            } else {
                sumBits_Q5 +=
                    *nBits_ptr.offset(*sum_pulses.as_mut_ptr().offset(i as isize) as isize) as i32;
            }
            i += 1;
        }
        if sumBits_Q5 < minSumBits_Q5 {
            minSumBits_Q5 = sumBits_Q5;
            RateLevelIndex = k;
        }
        k += 1;
    }
    ec_enc_icdf(
        psRangeEnc,
        RateLevelIndex,
        (silk_rate_levels_iCDF[(signalType >> 1 as i32) as usize]).as_ptr(),
        8 as i32 as u32,
    );
    cdf_ptr = (silk_pulses_per_block_iCDF[RateLevelIndex as usize]).as_ptr();
    i = 0 as i32;
    while i < iter {
        if *nRshifts.as_mut_ptr().offset(i as isize) == 0 as i32 {
            ec_enc_icdf(
                psRangeEnc,
                *sum_pulses.as_mut_ptr().offset(i as isize),
                cdf_ptr,
                8 as i32 as u32,
            );
        } else {
            ec_enc_icdf(
                psRangeEnc,
                SILK_MAX_PULSES + 1 as i32,
                cdf_ptr,
                8 as i32 as u32,
            );
            k = 0 as i32;
            while k < *nRshifts.as_mut_ptr().offset(i as isize) - 1 as i32 {
                ec_enc_icdf(
                    psRangeEnc,
                    SILK_MAX_PULSES + 1 as i32,
                    (silk_pulses_per_block_iCDF[(N_RATE_LEVELS - 1 as i32) as usize]).as_ptr(),
                    8 as i32 as u32,
                );
                k += 1;
            }
            ec_enc_icdf(
                psRangeEnc,
                *sum_pulses.as_mut_ptr().offset(i as isize),
                (silk_pulses_per_block_iCDF[(N_RATE_LEVELS - 1 as i32) as usize]).as_ptr(),
                8 as i32 as u32,
            );
        }
        i += 1;
    }
    i = 0 as i32;
    while i < iter {
        if *sum_pulses.as_mut_ptr().offset(i as isize) > 0 as i32 {
            silk_shell_encoder(
                psRangeEnc,
                &mut *abs_pulses
                    .as_mut_ptr()
                    .offset((i * SHELL_CODEC_FRAME_LENGTH) as isize),
            );
        }
        i += 1;
    }
    i = 0 as i32;
    while i < iter {
        if *nRshifts.as_mut_ptr().offset(i as isize) > 0 as i32 {
            pulses_ptr = &mut *pulses.offset((i * SHELL_CODEC_FRAME_LENGTH) as isize) as *mut i8;
            nLS = *nRshifts.as_mut_ptr().offset(i as isize) - 1 as i32;
            k = 0 as i32;
            while k < SHELL_CODEC_FRAME_LENGTH {
                abs_q = (if *pulses_ptr.offset(k as isize) as i32 > 0 as i32 {
                    *pulses_ptr.offset(k as isize) as i32
                } else {
                    -(*pulses_ptr.offset(k as isize) as i32)
                }) as i8 as i32;
                j = nLS;
                while j > 0 as i32 {
                    bit = abs_q >> j & 1 as i32;
                    ec_enc_icdf(psRangeEnc, bit, silk_lsb_iCDF.as_ptr(), 8 as i32 as u32);
                    j -= 1;
                }
                bit = abs_q & 1 as i32;
                ec_enc_icdf(psRangeEnc, bit, silk_lsb_iCDF.as_ptr(), 8 as i32 as u32);
                k += 1;
            }
        }
        i += 1;
    }
    silk_encode_signs(
        psRangeEnc,
        pulses as *const i8,
        frame_length,
        signalType,
        quantOffsetType,
        sum_pulses.as_mut_ptr() as *const i32,
    );
}
