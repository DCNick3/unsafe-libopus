use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:35"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:35"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:35"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:35"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:35"]
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
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/xmmintrin.h:35"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvtss_si32, _mm_cvt_ss2si, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvtss_si32, _mm_cvt_ss2si, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:35"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/control.h:35"]
pub mod control_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:9"]
    pub struct silk_EncControlStruct {
        pub nChannelsAPI: opus_int32,
        pub nChannelsInternal: opus_int32,
        pub API_sampleRate: opus_int32,
        pub maxInternalSampleRate: opus_int32,
        pub minInternalSampleRate: opus_int32,
        pub desiredInternalSampleRate: opus_int32,
        pub payloadSize_ms: libc::c_int,
        pub bitRate: opus_int32,
        pub packetLossPercentage: libc::c_int,
        pub complexity: libc::c_int,
        pub useInBandFEC: libc::c_int,
        pub LBRR_coded: libc::c_int,
        pub useDTX: libc::c_int,
        pub useCBR: libc::c_int,
        pub maxBits: libc::c_int,
        pub toMono: libc::c_int,
        pub opusCanSwitch: libc::c_int,
        pub reducedDependency: libc::c_int,
        pub internalSampleRate: opus_int32,
        pub allowBandwidthSwitch: libc::c_int,
        pub inWBmodeWithoutVariableLP: libc::c_int,
        pub stereoWidth_Q14: libc::c_int,
        pub switchReady: libc::c_int,
        pub signalType: libc::c_int,
        pub offset: libc::c_int,
    }
    use super::opus_types_h::opus_int32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/structs_FLP.h:35"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:35"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:35"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "546:1"]
    pub unsafe extern "C" fn silk_min_int(
        a: libc::c_int,
        b: libc::c_int,
    ) -> libc::c_int {
        return if a < b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "564:1"]
    pub unsafe extern "C" fn silk_max_int(
        a: libc::c_int,
        b: libc::c_int,
    ) -> libc::c_int {
        return if a > b { a } else { b };
    }
    use super::resampler_structs_h::silk_resampler_state_struct;
    use super::opus_types_h::{opus_int32, opus_int16};
    extern "C" {
        #[c2rust::src_loc = "62:1"]
        pub fn silk_resampler_init(
            S: *mut silk_resampler_state_struct,
            Fs_Hz_in: opus_int32,
            Fs_Hz_out: opus_int32,
            forEnc: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "72:1"]
        pub fn silk_resampler(
            S: *mut silk_resampler_state_struct,
            out: *mut opus_int16,
            in_0: *const opus_int16,
            inLen: opus_int32,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:35"]
pub mod define_h {
    #[c2rust::src_loc = "112:9"]
    pub const LA_SHAPE_MS: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "70:9"]
    pub const TYPE_NO_VOICE_ACTIVITY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "143:9"]
    pub const MIN_LPC_ORDER: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "142:9"]
    pub const MAX_LPC_ORDER: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "90:9"]
    pub const MAX_NB_SUBFR: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "160:9"]
    pub const MAX_DEL_DEC_STATES: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "94:9"]
    pub const SUB_FRAME_LENGTH_MS: libc::c_int = 5 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/errors.h:35"]
