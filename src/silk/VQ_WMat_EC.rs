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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "176:1"]
        pub fn silk_lin2log(inLin: opus_int32) -> opus_int32;
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
pub use self::define_h::LTP_ORDER;
pub use self::opus_types_h::{
    opus_int16, opus_int32, opus_int64, opus_int8, opus_uint32, opus_uint8,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
pub use self::typedef_h::silk_int32_MAX;
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __int8_t, __uint32_t, __uint8_t};
use self::SigProc_FIX_h::silk_lin2log;
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_VQ_WMat_EC_c(
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
) {
    let mut k: libc::c_int = 0;
    let mut gain_tmp_Q7: libc::c_int = 0;
    let mut cb_row_Q7: *const opus_int8 = 0 as *const opus_int8;
    let mut neg_xX_Q24: [opus_int32; 5] = [0; 5];
    let mut sum1_Q15: opus_int32 = 0;
    let mut sum2_Q24: opus_int32 = 0;
    let mut bits_res_Q8: opus_int32 = 0;
    let mut bits_tot_Q8: opus_int32 = 0;
    neg_xX_Q24[0 as libc::c_int as usize] = -(((*xX_Q17.offset(0 as libc::c_int as isize)
        as opus_uint32)
        << 7 as libc::c_int) as opus_int32);
    neg_xX_Q24[1 as libc::c_int as usize] = -(((*xX_Q17.offset(1 as libc::c_int as isize)
        as opus_uint32)
        << 7 as libc::c_int) as opus_int32);
    neg_xX_Q24[2 as libc::c_int as usize] = -(((*xX_Q17.offset(2 as libc::c_int as isize)
        as opus_uint32)
        << 7 as libc::c_int) as opus_int32);
    neg_xX_Q24[3 as libc::c_int as usize] = -(((*xX_Q17.offset(3 as libc::c_int as isize)
        as opus_uint32)
        << 7 as libc::c_int) as opus_int32);
    neg_xX_Q24[4 as libc::c_int as usize] = -(((*xX_Q17.offset(4 as libc::c_int as isize)
        as opus_uint32)
        << 7 as libc::c_int) as opus_int32);
    *rate_dist_Q8 = silk_int32_MAX;
    *res_nrg_Q15 = silk_int32_MAX;
    cb_row_Q7 = cb_Q7;
    *ind = 0 as libc::c_int as opus_int8;
    k = 0 as libc::c_int;
    while k < L {
        let mut penalty: opus_int32 = 0;
        gain_tmp_Q7 = *cb_gain_Q7.offset(k as isize) as libc::c_int;
        sum1_Q15 = (1.001f64
            * ((1 as libc::c_int as opus_int64) << 15 as libc::c_int) as libc::c_double
            + 0.5f64) as opus_int32;
        penalty = (((if gain_tmp_Q7 - max_gain_Q7 > 0 as libc::c_int {
            gain_tmp_Q7 - max_gain_Q7
        } else {
            0 as libc::c_int
        }) as opus_uint32)
            << 11 as libc::c_int) as opus_int32;
        sum2_Q24 = neg_xX_Q24[0 as libc::c_int as usize]
            + *XX_Q17.offset(1 as libc::c_int as isize)
                * *cb_row_Q7.offset(1 as libc::c_int as isize) as libc::c_int;
        sum2_Q24 = sum2_Q24
            + *XX_Q17.offset(2 as libc::c_int as isize)
                * *cb_row_Q7.offset(2 as libc::c_int as isize) as libc::c_int;
        sum2_Q24 = sum2_Q24
            + *XX_Q17.offset(3 as libc::c_int as isize)
                * *cb_row_Q7.offset(3 as libc::c_int as isize) as libc::c_int;
        sum2_Q24 = sum2_Q24
            + *XX_Q17.offset(4 as libc::c_int as isize)
                * *cb_row_Q7.offset(4 as libc::c_int as isize) as libc::c_int;
        sum2_Q24 = ((sum2_Q24 as opus_uint32) << 1 as libc::c_int) as opus_int32;
        sum2_Q24 = sum2_Q24
            + *XX_Q17.offset(0 as libc::c_int as isize)
                * *cb_row_Q7.offset(0 as libc::c_int as isize) as libc::c_int;
        sum1_Q15 = (sum1_Q15 as libc::c_long
            + (sum2_Q24 as libc::c_long
                * *cb_row_Q7.offset(0 as libc::c_int as isize) as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        sum2_Q24 = neg_xX_Q24[1 as libc::c_int as usize]
            + *XX_Q17.offset(7 as libc::c_int as isize)
                * *cb_row_Q7.offset(2 as libc::c_int as isize) as libc::c_int;
        sum2_Q24 = sum2_Q24
            + *XX_Q17.offset(8 as libc::c_int as isize)
                * *cb_row_Q7.offset(3 as libc::c_int as isize) as libc::c_int;
        sum2_Q24 = sum2_Q24
            + *XX_Q17.offset(9 as libc::c_int as isize)
                * *cb_row_Q7.offset(4 as libc::c_int as isize) as libc::c_int;
        sum2_Q24 = ((sum2_Q24 as opus_uint32) << 1 as libc::c_int) as opus_int32;
        sum2_Q24 = sum2_Q24
            + *XX_Q17.offset(6 as libc::c_int as isize)
                * *cb_row_Q7.offset(1 as libc::c_int as isize) as libc::c_int;
        sum1_Q15 = (sum1_Q15 as libc::c_long
            + (sum2_Q24 as libc::c_long
                * *cb_row_Q7.offset(1 as libc::c_int as isize) as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        sum2_Q24 = neg_xX_Q24[2 as libc::c_int as usize]
            + *XX_Q17.offset(13 as libc::c_int as isize)
                * *cb_row_Q7.offset(3 as libc::c_int as isize) as libc::c_int;
        sum2_Q24 = sum2_Q24
            + *XX_Q17.offset(14 as libc::c_int as isize)
                * *cb_row_Q7.offset(4 as libc::c_int as isize) as libc::c_int;
        sum2_Q24 = ((sum2_Q24 as opus_uint32) << 1 as libc::c_int) as opus_int32;
        sum2_Q24 = sum2_Q24
            + *XX_Q17.offset(12 as libc::c_int as isize)
                * *cb_row_Q7.offset(2 as libc::c_int as isize) as libc::c_int;
        sum1_Q15 = (sum1_Q15 as libc::c_long
            + (sum2_Q24 as libc::c_long
                * *cb_row_Q7.offset(2 as libc::c_int as isize) as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        sum2_Q24 = neg_xX_Q24[3 as libc::c_int as usize]
            + *XX_Q17.offset(19 as libc::c_int as isize)
                * *cb_row_Q7.offset(4 as libc::c_int as isize) as libc::c_int;
        sum2_Q24 = ((sum2_Q24 as opus_uint32) << 1 as libc::c_int) as opus_int32;
        sum2_Q24 = sum2_Q24
            + *XX_Q17.offset(18 as libc::c_int as isize)
                * *cb_row_Q7.offset(3 as libc::c_int as isize) as libc::c_int;
        sum1_Q15 = (sum1_Q15 as libc::c_long
            + (sum2_Q24 as libc::c_long
                * *cb_row_Q7.offset(3 as libc::c_int as isize) as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        sum2_Q24 = ((neg_xX_Q24[4 as libc::c_int as usize] as opus_uint32) << 1 as libc::c_int)
            as opus_int32;
        sum2_Q24 = sum2_Q24
            + *XX_Q17.offset(24 as libc::c_int as isize)
                * *cb_row_Q7.offset(4 as libc::c_int as isize) as libc::c_int;
        sum1_Q15 = (sum1_Q15 as libc::c_long
            + (sum2_Q24 as libc::c_long
                * *cb_row_Q7.offset(4 as libc::c_int as isize) as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        if sum1_Q15 >= 0 as libc::c_int {
            bits_res_Q8 = subfr_len as opus_int16 as opus_int32
                * (silk_lin2log(sum1_Q15 + penalty) - ((15 as libc::c_int) << 7 as libc::c_int))
                    as opus_int16 as opus_int32;
            bits_tot_Q8 = bits_res_Q8
                + ((*cl_Q5.offset(k as isize) as opus_uint32)
                    << 3 as libc::c_int - 1 as libc::c_int) as opus_int32;
            if bits_tot_Q8 <= *rate_dist_Q8 {
                *rate_dist_Q8 = bits_tot_Q8;
                *res_nrg_Q15 = sum1_Q15 + penalty;
                *ind = k as opus_int8;
                *gain_Q7 = gain_tmp_Q7;
            }
        }
        cb_row_Q7 = cb_row_Q7.offset(LTP_ORDER as isize);
        k += 1;
    }
}
