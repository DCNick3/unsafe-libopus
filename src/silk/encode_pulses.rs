use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
}
pub use self::typedef_h::silk_int32_MAX;
use crate::celt::celt::celt_fatal;
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
#[c2rust::src_loc = "39:1"]
unsafe fn combine_and_check(
    pulses_comb: *mut libc::c_int,
    pulses_in: *const libc::c_int,
    max_pulses: libc::c_int,
    len: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < len {
        sum = *pulses_in.offset((2 as libc::c_int * k) as isize)
            + *pulses_in.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize);
        if sum > max_pulses {
            return 1 as libc::c_int;
        }
        *pulses_comb.offset(k as isize) = sum;
        k += 1;
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "60:1"]
pub unsafe fn silk_encode_pulses(
    psRangeEnc: *mut ec_enc,
    signalType: libc::c_int,
    quantOffsetType: libc::c_int,
    pulses: *mut i8,
    frame_length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut nLS: libc::c_int = 0;
    let mut scale_down: libc::c_int = 0;
    let mut RateLevelIndex: libc::c_int = 0 as libc::c_int;
    let mut abs_q: i32 = 0;
    let mut minSumBits_Q5: i32 = 0;
    let mut sumBits_Q5: i32 = 0;
    let mut pulses_comb: [libc::c_int; 8] = [0; 8];
    let mut abs_pulses_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pulses_ptr: *const i8 = 0 as *const i8;
    let mut cdf_ptr: *const u8 = 0 as *const u8;
    let mut nBits_ptr: *const u8 = 0 as *const u8;
    memset(
        pulses_comb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    iter = frame_length >> 4 as libc::c_int;
    if iter * SHELL_CODEC_FRAME_LENGTH < frame_length {
        if !(frame_length == 12 as libc::c_int * 10 as libc::c_int) {
            celt_fatal(
                b"assertion failed: frame_length == 12 * 10\0" as *const u8 as *const libc::c_char,
                b"silk/encode_pulses.c\0" as *const u8 as *const libc::c_char,
                89 as libc::c_int,
            );
        }
        iter += 1;
        memset(
            &mut *pulses.offset(frame_length as isize) as *mut i8 as *mut libc::c_void,
            0 as libc::c_int,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i8>() as libc::c_ulong),
        );
    }
    let vla = (iter * 16 as libc::c_int) as usize;
    let mut abs_pulses: Vec<libc::c_int> = ::std::vec::from_elem(0, vla);
    i = 0 as libc::c_int;
    while i < iter * SHELL_CODEC_FRAME_LENGTH {
        *abs_pulses
            .as_mut_ptr()
            .offset((i + 0 as libc::c_int) as isize) =
            if *pulses.offset((i + 0 as libc::c_int) as isize) as libc::c_int > 0 as libc::c_int {
                *pulses.offset((i + 0 as libc::c_int) as isize) as libc::c_int
            } else {
                -(*pulses.offset((i + 0 as libc::c_int) as isize) as libc::c_int)
            };
        *abs_pulses
            .as_mut_ptr()
            .offset((i + 1 as libc::c_int) as isize) =
            if *pulses.offset((i + 1 as libc::c_int) as isize) as libc::c_int > 0 as libc::c_int {
                *pulses.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            } else {
                -(*pulses.offset((i + 1 as libc::c_int) as isize) as libc::c_int)
            };
        *abs_pulses
            .as_mut_ptr()
            .offset((i + 2 as libc::c_int) as isize) =
            if *pulses.offset((i + 2 as libc::c_int) as isize) as libc::c_int > 0 as libc::c_int {
                *pulses.offset((i + 2 as libc::c_int) as isize) as libc::c_int
            } else {
                -(*pulses.offset((i + 2 as libc::c_int) as isize) as libc::c_int)
            };
        *abs_pulses
            .as_mut_ptr()
            .offset((i + 3 as libc::c_int) as isize) =
            if *pulses.offset((i + 3 as libc::c_int) as isize) as libc::c_int > 0 as libc::c_int {
                *pulses.offset((i + 3 as libc::c_int) as isize) as libc::c_int
            } else {
                -(*pulses.offset((i + 3 as libc::c_int) as isize) as libc::c_int)
            };
        i += 4 as libc::c_int;
    }
    let vla_0 = iter as usize;
    let mut sum_pulses: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = iter as usize;
    let mut nRshifts: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_1);
    abs_pulses_ptr = abs_pulses.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < iter {
        *nRshifts.as_mut_ptr().offset(i as isize) = 0 as libc::c_int;
        loop {
            scale_down = combine_and_check(
                pulses_comb.as_mut_ptr(),
                abs_pulses_ptr,
                silk_max_pulses_table[0 as libc::c_int as usize] as libc::c_int,
                8 as libc::c_int,
            );
            scale_down += combine_and_check(
                pulses_comb.as_mut_ptr(),
                pulses_comb.as_mut_ptr(),
                silk_max_pulses_table[1 as libc::c_int as usize] as libc::c_int,
                4 as libc::c_int,
            );
            scale_down += combine_and_check(
                pulses_comb.as_mut_ptr(),
                pulses_comb.as_mut_ptr(),
                silk_max_pulses_table[2 as libc::c_int as usize] as libc::c_int,
                2 as libc::c_int,
            );
            scale_down += combine_and_check(
                &mut *sum_pulses.as_mut_ptr().offset(i as isize),
                pulses_comb.as_mut_ptr(),
                silk_max_pulses_table[3 as libc::c_int as usize] as libc::c_int,
                1 as libc::c_int,
            );
            if !(scale_down != 0) {
                break;
            }
            let ref mut fresh0 = *nRshifts.as_mut_ptr().offset(i as isize);
            *fresh0 += 1;
            k = 0 as libc::c_int;
            while k < SHELL_CODEC_FRAME_LENGTH {
                *abs_pulses_ptr.offset(k as isize) =
                    *abs_pulses_ptr.offset(k as isize) >> 1 as libc::c_int;
                k += 1;
            }
        }
        abs_pulses_ptr = abs_pulses_ptr.offset(SHELL_CODEC_FRAME_LENGTH as isize);
        i += 1;
    }
    minSumBits_Q5 = silk_int32_MAX;
    k = 0 as libc::c_int;
    while k < N_RATE_LEVELS - 1 as libc::c_int {
        nBits_ptr = (silk_pulses_per_block_BITS_Q5[k as usize]).as_ptr();
        sumBits_Q5 =
            silk_rate_levels_BITS_Q5[(signalType >> 1 as libc::c_int) as usize][k as usize] as i32;
        i = 0 as libc::c_int;
        while i < iter {
            if *nRshifts.as_mut_ptr().offset(i as isize) > 0 as libc::c_int {
                sumBits_Q5 +=
                    *nBits_ptr.offset((SILK_MAX_PULSES + 1 as libc::c_int) as isize) as libc::c_int;
            } else {
                sumBits_Q5 += *nBits_ptr
                    .offset(*sum_pulses.as_mut_ptr().offset(i as isize) as isize)
                    as libc::c_int;
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
        (silk_rate_levels_iCDF[(signalType >> 1 as libc::c_int) as usize]).as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
    cdf_ptr = (silk_pulses_per_block_iCDF[RateLevelIndex as usize]).as_ptr();
    i = 0 as libc::c_int;
    while i < iter {
        if *nRshifts.as_mut_ptr().offset(i as isize) == 0 as libc::c_int {
            ec_enc_icdf(
                psRangeEnc,
                *sum_pulses.as_mut_ptr().offset(i as isize),
                cdf_ptr,
                8 as libc::c_int as libc::c_uint,
            );
        } else {
            ec_enc_icdf(
                psRangeEnc,
                SILK_MAX_PULSES + 1 as libc::c_int,
                cdf_ptr,
                8 as libc::c_int as libc::c_uint,
            );
            k = 0 as libc::c_int;
            while k < *nRshifts.as_mut_ptr().offset(i as isize) - 1 as libc::c_int {
                ec_enc_icdf(
                    psRangeEnc,
                    SILK_MAX_PULSES + 1 as libc::c_int,
                    (silk_pulses_per_block_iCDF[(N_RATE_LEVELS - 1 as libc::c_int) as usize])
                        .as_ptr(),
                    8 as libc::c_int as libc::c_uint,
                );
                k += 1;
            }
            ec_enc_icdf(
                psRangeEnc,
                *sum_pulses.as_mut_ptr().offset(i as isize),
                (silk_pulses_per_block_iCDF[(N_RATE_LEVELS - 1 as libc::c_int) as usize]).as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < iter {
        if *sum_pulses.as_mut_ptr().offset(i as isize) > 0 as libc::c_int {
            silk_shell_encoder(
                psRangeEnc,
                &mut *abs_pulses
                    .as_mut_ptr()
                    .offset((i * SHELL_CODEC_FRAME_LENGTH) as isize),
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < iter {
        if *nRshifts.as_mut_ptr().offset(i as isize) > 0 as libc::c_int {
            pulses_ptr = &mut *pulses.offset((i * SHELL_CODEC_FRAME_LENGTH) as isize) as *mut i8;
            nLS = *nRshifts.as_mut_ptr().offset(i as isize) - 1 as libc::c_int;
            k = 0 as libc::c_int;
            while k < SHELL_CODEC_FRAME_LENGTH {
                abs_q = (if *pulses_ptr.offset(k as isize) as libc::c_int > 0 as libc::c_int {
                    *pulses_ptr.offset(k as isize) as libc::c_int
                } else {
                    -(*pulses_ptr.offset(k as isize) as libc::c_int)
                }) as i8 as i32;
                j = nLS;
                while j > 0 as libc::c_int {
                    bit = abs_q >> j & 1 as libc::c_int;
                    ec_enc_icdf(
                        psRangeEnc,
                        bit,
                        silk_lsb_iCDF.as_ptr(),
                        8 as libc::c_int as libc::c_uint,
                    );
                    j -= 1;
                }
                bit = abs_q & 1 as libc::c_int;
                ec_enc_icdf(
                    psRangeEnc,
                    bit,
                    silk_lsb_iCDF.as_ptr(),
                    8 as libc::c_int as libc::c_uint,
                );
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
        sum_pulses.as_mut_ptr() as *const libc::c_int,
    );
}
