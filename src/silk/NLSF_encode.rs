use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use crate::silk::structs::silk_NLSF_CB_struct;
    extern "C" {
        #[c2rust::src_loc = "383:1"]
        pub fn silk_NLSF_decode(
            pNLSF_Q15: *mut i16,
            NLSFIndices: *mut i8,
            psNLSF_CB: *const silk_NLSF_CB_struct,
        );
        #[c2rust::src_loc = "359:1"]
        pub fn silk_NLSF_del_dec_quant(
            indices: *mut i8,
            x_Q10: *const i16,
            w_Q5: *const i16,
            pred_coef_Q8: *const u8,
            ec_ix: *const i16,
            ec_rates_Q5: *const u8,
            quant_step_size_Q16: libc::c_int,
            inv_quant_step_size_Q6: i16,
            mu_Q20: i32,
            order: i16,
        ) -> i32;
        #[c2rust::src_loc = "373:1"]
        pub fn silk_NLSF_unpack(
            ec_ix: *mut i16,
            pred_Q8: *mut u8,
            psNLSF_CB: *const silk_NLSF_CB_struct,
            CB1_index: libc::c_int,
        );
        #[c2rust::src_loc = "349:1"]
        pub fn silk_NLSF_VQ(
            err_Q26: *mut i32,
            in_Q15: *const i16,
            pCB_Q8: *const u8,
            pWght_Q9: *const i16,
            K: libc::c_int,
            LPC_order: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "142:9"]
    pub const MAX_LPC_ORDER: libc::c_int = 16 as libc::c_int;
}
pub use self::define_h::MAX_LPC_ORDER;
use self::main_h::{silk_NLSF_VQ, silk_NLSF_decode, silk_NLSF_del_dec_quant, silk_NLSF_unpack};
use crate::celt::celt::celt_fatal;
use crate::externs::memcpy;
use crate::silk::lin2log::silk_lin2log;
use crate::silk::sort::silk_insertion_sort_increasing;
use crate::silk::structs::silk_NLSF_CB_struct;
use crate::silk::Inlines::silk_DIV32_varQ;
use crate::silk::NLSF_stabilize::silk_NLSF_stabilize;

