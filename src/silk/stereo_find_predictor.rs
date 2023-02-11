use crate::silk::inner_prod_aligned::silk_inner_prod_aligned_scale;
use crate::silk::sum_sqr_shift::silk_sum_sqr_shift;
use crate::silk::SigProc_FIX::silk_max_int;
use ::libc;

use crate::silk::Inlines::{silk_DIV32_varQ, silk_SQRT_APPROX};

#[c2rust::src_loc = "35:1"]
pub unsafe fn silk_stereo_find_predictor(
    ratio_Q14: *mut i32,
    x: *const i16,
    y: *const i16,
    mid_res_amp_Q0: *mut i32,
    length: libc::c_int,
    mut smooth_coef_Q16: libc::c_int,
) -> i32 {
    let mut scale: libc::c_int = 0;
    let mut scale1: libc::c_int = 0;
    let mut scale2: libc::c_int = 0;
    let mut nrgx: i32 = 0;
    let mut nrgy: i32 = 0;
    let mut corr: i32 = 0;
    let mut pred_Q13: i32 = 0;
    let mut pred2_Q10: i32 = 0;
    silk_sum_sqr_shift(&mut nrgx, &mut scale1, x, length);
    silk_sum_sqr_shift(&mut nrgy, &mut scale2, y, length);
    scale = silk_max_int(scale1, scale2);
    scale = scale + (scale & 1 as libc::c_int);
    nrgy = nrgy >> scale - scale2;
    nrgx = nrgx >> scale - scale1;
    nrgx = silk_max_int(nrgx, 1 as libc::c_int);
    corr = silk_inner_prod_aligned_scale(x, y, scale, length);
    pred_Q13 = silk_DIV32_varQ(corr, nrgx, 13 as libc::c_int);
    pred_Q13 =
        if -((1 as libc::c_int) << 14 as libc::c_int) > (1 as libc::c_int) << 14 as libc::c_int {
            if pred_Q13 > -((1 as libc::c_int) << 14 as libc::c_int) {
                -((1 as libc::c_int) << 14 as libc::c_int)
            } else if pred_Q13 < (1 as libc::c_int) << 14 as libc::c_int {
                (1 as libc::c_int) << 14 as libc::c_int
            } else {
                pred_Q13
            }
        } else if pred_Q13 > (1 as libc::c_int) << 14 as libc::c_int {
            (1 as libc::c_int) << 14 as libc::c_int
        } else if pred_Q13 < -((1 as libc::c_int) << 14 as libc::c_int) {
            -((1 as libc::c_int) << 14 as libc::c_int)
        } else {
            pred_Q13
        };
    pred2_Q10 = (pred_Q13 as libc::c_long * pred_Q13 as i16 as i64 >> 16 as libc::c_int) as i32;
    smooth_coef_Q16 = silk_max_int(
        smooth_coef_Q16,
        if pred2_Q10 > 0 as libc::c_int {
            pred2_Q10
        } else {
            -pred2_Q10
        },
    );
    scale = scale >> 1 as libc::c_int;
    *mid_res_amp_Q0.offset(0 as libc::c_int as isize) = (*mid_res_amp_Q0
        .offset(0 as libc::c_int as isize)
        as libc::c_long
        + ((((silk_SQRT_APPROX(nrgx) as u32) << scale) as i32
            - *mid_res_amp_Q0.offset(0 as libc::c_int as isize)) as libc::c_long
            * smooth_coef_Q16 as i16 as i64
            >> 16 as libc::c_int)) as i32;
    nrgy = nrgy
        - (((corr as libc::c_long * pred_Q13 as i16 as i64 >> 16 as libc::c_int) as i32 as u32)
            << 3 as libc::c_int + 1 as libc::c_int) as i32;
    nrgy = nrgy
        + (((nrgx as libc::c_long * pred2_Q10 as i16 as i64 >> 16 as libc::c_int) as i32 as u32)
            << 6 as libc::c_int) as i32;
    *mid_res_amp_Q0.offset(1 as libc::c_int as isize) = (*mid_res_amp_Q0
        .offset(1 as libc::c_int as isize)
        as libc::c_long
        + ((((silk_SQRT_APPROX(nrgy) as u32) << scale) as i32
            - *mid_res_amp_Q0.offset(1 as libc::c_int as isize)) as libc::c_long
            * smooth_coef_Q16 as i16 as i64
            >> 16 as libc::c_int)) as i32;
    *ratio_Q14 = silk_DIV32_varQ(
        *mid_res_amp_Q0.offset(1 as libc::c_int as isize),
        if *mid_res_amp_Q0.offset(0 as libc::c_int as isize) > 1 as libc::c_int {
            *mid_res_amp_Q0.offset(0 as libc::c_int as isize)
        } else {
            1 as libc::c_int
        },
        14 as libc::c_int,
    );
    *ratio_Q14 = if 0 as libc::c_int > 32767 as libc::c_int {
        if *ratio_Q14 > 0 as libc::c_int {
            0 as libc::c_int
        } else if *ratio_Q14 < 32767 as libc::c_int {
            32767 as libc::c_int
        } else {
            *ratio_Q14
        }
    } else if *ratio_Q14 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if *ratio_Q14 < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        *ratio_Q14
    };
    return pred_Q13;
}
