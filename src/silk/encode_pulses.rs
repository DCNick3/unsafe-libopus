use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int32_t, __int8_t};
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
    #[c2rust::src_loc = "47:1"]
    pub type ec_enc = ec_ctx;
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.h:32"]
pub mod entenc_h {
    use super::entcode_h::ec_enc;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn ec_enc_icdf(
            _this: *mut ec_enc,
            _s: libc::c_int,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "50:26"]
        pub static silk_pulses_per_block_iCDF: [[u8; 18]; 10];
        #[c2rust::src_loc = "51:26"]
        pub static silk_pulses_per_block_BITS_Q5: [[u8; 18]; 9];
        #[c2rust::src_loc = "53:26"]
        pub static silk_rate_levels_iCDF: [[u8; 9]; 2];
        #[c2rust::src_loc = "54:26"]
        pub static silk_rate_levels_BITS_Q5: [[u8; 9]; 2];
        #[c2rust::src_loc = "56:26"]
        pub static silk_max_pulses_table: [u8; 4];
        #[c2rust::src_loc = "64:26"]
        pub static silk_lsb_iCDF: [u8; 2];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use super::entcode_h::ec_enc;
    extern "C" {
        #[c2rust::src_loc = "115:1"]
        pub fn silk_encode_signs(
            psRangeEnc: *mut ec_enc,
            pulses: *const i8,
            length: libc::c_int,
            signalType: libc::c_int,
            quantOffsetType: libc::c_int,
            sum_pulses: *const libc::c_int,
        );
        #[c2rust::src_loc = "165:1"]
        pub fn silk_shell_encoder(psRangeEnc: *mut ec_enc, pulses0: *const libc::c_int);
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
}
use self::arch_h::celt_fatal;
pub use self::define_h::{N_RATE_LEVELS, SHELL_CODEC_FRAME_LENGTH, SILK_MAX_PULSES};
pub use self::entcode_h::{ec_ctx, ec_enc, ec_window};
use self::entenc_h::ec_enc_icdf;
use self::main_h::{silk_encode_signs, silk_shell_encoder};
pub use self::stdint_intn_h::{int32_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::string_h::memset;
use self::tables_h::{
    silk_lsb_iCDF, silk_max_pulses_table, silk_pulses_per_block_BITS_Q5,
    silk_pulses_per_block_iCDF, silk_rate_levels_BITS_Q5, silk_rate_levels_iCDF,
};
pub use self::typedef_h::silk_int32_MAX;
pub use self::types_h::{__int32_t, __int8_t, __uint32_t, __uint8_t};
#[inline]
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn combine_and_check(
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
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub unsafe extern "C" fn silk_encode_pulses(
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
