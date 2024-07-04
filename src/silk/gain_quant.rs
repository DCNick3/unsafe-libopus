use crate::silk::lin2log::silk_lin2log;
use crate::silk::log2lin::silk_log2lin;
use crate::silk::SigProc_FIX::{silk_LIMIT, silk_max_int, silk_min_32, silk_min_int};
use ndarray::azip;

use crate::silk::define::{
    MAX_DELTA_GAIN_QUANT, MAX_QGAIN_DB, MIN_DELTA_GAIN_QUANT, MIN_QGAIN_DB, N_LEVELS_QGAIN,
};
use crate::silk::macros::silk_SMULWB;

const OFFSET: i32 = MIN_QGAIN_DB * 128 / 6 + 16 * 128;
const SCALE_Q16: i32 =
    (65536 * (N_LEVELS_QGAIN as i32 - 1)) / (((MAX_QGAIN_DB - MIN_QGAIN_DB) * 128) / 6);
const INV_SCALE_Q16: i32 =
    (65536 * (((MAX_QGAIN_DB - MIN_QGAIN_DB) * 128) / 6)) / (N_LEVELS_QGAIN as i32 - 1);
pub unsafe fn silk_gains_quant(
    ind: *mut i8,
    gain_Q16: *mut i32,
    prev_ind: *mut i8,
    conditional: i32,
    nb_subfr: i32,
) {
    let mut k: i32 = 0;
    let mut double_step_size_threshold: i32 = 0;
    k = 0;
    while k < nb_subfr {
        *ind.offset(k as isize) = ((65536 * (64 - 1) / ((88 - 2) * 128 / 6)) as i64
            * (silk_lin2log(*gain_Q16.offset(k as isize)) - (2 * 128 / 6 + 16 * 128)) as i16 as i64
            >> 16) as i32 as i8;
        if (*ind.offset(k as isize) as i32) < *prev_ind as i32 {
            let ref mut fresh0 = *ind.offset(k as isize);
            *fresh0 += 1;
        }
        *ind.offset(k as isize) = (if 0 > 64 - 1 {
            if *ind.offset(k as isize) as i32 > 0 {
                0
            } else if (*ind.offset(k as isize) as i32) < 64 - 1 {
                64 - 1
            } else {
                *ind.offset(k as isize) as i32
            }
        } else if *ind.offset(k as isize) as i32 > 64 - 1 {
            64 - 1
        } else if (*ind.offset(k as isize) as i32) < 0 {
            0
        } else {
            *ind.offset(k as isize) as i32
        }) as i8;
        if k == 0 && conditional == 0 {
            *ind.offset(k as isize) = (if *prev_ind as i32 + -(4) > 64 - 1 {
                if *ind.offset(k as isize) as i32 > *prev_ind as i32 + -(4) {
                    *prev_ind as i32 + -(4)
                } else if (*ind.offset(k as isize) as i32) < 64 - 1 {
                    64 - 1
                } else {
                    *ind.offset(k as isize) as i32
                }
            } else if *ind.offset(k as isize) as i32 > 64 - 1 {
                64 - 1
            } else if (*ind.offset(k as isize) as i32) < *prev_ind as i32 + -(4) {
                *prev_ind as i32 + -(4)
            } else {
                *ind.offset(k as isize) as i32
            }) as i8;
            *prev_ind = *ind.offset(k as isize);
        } else {
            *ind.offset(k as isize) = (*ind.offset(k as isize) as i32 - *prev_ind as i32) as i8;
            double_step_size_threshold =
                2 * MAX_DELTA_GAIN_QUANT as i32 - N_LEVELS_QGAIN as i32 + *prev_ind as i32;
            if *ind.offset(k as isize) as i32 > double_step_size_threshold {
                *ind.offset(k as isize) = (double_step_size_threshold
                    + (*ind.offset(k as isize) as i32 - double_step_size_threshold + 1 >> 1))
                    as i8;
            }
            *ind.offset(k as isize) = (if -(4) > 36 {
                if *ind.offset(k as isize) as i32 > -(4) {
                    -(4)
                } else if (*ind.offset(k as isize) as i32) < 36 {
                    36
                } else {
                    *ind.offset(k as isize) as i32
                }
            } else if *ind.offset(k as isize) as i32 > 36 {
                36
            } else if (*ind.offset(k as isize) as i32) < -(4) {
                -(4)
            } else {
                *ind.offset(k as isize) as i32
            }) as i8;
            if *ind.offset(k as isize) as i32 > double_step_size_threshold {
                *prev_ind = (*prev_ind as i32
                    + (((*ind.offset(k as isize) as u32) << 1) as i32 - double_step_size_threshold))
                    as i8;
                *prev_ind = silk_min_int(*prev_ind as i32, N_LEVELS_QGAIN as i32 - 1) as i8;
            } else {
                *prev_ind = (*prev_ind as i32 + *ind.offset(k as isize) as i32) as i8;
            }
            let ref mut fresh1 = *ind.offset(k as isize);
            *fresh1 = (*fresh1 as i32 - MIN_DELTA_GAIN_QUANT as i32) as i8;
        }
        *gain_Q16.offset(k as isize) = silk_log2lin(silk_min_32(
            ((65536 * ((88 - 2) * 128 / 6) / (64 - 1)) as i64 * *prev_ind as i16 as i64 >> 16)
                as i32
                + OFFSET,
            3967,
        ));
        k += 1;
    }
}

/// Gains scalar dequantization, uniform on log scale
///
/// ```text
/// gain_Q16[ MAX_NB_SUBFR ]   O     quantized gains
/// ind[ MAX_NB_SUBFR ]        I     gain indices
/// prev_ind                   I/O   last index in previous frame
/// conditional                I     first gain is delta coded if 1
/// nb_subfr                   I     number of subframes
/// ```
pub fn silk_gains_dequant(gain_Q16: &mut [i32], ind: &[i8], prev_ind: &mut i8, conditional: bool) {
    azip!((index k, out in gain_Q16, &ind in ind) {
        if k == 0 && !conditional {
            /* Gain index is not allowed to go down more than 16 steps (~21.8 dB) */
            *prev_ind = silk_max_int(ind as i32, *prev_ind as i32 - 16) as i8;
        } else {
            /* Delta index */
            let ind_tmp = ind + MIN_DELTA_GAIN_QUANT;

            /* Accumulate deltas */
            let double_step_size_threshold = 2 * MAX_DELTA_GAIN_QUANT - N_LEVELS_QGAIN + *prev_ind;
            if ind_tmp > double_step_size_threshold {
                *prev_ind += ind_tmp * 2 - double_step_size_threshold;
            } else {
                *prev_ind += ind_tmp;
            }
        }
        *prev_ind = silk_LIMIT(*prev_ind, 0, N_LEVELS_QGAIN - 1);

        /* Scale and convert to linear scale */
        *out = silk_log2lin(silk_min_32(
            silk_SMULWB(INV_SCALE_Q16, *prev_ind as i32) + OFFSET,
            3967, /* 3967 = 31 in Q7 */
        ));
    });
}
pub unsafe fn silk_gains_ID(ind: *const i8, nb_subfr: i32) -> i32 {
    let mut k: i32 = 0;
    let mut gainsID: i32 = 0;
    gainsID = 0;
    k = 0;
    while k < nb_subfr {
        gainsID = *ind.offset(k as isize) as i32 + ((gainsID as u32) << 8) as i32;
        k += 1;
    }
    return gainsID;
}
