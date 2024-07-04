use crate::silk::lin2log::silk_lin2log;
use crate::silk::log2lin::silk_log2lin;
use crate::silk::SigProc_FIX::silk_LIMIT;
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

/// Gain scalar quantization with hysteresis, uniform on log scale
///
/// ```text
/// ind[ MAX_NB_SUBFR ]        O     gain indices
/// gain_Q16[ MAX_NB_SUBFR ]   I/O   gains (quantized out)
/// prev_ind                   I/O   last index in previous frame
/// conditional                I     first gain is delta coded if 1
/// nb_subfr                   I     number of subframes
/// ```
pub fn silk_gains_quant(
    ind: &mut [i8],
    gain_Q16: &mut [i32],
    prev_ind: &mut i8,
    conditional: bool,
) {
    azip!((index k, out in ind, gain in gain_Q16) {
        /* Convert to log scale, scale, floor() */
        let mut ind  = silk_SMULWB(SCALE_Q16, silk_lin2log(*gain) - OFFSET) as i8;

        /* Round towards previous quantized gain (hysteresis) */
        if ind < *prev_ind {
            ind += 1;
        }
        ind = silk_LIMIT(ind, 0, N_LEVELS_QGAIN - 1);

        /* Compute delta indices and limit */
        if k == 0 && !conditional {
            /* Full index */
            ind = silk_LIMIT(ind, *prev_ind + MIN_DELTA_GAIN_QUANT, N_LEVELS_QGAIN - 1);
            *prev_ind = ind;
        } else {
            /* Delta index */
            ind -= *prev_ind;

            /* Double the quantization step size for large gain increases, so that the max gain level can be reached */
            let double_step_size_threshold = 2 * MAX_DELTA_GAIN_QUANT - N_LEVELS_QGAIN + *prev_ind;
            if ind > double_step_size_threshold {
                ind = double_step_size_threshold + (ind - double_step_size_threshold + 1) / 2;
            }

            ind = silk_LIMIT(ind, MIN_DELTA_GAIN_QUANT, MAX_DELTA_GAIN_QUANT);

            /* Accumulate deltas */
            if ind > double_step_size_threshold {
                *prev_ind += ind * 2 - double_step_size_threshold;
                *prev_ind = std::cmp::min(*prev_ind, N_LEVELS_QGAIN - 1);
            } else {
                *prev_ind += ind;
            }

            /* Shift to make non-negative */
            ind -= MIN_DELTA_GAIN_QUANT;
        }

        *out = ind;

        /* Scale and convert to linear scale */
        *gain = silk_log2lin(std::cmp::min(
            silk_SMULWB(INV_SCALE_Q16, *prev_ind as i32) + OFFSET,
            3967, /* 3967 = 31 in Q7 */
        ));
    });
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
            *prev_ind = std::cmp::max(ind, *prev_ind - 16);
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
        *out = silk_log2lin(std::cmp::min(
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
