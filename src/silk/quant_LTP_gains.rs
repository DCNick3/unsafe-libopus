use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
}
pub use self::typedef_h::silk_int32_MAX;
use crate::externs::memcpy;
use crate::silk::define::LTP_ORDER;
use crate::silk::lin2log::silk_lin2log;
use crate::silk::log2lin::silk_log2lin;
use crate::silk::tables_LTP::{
    silk_LTP_gain_BITS_Q5_ptrs, silk_LTP_vq_gain_ptrs_Q7, silk_LTP_vq_ptrs_Q7, silk_LTP_vq_sizes,
};
use crate::silk::VQ_WMat_EC::silk_VQ_WMat_EC_c;

#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_quant_LTP_gains(
    B_Q14: *mut i16,
    cbk_index: *mut i8,
    periodicity_index: *mut i8,
    sum_log_gain_Q7: *mut i32,
    pred_gain_dB_Q7: *mut libc::c_int,
    XX_Q17: *const i32,
    xX_Q17: *const i32,
    subfr_len: libc::c_int,
    nb_subfr: libc::c_int,
    _arch: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut cbk_size: libc::c_int = 0;
    let mut temp_idx: [i8; 4] = [0; 4];
    let mut cl_ptr_Q5: *const u8 = 0 as *const u8;
    let mut cbk_ptr_Q7: *const i8 = 0 as *const i8;
    let mut cbk_gain_ptr_Q7: *const u8 = 0 as *const u8;
    let mut XX_Q17_ptr: *const i32 = 0 as *const i32;
    let mut xX_Q17_ptr: *const i32 = 0 as *const i32;
    let mut res_nrg_Q15_subfr: i32 = 0;
    let mut res_nrg_Q15: i32 = 0;
    let mut rate_dist_Q7_subfr: i32 = 0;
    let mut rate_dist_Q7: i32 = 0;
    let mut min_rate_dist_Q7: i32 = 0;
    let mut sum_log_gain_tmp_Q7: i32 = 0;
    let mut best_sum_log_gain_Q7: i32 = 0;
    let mut max_gain_Q7: i32 = 0;
    let mut gain_Q7: libc::c_int = 0;
    min_rate_dist_Q7 = silk_int32_MAX;
    best_sum_log_gain_Q7 = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < 3 as libc::c_int {
        let gain_safety: i32 = (0.4f64
            * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_double
            + 0.5f64) as i32;
        cl_ptr_Q5 = silk_LTP_gain_BITS_Q5_ptrs[k as usize];
        cbk_ptr_Q7 = silk_LTP_vq_ptrs_Q7[k as usize];
        cbk_gain_ptr_Q7 = silk_LTP_vq_gain_ptrs_Q7[k as usize];
        cbk_size = silk_LTP_vq_sizes[k as usize] as libc::c_int;
        XX_Q17_ptr = XX_Q17;
        xX_Q17_ptr = xX_Q17;
        res_nrg_Q15 = 0 as libc::c_int;
        rate_dist_Q7 = 0 as libc::c_int;
        sum_log_gain_tmp_Q7 = *sum_log_gain_Q7;
        j = 0 as libc::c_int;
        while j < nb_subfr {
            max_gain_Q7 = silk_log2lin(
                (250.0f32 as libc::c_double / 6.0f64
                    * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_double
                    + 0.5f64) as i32
                    - sum_log_gain_tmp_Q7
                    + ((7 as libc::c_int as libc::c_long
                        * ((1 as libc::c_int as i64) << 7 as libc::c_int))
                        as libc::c_double
                        + 0.5f64) as i32,
            ) - gain_safety;
            silk_VQ_WMat_EC_c(
                &mut *temp_idx.as_mut_ptr().offset(j as isize),
                &mut res_nrg_Q15_subfr,
                &mut rate_dist_Q7_subfr,
                &mut gain_Q7,
                XX_Q17_ptr,
                xX_Q17_ptr,
                cbk_ptr_Q7,
                cbk_gain_ptr_Q7,
                cl_ptr_Q5,
                subfr_len,
                max_gain_Q7,
                cbk_size,
            );
            res_nrg_Q15 = if (res_nrg_Q15 as u32).wrapping_add(res_nrg_Q15_subfr as u32)
                & 0x80000000 as libc::c_uint
                != 0
            {
                silk_int32_MAX
            } else {
                res_nrg_Q15 + res_nrg_Q15_subfr
            };
            rate_dist_Q7 = if (rate_dist_Q7 as u32).wrapping_add(rate_dist_Q7_subfr as u32)
                & 0x80000000 as libc::c_uint
                != 0
            {
                silk_int32_MAX
            } else {
                rate_dist_Q7 + rate_dist_Q7_subfr
            };
            sum_log_gain_tmp_Q7 = if 0 as libc::c_int
                > sum_log_gain_tmp_Q7 + silk_lin2log(gain_safety + gain_Q7)
                    - ((7 as libc::c_int as libc::c_long
                        * ((1 as libc::c_int as i64) << 7 as libc::c_int))
                        as libc::c_double
                        + 0.5f64) as i32
            {
                0 as libc::c_int
            } else {
                sum_log_gain_tmp_Q7 + silk_lin2log(gain_safety + gain_Q7)
                    - ((7 as libc::c_int as libc::c_long
                        * ((1 as libc::c_int as i64) << 7 as libc::c_int))
                        as libc::c_double
                        + 0.5f64) as i32
            };
            XX_Q17_ptr = XX_Q17_ptr.offset((LTP_ORDER * LTP_ORDER) as isize);
            xX_Q17_ptr = xX_Q17_ptr.offset(LTP_ORDER as isize);
            j += 1;
        }
        if rate_dist_Q7 <= min_rate_dist_Q7 {
            min_rate_dist_Q7 = rate_dist_Q7;
            *periodicity_index = k as i8;
            memcpy(
                cbk_index as *mut libc::c_void,
                temp_idx.as_mut_ptr() as *const libc::c_void,
                (nb_subfr as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i8>() as libc::c_ulong),
            );
            best_sum_log_gain_Q7 = sum_log_gain_tmp_Q7;
        }
        k += 1;
    }
    cbk_ptr_Q7 = silk_LTP_vq_ptrs_Q7[*periodicity_index as usize];
    j = 0 as libc::c_int;
    while j < nb_subfr {
        k = 0 as libc::c_int;
        while k < LTP_ORDER {
            *B_Q14.offset((j * LTP_ORDER + k) as isize) = ((*cbk_ptr_Q7.offset(
                (*cbk_index.offset(j as isize) as libc::c_int * 5 as libc::c_int + k) as isize,
            ) as u32)
                << 7 as libc::c_int)
                as i32 as i16;
            k += 1;
        }
        j += 1;
    }
    if nb_subfr == 2 as libc::c_int {
        res_nrg_Q15 = res_nrg_Q15 >> 1 as libc::c_int;
    } else {
        res_nrg_Q15 = res_nrg_Q15 >> 2 as libc::c_int;
    }
    *sum_log_gain_Q7 = best_sum_log_gain_Q7;
    *pred_gain_dB_Q7 = -(3 as libc::c_int) as i16 as i32
        * (silk_lin2log(res_nrg_Q15) - ((15 as libc::c_int) << 7 as libc::c_int)) as i16 as i32;
}
