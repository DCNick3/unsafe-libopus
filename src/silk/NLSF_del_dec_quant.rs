pub mod typedef_h {
    pub const silk_int32_MAX: i32 = 0x7fffffff as i32;
}
use crate::silk::define::{
    NLSF_QUANT_DEL_DEC_STATES, NLSF_QUANT_MAX_AMPLITUDE, NLSF_QUANT_MAX_AMPLITUDE_EXT,
};

pub use self::typedef_h::silk_int32_MAX;

use crate::externs::memcpy;
pub unsafe fn silk_NLSF_del_dec_quant(
    indices: *mut i8,
    x_Q10: *const i16,
    w_Q5: *const i16,
    pred_coef_Q8: *const u8,
    ec_ix: *const i16,
    ec_rates_Q5: *const u8,
    quant_step_size_Q16: i32,
    inv_quant_step_size_Q6: i16,
    mu_Q20: i32,
    order: i16,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut nStates: i32 = 0;
    let mut ind_tmp: i32 = 0;
    let mut ind_min_max: i32 = 0;
    let mut ind_max_min: i32 = 0;
    let mut in_Q10: i32 = 0;
    let mut res_Q10: i32 = 0;
    let mut pred_Q10: i32 = 0;
    let mut diff_Q10: i32 = 0;
    let mut rate0_Q5: i32 = 0;
    let mut rate1_Q5: i32 = 0;
    let mut out0_Q10: i16 = 0;
    let mut out1_Q10: i16 = 0;
    let mut RD_tmp_Q25: i32 = 0;
    let mut min_Q25: i32 = 0;
    let mut min_max_Q25: i32 = 0;
    let mut max_min_Q25: i32 = 0;
    let mut ind_sort: [i32; 4] = [0; 4];
    let mut ind: [[i8; 16]; 4] = [[0; 16]; 4];
    let mut prev_out_Q10: [i16; 8] = [0; 8];
    let mut RD_Q25: [i32; 8] = [0; 8];
    let mut RD_min_Q25: [i32; 4] = [0; 4];
    let mut RD_max_Q25: [i32; 4] = [0; 4];
    let mut rates_Q5: *const u8 = 0 as *const u8;
    let mut out0_Q10_table: [i32; 20] = [0; 20];
    let mut out1_Q10_table: [i32; 20] = [0; 20];
    i = -NLSF_QUANT_MAX_AMPLITUDE_EXT;
    while i <= NLSF_QUANT_MAX_AMPLITUDE_EXT - 1 as i32 {
        out0_Q10 = ((i as u32) << 10 as i32) as i32 as i16;
        out1_Q10 = (out0_Q10 as i32 + 1024 as i32) as i16;
        if i > 0 as i32 {
            out0_Q10 = (out0_Q10 as i32
                - (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64) as i32)
                as i16;
            out1_Q10 = (out1_Q10 as i32
                - (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64) as i32)
                as i16;
        } else if i == 0 as i32 {
            out1_Q10 = (out1_Q10 as i32
                - (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64) as i32)
                as i16;
        } else if i == -(1 as i32) {
            out0_Q10 = (out0_Q10 as i32
                + (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64) as i32)
                as i16;
        } else {
            out0_Q10 = (out0_Q10 as i32
                + (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64) as i32)
                as i16;
            out1_Q10 = (out1_Q10 as i32
                + (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64) as i32)
                as i16;
        }
        out0_Q10_table[(i + NLSF_QUANT_MAX_AMPLITUDE_EXT) as usize] =
            out0_Q10 as i32 * quant_step_size_Q16 as i16 as i32 >> 16 as i32;
        out1_Q10_table[(i + NLSF_QUANT_MAX_AMPLITUDE_EXT) as usize] =
            out1_Q10 as i32 * quant_step_size_Q16 as i16 as i32 >> 16 as i32;
        i += 1;
    }
    nStates = 1 as i32;
    RD_Q25[0 as i32 as usize] = 0 as i32;
    prev_out_Q10[0 as i32 as usize] = 0 as i32 as i16;
    i = order as i32 - 1 as i32;
    while i >= 0 as i32 {
        rates_Q5 = &*ec_rates_Q5.offset(*ec_ix.offset(i as isize) as isize) as *const u8;
        in_Q10 = *x_Q10.offset(i as isize) as i32;
        j = 0 as i32;
        while j < nStates {
            pred_Q10 = *pred_coef_Q8.offset(i as isize) as i16 as i32
                * prev_out_Q10[j as usize] as i32
                >> 8 as i32;
            res_Q10 = in_Q10 - pred_Q10;
            ind_tmp = inv_quant_step_size_Q6 as i32 * res_Q10 as i16 as i32 >> 16 as i32;
            ind_tmp = if -(10 as i32) > 10 as i32 - 1 as i32 {
                if ind_tmp > -(10 as i32) {
                    -(10 as i32)
                } else if ind_tmp < 10 as i32 - 1 as i32 {
                    10 as i32 - 1 as i32
                } else {
                    ind_tmp
                }
            } else if ind_tmp > 10 as i32 - 1 as i32 {
                10 as i32 - 1 as i32
            } else if ind_tmp < -(10 as i32) {
                -(10 as i32)
            } else {
                ind_tmp
            };
            ind[j as usize][i as usize] = ind_tmp as i8;
            out0_Q10 = out0_Q10_table[(ind_tmp + NLSF_QUANT_MAX_AMPLITUDE_EXT) as usize] as i16;
            out1_Q10 = out1_Q10_table[(ind_tmp + NLSF_QUANT_MAX_AMPLITUDE_EXT) as usize] as i16;
            out0_Q10 = (out0_Q10 as i32 + pred_Q10) as i16;
            out1_Q10 = (out1_Q10 as i32 + pred_Q10) as i16;
            prev_out_Q10[j as usize] = out0_Q10;
            prev_out_Q10[(j + nStates) as usize] = out1_Q10;
            if ind_tmp + 1 as i32 >= NLSF_QUANT_MAX_AMPLITUDE {
                if ind_tmp + 1 as i32 == NLSF_QUANT_MAX_AMPLITUDE {
                    rate0_Q5 =
                        *rates_Q5.offset((ind_tmp + NLSF_QUANT_MAX_AMPLITUDE) as isize) as i32;
                    rate1_Q5 = 280 as i32;
                } else {
                    rate0_Q5 = 280 as i32 - 43 as i32 * 4 as i32
                        + 43 as i32 as i16 as i32 * ind_tmp as i16 as i32;
                    rate1_Q5 = rate0_Q5 + 43 as i32;
                }
            } else if ind_tmp <= -NLSF_QUANT_MAX_AMPLITUDE {
                if ind_tmp == -NLSF_QUANT_MAX_AMPLITUDE {
                    rate0_Q5 = 280 as i32;
                    rate1_Q5 = *rates_Q5
                        .offset((ind_tmp + 1 as i32 + NLSF_QUANT_MAX_AMPLITUDE) as isize)
                        as i32;
                } else {
                    rate0_Q5 = 280 as i32 - 43 as i32 * 4 as i32
                        + -(43 as i32) as i16 as i32 * ind_tmp as i16 as i32;
                    rate1_Q5 = rate0_Q5 - 43 as i32;
                }
            } else {
                rate0_Q5 = *rates_Q5.offset((ind_tmp + NLSF_QUANT_MAX_AMPLITUDE) as isize) as i32;
                rate1_Q5 = *rates_Q5
                    .offset((ind_tmp + 1 as i32 + NLSF_QUANT_MAX_AMPLITUDE) as isize)
                    as i32;
            }
            RD_tmp_Q25 = RD_Q25[j as usize];
            diff_Q10 = in_Q10 - out0_Q10 as i32;
            RD_Q25[j as usize] = RD_tmp_Q25
                + diff_Q10 as i16 as i32 * diff_Q10 as i16 as i32 * *w_Q5.offset(i as isize) as i32
                + mu_Q20 as i16 as i32 * rate0_Q5 as i16 as i32;
            diff_Q10 = in_Q10 - out1_Q10 as i32;
            RD_Q25[(j + nStates) as usize] = RD_tmp_Q25
                + diff_Q10 as i16 as i32 * diff_Q10 as i16 as i32 * *w_Q5.offset(i as isize) as i32
                + mu_Q20 as i16 as i32 * rate1_Q5 as i16 as i32;
            j += 1;
        }
        if nStates <= NLSF_QUANT_DEL_DEC_STATES / 2 as i32 {
            j = 0 as i32;
            while j < nStates {
                ind[(j + nStates) as usize][i as usize] =
                    (ind[j as usize][i as usize] as i32 + 1 as i32) as i8;
                j += 1;
            }
            nStates = ((nStates as u32) << 1 as i32) as i32;
            j = nStates;
            while j < NLSF_QUANT_DEL_DEC_STATES {
                ind[j as usize][i as usize] = ind[(j - nStates) as usize][i as usize];
                j += 1;
            }
        } else {
            j = 0 as i32;
            while j < NLSF_QUANT_DEL_DEC_STATES {
                if RD_Q25[j as usize] > RD_Q25[(j + NLSF_QUANT_DEL_DEC_STATES) as usize] {
                    RD_max_Q25[j as usize] = RD_Q25[j as usize];
                    RD_min_Q25[j as usize] = RD_Q25[(j + NLSF_QUANT_DEL_DEC_STATES) as usize];
                    RD_Q25[j as usize] = RD_min_Q25[j as usize];
                    RD_Q25[(j + NLSF_QUANT_DEL_DEC_STATES) as usize] = RD_max_Q25[j as usize];
                    out0_Q10 = prev_out_Q10[j as usize];
                    prev_out_Q10[j as usize] =
                        prev_out_Q10[(j + NLSF_QUANT_DEL_DEC_STATES) as usize];
                    prev_out_Q10[(j + NLSF_QUANT_DEL_DEC_STATES) as usize] = out0_Q10;
                    ind_sort[j as usize] = j + NLSF_QUANT_DEL_DEC_STATES;
                } else {
                    RD_min_Q25[j as usize] = RD_Q25[j as usize];
                    RD_max_Q25[j as usize] = RD_Q25[(j + NLSF_QUANT_DEL_DEC_STATES) as usize];
                    ind_sort[j as usize] = j;
                }
                j += 1;
            }
            loop {
                min_max_Q25 = silk_int32_MAX;
                max_min_Q25 = 0 as i32;
                ind_min_max = 0 as i32;
                ind_max_min = 0 as i32;
                j = 0 as i32;
                while j < NLSF_QUANT_DEL_DEC_STATES {
                    if min_max_Q25 > RD_max_Q25[j as usize] {
                        min_max_Q25 = RD_max_Q25[j as usize];
                        ind_min_max = j;
                    }
                    if max_min_Q25 < RD_min_Q25[j as usize] {
                        max_min_Q25 = RD_min_Q25[j as usize];
                        ind_max_min = j;
                    }
                    j += 1;
                }
                if min_max_Q25 >= max_min_Q25 {
                    break;
                }
                ind_sort[ind_max_min as usize] =
                    ind_sort[ind_min_max as usize] ^ NLSF_QUANT_DEL_DEC_STATES;
                RD_Q25[ind_max_min as usize] =
                    RD_Q25[(ind_min_max + NLSF_QUANT_DEL_DEC_STATES) as usize];
                prev_out_Q10[ind_max_min as usize] =
                    prev_out_Q10[(ind_min_max + NLSF_QUANT_DEL_DEC_STATES) as usize];
                RD_min_Q25[ind_max_min as usize] = 0 as i32;
                RD_max_Q25[ind_min_max as usize] = silk_int32_MAX;
                memcpy(
                    (ind[ind_max_min as usize]).as_mut_ptr() as *mut core::ffi::c_void,
                    (ind[ind_min_max as usize]).as_mut_ptr() as *const core::ffi::c_void,
                    (16 as i32 as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
            }
            j = 0 as i32;
            while j < NLSF_QUANT_DEL_DEC_STATES {
                ind[j as usize][i as usize] =
                    (ind[j as usize][i as usize] as i32 + (ind_sort[j as usize] >> 2 as i32)) as i8;
                j += 1;
            }
        }
        i -= 1;
    }
    ind_tmp = 0 as i32;
    min_Q25 = silk_int32_MAX;
    j = 0 as i32;
    while j < 2 as i32 * NLSF_QUANT_DEL_DEC_STATES {
        if min_Q25 > RD_Q25[j as usize] {
            min_Q25 = RD_Q25[j as usize];
            ind_tmp = j;
        }
        j += 1;
    }
    j = 0 as i32;
    while j < order as i32 {
        *indices.offset(j as isize) =
            ind[(ind_tmp & NLSF_QUANT_DEL_DEC_STATES - 1 as i32) as usize][j as usize];
        j += 1;
    }
    let ref mut fresh0 = *indices.offset(0 as i32 as isize);
    *fresh0 = (*fresh0 as i32 + (ind_tmp >> 2 as i32)) as i8;
    return min_Q25;
}
