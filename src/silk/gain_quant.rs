use crate::silk::lin2log::silk_lin2log;
use crate::silk::log2lin::silk_log2lin;
use crate::silk::SigProc_FIX::{silk_max_int, silk_min_32, silk_min_int};

use crate::silk::define::{
    MAX_DELTA_GAIN_QUANT, MIN_DELTA_GAIN_QUANT, MIN_QGAIN_DB, N_LEVELS_QGAIN,
};

pub const OFFSET: i32 = MIN_QGAIN_DB * 128 as i32 / 6 as i32 + 16 as i32 * 128 as i32;
pub unsafe fn silk_gains_quant(
    ind: *mut i8,
    gain_Q16: *mut i32,
    prev_ind: *mut i8,
    conditional: i32,
    nb_subfr: i32,
) {
    let mut k: i32 = 0;
    let mut double_step_size_threshold: i32 = 0;
    k = 0 as i32;
    while k < nb_subfr {
        *ind.offset(k as isize) = ((65536 as i32 * (64 as i32 - 1 as i32)
            / ((88 as i32 - 2 as i32) * 128 as i32 / 6 as i32))
            as i64
            * (silk_lin2log(*gain_Q16.offset(k as isize))
                - (2 as i32 * 128 as i32 / 6 as i32 + 16 as i32 * 128 as i32)) as i16
                as i64
            >> 16 as i32) as i32 as i8;
        if (*ind.offset(k as isize) as i32) < *prev_ind as i32 {
            let ref mut fresh0 = *ind.offset(k as isize);
            *fresh0 += 1;
        }
        *ind.offset(k as isize) = (if 0 as i32 > 64 as i32 - 1 as i32 {
            if *ind.offset(k as isize) as i32 > 0 as i32 {
                0 as i32
            } else if (*ind.offset(k as isize) as i32) < 64 as i32 - 1 as i32 {
                64 as i32 - 1 as i32
            } else {
                *ind.offset(k as isize) as i32
            }
        } else if *ind.offset(k as isize) as i32 > 64 as i32 - 1 as i32 {
            64 as i32 - 1 as i32
        } else if (*ind.offset(k as isize) as i32) < 0 as i32 {
            0 as i32
        } else {
            *ind.offset(k as isize) as i32
        }) as i8;
        if k == 0 as i32 && conditional == 0 as i32 {
            *ind.offset(k as isize) = (if *prev_ind as i32 + -(4 as i32) > 64 as i32 - 1 as i32 {
                if *ind.offset(k as isize) as i32 > *prev_ind as i32 + -(4 as i32) {
                    *prev_ind as i32 + -(4 as i32)
                } else if (*ind.offset(k as isize) as i32) < 64 as i32 - 1 as i32 {
                    64 as i32 - 1 as i32
                } else {
                    *ind.offset(k as isize) as i32
                }
            } else if *ind.offset(k as isize) as i32 > 64 as i32 - 1 as i32 {
                64 as i32 - 1 as i32
            } else if (*ind.offset(k as isize) as i32) < *prev_ind as i32 + -(4 as i32) {
                *prev_ind as i32 + -(4 as i32)
            } else {
                *ind.offset(k as isize) as i32
            }) as i8;
            *prev_ind = *ind.offset(k as isize);
        } else {
            *ind.offset(k as isize) = (*ind.offset(k as isize) as i32 - *prev_ind as i32) as i8;
            double_step_size_threshold =
                2 as i32 * MAX_DELTA_GAIN_QUANT - N_LEVELS_QGAIN + *prev_ind as i32;
            if *ind.offset(k as isize) as i32 > double_step_size_threshold {
                *ind.offset(k as isize) = (double_step_size_threshold
                    + (*ind.offset(k as isize) as i32 - double_step_size_threshold + 1 as i32
                        >> 1 as i32)) as i8;
            }
            *ind.offset(k as isize) = (if -(4 as i32) > 36 as i32 {
                if *ind.offset(k as isize) as i32 > -(4 as i32) {
                    -(4 as i32)
                } else if (*ind.offset(k as isize) as i32) < 36 as i32 {
                    36 as i32
                } else {
                    *ind.offset(k as isize) as i32
                }
            } else if *ind.offset(k as isize) as i32 > 36 as i32 {
                36 as i32
            } else if (*ind.offset(k as isize) as i32) < -(4 as i32) {
                -(4 as i32)
            } else {
                *ind.offset(k as isize) as i32
            }) as i8;
            if *ind.offset(k as isize) as i32 > double_step_size_threshold {
                *prev_ind = (*prev_ind as i32
                    + (((*ind.offset(k as isize) as u32) << 1 as i32) as i32
                        - double_step_size_threshold)) as i8;
                *prev_ind = silk_min_int(*prev_ind as i32, N_LEVELS_QGAIN - 1 as i32) as i8;
            } else {
                *prev_ind = (*prev_ind as i32 + *ind.offset(k as isize) as i32) as i8;
            }
            let ref mut fresh1 = *ind.offset(k as isize);
            *fresh1 = (*fresh1 as i32 - MIN_DELTA_GAIN_QUANT) as i8;
        }
        *gain_Q16.offset(k as isize) = silk_log2lin(silk_min_32(
            ((65536 as i32 * ((88 as i32 - 2 as i32) * 128 as i32 / 6 as i32)
                / (64 as i32 - 1 as i32)) as i64
                * *prev_ind as i16 as i64
                >> 16 as i32) as i32
                + OFFSET,
            3967 as i32,
        ));
        k += 1;
    }
}
pub unsafe fn silk_gains_dequant(
    gain_Q16: *mut i32,
    ind: *const i8,
    prev_ind: *mut i8,
    conditional: i32,
    nb_subfr: i32,
) {
    let mut k: i32 = 0;
    let mut ind_tmp: i32 = 0;
    let mut double_step_size_threshold: i32 = 0;
    k = 0 as i32;
    while k < nb_subfr {
        if k == 0 as i32 && conditional == 0 as i32 {
            *prev_ind =
                silk_max_int(*ind.offset(k as isize) as i32, *prev_ind as i32 - 16 as i32) as i8;
        } else {
            ind_tmp = *ind.offset(k as isize) as i32 + MIN_DELTA_GAIN_QUANT;
            double_step_size_threshold =
                2 as i32 * MAX_DELTA_GAIN_QUANT - N_LEVELS_QGAIN + *prev_ind as i32;
            if ind_tmp > double_step_size_threshold {
                *prev_ind = (*prev_ind as i32
                    + (((ind_tmp as u32) << 1 as i32) as i32 - double_step_size_threshold))
                    as i8;
            } else {
                *prev_ind = (*prev_ind as i32 + ind_tmp) as i8;
            }
        }
        *prev_ind = (if 0 as i32 > 64 as i32 - 1 as i32 {
            if *prev_ind as i32 > 0 as i32 {
                0 as i32
            } else if (*prev_ind as i32) < 64 as i32 - 1 as i32 {
                64 as i32 - 1 as i32
            } else {
                *prev_ind as i32
            }
        } else if *prev_ind as i32 > 64 as i32 - 1 as i32 {
            64 as i32 - 1 as i32
        } else if (*prev_ind as i32) < 0 as i32 {
            0 as i32
        } else {
            *prev_ind as i32
        }) as i8;
        *gain_Q16.offset(k as isize) = silk_log2lin(silk_min_32(
            ((65536 as i32 * ((88 as i32 - 2 as i32) * 128 as i32 / 6 as i32)
                / (64 as i32 - 1 as i32)) as i64
                * *prev_ind as i16 as i64
                >> 16 as i32) as i32
                + OFFSET,
            3967 as i32,
        ));
        k += 1;
    }
}
pub unsafe fn silk_gains_ID(ind: *const i8, nb_subfr: i32) -> i32 {
    let mut k: i32 = 0;
    let mut gainsID: i32 = 0;
    gainsID = 0 as i32;
    k = 0 as i32;
    while k < nb_subfr {
        gainsID = *ind.offset(k as isize) as i32 + ((gainsID as u32) << 8 as i32) as i32;
        k += 1;
    }
    return gainsID;
}
