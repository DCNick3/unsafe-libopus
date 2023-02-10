use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:9"]
    pub struct stereo_enc_state {
        pub pred_prev_Q13: [i16; 2],
        pub sMid: [i16; 2],
        pub sSide: [i16; 2],
        pub mid_side_amp_Q0: [i32; 4],
        pub smth_width_Q14: i16,
        pub width_prev_Q14: i16,
        pub silent_side_len: i16,
        pub predIx: [[[i8; 3]; 2]; 3],
        pub mid_only_flags: [i8; 3],
    }
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:32"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:32"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int =
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/macros.h:32"]
pub mod macros_h {
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn silk_CLZ32(in32: i32) -> i32 {
        return if in32 != 0 {
            32 as libc::c_int - (EC_CLZ0 - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }
    use super::ecintrin_h::EC_CLZ0;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/Inlines.h:32"]
pub mod Inlines_h {
    #[inline]
    #[c2rust::src_loc = "97:1"]
    pub unsafe extern "C" fn silk_DIV32_varQ(a32: i32, b32: i32, Qres: libc::c_int) -> i32 {
        let mut a_headrm: libc::c_int = 0;
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: i32 = 0;
        let mut a32_nrm: i32 = 0;
        let mut b32_nrm: i32 = 0;
        let mut result: i32 = 0;
        a_headrm = silk_CLZ32(if a32 > 0 as libc::c_int { a32 } else { -a32 }) - 1 as libc::c_int;
        a32_nrm = ((a32 as u32) << a_headrm) as i32;
        b_headrm = silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 }) - 1 as libc::c_int;
        b32_nrm = ((b32 as u32) << b_headrm) as i32;
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int) / (b32_nrm >> 16 as libc::c_int);
        result = (a32_nrm as libc::c_long * b32_inv as i16 as i64 >> 16 as libc::c_int) as i32;
        a32_nrm = (a32_nrm as u32).wrapping_sub(
            (((b32_nrm as i64 * result as libc::c_long >> 32 as libc::c_int) as i32 as u32)
                << 3 as libc::c_int) as i32 as u32,
        ) as i32;
        result = (result as libc::c_long
            + (a32_nrm as libc::c_long * b32_inv as i16 as i64 >> 16 as libc::c_int))
            as i32;
        lshift = 29 as libc::c_int + a_headrm - b_headrm - Qres;
        if lshift < 0 as libc::c_int {
            return (((if 0x80000000 as libc::c_uint as i32 >> -lshift
                > 0x7fffffff as libc::c_int >> -lshift
            {
                if result > 0x80000000 as libc::c_uint as i32 >> -lshift {
                    0x80000000 as libc::c_uint as i32 >> -lshift
                } else {
                    if result < 0x7fffffff as libc::c_int >> -lshift {
                        0x7fffffff as libc::c_int >> -lshift
                    } else {
                        result
                    }
                }
            } else {
                if result > 0x7fffffff as libc::c_int >> -lshift {
                    0x7fffffff as libc::c_int >> -lshift
                } else {
                    if result < 0x80000000 as libc::c_uint as i32 >> -lshift {
                        0x80000000 as libc::c_uint as i32 >> -lshift
                    } else {
                        result
                    }
                }
            }) as u32)
                << -lshift) as i32;
        } else if lshift < 32 as libc::c_int {
            return result >> lshift;
        } else {
            return 0 as libc::c_int;
        };
    }
    use super::macros_h::silk_CLZ32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "82:9"]
    pub const STEREO_INTERP_LEN_MS: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "112:9"]
    pub const LA_SHAPE_MS: libc::c_int = 5 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    extern "C" {
        #[c2rust::src_loc = "85:1"]
        pub fn silk_stereo_quant_pred(pred_Q13: *mut i32, ix: *mut [i8; 3]);
        #[c2rust::src_loc = "75:1"]
        pub fn silk_stereo_find_predictor(
            ratio_Q14: *mut i32,
            x: *const i16,
            y: *const i16,
            mid_res_amp_Q0: *mut i32,
            length: libc::c_int,
            smooth_coef_Q16: libc::c_int,
        ) -> i32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "564:1"]
    pub unsafe extern "C" fn silk_max_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
        return if a > b { a } else { b };
    }
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
pub use self::define_h::{LA_SHAPE_MS, STEREO_INTERP_LEN_MS};
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
pub use self::macros_h::silk_CLZ32;
use self::main_h::{silk_stereo_find_predictor, silk_stereo_quant_pred};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::uint32_t;
use self::string_h::memcpy;
pub use self::structs_h::stereo_enc_state;
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __int8_t, __uint32_t};
pub use self::Inlines_h::silk_DIV32_varQ;
pub use self::SigProc_FIX_h::silk_max_int;
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_stereo_LR_to_MS(
    mut state: *mut stereo_enc_state,
    x1: *mut i16,
    x2: *mut i16,
    ix: *mut [i8; 3],
    mid_only_flag: *mut i8,
    mid_side_rates_bps: *mut i32,
    mut total_rate_bps: i32,
    prev_speech_act_Q8: libc::c_int,
    toMono: libc::c_int,
    fs_kHz: libc::c_int,
    frame_length: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut is10msFrame: libc::c_int = 0;
    let mut denom_Q16: libc::c_int = 0;
    let mut delta0_Q13: libc::c_int = 0;
    let mut delta1_Q13: libc::c_int = 0;
    let mut sum: i32 = 0;
    let mut diff: i32 = 0;
    let mut smooth_coef_Q16: i32 = 0;
    let mut pred_Q13: [i32; 2] = [0; 2];
    let mut pred0_Q13: i32 = 0;
    let mut pred1_Q13: i32 = 0;
    let mut LP_ratio_Q14: i32 = 0;
    let mut HP_ratio_Q14: i32 = 0;
    let mut frac_Q16: i32 = 0;
    let mut frac_3_Q16: i32 = 0;
    let mut min_mid_rate_bps: i32 = 0;
    let mut width_Q14: i32 = 0;
    let mut w_Q24: i32 = 0;
    let mut deltaw_Q24: i32 = 0;
    let mid: *mut i16 = &mut *x1.offset(-(2 as libc::c_int) as isize) as *mut i16;
    let vla = (frame_length + 2 as libc::c_int) as usize;
    let mut side: Vec<i16> = ::std::vec::from_elem(0, vla);
    n = 0 as libc::c_int;
    while n < frame_length + 2 as libc::c_int {
        sum = *x1.offset((n - 2 as libc::c_int) as isize) as libc::c_int
            + *x2.offset((n - 2 as libc::c_int) as isize) as i32;
        diff = *x1.offset((n - 2 as libc::c_int) as isize) as libc::c_int
            - *x2.offset((n - 2 as libc::c_int) as isize) as i32;
        *mid.offset(n as isize) = (if 1 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        *side.as_mut_ptr().offset(n as isize) = (if (if 1 as libc::c_int == 1 as libc::c_int {
            (diff >> 1 as libc::c_int) + (diff & 1 as libc::c_int)
        } else {
            (diff >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 1 as libc::c_int == 1 as libc::c_int {
            (diff >> 1 as libc::c_int) + (diff & 1 as libc::c_int)
        } else {
            (diff >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (diff >> 1 as libc::c_int) + (diff & 1 as libc::c_int)
        } else {
            (diff >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        n += 1;
    }
    memcpy(
        mid as *mut libc::c_void,
        ((*state).sMid).as_mut_ptr() as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    memcpy(
        side.as_mut_ptr() as *mut libc::c_void,
        ((*state).sSide).as_mut_ptr() as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    memcpy(
        ((*state).sMid).as_mut_ptr() as *mut libc::c_void,
        &mut *mid.offset(frame_length as isize) as *mut i16 as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    memcpy(
        ((*state).sSide).as_mut_ptr() as *mut libc::c_void,
        &mut *side.as_mut_ptr().offset(frame_length as isize) as *mut i16 as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    let vla_0 = frame_length as usize;
    let mut LP_mid: Vec<i16> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = frame_length as usize;
    let mut HP_mid: Vec<i16> = ::std::vec::from_elem(0, vla_1);
    n = 0 as libc::c_int;
    while n < frame_length {
        sum = if 2 as libc::c_int == 1 as libc::c_int {
            (*mid.offset(n as isize) as libc::c_int
                + *mid.offset((n + 2 as libc::c_int) as isize) as i32
                + ((*mid.offset((n + 1 as libc::c_int) as isize) as u32) << 1 as libc::c_int)
                    as i32
                >> 1 as libc::c_int)
                + (*mid.offset(n as isize) as libc::c_int
                    + *mid.offset((n + 2 as libc::c_int) as isize) as i32
                    + ((*mid.offset((n + 1 as libc::c_int) as isize) as u32) << 1 as libc::c_int)
                        as i32
                    & 1 as libc::c_int)
        } else {
            (*mid.offset(n as isize) as libc::c_int
                + *mid.offset((n + 2 as libc::c_int) as isize) as i32
                + ((*mid.offset((n + 1 as libc::c_int) as isize) as u32) << 1 as libc::c_int)
                    as i32
                >> 2 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        *LP_mid.as_mut_ptr().offset(n as isize) = sum as i16;
        *HP_mid.as_mut_ptr().offset(n as isize) =
            (*mid.offset((n + 1 as libc::c_int) as isize) as libc::c_int - sum) as i16;
        n += 1;
    }
    let vla_2 = frame_length as usize;
    let mut LP_side: Vec<i16> = ::std::vec::from_elem(0, vla_2);
    let vla_3 = frame_length as usize;
    let mut HP_side: Vec<i16> = ::std::vec::from_elem(0, vla_3);
    n = 0 as libc::c_int;
    while n < frame_length {
        sum = if 2 as libc::c_int == 1 as libc::c_int {
            (*side.as_mut_ptr().offset(n as isize) as libc::c_int
                + *side.as_mut_ptr().offset((n + 2 as libc::c_int) as isize) as i32
                + ((*side.as_mut_ptr().offset((n + 1 as libc::c_int) as isize) as u32)
                    << 1 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + (*side.as_mut_ptr().offset(n as isize) as libc::c_int
                    + *side.as_mut_ptr().offset((n + 2 as libc::c_int) as isize) as i32
                    + ((*side.as_mut_ptr().offset((n + 1 as libc::c_int) as isize) as u32)
                        << 1 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            (*side.as_mut_ptr().offset(n as isize) as libc::c_int
                + *side.as_mut_ptr().offset((n + 2 as libc::c_int) as isize) as i32
                + ((*side.as_mut_ptr().offset((n + 1 as libc::c_int) as isize) as u32)
                    << 1 as libc::c_int) as i32
                >> 2 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        *LP_side.as_mut_ptr().offset(n as isize) = sum as i16;
        *HP_side.as_mut_ptr().offset(n as isize) =
            (*side.as_mut_ptr().offset((n + 1 as libc::c_int) as isize) as libc::c_int - sum)
                as i16;
        n += 1;
    }
    is10msFrame = (frame_length == 10 as libc::c_int * fs_kHz) as libc::c_int;
    smooth_coef_Q16 = if is10msFrame != 0 {
        (0.01f64 / 2 as libc::c_int as libc::c_double
            * ((1 as libc::c_int as i64) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as i32
    } else {
        (0.01f64 * ((1 as libc::c_int as i64) << 16 as libc::c_int) as libc::c_double + 0.5f64)
            as i32
    };
    smooth_coef_Q16 = ((prev_speech_act_Q8 as i16 as i32 * prev_speech_act_Q8 as i16 as i32)
        as libc::c_long
        * smooth_coef_Q16 as i16 as i64
        >> 16 as libc::c_int) as i32;
    pred_Q13[0 as libc::c_int as usize] = silk_stereo_find_predictor(
        &mut LP_ratio_Q14,
        LP_mid.as_mut_ptr() as *const i16,
        LP_side.as_mut_ptr() as *const i16,
        &mut *((*state).mid_side_amp_Q0)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        frame_length,
        smooth_coef_Q16,
    );
    pred_Q13[1 as libc::c_int as usize] = silk_stereo_find_predictor(
        &mut HP_ratio_Q14,
        HP_mid.as_mut_ptr() as *const i16,
        HP_side.as_mut_ptr() as *const i16,
        &mut *((*state).mid_side_amp_Q0)
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
        frame_length,
        smooth_coef_Q16,
    );
    frac_Q16 = HP_ratio_Q14 + LP_ratio_Q14 as i16 as i32 * 3 as libc::c_int as i16 as i32;
    frac_Q16 = if frac_Q16
        < ((1 as libc::c_int as libc::c_long * ((1 as libc::c_int as i64) << 16 as libc::c_int))
            as libc::c_double
            + 0.5f64) as i32
    {
        frac_Q16
    } else {
        ((1 as libc::c_int as libc::c_long * ((1 as libc::c_int as i64) << 16 as libc::c_int))
            as libc::c_double
            + 0.5f64) as i32
    };
    total_rate_bps -= if is10msFrame != 0 {
        1200 as libc::c_int
    } else {
        600 as libc::c_int
    };
    if total_rate_bps < 1 as libc::c_int {
        total_rate_bps = 1 as libc::c_int;
    }
    min_mid_rate_bps =
        2000 as libc::c_int + fs_kHz as i16 as i32 * 600 as libc::c_int as i16 as i32;
    frac_3_Q16 = 3 as libc::c_int * frac_Q16;
    *mid_side_rates_bps.offset(0 as libc::c_int as isize) = silk_DIV32_varQ(
        total_rate_bps,
        (((8 as libc::c_int + 5 as libc::c_int) as libc::c_long
            * ((1 as libc::c_int as i64) << 16 as libc::c_int)) as libc::c_double
            + 0.5f64) as i32
            + frac_3_Q16,
        16 as libc::c_int + 3 as libc::c_int,
    );
    if *mid_side_rates_bps.offset(0 as libc::c_int as isize) < min_mid_rate_bps {
        *mid_side_rates_bps.offset(0 as libc::c_int as isize) = min_mid_rate_bps;
        *mid_side_rates_bps.offset(1 as libc::c_int as isize) =
            total_rate_bps - *mid_side_rates_bps.offset(0 as libc::c_int as isize);
        width_Q14 = silk_DIV32_varQ(
            ((*mid_side_rates_bps.offset(1 as libc::c_int as isize) as u32) << 1 as libc::c_int)
                as i32
                - min_mid_rate_bps,
            ((((1 as libc::c_int as libc::c_long * ((1 as libc::c_int as i64) << 16 as libc::c_int))
                as libc::c_double
                + 0.5f64) as i32
                + frac_3_Q16) as libc::c_long
                * min_mid_rate_bps as i16 as i64
                >> 16 as libc::c_int) as i32,
            14 as libc::c_int + 2 as libc::c_int,
        );
        width_Q14 = if 0 as libc::c_int
            > ((1 as libc::c_int as libc::c_long * ((1 as libc::c_int as i64) << 14 as libc::c_int))
                as libc::c_double
                + 0.5f64) as i32
        {
            if width_Q14 > 0 as libc::c_int {
                0 as libc::c_int
            } else if width_Q14
                < ((1 as libc::c_int as libc::c_long
                    * ((1 as libc::c_int as i64) << 14 as libc::c_int))
                    as libc::c_double
                    + 0.5f64) as i32
            {
                ((1 as libc::c_int as libc::c_long
                    * ((1 as libc::c_int as i64) << 14 as libc::c_int))
                    as libc::c_double
                    + 0.5f64) as i32
            } else {
                width_Q14
            }
        } else if width_Q14
            > ((1 as libc::c_int as libc::c_long * ((1 as libc::c_int as i64) << 14 as libc::c_int))
                as libc::c_double
                + 0.5f64) as i32
        {
            ((1 as libc::c_int as libc::c_long * ((1 as libc::c_int as i64) << 14 as libc::c_int))
                as libc::c_double
                + 0.5f64) as i32
        } else if width_Q14 < 0 as libc::c_int {
            0 as libc::c_int
        } else {
            width_Q14
        };
    } else {
        *mid_side_rates_bps.offset(1 as libc::c_int as isize) =
            total_rate_bps - *mid_side_rates_bps.offset(0 as libc::c_int as isize);
        width_Q14 = ((1 as libc::c_int as libc::c_long
            * ((1 as libc::c_int as i64) << 14 as libc::c_int))
            as libc::c_double
            + 0.5f64) as i32;
    }
    (*state).smth_width_Q14 = ((*state).smth_width_Q14 as libc::c_long
        + ((width_Q14 - (*state).smth_width_Q14 as libc::c_int) as libc::c_long
            * smooth_coef_Q16 as i16 as i64
            >> 16 as libc::c_int)) as i32 as i16;
    *mid_only_flag = 0 as libc::c_int as i8;
    if toMono != 0 {
        width_Q14 = 0 as libc::c_int;
        pred_Q13[0 as libc::c_int as usize] = 0 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = 0 as libc::c_int;
        silk_stereo_quant_pred(pred_Q13.as_mut_ptr(), ix);
    } else if (*state).width_prev_Q14 as libc::c_int == 0 as libc::c_int
        && (8 as libc::c_int * total_rate_bps < 13 as libc::c_int * min_mid_rate_bps
            || ((frac_Q16 as libc::c_long * (*state).smth_width_Q14 as i64 >> 16 as libc::c_int)
                as i32)
                < (0.05f64 * ((1 as libc::c_int as i64) << 14 as libc::c_int) as libc::c_double
                    + 0.5f64) as i32)
    {
        pred_Q13[0 as libc::c_int as usize] = (*state).smth_width_Q14 as i32
            * pred_Q13[0 as libc::c_int as usize] as i16 as i32
            >> 14 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = (*state).smth_width_Q14 as i32
            * pred_Q13[1 as libc::c_int as usize] as i16 as i32
            >> 14 as libc::c_int;
        silk_stereo_quant_pred(pred_Q13.as_mut_ptr(), ix);
        width_Q14 = 0 as libc::c_int;
        pred_Q13[0 as libc::c_int as usize] = 0 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = 0 as libc::c_int;
        *mid_side_rates_bps.offset(0 as libc::c_int as isize) = total_rate_bps;
        *mid_side_rates_bps.offset(1 as libc::c_int as isize) = 0 as libc::c_int;
        *mid_only_flag = 1 as libc::c_int as i8;
    } else if (*state).width_prev_Q14 as libc::c_int != 0 as libc::c_int
        && (8 as libc::c_int * total_rate_bps < 11 as libc::c_int * min_mid_rate_bps
            || ((frac_Q16 as libc::c_long * (*state).smth_width_Q14 as i64 >> 16 as libc::c_int)
                as i32)
                < (0.02f64 * ((1 as libc::c_int as i64) << 14 as libc::c_int) as libc::c_double
                    + 0.5f64) as i32)
    {
        pred_Q13[0 as libc::c_int as usize] = (*state).smth_width_Q14 as i32
            * pred_Q13[0 as libc::c_int as usize] as i16 as i32
            >> 14 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = (*state).smth_width_Q14 as i32
            * pred_Q13[1 as libc::c_int as usize] as i16 as i32
            >> 14 as libc::c_int;
        silk_stereo_quant_pred(pred_Q13.as_mut_ptr(), ix);
        width_Q14 = 0 as libc::c_int;
        pred_Q13[0 as libc::c_int as usize] = 0 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = 0 as libc::c_int;
    } else if (*state).smth_width_Q14 as libc::c_int
        > (0.95f64 * ((1 as libc::c_int as i64) << 14 as libc::c_int) as libc::c_double + 0.5f64)
            as i32
    {
        silk_stereo_quant_pred(pred_Q13.as_mut_ptr(), ix);
        width_Q14 = ((1 as libc::c_int as libc::c_long
            * ((1 as libc::c_int as i64) << 14 as libc::c_int))
            as libc::c_double
            + 0.5f64) as i32;
    } else {
        pred_Q13[0 as libc::c_int as usize] = (*state).smth_width_Q14 as i32
            * pred_Q13[0 as libc::c_int as usize] as i16 as i32
            >> 14 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = (*state).smth_width_Q14 as i32
            * pred_Q13[1 as libc::c_int as usize] as i16 as i32
            >> 14 as libc::c_int;
        silk_stereo_quant_pred(pred_Q13.as_mut_ptr(), ix);
        width_Q14 = (*state).smth_width_Q14 as i32;
    }
    if *mid_only_flag as libc::c_int == 1 as libc::c_int {
        (*state).silent_side_len = ((*state).silent_side_len as libc::c_int
            + (frame_length - STEREO_INTERP_LEN_MS * fs_kHz))
            as i16;
        if ((*state).silent_side_len as libc::c_int) < LA_SHAPE_MS * fs_kHz {
            *mid_only_flag = 0 as libc::c_int as i8;
        } else {
            (*state).silent_side_len = 10000 as libc::c_int as i16;
        }
    } else {
        (*state).silent_side_len = 0 as libc::c_int as i16;
    }
    if *mid_only_flag as libc::c_int == 0 as libc::c_int
        && *mid_side_rates_bps.offset(1 as libc::c_int as isize) < 1 as libc::c_int
    {
        *mid_side_rates_bps.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
        *mid_side_rates_bps.offset(0 as libc::c_int as isize) = silk_max_int(
            1 as libc::c_int,
            total_rate_bps - *mid_side_rates_bps.offset(1 as libc::c_int as isize),
        );
    }
    pred0_Q13 = -((*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int);
    pred1_Q13 = -((*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int);
    w_Q24 = (((*state).width_prev_Q14 as u32) << 10 as libc::c_int) as i32;
    denom_Q16 = ((1 as libc::c_int) << 16 as libc::c_int) / (8 as libc::c_int * fs_kHz);
    delta0_Q13 = -if 16 as libc::c_int == 1 as libc::c_int {
        ((pred_Q13[0 as libc::c_int as usize]
            - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int) as i16
            as i32
            * denom_Q16 as i16 as i32
            >> 1 as libc::c_int)
            + ((pred_Q13[0 as libc::c_int as usize]
                - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int)
                as i16 as i32
                * denom_Q16 as i16 as i32
                & 1 as libc::c_int)
    } else {
        ((pred_Q13[0 as libc::c_int as usize]
            - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int) as i16
            as i32
            * denom_Q16 as i16 as i32
            >> 16 as libc::c_int - 1 as libc::c_int)
            + 1 as libc::c_int
            >> 1 as libc::c_int
    };
    delta1_Q13 = -if 16 as libc::c_int == 1 as libc::c_int {
        ((pred_Q13[1 as libc::c_int as usize]
            - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int) as i16
            as i32
            * denom_Q16 as i16 as i32
            >> 1 as libc::c_int)
            + ((pred_Q13[1 as libc::c_int as usize]
                - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int)
                as i16 as i32
                * denom_Q16 as i16 as i32
                & 1 as libc::c_int)
    } else {
        ((pred_Q13[1 as libc::c_int as usize]
            - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int) as i16
            as i32
            * denom_Q16 as i16 as i32
            >> 16 as libc::c_int - 1 as libc::c_int)
            + 1 as libc::c_int
            >> 1 as libc::c_int
    };
    deltaw_Q24 = ((((width_Q14 - (*state).width_prev_Q14 as libc::c_int) as libc::c_long
        * denom_Q16 as i16 as i64
        >> 16 as libc::c_int) as i32 as u32)
        << 10 as libc::c_int) as i32;
    n = 0 as libc::c_int;
    while n < STEREO_INTERP_LEN_MS * fs_kHz {
        pred0_Q13 += delta0_Q13;
        pred1_Q13 += delta1_Q13;
        w_Q24 += deltaw_Q24;
        sum = (((*mid.offset(n as isize) as libc::c_int
            + *mid.offset((n + 2 as libc::c_int) as isize) as i32
            + ((*mid.offset((n + 1 as libc::c_int) as isize) as u32) << 1 as libc::c_int) as i32)
            as u32)
            << 9 as libc::c_int) as i32;
        sum = ((w_Q24 as libc::c_long
            * *side.as_mut_ptr().offset((n + 1 as libc::c_int) as isize) as i64
            >> 16 as libc::c_int) as i32 as libc::c_long
            + (sum as libc::c_long * pred0_Q13 as i16 as i64 >> 16 as libc::c_int))
            as i32;
        sum = (sum as libc::c_long
            + (((*mid.offset((n + 1 as libc::c_int) as isize) as i32 as u32) << 11 as libc::c_int)
                as i32 as libc::c_long
                * pred1_Q13 as i16 as i64
                >> 16 as libc::c_int)) as i32;
        *x2.offset((n - 1 as libc::c_int) as isize) = (if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        n += 1;
    }
    pred0_Q13 = -pred_Q13[0 as libc::c_int as usize];
    pred1_Q13 = -pred_Q13[1 as libc::c_int as usize];
    w_Q24 = ((width_Q14 as u32) << 10 as libc::c_int) as i32;
    n = STEREO_INTERP_LEN_MS * fs_kHz;
    while n < frame_length {
        sum = (((*mid.offset(n as isize) as libc::c_int
            + *mid.offset((n + 2 as libc::c_int) as isize) as i32
            + ((*mid.offset((n + 1 as libc::c_int) as isize) as u32) << 1 as libc::c_int) as i32)
            as u32)
            << 9 as libc::c_int) as i32;
        sum = ((w_Q24 as libc::c_long
            * *side.as_mut_ptr().offset((n + 1 as libc::c_int) as isize) as i64
            >> 16 as libc::c_int) as i32 as libc::c_long
            + (sum as libc::c_long * pred0_Q13 as i16 as i64 >> 16 as libc::c_int))
            as i32;
        sum = (sum as libc::c_long
            + (((*mid.offset((n + 1 as libc::c_int) as isize) as i32 as u32) << 11 as libc::c_int)
                as i32 as libc::c_long
                * pred1_Q13 as i16 as i64
                >> 16 as libc::c_int)) as i32;
        *x2.offset((n - 1 as libc::c_int) as isize) = (if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        n += 1;
    }
    (*state).pred_prev_Q13[0 as libc::c_int as usize] = pred_Q13[0 as libc::c_int as usize] as i16;
    (*state).pred_prev_Q13[1 as libc::c_int as usize] = pred_Q13[1 as libc::c_int as usize] as i16;
    (*state).width_prev_Q14 = width_Q14 as i16;
}
