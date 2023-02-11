use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:34"]
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
    use crate::silk::resampler_structs::silk_resampler_state_struct;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/structs_FLP.h:34"]
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
    use super::structs_h::silk_encoder_state;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:34"]
pub mod SigProc_FIX_h {
    extern "C" {
        #[c2rust::src_loc = "176:1"]
        pub fn silk_lin2log(inLin: i32) -> i32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:34"]
pub mod define_h {
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
}
pub use self::define_h::TYPE_VOICED;

pub use self::structs_FLP_h::{silk_encoder_state_FLP, silk_shape_state_FLP};
pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    SideInfoIndices,
};

use self::SigProc_FIX_h::silk_lin2log;
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_HP_variable_cutoff(state_Fxx: *mut silk_encoder_state_FLP) {
    let mut quality_Q15: libc::c_int = 0;
    let mut pitch_freq_Hz_Q16: i32 = 0;
    let mut pitch_freq_log_Q7: i32 = 0;
    let mut delta_freq_Q7: i32 = 0;
    let mut psEncC1: *mut silk_encoder_state =
        &mut (*state_Fxx.offset(0 as libc::c_int as isize)).sCmn;
    if (*psEncC1).prevSignalType as libc::c_int == TYPE_VOICED {
        pitch_freq_Hz_Q16 = ((((*psEncC1).fs_kHz * 1000 as libc::c_int) as u32)
            << 16 as libc::c_int) as i32
            / (*psEncC1).prevLag;
        pitch_freq_log_Q7 =
            silk_lin2log(pitch_freq_Hz_Q16) - ((16 as libc::c_int) << 7 as libc::c_int);
        quality_Q15 = (*psEncC1).input_quality_bands_Q15[0 as libc::c_int as usize];
        pitch_freq_log_Q7 = (pitch_freq_log_Q7 as libc::c_long
            + ((((-quality_Q15 as u32) << 2 as libc::c_int) as i32 as libc::c_long
                * quality_Q15 as i16 as i64
                >> 16 as libc::c_int) as i32 as libc::c_long
                * (pitch_freq_log_Q7
                    - (silk_lin2log(
                        ((60 as libc::c_int as libc::c_long
                            * ((1 as libc::c_int as i64) << 16 as libc::c_int))
                            as libc::c_double
                            + 0.5f64) as i32,
                    ) - ((16 as libc::c_int) << 7 as libc::c_int))) as i16
                    as i64
                >> 16 as libc::c_int)) as i32;
        delta_freq_Q7 = pitch_freq_log_Q7 - ((*psEncC1).variable_HP_smth1_Q15 >> 8 as libc::c_int);
        if delta_freq_Q7 < 0 as libc::c_int {
            delta_freq_Q7 = delta_freq_Q7 * 3 as libc::c_int;
        }
        delta_freq_Q7 = if -(((0.4f32
            * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
            as libc::c_double
            + 0.5f64) as i32)
            > ((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32
        {
            if delta_freq_Q7
                > -(((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as i32)
            {
                -(((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as i32)
            } else if delta_freq_Q7
                < ((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as i32
            {
                ((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as i32
            } else {
                delta_freq_Q7
            }
        } else if delta_freq_Q7
            > ((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32
        {
            ((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32
        } else if delta_freq_Q7
            < -(((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32)
        {
            -(((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32)
        } else {
            delta_freq_Q7
        };
        (*psEncC1).variable_HP_smth1_Q15 = ((*psEncC1).variable_HP_smth1_Q15 as libc::c_long
            + (((*psEncC1).speech_activity_Q8 as i16 as i32 * delta_freq_Q7 as i16 as i32)
                as libc::c_long
                * ((0.1f32 * ((1 as libc::c_int as i64) << 16 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as i32 as i16 as i64
                >> 16 as libc::c_int)) as i32;
        (*psEncC1).variable_HP_smth1_Q15 = if ((silk_lin2log(60 as libc::c_int) as u32)
            << 8 as libc::c_int) as i32
            > ((silk_lin2log(100 as libc::c_int) as u32) << 8 as libc::c_int) as i32
        {
            if (*psEncC1).variable_HP_smth1_Q15
                > ((silk_lin2log(60 as libc::c_int) as u32) << 8 as libc::c_int) as i32
            {
                ((silk_lin2log(60 as libc::c_int) as u32) << 8 as libc::c_int) as i32
            } else if (*psEncC1).variable_HP_smth1_Q15
                < ((silk_lin2log(100 as libc::c_int) as u32) << 8 as libc::c_int) as i32
            {
                ((silk_lin2log(100 as libc::c_int) as u32) << 8 as libc::c_int) as i32
            } else {
                (*psEncC1).variable_HP_smth1_Q15
            }
        } else if (*psEncC1).variable_HP_smth1_Q15
            > ((silk_lin2log(100 as libc::c_int) as u32) << 8 as libc::c_int) as i32
        {
            ((silk_lin2log(100 as libc::c_int) as u32) << 8 as libc::c_int) as i32
        } else if (*psEncC1).variable_HP_smth1_Q15
            < ((silk_lin2log(60 as libc::c_int) as u32) << 8 as libc::c_int) as i32
        {
            ((silk_lin2log(60 as libc::c_int) as u32) << 8 as libc::c_int) as i32
        } else {
            (*psEncC1).variable_HP_smth1_Q15
        };
    }
}
