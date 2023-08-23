use crate::silk::NLSF_stabilize::silk_NLSF_stabilize;
use crate::silk::NLSF_unpack::silk_NLSF_unpack;

use crate::silk::structs::silk_NLSF_CB_struct;

#[inline]
unsafe fn silk_NLSF_residual_dequant(
    x_Q10: *mut i16,
    indices: *const i8,
    pred_coef_Q8: *const u8,
    quant_step_size_Q16: i32,
    order: i16,
) {
    let mut i: i32 = 0;
    let mut out_Q10: i32 = 0;
    let mut pred_Q10: i32 = 0;
    out_Q10 = 0;
    i = order as i32 - 1;
    while i >= 0 {
        pred_Q10 = out_Q10 as i16 as i32 * *pred_coef_Q8.offset(i as isize) as i16 as i32 >> 8;
        out_Q10 = ((*indices.offset(i as isize) as u32) << 10) as i32;
        if out_Q10 > 0 {
            out_Q10 = out_Q10 - (0.1f64 * ((1) << 10) as f64 + 0.5f64) as i32;
        } else if out_Q10 < 0 {
            out_Q10 = out_Q10 + (0.1f64 * ((1) << 10) as f64 + 0.5f64) as i32;
        }
        out_Q10 =
            (pred_Q10 as i64 + (out_Q10 as i64 * quant_step_size_Q16 as i16 as i64 >> 16)) as i32;
        *x_Q10.offset(i as isize) = out_Q10 as i16;
        i -= 1;
    }
}
pub unsafe fn silk_NLSF_decode(
    pNLSF_Q15: *mut i16,
    NLSFIndices: *mut i8,
    psNLSF_CB: &silk_NLSF_CB_struct,
) {
    let mut i: i32 = 0;
    let mut pred_Q8: [u8; 16] = [0; 16];
    let mut ec_ix: [i16; 16] = [0; 16];
    let mut res_Q10: [i16; 16] = [0; 16];
    let mut NLSF_Q15_tmp: i32 = 0;
    silk_NLSF_unpack(
        ec_ix.as_mut_ptr(),
        pred_Q8.as_mut_ptr(),
        psNLSF_CB,
        *NLSFIndices.offset(0) as i32,
    );
    silk_NLSF_residual_dequant(
        res_Q10.as_mut_ptr(),
        &mut *NLSFIndices.offset(1) as *mut i8 as *const i8,
        pred_Q8.as_mut_ptr() as *const u8,
        psNLSF_CB.quantStepSize_Q16 as i32,
        psNLSF_CB.order,
    );
    let pCB_element =
        &psNLSF_CB.CB1_NLSF_Q8[(*NLSFIndices.offset(0) as i32 * psNLSF_CB.order as i32) as usize..];
    let pCB_Wght_Q9 =
        &psNLSF_CB.CB1_Wght_Q9[(*NLSFIndices.offset(0) as i32 * psNLSF_CB.order as i32) as usize..];
    i = 0;
    while i < psNLSF_CB.order as i32 {
        NLSF_Q15_tmp = ((res_Q10[i as usize] as i32 as u32) << 14) as i32
            / pCB_Wght_Q9[i as usize] as i32
            + ((pCB_element[i as usize] as i16 as u32) << 7) as i32;
        *pNLSF_Q15.offset(i as isize) = (if 0 > 32767 {
            if NLSF_Q15_tmp > 0 {
                0
            } else if NLSF_Q15_tmp < 32767 {
                32767
            } else {
                NLSF_Q15_tmp
            }
        } else if NLSF_Q15_tmp > 32767 {
            32767
        } else if NLSF_Q15_tmp < 0 {
            0
        } else {
            NLSF_Q15_tmp
        }) as i16;
        i += 1;
    }
    silk_NLSF_stabilize(pNLSF_Q15, psNLSF_CB.deltaMin_Q15, psNLSF_CB.order as i32);
}
