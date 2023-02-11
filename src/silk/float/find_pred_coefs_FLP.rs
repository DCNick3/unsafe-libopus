use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/structs_FLP.h:32"]
pub mod structs_FLP_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:9"]
    pub struct silk_shape_state_FLP {
        pub LastGainIndex: i8,
        pub HarmShapeGain_smth: libc::c_float,
        pub Tilt_smth: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:9"]
    pub struct silk_encoder_state_FLP {
        pub sCmn: silk_encoder_state,
        pub sShape: silk_shape_state_FLP,
        pub x_buf: [libc::c_float; 720],
        pub LTPCorr: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "64:9"]
    pub struct silk_encoder_control_FLP {
        pub Gains: [libc::c_float; 4],
        pub PredCoef: [[libc::c_float; 16]; 2],
        pub LTPCoef: [libc::c_float; 20],
        pub LTP_scale: libc::c_float,
        pub pitchL: [libc::c_int; 4],
        pub AR: [libc::c_float; 96],
        pub LF_MA_shp: [libc::c_float; 4],
        pub LF_AR_shp: [libc::c_float; 4],
        pub Tilt: [libc::c_float; 4],
        pub HarmShapeGain: [libc::c_float; 4],
        pub Lambda: libc::c_float,
        pub input_quality: libc::c_float,
        pub coding_quality: libc::c_float,
        pub predGain: libc::c_float,
        pub LTPredCodGain: libc::c_float,
        pub ResNrg: [libc::c_float; 4],
        pub GainsUnq_Q16: [i32; 4],
        pub lastGainIndexPrev: i8,
    }
    use crate::silk::structs::silk_encoder_state;
}

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:32"]
pub mod SigProc_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "119:1"]
        pub fn silk_scale_copy_vector_FLP(
            data_out: *mut libc::c_float,
            data_in: *const libc::c_float,
            gain: libc::c_float,
            dataSize: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/main_FLP.h:32"]
pub mod main_FLP_h {
    use super::structs_FLP_h::{silk_encoder_control_FLP, silk_encoder_state_FLP};
    use crate::silk::structs::silk_encoder_state;
    extern "C" {
        #[c2rust::src_loc = "109:1"]
        pub fn silk_LTP_scale_ctrl_FLP(
            psEnc: *mut silk_encoder_state_FLP,
            psEncCtrl: *mut silk_encoder_control_FLP,
            condCoding: libc::c_int,
        );
        #[c2rust::src_loc = "167:1"]
        pub fn silk_residual_energy_FLP(
            nrgs: *mut libc::c_float,
            x: *const libc::c_float,
            a: *mut [libc::c_float; 16],
            gains: *const libc::c_float,
            subfr_length: libc::c_int,
            nb_subfr: libc::c_int,
            LPC_order: libc::c_int,
        );
        #[c2rust::src_loc = "265:1"]
        pub fn silk_process_NLSFs_FLP(
            psEncC: *mut silk_encoder_state,
            PredCoef: *mut [libc::c_float; 16],
            NLSF_Q15: *mut i16,
            prev_NLSF_Q15: *const i16,
        );
        #[c2rust::src_loc = "137:1"]
        pub fn silk_find_LPC_FLP(
            psEncC: *mut silk_encoder_state,
            NLSF_Q15: *mut i16,
            x: *const libc::c_float,
            minInvGain: libc::c_float,
        );
        #[c2rust::src_loc = "154:1"]
        pub fn silk_LTP_analysis_filter_FLP(
            LTP_res: *mut libc::c_float,
            x: *const libc::c_float,
            B: *const libc::c_float,
            pitchL: *const libc::c_int,
            invGains: *const libc::c_float,
            subfr_length: libc::c_int,
            nb_subfr: libc::c_int,
            pre_length: libc::c_int,
        );
        #[c2rust::src_loc = "187:1"]
        pub fn silk_quant_LTP_gains_FLP(
            B: *mut libc::c_float,
            cbk_index: *mut i8,
            periodicity_index: *mut i8,
            sum_log_gain_Q7: *mut i32,
            pred_gain_dB: *mut libc::c_float,
            XX: *const libc::c_float,
            xX: *const libc::c_float,
            subfr_len: libc::c_int,
            nb_subfr: libc::c_int,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "145:1"]
        pub fn silk_find_LTP_FLP(
            XX: *mut libc::c_float,
            xX: *mut libc::c_float,
            r_ptr: *const libc::c_float,
            lag: *const libc::c_int,
            subfr_length: libc::c_int,
            nb_subfr: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "139:9"]
    pub const MAX_PREDICTION_POWER_GAIN: libc::c_float = 1e4f32;
    #[c2rust::src_loc = "140:9"]
    pub const MAX_PREDICTION_POWER_GAIN_AFTER_RESET: libc::c_float = 1e2f32;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
}
pub use self::define_h::{
    MAX_PREDICTION_POWER_GAIN, MAX_PREDICTION_POWER_GAIN_AFTER_RESET, TYPE_VOICED,
};
use self::main_FLP_h::{
    silk_LTP_analysis_filter_FLP, silk_LTP_scale_ctrl_FLP, silk_find_LPC_FLP, silk_find_LTP_FLP,
    silk_process_NLSFs_FLP, silk_quant_LTP_gains_FLP, silk_residual_energy_FLP,
};
use crate::celt::celt::celt_fatal;

pub use self::structs_FLP_h::{
    silk_encoder_control_FLP, silk_encoder_state_FLP, silk_shape_state_FLP,
};

use self::SigProc_FLP_h::silk_scale_copy_vector_FLP;
use crate::externs::{memcpy, memset};
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
