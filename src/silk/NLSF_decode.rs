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
    use super::types_h::{__int8_t, __int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
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
    use super::stdint_intn_h::{int8_t, int16_t, int32_t, int64_t};
    use super::stdint_uintn_h::{uint8_t, uint32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:9"]
    pub struct silk_NLSF_CB_struct {
        pub nVectors: opus_int16,
        pub order: opus_int16,
        pub quantStepSize_Q16: opus_int16,
        pub invQuantStepSize_Q6: opus_int16,
        pub CB1_NLSF_Q8: *const opus_uint8,
        pub CB1_Wght_Q9: *const opus_int16,
        pub CB1_iCDF: *const opus_uint8,
        pub pred_Q8: *const opus_uint8,
        pub ec_sel: *const opus_uint8,
        pub ec_iCDF: *const opus_uint8,
        pub ec_Rates_Q5: *const opus_uint8,
        pub deltaMin_Q15: *const opus_int16,
    }
    use super::opus_types_h::{opus_int16, opus_uint8};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    use super::opus_types_h::opus_int16;
    extern "C" {
        #[c2rust::src_loc = "322:1"]
        pub fn silk_NLSF_stabilize(
            NLSF_Q15: *mut opus_int16,
            NDeltaMin_Q15: *const opus_int16,
            L: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use super::opus_types_h::{opus_int16, opus_uint8};
    use super::structs_h::silk_NLSF_CB_struct;
    extern "C" {
        #[c2rust::src_loc = "373:1"]
        pub fn silk_NLSF_unpack(
            ec_ix: *mut opus_int16,
            pred_Q8: *mut opus_uint8,
            psNLSF_CB: *const silk_NLSF_CB_struct,
            CB1_index: libc::c_int,
        );
    }
}
pub use self::types_h::{
    __int8_t, __uint8_t, __int16_t, __int32_t, __uint32_t, __int64_t,
};
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::opus_types_h::{
    opus_int8, opus_uint8, opus_int16, opus_int32, opus_uint32, opus_int64,
};
pub use self::structs_h::silk_NLSF_CB_struct;
use self::SigProc_FIX_h::silk_NLSF_stabilize;
use self::main_h::silk_NLSF_unpack;
#[inline]
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn silk_NLSF_residual_dequant(
    mut x_Q10: *mut opus_int16,
    mut indices: *const opus_int8,
    mut pred_coef_Q8: *const opus_uint8,
    quant_step_size_Q16: libc::c_int,
    order: opus_int16,
) {
    let mut i: libc::c_int = 0;
    let mut out_Q10: libc::c_int = 0;
    let mut pred_Q10: libc::c_int = 0;
    out_Q10 = 0 as libc::c_int;
    i = order as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        pred_Q10 = out_Q10 as opus_int16 as opus_int32
            * *pred_coef_Q8.offset(i as isize) as opus_int16 as opus_int32
            >> 8 as libc::c_int;
        out_Q10 = ((*indices.offset(i as isize) as opus_uint32) << 10 as libc::c_int)
            as opus_int32;
        if out_Q10 > 0 as libc::c_int {
            out_Q10 = out_Q10
                - (0.1f64
                    * ((1 as libc::c_int as opus_int64) << 10 as libc::c_int)
                        as libc::c_double + 0.5f64) as opus_int32;
        } else if out_Q10 < 0 as libc::c_int {
            out_Q10 = out_Q10
                + (0.1f64
                    * ((1 as libc::c_int as opus_int64) << 10 as libc::c_int)
                        as libc::c_double + 0.5f64) as opus_int32;
        }
        out_Q10 = (pred_Q10 as libc::c_long
            + (out_Q10 as libc::c_long * quant_step_size_Q16 as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *x_Q10.offset(i as isize) = out_Q10 as opus_int16;
        i -= 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn silk_NLSF_decode(
    mut pNLSF_Q15: *mut opus_int16,
    mut NLSFIndices: *mut opus_int8,
    mut psNLSF_CB: *const silk_NLSF_CB_struct,
) {
    let mut i: libc::c_int = 0;
    let mut pred_Q8: [opus_uint8; 16] = [0; 16];
    let mut ec_ix: [opus_int16; 16] = [0; 16];
    let mut res_Q10: [opus_int16; 16] = [0; 16];
    let mut NLSF_Q15_tmp: opus_int32 = 0;
    let mut pCB_element: *const opus_uint8 = 0 as *const opus_uint8;
    let mut pCB_Wght_Q9: *const opus_int16 = 0 as *const opus_int16;
    silk_NLSF_unpack(
        ec_ix.as_mut_ptr(),
        pred_Q8.as_mut_ptr(),
        psNLSF_CB,
        *NLSFIndices.offset(0 as libc::c_int as isize) as libc::c_int,
    );
    silk_NLSF_residual_dequant(
        res_Q10.as_mut_ptr(),
        &mut *NLSFIndices.offset(1 as libc::c_int as isize) as *mut opus_int8
            as *const opus_int8,
        pred_Q8.as_mut_ptr() as *const opus_uint8,
        (*psNLSF_CB).quantStepSize_Q16 as libc::c_int,
        (*psNLSF_CB).order,
    );
    pCB_element = &*((*psNLSF_CB).CB1_NLSF_Q8)
        .offset(
            (*NLSFIndices.offset(0 as libc::c_int as isize) as libc::c_int
                * (*psNLSF_CB).order as libc::c_int) as isize,
        ) as *const opus_uint8;
    pCB_Wght_Q9 = &*((*psNLSF_CB).CB1_Wght_Q9)
        .offset(
            (*NLSFIndices.offset(0 as libc::c_int as isize) as libc::c_int
                * (*psNLSF_CB).order as libc::c_int) as isize,
        ) as *const opus_int16;
    i = 0 as libc::c_int;
    while i < (*psNLSF_CB).order as libc::c_int {
        NLSF_Q15_tmp = ((res_Q10[i as usize] as opus_int32 as opus_uint32)
            << 14 as libc::c_int) as opus_int32
            / *pCB_Wght_Q9.offset(i as isize) as libc::c_int
            + ((*pCB_element.offset(i as isize) as opus_int16 as opus_uint32)
                << 7 as libc::c_int) as opus_int32;
        *pNLSF_Q15
            .offset(
                i as isize,
            ) = (if 0 as libc::c_int > 32767 as libc::c_int {
            if NLSF_Q15_tmp > 0 as libc::c_int {
                0 as libc::c_int
            } else if NLSF_Q15_tmp < 32767 as libc::c_int {
                32767 as libc::c_int
            } else {
                NLSF_Q15_tmp
            }
        } else if NLSF_Q15_tmp > 32767 as libc::c_int {
            32767 as libc::c_int
        } else if NLSF_Q15_tmp < 0 as libc::c_int {
            0 as libc::c_int
        } else {
            NLSF_Q15_tmp
        }) as opus_int16;
        i += 1;
    }
    silk_NLSF_stabilize(
        pNLSF_Q15,
        (*psNLSF_CB).deltaMin_Q15,
        (*psNLSF_CB).order as libc::c_int,
    );
}
