use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:34"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:34"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:34"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:34"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:34"]
pub mod resampler_structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [opus_int32; 6],
        pub sFIR: C2RustUnnamed,
        pub delayBuf: [opus_int16; 48],
        pub resampler_function: libc::c_int,
        pub batchSize: libc::c_int,
        pub invRatio_Q16: opus_int32,
        pub FIR_Order: libc::c_int,
        pub FIR_Fracs: libc::c_int,
        pub Fs_in_kHz: libc::c_int,
        pub Fs_out_kHz: libc::c_int,
        pub inputDelay: libc::c_int,
        pub Coefs: *const opus_int16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:5"]
    pub union C2RustUnnamed {
        pub i32_0: [opus_int32; 36],
        pub i16_0: [opus_int16; 36],
    }
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
    use super::opus_types_h::{opus_int32, opus_int16};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:34"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:9"]
    pub struct silk_nsq_state {
        pub xq: [opus_int16; 640],
        pub sLTP_shp_Q14: [opus_int32; 640],
        pub sLPC_Q14: [opus_int32; 96],
        pub sAR2_Q14: [opus_int32; 24],
        pub sLF_AR_shp_Q14: opus_int32,
        pub sDiff_shp_Q14: opus_int32,
        pub lagPrev: libc::c_int,
        pub sLTP_buf_idx: libc::c_int,
        pub sLTP_shp_buf_idx: libc::c_int,
        pub rand_seed: opus_int32,
        pub prev_gain_Q16: opus_int32,
        pub rewhite_flag: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:9"]
    pub struct silk_VAD_state {
        pub AnaState: [opus_int32; 2],
        pub AnaState1: [opus_int32; 2],
        pub AnaState2: [opus_int32; 2],
        pub XnrgSubfr: [opus_int32; 4],
        pub NrgRatioSmth_Q8: [opus_int32; 4],
        pub HPstate: opus_int16,
        pub NL: [opus_int32; 4],
        pub inv_NL: [opus_int32; 4],
        pub NoiseLevelBias: [opus_int32; 4],
        pub counter: opus_int32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:9"]
    pub struct silk_LP_state {
        pub In_LP_State: [opus_int32; 2],
        pub transition_frame_no: opus_int32,
        pub mode: libc::c_int,
        pub saved_fs_kHz: opus_int32,
    }
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:9"]
    pub struct SideInfoIndices {
        pub GainsIndices: [opus_int8; 4],
        pub LTPIndex: [opus_int8; 4],
        pub NLSFIndices: [opus_int8; 17],
        pub lagIndex: opus_int16,
        pub contourIndex: opus_int8,
        pub signalType: opus_int8,
        pub quantOffsetType: opus_int8,
        pub NLSFInterpCoef_Q2: opus_int8,
        pub PERIndex: opus_int8,
        pub LTP_scaleIndex: opus_int8,
        pub Seed: opus_int8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:9"]
    pub struct silk_encoder_state {
        pub In_HP_State: [opus_int32; 2],
        pub variable_HP_smth1_Q15: opus_int32,
        pub variable_HP_smth2_Q15: opus_int32,
        pub sLP: silk_LP_state,
        pub sVAD: silk_VAD_state,
        pub sNSQ: silk_nsq_state,
        pub prev_NLSFq_Q15: [opus_int16; 16],
        pub speech_activity_Q8: libc::c_int,
        pub allow_bandwidth_switch: libc::c_int,
        pub LBRRprevLastGainIndex: opus_int8,
        pub prevSignalType: opus_int8,
        pub prevLag: libc::c_int,
        pub pitch_LPC_win_length: libc::c_int,
        pub max_pitch_lag: libc::c_int,
        pub API_fs_Hz: opus_int32,
        pub prev_API_fs_Hz: opus_int32,
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
        pub TargetRate_bps: opus_int32,
        pub PacketSize_ms: libc::c_int,
        pub PacketLoss_perc: libc::c_int,
        pub frameCounter: opus_int32,
        pub Complexity: libc::c_int,
        pub nStatesDelayedDecision: libc::c_int,
        pub useInterpolatedNLSFs: libc::c_int,
        pub shapingLPCOrder: libc::c_int,
        pub predictLPCOrder: libc::c_int,
        pub pitchEstimationComplexity: libc::c_int,
        pub pitchEstimationLPCOrder: libc::c_int,
        pub pitchEstimationThreshold_Q16: opus_int32,
        pub sum_log_gain_Q7: opus_int32,
        pub NLSF_MSVQ_Survivors: libc::c_int,
        pub first_frame_after_reset: libc::c_int,
        pub controlled_since_last_payload: libc::c_int,
        pub warping_Q16: libc::c_int,
        pub useCBR: libc::c_int,
        pub prefillFlag: libc::c_int,
        pub pitch_lag_low_bits_iCDF: *const opus_uint8,
        pub pitch_contour_iCDF: *const opus_uint8,
        pub psNLSF_CB: *const silk_NLSF_CB_struct,
        pub input_quality_bands_Q15: [libc::c_int; 4],
        pub input_tilt_Q15: libc::c_int,
        pub SNR_dB_Q7: libc::c_int,
        pub VAD_flags: [opus_int8; 3],
        pub LBRR_flag: opus_int8,
        pub LBRR_flags: [libc::c_int; 3],
        pub indices: SideInfoIndices,
        pub pulses: [opus_int8; 320],
        pub arch: libc::c_int,
        pub inputBuf: [opus_int16; 322],
        pub inputBufIx: libc::c_int,
        pub nFramesPerPacket: libc::c_int,
        pub nFramesEncoded: libc::c_int,
        pub nChannelsAPI: libc::c_int,
        pub nChannelsInternal: libc::c_int,
        pub channelNb: libc::c_int,
        pub frames_since_onset: libc::c_int,
        pub ec_prevSignalType: libc::c_int,
        pub ec_prevLagIndex: opus_int16,
        pub resampler_state: silk_resampler_state_struct,
        pub useDTX: libc::c_int,
        pub inDTX: libc::c_int,
        pub noSpeechCounter: libc::c_int,
        pub useInBandFEC: libc::c_int,
        pub LBRR_enabled: libc::c_int,
        pub LBRR_GainIncreases: libc::c_int,
        pub indices_LBRR: [SideInfoIndices; 3],
        pub pulses_LBRR: [[opus_int8; 320]; 3],
    }
    use super::opus_types_h::{opus_int16, opus_int32, opus_uint8, opus_int8};
    use super::resampler_structs_h::silk_resampler_state_struct;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/structs_FLP.h:34"]
pub mod structs_FLP_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:9"]
    pub struct silk_shape_state_FLP {
        pub LastGainIndex: opus_int8,
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
    use super::opus_types_h::opus_int8;
    use super::structs_h::silk_encoder_state;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:34"]
pub mod SigProc_FIX_h {
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "176:1"]
        pub fn silk_lin2log(inLin: opus_int32) -> opus_int32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:34"]
pub mod define_h {
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
}
pub use self::types_h::{
    __int8_t, __uint8_t, __int16_t, __int32_t, __uint32_t, __int64_t,
};
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::opus_types_h::{
    opus_int8, opus_uint8, opus_int16, opus_int32, opus_uint32, opus_int64,
};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, C2RustUnnamed, silk_resampler_state_struct,
};
pub use self::structs_h::{
    silk_nsq_state, silk_VAD_state, silk_LP_state, silk_NLSF_CB_struct, SideInfoIndices,
    silk_encoder_state,
};
pub use self::structs_FLP_h::{silk_shape_state_FLP, silk_encoder_state_FLP};
use self::SigProc_FIX_h::silk_lin2log;
pub use self::define_h::TYPE_VOICED;
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_HP_variable_cutoff(
    state_Fxx: *mut silk_encoder_state_FLP,
) {
    let mut quality_Q15: libc::c_int = 0;
    let mut pitch_freq_Hz_Q16: opus_int32 = 0;
    let mut pitch_freq_log_Q7: opus_int32 = 0;
    let mut delta_freq_Q7: opus_int32 = 0;
    let mut psEncC1: *mut silk_encoder_state = &mut (*state_Fxx
        .offset(0 as libc::c_int as isize))
        .sCmn;
    if (*psEncC1).prevSignalType as libc::c_int == TYPE_VOICED {
        pitch_freq_Hz_Q16 = ((((*psEncC1).fs_kHz * 1000 as libc::c_int) as opus_uint32)
            << 16 as libc::c_int) as opus_int32 / (*psEncC1).prevLag;
        pitch_freq_log_Q7 = silk_lin2log(pitch_freq_Hz_Q16)
            - ((16 as libc::c_int) << 7 as libc::c_int);
        quality_Q15 = (*psEncC1).input_quality_bands_Q15[0 as libc::c_int as usize];
        pitch_freq_log_Q7 = (pitch_freq_log_Q7 as libc::c_long
            + ((((-quality_Q15 as opus_uint32) << 2 as libc::c_int) as opus_int32
                as libc::c_long * quality_Q15 as opus_int16 as opus_int64
                >> 16 as libc::c_int) as opus_int32 as libc::c_long
                * (pitch_freq_log_Q7
                    - (silk_lin2log(
                        ((60 as libc::c_int as libc::c_long
                            * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int))
                            as libc::c_double + 0.5f64) as opus_int32,
                    ) - ((16 as libc::c_int) << 7 as libc::c_int))) as opus_int16
                    as opus_int64 >> 16 as libc::c_int)) as opus_int32;
        delta_freq_Q7 = pitch_freq_log_Q7
            - ((*psEncC1).variable_HP_smth1_Q15 >> 8 as libc::c_int);
        if delta_freq_Q7 < 0 as libc::c_int {
            delta_freq_Q7 = delta_freq_Q7 * 3 as libc::c_int;
        }
        delta_freq_Q7 = if -(((0.4f32
            * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int) as libc::c_float)
            as libc::c_double + 0.5f64) as opus_int32)
            > ((0.4f32
                * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int)
                    as libc::c_float) as libc::c_double + 0.5f64) as opus_int32
        {
            if delta_freq_Q7
                > -(((0.4f32
                    * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as opus_int32)
            {
                -(((0.4f32
                    * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as opus_int32)
            } else if delta_freq_Q7
                < ((0.4f32
                    * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as opus_int32
            {
                ((0.4f32
                    * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as opus_int32
            } else {
                delta_freq_Q7
            }
        } else if delta_freq_Q7
            > ((0.4f32
                * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int)
                    as libc::c_float) as libc::c_double + 0.5f64) as opus_int32
        {
            ((0.4f32
                * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int)
                    as libc::c_float) as libc::c_double + 0.5f64) as opus_int32
        } else if delta_freq_Q7
            < -(((0.4f32
                * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int)
                    as libc::c_float) as libc::c_double + 0.5f64) as opus_int32)
        {
            -(((0.4f32
                * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int)
                    as libc::c_float) as libc::c_double + 0.5f64) as opus_int32)
        } else {
            delta_freq_Q7
        };
        (*psEncC1)
            .variable_HP_smth1_Q15 = ((*psEncC1).variable_HP_smth1_Q15 as libc::c_long
            + (((*psEncC1).speech_activity_Q8 as opus_int16 as opus_int32
                * delta_freq_Q7 as opus_int16 as opus_int32) as libc::c_long
                * ((0.1f32
                    * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int)
                        as libc::c_float) as libc::c_double + 0.5f64) as opus_int32
                    as opus_int16 as opus_int64 >> 16 as libc::c_int)) as opus_int32;
        (*psEncC1)
            .variable_HP_smth1_Q15 = if ((silk_lin2log(60 as libc::c_int) as opus_uint32)
            << 8 as libc::c_int) as opus_int32
            > ((silk_lin2log(100 as libc::c_int) as opus_uint32) << 8 as libc::c_int)
                as opus_int32
        {
            if (*psEncC1).variable_HP_smth1_Q15
                > ((silk_lin2log(60 as libc::c_int) as opus_uint32) << 8 as libc::c_int)
                    as opus_int32
            {
                ((silk_lin2log(60 as libc::c_int) as opus_uint32) << 8 as libc::c_int)
                    as opus_int32
            } else if (*psEncC1).variable_HP_smth1_Q15
                < ((silk_lin2log(100 as libc::c_int) as opus_uint32) << 8 as libc::c_int)
                    as opus_int32
            {
                ((silk_lin2log(100 as libc::c_int) as opus_uint32) << 8 as libc::c_int)
                    as opus_int32
            } else {
                (*psEncC1).variable_HP_smth1_Q15
            }
        } else if (*psEncC1).variable_HP_smth1_Q15
            > ((silk_lin2log(100 as libc::c_int) as opus_uint32) << 8 as libc::c_int)
                as opus_int32
        {
            ((silk_lin2log(100 as libc::c_int) as opus_uint32) << 8 as libc::c_int)
                as opus_int32
        } else if (*psEncC1).variable_HP_smth1_Q15
            < ((silk_lin2log(60 as libc::c_int) as opus_uint32) << 8 as libc::c_int)
                as opus_int32
        {
            ((silk_lin2log(60 as libc::c_int) as opus_uint32) << 8 as libc::c_int)
                as opus_int32
        } else {
            (*psEncC1).variable_HP_smth1_Q15
        };
    }
}
