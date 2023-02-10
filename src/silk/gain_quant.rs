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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "546:1"]
    pub unsafe extern "C" fn silk_min_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
        return if a < b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "554:1"]
    pub unsafe extern "C" fn silk_min_32(a: i32, b: i32) -> i32 {
        return if a < b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "564:1"]
    pub unsafe extern "C" fn silk_max_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
        return if a > b { a } else { b };
    }
    extern "C" {
        #[c2rust::src_loc = "176:1"]
        pub fn silk_lin2log(inLin: i32) -> i32;
        #[c2rust::src_loc = "187:1"]
        pub fn silk_log2lin(inLog_Q7: i32) -> i32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "119:9"]
    pub const MIN_QGAIN_DB: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "127:9"]
    pub const MIN_DELTA_GAIN_QUANT: libc::c_int = -(4 as libc::c_int);
    #[c2rust::src_loc = "123:9"]
    pub const N_LEVELS_QGAIN: libc::c_int = 64 as libc::c_int;
    #[c2rust::src_loc = "125:9"]
    pub const MAX_DELTA_GAIN_QUANT: libc::c_int = 36 as libc::c_int;
}
pub use self::define_h::{
    MAX_DELTA_GAIN_QUANT, MIN_DELTA_GAIN_QUANT, MIN_QGAIN_DB, N_LEVELS_QGAIN,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __int8_t, __uint32_t};