#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn silk_NLSF_encode(
    NLSFIndices: *mut i8,
    pNLSF_Q15: *mut i16,
    psNLSF_CB: *const silk_NLSF_CB_struct,
    pW_Q2: *const i16,
    NLSF_mu_Q20: libc::c_int,
    nSurvivors: libc::c_int,
    signalType: libc::c_int,
) -> i32 {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut ind1: libc::c_int = 0;
    let mut bestIndex: libc::c_int = 0;
    let mut prob_Q8: libc::c_int = 0;
    let mut bits_q7: libc::c_int = 0;
    let mut W_tmp_Q9: i32 = 0;
    let mut ret: i32 = 0;
    let mut res_Q10: [i16; 16] = [0; 16];
    let mut NLSF_tmp_Q15: [i16; 16] = [0; 16];
    let mut W_adj_Q5: [i16; 16] = [0; 16];
    let mut pred_Q8: [u8; 16] = [0; 16];
    let mut ec_ix: [i16; 16] = [0; 16];
    let mut pCB_element: *const u8 = 0 as *const u8;
    let mut iCDF_ptr: *const u8 = 0 as *const u8;
    let mut pCB_Wght_Q9: *const i16 = 0 as *const i16;
    if !(signalType >= 0 as libc::c_int && signalType <= 2 as libc::c_int) {
        celt_fatal(
            b"assertion failed: signalType >= 0 && signalType <= 2\0" as *const u8
                as *const libc::c_char,
            b"silk/NLSF_encode.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
        );
    }
    silk_NLSF_stabilize(
        pNLSF_Q15,
        (*psNLSF_CB).deltaMin_Q15,
        (*psNLSF_CB).order as libc::c_int,
    );
    let vla = (*psNLSF_CB).nVectors as usize;
    let mut err_Q24: Vec<i32> = ::std::vec::from_elem(0, vla);
    silk_NLSF_VQ(
        err_Q24.as_mut_ptr(),
        pNLSF_Q15 as *const i16,
        (*psNLSF_CB).CB1_NLSF_Q8,
        (*psNLSF_CB).CB1_Wght_Q9,
        (*psNLSF_CB).nVectors as libc::c_int,
        (*psNLSF_CB).order as libc::c_int,
    );
    let vla_0 = nSurvivors as usize;
    let mut tempIndices1: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    silk_insertion_sort_increasing(
        err_Q24.as_mut_ptr(),
        tempIndices1.as_mut_ptr(),
        (*psNLSF_CB).nVectors as libc::c_int,
        nSurvivors,
    );
    let vla_1 = nSurvivors as usize;
    let mut RD_Q25: Vec<i32> = ::std::vec::from_elem(0, vla_1);
    let vla_2 = (nSurvivors * 16 as libc::c_int) as usize;
    let mut tempIndices2: Vec<i8> = ::std::vec::from_elem(0, vla_2);
    s = 0 as libc::c_int;
    while s < nSurvivors {
        ind1 = *tempIndices1.as_mut_ptr().offset(s as isize);
        pCB_element = &*((*psNLSF_CB).CB1_NLSF_Q8)
            .offset((ind1 * (*psNLSF_CB).order as libc::c_int) as isize)
            as *const u8;
        pCB_Wght_Q9 = &*((*psNLSF_CB).CB1_Wght_Q9)
            .offset((ind1 * (*psNLSF_CB).order as libc::c_int) as isize)
            as *const i16;
        i = 0 as libc::c_int;
        while i < (*psNLSF_CB).order as libc::c_int {
            NLSF_tmp_Q15[i as usize] = ((*pCB_element.offset(i as isize) as i16 as u16
                as libc::c_int)
                << 7 as libc::c_int) as i16;
            W_tmp_Q9 = *pCB_Wght_Q9.offset(i as isize) as i32;
            res_Q10[i as usize] = ((*pNLSF_Q15.offset(i as isize) as libc::c_int
                - NLSF_tmp_Q15[i as usize] as libc::c_int) as i16
                as i32
                * W_tmp_Q9 as i16 as i32
                >> 14 as libc::c_int) as i16;
            W_adj_Q5[i as usize] = silk_DIV32_varQ(
                *pW_Q2.offset(i as isize) as i32,
                W_tmp_Q9 as i16 as i32 * W_tmp_Q9 as i16 as i32,
                21 as libc::c_int,
            ) as i16;
            i += 1;
        }
        silk_NLSF_unpack(ec_ix.as_mut_ptr(), pred_Q8.as_mut_ptr(), psNLSF_CB, ind1);
        *RD_Q25.as_mut_ptr().offset(s as isize) = silk_NLSF_del_dec_quant(
            &mut *tempIndices2
                .as_mut_ptr()
                .offset((s * MAX_LPC_ORDER) as isize),
            res_Q10.as_mut_ptr() as *const i16,
            W_adj_Q5.as_mut_ptr() as *const i16,
            pred_Q8.as_mut_ptr() as *const u8,
            ec_ix.as_mut_ptr() as *const i16,
            (*psNLSF_CB).ec_Rates_Q5,
            (*psNLSF_CB).quantStepSize_Q16 as libc::c_int,
            (*psNLSF_CB).invQuantStepSize_Q6,
            NLSF_mu_Q20,
            (*psNLSF_CB).order,
        );
        iCDF_ptr = &*((*psNLSF_CB).CB1_iCDF).offset(
            ((signalType >> 1 as libc::c_int) * (*psNLSF_CB).nVectors as libc::c_int) as isize,
        ) as *const u8;
        if ind1 == 0 as libc::c_int {
            prob_Q8 = 256 as libc::c_int - *iCDF_ptr.offset(ind1 as isize) as libc::c_int;
        } else {
            prob_Q8 = *iCDF_ptr.offset((ind1 - 1 as libc::c_int) as isize) as libc::c_int
                - *iCDF_ptr.offset(ind1 as isize) as libc::c_int;
        }
        bits_q7 = ((8 as libc::c_int) << 7 as libc::c_int) - silk_lin2log(prob_Q8);
        *RD_Q25.as_mut_ptr().offset(s as isize) = *RD_Q25.as_mut_ptr().offset(s as isize)
            + bits_q7 as i16 as i32 * (NLSF_mu_Q20 >> 2 as libc::c_int) as i16 as i32;
        s += 1;
    }
    silk_insertion_sort_increasing(
        RD_Q25.as_mut_ptr(),
        &mut bestIndex,
        nSurvivors,
        1 as libc::c_int,
    );
    *NLSFIndices.offset(0 as libc::c_int as isize) =
        *tempIndices1.as_mut_ptr().offset(bestIndex as isize) as i8;
    memcpy(
        &mut *NLSFIndices.offset(1 as libc::c_int as isize) as *mut i8 as *mut libc::c_void,
        &mut *tempIndices2
            .as_mut_ptr()
            .offset((bestIndex * 16 as libc::c_int) as isize) as *mut i8
            as *const libc::c_void,
        ((*psNLSF_CB).order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i8>() as libc::c_ulong),
    );
    silk_NLSF_decode(pNLSF_Q15, NLSFIndices, psNLSF_CB);
    ret = *RD_Q25.as_mut_ptr().offset(0 as libc::c_int as isize);
    return ret;
}
