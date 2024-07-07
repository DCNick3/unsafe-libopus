use crate::externs::{memcpy, memset};
use crate::silk::define::{
    LTP_ORDER, MAX_LPC_ORDER, MAX_NB_SUBFR, MAX_PREDICTION_POWER_GAIN,
    MAX_PREDICTION_POWER_GAIN_AFTER_RESET, TYPE_VOICED,
};
use crate::silk::float::find_LPC_FLP::silk_find_LPC_FLP;
use crate::silk::float::find_LTP_FLP::silk_find_LTP_FLP;
use crate::silk::float::residual_energy_FLP::silk_residual_energy_FLP;
use crate::silk::float::scale_copy_vector_FLP::silk_scale_copy_vector_FLP;
use crate::silk::float::structs_FLP::{silk_encoder_control_FLP, silk_encoder_state_FLP};
use crate::silk::float::wrappers_FLP::{silk_process_NLSFs_FLP, silk_quant_LTP_gains_FLP};
use crate::silk::float::LTP_analysis_filter_FLP::silk_LTP_analysis_filter_FLP;
use crate::silk::float::LTP_scale_ctrl_FLP::silk_LTP_scale_ctrl_FLP;
use crate::silk::mathops::silk_exp2;
use crate::util::nalgebra::make_viewr_mut_generic;
use nalgebra::{Const, Dyn, VectorView};

