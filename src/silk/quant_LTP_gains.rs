use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
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
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    #[c2rust::src_loc = "52:4"]
    pub type opus_uint8 = uint8_t;
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
    use super::stdint_uintn_h::{uint32_t, uint8_t};
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "176:1"]
        pub fn silk_lin2log(inLin: opus_int32) -> opus_int32;
        #[c2rust::src_loc = "187:1"]
        pub fn silk_log2lin(inLog_Q7: opus_int32) -> opus_int32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    use super::opus_types_h::{opus_int8, opus_uint8};
    extern "C" {
        #[c2rust::src_loc = "78:34"]
        pub static silk_LTP_gain_BITS_Q5_ptrs: [*const opus_uint8; 3];
        #[c2rust::src_loc = "79:34"]
        pub static silk_LTP_vq_ptrs_Q7: [*const opus_int8; 3];
        #[c2rust::src_loc = "80:34"]
        pub static silk_LTP_vq_gain_ptrs_Q7: [*const opus_uint8; 3];
        #[c2rust::src_loc = "81:26"]
        pub static silk_LTP_vq_sizes: [opus_int8; 3];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "146:9"]
    pub const LTP_ORDER: libc::c_int = 5 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use super::opus_types_h::{opus_int32, opus_int8, opus_uint8};
    extern "C" {
        #[c2rust::src_loc = "225:1"]
        pub fn silk_VQ_WMat_EC_c(
            ind: *mut opus_int8,
            res_nrg_Q15: *mut opus_int32,
            rate_dist_Q8: *mut opus_int32,
            gain_Q7: *mut libc::c_int,
            XX_Q17: *const opus_int32,
            xX_Q17: *const opus_int32,
            cb_Q7: *const opus_int8,
            cb_gain_Q7: *const opus_uint8,
            cl_Q5: *const opus_uint8,
            subfr_len: libc::c_int,
            max_gain_Q7: opus_int32,
            L: libc::c_int,
        );
    }
}
pub use self::define_h::LTP_ORDER;
use self::main_h::silk_VQ_WMat_EC_c;
pub use self::opus_types_h::{
    opus_int16, opus_int32, opus_int64, opus_int8, opus_uint32, opus_uint8,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::string_h::memcpy;
use self::tables_h::{
    silk_LTP_gain_BITS_Q5_ptrs, silk_LTP_vq_gain_ptrs_Q7, silk_LTP_vq_ptrs_Q7, silk_LTP_vq_sizes,
};
pub use self::typedef_h::silk_int32_MAX;
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __int8_t, __uint32_t, __uint8_t};
use self::SigProc_FIX_h::{silk_lin2log, silk_log2lin};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_quant_LTP_gains(
    B_Q14: *mut opus_int16,
    cbk_index: *mut opus_int8,
    periodicity_index: *mut opus_int8,
    sum_log_gain_Q7: *mut opus_int32,
    pred_gain_dB_Q7: *mut libc::c_int,
    XX_Q17: *const opus_int32,
    xX_Q17: *const opus_int32,
    subfr_len: libc::c_int,
    nb_subfr: libc::c_int,
    _arch: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut cbk_size: libc::c_int = 0;
    let mut temp_idx: [opus_int8; 4] = [0; 4];
    let mut cl_ptr_Q5: *const opus_uint8 = 0 as *const opus_uint8;
    let mut cbk_ptr_Q7: *const opus_int8 = 0 as *const opus_int8;
    let mut cbk_gain_ptr_Q7: *const opus_uint8 = 0 as *const opus_uint8;
    let mut XX_Q17_ptr: *const opus_int32 = 0 as *const opus_int32;
    let mut xX_Q17_ptr: *const opus_int32 = 0 as *const opus_int32;
    let mut res_nrg_Q15_subfr: opus_int32 = 0;
    let mut res_nrg_Q15: opus_int32 = 0;
    let mut rate_dist_Q7_subfr: opus_int32 = 0;
    let mut rate_dist_Q7: opus_int32 = 0;
    let mut min_rate_dist_Q7: opus_int32 = 0;
    let mut sum_log_gain_tmp_Q7: opus_int32 = 0;
    let mut best_sum_log_gain_Q7: opus_int32 = 0;
    let mut max_gain_Q7: opus_int32 = 0;
    let mut gain_Q7: libc::c_int = 0;
    min_rate_dist_Q7 = silk_int32_MAX;
    best_sum_log_gain_Q7 = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < 3 as libc::c_int {
        let gain_safety: opus_int32 = (0.4f64
            * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int) as libc::c_double
            + 0.5f64) as opus_int32;
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
                    * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int) as libc::c_double
                    + 0.5f64) as opus_int32
                    - sum_log_gain_tmp_Q7
                    + ((7 as libc::c_int as libc::c_long
                        * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int))
                        as libc::c_double
                        + 0.5f64) as opus_int32,
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
            res_nrg_Q15 = if (res_nrg_Q15 as opus_uint32)
                .wrapping_add(res_nrg_Q15_subfr as opus_uint32)
                & 0x80000000 as libc::c_uint
                != 0
            {
                silk_int32_MAX
            } else {
                res_nrg_Q15 + res_nrg_Q15_subfr
            };
            rate_dist_Q7 = if (rate_dist_Q7 as opus_uint32)
                .wrapping_add(rate_dist_Q7_subfr as opus_uint32)
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
                        * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int))
                        as libc::c_double
                        + 0.5f64) as opus_int32
            {
                0 as libc::c_int
            } else {
                sum_log_gain_tmp_Q7 + silk_lin2log(gain_safety + gain_Q7)
                    - ((7 as libc::c_int as libc::c_long
                        * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int))
                        as libc::c_double
                        + 0.5f64) as opus_int32
            };
            XX_Q17_ptr = XX_Q17_ptr.offset((LTP_ORDER * LTP_ORDER) as isize);
            xX_Q17_ptr = xX_Q17_ptr.offset(LTP_ORDER as isize);
            j += 1;
        }
        if rate_dist_Q7 <= min_rate_dist_Q7 {
            min_rate_dist_Q7 = rate_dist_Q7;
            *periodicity_index = k as opus_int8;
            memcpy(
                cbk_index as *mut libc::c_void,
                temp_idx.as_mut_ptr() as *const libc::c_void,
                (nb_subfr as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_int8>() as libc::c_ulong),
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
            *B_Q14.offset((j * LTP_ORDER + k) as isize) =
                ((*cbk_ptr_Q7.offset(
                    (*cbk_index.offset(j as isize) as libc::c_int * 5 as libc::c_int + k) as isize,
                ) as opus_uint32)
                    << 7 as libc::c_int) as opus_int32 as opus_int16;
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
    *pred_gain_dB_Q7 = -(3 as libc::c_int) as opus_int16 as opus_int32
        * (silk_lin2log(res_nrg_Q15) - ((15 as libc::c_int) << 7 as libc::c_int)) as opus_int16
            as opus_int32;
}