pub use self::SigProc_FIX_h::{
    silk_lin2log, silk_log2lin, silk_max_int, silk_min_32, silk_min_int,
};
#[c2rust::src_loc = "34:9"]
pub const OFFSET: libc::c_int =
    MIN_QGAIN_DB * 128 as libc::c_int / 6 as libc::c_int + 16 as libc::c_int * 128 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_gains_quant(
    ind: *mut i8,
    gain_Q16: *mut i32,
    prev_ind: *mut i8,
    conditional: libc::c_int,
    nb_subfr: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut double_step_size_threshold: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < nb_subfr {
        *ind.offset(k as isize) = ((65536 as libc::c_int * (64 as libc::c_int - 1 as libc::c_int)
            / ((88 as libc::c_int - 2 as libc::c_int) * 128 as libc::c_int / 6 as libc::c_int))
            as libc::c_long
            * (silk_lin2log(*gain_Q16.offset(k as isize))
                - (2 as libc::c_int * 128 as libc::c_int / 6 as libc::c_int
                    + 16 as libc::c_int * 128 as libc::c_int)) as i16 as i64
            >> 16 as libc::c_int) as i32 as i8;
        if (*ind.offset(k as isize) as libc::c_int) < *prev_ind as libc::c_int {
            let ref mut fresh0 = *ind.offset(k as isize);
            *fresh0 += 1;
        }
        *ind.offset(k as isize) = (if 0 as libc::c_int > 64 as libc::c_int - 1 as libc::c_int {
            if *ind.offset(k as isize) as libc::c_int > 0 as libc::c_int {
                0 as libc::c_int
            } else if (*ind.offset(k as isize) as libc::c_int)
                < 64 as libc::c_int - 1 as libc::c_int
            {
                64 as libc::c_int - 1 as libc::c_int
            } else {
                *ind.offset(k as isize) as libc::c_int
            }
        } else if *ind.offset(k as isize) as libc::c_int > 64 as libc::c_int - 1 as libc::c_int {
            64 as libc::c_int - 1 as libc::c_int
        } else if (*ind.offset(k as isize) as libc::c_int) < 0 as libc::c_int {
            0 as libc::c_int
        } else {
            *ind.offset(k as isize) as libc::c_int
        }) as i8;
        if k == 0 as libc::c_int && conditional == 0 as libc::c_int {
            *ind.offset(k as isize) = (if *prev_ind as libc::c_int + -(4 as libc::c_int)
                > 64 as libc::c_int - 1 as libc::c_int
            {
                if *ind.offset(k as isize) as libc::c_int
                    > *prev_ind as libc::c_int + -(4 as libc::c_int)
                {
                    *prev_ind as libc::c_int + -(4 as libc::c_int)
                } else if (*ind.offset(k as isize) as libc::c_int)
                    < 64 as libc::c_int - 1 as libc::c_int
                {
                    64 as libc::c_int - 1 as libc::c_int
                } else {
                    *ind.offset(k as isize) as libc::c_int
                }
            } else if *ind.offset(k as isize) as libc::c_int > 64 as libc::c_int - 1 as libc::c_int
            {
                64 as libc::c_int - 1 as libc::c_int
            } else if (*ind.offset(k as isize) as libc::c_int)
                < *prev_ind as libc::c_int + -(4 as libc::c_int)
            {
                *prev_ind as libc::c_int + -(4 as libc::c_int)
            } else {
                *ind.offset(k as isize) as libc::c_int
            }) as i8;
            *prev_ind = *ind.offset(k as isize);
        } else {
            *ind.offset(k as isize) =
                (*ind.offset(k as isize) as libc::c_int - *prev_ind as libc::c_int) as i8;
            double_step_size_threshold =
                2 as libc::c_int * MAX_DELTA_GAIN_QUANT - N_LEVELS_QGAIN + *prev_ind as libc::c_int;
            if *ind.offset(k as isize) as libc::c_int > double_step_size_threshold {
                *ind.offset(k as isize) = (double_step_size_threshold
                    + (*ind.offset(k as isize) as libc::c_int - double_step_size_threshold
                        + 1 as libc::c_int
                        >> 1 as libc::c_int)) as i8;
            }
            *ind.offset(k as isize) = (if -(4 as libc::c_int) > 36 as libc::c_int {
                if *ind.offset(k as isize) as libc::c_int > -(4 as libc::c_int) {
                    -(4 as libc::c_int)
                } else if (*ind.offset(k as isize) as libc::c_int) < 36 as libc::c_int {
                    36 as libc::c_int
                } else {
                    *ind.offset(k as isize) as libc::c_int
                }
            } else if *ind.offset(k as isize) as libc::c_int > 36 as libc::c_int {
                36 as libc::c_int
            } else if (*ind.offset(k as isize) as libc::c_int) < -(4 as libc::c_int) {
                -(4 as libc::c_int)
            } else {
                *ind.offset(k as isize) as libc::c_int
            }) as i8;
            if *ind.offset(k as isize) as libc::c_int > double_step_size_threshold {
                *prev_ind = (*prev_ind as libc::c_int
                    + (((*ind.offset(k as isize) as u32) << 1 as libc::c_int) as i32
                        - double_step_size_threshold)) as i8;
                *prev_ind =
                    silk_min_int(*prev_ind as libc::c_int, N_LEVELS_QGAIN - 1 as libc::c_int) as i8;
            } else {
                *prev_ind =
                    (*prev_ind as libc::c_int + *ind.offset(k as isize) as libc::c_int) as i8;
            }
            let ref mut fresh1 = *ind.offset(k as isize);
            *fresh1 = (*fresh1 as libc::c_int - MIN_DELTA_GAIN_QUANT) as i8;
        }
        *gain_Q16.offset(k as isize) = silk_log2lin(silk_min_32(
            ((65536 as libc::c_int
                * ((88 as libc::c_int - 2 as libc::c_int) * 128 as libc::c_int / 6 as libc::c_int)
                / (64 as libc::c_int - 1 as libc::c_int)) as libc::c_long
                * *prev_ind as i16 as i64
                >> 16 as libc::c_int) as i32
                + OFFSET,
            3967 as libc::c_int,
        ));
        k += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "94:1"]
pub unsafe extern "C" fn silk_gains_dequant(
    gain_Q16: *mut i32,
    ind: *const i8,
    prev_ind: *mut i8,
    conditional: libc::c_int,
    nb_subfr: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut ind_tmp: libc::c_int = 0;
    let mut double_step_size_threshold: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < nb_subfr {
        if k == 0 as libc::c_int && conditional == 0 as libc::c_int {
            *prev_ind = silk_max_int(
                *ind.offset(k as isize) as libc::c_int,
                *prev_ind as libc::c_int - 16 as libc::c_int,
            ) as i8;
        } else {
            ind_tmp = *ind.offset(k as isize) as libc::c_int + MIN_DELTA_GAIN_QUANT;
            double_step_size_threshold =
                2 as libc::c_int * MAX_DELTA_GAIN_QUANT - N_LEVELS_QGAIN + *prev_ind as libc::c_int;
            if ind_tmp > double_step_size_threshold {
                *prev_ind = (*prev_ind as libc::c_int
                    + (((ind_tmp as u32) << 1 as libc::c_int) as i32 - double_step_size_threshold))
                    as i8;
            } else {
                *prev_ind = (*prev_ind as libc::c_int + ind_tmp) as i8;
            }
        }
        *prev_ind = (if 0 as libc::c_int > 64 as libc::c_int - 1 as libc::c_int {
            if *prev_ind as libc::c_int > 0 as libc::c_int {
                0 as libc::c_int
            } else if (*prev_ind as libc::c_int) < 64 as libc::c_int - 1 as libc::c_int {
                64 as libc::c_int - 1 as libc::c_int
            } else {
                *prev_ind as libc::c_int
            }
        } else if *prev_ind as libc::c_int > 64 as libc::c_int - 1 as libc::c_int {
            64 as libc::c_int - 1 as libc::c_int
        } else if (*prev_ind as libc::c_int) < 0 as libc::c_int {
            0 as libc::c_int
        } else {
            *prev_ind as libc::c_int
        }) as i8;
        *gain_Q16.offset(k as isize) = silk_log2lin(silk_min_32(
            ((65536 as libc::c_int
                * ((88 as libc::c_int - 2 as libc::c_int) * 128 as libc::c_int / 6 as libc::c_int)
                / (64 as libc::c_int - 1 as libc::c_int)) as libc::c_long
                * *prev_ind as i16 as i64
                >> 16 as libc::c_int) as i32
                + OFFSET,
            3967 as libc::c_int,
        ));
        k += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "128:1"]
pub unsafe extern "C" fn silk_gains_ID(ind: *const i8, nb_subfr: libc::c_int) -> i32 {
    let mut k: libc::c_int = 0;
    let mut gainsID: i32 = 0;
    gainsID = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < nb_subfr {
        gainsID =
            *ind.offset(k as isize) as libc::c_int + ((gainsID as u32) << 8 as libc::c_int) as i32;
        k += 1;
    }
    return gainsID;
}
