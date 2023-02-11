use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "79:34"]
        pub static silk_LTP_vq_ptrs_Q7: [*const i8; 3];
        #[c2rust::src_loc = "84:26"]
        pub static silk_LTPScales_table_Q14: [i16; 3];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use crate::silk::structs::silk_NLSF_CB_struct;
    extern "C" {
        #[c2rust::src_loc = "187:1"]
        pub fn silk_gains_dequant(
            gain_Q16: *mut i32,
            ind: *const i8,
            prev_ind: *mut i8,
            conditional: libc::c_int,
            nb_subfr: libc::c_int,
        );
        #[c2rust::src_loc = "383:1"]
        pub fn silk_NLSF_decode(
            pNLSF_Q15: *mut i16,
            NLSFIndices: *mut i8,
            psNLSF_CB: *const silk_NLSF_CB_struct,
        );
    }
}
use self::main_h::{silk_NLSF_decode, silk_gains_dequant};
use self::tables_h::{silk_LTPScales_table_Q14, silk_LTP_vq_ptrs_Q7};
use crate::externs::{memcpy, memset};
use crate::silk::bwexpander::silk_bwexpander;
use crate::silk::decode_pitch::silk_decode_pitch;
use crate::silk::define::{BWE_AFTER_LOSS_Q16, CODE_CONDITIONALLY, LTP_ORDER, TYPE_VOICED};
use crate::silk::structs::{silk_decoder_control, silk_decoder_state};
use crate::silk::NLSF2A::silk_NLSF2A;

#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_decode_parameters(
    mut psDec: *mut silk_decoder_state,
    mut psDecCtrl: *mut silk_decoder_control,
    condCoding: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut Ix: libc::c_int = 0;
    let mut pNLSF_Q15: [i16; 16] = [0; 16];
    let mut pNLSF0_Q15: [i16; 16] = [0; 16];
    let mut cbk_ptr_Q7: *const i8 = 0 as *const i8;
    silk_gains_dequant(
        ((*psDecCtrl).Gains_Q16).as_mut_ptr(),
        ((*psDec).indices.GainsIndices).as_mut_ptr() as *const i8,
        &mut (*psDec).LastGainIndex,
        (condCoding == CODE_CONDITIONALLY) as libc::c_int,
        (*psDec).nb_subfr,
    );
    silk_NLSF_decode(
        pNLSF_Q15.as_mut_ptr(),
        ((*psDec).indices.NLSFIndices).as_mut_ptr(),
        (*psDec).psNLSF_CB,
    );
    silk_NLSF2A(
        ((*psDecCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr(),
        pNLSF_Q15.as_mut_ptr(),
        (*psDec).LPC_order,
        (*psDec).arch,
    );
    if (*psDec).first_frame_after_reset == 1 as libc::c_int {
        (*psDec).indices.NLSFInterpCoef_Q2 = 4 as libc::c_int as i8;
    }
    if ((*psDec).indices.NLSFInterpCoef_Q2 as libc::c_int) < 4 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*psDec).LPC_order {
            pNLSF0_Q15[i as usize] = ((*psDec).prevNLSF_Q15[i as usize] as libc::c_int
                + ((*psDec).indices.NLSFInterpCoef_Q2 as libc::c_int
                    * (pNLSF_Q15[i as usize] as libc::c_int
                        - (*psDec).prevNLSF_Q15[i as usize] as libc::c_int)
                    >> 2 as libc::c_int)) as i16;
            i += 1;
        }
        silk_NLSF2A(
            ((*psDecCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr(),
            pNLSF0_Q15.as_mut_ptr(),
            (*psDec).LPC_order,
            (*psDec).arch,
        );
    } else {
        memcpy(
            ((*psDecCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr()
                as *mut libc::c_void,
            ((*psDecCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr()
                as *const libc::c_void,
            ((*psDec).LPC_order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
        );
    }
    memcpy(
        ((*psDec).prevNLSF_Q15).as_mut_ptr() as *mut libc::c_void,
        pNLSF_Q15.as_mut_ptr() as *const libc::c_void,
        ((*psDec).LPC_order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    if (*psDec).lossCnt != 0 {
        silk_bwexpander(
            ((*psDecCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr(),
            (*psDec).LPC_order,
            BWE_AFTER_LOSS_Q16,
        );
        silk_bwexpander(
            ((*psDecCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr(),
            (*psDec).LPC_order,
            BWE_AFTER_LOSS_Q16,
        );
    }
    if (*psDec).indices.signalType as libc::c_int == TYPE_VOICED {
        silk_decode_pitch(
            (*psDec).indices.lagIndex,
            (*psDec).indices.contourIndex,
            ((*psDecCtrl).pitchL).as_mut_ptr(),
            (*psDec).fs_kHz,
            (*psDec).nb_subfr,
        );
        cbk_ptr_Q7 = silk_LTP_vq_ptrs_Q7[(*psDec).indices.PERIndex as usize];
        k = 0 as libc::c_int;
        while k < (*psDec).nb_subfr {
            Ix = (*psDec).indices.LTPIndex[k as usize] as libc::c_int;
            i = 0 as libc::c_int;
            while i < LTP_ORDER {
                (*psDecCtrl).LTPCoef_Q14[(k * LTP_ORDER + i) as usize] =
                    ((*cbk_ptr_Q7.offset((Ix * 5 as libc::c_int + i) as isize) as u32)
                        << 7 as libc::c_int) as i32 as i16;
                i += 1;
            }
            k += 1;
        }
        Ix = (*psDec).indices.LTP_scaleIndex as libc::c_int;
        (*psDecCtrl).LTP_scale_Q14 = silk_LTPScales_table_Q14[Ix as usize] as libc::c_int;
    } else {
        memset(
            ((*psDecCtrl).pitchL).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ((*psDec).nb_subfr as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memset(
            ((*psDecCtrl).LTPCoef_Q14).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ((5 as libc::c_int * (*psDec).nb_subfr) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
        );
        (*psDec).indices.PERIndex = 0 as libc::c_int as i8;
        (*psDecCtrl).LTP_scale_Q14 = 0 as libc::c_int;
    };
}
