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
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:33"]
pub mod resampler_structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [i32; 6],
        pub sFIR: C2RustUnnamed,
        pub delayBuf: [i16; 48],
        pub resampler_function: libc::c_int,
        pub batchSize: libc::c_int,
        pub invRatio_Q16: i32,
        pub FIR_Order: libc::c_int,
        pub FIR_Fracs: libc::c_int,
        pub Fs_in_kHz: libc::c_int,
        pub Fs_out_kHz: libc::c_int,
        pub inputDelay: libc::c_int,
        pub Coefs: *const i16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:5"]
    pub union C2RustUnnamed {
        pub i32_0: [i32; 36],
        pub i16_0: [i16; 36],
    }
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:33"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:9"]
    pub struct silk_nsq_state {
        pub xq: [i16; 640],
        pub sLTP_shp_Q14: [i32; 640],
        pub sLPC_Q14: [i32; 96],
        pub sAR2_Q14: [i32; 24],
        pub sLF_AR_shp_Q14: i32,
        pub sDiff_shp_Q14: i32,
        pub lagPrev: libc::c_int,
        pub sLTP_buf_idx: libc::c_int,
        pub sLTP_shp_buf_idx: libc::c_int,
        pub rand_seed: i32,
        pub prev_gain_Q16: i32,
        pub rewhite_flag: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:9"]
    pub struct silk_VAD_state {
        pub AnaState: [i32; 2],
        pub AnaState1: [i32; 2],
        pub AnaState2: [i32; 2],
        pub XnrgSubfr: [i32; 4],
        pub NrgRatioSmth_Q8: [i32; 4],
        pub HPstate: i16,
        pub NL: [i32; 4],
        pub inv_NL: [i32; 4],
        pub NoiseLevelBias: [i32; 4],
        pub counter: i32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:9"]
    pub struct silk_LP_state {
        pub In_LP_State: [i32; 2],
        pub transition_frame_no: i32,
        pub mode: libc::c_int,
        pub saved_fs_kHz: i32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:9"]
    pub struct silk_NLSF_CB_struct {
        pub nVectors: i16,
        pub order: i16,
        pub quantStepSize_Q16: i16,
        pub invQuantStepSize_Q6: i16,
        pub CB1_NLSF_Q8: *const u8,
        pub CB1_Wght_Q9: *const i16,
        pub CB1_iCDF: *const u8,
        pub pred_Q8: *const u8,
        pub ec_sel: *const u8,
        pub ec_iCDF: *const u8,
        pub ec_Rates_Q5: *const u8,
        pub deltaMin_Q15: *const i16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:9"]
    pub struct SideInfoIndices {
        pub GainsIndices: [i8; 4],
        pub LTPIndex: [i8; 4],
        pub NLSFIndices: [i8; 17],
        pub lagIndex: i16,
        pub contourIndex: i8,
        pub signalType: i8,
        pub quantOffsetType: i8,
        pub NLSFInterpCoef_Q2: i8,
        pub PERIndex: i8,
        pub LTP_scaleIndex: i8,
        pub Seed: i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:9"]
    pub struct silk_encoder_state {
        pub In_HP_State: [i32; 2],
        pub variable_HP_smth1_Q15: i32,
        pub variable_HP_smth2_Q15: i32,
        pub sLP: silk_LP_state,
        pub sVAD: silk_VAD_state,
        pub sNSQ: silk_nsq_state,
        pub prev_NLSFq_Q15: [i16; 16],
        pub speech_activity_Q8: libc::c_int,
        pub allow_bandwidth_switch: libc::c_int,
        pub LBRRprevLastGainIndex: i8,
        pub prevSignalType: i8,
        pub prevLag: libc::c_int,
        pub pitch_LPC_win_length: libc::c_int,
        pub max_pitch_lag: libc::c_int,
        pub API_fs_Hz: i32,
        pub prev_API_fs_Hz: i32,
        pub maxInternal_fs_Hz: libc::c_int,
        pub minInternal_fs_Hz: libc::c_int,
        pub desiredInternal_fs_Hz: libc::c_int,
        pub fs_kHz: libc::c_int,
        pub nb_subfr: libc::c_int,
        pub frame_length: libc::c_int,
        pub subfr_length: libc::c_int,
        pub ltp_mem_length: libc::c_int,
        pub la_pitch: libc::c_int,
        pub la_shape: libc::c_int,
        pub shapeWinLength: libc::c_int,
        pub TargetRate_bps: i32,
        pub PacketSize_ms: libc::c_int,
        pub PacketLoss_perc: libc::c_int,
        pub frameCounter: i32,
        pub Complexity: libc::c_int,
        pub nStatesDelayedDecision: libc::c_int,
        pub useInterpolatedNLSFs: libc::c_int,
        pub shapingLPCOrder: libc::c_int,
        pub predictLPCOrder: libc::c_int,
        pub pitchEstimationComplexity: libc::c_int,
        pub pitchEstimationLPCOrder: libc::c_int,
        pub pitchEstimationThreshold_Q16: i32,
        pub sum_log_gain_Q7: i32,
        pub NLSF_MSVQ_Survivors: libc::c_int,
        pub first_frame_after_reset: libc::c_int,
        pub controlled_since_last_payload: libc::c_int,
        pub warping_Q16: libc::c_int,
        pub useCBR: libc::c_int,
        pub prefillFlag: libc::c_int,
        pub pitch_lag_low_bits_iCDF: *const u8,
        pub pitch_contour_iCDF: *const u8,
        pub psNLSF_CB: *const silk_NLSF_CB_struct,
        pub input_quality_bands_Q15: [libc::c_int; 4],
        pub input_tilt_Q15: libc::c_int,
        pub SNR_dB_Q7: libc::c_int,
        pub VAD_flags: [i8; 3],
        pub LBRR_flag: i8,
        pub LBRR_flags: [libc::c_int; 3],
        pub indices: SideInfoIndices,
        pub pulses: [i8; 320],
        pub arch: libc::c_int,
        pub inputBuf: [i16; 322],
        pub inputBufIx: libc::c_int,
        pub nFramesPerPacket: libc::c_int,
        pub nFramesEncoded: libc::c_int,
        pub nChannelsAPI: libc::c_int,
        pub nChannelsInternal: libc::c_int,
        pub channelNb: libc::c_int,
        pub frames_since_onset: libc::c_int,
        pub ec_prevSignalType: libc::c_int,
        pub ec_prevLagIndex: i16,
        pub resampler_state: silk_resampler_state_struct,
        pub useDTX: libc::c_int,
        pub inDTX: libc::c_int,
        pub noSpeechCounter: libc::c_int,
        pub useInBandFEC: libc::c_int,
        pub LBRR_enabled: libc::c_int,
        pub LBRR_GainIncreases: libc::c_int,
        pub indices_LBRR: [SideInfoIndices; 3],
        pub pulses_LBRR: [[i8; 320]; 3],
    }
    use super::resampler_structs_h::silk_resampler_state_struct;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/structs_FLP.h:33"]
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
    use super::structs_h::silk_encoder_state;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/main_FLP.h:33"]
pub mod main_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "240:1"]
        pub fn silk_apply_sine_window_FLP(
            px_win: *mut libc::c_float,
            px: *const libc::c_float,
            win_type: libc::c_int,
            length: libc::c_int,
        );
        #[c2rust::src_loc = "178:1"]
        pub fn silk_LPC_analysis_filter_FLP(
            r_LPC: *mut libc::c_float,
            PredCoef: *const libc::c_float,
            s: *const libc::c_float,
            length: libc::c_int,
            Order: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:33"]
pub mod define_h {
    #[c2rust::src_loc = "70:9"]
    pub const TYPE_NO_VOICE_ACTIVITY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "71:9"]
    pub const TYPE_UNVOICED: libc::c_int = 1 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:33"]
pub mod SigProc_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "45:1"]
        pub fn silk_bwexpander_FLP(ar: *mut libc::c_float, d: libc::c_int, chirp: libc::c_float);
        #[c2rust::src_loc = "59:1"]
        pub fn silk_schur_FLP(
            refl_coef: *mut libc::c_float,
            auto_corr: *const libc::c_float,
            order: libc::c_int,
        ) -> libc::c_float;
        #[c2rust::src_loc = "65:1"]
        pub fn silk_k2a_FLP(A: *mut libc::c_float, rc: *const libc::c_float, order: i32);
        #[c2rust::src_loc = "72:1"]
        pub fn silk_autocorrelation_FLP(
            results: *mut libc::c_float,
            inputData: *const libc::c_float,
            inputDataSize: libc::c_int,
            correlationCount: libc::c_int,
        );
        #[c2rust::src_loc = "79:1"]
        pub fn silk_pitch_analysis_core_FLP(
            frame: *const libc::c_float,
            pitch_out: *mut libc::c_int,
            lagIndex: *mut i16,
            contourIndex: *mut i8,
            LTPCorr: *mut libc::c_float,
            prevLag: libc::c_int,
            search_thres1: libc::c_float,
            search_thres2: libc::c_float,
            Fs_kHz: libc::c_int,
            complexity: libc::c_int,
            nb_subfr: libc::c_int,
            arch: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tuning_parameters.h:34"]
pub mod tuning_parameters_h {
    #[c2rust::src_loc = "44:9"]
    pub const FIND_PITCH_WHITE_NOISE_FRACTION: libc::c_float = 1e-3f32;
    #[c2rust::src_loc = "47:9"]
    pub const FIND_PITCH_BANDWIDTH_EXPANSION: libc::c_float = 0.99f32;
}
use self::arch_h::celt_fatal;
pub use self::define_h::{TYPE_NO_VOICE_ACTIVITY, TYPE_UNVOICED, TYPE_VOICED};
use self::main_FLP_h::{silk_LPC_analysis_filter_FLP, silk_apply_sine_window_FLP};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::uint8_t;
pub use self::structs_FLP_h::{
    silk_encoder_control_FLP, silk_encoder_state_FLP, silk_shape_state_FLP,
};
pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    SideInfoIndices,
};
pub use self::tuning_parameters_h::{
    FIND_PITCH_BANDWIDTH_EXPANSION, FIND_PITCH_WHITE_NOISE_FRACTION,
};
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint8_t};
use self::SigProc_FLP_h::{
    silk_autocorrelation_FLP, silk_bwexpander_FLP, silk_k2a_FLP, silk_pitch_analysis_core_FLP,
    silk_schur_FLP,
};
use crate::externs::{memcpy, memset};
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_find_pitch_lags_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut psEncCtrl: *mut silk_encoder_control_FLP,
    res: *mut libc::c_float,
    x: *const libc::c_float,
    arch: libc::c_int,
) {
    let mut buf_len: libc::c_int = 0;
    let mut thrhld: libc::c_float = 0.;
    let mut res_nrg: libc::c_float = 0.;
    let mut x_buf_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut x_buf: *const libc::c_float = 0 as *const libc::c_float;
    let mut auto_corr: [libc::c_float; 17] = [0.; 17];
    let mut A: [libc::c_float; 16] = [0.; 16];
    let mut refl_coef: [libc::c_float; 16] = [0.; 16];
    let mut Wsig: [libc::c_float; 384] = [0.; 384];
    let mut Wsig_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    buf_len = (*psEnc).sCmn.la_pitch + (*psEnc).sCmn.frame_length + (*psEnc).sCmn.ltp_mem_length;
    if !(buf_len >= (*psEnc).sCmn.pitch_LPC_win_length) {
        celt_fatal(
            b"assertion failed: buf_len >= psEnc->sCmn.pitch_LPC_win_length\0" as *const u8
                as *const libc::c_char,
            b"silk/float/find_pitch_lags_FLP.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
        );
    }
    x_buf = x.offset(-((*psEnc).sCmn.ltp_mem_length as isize));
    x_buf_ptr = x_buf
        .offset(buf_len as isize)
        .offset(-((*psEnc).sCmn.pitch_LPC_win_length as isize));
    Wsig_ptr = Wsig.as_mut_ptr();
    silk_apply_sine_window_FLP(
        Wsig_ptr,
        x_buf_ptr,
        1 as libc::c_int,
        (*psEnc).sCmn.la_pitch,
    );
    Wsig_ptr = Wsig_ptr.offset((*psEnc).sCmn.la_pitch as isize);
    x_buf_ptr = x_buf_ptr.offset((*psEnc).sCmn.la_pitch as isize);
    memcpy(
        Wsig_ptr as *mut libc::c_void,
        x_buf_ptr as *const libc::c_void,
        (((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    Wsig_ptr = Wsig_ptr.offset(
        ((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as libc::c_int))
            as isize,
    );
    x_buf_ptr = x_buf_ptr.offset(
        ((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as libc::c_int))
            as isize,
    );
    silk_apply_sine_window_FLP(
        Wsig_ptr,
        x_buf_ptr,
        2 as libc::c_int,
        (*psEnc).sCmn.la_pitch,
    );
    silk_autocorrelation_FLP(
        auto_corr.as_mut_ptr(),
        Wsig.as_mut_ptr(),
        (*psEnc).sCmn.pitch_LPC_win_length,
        (*psEnc).sCmn.pitchEstimationLPCOrder + 1 as libc::c_int,
    );
    auto_corr[0 as libc::c_int as usize] += auto_corr[0 as libc::c_int as usize]
        * FIND_PITCH_WHITE_NOISE_FRACTION
        + 1 as libc::c_int as libc::c_float;
    res_nrg = silk_schur_FLP(
        refl_coef.as_mut_ptr(),
        auto_corr.as_mut_ptr() as *const libc::c_float,
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    (*psEncCtrl).predGain =
        auto_corr[0 as libc::c_int as usize] / (if res_nrg > 1.0f32 { res_nrg } else { 1.0f32 });
    silk_k2a_FLP(
        A.as_mut_ptr(),
        refl_coef.as_mut_ptr(),
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    silk_bwexpander_FLP(
        A.as_mut_ptr(),
        (*psEnc).sCmn.pitchEstimationLPCOrder,
        FIND_PITCH_BANDWIDTH_EXPANSION,
    );
    silk_LPC_analysis_filter_FLP(
        res,
        A.as_mut_ptr() as *const libc::c_float,
        x_buf,
        buf_len,
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    if (*psEnc).sCmn.indices.signalType as libc::c_int != TYPE_NO_VOICE_ACTIVITY
        && (*psEnc).sCmn.first_frame_after_reset == 0 as libc::c_int
    {
        thrhld = 0.6f32;
        thrhld -= 0.004f32 * (*psEnc).sCmn.pitchEstimationLPCOrder as libc::c_float;
        thrhld -= 0.1f32 * (*psEnc).sCmn.speech_activity_Q8 as libc::c_float * (1.0f32 / 256.0f32);
        thrhld -= 0.15f32
            * ((*psEnc).sCmn.prevSignalType as libc::c_int >> 1 as libc::c_int) as libc::c_float;
        thrhld -= 0.1f32 * (*psEnc).sCmn.input_tilt_Q15 as libc::c_float * (1.0f32 / 32768.0f32);
        if silk_pitch_analysis_core_FLP(
            res as *const libc::c_float,
            ((*psEncCtrl).pitchL).as_mut_ptr(),
            &mut (*psEnc).sCmn.indices.lagIndex,
            &mut (*psEnc).sCmn.indices.contourIndex,
            &mut (*psEnc).LTPCorr,
            (*psEnc).sCmn.prevLag,
            (*psEnc).sCmn.pitchEstimationThreshold_Q16 as libc::c_float / 65536.0f32,
            thrhld,
            (*psEnc).sCmn.fs_kHz,
            (*psEnc).sCmn.pitchEstimationComplexity,
            (*psEnc).sCmn.nb_subfr,
            arch,
        ) == 0 as libc::c_int
        {
            (*psEnc).sCmn.indices.signalType = TYPE_VOICED as i8;
        } else {
            (*psEnc).sCmn.indices.signalType = TYPE_UNVOICED as i8;
        }
    } else {
        memset(
            ((*psEncCtrl).pitchL).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong,
        );
        (*psEnc).sCmn.indices.lagIndex = 0 as libc::c_int as i16;
        (*psEnc).sCmn.indices.contourIndex = 0 as libc::c_int as i8;
        (*psEnc).LTPCorr = 0 as libc::c_int as libc::c_float;
    };
}
