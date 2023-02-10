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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:32"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
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
    use super::structs_h::silk_encoder_state;
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:32"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "107:17"]
        pub fn log10(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "140:17"]
        pub fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/main_FLP.h:32"]
pub mod main_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "240:1"]
        pub fn silk_apply_sine_window_FLP(
            px_win: *mut libc::c_float,
            px: *const libc::c_float,
            win_type: libc::c_int,
            length: libc::c_int,
        );
        #[c2rust::src_loc = "100:1"]
        pub fn silk_warped_autocorrelation_FLP(
            corr: *mut libc::c_float,
            input: *const libc::c_float,
            warping: libc::c_float,
            length: libc::c_int,
            order: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "155:9"]
    pub const MAX_SHAPE_LPC_ORDER: libc::c_int = 24 as libc::c_int;
    #[c2rust::src_loc = "119:9"]
    pub const MIN_QGAIN_DB: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "152:9"]
    pub const USE_HARM_SHAPING: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:32"]
pub mod SigProc_FLP_h {
    #[inline]
    #[c2rust::src_loc = "150:1"]
    pub unsafe extern "C" fn silk_sigmoid(x: libc::c_float) -> libc::c_float {
        return (1.0f64 / (1.0f64 + (-x as f64).exp())) as libc::c_float;
    }
    #[inline]
    #[c2rust::src_loc = "188:1"]
    pub unsafe extern "C" fn silk_log2(x: libc::c_double) -> libc::c_float {
        return (3.32192809488736f64 * log10(x)) as libc::c_float;
    }
    use super::mathcalls_h::log10;
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
        #[c2rust::src_loc = "134:1"]
        pub fn silk_energy_FLP(data: *const libc::c_float, dataSize: libc::c_int)
            -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tuning_parameters.h:33"]
pub mod tuning_parameters_h {
    #[c2rust::src_loc = "44:9"]
    pub const FIND_PITCH_WHITE_NOISE_FRACTION: libc::c_float = 1e-3f32;
    #[c2rust::src_loc = "90:9"]
    pub const BG_SNR_DECR_dB: libc::c_float = 2.0f32;
    #[c2rust::src_loc = "93:9"]
    pub const HARM_SNR_INCR_dB: libc::c_float = 2.0f32;
    #[c2rust::src_loc = "99:9"]
    pub const ENERGY_VARIATION_THRESHOLD_QNT_OFFSET: libc::c_float = 0.6f32;
    #[c2rust::src_loc = "105:9"]
    pub const SHAPE_WHITE_NOISE_FRACTION: libc::c_float = 3e-5f32;
    #[c2rust::src_loc = "108:9"]
    pub const BANDWIDTH_EXPANSION: libc::c_float = 0.94f32;
    #[c2rust::src_loc = "111:9"]
    pub const HARMONIC_SHAPING: libc::c_float = 0.3f32;
    #[c2rust::src_loc = "114:9"]
    pub const HIGH_RATE_OR_LOW_QUALITY_HARMONIC_SHAPING: libc::c_float = 0.2f32;
    #[c2rust::src_loc = "117:9"]
    pub const HP_NOISE_COEF: libc::c_float = 0.25f32;
    #[c2rust::src_loc = "120:9"]
    pub const HARM_HP_NOISE_COEF: libc::c_float = 0.35f32;
    #[c2rust::src_loc = "129:9"]
    pub const LOW_FREQ_SHAPING: libc::c_float = 4.0f32;
    #[c2rust::src_loc = "132:9"]
    pub const LOW_QUALITY_LOW_FREQ_SHAPING_DECR: libc::c_float = 0.5f32;
    #[c2rust::src_loc = "135:9"]
    pub const SUBFR_SMTH_COEF: libc::c_float = 0.4f32;
}
pub use self::define_h::{MAX_SHAPE_LPC_ORDER, MIN_QGAIN_DB, TYPE_VOICED, USE_HARM_SHAPING};
use self::main_FLP_h::{silk_apply_sine_window_FLP, silk_warped_autocorrelation_FLP};
use self::mathcalls_h::pow;
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::uint8_t;
use self::string_h::memcpy;
pub use self::structs_FLP_h::{
    silk_encoder_control_FLP, silk_encoder_state_FLP, silk_shape_state_FLP,
};
pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    SideInfoIndices,
};
pub use self::tuning_parameters_h::{
    BG_SNR_DECR_dB, HARM_SNR_INCR_dB, BANDWIDTH_EXPANSION, ENERGY_VARIATION_THRESHOLD_QNT_OFFSET,
    FIND_PITCH_WHITE_NOISE_FRACTION, HARMONIC_SHAPING, HARM_HP_NOISE_COEF,
    HIGH_RATE_OR_LOW_QUALITY_HARMONIC_SHAPING, HP_NOISE_COEF, LOW_FREQ_SHAPING,
    LOW_QUALITY_LOW_FREQ_SHAPING_DECR, SHAPE_WHITE_NOISE_FRACTION, SUBFR_SMTH_COEF,
};
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint8_t};
pub use self::SigProc_FLP_h::{
    silk_autocorrelation_FLP, silk_bwexpander_FLP, silk_energy_FLP, silk_k2a_FLP, silk_log2,
    silk_schur_FLP, silk_sigmoid,
};
#[inline]
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn warped_gain(
    coefs: *const libc::c_float,
    mut lambda: libc::c_float,
    order: libc::c_int,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut gain: libc::c_float = 0.;
    lambda = -lambda;
    gain = *coefs.offset((order - 1 as libc::c_int) as isize);
    i = order - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        gain = lambda * gain + *coefs.offset(i as isize);
        i -= 1;
    }
    return 1.0f32 / (1.0f32 - lambda * gain);
}
#[inline]
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn warped_true2monic_coefs(
    coefs: *mut libc::c_float,
    lambda: libc::c_float,
    limit: libc::c_float,
    order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut ind: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_float = 0.;
    let mut maxabs: libc::c_float = 0.;
    let mut chirp: libc::c_float = 0.;
    let mut gain: libc::c_float = 0.;
    i = order - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        *coefs.offset((i - 1 as libc::c_int) as isize) -= lambda * *coefs.offset(i as isize);
        i -= 1;
    }
    gain =
        (1.0f32 - lambda * lambda) / (1.0f32 + lambda * *coefs.offset(0 as libc::c_int as isize));
    i = 0 as libc::c_int;
    while i < order {
        *coefs.offset(i as isize) *= gain;
        i += 1;
    }
    iter = 0 as libc::c_int;
    while iter < 10 as libc::c_int {
        maxabs = -1.0f32;
        i = 0 as libc::c_int;
        while i < order {
            tmp = (*coefs.offset(i as isize)).abs();
            if tmp > maxabs {
                maxabs = tmp;
                ind = i;
            }
            i += 1;
        }
        if maxabs <= limit {
            return;
        }
        i = 1 as libc::c_int;
        while i < order {
            *coefs.offset((i - 1 as libc::c_int) as isize) += lambda * *coefs.offset(i as isize);
            i += 1;
        }
        gain = 1.0f32 / gain;
        i = 0 as libc::c_int;
        while i < order {
            *coefs.offset(i as isize) *= gain;
            i += 1;
        }
        chirp = 0.99f32
            - (0.8f32 + 0.1f32 * iter as libc::c_float) * (maxabs - limit)
                / (maxabs * (ind + 1 as libc::c_int) as libc::c_float);
        silk_bwexpander_FLP(coefs, order, chirp);
        i = order - 1 as libc::c_int;
        while i > 0 as libc::c_int {
            *coefs.offset((i - 1 as libc::c_int) as isize) -= lambda * *coefs.offset(i as isize);
            i -= 1;
        }
        gain = (1.0f32 - lambda * lambda)
            / (1.0f32 + lambda * *coefs.offset(0 as libc::c_int as isize));
        i = 0 as libc::c_int;
        while i < order {
            *coefs.offset(i as isize) *= gain;
            i += 1;
        }
        iter += 1;
    }
}
#[inline]
#[c2rust::src_loc = "116:1"]
unsafe extern "C" fn limit_coefs(
    coefs: *mut libc::c_float,
    limit: libc::c_float,
    order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut ind: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_float = 0.;
    let mut maxabs: libc::c_float = 0.;
    let mut chirp: libc::c_float = 0.;
    iter = 0 as libc::c_int;
    while iter < 10 as libc::c_int {
        maxabs = -1.0f32;
        i = 0 as libc::c_int;
        while i < order {
            tmp = (*coefs.offset(i as isize)).abs();
            if tmp > maxabs {
                maxabs = tmp;
                ind = i;
            }
            i += 1;
        }
        if maxabs <= limit {
            return;
        }
        chirp = 0.99f32
            - (0.8f32 + 0.1f32 * iter as libc::c_float) * (maxabs - limit)
                / (maxabs * (ind + 1 as libc::c_int) as libc::c_float);
        silk_bwexpander_FLP(coefs, order, chirp);
        iter += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "147:1"]
pub unsafe extern "C" fn silk_noise_shape_analysis_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut psEncCtrl: *mut silk_encoder_control_FLP,
    pitch_res: *const libc::c_float,
    x: *const libc::c_float,
) {
    let mut psShapeSt: *mut silk_shape_state_FLP = &mut (*psEnc).sShape;
    let mut k: libc::c_int = 0;
    let mut nSamples: libc::c_int = 0;
    let mut nSegs: libc::c_int = 0;
    let mut SNR_adj_dB: libc::c_float = 0.;
    let mut HarmShapeGain: libc::c_float = 0.;
    let mut Tilt: libc::c_float = 0.;
    let mut nrg: libc::c_float = 0.;
    let mut log_energy: libc::c_float = 0.;
    let mut log_energy_prev: libc::c_float = 0.;
    let mut energy_variation: libc::c_float = 0.;
    let mut BWExp: libc::c_float = 0.;
    let mut gain_mult: libc::c_float = 0.;
    let mut gain_add: libc::c_float = 0.;
    let mut strength: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    let mut warping: libc::c_float = 0.;
    let mut x_windowed: [libc::c_float; 240] = [0.; 240];
    let mut auto_corr: [libc::c_float; 25] = [0.; 25];
    let mut rc: [libc::c_float; 25] = [0.; 25];
    let mut x_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut pitch_res_ptr: *const libc::c_float = 0 as *const libc::c_float;
    x_ptr = x.offset(-((*psEnc).sCmn.la_shape as isize));
    SNR_adj_dB =
        (*psEnc).sCmn.SNR_dB_Q7 as libc::c_float * (1 as libc::c_int as libc::c_float / 128.0f32);
    (*psEncCtrl).input_quality = 0.5f32
        * ((*psEnc).sCmn.input_quality_bands_Q15[0 as libc::c_int as usize]
            + (*psEnc).sCmn.input_quality_bands_Q15[1 as libc::c_int as usize])
            as libc::c_float
        * (1.0f32 / 32768.0f32);
    (*psEncCtrl).coding_quality = silk_sigmoid(0.25f32 * (SNR_adj_dB - 20.0f32));
    if (*psEnc).sCmn.useCBR == 0 as libc::c_int {
        b = 1.0f32 - (*psEnc).sCmn.speech_activity_Q8 as libc::c_float * (1.0f32 / 256.0f32);
        SNR_adj_dB -= BG_SNR_DECR_dB
            * (*psEncCtrl).coding_quality
            * (0.5f32 + 0.5f32 * (*psEncCtrl).input_quality)
            * b
            * b;
    }
    if (*psEnc).sCmn.indices.signalType as libc::c_int == TYPE_VOICED {
        SNR_adj_dB += HARM_SNR_INCR_dB * (*psEnc).LTPCorr;
    } else {
        SNR_adj_dB += (-0.4f32
            * (*psEnc).sCmn.SNR_dB_Q7 as libc::c_float
            * (1 as libc::c_int as libc::c_float / 128.0f32)
            + 6.0f32)
            * (1.0f32 - (*psEncCtrl).input_quality);
    }
    if (*psEnc).sCmn.indices.signalType as libc::c_int == TYPE_VOICED {
        (*psEnc).sCmn.indices.quantOffsetType = 0 as libc::c_int as i8;
    } else {
        nSamples = 2 as libc::c_int * (*psEnc).sCmn.fs_kHz;
        energy_variation = 0.0f32;
        log_energy_prev = 0.0f32;
        pitch_res_ptr = pitch_res;
        nSegs = 5 as libc::c_int as i16 as i32 * (*psEnc).sCmn.nb_subfr as i16 as i32
            / 2 as libc::c_int;
        k = 0 as libc::c_int;
        while k < nSegs {
            nrg = nSamples as libc::c_float
                + silk_energy_FLP(pitch_res_ptr, nSamples) as libc::c_float;
            log_energy = silk_log2(nrg as libc::c_double);
            if k > 0 as libc::c_int {
                energy_variation += (log_energy - log_energy_prev).abs();
            }
            log_energy_prev = log_energy;
            pitch_res_ptr = pitch_res_ptr.offset(nSamples as isize);
            k += 1;
        }
        if energy_variation
            > ENERGY_VARIATION_THRESHOLD_QNT_OFFSET * (nSegs - 1 as libc::c_int) as libc::c_float
        {
            (*psEnc).sCmn.indices.quantOffsetType = 0 as libc::c_int as i8;
        } else {
            (*psEnc).sCmn.indices.quantOffsetType = 1 as libc::c_int as i8;
        }
    }
    strength = FIND_PITCH_WHITE_NOISE_FRACTION * (*psEncCtrl).predGain;
    BWExp = BANDWIDTH_EXPANSION / (1.0f32 + strength * strength);
    warping = (*psEnc).sCmn.warping_Q16 as libc::c_float / 65536.0f32
        + 0.01f32 * (*psEncCtrl).coding_quality;
    k = 0 as libc::c_int;
    while k < (*psEnc).sCmn.nb_subfr {
        let mut shift: libc::c_int = 0;
        let mut slope_part: libc::c_int = 0;
        let mut flat_part: libc::c_int = 0;
        flat_part = (*psEnc).sCmn.fs_kHz * 3 as libc::c_int;
        slope_part = ((*psEnc).sCmn.shapeWinLength - flat_part) / 2 as libc::c_int;
        silk_apply_sine_window_FLP(x_windowed.as_mut_ptr(), x_ptr, 1 as libc::c_int, slope_part);
        shift = slope_part;
        memcpy(
            x_windowed.as_mut_ptr().offset(shift as isize) as *mut libc::c_void,
            x_ptr.offset(shift as isize) as *const libc::c_void,
            (flat_part as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
        shift += flat_part;
        silk_apply_sine_window_FLP(
            x_windowed.as_mut_ptr().offset(shift as isize),
            x_ptr.offset(shift as isize),
            2 as libc::c_int,
            slope_part,
        );
        x_ptr = x_ptr.offset((*psEnc).sCmn.subfr_length as isize);
        if (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int {
            silk_warped_autocorrelation_FLP(
                auto_corr.as_mut_ptr(),
                x_windowed.as_mut_ptr(),
                warping,
                (*psEnc).sCmn.shapeWinLength,
                (*psEnc).sCmn.shapingLPCOrder,
            );
        } else {
            silk_autocorrelation_FLP(
                auto_corr.as_mut_ptr(),
                x_windowed.as_mut_ptr(),
                (*psEnc).sCmn.shapeWinLength,
                (*psEnc).sCmn.shapingLPCOrder + 1 as libc::c_int,
            );
        }
        auto_corr[0 as libc::c_int as usize] +=
            auto_corr[0 as libc::c_int as usize] * SHAPE_WHITE_NOISE_FRACTION + 1.0f32;
        nrg = silk_schur_FLP(
            rc.as_mut_ptr(),
            auto_corr.as_mut_ptr() as *const libc::c_float,
            (*psEnc).sCmn.shapingLPCOrder,
        );
        silk_k2a_FLP(
            &mut *((*psEncCtrl).AR)
                .as_mut_ptr()
                .offset((k * MAX_SHAPE_LPC_ORDER) as isize),
            rc.as_mut_ptr(),
            (*psEnc).sCmn.shapingLPCOrder,
        );
        (*psEncCtrl).Gains[k as usize] = (nrg).sqrt();
        if (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int {
            (*psEncCtrl).Gains[k as usize] *= warped_gain(
                &mut *((*psEncCtrl).AR)
                    .as_mut_ptr()
                    .offset((k * MAX_SHAPE_LPC_ORDER) as isize),
                warping,
                (*psEnc).sCmn.shapingLPCOrder,
            );
        }
        silk_bwexpander_FLP(
            &mut *((*psEncCtrl).AR)
                .as_mut_ptr()
                .offset((k * MAX_SHAPE_LPC_ORDER) as isize),
            (*psEnc).sCmn.shapingLPCOrder,
            BWExp,
        );
        if (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int {
            warped_true2monic_coefs(
                &mut *((*psEncCtrl).AR)
                    .as_mut_ptr()
                    .offset((k * MAX_SHAPE_LPC_ORDER) as isize),
                warping,
                3.999f32,
                (*psEnc).sCmn.shapingLPCOrder,
            );
        } else {
            limit_coefs(
                &mut *((*psEncCtrl).AR)
                    .as_mut_ptr()
                    .offset((k * MAX_SHAPE_LPC_ORDER) as isize),
                3.999f32,
                (*psEnc).sCmn.shapingLPCOrder,
            );
        }
        k += 1;
    }
    gain_mult = pow(
        2.0f32 as libc::c_double,
        (-0.16f32 * SNR_adj_dB) as libc::c_double,
    ) as libc::c_float;
    gain_add = pow(
        2.0f32 as libc::c_double,
        (0.16f32 * MIN_QGAIN_DB as libc::c_float) as libc::c_double,
    ) as libc::c_float;
    k = 0 as libc::c_int;
    while k < (*psEnc).sCmn.nb_subfr {
        (*psEncCtrl).Gains[k as usize] *= gain_mult;
        (*psEncCtrl).Gains[k as usize] += gain_add;
        k += 1;
    }
    strength = LOW_FREQ_SHAPING
        * (1.0f32
            + LOW_QUALITY_LOW_FREQ_SHAPING_DECR
                * ((*psEnc).sCmn.input_quality_bands_Q15[0 as libc::c_int as usize]
                    as libc::c_float
                    * (1.0f32 / 32768.0f32)
                    - 1.0f32));
    strength *= (*psEnc).sCmn.speech_activity_Q8 as libc::c_float * (1.0f32 / 256.0f32);
    if (*psEnc).sCmn.indices.signalType as libc::c_int == TYPE_VOICED {
        k = 0 as libc::c_int;
        while k < (*psEnc).sCmn.nb_subfr {
            b = 0.2f32 / (*psEnc).sCmn.fs_kHz as libc::c_float
                + 3.0f32 / (*psEncCtrl).pitchL[k as usize] as libc::c_float;
            (*psEncCtrl).LF_MA_shp[k as usize] = -1.0f32 + b;
            (*psEncCtrl).LF_AR_shp[k as usize] = 1.0f32 - b - b * strength;
            k += 1;
        }
        Tilt = -HP_NOISE_COEF
            - (1 as libc::c_int as libc::c_float - HP_NOISE_COEF)
                * HARM_HP_NOISE_COEF
                * (*psEnc).sCmn.speech_activity_Q8 as libc::c_float
                * (1.0f32 / 256.0f32);
    } else {
        b = 1.3f32 / (*psEnc).sCmn.fs_kHz as libc::c_float;
        (*psEncCtrl).LF_MA_shp[0 as libc::c_int as usize] = -1.0f32 + b;
        (*psEncCtrl).LF_AR_shp[0 as libc::c_int as usize] = 1.0f32 - b - b * strength * 0.6f32;
        k = 1 as libc::c_int;
        while k < (*psEnc).sCmn.nb_subfr {
            (*psEncCtrl).LF_MA_shp[k as usize] = (*psEncCtrl).LF_MA_shp[0 as libc::c_int as usize];
            (*psEncCtrl).LF_AR_shp[k as usize] = (*psEncCtrl).LF_AR_shp[0 as libc::c_int as usize];
            k += 1;
        }
        Tilt = -HP_NOISE_COEF;
    }
    if USE_HARM_SHAPING != 0 && (*psEnc).sCmn.indices.signalType as libc::c_int == TYPE_VOICED {
        HarmShapeGain = HARMONIC_SHAPING;
        HarmShapeGain += HIGH_RATE_OR_LOW_QUALITY_HARMONIC_SHAPING
            * (1.0f32 - (1.0f32 - (*psEncCtrl).coding_quality) * (*psEncCtrl).input_quality);
        HarmShapeGain *= ((*psEnc).LTPCorr).sqrt();
    } else {
        HarmShapeGain = 0.0f32;
    }
    k = 0 as libc::c_int;
    while k < (*psEnc).sCmn.nb_subfr {
        (*psShapeSt).HarmShapeGain_smth +=
            SUBFR_SMTH_COEF * (HarmShapeGain - (*psShapeSt).HarmShapeGain_smth);
        (*psEncCtrl).HarmShapeGain[k as usize] = (*psShapeSt).HarmShapeGain_smth;
        (*psShapeSt).Tilt_smth += SUBFR_SMTH_COEF * (Tilt - (*psShapeSt).Tilt_smth);
        (*psEncCtrl).Tilt[k as usize] = (*psShapeSt).Tilt_smth;
        k += 1;
    }
}
