use ::libc;

use crate::celt::celt::celt_fatal;
use crate::externs::{memcpy, memset};
use crate::silk::define::{
    MAX_PREDICTION_POWER_GAIN, MAX_PREDICTION_POWER_GAIN_AFTER_RESET, TYPE_VOICED,
};
use crate::silk::float::find_LPC_FLP::silk_find_LPC_FLP;
use crate::silk::float::find_LTP_FLP::silk_find_LTP_FLP;
use crate::silk::float::residual_energy_FLP::silk_residual_energy_FLP;
use crate::silk::float::scale_copy_vector_FLP::silk_scale_copy_vector_FLP;
use crate::silk::float::structs_FLP::{silk_encoder_control_FLP, silk_encoder_state_FLP};
use crate::silk::float::wrappers_FLP::{silk_process_NLSFs_FLP, silk_quant_LTP_gains_FLP};
use crate::silk::float::LTP_analysis_filter_FLP::silk_LTP_analysis_filter_FLP;
use crate::silk::float::LTP_scale_ctrl_FLP::silk_LTP_scale_ctrl_FLP;

#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_find_pred_coefs_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut psEncCtrl: *mut silk_encoder_control_FLP,
    res_pitch: *const libc::c_float,
    x: *const libc::c_float,
    condCoding: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut XXLTP: [libc::c_float; 100] = [0.; 100];
    let mut xXLTP: [libc::c_float; 20] = [0.; 20];
    let mut invGains: [libc::c_float; 4] = [0.; 4];
    let mut NLSF_Q15: [i16; 16] = [0; 16];
    let mut x_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut x_pre_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut LPC_in_pre: [libc::c_float; 384] = [0.; 384];
    let mut minInvGain: libc::c_float = 0.;
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.nb_subfr {
        invGains[i as usize] = 1.0f32 / (*psEncCtrl).Gains[i as usize];
        i += 1;
    }
    if (*psEnc).sCmn.indices.signalType as libc::c_int == TYPE_VOICED {
        if !((*psEnc).sCmn.ltp_mem_length - (*psEnc).sCmn.predictLPCOrder
            >= (*psEncCtrl).pitchL[0 as libc::c_int as usize] + 5 as libc::c_int / 2 as libc::c_int)
        {
            celt_fatal(
                b"assertion failed: psEnc->sCmn.ltp_mem_length - psEnc->sCmn.predictLPCOrder >= psEncCtrl->pitchL[ 0 ] + LTP_ORDER / 2\0"
                    as *const u8 as *const libc::c_char,
                b"silk/float/find_pred_coefs_FLP.c\0" as *const u8
                    as *const libc::c_char,
                62 as libc::c_int,
            );
        }
        silk_find_LTP_FLP(
            XXLTP.as_mut_ptr(),
            xXLTP.as_mut_ptr(),
            res_pitch,
            ((*psEncCtrl).pitchL).as_mut_ptr() as *const libc::c_int,
            (*psEnc).sCmn.subfr_length,
            (*psEnc).sCmn.nb_subfr,
        );
        silk_quant_LTP_gains_FLP(
            ((*psEncCtrl).LTPCoef).as_mut_ptr(),
            ((*psEnc).sCmn.indices.LTPIndex).as_mut_ptr(),
            &mut (*psEnc).sCmn.indices.PERIndex,
            &mut (*psEnc).sCmn.sum_log_gain_Q7,
            &mut (*psEncCtrl).LTPredCodGain,
            XXLTP.as_mut_ptr() as *const libc::c_float,
            xXLTP.as_mut_ptr() as *const libc::c_float,
            (*psEnc).sCmn.subfr_length,
            (*psEnc).sCmn.nb_subfr,
            (*psEnc).sCmn.arch,
        );
        silk_LTP_scale_ctrl_FLP(psEnc, psEncCtrl, condCoding);
        silk_LTP_analysis_filter_FLP(
            LPC_in_pre.as_mut_ptr(),
            x.offset(-((*psEnc).sCmn.predictLPCOrder as isize)),
            ((*psEncCtrl).LTPCoef).as_mut_ptr() as *const libc::c_float,
            ((*psEncCtrl).pitchL).as_mut_ptr() as *const libc::c_int,
            invGains.as_mut_ptr() as *const libc::c_float,
            (*psEnc).sCmn.subfr_length,
            (*psEnc).sCmn.nb_subfr,
            (*psEnc).sCmn.predictLPCOrder,
        );
    } else {
        x_ptr = x.offset(-((*psEnc).sCmn.predictLPCOrder as isize));
        x_pre_ptr = LPC_in_pre.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < (*psEnc).sCmn.nb_subfr {
            silk_scale_copy_vector_FLP(
                x_pre_ptr,
                x_ptr,
                invGains[i as usize],
                (*psEnc).sCmn.subfr_length + (*psEnc).sCmn.predictLPCOrder,
            );
            x_pre_ptr = x_pre_ptr
                .offset(((*psEnc).sCmn.subfr_length + (*psEnc).sCmn.predictLPCOrder) as isize);
            x_ptr = x_ptr.offset((*psEnc).sCmn.subfr_length as isize);
            i += 1;
        }
        memset(
            ((*psEncCtrl).LTPCoef).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (((*psEnc).sCmn.nb_subfr * 5 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
        (*psEncCtrl).LTPredCodGain = 0.0f32;
        (*psEnc).sCmn.sum_log_gain_Q7 = 0 as libc::c_int;
    }
    if (*psEnc).sCmn.first_frame_after_reset != 0 {
        minInvGain = 1.0f32 / MAX_PREDICTION_POWER_GAIN_AFTER_RESET;
    } else {
        minInvGain = 2.0f32.powf((*psEncCtrl).LTPredCodGain / 3.0) / MAX_PREDICTION_POWER_GAIN;
        minInvGain /= 0.25f32 + 0.75f32 * (*psEncCtrl).coding_quality;
    }
    silk_find_LPC_FLP(
        &mut (*psEnc).sCmn,
        NLSF_Q15.as_mut_ptr(),
        LPC_in_pre.as_mut_ptr() as *const libc::c_float,
        minInvGain,
    );
    silk_process_NLSFs_FLP(
        &mut (*psEnc).sCmn,
        ((*psEncCtrl).PredCoef).as_mut_ptr(),
        NLSF_Q15.as_mut_ptr(),
        ((*psEnc).sCmn.prev_NLSFq_Q15).as_mut_ptr() as *const i16,
    );
    silk_residual_energy_FLP(
        ((*psEncCtrl).ResNrg).as_mut_ptr(),
        LPC_in_pre.as_mut_ptr() as *const libc::c_float,
        ((*psEncCtrl).PredCoef).as_mut_ptr(),
        ((*psEncCtrl).Gains).as_mut_ptr() as *const libc::c_float,
        (*psEnc).sCmn.subfr_length,
        (*psEnc).sCmn.nb_subfr,
        (*psEnc).sCmn.predictLPCOrder,
    );
    memcpy(
        ((*psEnc).sCmn.prev_NLSFq_Q15).as_mut_ptr() as *mut libc::c_void,
        NLSF_Q15.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[i16; 16]>() as libc::c_ulong,
    );
}
