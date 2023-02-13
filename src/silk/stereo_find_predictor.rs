use crate::silk::inner_prod_aligned::silk_inner_prod_aligned_scale;
use crate::silk::sum_sqr_shift::silk_sum_sqr_shift;
use crate::silk::SigProc_FIX::silk_max_int;

use crate::silk::Inlines::{silk_DIV32_varQ, silk_SQRT_APPROX};

#[c2rust::src_loc = "35:1"]
pub unsafe fn silk_stereo_find_predictor(
    ratio_Q14: *mut i32,
    x: *const i16,
    y: *const i16,
    mid_res_amp_Q0: *mut i32,
    length: i32,
    mut smooth_coef_Q16: i32,
) -> i32 {
    let mut scale: i32 = 0;
    let mut scale1: i32 = 0;
    let mut scale2: i32 = 0;
    let mut nrgx: i32 = 0;
    let mut nrgy: i32 = 0;
    let mut corr: i32 = 0;
    let mut pred_Q13: i32 = 0;
    let mut pred2_Q10: i32 = 0;
    silk_sum_sqr_shift(&mut nrgx, &mut scale1, x, length);
    silk_sum_sqr_shift(&mut nrgy, &mut scale2, y, length);
    scale = silk_max_int(scale1, scale2);
    scale = scale + (scale & 1 as i32);
    nrgy = nrgy >> scale - scale2;
    nrgx = nrgx >> scale - scale1;
    nrgx = silk_max_int(nrgx, 1 as i32);
    corr = silk_inner_prod_aligned_scale(x, y, scale, length);
    pred_Q13 = silk_DIV32_varQ(corr, nrgx, 13 as i32);
    pred_Q13 = if -((1 as i32) << 14 as i32) > (1 as i32) << 14 as i32 {
        if pred_Q13 > -((1 as i32) << 14 as i32) {
            -((1 as i32) << 14 as i32)
        } else if pred_Q13 < (1 as i32) << 14 as i32 {
            (1 as i32) << 14 as i32
        } else {
            pred_Q13
        }
    } else if pred_Q13 > (1 as i32) << 14 as i32 {
        (1 as i32) << 14 as i32
    } else if pred_Q13 < -((1 as i32) << 14 as i32) {
        -((1 as i32) << 14 as i32)
    } else {
        pred_Q13
    };
    pred2_Q10 = (pred_Q13 as i64 * pred_Q13 as i16 as i64 >> 16 as i32) as i32;
    smooth_coef_Q16 = silk_max_int(
        smooth_coef_Q16,
        if pred2_Q10 > 0 as i32 {
            pred2_Q10
        } else {
            -pred2_Q10
        },
    );
    scale = scale >> 1 as i32;
    *mid_res_amp_Q0.offset(0 as i32 as isize) = (*mid_res_amp_Q0.offset(0 as i32 as isize) as i64
        + ((((silk_SQRT_APPROX(nrgx) as u32) << scale) as i32
            - *mid_res_amp_Q0.offset(0 as i32 as isize)) as i64
            * smooth_coef_Q16 as i16 as i64
            >> 16 as i32)) as i32;
    nrgy = nrgy
        - (((corr as i64 * pred_Q13 as i16 as i64 >> 16 as i32) as i32 as u32)
            << 3 as i32 + 1 as i32) as i32;
    nrgy = nrgy
        + (((nrgx as i64 * pred2_Q10 as i16 as i64 >> 16 as i32) as i32 as u32) << 6 as i32) as i32;
    *mid_res_amp_Q0.offset(1 as i32 as isize) = (*mid_res_amp_Q0.offset(1 as i32 as isize) as i64
        + ((((silk_SQRT_APPROX(nrgy) as u32) << scale) as i32
            - *mid_res_amp_Q0.offset(1 as i32 as isize)) as i64
            * smooth_coef_Q16 as i16 as i64
            >> 16 as i32)) as i32;
    *ratio_Q14 = silk_DIV32_varQ(
        *mid_res_amp_Q0.offset(1 as i32 as isize),
        if *mid_res_amp_Q0.offset(0 as i32 as isize) > 1 as i32 {
            *mid_res_amp_Q0.offset(0 as i32 as isize)
        } else {
            1 as i32
        },
        14 as i32,
    );
    *ratio_Q14 = if 0 as i32 > 32767 as i32 {
        if *ratio_Q14 > 0 as i32 {
            0 as i32
        } else if *ratio_Q14 < 32767 as i32 {
            32767 as i32
        } else {
            *ratio_Q14
        }
    } else if *ratio_Q14 > 32767 as i32 {
        32767 as i32
    } else if *ratio_Q14 < 0 as i32 {
        0 as i32
    } else {
        *ratio_Q14
    };
    return pred_Q13;
}
