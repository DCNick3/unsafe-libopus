use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
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
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    #[c2rust::src_loc = "52:4"]
    pub type opus_uint8 = uint8_t;
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "54:4"]
    pub type opus_uint16 = uint16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:32"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
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
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:32"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:32"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int =
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/macros.h:32"]
pub mod macros_h {
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn silk_CLZ32(in32: opus_int32) -> opus_int32 {
        return if in32 != 0 {
            32 as libc::c_int - (EC_CLZ0 - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }
    use super::ecintrin_h::EC_CLZ0;
    use super::opus_types_h::opus_int32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "176:1"]
        pub fn silk_lin2log(inLin: opus_int32) -> opus_int32;
        #[c2rust::src_loc = "302:1"]
        pub fn silk_insertion_sort_increasing(
            a: *mut opus_int32,
            idx: *mut libc::c_int,
            L: libc::c_int,
            K: libc::c_int,
        );
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
    use super::opus_types_h::{opus_int16, opus_int32, opus_int8, opus_uint8};
    use super::structs_h::silk_NLSF_CB_struct;
    extern "C" {
        #[c2rust::src_loc = "383:1"]
        pub fn silk_NLSF_decode(
            pNLSF_Q15: *mut opus_int16,
            NLSFIndices: *mut opus_int8,
            psNLSF_CB: *const silk_NLSF_CB_struct,
        );
        #[c2rust::src_loc = "359:1"]
        pub fn silk_NLSF_del_dec_quant(
            indices: *mut opus_int8,
            x_Q10: *const opus_int16,
            w_Q5: *const opus_int16,
            pred_coef_Q8: *const opus_uint8,
            ec_ix: *const opus_int16,
            ec_rates_Q5: *const opus_uint8,
            quant_step_size_Q16: libc::c_int,
            inv_quant_step_size_Q6: opus_int16,
            mu_Q20: opus_int32,
            order: opus_int16,
        ) -> opus_int32;
        #[c2rust::src_loc = "373:1"]
        pub fn silk_NLSF_unpack(
            ec_ix: *mut opus_int16,
            pred_Q8: *mut opus_uint8,
            psNLSF_CB: *const silk_NLSF_CB_struct,
            CB1_index: libc::c_int,
        );
        #[c2rust::src_loc = "349:1"]
        pub fn silk_NLSF_VQ(
            err_Q26: *mut opus_int32,
            in_Q15: *const opus_int16,
            pCB_Q8: *const opus_uint8,
            pWght_Q9: *const opus_int16,
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/Inlines.h:32"]
pub mod Inlines_h {
    #[inline]
    #[c2rust::src_loc = "97:1"]
    pub unsafe extern "C" fn silk_DIV32_varQ(
        a32: opus_int32,
        b32: opus_int32,
        Qres: libc::c_int,
    ) -> opus_int32 {
        let mut a_headrm: libc::c_int = 0;
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: opus_int32 = 0;
        let mut a32_nrm: opus_int32 = 0;
        let mut b32_nrm: opus_int32 = 0;
        let mut result: opus_int32 = 0;
        a_headrm = silk_CLZ32(if a32 > 0 as libc::c_int { a32 } else { -a32 }) - 1 as libc::c_int;
        a32_nrm = ((a32 as opus_uint32) << a_headrm) as opus_int32;
        b_headrm = silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 }) - 1 as libc::c_int;
        b32_nrm = ((b32 as opus_uint32) << b_headrm) as opus_int32;
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int) / (b32_nrm >> 16 as libc::c_int);
        result = (a32_nrm as libc::c_long * b32_inv as opus_int16 as opus_int64
            >> 16 as libc::c_int) as opus_int32;
        a32_nrm = (a32_nrm as opus_uint32).wrapping_sub(
            (((b32_nrm as opus_int64 * result as libc::c_long >> 32 as libc::c_int) as opus_int32
                as opus_uint32)
                << 3 as libc::c_int) as opus_int32 as opus_uint32,
        ) as opus_int32;
        result = (result as libc::c_long
            + (a32_nrm as libc::c_long * b32_inv as opus_int16 as opus_int64 >> 16 as libc::c_int))
            as opus_int32;
        lshift = 29 as libc::c_int + a_headrm - b_headrm - Qres;
        if lshift < 0 as libc::c_int {
            return (((if 0x80000000 as libc::c_uint as opus_int32 >> -lshift
                > 0x7fffffff as libc::c_int >> -lshift
            {
                if result > 0x80000000 as libc::c_uint as opus_int32 >> -lshift {
                    0x80000000 as libc::c_uint as opus_int32 >> -lshift
                } else {
                    if result < 0x7fffffff as libc::c_int >> -lshift {
                        0x7fffffff as libc::c_int >> -lshift
                    } else {
                        result
                    }
                }
            } else {
                if result > 0x7fffffff as libc::c_int >> -lshift {
                    0x7fffffff as libc::c_int >> -lshift
                } else {
                    if result < 0x80000000 as libc::c_uint as opus_int32 >> -lshift {
                        0x80000000 as libc::c_uint as opus_int32 >> -lshift
                    } else {
                        result
                    }
                }
            }) as opus_uint32)
                << -lshift) as opus_int32;
        } else if lshift < 32 as libc::c_int {
            return result >> lshift;
        } else {
            return 0 as libc::c_int;
        };
    }
    use super::macros_h::silk_CLZ32;
    use super::opus_types_h::{opus_int16, opus_int32, opus_int64, opus_uint32};
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
use self::arch_h::celt_fatal;
pub use self::define_h::MAX_LPC_ORDER;
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
pub use self::macros_h::silk_CLZ32;
use self::main_h::{silk_NLSF_VQ, silk_NLSF_decode, silk_NLSF_del_dec_quant, silk_NLSF_unpack};
pub use self::opus_types_h::{
    opus_int16, opus_int32, opus_int64, opus_int8, opus_uint16, opus_uint32, opus_uint8,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use self::string_h::memcpy;
pub use self::structs_h::silk_NLSF_CB_struct;
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::Inlines_h::silk_DIV32_varQ;
use self::SigProc_FIX_h::{silk_NLSF_stabilize, silk_insertion_sort_increasing, silk_lin2log};
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn silk_NLSF_encode(
    NLSFIndices: *mut opus_int8,
    pNLSF_Q15: *mut opus_int16,
    psNLSF_CB: *const silk_NLSF_CB_struct,
    pW_Q2: *const opus_int16,
    NLSF_mu_Q20: libc::c_int,
    nSurvivors: libc::c_int,
    signalType: libc::c_int,
) -> opus_int32 {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut ind1: libc::c_int = 0;
    let mut bestIndex: libc::c_int = 0;
    let mut prob_Q8: libc::c_int = 0;
    let mut bits_q7: libc::c_int = 0;
    let mut W_tmp_Q9: opus_int32 = 0;
    let mut ret: opus_int32 = 0;
    let mut res_Q10: [opus_int16; 16] = [0; 16];
    let mut NLSF_tmp_Q15: [opus_int16; 16] = [0; 16];
    let mut W_adj_Q5: [opus_int16; 16] = [0; 16];
    let mut pred_Q8: [opus_uint8; 16] = [0; 16];
    let mut ec_ix: [opus_int16; 16] = [0; 16];
    let mut pCB_element: *const opus_uint8 = 0 as *const opus_uint8;
    let mut iCDF_ptr: *const opus_uint8 = 0 as *const opus_uint8;
    let mut pCB_Wght_Q9: *const opus_int16 = 0 as *const opus_int16;
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
    let mut err_Q24: Vec<opus_int32> = ::std::vec::from_elem(0, vla);
    silk_NLSF_VQ(
        err_Q24.as_mut_ptr(),
        pNLSF_Q15 as *const opus_int16,
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
    let mut RD_Q25: Vec<opus_int32> = ::std::vec::from_elem(0, vla_1);
    let vla_2 = (nSurvivors * 16 as libc::c_int) as usize;
    let mut tempIndices2: Vec<opus_int8> = ::std::vec::from_elem(0, vla_2);
    s = 0 as libc::c_int;
    while s < nSurvivors {
        ind1 = *tempIndices1.as_mut_ptr().offset(s as isize);
        pCB_element = &*((*psNLSF_CB).CB1_NLSF_Q8)
            .offset((ind1 * (*psNLSF_CB).order as libc::c_int) as isize)
            as *const opus_uint8;
        pCB_Wght_Q9 = &*((*psNLSF_CB).CB1_Wght_Q9)
            .offset((ind1 * (*psNLSF_CB).order as libc::c_int) as isize)
            as *const opus_int16;
        i = 0 as libc::c_int;
        while i < (*psNLSF_CB).order as libc::c_int {
            NLSF_tmp_Q15[i as usize] = ((*pCB_element.offset(i as isize) as opus_int16
                as opus_uint16 as libc::c_int)
                << 7 as libc::c_int) as opus_int16;
            W_tmp_Q9 = *pCB_Wght_Q9.offset(i as isize) as opus_int32;
            res_Q10[i as usize] = ((*pNLSF_Q15.offset(i as isize) as libc::c_int
                - NLSF_tmp_Q15[i as usize] as libc::c_int)
                as opus_int16 as opus_int32
                * W_tmp_Q9 as opus_int16 as opus_int32
                >> 14 as libc::c_int) as opus_int16;
            W_adj_Q5[i as usize] = silk_DIV32_varQ(
                *pW_Q2.offset(i as isize) as opus_int32,
                W_tmp_Q9 as opus_int16 as opus_int32 * W_tmp_Q9 as opus_int16 as opus_int32,
                21 as libc::c_int,
            ) as opus_int16;
            i += 1;
        }
        silk_NLSF_unpack(ec_ix.as_mut_ptr(), pred_Q8.as_mut_ptr(), psNLSF_CB, ind1);
        *RD_Q25.as_mut_ptr().offset(s as isize) = silk_NLSF_del_dec_quant(
            &mut *tempIndices2
                .as_mut_ptr()
                .offset((s * MAX_LPC_ORDER) as isize),
            res_Q10.as_mut_ptr() as *const opus_int16,
            W_adj_Q5.as_mut_ptr() as *const opus_int16,
            pred_Q8.as_mut_ptr() as *const opus_uint8,
            ec_ix.as_mut_ptr() as *const opus_int16,
            (*psNLSF_CB).ec_Rates_Q5,
            (*psNLSF_CB).quantStepSize_Q16 as libc::c_int,
            (*psNLSF_CB).invQuantStepSize_Q6,
            NLSF_mu_Q20,
            (*psNLSF_CB).order,
        );
        iCDF_ptr = &*((*psNLSF_CB).CB1_iCDF).offset(
            ((signalType >> 1 as libc::c_int) * (*psNLSF_CB).nVectors as libc::c_int) as isize,
        ) as *const opus_uint8;
        if ind1 == 0 as libc::c_int {
            prob_Q8 = 256 as libc::c_int - *iCDF_ptr.offset(ind1 as isize) as libc::c_int;
        } else {
            prob_Q8 = *iCDF_ptr.offset((ind1 - 1 as libc::c_int) as isize) as libc::c_int
                - *iCDF_ptr.offset(ind1 as isize) as libc::c_int;
        }
        bits_q7 = ((8 as libc::c_int) << 7 as libc::c_int) - silk_lin2log(prob_Q8);
        *RD_Q25.as_mut_ptr().offset(s as isize) = *RD_Q25.as_mut_ptr().offset(s as isize)
            + bits_q7 as opus_int16 as opus_int32
                * (NLSF_mu_Q20 >> 2 as libc::c_int) as opus_int16 as opus_int32;
        s += 1;
    }
    silk_insertion_sort_increasing(
        RD_Q25.as_mut_ptr(),
        &mut bestIndex,
        nSurvivors,
        1 as libc::c_int,
    );
    *NLSFIndices.offset(0 as libc::c_int as isize) =
        *tempIndices1.as_mut_ptr().offset(bestIndex as isize) as opus_int8;
    memcpy(
        &mut *NLSFIndices.offset(1 as libc::c_int as isize) as *mut opus_int8 as *mut libc::c_void,
        &mut *tempIndices2
            .as_mut_ptr()
            .offset((bestIndex * 16 as libc::c_int) as isize) as *mut opus_int8
            as *const libc::c_void,
        ((*psNLSF_CB).order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int8>() as libc::c_ulong),
    );
    silk_NLSF_decode(pNLSF_Q15, NLSFIndices, psNLSF_CB);
    ret = *RD_Q25.as_mut_ptr().offset(0 as libc::c_int as isize);
    return ret;
}