pub mod errors_h {
    #[c2rust::src_loc = "52:9"]
    pub const SILK_ENC_PACKET_SIZE_NOT_SUPPORTED: libc::c_int = -(103 as libc::c_int);
    #[c2rust::src_loc = "39:9"]
    pub const SILK_NO_ERROR: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/string.h:35"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(
            _: *mut libc::c_void,
            _: libc::c_int,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/float_cast.h:35"]
pub mod float_cast_h {
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn float2int(x: libc::c_float) -> opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }
    use super::opus_types_h::opus_int32;
    use super::xmmintrin_h::{_mm_cvt_ss2si, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:35"]
pub mod SigProc_FLP_h {
    #[inline]
    #[c2rust::src_loc = "162:1"]
    pub unsafe extern "C" fn silk_float2short_array(
        out: *mut opus_int16,
        in_0: *const libc::c_float,
        length: opus_int32,
    ) {
        let mut k: opus_int32 = 0;
        k = length - 1 as libc::c_int;
        while k >= 0 as libc::c_int {
            *out
                .offset(
                    k as isize,
                ) = (if float2int(*in_0.offset(k as isize)) > silk_int16_MAX {
                silk_int16_MAX
            } else if float2int(*in_0.offset(k as isize)) < silk_int16_MIN {
                silk_int16_MIN
            } else {
                float2int(*in_0.offset(k as isize))
            }) as opus_int16;
            k -= 1;
        }
    }
    #[inline]
    #[c2rust::src_loc = "175:1"]
    pub unsafe extern "C" fn silk_short2float_array(
        out: *mut libc::c_float,
        in_0: *const opus_int16,
        length: opus_int32,
    ) {
        let mut k: opus_int32 = 0;
        k = length - 1 as libc::c_int;
        while k >= 0 as libc::c_int {
            *out.offset(k as isize) = *in_0.offset(k as isize) as libc::c_float;
            k -= 1;
        }
    }
    use super::opus_types_h::{opus_int16, opus_int32};
    use super::float_cast_h::float2int;
    use super::typedef_h::{silk_int16_MAX, silk_int16_MIN};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:35"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:35"]
pub mod tables_h {
    use super::opus_types_h::{opus_uint8};
    use super::structs_h::silk_NLSF_CB_struct;
    extern "C" {
        #[c2rust::src_loc = "45:26"]
        pub static silk_pitch_contour_iCDF: [opus_uint8; 34];
        #[c2rust::src_loc = "46:26"]
        pub static silk_pitch_contour_NB_iCDF: [opus_uint8; 11];
        #[c2rust::src_loc = "47:26"]
        pub static silk_pitch_contour_10_ms_iCDF: [opus_uint8; 12];
        #[c2rust::src_loc = "48:26"]
        pub static silk_pitch_contour_10_ms_NB_iCDF: [opus_uint8; 3];
        #[c2rust::src_loc = "69:26"]
        pub static silk_uniform4_iCDF: [opus_uint8; 4];
        #[c2rust::src_loc = "71:26"]
        pub static silk_uniform6_iCDF: [opus_uint8; 6];
        #[c2rust::src_loc = "72:26"]
        pub static silk_uniform8_iCDF: [opus_uint8; 8];
        #[c2rust::src_loc = "97:34"]
        pub static silk_NLSF_CB_WB: silk_NLSF_CB_struct;
        #[c2rust::src_loc = "98:34"]
        pub static silk_NLSF_CB_NB_MB: silk_NLSF_CB_struct;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:35"]
pub mod main_h {
    use super::structs_h::silk_encoder_state;
    use super::control_h::silk_EncControlStruct;
    extern "C" {
        #[c2rust::src_loc = "140:1"]
        pub fn silk_control_audio_bandwidth(
            psEncC: *mut silk_encoder_state,
            encControl: *mut silk_EncControlStruct,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/pitch_est_defines.h:40"]
pub mod pitch_est_defines_h {
    #[c2rust::src_loc = "72:9"]
    pub const SILK_PE_MIN_COMPLEX: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "73:9"]
    pub const SILK_PE_MID_COMPLEX: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "74:9"]
    pub const SILK_PE_MAX_COMPLEX: libc::c_int = 2 as libc::c_int;
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
pub use self::control_h::silk_EncControlStruct;
pub use self::structs_FLP_h::{silk_shape_state_FLP, silk_encoder_state_FLP};
use self::arch_h::celt_fatal;
pub use self::SigProc_FIX_h::{
    silk_min_int, silk_max_int, silk_resampler_init, silk_resampler,
};
pub use self::define_h::{
    LA_SHAPE_MS, TYPE_NO_VOICE_ACTIVITY, MIN_LPC_ORDER, MAX_LPC_ORDER, MAX_NB_SUBFR,
    MAX_DEL_DEC_STATES, SUB_FRAME_LENGTH_MS,
};
pub use self::errors_h::{SILK_ENC_PACKET_SIZE_NOT_SUPPORTED, SILK_NO_ERROR};
use self::string_h::memset;
pub use self::float_cast_h::float2int;
pub use self::SigProc_FLP_h::{silk_float2short_array, silk_short2float_array};
pub use self::typedef_h::{silk_int16_MIN, silk_int16_MAX};
use self::tables_h::{
    silk_pitch_contour_iCDF, silk_pitch_contour_NB_iCDF, silk_pitch_contour_10_ms_iCDF,
    silk_pitch_contour_10_ms_NB_iCDF, silk_uniform4_iCDF, silk_uniform6_iCDF,
    silk_uniform8_iCDF, silk_NLSF_CB_WB, silk_NLSF_CB_NB_MB,
};
use self::main_h::silk_control_audio_bandwidth;
pub use self::pitch_est_defines_h::{
    SILK_PE_MIN_COMPLEX, SILK_PE_MID_COMPLEX, SILK_PE_MAX_COMPLEX,
};
#[no_mangle]
#[c2rust::src_loc = "65:1"]
pub unsafe extern "C" fn silk_control_encoder(
    mut psEnc: *mut silk_encoder_state_FLP,
    encControl: *mut silk_EncControlStruct,
    allow_bw_switch: libc::c_int,
    channelNb: libc::c_int,
    force_fs_kHz: libc::c_int,
) -> libc::c_int {
    let mut fs_kHz: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    (*psEnc).sCmn.useDTX = (*encControl).useDTX;
    (*psEnc).sCmn.useCBR = (*encControl).useCBR;
    (*psEnc).sCmn.API_fs_Hz = (*encControl).API_sampleRate;
    (*psEnc).sCmn.maxInternal_fs_Hz = (*encControl).maxInternalSampleRate;
    (*psEnc).sCmn.minInternal_fs_Hz = (*encControl).minInternalSampleRate;
    (*psEnc).sCmn.desiredInternal_fs_Hz = (*encControl).desiredInternalSampleRate;
    (*psEnc).sCmn.useInBandFEC = (*encControl).useInBandFEC;
    (*psEnc).sCmn.nChannelsAPI = (*encControl).nChannelsAPI;
    (*psEnc).sCmn.nChannelsInternal = (*encControl).nChannelsInternal;
    (*psEnc).sCmn.allow_bandwidth_switch = allow_bw_switch;
    (*psEnc).sCmn.channelNb = channelNb;
    if (*psEnc).sCmn.controlled_since_last_payload != 0 as libc::c_int
        && (*psEnc).sCmn.prefillFlag == 0 as libc::c_int
    {
        if (*psEnc).sCmn.API_fs_Hz != (*psEnc).sCmn.prev_API_fs_Hz
            && (*psEnc).sCmn.fs_kHz > 0 as libc::c_int
        {
            ret += silk_setup_resamplers(psEnc, (*psEnc).sCmn.fs_kHz);
        }
        return ret;
    }
    fs_kHz = silk_control_audio_bandwidth(&mut (*psEnc).sCmn, encControl);
    if force_fs_kHz != 0 {
        fs_kHz = force_fs_kHz;
    }
    ret += silk_setup_resamplers(psEnc, fs_kHz);
    ret += silk_setup_fs(psEnc, fs_kHz, (*encControl).payloadSize_ms);
    ret += silk_setup_complexity(&mut (*psEnc).sCmn, (*encControl).complexity);
    (*psEnc).sCmn.PacketLoss_perc = (*encControl).packetLossPercentage;
    ret += silk_setup_LBRR(&mut (*psEnc).sCmn, encControl);
    (*psEnc).sCmn.controlled_since_last_payload = 1 as libc::c_int;
    return ret;
}
#[c2rust::src_loc = "134:1"]
unsafe extern "C" fn silk_setup_resamplers(
    mut psEnc: *mut silk_encoder_state_FLP,
    fs_kHz: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = SILK_NO_ERROR;
    if (*psEnc).sCmn.fs_kHz != fs_kHz
        || (*psEnc).sCmn.prev_API_fs_Hz != (*psEnc).sCmn.API_fs_Hz
    {
        if (*psEnc).sCmn.fs_kHz == 0 as libc::c_int {
            ret
                += silk_resampler_init(
                    &mut (*psEnc).sCmn.resampler_state,
                    (*psEnc).sCmn.API_fs_Hz,
                    fs_kHz * 1000 as libc::c_int,
                    1 as libc::c_int,
                );
        } else {
            let mut new_buf_samples: opus_int32 = 0;
            let mut api_buf_samples: opus_int32 = 0;
            let mut old_buf_samples: opus_int32 = 0;
            let mut buf_length_ms: opus_int32 = 0;
            buf_length_ms = ((((*psEnc).sCmn.nb_subfr * 5 as libc::c_int) as opus_uint32)
                << 1 as libc::c_int) as opus_int32 + LA_SHAPE_MS;
            old_buf_samples = buf_length_ms * (*psEnc).sCmn.fs_kHz;
            new_buf_samples = buf_length_ms * fs_kHz;
            let vla = (if old_buf_samples > new_buf_samples {
                old_buf_samples
            } else {
                new_buf_samples
            }) as usize;
            let mut x_bufFIX: Vec::<opus_int16> = ::std::vec::from_elem(0, vla);
            silk_float2short_array(
                x_bufFIX.as_mut_ptr(),
                ((*psEnc).x_buf).as_mut_ptr(),
                old_buf_samples,
            );
            let mut temp_resampler_state: [silk_resampler_state_struct; 1] = [silk_resampler_state_struct {
                sIIR: [0; 6],
                sFIR: C2RustUnnamed { i32_0: [0; 36] },
                delayBuf: [0; 48],
                resampler_function: 0,
                batchSize: 0,
                invRatio_Q16: 0,
                FIR_Order: 0,
                FIR_Fracs: 0,
                Fs_in_kHz: 0,
                Fs_out_kHz: 0,
                inputDelay: 0,
                Coefs: 0 as *const opus_int16,
            }; 1];
            ret
                += silk_resampler_init(
                    temp_resampler_state.as_mut_ptr(),
                    (*psEnc).sCmn.fs_kHz as opus_int16 as opus_int32
                        * 1000 as libc::c_int as opus_int16 as opus_int32,
                    (*psEnc).sCmn.API_fs_Hz,
                    0 as libc::c_int,
                );
            api_buf_samples = buf_length_ms
                * ((*psEnc).sCmn.API_fs_Hz / 1000 as libc::c_int);
            let vla_0 = api_buf_samples as usize;
            let mut x_buf_API_fs_Hz: Vec::<opus_int16> = ::std::vec::from_elem(0, vla_0);
            ret
                += silk_resampler(
                    temp_resampler_state.as_mut_ptr(),
                    x_buf_API_fs_Hz.as_mut_ptr(),
                    x_bufFIX.as_mut_ptr() as *const opus_int16,
                    old_buf_samples,
                );
            ret
                += silk_resampler_init(
                    &mut (*psEnc).sCmn.resampler_state,
                    (*psEnc).sCmn.API_fs_Hz,
                    fs_kHz as opus_int16 as opus_int32
                        * 1000 as libc::c_int as opus_int16 as opus_int32,
                    1 as libc::c_int,
                );
            ret
                += silk_resampler(
                    &mut (*psEnc).sCmn.resampler_state,
                    x_bufFIX.as_mut_ptr(),
                    x_buf_API_fs_Hz.as_mut_ptr() as *const opus_int16,
                    api_buf_samples,
                );
            silk_short2float_array(
                ((*psEnc).x_buf).as_mut_ptr(),
                x_bufFIX.as_mut_ptr(),
                new_buf_samples,
            );
        }
    }
    (*psEnc).sCmn.prev_API_fs_Hz = (*psEnc).sCmn.API_fs_Hz;
    return ret;
}
#[c2rust::src_loc = "199:1"]
unsafe extern "C" fn silk_setup_fs(
    mut psEnc: *mut silk_encoder_state_FLP,
    fs_kHz: libc::c_int,
    PacketSize_ms: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = SILK_NO_ERROR;
    if PacketSize_ms != (*psEnc).sCmn.PacketSize_ms {
        if PacketSize_ms != 10 as libc::c_int && PacketSize_ms != 20 as libc::c_int
            && PacketSize_ms != 40 as libc::c_int && PacketSize_ms != 60 as libc::c_int
        {
            ret = SILK_ENC_PACKET_SIZE_NOT_SUPPORTED;
        }
        if PacketSize_ms <= 10 as libc::c_int {
            (*psEnc).sCmn.nFramesPerPacket = 1 as libc::c_int;
            (*psEnc)
                .sCmn
                .nb_subfr = if PacketSize_ms == 10 as libc::c_int {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            (*psEnc)
                .sCmn
                .frame_length = PacketSize_ms as opus_int16 as opus_int32
                * fs_kHz as opus_int16 as opus_int32;
            (*psEnc)
                .sCmn
                .pitch_LPC_win_length = (10 as libc::c_int
                + ((2 as libc::c_int) << 1 as libc::c_int)) as opus_int16 as opus_int32
                * fs_kHz as opus_int16 as opus_int32;
            if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int {
                (*psEnc)
                    .sCmn
                    .pitch_contour_iCDF = silk_pitch_contour_10_ms_NB_iCDF.as_ptr();
            } else {
                (*psEnc)
                    .sCmn
                    .pitch_contour_iCDF = silk_pitch_contour_10_ms_iCDF.as_ptr();
            }
        } else {
            (*psEnc)
                .sCmn
                .nFramesPerPacket = PacketSize_ms
                / (5 as libc::c_int * 4 as libc::c_int);
            (*psEnc).sCmn.nb_subfr = MAX_NB_SUBFR;
            (*psEnc)
                .sCmn
                .frame_length = 20 as libc::c_int as opus_int16 as opus_int32
                * fs_kHz as opus_int16 as opus_int32;
            (*psEnc)
                .sCmn
                .pitch_LPC_win_length = (20 as libc::c_int
                + ((2 as libc::c_int) << 1 as libc::c_int)) as opus_int16 as opus_int32
                * fs_kHz as opus_int16 as opus_int32;
            if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int {
                (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_NB_iCDF.as_ptr();
            } else {
                (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_iCDF.as_ptr();
            }
        }
        (*psEnc).sCmn.PacketSize_ms = PacketSize_ms;
        (*psEnc).sCmn.TargetRate_bps = 0 as libc::c_int;
    }
    if !(fs_kHz == 8 as libc::c_int || fs_kHz == 12 as libc::c_int
        || fs_kHz == 16 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: fs_kHz == 8 || fs_kHz == 12 || fs_kHz == 16\0"
                as *const u8 as *const libc::c_char,
            b"silk/control_codec.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int,
        );
    }
    if !((*psEnc).sCmn.nb_subfr == 2 as libc::c_int
        || (*psEnc).sCmn.nb_subfr == 4 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: psEnc->sCmn.nb_subfr == 2 || psEnc->sCmn.nb_subfr == 4\0"
                as *const u8 as *const libc::c_char,
            b"silk/control_codec.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int,
        );
    }
    if (*psEnc).sCmn.fs_kHz != fs_kHz {
        memset(
            &mut (*psEnc).sShape as *mut silk_shape_state_FLP as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<silk_shape_state_FLP>() as libc::c_ulong,
        );
        memset(
            &mut (*psEnc).sCmn.sNSQ as *mut silk_nsq_state as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<silk_nsq_state>() as libc::c_ulong,
        );
        memset(
            ((*psEnc).sCmn.prev_NLSFq_Q15).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[opus_int16; 16]>() as libc::c_ulong,
        );
        memset(
            &mut (*psEnc).sCmn.sLP.In_LP_State as *mut [opus_int32; 2]
                as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[opus_int32; 2]>() as libc::c_ulong,
        );
        (*psEnc).sCmn.inputBufIx = 0 as libc::c_int;
        (*psEnc).sCmn.nFramesEncoded = 0 as libc::c_int;
        (*psEnc).sCmn.TargetRate_bps = 0 as libc::c_int;
        (*psEnc).sCmn.prevLag = 100 as libc::c_int;
        (*psEnc).sCmn.first_frame_after_reset = 1 as libc::c_int;
        (*psEnc).sShape.LastGainIndex = 10 as libc::c_int as opus_int8;
        (*psEnc).sCmn.sNSQ.lagPrev = 100 as libc::c_int;
        (*psEnc).sCmn.sNSQ.prev_gain_Q16 = 65536 as libc::c_int;
        (*psEnc).sCmn.prevSignalType = TYPE_NO_VOICE_ACTIVITY as opus_int8;
        (*psEnc).sCmn.fs_kHz = fs_kHz;
        if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int {
            if (*psEnc).sCmn.nb_subfr == MAX_NB_SUBFR {
                (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_NB_iCDF.as_ptr();
            } else {
                (*psEnc)
                    .sCmn
                    .pitch_contour_iCDF = silk_pitch_contour_10_ms_NB_iCDF.as_ptr();
            }
        } else if (*psEnc).sCmn.nb_subfr == MAX_NB_SUBFR {
            (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_iCDF.as_ptr();
        } else {
            (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_10_ms_iCDF.as_ptr();
        }
        if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int
            || (*psEnc).sCmn.fs_kHz == 12 as libc::c_int
        {
            (*psEnc).sCmn.predictLPCOrder = MIN_LPC_ORDER;
            (*psEnc).sCmn.psNLSF_CB = &silk_NLSF_CB_NB_MB;
        } else {
            (*psEnc).sCmn.predictLPCOrder = MAX_LPC_ORDER;
            (*psEnc).sCmn.psNLSF_CB = &silk_NLSF_CB_WB;
        }
        (*psEnc).sCmn.subfr_length = SUB_FRAME_LENGTH_MS * fs_kHz;
        (*psEnc)
            .sCmn
            .frame_length = (*psEnc).sCmn.subfr_length as opus_int16 as opus_int32
            * (*psEnc).sCmn.nb_subfr as opus_int16 as opus_int32;
        (*psEnc)
            .sCmn
            .ltp_mem_length = 20 as libc::c_int as opus_int16 as opus_int32
            * fs_kHz as opus_int16 as opus_int32;
        (*psEnc)
            .sCmn
            .la_pitch = 2 as libc::c_int as opus_int16 as opus_int32
            * fs_kHz as opus_int16 as opus_int32;
        (*psEnc)
            .sCmn
            .max_pitch_lag = 18 as libc::c_int as opus_int16 as opus_int32
            * fs_kHz as opus_int16 as opus_int32;
        if (*psEnc).sCmn.nb_subfr == MAX_NB_SUBFR {
            (*psEnc)
                .sCmn
                .pitch_LPC_win_length = (20 as libc::c_int
                + ((2 as libc::c_int) << 1 as libc::c_int)) as opus_int16 as opus_int32
                * fs_kHz as opus_int16 as opus_int32;
        } else {
            (*psEnc)
                .sCmn
                .pitch_LPC_win_length = (10 as libc::c_int
                + ((2 as libc::c_int) << 1 as libc::c_int)) as opus_int16 as opus_int32
                * fs_kHz as opus_int16 as opus_int32;
        }
        if (*psEnc).sCmn.fs_kHz == 16 as libc::c_int {
            (*psEnc).sCmn.pitch_lag_low_bits_iCDF = silk_uniform8_iCDF.as_ptr();
        } else if (*psEnc).sCmn.fs_kHz == 12 as libc::c_int {
            (*psEnc).sCmn.pitch_lag_low_bits_iCDF = silk_uniform6_iCDF.as_ptr();
        } else {
            (*psEnc).sCmn.pitch_lag_low_bits_iCDF = silk_uniform4_iCDF.as_ptr();
        }
    }
    if !((*psEnc).sCmn.subfr_length * (*psEnc).sCmn.nb_subfr
        == (*psEnc).sCmn.frame_length)
    {
        celt_fatal(
            b"assertion failed: ( psEnc->sCmn.subfr_length * psEnc->sCmn.nb_subfr ) == psEnc->sCmn.frame_length\0"
                as *const u8 as *const libc::c_char,
            b"silk/control_codec.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int,
        );
    }
    return ret;
}
#[c2rust::src_loc = "307:1"]
unsafe extern "C" fn silk_setup_complexity(
    mut psEncC: *mut silk_encoder_state,
    Complexity: libc::c_int,
) -> libc::c_int {
    let ret: libc::c_int = 0 as libc::c_int;
    if !(Complexity >= 0 as libc::c_int && Complexity <= 10 as libc::c_int) {
        celt_fatal(
            b"assertion failed: Complexity >= 0 && Complexity <= 10\0" as *const u8
                as *const libc::c_char,
            b"silk/control_codec.c\0" as *const u8 as *const libc::c_char,
            315 as libc::c_int,
        );
    }
    if Complexity < 1 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MIN_COMPLEX;
        (*psEncC)
            .pitchEstimationThreshold_Q16 = (0.8f64
            * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 6 as libc::c_int;
        (*psEncC).shapingLPCOrder = 12 as libc::c_int;
        (*psEncC).la_shape = 3 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 1 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 0 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 2 as libc::c_int;
        (*psEncC).warping_Q16 = 0 as libc::c_int;
    } else if Complexity < 2 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MID_COMPLEX;
        (*psEncC)
            .pitchEstimationThreshold_Q16 = (0.76f64
            * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 8 as libc::c_int;
        (*psEncC).shapingLPCOrder = 14 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 1 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 0 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 3 as libc::c_int;
        (*psEncC).warping_Q16 = 0 as libc::c_int;
    } else if Complexity < 3 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MIN_COMPLEX;
        (*psEncC)
            .pitchEstimationThreshold_Q16 = (0.8f64
            * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 6 as libc::c_int;
        (*psEncC).shapingLPCOrder = 12 as libc::c_int;
        (*psEncC).la_shape = 3 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 2 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 0 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 2 as libc::c_int;
        (*psEncC).warping_Q16 = 0 as libc::c_int;
    } else if Complexity < 4 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MID_COMPLEX;
        (*psEncC)
            .pitchEstimationThreshold_Q16 = (0.76f64
            * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 8 as libc::c_int;
        (*psEncC).shapingLPCOrder = 14 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 2 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 0 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 4 as libc::c_int;
        (*psEncC).warping_Q16 = 0 as libc::c_int;
    } else if Complexity < 6 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MID_COMPLEX;
        (*psEncC)
            .pitchEstimationThreshold_Q16 = (0.74f64
            * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 10 as libc::c_int;
        (*psEncC).shapingLPCOrder = 16 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 2 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 1 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 6 as libc::c_int;
        (*psEncC)
            .warping_Q16 = (*psEncC).fs_kHz
            * ((0.015f32
                * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int)
                    as libc::c_float) as libc::c_double + 0.5f64) as opus_int32;
    } else if Complexity < 8 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MID_COMPLEX;
        (*psEncC)
            .pitchEstimationThreshold_Q16 = (0.72f64
            * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 12 as libc::c_int;
        (*psEncC).shapingLPCOrder = 20 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 3 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 1 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 8 as libc::c_int;
        (*psEncC)
            .warping_Q16 = (*psEncC).fs_kHz
            * ((0.015f32
                * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int)
                    as libc::c_float) as libc::c_double + 0.5f64) as opus_int32;
    } else {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MAX_COMPLEX;
        (*psEncC)
            .pitchEstimationThreshold_Q16 = (0.7f64
            * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 16 as libc::c_int;
        (*psEncC).shapingLPCOrder = 24 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = MAX_DEL_DEC_STATES;
        (*psEncC).useInterpolatedNLSFs = 1 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 16 as libc::c_int;
        (*psEncC)
            .warping_Q16 = (*psEncC).fs_kHz
            * ((0.015f32
                * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int)
                    as libc::c_float) as libc::c_double + 0.5f64) as opus_int32;
    }
    (*psEncC)
        .pitchEstimationLPCOrder = silk_min_int(
        (*psEncC).pitchEstimationLPCOrder,
        (*psEncC).predictLPCOrder,
    );
    (*psEncC)
        .shapeWinLength = SUB_FRAME_LENGTH_MS * (*psEncC).fs_kHz
        + 2 as libc::c_int * (*psEncC).la_shape;
    (*psEncC).Complexity = Complexity;
    if !((*psEncC).pitchEstimationLPCOrder <= 16 as libc::c_int) {
        celt_fatal(
            b"assertion failed: psEncC->pitchEstimationLPCOrder <= MAX_FIND_PITCH_LPC_ORDER\0"
                as *const u8 as *const libc::c_char,
            b"silk/control_codec.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
        );
    }
    if !((*psEncC).shapingLPCOrder <= 24 as libc::c_int) {
        celt_fatal(
            b"assertion failed: psEncC->shapingLPCOrder <= MAX_SHAPE_LPC_ORDER\0"
                as *const u8 as *const libc::c_char,
            b"silk/control_codec.c\0" as *const u8 as *const libc::c_char,
            394 as libc::c_int,
        );
    }
    if !((*psEncC).nStatesDelayedDecision <= 4 as libc::c_int) {
        celt_fatal(
            b"assertion failed: psEncC->nStatesDelayedDecision <= MAX_DEL_DEC_STATES\0"
                as *const u8 as *const libc::c_char,
            b"silk/control_codec.c\0" as *const u8 as *const libc::c_char,
            395 as libc::c_int,
        );
    }
    if !((*psEncC).warping_Q16 <= 32767 as libc::c_int) {
        celt_fatal(
            b"assertion failed: psEncC->warping_Q16 <= 32767\0" as *const u8
                as *const libc::c_char,
            b"silk/control_codec.c\0" as *const u8 as *const libc::c_char,
            396 as libc::c_int,
        );
    }
    if !((*psEncC).la_shape <= 5 as libc::c_int * 16 as libc::c_int) {
        celt_fatal(
            b"assertion failed: psEncC->la_shape <= LA_SHAPE_MAX\0" as *const u8
                as *const libc::c_char,
            b"silk/control_codec.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
        );
    }
    if !((*psEncC).shapeWinLength <= 15 as libc::c_int * 16 as libc::c_int) {
        celt_fatal(
            b"assertion failed: psEncC->shapeWinLength <= SHAPE_LPC_WIN_MAX\0"
                as *const u8 as *const libc::c_char,
            b"silk/control_codec.c\0" as *const u8 as *const libc::c_char,
            398 as libc::c_int,
        );
    }
    return ret;
}
#[inline]
#[c2rust::src_loc = "403:1"]
unsafe extern "C" fn silk_setup_LBRR(
    mut psEncC: *mut silk_encoder_state,
    encControl: *const silk_EncControlStruct,
) -> libc::c_int {
    let mut LBRR_in_previous_packet: libc::c_int = 0;
    let ret: libc::c_int = SILK_NO_ERROR;
    LBRR_in_previous_packet = (*psEncC).LBRR_enabled;
    (*psEncC).LBRR_enabled = (*encControl).LBRR_coded;
    if (*psEncC).LBRR_enabled != 0 {
        if LBRR_in_previous_packet == 0 as libc::c_int {
            (*psEncC).LBRR_GainIncreases = 7 as libc::c_int;
        } else {
            (*psEncC)
                .LBRR_GainIncreases = silk_max_int(
                7 as libc::c_int
                    - ((*psEncC).PacketLoss_perc as libc::c_long
                        * (0.4f64
                            * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int)
                                as libc::c_double + 0.5f64) as opus_int32 as opus_int16
                            as opus_int64 >> 16 as libc::c_int) as opus_int32,
                2 as libc::c_int,
            );
        }
    }
    return ret;
}