pub unsafe fn silk_find_pred_coefs_FLP(
    psEnc: *mut silk_encoder_state_FLP,
    psEncCtrl: *mut silk_encoder_control_FLP,
    res_pitch: *const f32,
    x: *const f32,
    condCoding: i32,
) {
    let mut i: i32 = 0;
    let mut XXLTP: [f32; MAX_NB_SUBFR * LTP_ORDER * LTP_ORDER] = [0.; 100];
    let mut xXLTP: [f32; MAX_NB_SUBFR * LTP_ORDER] = [0.; 20];
    let mut invGains: [f32; MAX_NB_SUBFR] = [0.; 4];
    let mut NLSF_Q15: [i16; MAX_LPC_ORDER] = [0; 16];
    let mut x_ptr: *const f32 = 0 as *const f32;
    let mut x_pre_ptr: *mut f32 = 0 as *mut f32;
    let mut LPC_in_pre: [f32; MAX_NB_SUBFR * MAX_LPC_ORDER + 320] = [0.; 384];
    let mut minInvGain: f32 = 0.;
    i = 0;
    while i < (*psEnc).sCmn.nb_subfr as i32 {
        invGains[i as usize] = 1.0f32 / (*psEncCtrl).Gains[i as usize];
        i += 1;
    }
    if (*psEnc).sCmn.indices.signalType as i32 == TYPE_VOICED {
        assert!(
            (*psEnc).sCmn.ltp_mem_length as i32 - (*psEnc).sCmn.predictLPCOrder
                >= (*psEncCtrl).pitchL[0 as usize] + 5 / 2
        );
        let nb_subfr = (*psEnc).sCmn.nb_subfr as usize;
        let subfr_length = (*psEnc).sCmn.subfr_length as usize;

        const LTP_ORDER: usize = crate::silk::define::LTP_ORDER as usize;

        // nalgebra would only allow us to construct column-major matrices
        // we fix this with unsafe!
        let mut XXLTP_mat = make_viewr_mut_generic(
            &mut XXLTP,
            Dyn(nb_subfr * LTP_ORDER),
            Const::<{ LTP_ORDER }>,
        );

        let mut xXLTP_mat =
            make_viewr_mut_generic(&mut xXLTP, Dyn(nb_subfr), Const::<{ LTP_ORDER }>);

        let r_ptr = (*psEnc).sCmn.ltp_mem_length as usize;
        let res_pitch = std::slice::from_raw_parts(
            res_pitch.offset(-(r_ptr as isize)),
            subfr_length * nb_subfr + r_ptr + LTP_ORDER,
        );
        let lag = VectorView::<i32, Dyn>::from_slice(&(*psEncCtrl).pitchL[..nb_subfr], nb_subfr);

        silk_find_LTP_FLP(
            &mut XXLTP_mat,
            &mut xXLTP_mat,
            res_pitch,
            r_ptr,
            &lag,
            subfr_length,
        );
        silk_quant_LTP_gains_FLP(
            ((*psEncCtrl).LTPCoef).as_mut_ptr(),
            ((*psEnc).sCmn.indices.LTPIndex).as_mut_ptr(),
            &mut (*psEnc).sCmn.indices.PERIndex,
            &mut (*psEnc).sCmn.sum_log_gain_Q7,
            &mut (*psEncCtrl).LTPredCodGain,
            XXLTP.as_mut_ptr() as *const f32,
            xXLTP.as_mut_ptr() as *const f32,
            (*psEnc).sCmn.subfr_length as i32,
            (*psEnc).sCmn.nb_subfr as i32,
            (*psEnc).sCmn.arch,
        );
        silk_LTP_scale_ctrl_FLP(psEnc, psEncCtrl, condCoding);
        silk_LTP_analysis_filter_FLP(
            LPC_in_pre.as_mut_ptr(),
            x.offset(-((*psEnc).sCmn.predictLPCOrder as isize)),
            ((*psEncCtrl).LTPCoef).as_mut_ptr() as *const f32,
            ((*psEncCtrl).pitchL).as_mut_ptr() as *const i32,
            invGains.as_mut_ptr() as *const f32,
            (*psEnc).sCmn.subfr_length as i32,
            (*psEnc).sCmn.nb_subfr as i32,
            (*psEnc).sCmn.predictLPCOrder,
        );
    } else {
        x_ptr = x.offset(-((*psEnc).sCmn.predictLPCOrder as isize));
        x_pre_ptr = LPC_in_pre.as_mut_ptr();
        i = 0;
        while i < (*psEnc).sCmn.nb_subfr as i32 {
            silk_scale_copy_vector_FLP(
                x_pre_ptr,
                x_ptr,
                invGains[i as usize],
                (*psEnc).sCmn.subfr_length as i32 + (*psEnc).sCmn.predictLPCOrder as i32,
            );
            x_pre_ptr = x_pre_ptr.offset(
                ((*psEnc).sCmn.subfr_length as i32 + (*psEnc).sCmn.predictLPCOrder) as isize,
            );
            x_ptr = x_ptr.offset((*psEnc).sCmn.subfr_length as isize);
            i += 1;
        }
        memset(
            ((*psEncCtrl).LTPCoef).as_mut_ptr() as *mut core::ffi::c_void,
            0,
            (((*psEnc).sCmn.nb_subfr * 5) as u64)
                .wrapping_mul(::core::mem::size_of::<f32>() as u64),
        );
        (*psEncCtrl).LTPredCodGain = 0.0f32;
        (*psEnc).sCmn.sum_log_gain_Q7 = 0;
    }
    if (*psEnc).sCmn.first_frame_after_reset != 0 {
        minInvGain = 1.0f32 / MAX_PREDICTION_POWER_GAIN_AFTER_RESET;
    } else {
        minInvGain = silk_exp2((*psEncCtrl).LTPredCodGain / 3.0) / MAX_PREDICTION_POWER_GAIN;
        minInvGain /= 0.25f32 + 0.75f32 * (*psEncCtrl).coding_quality;
    }
    silk_find_LPC_FLP(
        &mut (*psEnc).sCmn,
        NLSF_Q15.as_mut_ptr(),
        LPC_in_pre.as_mut_ptr() as *const f32,
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
        LPC_in_pre.as_mut_ptr() as *const f32,
        ((*psEncCtrl).PredCoef).as_mut_ptr(),
        ((*psEncCtrl).Gains).as_mut_ptr() as *const f32,
        (*psEnc).sCmn.subfr_length as i32,
        (*psEnc).sCmn.nb_subfr as i32,
        (*psEnc).sCmn.predictLPCOrder,
    );
    memcpy(
        ((*psEnc).sCmn.prev_NLSFq_Q15).as_mut_ptr() as *mut core::ffi::c_void,
        NLSF_Q15.as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[i16; 16]>() as u64,
    );
}
