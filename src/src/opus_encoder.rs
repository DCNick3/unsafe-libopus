use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stdarg.h:32"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:33"]
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
    use super::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
    use super::stdint_uintn_h::{uint32_t, uint8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_custom.h:33"]
pub mod opus_custom_h {
    extern "C" {
        #[c2rust::src_loc = "95:16"]
        pub type OpusCustomEncoder;
        #[c2rust::src_loc = "238:20"]
        pub fn opus_custom_encoder_ctl(
            st: *mut OpusCustomEncoder,
            request: libc::c_int,
            _: ...
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/modes.h:35"]
pub mod modes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:8"]
    pub struct OpusCustomMode {
        pub Fs: opus_int32,
        pub overlap: libc::c_int,
        pub nbEBands: libc::c_int,
        pub effEBands: libc::c_int,
        pub preemph: [opus_val16; 4],
        pub eBands: *const opus_int16,
        pub maxLM: libc::c_int,
        pub nbShortMdcts: libc::c_int,
        pub shortMdctSize: libc::c_int,
        pub nbAllocVectors: libc::c_int,
        pub allocVectors: *const libc::c_uchar,
        pub logN: *const opus_int16,
        pub window: *const opus_val16,
        pub mdct: mdct_lookup,
        pub cache: PulseCache,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:9"]
    pub struct PulseCache {
        pub size: libc::c_int,
        pub index: *const opus_int16,
        pub bits: *const libc::c_uchar,
        pub caps: *const libc::c_uchar,
    }
    use super::arch_h::opus_val16;
    use super::mdct_h::mdct_lookup;
    use super::opus_types_h::{opus_int16, opus_int32};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mdct.h:35"]
pub mod mdct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:9"]
    pub struct mdct_lookup {
        pub n: libc::c_int,
        pub maxshift: libc::c_int,
        pub kfft: [*const kiss_fft_state; 4],
        pub trig: *const libc::c_float,
    }
    use super::kiss_fft_h::kiss_fft_state;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/kiss_fft.h:35"]
pub mod kiss_fft_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:16"]
    pub struct kiss_fft_state {
        pub nfft: libc::c_int,
        pub scale: opus_val16,
        pub shift: libc::c_int,
        pub factors: [opus_int16; 16],
        pub bitrev: *const opus_int16,
        pub twiddles: *const kiss_twiddle_cpx,
        pub arch_fft: *mut arch_fft_state,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct arch_fft_state {
        pub is_supported: libc::c_int,
        pub priv_0: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:9"]
    pub struct kiss_twiddle_cpx {
        pub r: libc::c_float,
        pub i: libc::c_float,
    }
    use super::arch_h::opus_val16;
    use super::opus_types_h::opus_int16;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:33"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = opus_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: opus_uint32,
        pub end_offs: opus_uint32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: opus_uint32,
        pub rng: opus_uint32,
        pub val: opus_uint32,
        pub ext: opus_uint32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "47:1"]
    pub type ec_enc = ec_ctx;
    #[inline]
    #[c2rust::src_loc = "111:1"]
    pub unsafe extern "C" fn ec_tell(mut _this: *mut ec_ctx) -> libc::c_int {
        return (*_this).nbits_total
            - (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
                - ((*_this).rng).leading_zeros() as i32);
    }
    use super::opus_types_h::opus_uint32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/celt.h:33"]
pub mod celt_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:9"]
    pub struct AnalysisInfo {
        pub valid: libc::c_int,
        pub tonality: libc::c_float,
        pub tonality_slope: libc::c_float,
        pub noisiness: libc::c_float,
        pub activity: libc::c_float,
        pub music_prob: libc::c_float,
        pub music_prob_min: libc::c_float,
        pub music_prob_max: libc::c_float,
        pub bandwidth: libc::c_int,
        pub activity_probability: libc::c_float,
        pub max_pitch_ratio: libc::c_float,
        pub leak_boost: [libc::c_uchar; 19],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:9"]
    pub struct SILKInfo {
        pub signalType: libc::c_int,
        pub offset: libc::c_int,
    }
    use super::arch_h::opus_val16;
    use super::entcode_h::ec_enc;
    use super::opus_custom_h::OpusCustomEncoder;
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "136:1"]
        pub fn celt_encoder_get_size(channels: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "140:1"]
        pub fn celt_encoder_init(
            st: *mut OpusCustomEncoder,
            sampling_rate: opus_int32,
            channels: libc::c_int,
            arch: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "138:1"]
        pub fn celt_encode_with_ec(
            st: *mut OpusCustomEncoder,
            pcm: *const opus_val16,
            frame_size: libc::c_int,
            compressed: *mut libc::c_uchar,
            nbCompressedBytes: libc::c_int,
            enc: *mut ec_enc,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/control.h:36"]
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
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/xmmintrin.h:38"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/analysis.h:45"]
pub mod analysis_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:9"]
    pub struct TonalityAnalysisState {
        pub arch: libc::c_int,
        pub application: libc::c_int,
        pub Fs: opus_int32,
        pub angle: [libc::c_float; 240],
        pub d_angle: [libc::c_float; 240],
        pub d2_angle: [libc::c_float; 240],
        pub inmem: [opus_val32; 720],
        pub mem_fill: libc::c_int,
        pub prev_band_tonality: [libc::c_float; 18],
        pub prev_tonality: libc::c_float,
        pub prev_bandwidth: libc::c_int,
        pub E: [[libc::c_float; 18]; 8],
        pub logE: [[libc::c_float; 18]; 8],
        pub lowE: [libc::c_float; 18],
        pub highE: [libc::c_float; 18],
        pub meanE: [libc::c_float; 19],
        pub mem: [libc::c_float; 32],
        pub cmean: [libc::c_float; 8],
        pub std: [libc::c_float; 9],
        pub Etracker: libc::c_float,
        pub lowECount: libc::c_float,
        pub E_count: libc::c_int,
        pub count: libc::c_int,
        pub analysis_offset: libc::c_int,
        pub write_pos: libc::c_int,
        pub read_pos: libc::c_int,
        pub read_subframe: libc::c_int,
        pub hp_ener_accum: libc::c_float,
        pub initialized: libc::c_int,
        pub rnn_state: [libc::c_float; 32],
        pub downmix_state: [opus_val32; 3],
        pub info: [AnalysisInfo; 100],
    }
    use super::arch_h::opus_val32;
    use super::celt_h::AnalysisInfo;
    use super::modes_h::OpusCustomMode;
    use super::opus_private_h::downmix_func;
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "89:1"]
        pub fn tonality_analysis_init(analysis: *mut TonalityAnalysisState, Fs: opus_int32);
        #[c2rust::src_loc = "99:1"]
        pub fn run_analysis(
            analysis: *mut TonalityAnalysisState,
            celt_mode: *const OpusCustomMode,
            analysis_pcm: *const libc::c_void,
            analysis_frame_size: libc::c_int,
            frame_size: libc::c_int,
            c1: libc::c_int,
            c2: libc::c_int,
            C: libc::c_int,
            Fs: opus_int32,
            lsb_depth: libc::c_int,
            downmix: downmix_func,
            analysis_info: *mut AnalysisInfo,
        );
        #[c2rust::src_loc = "95:1"]
        pub fn tonality_analysis_reset(analysis: *mut TonalityAnalysisState);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/opus_private.h:42"]
pub mod opus_private_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "156:12"]
    pub struct foo {
        pub c: libc::c_char,
        pub u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "156:25"]
    pub union C2RustUnnamed {
        pub p: *mut libc::c_void,
        pub i: opus_int32,
        pub v: opus_val32,
    }
    #[c2rust::src_loc = "135:1"]
    pub type downmix_func = Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            *mut opus_val32,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:8"]
    pub struct OpusRepacketizer {
        pub toc: libc::c_uchar,
        pub nb_frames: libc::c_int,
        pub frames: [*const libc::c_uchar; 48],
        pub len: [opus_int16; 48],
        pub framesize: libc::c_int,
    }
    #[inline]
    #[c2rust::src_loc = "154:1"]
    pub unsafe extern "C" fn align(mut i: libc::c_int) -> libc::c_int {
        let mut alignment: libc::c_uint = 8 as libc::c_ulong as libc::c_uint;
        return (i as libc::c_uint)
            .wrapping_add(alignment)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(alignment)
            .wrapping_mul(alignment) as libc::c_int;
    }
    use super::arch_h::opus_val32;
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "170:1"]
        pub fn opus_repacketizer_out_range_impl(
            rp: *mut OpusRepacketizer,
            begin: libc::c_int,
            end: libc::c_int,
            data: *mut libc::c_uchar,
            maxlen: opus_int32,
            self_delimited: libc::c_int,
            pad: libc::c_int,
        ) -> opus_int32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:51"]
pub mod structs_h {
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
    #[c2rust::src_loc = "100:9"]
    pub struct stereo_enc_state {
        pub pred_prev_Q13: [opus_int16; 2],
        pub sMid: [opus_int16; 2],
        pub sSide: [opus_int16; 2],
        pub mid_side_amp_Q0: [opus_int32; 4],
        pub smth_width_Q14: opus_int16,
        pub width_prev_Q14: opus_int16,
        pub silent_side_len: opus_int16,
        pub predIx: [[[opus_int8; 3]; 2]; 3],
        pub mid_only_flags: [opus_int8; 3],
    }
    use super::opus_types_h::{opus_int16, opus_int32, opus_int8, opus_uint8};
    use super::resampler_structs_h::silk_resampler_state_struct;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:51"]
pub mod resampler_structs_h {
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [opus_int32; 6],
        pub sFIR: C2RustUnnamed_0,
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
    pub union C2RustUnnamed_0 {
        pub i32_0: [opus_int32; 36],
        pub i16_0: [opus_int16; 36],
    }
    use super::opus_types_h::{opus_int16, opus_int32};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/structs_FLP.h:51"]
pub mod structs_FLP_h {
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
    #[c2rust::src_loc = "43:9"]
    pub struct silk_shape_state_FLP {
        pub LastGainIndex: opus_int8,
        pub HarmShapeGain_smth: libc::c_float,
        pub Tilt_smth: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:9"]
    pub struct silk_encoder {
        pub state_Fxx: [silk_encoder_state_FLP; 2],
        pub sStereo: stereo_enc_state,
        pub nBitsUsedLBRR: opus_int32,
        pub nBitsExceeded: opus_int32,
        pub nChannelsAPI: libc::c_int,
        pub nChannelsInternal: libc::c_int,
        pub nPrevChannelsInternal: libc::c_int,
        pub timeSinceSwitchAllowed_ms: libc::c_int,
        pub allowBandwidthSwitch: libc::c_int,
        pub prev_decode_only_middle: libc::c_int,
    }
    use super::opus_types_h::{opus_int32, opus_int8};
    use super::structs_h::{silk_encoder_state, stereo_enc_state};
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:33"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "95:17"]
        pub fn exp(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "143:13"]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "165:14"]
        pub fn floor(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "162:14"]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.h:33"]
pub mod entenc_h {
    use super::entcode_h::ec_enc;
    use super::opus_types_h::opus_uint32;
    extern "C" {
        #[c2rust::src_loc = "108:1"]
        pub fn ec_enc_done(_this: *mut ec_enc);
        #[c2rust::src_loc = "103:1"]
        pub fn ec_enc_shrink(_this: *mut ec_enc, _size: opus_uint32);
        #[c2rust::src_loc = "71:1"]
        pub fn ec_enc_uint(_this: *mut ec_enc, _fl: opus_uint32, _ft: opus_uint32);
        #[c2rust::src_loc = "56:1"]
        pub fn ec_enc_bit_logp(_this: *mut ec_enc, _val: libc::c_int, _logp: libc::c_uint);
        #[c2rust::src_loc = "36:1"]
        pub fn ec_enc_init(_this: *mut ec_enc, _buf: *mut libc::c_uchar, _size: opus_uint32);
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:35"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cpu_support.h:35"]
pub mod cpu_support_h {
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn opus_select_arch() -> libc::c_int {
        return 0 as libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/API.h:36"]
pub mod API_h {
    use super::control_h::silk_EncControlStruct;
    use super::entcode_h::ec_enc;
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "58:1"]
        pub fn silk_Get_Encoder_Size(encSizeBytes: *mut libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "65:1"]
        pub fn silk_InitEncoder(
            encState: *mut libc::c_void,
            arch: libc::c_int,
            encStatus: *mut silk_EncControlStruct,
        ) -> libc::c_int;
        #[c2rust::src_loc = "76:1"]
        pub fn silk_Encode(
            encState: *mut libc::c_void,
            encControl: *mut silk_EncControlStruct,
            samplesIn: *const opus_int16,
            nSamplesIn: libc::c_int,
            psRangeEnc: *mut ec_enc,
            nBytesOut: *mut opus_int32,
            prefillFlag: libc::c_int,
            activity: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/float_cast.h:38"]
pub mod float_cast_h {
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn float2int(mut x: libc::c_float) -> opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }
    #[inline]
    #[c2rust::src_loc = "137:1"]
    pub unsafe extern "C" fn FLOAT2INT16(mut x: libc::c_float) -> opus_int16 {
        x = x * 32768.0f32;
        x = if x > -(32768 as libc::c_int) as libc::c_float {
            x
        } else {
            -(32768 as libc::c_int) as libc::c_float
        };
        x = if x < 32767 as libc::c_int as libc::c_float {
            x
        } else {
            32767 as libc::c_int as libc::c_float
        };
        return float2int(x) as opus_int16;
    }
    use super::opus_types_h::{opus_int16, opus_int32};
    use super::xmmintrin_h::{_mm_cvt_ss2si, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus.h:39"]
pub mod opus_h {
    use super::opus_private_h::OpusRepacketizer;
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "778:1"]
        pub fn opus_repacketizer_init(rp: *mut OpusRepacketizer) -> *mut OpusRepacketizer;
        #[c2rust::src_loc = "838:1"]
        pub fn opus_repacketizer_cat(
            rp: *mut OpusRepacketizer,
            data: *const libc::c_uchar,
            len: opus_int32,
        ) -> libc::c_int;
        #[c2rust::src_loc = "929:1"]
        pub fn opus_packet_pad(
            data: *mut libc::c_uchar,
            len: opus_int32,
            new_len: opus_int32,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/pitch.h:41"]
pub mod pitch_h {
    #[inline]
    #[c2rust::src_loc = "159:1"]
    pub unsafe extern "C" fn celt_inner_prod_c(
        mut x: *const opus_val16,
        mut y: *const opus_val16,
        mut N: libc::c_int,
    ) -> opus_val32 {
        let mut i: libc::c_int = 0;
        let mut xy: opus_val32 = 0 as libc::c_int as opus_val32;
        i = 0 as libc::c_int;
        while i < N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
            i += 1;
        }
        return xy;
    }
    use super::arch_h::{opus_val16, opus_val32};
}
#[c2rust::header_src = "/usr/include/string.h:43"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/os_support.h:43"]
pub mod os_support_h {
    #[inline]
    #[c2rust::src_loc = "64:1"]
    pub unsafe extern "C" fn opus_free(mut ptr: *mut libc::c_void) {
        free(ptr);
    }
    #[inline]
    #[c2rust::src_loc = "47:1"]
    pub unsafe extern "C" fn opus_alloc(mut size: size_t) -> *mut libc::c_void {
        return malloc(size);
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::{free, malloc};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mathops.h:46"]
pub mod mathops_h {
    #[inline]
    #[c2rust::src_loc = "80:1"]
    pub unsafe extern "C" fn celt_maxabs16(
        mut x: *const opus_val16,
        mut len: libc::c_int,
    ) -> opus_val32 {
        let mut i: libc::c_int = 0;
        let mut maxval: opus_val16 = 0 as libc::c_int as opus_val16;
        let mut minval: opus_val16 = 0 as libc::c_int as opus_val16;
        i = 0 as libc::c_int;
        while i < len {
            maxval = if maxval > *x.offset(i as isize) {
                maxval
            } else {
                *x.offset(i as isize)
            };
            minval = if minval < *x.offset(i as isize) {
                minval
            } else {
                *x.offset(i as isize)
            };
            i += 1;
        }
        return if maxval > -minval { maxval } else { -minval };
    }
    use super::arch_h::{opus_val16, opus_val32};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:51"]
pub mod SigProc_FIX_h {
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "176:1"]
        pub fn silk_lin2log(inLin: opus_int32) -> opus_int32;
        #[c2rust::src_loc = "187:1"]
        pub fn silk_log2lin(inLog_Q7: opus_int32) -> opus_int32;
    }
}
pub use self::analysis_h::{
    run_analysis, tonality_analysis_init, tonality_analysis_reset, TonalityAnalysisState,
};
pub use self::arch_h::{opus_val16, opus_val32};
pub use self::celt_h::{
    celt_encode_with_ec, celt_encoder_get_size, celt_encoder_init, AnalysisInfo, SILKInfo,
};
pub use self::control_h::silk_EncControlStruct;
pub use self::cpu_support_h::opus_select_arch;
pub use self::entcode_h::{ec_ctx, ec_enc, ec_tell, ec_window};
use self::entenc_h::{ec_enc_bit_logp, ec_enc_done, ec_enc_init, ec_enc_shrink, ec_enc_uint};
pub use self::float_cast_h::{float2int, FLOAT2INT16};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::kiss_fft_h::{arch_fft_state, kiss_fft_state, kiss_twiddle_cpx};
use self::mathcalls_h::{exp, fabs, floor, sqrt};
pub use self::mathops_h::celt_maxabs16;
pub use self::mdct_h::mdct_lookup;
pub use self::modes_h::{OpusCustomMode, PulseCache};
use self::opus_custom_h::{opus_custom_encoder_ctl, OpusCustomEncoder};
use self::opus_h::{opus_packet_pad, opus_repacketizer_cat, opus_repacketizer_init};
pub use self::opus_private_h::{
    align, downmix_func, foo, opus_repacketizer_out_range_impl, C2RustUnnamed, OpusRepacketizer,
};
pub use self::opus_types_h::{
    opus_int16, opus_int32, opus_int64, opus_int8, opus_uint32, opus_uint8,
};
pub use self::os_support_h::{opus_alloc, opus_free};
pub use self::pitch_h::celt_inner_prod_c;
pub use self::resampler_structs_h::{
    silk_resampler_state_struct, C2RustUnnamed_0, _silk_resampler_state_struct,
};
pub use self::stdarg_h::va_list;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::stdlib_h::{free, malloc};
use self::string_h::{memcpy, memmove, memset};
pub use self::structs_FLP_h::{silk_encoder, silk_encoder_state_FLP, silk_shape_state_FLP};
pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    stereo_enc_state, SideInfoIndices,
};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __int8_t, __uint32_t, __uint8_t};
use self::API_h::{silk_Encode, silk_Get_Encoder_Size, silk_InitEncoder};
use self::SigProc_FIX_h::{silk_lin2log, silk_log2lin};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "66:8"]
pub struct OpusEncoder {
    pub celt_enc_offset: libc::c_int,
    pub silk_enc_offset: libc::c_int,
    pub silk_mode: silk_EncControlStruct,
    pub application: libc::c_int,
    pub channels: libc::c_int,
    pub delay_compensation: libc::c_int,
    pub force_channels: libc::c_int,
    pub signal_type: libc::c_int,
    pub user_bandwidth: libc::c_int,
    pub max_bandwidth: libc::c_int,
    pub user_forced_mode: libc::c_int,
    pub voice_ratio: libc::c_int,
    pub Fs: opus_int32,
    pub use_vbr: libc::c_int,
    pub vbr_constraint: libc::c_int,
    pub variable_duration: libc::c_int,
    pub bitrate_bps: opus_int32,
    pub user_bitrate_bps: opus_int32,
    pub lsb_depth: libc::c_int,
    pub encoder_buffer: libc::c_int,
    pub lfe: libc::c_int,
    pub arch: libc::c_int,
    pub use_dtx: libc::c_int,
    pub analysis: TonalityAnalysisState,
    pub stream_channels: libc::c_int,
    pub hybrid_stereo_width_Q14: opus_int16,
    pub variable_HP_smth2_Q15: opus_int32,
    pub prev_HB_gain: opus_val16,
    pub hp_mem: [opus_val32; 4],
    pub mode: libc::c_int,
    pub prev_mode: libc::c_int,
    pub prev_channels: libc::c_int,
    pub prev_framesize: libc::c_int,
    pub bandwidth: libc::c_int,
    pub auto_bandwidth: libc::c_int,
    pub silk_bw_switch: libc::c_int,
    pub first: libc::c_int,
    pub energy_masking: *mut opus_val16,
    pub width_mem: StereoWidthState,
    pub delay_buffer: [opus_val16; 960],
    pub detected_bandwidth: libc::c_int,
    pub nb_no_activity_frames: libc::c_int,
    pub peak_signal_energy: opus_val32,
    pub nonfinal_frame: libc::c_int,
    pub rangeFinal: opus_uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "60:9"]
pub struct StereoWidthState {
    pub XX: opus_val32,
    pub XY: opus_val32,
    pub YY: opus_val32,
    pub smoothed_width: opus_val16,
    pub max_follower: opus_val16,
}
#[c2rust::src_loc = "125:25"]
static mut mono_voice_bandwidth_thresholds: [opus_int32; 8] = [
    9000 as libc::c_int,
    700 as libc::c_int,
    9000 as libc::c_int,
    700 as libc::c_int,
    13500 as libc::c_int,
    1000 as libc::c_int,
    14000 as libc::c_int,
    2000 as libc::c_int,
];
#[c2rust::src_loc = "131:25"]
static mut mono_music_bandwidth_thresholds: [opus_int32; 8] = [
    9000 as libc::c_int,
    700 as libc::c_int,
    9000 as libc::c_int,
    700 as libc::c_int,
    11000 as libc::c_int,
    1000 as libc::c_int,
    12000 as libc::c_int,
    2000 as libc::c_int,
];
#[c2rust::src_loc = "137:25"]
static mut stereo_voice_bandwidth_thresholds: [opus_int32; 8] = [
    9000 as libc::c_int,
    700 as libc::c_int,
    9000 as libc::c_int,
    700 as libc::c_int,
    13500 as libc::c_int,
    1000 as libc::c_int,
    14000 as libc::c_int,
    2000 as libc::c_int,
];
#[c2rust::src_loc = "143:25"]
static mut stereo_music_bandwidth_thresholds: [opus_int32; 8] = [
    9000 as libc::c_int,
    700 as libc::c_int,
    9000 as libc::c_int,
    700 as libc::c_int,
    11000 as libc::c_int,
    1000 as libc::c_int,
    12000 as libc::c_int,
    2000 as libc::c_int,
];
#[c2rust::src_loc = "150:25"]
static mut stereo_voice_threshold: opus_int32 = 19000 as libc::c_int;
#[c2rust::src_loc = "151:25"]
static mut stereo_music_threshold: opus_int32 = 17000 as libc::c_int;
#[c2rust::src_loc = "154:25"]
static mut mode_thresholds: [[opus_int32; 2]; 2] = [
    [64000 as libc::c_int, 10000 as libc::c_int],
    [44000 as libc::c_int, 10000 as libc::c_int],
];
#[c2rust::src_loc = "160:25"]
static mut fec_thresholds: [opus_int32; 10] = [
    12000 as libc::c_int,
    1000 as libc::c_int,
    14000 as libc::c_int,
    1000 as libc::c_int,
    16000 as libc::c_int,
    1000 as libc::c_int,
    20000 as libc::c_int,
    1000 as libc::c_int,
    22000 as libc::c_int,
    1000 as libc::c_int,
];
#[no_mangle]
#[c2rust::src_loc = "168:1"]
pub unsafe extern "C" fn opus_encoder_get_size(mut channels: libc::c_int) -> libc::c_int {
    let mut silkEncSizeBytes: libc::c_int = 0;
    let mut celtEncSizeBytes: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if channels < 1 as libc::c_int || channels > 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    ret = silk_Get_Encoder_Size(&mut silkEncSizeBytes);
    if ret != 0 {
        return 0 as libc::c_int;
    }
    silkEncSizeBytes = align(silkEncSizeBytes);
    celtEncSizeBytes = celt_encoder_get_size(channels);
    return align(::core::mem::size_of::<OpusEncoder>() as libc::c_ulong as libc::c_int)
        + silkEncSizeBytes
        + celtEncSizeBytes;
}
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub unsafe extern "C" fn opus_encoder_init(
    mut st: *mut OpusEncoder,
    mut Fs: opus_int32,
    mut channels: libc::c_int,
    mut application: libc::c_int,
) -> libc::c_int {
    let mut silk_enc: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut celt_enc: *mut OpusCustomEncoder = 0 as *mut OpusCustomEncoder;
    let mut err: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut silkEncSizeBytes: libc::c_int = 0;
    if Fs != 48000 as libc::c_int
        && Fs != 24000 as libc::c_int
        && Fs != 16000 as libc::c_int
        && Fs != 12000 as libc::c_int
        && Fs != 8000 as libc::c_int
        || channels != 1 as libc::c_int && channels != 2 as libc::c_int
        || application != 2048 as libc::c_int
            && application != 2049 as libc::c_int
            && application != 2051 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    memset(
        st as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
        (opus_encoder_get_size(channels) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    ret = silk_Get_Encoder_Size(&mut silkEncSizeBytes);
    if ret != 0 {
        return -(1 as libc::c_int);
    }
    silkEncSizeBytes = align(silkEncSizeBytes);
    (*st).silk_enc_offset =
        align(::core::mem::size_of::<OpusEncoder>() as libc::c_ulong as libc::c_int);
    (*st).celt_enc_offset = (*st).silk_enc_offset + silkEncSizeBytes;
    silk_enc =
        (st as *mut libc::c_char).offset((*st).silk_enc_offset as isize) as *mut libc::c_void;
    celt_enc =
        (st as *mut libc::c_char).offset((*st).celt_enc_offset as isize) as *mut OpusCustomEncoder;
    (*st).channels = channels;
    (*st).stream_channels = (*st).channels;
    (*st).Fs = Fs;
    (*st).arch = opus_select_arch();
    ret = silk_InitEncoder(silk_enc, (*st).arch, &mut (*st).silk_mode);
    if ret != 0 {
        return -(3 as libc::c_int);
    }
    (*st).silk_mode.nChannelsAPI = channels;
    (*st).silk_mode.nChannelsInternal = channels;
    (*st).silk_mode.API_sampleRate = (*st).Fs;
    (*st).silk_mode.maxInternalSampleRate = 16000 as libc::c_int;
    (*st).silk_mode.minInternalSampleRate = 8000 as libc::c_int;
    (*st).silk_mode.desiredInternalSampleRate = 16000 as libc::c_int;
    (*st).silk_mode.payloadSize_ms = 20 as libc::c_int;
    (*st).silk_mode.bitRate = 25000 as libc::c_int;
    (*st).silk_mode.packetLossPercentage = 0 as libc::c_int;
    (*st).silk_mode.complexity = 9 as libc::c_int;
    (*st).silk_mode.useInBandFEC = 0 as libc::c_int;
    (*st).silk_mode.useDTX = 0 as libc::c_int;
    (*st).silk_mode.useCBR = 0 as libc::c_int;
    (*st).silk_mode.reducedDependency = 0 as libc::c_int;
    err = celt_encoder_init(celt_enc, Fs, channels, (*st).arch);
    if err != 0 as libc::c_int {
        return -(3 as libc::c_int);
    }
    opus_custom_encoder_ctl(celt_enc, 10016 as libc::c_int, 0 as libc::c_int);
    opus_custom_encoder_ctl(celt_enc, 4010 as libc::c_int, (*st).silk_mode.complexity);
    (*st).use_vbr = 1 as libc::c_int;
    (*st).vbr_constraint = 1 as libc::c_int;
    (*st).user_bitrate_bps = -(1000 as libc::c_int);
    (*st).bitrate_bps = 3000 as libc::c_int + Fs * channels;
    (*st).application = application;
    (*st).signal_type = -(1000 as libc::c_int);
    (*st).user_bandwidth = -(1000 as libc::c_int);
    (*st).max_bandwidth = 1105 as libc::c_int;
    (*st).force_channels = -(1000 as libc::c_int);
    (*st).user_forced_mode = -(1000 as libc::c_int);
    (*st).voice_ratio = -(1 as libc::c_int);
    (*st).encoder_buffer = (*st).Fs / 100 as libc::c_int;
    (*st).lsb_depth = 24 as libc::c_int;
    (*st).variable_duration = 5000 as libc::c_int;
    (*st).delay_compensation = (*st).Fs / 250 as libc::c_int;
    (*st).hybrid_stereo_width_Q14 = ((1 as libc::c_int) << 14 as libc::c_int) as opus_int16;
    (*st).prev_HB_gain = 1.0f32;
    (*st).variable_HP_smth2_Q15 =
        ((silk_lin2log(60 as libc::c_int) as opus_uint32) << 8 as libc::c_int) as opus_int32;
    (*st).first = 1 as libc::c_int;
    (*st).mode = 1001 as libc::c_int;
    (*st).bandwidth = 1105 as libc::c_int;
    tonality_analysis_init(&mut (*st).analysis, (*st).Fs);
    (*st).analysis.application = (*st).application;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "273:1"]
unsafe extern "C" fn gen_toc(
    mut mode: libc::c_int,
    mut framerate: libc::c_int,
    mut bandwidth: libc::c_int,
    mut channels: libc::c_int,
) -> libc::c_uchar {
    let mut period: libc::c_int = 0;
    let mut toc: libc::c_uchar = 0;
    period = 0 as libc::c_int;
    while framerate < 400 as libc::c_int {
        framerate <<= 1 as libc::c_int;
        period += 1;
    }
    if mode == 1000 as libc::c_int {
        toc = ((bandwidth - 1101 as libc::c_int) << 5 as libc::c_int) as libc::c_uchar;
        toc =
            (toc as libc::c_int | (period - 2 as libc::c_int) << 3 as libc::c_int) as libc::c_uchar;
    } else if mode == 1002 as libc::c_int {
        let mut tmp: libc::c_int = bandwidth - 1102 as libc::c_int;
        if tmp < 0 as libc::c_int {
            tmp = 0 as libc::c_int;
        }
        toc = 0x80 as libc::c_int as libc::c_uchar;
        toc = (toc as libc::c_int | tmp << 5 as libc::c_int) as libc::c_uchar;
        toc = (toc as libc::c_int | period << 3 as libc::c_int) as libc::c_uchar;
    } else {
        toc = 0x60 as libc::c_int as libc::c_uchar;
        toc = (toc as libc::c_int | (bandwidth - 1104 as libc::c_int) << 4 as libc::c_int)
            as libc::c_uchar;
        toc =
            (toc as libc::c_int | (period - 2 as libc::c_int) << 3 as libc::c_int) as libc::c_uchar;
    }
    toc = (toc as libc::c_int | ((channels == 2 as libc::c_int) as libc::c_int) << 2 as libc::c_int)
        as libc::c_uchar;
    return toc;
}
#[c2rust::src_loc = "306:1"]
unsafe extern "C" fn silk_biquad_float(
    mut in_0: *const opus_val16,
    mut B_Q28: *const opus_int32,
    mut A_Q28: *const opus_int32,
    mut S: *mut opus_val32,
    mut out: *mut opus_val16,
    len: opus_int32,
    mut stride: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut vout: opus_val32 = 0.;
    let mut inval: opus_val32 = 0.;
    let mut A: [opus_val32; 2] = [0.; 2];
    let mut B: [opus_val32; 3] = [0.; 3];
    A[0 as libc::c_int as usize] = *A_Q28.offset(0 as libc::c_int as isize) as libc::c_float
        * (1.0f32 / ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_float);
    A[1 as libc::c_int as usize] = *A_Q28.offset(1 as libc::c_int as isize) as libc::c_float
        * (1.0f32 / ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_float);
    B[0 as libc::c_int as usize] = *B_Q28.offset(0 as libc::c_int as isize) as libc::c_float
        * (1.0f32 / ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_float);
    B[1 as libc::c_int as usize] = *B_Q28.offset(1 as libc::c_int as isize) as libc::c_float
        * (1.0f32 / ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_float);
    B[2 as libc::c_int as usize] = *B_Q28.offset(2 as libc::c_int as isize) as libc::c_float
        * (1.0f32 / ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_float);
    k = 0 as libc::c_int;
    while k < len {
        inval = *in_0.offset((k * stride) as isize);
        vout = *S.offset(0 as libc::c_int as isize) + B[0 as libc::c_int as usize] * inval;
        *S.offset(0 as libc::c_int as isize) = *S.offset(1 as libc::c_int as isize)
            - vout * A[0 as libc::c_int as usize]
            + B[1 as libc::c_int as usize] * inval;
        *S.offset(1 as libc::c_int as isize) =
            -vout * A[1 as libc::c_int as usize] + B[2 as libc::c_int as usize] * inval + 1e-30f32;
        *out.offset((k * stride) as isize) = vout;
        k += 1;
    }
}
#[c2rust::src_loc = "345:1"]
unsafe extern "C" fn hp_cutoff(
    mut in_0: *const opus_val16,
    mut cutoff_Hz: opus_int32,
    mut out: *mut opus_val16,
    mut hp_mem: *mut opus_val32,
    mut len: libc::c_int,
    mut channels: libc::c_int,
    mut Fs: opus_int32,
    mut arch: libc::c_int,
) {
    let mut B_Q28: [opus_int32; 3] = [0; 3];
    let mut A_Q28: [opus_int32; 2] = [0; 2];
    let mut Fc_Q19: opus_int32 = 0;
    let mut r_Q28: opus_int32 = 0;
    let mut r_Q22: opus_int32 = 0;
    Fc_Q19 = (1.5f64 * 3.14159f64 / 1000 as libc::c_int as libc::c_double
        * ((1 as libc::c_int as opus_int64) << 19 as libc::c_int) as libc::c_double
        + 0.5f64) as opus_int32 as opus_int16 as opus_int32
        * cutoff_Hz as opus_int16 as opus_int32
        / (Fs / 1000 as libc::c_int);
    r_Q28 = (1.0f64 * ((1 as libc::c_int as opus_int64) << 28 as libc::c_int) as libc::c_double
        + 0.5f64) as opus_int32
        - (0.92f64 * ((1 as libc::c_int as opus_int64) << 9 as libc::c_int) as libc::c_double
            + 0.5f64) as opus_int32
            * Fc_Q19;
    B_Q28[0 as libc::c_int as usize] = r_Q28;
    B_Q28[1 as libc::c_int as usize] = ((-r_Q28 as opus_uint32) << 1 as libc::c_int) as opus_int32;
    B_Q28[2 as libc::c_int as usize] = r_Q28;
    r_Q22 = r_Q28 >> 6 as libc::c_int;
    A_Q28[0 as libc::c_int as usize] = (r_Q22 as opus_int64
        * ((Fc_Q19 as opus_int64 * Fc_Q19 as libc::c_long >> 16 as libc::c_int) as opus_int32
            - (2.0f64 * ((1 as libc::c_int as opus_int64) << 22 as libc::c_int) as libc::c_double
                + 0.5f64) as opus_int32) as libc::c_long
        >> 16 as libc::c_int) as opus_int32;
    A_Q28[1 as libc::c_int as usize] =
        (r_Q22 as opus_int64 * r_Q22 as libc::c_long >> 16 as libc::c_int) as opus_int32;
    silk_biquad_float(
        in_0,
        B_Q28.as_mut_ptr(),
        A_Q28.as_mut_ptr(),
        hp_mem,
        out,
        len,
        channels,
    );
    if channels == 2 as libc::c_int {
        silk_biquad_float(
            in_0.offset(1 as libc::c_int as isize),
            B_Q28.as_mut_ptr(),
            A_Q28.as_mut_ptr(),
            hp_mem.offset(2 as libc::c_int as isize),
            out.offset(1 as libc::c_int as isize),
            len,
            channels,
        );
    }
}
#[c2rust::src_loc = "404:1"]
unsafe extern "C" fn dc_reject(
    mut in_0: *const opus_val16,
    mut cutoff_Hz: opus_int32,
    mut out: *mut opus_val16,
    mut hp_mem: *mut opus_val32,
    mut len: libc::c_int,
    mut channels: libc::c_int,
    mut Fs: opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut coef: libc::c_float = 0.;
    let mut coef2: libc::c_float = 0.;
    coef = 6.3f32 * cutoff_Hz as libc::c_float / Fs as libc::c_float;
    coef2 = 1 as libc::c_int as libc::c_float - coef;
    if channels == 2 as libc::c_int {
        let mut m0: libc::c_float = 0.;
        let mut m2: libc::c_float = 0.;
        m0 = *hp_mem.offset(0 as libc::c_int as isize);
        m2 = *hp_mem.offset(2 as libc::c_int as isize);
        i = 0 as libc::c_int;
        while i < len {
            let mut x0: opus_val32 = 0.;
            let mut x1: opus_val32 = 0.;
            let mut out0: opus_val32 = 0.;
            let mut out1: opus_val32 = 0.;
            x0 = *in_0.offset((2 as libc::c_int * i + 0 as libc::c_int) as isize);
            x1 = *in_0.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
            out0 = x0 - m0;
            out1 = x1 - m2;
            m0 = coef * x0 + 1e-30f32 + coef2 * m0;
            m2 = coef * x1 + 1e-30f32 + coef2 * m2;
            *out.offset((2 as libc::c_int * i + 0 as libc::c_int) as isize) = out0;
            *out.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) = out1;
            i += 1;
        }
        *hp_mem.offset(0 as libc::c_int as isize) = m0;
        *hp_mem.offset(2 as libc::c_int as isize) = m2;
    } else {
        let mut m0_0: libc::c_float = 0.;
        m0_0 = *hp_mem.offset(0 as libc::c_int as isize);
        i = 0 as libc::c_int;
        while i < len {
            let mut x: opus_val32 = 0.;
            let mut y: opus_val32 = 0.;
            x = *in_0.offset(i as isize);
            y = x - m0_0;
            m0_0 = coef * x + 1e-30f32 + coef2 * m0_0;
            *out.offset(i as isize) = y;
            i += 1;
        }
        *hp_mem.offset(0 as libc::c_int as isize) = m0_0;
    };
}
#[c2rust::src_loc = "445:1"]
unsafe extern "C" fn stereo_fade(
    mut in_0: *const opus_val16,
    mut out: *mut opus_val16,
    mut g1: opus_val16,
    mut g2: opus_val16,
    mut overlap48: libc::c_int,
    mut frame_size: libc::c_int,
    mut channels: libc::c_int,
    mut window: *const opus_val16,
    mut Fs: opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    let mut inc: libc::c_int = 0;
    inc = 48000 as libc::c_int / Fs;
    overlap = overlap48 / inc;
    g1 = 1.0f32 - g1;
    g2 = 1.0f32 - g2;
    i = 0 as libc::c_int;
    while i < overlap {
        let mut diff: opus_val32 = 0.;
        let mut g: opus_val16 = 0.;
        let mut w: opus_val16 = 0.;
        w = *window.offset((i * inc) as isize) * *window.offset((i * inc) as isize);
        g = w * g2 + (1.0f32 - w) * g1;
        diff = 0.5f32
            * (*in_0.offset((i * channels) as isize)
                - *in_0.offset((i * channels + 1 as libc::c_int) as isize));
        diff = g * diff;
        *out.offset((i * channels) as isize) = *out.offset((i * channels) as isize) - diff;
        *out.offset((i * channels + 1 as libc::c_int) as isize) =
            *out.offset((i * channels + 1 as libc::c_int) as isize) + diff;
        i += 1;
    }
    while i < frame_size {
        let mut diff_0: opus_val32 = 0.;
        diff_0 = 0.5f32
            * (*in_0.offset((i * channels) as isize)
                - *in_0.offset((i * channels + 1 as libc::c_int) as isize));
        diff_0 = g2 * diff_0;
        *out.offset((i * channels) as isize) = *out.offset((i * channels) as isize) - diff_0;
        *out.offset((i * channels + 1 as libc::c_int) as isize) =
            *out.offset((i * channels + 1 as libc::c_int) as isize) + diff_0;
        i += 1;
    }
}
#[c2rust::src_loc = "477:1"]
unsafe extern "C" fn gain_fade(
    mut in_0: *const opus_val16,
    mut out: *mut opus_val16,
    mut g1: opus_val16,
    mut g2: opus_val16,
    mut overlap48: libc::c_int,
    mut frame_size: libc::c_int,
    mut channels: libc::c_int,
    mut window: *const opus_val16,
    mut Fs: opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut inc: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    inc = 48000 as libc::c_int / Fs;
    overlap = overlap48 / inc;
    if channels == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < overlap {
            let mut g: opus_val16 = 0.;
            let mut w: opus_val16 = 0.;
            w = *window.offset((i * inc) as isize) * *window.offset((i * inc) as isize);
            g = w * g2 + (1.0f32 - w) * g1;
            *out.offset(i as isize) = g * *in_0.offset(i as isize);
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < overlap {
            let mut g_0: opus_val16 = 0.;
            let mut w_0: opus_val16 = 0.;
            w_0 = *window.offset((i * inc) as isize) * *window.offset((i * inc) as isize);
            g_0 = w_0 * g2 + (1.0f32 - w_0) * g1;
            *out.offset((i * 2 as libc::c_int) as isize) =
                g_0 * *in_0.offset((i * 2 as libc::c_int) as isize);
            *out.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize) =
                g_0 * *in_0.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize);
            i += 1;
        }
    }
    c = 0 as libc::c_int;
    loop {
        i = overlap;
        while i < frame_size {
            *out.offset((i * channels + c) as isize) =
                g2 * *in_0.offset((i * channels + c) as isize);
            i += 1;
        }
        c += 1;
        if !(c < channels) {
            break;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "516:1"]
pub unsafe extern "C" fn opus_encoder_create(
    mut Fs: opus_int32,
    mut channels: libc::c_int,
    mut application: libc::c_int,
    mut error: *mut libc::c_int,
) -> *mut OpusEncoder {
    let mut ret: libc::c_int = 0;
    let mut st: *mut OpusEncoder = 0 as *mut OpusEncoder;
    if Fs != 48000 as libc::c_int
        && Fs != 24000 as libc::c_int
        && Fs != 16000 as libc::c_int
        && Fs != 12000 as libc::c_int
        && Fs != 8000 as libc::c_int
        || channels != 1 as libc::c_int && channels != 2 as libc::c_int
        || application != 2048 as libc::c_int
            && application != 2049 as libc::c_int
            && application != 2051 as libc::c_int
    {
        if !error.is_null() {
            *error = -(1 as libc::c_int);
        }
        return 0 as *mut OpusEncoder;
    }
    st = opus_alloc(opus_encoder_get_size(channels) as size_t) as *mut OpusEncoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as libc::c_int);
        }
        return 0 as *mut OpusEncoder;
    }
    ret = opus_encoder_init(st, Fs, channels, application);
    if !error.is_null() {
        *error = ret;
    }
    if ret != 0 as libc::c_int {
        opus_free(st as *mut libc::c_void);
        st = 0 as *mut OpusEncoder;
    }
    return st;
}
#[c2rust::src_loc = "546:1"]
unsafe extern "C" fn user_bitrate_to_bitrate(
    mut st: *mut OpusEncoder,
    mut frame_size: libc::c_int,
    mut max_data_bytes: libc::c_int,
) -> opus_int32 {
    if frame_size == 0 {
        frame_size = (*st).Fs / 400 as libc::c_int;
    }
    if (*st).user_bitrate_bps == -(1000 as libc::c_int) {
        return 60 as libc::c_int * (*st).Fs / frame_size + (*st).Fs * (*st).channels;
    } else if (*st).user_bitrate_bps == -(1 as libc::c_int) {
        return max_data_bytes * 8 as libc::c_int * (*st).Fs / frame_size;
    } else {
        return (*st).user_bitrate_bps;
    };
}
#[no_mangle]
#[c2rust::src_loc = "564:1"]
pub unsafe extern "C" fn downmix_float(
    mut _x: *const libc::c_void,
    mut y: *mut opus_val32,
    mut subframe: libc::c_int,
    mut offset: libc::c_int,
    mut c1: libc::c_int,
    mut c2: libc::c_int,
    mut C: libc::c_int,
) {
    let mut x: *const libc::c_float = 0 as *const libc::c_float;
    let mut j: libc::c_int = 0;
    x = _x as *const libc::c_float;
    j = 0 as libc::c_int;
    while j < subframe {
        *y.offset(j as isize) = *x.offset(((j + offset) * C + c1) as isize) * 32768.0f32;
        j += 1;
    }
    if c2 > -(1 as libc::c_int) {
        j = 0 as libc::c_int;
        while j < subframe {
            let ref mut fresh0 = *y.offset(j as isize);
            *fresh0 += *x.offset(((j + offset) * C + c2) as isize) * 32768.0f32;
            j += 1;
        }
    } else if c2 == -(2 as libc::c_int) {
        let mut c: libc::c_int = 0;
        c = 1 as libc::c_int;
        while c < C {
            j = 0 as libc::c_int;
            while j < subframe {
                let ref mut fresh1 = *y.offset(j as isize);
                *fresh1 += *x.offset(((j + offset) * C + c) as isize) * 32768.0f32;
                j += 1;
            }
            c += 1;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "588:1"]
pub unsafe extern "C" fn downmix_int(
    mut _x: *const libc::c_void,
    mut y: *mut opus_val32,
    mut subframe: libc::c_int,
    mut offset: libc::c_int,
    mut c1: libc::c_int,
    mut c2: libc::c_int,
    mut C: libc::c_int,
) {
    let mut x: *const opus_int16 = 0 as *const opus_int16;
    let mut j: libc::c_int = 0;
    x = _x as *const opus_int16;
    j = 0 as libc::c_int;
    while j < subframe {
        *y.offset(j as isize) = *x.offset(((j + offset) * C + c1) as isize) as opus_val32;
        j += 1;
    }
    if c2 > -(1 as libc::c_int) {
        j = 0 as libc::c_int;
        while j < subframe {
            let ref mut fresh2 = *y.offset(j as isize);
            *fresh2 += *x.offset(((j + offset) * C + c2) as isize) as libc::c_int as libc::c_float;
            j += 1;
        }
    } else if c2 == -(2 as libc::c_int) {
        let mut c: libc::c_int = 0;
        c = 1 as libc::c_int;
        while c < C {
            j = 0 as libc::c_int;
            while j < subframe {
                let ref mut fresh3 = *y.offset(j as isize);
                *fresh3 +=
                    *x.offset(((j + offset) * C + c) as isize) as libc::c_int as libc::c_float;
                j += 1;
            }
            c += 1;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "611:1"]
pub unsafe extern "C" fn frame_size_select(
    mut frame_size: opus_int32,
    mut variable_duration: libc::c_int,
    mut Fs: opus_int32,
) -> opus_int32 {
    let mut new_size: libc::c_int = 0;
    if frame_size < Fs / 400 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if variable_duration == 5000 as libc::c_int {
        new_size = frame_size;
    } else if variable_duration >= 5001 as libc::c_int && variable_duration <= 5009 as libc::c_int {
        if variable_duration <= 5005 as libc::c_int {
            new_size = (Fs / 400 as libc::c_int) << variable_duration - 5001 as libc::c_int;
        } else {
            new_size = (variable_duration - 5001 as libc::c_int - 2 as libc::c_int) * Fs
                / 50 as libc::c_int;
        }
    } else {
        return -(1 as libc::c_int);
    }
    if new_size > frame_size {
        return -(1 as libc::c_int);
    }
    if 400 as libc::c_int * new_size != Fs
        && 200 as libc::c_int * new_size != Fs
        && 100 as libc::c_int * new_size != Fs
        && 50 as libc::c_int * new_size != Fs
        && 25 as libc::c_int * new_size != Fs
        && 50 as libc::c_int * new_size != 3 as libc::c_int * Fs
        && 50 as libc::c_int * new_size != 4 as libc::c_int * Fs
        && 50 as libc::c_int * new_size != 5 as libc::c_int * Fs
        && 50 as libc::c_int * new_size != 6 as libc::c_int * Fs
    {
        return -(1 as libc::c_int);
    }
    return new_size;
}
#[no_mangle]
#[c2rust::src_loc = "636:1"]
pub unsafe extern "C" fn compute_stereo_width(
    mut pcm: *const opus_val16,
    mut frame_size: libc::c_int,
    mut Fs: opus_int32,
    mut mem: *mut StereoWidthState,
) -> opus_val16 {
    let mut xx: opus_val32 = 0.;
    let mut xy: opus_val32 = 0.;
    let mut yy: opus_val32 = 0.;
    let mut sqrt_xx: opus_val16 = 0.;
    let mut sqrt_yy: opus_val16 = 0.;
    let mut qrrt_xx: opus_val16 = 0.;
    let mut qrrt_yy: opus_val16 = 0.;
    let mut frame_rate: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut short_alpha: opus_val16 = 0.;
    frame_rate = Fs / frame_size;
    short_alpha = 1.0f32
        - 25 as libc::c_int as opus_val32 * 1.0f32
            / (if 50 as libc::c_int > frame_rate {
                50 as libc::c_int
            } else {
                frame_rate
            }) as libc::c_float;
    yy = 0 as libc::c_int as opus_val32;
    xy = yy;
    xx = xy;
    i = 0 as libc::c_int;
    while i < frame_size - 3 as libc::c_int {
        let mut pxx: opus_val32 = 0 as libc::c_int as opus_val32;
        let mut pxy: opus_val32 = 0 as libc::c_int as opus_val32;
        let mut pyy: opus_val32 = 0 as libc::c_int as opus_val32;
        let mut x: opus_val16 = 0.;
        let mut y: opus_val16 = 0.;
        x = *pcm.offset((2 as libc::c_int * i) as isize);
        y = *pcm.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
        pxx = x * x;
        pxy = x * y;
        pyy = y * y;
        x = *pcm.offset((2 as libc::c_int * i + 2 as libc::c_int) as isize);
        y = *pcm.offset((2 as libc::c_int * i + 3 as libc::c_int) as isize);
        pxx += x * x;
        pxy += x * y;
        pyy += y * y;
        x = *pcm.offset((2 as libc::c_int * i + 4 as libc::c_int) as isize);
        y = *pcm.offset((2 as libc::c_int * i + 5 as libc::c_int) as isize);
        pxx += x * x;
        pxy += x * y;
        pyy += y * y;
        x = *pcm.offset((2 as libc::c_int * i + 6 as libc::c_int) as isize);
        y = *pcm.offset((2 as libc::c_int * i + 7 as libc::c_int) as isize);
        pxx += x * x;
        pxy += x * y;
        pyy += y * y;
        xx += pxx;
        xy += pxy;
        yy += pyy;
        i += 4 as libc::c_int;
    }
    if !(xx < 1e9f32) || xx != xx || !(yy < 1e9f32) || yy != yy {
        yy = 0 as libc::c_int as opus_val32;
        xx = yy;
        xy = xx;
    }
    (*mem).XX += short_alpha * (xx - (*mem).XX);
    (*mem).XY += short_alpha * (xy - (*mem).XY);
    (*mem).YY += short_alpha * (yy - (*mem).YY);
    (*mem).XX = if 0 as libc::c_int as libc::c_float > (*mem).XX {
        0 as libc::c_int as libc::c_float
    } else {
        (*mem).XX
    };
    (*mem).XY = if 0 as libc::c_int as libc::c_float > (*mem).XY {
        0 as libc::c_int as libc::c_float
    } else {
        (*mem).XY
    };
    (*mem).YY = if 0 as libc::c_int as libc::c_float > (*mem).YY {
        0 as libc::c_int as libc::c_float
    } else {
        (*mem).YY
    };
    if (if (*mem).XX > (*mem).YY {
        (*mem).XX
    } else {
        (*mem).YY
    }) > 8e-4f32
    {
        let mut corr: opus_val16 = 0.;
        let mut ldiff: opus_val16 = 0.;
        let mut width: opus_val16 = 0.;
        sqrt_xx = sqrt((*mem).XX as libc::c_double) as libc::c_float;
        sqrt_yy = sqrt((*mem).YY as libc::c_double) as libc::c_float;
        qrrt_xx = sqrt(sqrt_xx as libc::c_double) as libc::c_float;
        qrrt_yy = sqrt(sqrt_yy as libc::c_double) as libc::c_float;
        (*mem).XY = if (*mem).XY < sqrt_xx * sqrt_yy {
            (*mem).XY
        } else {
            sqrt_xx * sqrt_yy
        };
        corr = (*mem).XY / (1e-15f32 + sqrt_xx * sqrt_yy);
        ldiff = 1.0f32 * fabs((qrrt_xx - qrrt_yy) as libc::c_double) as libc::c_float
            / (1e-15f32 + qrrt_xx + qrrt_yy);
        width = sqrt((1.0f32 - corr * corr) as libc::c_double) as libc::c_float * ldiff;
        (*mem).smoothed_width += (width - (*mem).smoothed_width) / frame_rate as libc::c_float;
        (*mem).max_follower = if (*mem).max_follower - 0.02f32 / frame_rate as libc::c_float
            > (*mem).smoothed_width
        {
            (*mem).max_follower - 0.02f32 / frame_rate as libc::c_float
        } else {
            (*mem).smoothed_width
        };
    }
    return if 1.0f32 < 20 as libc::c_int as opus_val32 * (*mem).max_follower {
        1.0f32
    } else {
        20 as libc::c_int as opus_val32 * (*mem).max_follower
    };
}
#[c2rust::src_loc = "718:1"]
unsafe extern "C" fn decide_fec(
    mut useInBandFEC: libc::c_int,
    mut PacketLoss_perc: libc::c_int,
    mut last_fec: libc::c_int,
    mut mode: libc::c_int,
    mut bandwidth: *mut libc::c_int,
    mut rate: opus_int32,
) -> libc::c_int {
    let mut orig_bandwidth: libc::c_int = 0;
    if useInBandFEC == 0 || PacketLoss_perc == 0 as libc::c_int || mode == 1002 as libc::c_int {
        return 0 as libc::c_int;
    }
    orig_bandwidth = *bandwidth;
    loop {
        let mut hysteresis: opus_int32 = 0;
        let mut LBRR_rate_thres_bps: opus_int32 = 0;
        LBRR_rate_thres_bps =
            fec_thresholds[(2 as libc::c_int * (*bandwidth - 1101 as libc::c_int)) as usize];
        hysteresis = fec_thresholds
            [(2 as libc::c_int * (*bandwidth - 1101 as libc::c_int) + 1 as libc::c_int) as usize];
        if last_fec == 1 as libc::c_int {
            LBRR_rate_thres_bps -= hysteresis;
        }
        if last_fec == 0 as libc::c_int {
            LBRR_rate_thres_bps += hysteresis;
        }
        LBRR_rate_thres_bps = ((LBRR_rate_thres_bps
            * (125 as libc::c_int
                - (if PacketLoss_perc < 25 as libc::c_int {
                    PacketLoss_perc
                } else {
                    25 as libc::c_int
                }))) as libc::c_long
            * (0.01f64 * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as opus_int32 as opus_int16 as opus_int64
            >> 16 as libc::c_int) as opus_int32;
        if rate > LBRR_rate_thres_bps {
            return 1 as libc::c_int;
        } else if PacketLoss_perc <= 5 as libc::c_int {
            return 0 as libc::c_int;
        } else {
            if !(*bandwidth > 1101 as libc::c_int) {
                break;
            }
            *bandwidth -= 1;
        }
    }
    *bandwidth = orig_bandwidth;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "751:1"]
unsafe extern "C" fn compute_silk_rate_for_hybrid(
    mut rate: libc::c_int,
    mut bandwidth: libc::c_int,
    mut frame20ms: libc::c_int,
    mut vbr: libc::c_int,
    mut fec: libc::c_int,
    mut channels: libc::c_int,
) -> libc::c_int {
    let mut entry: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut silk_rate: libc::c_int = 0;
    static mut rate_table: [[libc::c_int; 5]; 7] = [
        [
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ],
        [
            12000 as libc::c_int,
            10000 as libc::c_int,
            10000 as libc::c_int,
            11000 as libc::c_int,
            11000 as libc::c_int,
        ],
        [
            16000 as libc::c_int,
            13500 as libc::c_int,
            13500 as libc::c_int,
            15000 as libc::c_int,
            15000 as libc::c_int,
        ],
        [
            20000 as libc::c_int,
            16000 as libc::c_int,
            16000 as libc::c_int,
            18000 as libc::c_int,
            18000 as libc::c_int,
        ],
        [
            24000 as libc::c_int,
            18000 as libc::c_int,
            18000 as libc::c_int,
            21000 as libc::c_int,
            21000 as libc::c_int,
        ],
        [
            32000 as libc::c_int,
            22000 as libc::c_int,
            22000 as libc::c_int,
            28000 as libc::c_int,
            28000 as libc::c_int,
        ],
        [
            64000 as libc::c_int,
            38000 as libc::c_int,
            38000 as libc::c_int,
            50000 as libc::c_int,
            50000 as libc::c_int,
        ],
    ];
    rate /= channels;
    entry = 1 as libc::c_int + frame20ms + 2 as libc::c_int * fec;
    N = (::core::mem::size_of::<[[libc::c_int; 5]; 7]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong)
        as libc::c_int;
    i = 1 as libc::c_int;
    while i < N {
        if rate_table[i as usize][0 as libc::c_int as usize] > rate {
            break;
        }
        i += 1;
    }
    if i == N {
        silk_rate = rate_table[(i - 1 as libc::c_int) as usize][entry as usize];
        silk_rate += (rate
            - rate_table[(i - 1 as libc::c_int) as usize][0 as libc::c_int as usize])
            / 2 as libc::c_int;
    } else {
        let mut lo: opus_int32 = 0;
        let mut hi: opus_int32 = 0;
        let mut x0: opus_int32 = 0;
        let mut x1: opus_int32 = 0;
        lo = rate_table[(i - 1 as libc::c_int) as usize][entry as usize];
        hi = rate_table[i as usize][entry as usize];
        x0 = rate_table[(i - 1 as libc::c_int) as usize][0 as libc::c_int as usize];
        x1 = rate_table[i as usize][0 as libc::c_int as usize];
        silk_rate = (lo * (x1 - rate) + hi * (rate - x0)) / (x1 - x0);
    }
    if vbr == 0 {
        silk_rate += 100 as libc::c_int;
    }
    if bandwidth == 1104 as libc::c_int {
        silk_rate += 300 as libc::c_int;
    }
    silk_rate *= channels;
    if channels == 2 as libc::c_int && rate >= 12000 as libc::c_int {
        silk_rate -= 1000 as libc::c_int;
    }
    return silk_rate;
}
#[c2rust::src_loc = "805:1"]
unsafe extern "C" fn compute_equiv_rate(
    mut bitrate: opus_int32,
    mut channels: libc::c_int,
    mut frame_rate: libc::c_int,
    mut vbr: libc::c_int,
    mut mode: libc::c_int,
    mut complexity: libc::c_int,
    mut loss: libc::c_int,
) -> opus_int32 {
    let mut equiv: opus_int32 = 0;
    equiv = bitrate;
    if frame_rate > 50 as libc::c_int {
        equiv -=
            (40 as libc::c_int * channels + 20 as libc::c_int) * (frame_rate - 50 as libc::c_int);
    }
    if vbr == 0 {
        equiv -= equiv / 12 as libc::c_int;
    }
    equiv = equiv * (90 as libc::c_int + complexity) / 100 as libc::c_int;
    if mode == 1000 as libc::c_int || mode == 1001 as libc::c_int {
        if complexity < 2 as libc::c_int {
            equiv = equiv * 4 as libc::c_int / 5 as libc::c_int;
        }
        equiv -= equiv * loss / (6 as libc::c_int * loss + 10 as libc::c_int);
    } else if mode == 1002 as libc::c_int {
        if complexity < 5 as libc::c_int {
            equiv = equiv * 9 as libc::c_int / 10 as libc::c_int;
        }
    } else {
        equiv -= equiv * loss / (12 as libc::c_int * loss + 20 as libc::c_int);
    }
    return equiv;
}
#[no_mangle]
#[c2rust::src_loc = "840:1"]
pub unsafe extern "C" fn is_digital_silence(
    mut pcm: *const opus_val16,
    mut frame_size: libc::c_int,
    mut channels: libc::c_int,
    mut lsb_depth: libc::c_int,
) -> libc::c_int {
    let mut silence: libc::c_int = 0 as libc::c_int;
    let mut sample_max: opus_val32 = 0 as libc::c_int as opus_val32;
    sample_max = celt_maxabs16(pcm, frame_size * channels);
    silence = (sample_max
        <= 1 as libc::c_int as opus_val16 / ((1 as libc::c_int) << lsb_depth) as libc::c_float)
        as libc::c_int;
    return silence;
}
#[c2rust::src_loc = "887:1"]
unsafe extern "C" fn compute_frame_energy(
    mut pcm: *const opus_val16,
    mut frame_size: libc::c_int,
    mut channels: libc::c_int,
    mut arch: libc::c_int,
) -> opus_val32 {
    let mut len: libc::c_int = frame_size * channels;
    return celt_inner_prod_c(pcm, pcm, len) / len as libc::c_float;
}
#[c2rust::src_loc = "895:1"]
unsafe extern "C" fn decide_dtx_mode(
    mut activity_probability: libc::c_float,
    mut nb_no_activity_frames: *mut libc::c_int,
    mut peak_signal_energy: opus_val32,
    mut pcm: *const opus_val16,
    mut frame_size: libc::c_int,
    mut channels: libc::c_int,
    mut is_silence: libc::c_int,
    mut arch: libc::c_int,
) -> libc::c_int {
    let mut noise_energy: opus_val32 = 0.;
    if is_silence == 0 {
        if activity_probability < 0.1f32 {
            noise_energy = compute_frame_energy(pcm, frame_size, channels, arch);
            is_silence = (peak_signal_energy >= 316.23f32 * noise_energy) as libc::c_int;
        }
    }
    if is_silence != 0 {
        *nb_no_activity_frames += 1;
        if *nb_no_activity_frames > 10 as libc::c_int {
            if *nb_no_activity_frames <= 10 as libc::c_int + 20 as libc::c_int {
                return 1 as libc::c_int;
            } else {
                *nb_no_activity_frames = 10 as libc::c_int;
            }
        }
    } else {
        *nb_no_activity_frames = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "939:1"]
unsafe extern "C" fn encode_multiframe_packet(
    mut st: *mut OpusEncoder,
    mut pcm: *const opus_val16,
    mut nb_frames: libc::c_int,
    mut frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut out_data_bytes: opus_int32,
    mut to_celt: libc::c_int,
    mut lsb_depth: libc::c_int,
    mut float_api: libc::c_int,
) -> opus_int32 {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut bak_mode: libc::c_int = 0;
    let mut bak_bandwidth: libc::c_int = 0;
    let mut bak_channels: libc::c_int = 0;
    let mut bak_to_mono: libc::c_int = 0;
    let mut max_header_bytes: libc::c_int = 0;
    let mut bytes_per_frame: opus_int32 = 0;
    let mut cbr_bytes: opus_int32 = 0;
    let mut repacketize_len: opus_int32 = 0;
    let mut tmp_len: libc::c_int = 0;
    max_header_bytes = if nb_frames == 2 as libc::c_int {
        3 as libc::c_int
    } else {
        2 as libc::c_int + (nb_frames - 1 as libc::c_int) * 2 as libc::c_int
    };
    if (*st).use_vbr != 0 || (*st).user_bitrate_bps == -(1 as libc::c_int) {
        repacketize_len = out_data_bytes;
    } else {
        cbr_bytes = 3 as libc::c_int * (*st).bitrate_bps
            / (3 as libc::c_int * 8 as libc::c_int * (*st).Fs / (frame_size * nb_frames));
        repacketize_len = if cbr_bytes < out_data_bytes {
            cbr_bytes
        } else {
            out_data_bytes
        };
    }
    bytes_per_frame = if (1276 as libc::c_int)
        < 1 as libc::c_int + (repacketize_len - max_header_bytes) / nb_frames
    {
        1276 as libc::c_int
    } else {
        1 as libc::c_int + (repacketize_len - max_header_bytes) / nb_frames
    };
    let vla = (nb_frames * bytes_per_frame) as usize;
    let mut tmp_data: Vec<libc::c_uchar> = ::std::vec::from_elem(0, vla);
    let mut rp: [OpusRepacketizer; 1] = [OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [0 as *const libc::c_uchar; 48],
        len: [0; 48],
        framesize: 0,
    }; 1];
    opus_repacketizer_init(rp.as_mut_ptr());
    bak_mode = (*st).user_forced_mode;
    bak_bandwidth = (*st).user_bandwidth;
    bak_channels = (*st).force_channels;
    (*st).user_forced_mode = (*st).mode;
    (*st).user_bandwidth = (*st).bandwidth;
    (*st).force_channels = (*st).stream_channels;
    bak_to_mono = (*st).silk_mode.toMono;
    if bak_to_mono != 0 {
        (*st).force_channels = 1 as libc::c_int;
    } else {
        (*st).prev_channels = (*st).stream_channels;
    }
    i = 0 as libc::c_int;
    while i < nb_frames {
        (*st).silk_mode.toMono = 0 as libc::c_int;
        (*st).nonfinal_frame = (i < nb_frames - 1 as libc::c_int) as libc::c_int;
        if to_celt != 0 && i == nb_frames - 1 as libc::c_int {
            (*st).user_forced_mode = 1002 as libc::c_int;
        }
        tmp_len = opus_encode_native(
            st,
            pcm.offset((i * ((*st).channels * frame_size)) as isize),
            frame_size,
            tmp_data.as_mut_ptr().offset((i * bytes_per_frame) as isize),
            bytes_per_frame,
            lsb_depth,
            0 as *const libc::c_void,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            None,
            float_api,
        );
        if tmp_len < 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        ret = opus_repacketizer_cat(
            rp.as_mut_ptr(),
            tmp_data.as_mut_ptr().offset((i * bytes_per_frame) as isize),
            tmp_len,
        );
        if ret < 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        i += 1;
    }
    ret = opus_repacketizer_out_range_impl(
        rp.as_mut_ptr(),
        0 as libc::c_int,
        nb_frames,
        data,
        repacketize_len,
        0 as libc::c_int,
        ((*st).use_vbr == 0) as libc::c_int,
    );
    if ret < 0 as libc::c_int {
        return -(3 as libc::c_int);
    }
    (*st).user_forced_mode = bak_mode;
    (*st).user_bandwidth = bak_bandwidth;
    (*st).force_channels = bak_channels;
    (*st).silk_mode.toMono = bak_to_mono;
    return ret;
}
#[c2rust::src_loc = "1038:1"]
unsafe extern "C" fn compute_redundancy_bytes(
    mut max_data_bytes: opus_int32,
    mut bitrate_bps: opus_int32,
    mut frame_rate: libc::c_int,
    mut channels: libc::c_int,
) -> libc::c_int {
    let mut redundancy_bytes_cap: libc::c_int = 0;
    let mut redundancy_bytes: libc::c_int = 0;
    let mut redundancy_rate: opus_int32 = 0;
    let mut base_bits: libc::c_int = 0;
    let mut available_bits: opus_int32 = 0;
    base_bits = 40 as libc::c_int * channels + 20 as libc::c_int;
    redundancy_rate = bitrate_bps + base_bits * (200 as libc::c_int - frame_rate);
    redundancy_rate = 3 as libc::c_int * redundancy_rate / 2 as libc::c_int;
    redundancy_bytes = redundancy_rate / 1600 as libc::c_int;
    available_bits = max_data_bytes * 8 as libc::c_int - 2 as libc::c_int * base_bits;
    redundancy_bytes_cap = (available_bits * 240 as libc::c_int
        / (240 as libc::c_int + 48000 as libc::c_int / frame_rate)
        + base_bits)
        / 8 as libc::c_int;
    redundancy_bytes = if redundancy_bytes < redundancy_bytes_cap {
        redundancy_bytes
    } else {
        redundancy_bytes_cap
    };
    if redundancy_bytes > 4 as libc::c_int + 8 as libc::c_int * channels {
        redundancy_bytes = if (257 as libc::c_int) < redundancy_bytes {
            257 as libc::c_int
        } else {
            redundancy_bytes
        };
    } else {
        redundancy_bytes = 0 as libc::c_int;
    }
    return redundancy_bytes;
}
#[no_mangle]
#[c2rust::src_loc = "1066:1"]
pub unsafe extern "C" fn opus_encode_native(
    mut st: *mut OpusEncoder,
    mut pcm: *const opus_val16,
    mut frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut out_data_bytes: opus_int32,
    mut lsb_depth: libc::c_int,
    mut analysis_pcm: *const libc::c_void,
    mut analysis_size: opus_int32,
    mut c1: libc::c_int,
    mut c2: libc::c_int,
    mut analysis_channels: libc::c_int,
    mut downmix: downmix_func,
    mut float_api: libc::c_int,
) -> opus_int32 {
    let mut silk_enc: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut celt_enc: *mut OpusCustomEncoder = 0 as *mut OpusCustomEncoder;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut nBytes: opus_int32 = 0;
    let mut enc: ec_enc = ec_enc {
        buf: 0 as *mut libc::c_uchar,
        storage: 0,
        end_offs: 0,
        end_window: 0,
        nend_bits: 0,
        nbits_total: 0,
        offs: 0,
        rng: 0,
        val: 0,
        ext: 0,
        rem: 0,
        error: 0,
    };
    let mut bytes_target: libc::c_int = 0;
    let mut prefill: libc::c_int = 0 as libc::c_int;
    let mut start_band: libc::c_int = 0 as libc::c_int;
    let mut redundancy: libc::c_int = 0 as libc::c_int;
    let mut redundancy_bytes: libc::c_int = 0 as libc::c_int;
    let mut celt_to_silk: libc::c_int = 0 as libc::c_int;
    let mut nb_compr_bytes: libc::c_int = 0;
    let mut to_celt: libc::c_int = 0 as libc::c_int;
    let mut redundant_rng: opus_uint32 = 0 as libc::c_int as opus_uint32;
    let mut cutoff_Hz: libc::c_int = 0;
    let mut hp_freq_smth1: libc::c_int = 0;
    let mut voice_est: libc::c_int = 0;
    let mut equiv_rate: opus_int32 = 0;
    let mut delay_compensation: libc::c_int = 0;
    let mut frame_rate: libc::c_int = 0;
    let mut max_rate: opus_int32 = 0;
    let mut curr_bandwidth: libc::c_int = 0;
    let mut HB_gain: opus_val16 = 0.;
    let mut max_data_bytes: opus_int32 = 0;
    let mut total_buffer: libc::c_int = 0;
    let mut stereo_width: opus_val16 = 0.;
    let mut celt_mode: *const OpusCustomMode = 0 as *const OpusCustomMode;
    let mut analysis_info: AnalysisInfo = AnalysisInfo {
        valid: 0,
        tonality: 0.,
        tonality_slope: 0.,
        noisiness: 0.,
        activity: 0.,
        music_prob: 0.,
        music_prob_min: 0.,
        music_prob_max: 0.,
        bandwidth: 0,
        activity_probability: 0.,
        max_pitch_ratio: 0.,
        leak_boost: [0; 19],
    };
    let mut analysis_read_pos_bak: libc::c_int = -(1 as libc::c_int);
    let mut analysis_read_subframe_bak: libc::c_int = -(1 as libc::c_int);
    let mut is_silence: libc::c_int = 0 as libc::c_int;
    max_data_bytes = if (1276 as libc::c_int) < out_data_bytes {
        1276 as libc::c_int
    } else {
        out_data_bytes
    };
    (*st).rangeFinal = 0 as libc::c_int as opus_uint32;
    if frame_size <= 0 as libc::c_int || max_data_bytes <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if max_data_bytes == 1 as libc::c_int && (*st).Fs == frame_size * 10 as libc::c_int {
        return -(2 as libc::c_int);
    }
    silk_enc =
        (st as *mut libc::c_char).offset((*st).silk_enc_offset as isize) as *mut libc::c_void;
    celt_enc =
        (st as *mut libc::c_char).offset((*st).celt_enc_offset as isize) as *mut OpusCustomEncoder;
    if (*st).application == 2051 as libc::c_int {
        delay_compensation = 0 as libc::c_int;
    } else {
        delay_compensation = (*st).delay_compensation;
    }
    lsb_depth = if lsb_depth < (*st).lsb_depth {
        lsb_depth
    } else {
        (*st).lsb_depth
    };
    opus_custom_encoder_ctl(
        celt_enc,
        10015 as libc::c_int,
        (&mut celt_mode as *mut *const OpusCustomMode).offset(
            (&mut celt_mode as *mut *const OpusCustomMode)
                .offset_from(&mut celt_mode as *mut *const OpusCustomMode)
                as libc::c_long as isize,
        ),
    );
    analysis_info.valid = 0 as libc::c_int;
    if (*st).silk_mode.complexity >= 7 as libc::c_int && (*st).Fs >= 16000 as libc::c_int {
        is_silence = is_digital_silence(pcm, frame_size, (*st).channels, lsb_depth);
        analysis_read_pos_bak = (*st).analysis.read_pos;
        analysis_read_subframe_bak = (*st).analysis.read_subframe;
        run_analysis(
            &mut (*st).analysis,
            celt_mode,
            analysis_pcm,
            analysis_size,
            frame_size,
            c1,
            c2,
            analysis_channels,
            (*st).Fs,
            lsb_depth,
            downmix,
            &mut analysis_info,
        );
        if is_silence == 0 && analysis_info.activity_probability > 0.1f32 {
            (*st).peak_signal_energy = if 0.999f32 * (*st).peak_signal_energy
                > compute_frame_energy(pcm, frame_size, (*st).channels, (*st).arch)
            {
                0.999f32 * (*st).peak_signal_energy
            } else {
                compute_frame_energy(pcm, frame_size, (*st).channels, (*st).arch)
            };
        }
    } else if (*st).analysis.initialized != 0 {
        tonality_analysis_reset(&mut (*st).analysis);
    }
    if is_silence == 0 {
        (*st).voice_ratio = -(1 as libc::c_int);
    }
    (*st).detected_bandwidth = 0 as libc::c_int;
    if analysis_info.valid != 0 {
        let mut analysis_bandwidth: libc::c_int = 0;
        if (*st).signal_type == -(1000 as libc::c_int) {
            let mut prob: libc::c_float = 0.;
            if (*st).prev_mode == 0 as libc::c_int {
                prob = analysis_info.music_prob;
            } else if (*st).prev_mode == 1002 as libc::c_int {
                prob = analysis_info.music_prob_max;
            } else {
                prob = analysis_info.music_prob_min;
            }
            (*st).voice_ratio = floor(
                0.5f64
                    + (100 as libc::c_int as libc::c_float
                        * (1 as libc::c_int as libc::c_float - prob))
                        as libc::c_double,
            ) as libc::c_int;
        }
        analysis_bandwidth = analysis_info.bandwidth;
        if analysis_bandwidth <= 12 as libc::c_int {
            (*st).detected_bandwidth = 1101 as libc::c_int;
        } else if analysis_bandwidth <= 14 as libc::c_int {
            (*st).detected_bandwidth = 1102 as libc::c_int;
        } else if analysis_bandwidth <= 16 as libc::c_int {
            (*st).detected_bandwidth = 1103 as libc::c_int;
        } else if analysis_bandwidth <= 18 as libc::c_int {
            (*st).detected_bandwidth = 1104 as libc::c_int;
        } else {
            (*st).detected_bandwidth = 1105 as libc::c_int;
        }
    }
    if (*st).channels == 2 as libc::c_int && (*st).force_channels != 1 as libc::c_int {
        stereo_width = compute_stereo_width(pcm, frame_size, (*st).Fs, &mut (*st).width_mem);
    } else {
        stereo_width = 0 as libc::c_int as opus_val16;
    }
    total_buffer = delay_compensation;
    (*st).bitrate_bps = user_bitrate_to_bitrate(st, frame_size, max_data_bytes);
    frame_rate = (*st).Fs / frame_size;
    if (*st).use_vbr == 0 {
        let mut cbrBytes: libc::c_int = 0;
        let mut frame_rate12: libc::c_int = 12 as libc::c_int * (*st).Fs / frame_size;
        cbrBytes = if (12 as libc::c_int * (*st).bitrate_bps / 8 as libc::c_int
            + frame_rate12 / 2 as libc::c_int)
            / frame_rate12
            < max_data_bytes
        {
            (12 as libc::c_int * (*st).bitrate_bps / 8 as libc::c_int
                + frame_rate12 / 2 as libc::c_int)
                / frame_rate12
        } else {
            max_data_bytes
        };
        (*st).bitrate_bps = cbrBytes * frame_rate12 * 8 as libc::c_int / 12 as libc::c_int;
        max_data_bytes = if 1 as libc::c_int > cbrBytes {
            1 as libc::c_int
        } else {
            cbrBytes
        };
    }
    if max_data_bytes < 3 as libc::c_int
        || (*st).bitrate_bps < 3 as libc::c_int * frame_rate * 8 as libc::c_int
        || frame_rate < 50 as libc::c_int
            && (max_data_bytes * frame_rate < 300 as libc::c_int
                || (*st).bitrate_bps < 2400 as libc::c_int)
    {
        let mut tocmode: libc::c_int = (*st).mode;
        let mut bw: libc::c_int = if (*st).bandwidth == 0 as libc::c_int {
            1101 as libc::c_int
        } else {
            (*st).bandwidth
        };
        let mut packet_code: libc::c_int = 0 as libc::c_int;
        let mut num_multiframes: libc::c_int = 0 as libc::c_int;
        if tocmode == 0 as libc::c_int {
            tocmode = 1000 as libc::c_int;
        }
        if frame_rate > 100 as libc::c_int {
            tocmode = 1002 as libc::c_int;
        }
        if frame_rate == 25 as libc::c_int && tocmode != 1000 as libc::c_int {
            frame_rate = 50 as libc::c_int;
            packet_code = 1 as libc::c_int;
        }
        if frame_rate <= 16 as libc::c_int {
            if out_data_bytes == 1 as libc::c_int
                || tocmode == 1000 as libc::c_int && frame_rate != 10 as libc::c_int
            {
                tocmode = 1000 as libc::c_int;
                packet_code = (frame_rate <= 12 as libc::c_int) as libc::c_int;
                frame_rate = if frame_rate == 12 as libc::c_int {
                    25 as libc::c_int
                } else {
                    16 as libc::c_int
                };
            } else {
                num_multiframes = 50 as libc::c_int / frame_rate;
                frame_rate = 50 as libc::c_int;
                packet_code = 3 as libc::c_int;
            }
        }
        if tocmode == 1000 as libc::c_int && bw > 1103 as libc::c_int {
            bw = 1103 as libc::c_int;
        } else if tocmode == 1002 as libc::c_int && bw == 1102 as libc::c_int {
            bw = 1101 as libc::c_int;
        } else if tocmode == 1001 as libc::c_int && bw <= 1104 as libc::c_int {
            bw = 1104 as libc::c_int;
        }
        *data.offset(0 as libc::c_int as isize) =
            gen_toc(tocmode, frame_rate, bw, (*st).stream_channels);
        let ref mut fresh4 = *data.offset(0 as libc::c_int as isize);
        *fresh4 = (*fresh4 as libc::c_int | packet_code) as libc::c_uchar;
        ret = if packet_code <= 1 as libc::c_int {
            1 as libc::c_int
        } else {
            2 as libc::c_int
        };
        max_data_bytes = if max_data_bytes > ret {
            max_data_bytes
        } else {
            ret
        };
        if packet_code == 3 as libc::c_int {
            *data.offset(1 as libc::c_int as isize) = num_multiframes as libc::c_uchar;
        }
        if (*st).use_vbr == 0 {
            ret = opus_packet_pad(data, ret, max_data_bytes);
            if ret == 0 as libc::c_int {
                ret = max_data_bytes;
            } else {
                ret = -(3 as libc::c_int);
            }
        }
        return ret;
    }
    max_rate = frame_rate * max_data_bytes * 8 as libc::c_int;
    equiv_rate = compute_equiv_rate(
        (*st).bitrate_bps,
        (*st).channels,
        (*st).Fs / frame_size,
        (*st).use_vbr,
        0 as libc::c_int,
        (*st).silk_mode.complexity,
        (*st).silk_mode.packetLossPercentage,
    );
    if (*st).signal_type == 3001 as libc::c_int {
        voice_est = 127 as libc::c_int;
    } else if (*st).signal_type == 3002 as libc::c_int {
        voice_est = 0 as libc::c_int;
    } else if (*st).voice_ratio >= 0 as libc::c_int {
        voice_est = (*st).voice_ratio * 327 as libc::c_int >> 8 as libc::c_int;
        if (*st).application == 2049 as libc::c_int {
            voice_est = if voice_est < 115 as libc::c_int {
                voice_est
            } else {
                115 as libc::c_int
            };
        }
    } else if (*st).application == 2048 as libc::c_int {
        voice_est = 115 as libc::c_int;
    } else {
        voice_est = 48 as libc::c_int;
    }
    if (*st).force_channels != -(1000 as libc::c_int) && (*st).channels == 2 as libc::c_int {
        (*st).stream_channels = (*st).force_channels;
    } else if (*st).channels == 2 as libc::c_int {
        let mut stereo_threshold: opus_int32 = 0;
        stereo_threshold = stereo_music_threshold
            + (voice_est * voice_est * (stereo_voice_threshold - stereo_music_threshold)
                >> 14 as libc::c_int);
        if (*st).stream_channels == 2 as libc::c_int {
            stereo_threshold -= 1000 as libc::c_int;
        } else {
            stereo_threshold += 1000 as libc::c_int;
        }
        (*st).stream_channels = if equiv_rate > stereo_threshold {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        };
    } else {
        (*st).stream_channels = (*st).channels;
    }
    equiv_rate = compute_equiv_rate(
        (*st).bitrate_bps,
        (*st).stream_channels,
        (*st).Fs / frame_size,
        (*st).use_vbr,
        0 as libc::c_int,
        (*st).silk_mode.complexity,
        (*st).silk_mode.packetLossPercentage,
    );
    (*st).silk_mode.useDTX =
        ((*st).use_dtx != 0 && !(analysis_info.valid != 0 || is_silence != 0)) as libc::c_int;
    if (*st).application == 2051 as libc::c_int {
        (*st).mode = 1002 as libc::c_int;
    } else if (*st).user_forced_mode == -(1000 as libc::c_int) {
        let mut mode_voice: opus_int32 = 0;
        let mut mode_music: opus_int32 = 0;
        let mut threshold: opus_int32 = 0;
        mode_voice = ((1.0f32 - stereo_width)
            * mode_thresholds[0 as libc::c_int as usize][0 as libc::c_int as usize]
                as libc::c_float
            + stereo_width
                * mode_thresholds[1 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_float) as opus_int32;
        mode_music = ((1.0f32 - stereo_width)
            * mode_thresholds[1 as libc::c_int as usize][1 as libc::c_int as usize]
                as libc::c_float
            + stereo_width
                * mode_thresholds[1 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_float) as opus_int32;
        threshold =
            mode_music + (voice_est * voice_est * (mode_voice - mode_music) >> 14 as libc::c_int);
        if (*st).application == 2048 as libc::c_int {
            threshold += 8000 as libc::c_int;
        }
        if (*st).prev_mode == 1002 as libc::c_int {
            threshold -= 4000 as libc::c_int;
        } else if (*st).prev_mode > 0 as libc::c_int {
            threshold += 4000 as libc::c_int;
        }
        (*st).mode = if equiv_rate >= threshold {
            1002 as libc::c_int
        } else {
            1000 as libc::c_int
        };
        if (*st).silk_mode.useInBandFEC != 0
            && (*st).silk_mode.packetLossPercentage
                > 128 as libc::c_int - voice_est >> 4 as libc::c_int
        {
            (*st).mode = 1000 as libc::c_int;
        }
        if (*st).silk_mode.useDTX != 0 && voice_est > 100 as libc::c_int {
            (*st).mode = 1000 as libc::c_int;
        }
        if max_data_bytes
            < (if frame_rate > 50 as libc::c_int {
                9000 as libc::c_int
            } else {
                6000 as libc::c_int
            }) * frame_size
                / ((*st).Fs * 8 as libc::c_int)
        {
            (*st).mode = 1002 as libc::c_int;
        }
    } else {
        (*st).mode = (*st).user_forced_mode;
    }
    if (*st).mode != 1002 as libc::c_int && frame_size < (*st).Fs / 100 as libc::c_int {
        (*st).mode = 1002 as libc::c_int;
    }
    if (*st).lfe != 0 {
        (*st).mode = 1002 as libc::c_int;
    }
    if (*st).prev_mode > 0 as libc::c_int
        && ((*st).mode != 1002 as libc::c_int && (*st).prev_mode == 1002 as libc::c_int
            || (*st).mode == 1002 as libc::c_int && (*st).prev_mode != 1002 as libc::c_int)
    {
        redundancy = 1 as libc::c_int;
        celt_to_silk = ((*st).mode != 1002 as libc::c_int) as libc::c_int;
        if celt_to_silk == 0 {
            if frame_size >= (*st).Fs / 100 as libc::c_int {
                (*st).mode = (*st).prev_mode;
                to_celt = 1 as libc::c_int;
            } else {
                redundancy = 0 as libc::c_int;
            }
        }
    }
    if (*st).stream_channels == 1 as libc::c_int
        && (*st).prev_channels == 2 as libc::c_int
        && (*st).silk_mode.toMono == 0 as libc::c_int
        && (*st).mode != 1002 as libc::c_int
        && (*st).prev_mode != 1002 as libc::c_int
    {
        (*st).silk_mode.toMono = 1 as libc::c_int;
        (*st).stream_channels = 2 as libc::c_int;
    } else {
        (*st).silk_mode.toMono = 0 as libc::c_int;
    }
    equiv_rate = compute_equiv_rate(
        (*st).bitrate_bps,
        (*st).stream_channels,
        (*st).Fs / frame_size,
        (*st).use_vbr,
        (*st).mode,
        (*st).silk_mode.complexity,
        (*st).silk_mode.packetLossPercentage,
    );
    if (*st).mode != 1002 as libc::c_int && (*st).prev_mode == 1002 as libc::c_int {
        let mut dummy: silk_EncControlStruct = silk_EncControlStruct {
            nChannelsAPI: 0,
            nChannelsInternal: 0,
            API_sampleRate: 0,
            maxInternalSampleRate: 0,
            minInternalSampleRate: 0,
            desiredInternalSampleRate: 0,
            payloadSize_ms: 0,
            bitRate: 0,
            packetLossPercentage: 0,
            complexity: 0,
            useInBandFEC: 0,
            LBRR_coded: 0,
            useDTX: 0,
            useCBR: 0,
            maxBits: 0,
            toMono: 0,
            opusCanSwitch: 0,
            reducedDependency: 0,
            internalSampleRate: 0,
            allowBandwidthSwitch: 0,
            inWBmodeWithoutVariableLP: 0,
            stereoWidth_Q14: 0,
            switchReady: 0,
            signalType: 0,
            offset: 0,
        };
        silk_InitEncoder(silk_enc, (*st).arch, &mut dummy);
        prefill = 1 as libc::c_int;
    }
    if (*st).mode == 1002 as libc::c_int
        || (*st).first != 0
        || (*st).silk_mode.allowBandwidthSwitch != 0
    {
        let mut voice_bandwidth_thresholds: *const opus_int32 = 0 as *const opus_int32;
        let mut music_bandwidth_thresholds: *const opus_int32 = 0 as *const opus_int32;
        let mut bandwidth_thresholds: [opus_int32; 8] = [0; 8];
        let mut bandwidth: libc::c_int = 1105 as libc::c_int;
        if (*st).channels == 2 as libc::c_int && (*st).force_channels != 1 as libc::c_int {
            voice_bandwidth_thresholds = stereo_voice_bandwidth_thresholds.as_ptr();
            music_bandwidth_thresholds = stereo_music_bandwidth_thresholds.as_ptr();
        } else {
            voice_bandwidth_thresholds = mono_voice_bandwidth_thresholds.as_ptr();
            music_bandwidth_thresholds = mono_music_bandwidth_thresholds.as_ptr();
        }
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            bandwidth_thresholds[i as usize] = *music_bandwidth_thresholds.offset(i as isize)
                + (voice_est
                    * voice_est
                    * (*voice_bandwidth_thresholds.offset(i as isize)
                        - *music_bandwidth_thresholds.offset(i as isize))
                    >> 14 as libc::c_int);
            i += 1;
        }
        loop {
            let mut threshold_0: libc::c_int = 0;
            let mut hysteresis: libc::c_int = 0;
            threshold_0 = bandwidth_thresholds
                [(2 as libc::c_int * (bandwidth - 1102 as libc::c_int)) as usize];
            hysteresis = bandwidth_thresholds[(2 as libc::c_int * (bandwidth - 1102 as libc::c_int)
                + 1 as libc::c_int) as usize];
            if (*st).first == 0 {
                if (*st).auto_bandwidth >= bandwidth {
                    threshold_0 -= hysteresis;
                } else {
                    threshold_0 += hysteresis;
                }
            }
            if equiv_rate >= threshold_0 {
                break;
            }
            bandwidth -= 1;
            if !(bandwidth > 1101 as libc::c_int) {
                break;
            }
        }
        if bandwidth == 1102 as libc::c_int {
            bandwidth = 1103 as libc::c_int;
        }
        (*st).auto_bandwidth = bandwidth;
        (*st).bandwidth = (*st).auto_bandwidth;
        if (*st).first == 0
            && (*st).mode != 1002 as libc::c_int
            && (*st).silk_mode.inWBmodeWithoutVariableLP == 0
            && (*st).bandwidth > 1103 as libc::c_int
        {
            (*st).bandwidth = 1103 as libc::c_int;
        }
    }
    if (*st).bandwidth > (*st).max_bandwidth {
        (*st).bandwidth = (*st).max_bandwidth;
    }
    if (*st).user_bandwidth != -(1000 as libc::c_int) {
        (*st).bandwidth = (*st).user_bandwidth;
    }
    if (*st).mode != 1002 as libc::c_int && max_rate < 15000 as libc::c_int {
        (*st).bandwidth = if (*st).bandwidth < 1103 as libc::c_int {
            (*st).bandwidth
        } else {
            1103 as libc::c_int
        };
    }
    if (*st).Fs <= 24000 as libc::c_int && (*st).bandwidth > 1104 as libc::c_int {
        (*st).bandwidth = 1104 as libc::c_int;
    }
    if (*st).Fs <= 16000 as libc::c_int && (*st).bandwidth > 1103 as libc::c_int {
        (*st).bandwidth = 1103 as libc::c_int;
    }
    if (*st).Fs <= 12000 as libc::c_int && (*st).bandwidth > 1102 as libc::c_int {
        (*st).bandwidth = 1102 as libc::c_int;
    }
    if (*st).Fs <= 8000 as libc::c_int && (*st).bandwidth > 1101 as libc::c_int {
        (*st).bandwidth = 1101 as libc::c_int;
    }
    if (*st).detected_bandwidth != 0 && (*st).user_bandwidth == -(1000 as libc::c_int) {
        let mut min_detected_bandwidth: libc::c_int = 0;
        if equiv_rate <= 18000 as libc::c_int * (*st).stream_channels
            && (*st).mode == 1002 as libc::c_int
        {
            min_detected_bandwidth = 1101 as libc::c_int;
        } else if equiv_rate <= 24000 as libc::c_int * (*st).stream_channels
            && (*st).mode == 1002 as libc::c_int
        {
            min_detected_bandwidth = 1102 as libc::c_int;
        } else if equiv_rate <= 30000 as libc::c_int * (*st).stream_channels {
            min_detected_bandwidth = 1103 as libc::c_int;
        } else if equiv_rate <= 44000 as libc::c_int * (*st).stream_channels {
            min_detected_bandwidth = 1104 as libc::c_int;
        } else {
            min_detected_bandwidth = 1105 as libc::c_int;
        }
        (*st).detected_bandwidth = if (*st).detected_bandwidth > min_detected_bandwidth {
            (*st).detected_bandwidth
        } else {
            min_detected_bandwidth
        };
        (*st).bandwidth = if (*st).bandwidth < (*st).detected_bandwidth {
            (*st).bandwidth
        } else {
            (*st).detected_bandwidth
        };
    }
    (*st).silk_mode.LBRR_coded = decide_fec(
        (*st).silk_mode.useInBandFEC,
        (*st).silk_mode.packetLossPercentage,
        (*st).silk_mode.LBRR_coded,
        (*st).mode,
        &mut (*st).bandwidth,
        equiv_rate,
    );
    opus_custom_encoder_ctl(celt_enc, 4036 as libc::c_int, lsb_depth);
    if (*st).mode == 1002 as libc::c_int && (*st).bandwidth == 1102 as libc::c_int {
        (*st).bandwidth = 1103 as libc::c_int;
    }
    if (*st).lfe != 0 {
        (*st).bandwidth = 1101 as libc::c_int;
    }
    curr_bandwidth = (*st).bandwidth;
    if (*st).mode == 1000 as libc::c_int && curr_bandwidth > 1103 as libc::c_int {
        (*st).mode = 1001 as libc::c_int;
    }
    if (*st).mode == 1001 as libc::c_int && curr_bandwidth <= 1103 as libc::c_int {
        (*st).mode = 1000 as libc::c_int;
    }
    if frame_size > (*st).Fs / 50 as libc::c_int && (*st).mode != 1000 as libc::c_int
        || frame_size > 3 as libc::c_int * (*st).Fs / 50 as libc::c_int
    {
        let mut enc_frame_size: libc::c_int = 0;
        let mut nb_frames: libc::c_int = 0;
        if (*st).mode == 1000 as libc::c_int {
            if frame_size == 2 as libc::c_int * (*st).Fs / 25 as libc::c_int {
                enc_frame_size = (*st).Fs / 25 as libc::c_int;
            } else if frame_size == 3 as libc::c_int * (*st).Fs / 25 as libc::c_int {
                enc_frame_size = 3 as libc::c_int * (*st).Fs / 50 as libc::c_int;
            } else {
                enc_frame_size = (*st).Fs / 50 as libc::c_int;
            }
        } else {
            enc_frame_size = (*st).Fs / 50 as libc::c_int;
        }
        nb_frames = frame_size / enc_frame_size;
        if analysis_read_pos_bak != -(1 as libc::c_int) {
            (*st).analysis.read_pos = analysis_read_pos_bak;
            (*st).analysis.read_subframe = analysis_read_subframe_bak;
        }
        ret = encode_multiframe_packet(
            st,
            pcm,
            nb_frames,
            enc_frame_size,
            data,
            out_data_bytes,
            to_celt,
            lsb_depth,
            float_api,
        );
        return ret;
    }
    if (*st).silk_bw_switch != 0 {
        redundancy = 1 as libc::c_int;
        celt_to_silk = 1 as libc::c_int;
        (*st).silk_bw_switch = 0 as libc::c_int;
        prefill = 2 as libc::c_int;
    }
    if (*st).mode == 1002 as libc::c_int {
        redundancy = 0 as libc::c_int;
    }
    if redundancy != 0 {
        redundancy_bytes = compute_redundancy_bytes(
            max_data_bytes,
            (*st).bitrate_bps,
            frame_rate,
            (*st).stream_channels,
        );
        if redundancy_bytes == 0 as libc::c_int {
            redundancy = 0 as libc::c_int;
        }
    }
    bytes_target = (if max_data_bytes - redundancy_bytes
        < (*st).bitrate_bps * frame_size / ((*st).Fs * 8 as libc::c_int)
    {
        max_data_bytes - redundancy_bytes
    } else {
        (*st).bitrate_bps * frame_size / ((*st).Fs * 8 as libc::c_int)
    }) - 1 as libc::c_int;
    data = data.offset(1 as libc::c_int as isize);
    ec_enc_init(
        &mut enc,
        data,
        (max_data_bytes - 1 as libc::c_int) as opus_uint32,
    );
    let vla = ((total_buffer + frame_size) * (*st).channels) as usize;
    let mut pcm_buf: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    memcpy(
        pcm_buf.as_mut_ptr() as *mut libc::c_void,
        &mut *((*st).delay_buffer)
            .as_mut_ptr()
            .offset((((*st).encoder_buffer - total_buffer) * (*st).channels) as isize)
            as *mut opus_val16 as *const libc::c_void,
        ((total_buffer * (*st).channels) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * pcm_buf.as_mut_ptr().offset_from(
                        &mut *((*st).delay_buffer).as_mut_ptr().offset(
                            (((*st).encoder_buffer - total_buffer) * (*st).channels) as isize,
                        ),
                    ) as libc::c_long) as libc::c_ulong,
            ),
    );
    if (*st).mode == 1002 as libc::c_int {
        hp_freq_smth1 =
            ((silk_lin2log(60 as libc::c_int) as opus_uint32) << 8 as libc::c_int) as opus_int32;
    } else {
        hp_freq_smth1 = (*(silk_enc as *mut silk_encoder)).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .variable_HP_smth1_Q15;
    }
    (*st).variable_HP_smth2_Q15 = ((*st).variable_HP_smth2_Q15 as libc::c_long
        + ((hp_freq_smth1 - (*st).variable_HP_smth2_Q15) as libc::c_long
            * ((0.015f32 * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as opus_int32 as opus_int16 as opus_int64
            >> 16 as libc::c_int)) as opus_int32;
    cutoff_Hz = silk_log2lin((*st).variable_HP_smth2_Q15 >> 8 as libc::c_int);
    if (*st).application == 2048 as libc::c_int {
        hp_cutoff(
            pcm,
            cutoff_Hz,
            &mut *pcm_buf
                .as_mut_ptr()
                .offset((total_buffer * (*st).channels) as isize),
            ((*st).hp_mem).as_mut_ptr(),
            frame_size,
            (*st).channels,
            (*st).Fs,
            (*st).arch,
        );
    } else {
        dc_reject(
            pcm,
            3 as libc::c_int,
            &mut *pcm_buf
                .as_mut_ptr()
                .offset((total_buffer * (*st).channels) as isize),
            ((*st).hp_mem).as_mut_ptr(),
            frame_size,
            (*st).channels,
            (*st).Fs,
        );
    }
    if float_api != 0 {
        let mut sum: opus_val32 = 0.;
        sum = celt_inner_prod_c(
            &mut *pcm_buf
                .as_mut_ptr()
                .offset((total_buffer * (*st).channels) as isize),
            &mut *pcm_buf
                .as_mut_ptr()
                .offset((total_buffer * (*st).channels) as isize),
            frame_size * (*st).channels,
        );
        if !(sum < 1e9f32) || sum != sum {
            memset(
                &mut *pcm_buf
                    .as_mut_ptr()
                    .offset((total_buffer * (*st).channels) as isize)
                    as *mut opus_val16 as *mut libc::c_void,
                0 as libc::c_int,
                ((frame_size * (*st).channels) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong),
            );
            (*st).hp_mem[3 as libc::c_int as usize] = 0 as libc::c_int as opus_val32;
            (*st).hp_mem[2 as libc::c_int as usize] = (*st).hp_mem[3 as libc::c_int as usize];
            (*st).hp_mem[1 as libc::c_int as usize] = (*st).hp_mem[2 as libc::c_int as usize];
            (*st).hp_mem[0 as libc::c_int as usize] = (*st).hp_mem[1 as libc::c_int as usize];
        }
    }
    HB_gain = 1.0f32;
    if (*st).mode != 1002 as libc::c_int {
        let mut total_bitRate: opus_int32 = 0;
        let mut celt_rate: opus_int32 = 0;
        let mut activity: libc::c_int = 0;
        let vla_0 = ((*st).channels * frame_size) as usize;
        let mut pcm_silk: Vec<opus_int16> = ::std::vec::from_elem(0, vla_0);
        activity = -(1 as libc::c_int);
        if analysis_info.valid != 0 {
            activity = (analysis_info.activity_probability >= 0.1f32) as libc::c_int;
        }
        total_bitRate = 8 as libc::c_int * bytes_target * frame_rate;
        if (*st).mode == 1001 as libc::c_int {
            (*st).silk_mode.bitRate = compute_silk_rate_for_hybrid(
                total_bitRate,
                curr_bandwidth,
                ((*st).Fs == 50 as libc::c_int * frame_size) as libc::c_int,
                (*st).use_vbr,
                (*st).silk_mode.LBRR_coded,
                (*st).stream_channels,
            );
            if ((*st).energy_masking).is_null() {
                celt_rate = total_bitRate - (*st).silk_mode.bitRate;
                HB_gain = 1.0f32
                    - exp(0.6931471805599453094f64
                        * (-celt_rate as libc::c_float
                            * (1.0f32 / 1024 as libc::c_int as libc::c_float))
                            as libc::c_double) as libc::c_float;
            }
        } else {
            (*st).silk_mode.bitRate = total_bitRate;
        }
        if !((*st).energy_masking).is_null() && (*st).use_vbr != 0 && (*st).lfe == 0 {
            let mut mask_sum: opus_val32 = 0 as libc::c_int as opus_val32;
            let mut masking_depth: opus_val16 = 0.;
            let mut rate_offset: opus_int32 = 0;
            let mut c: libc::c_int = 0;
            let mut end: libc::c_int = 17 as libc::c_int;
            let mut srate: opus_int16 = 16000 as libc::c_int as opus_int16;
            if (*st).bandwidth == 1101 as libc::c_int {
                end = 13 as libc::c_int;
                srate = 8000 as libc::c_int as opus_int16;
            } else if (*st).bandwidth == 1102 as libc::c_int {
                end = 15 as libc::c_int;
                srate = 12000 as libc::c_int as opus_int16;
            }
            c = 0 as libc::c_int;
            while c < (*st).channels {
                i = 0 as libc::c_int;
                while i < end {
                    let mut mask: opus_val16 = 0.;
                    mask = if (if *((*st).energy_masking)
                        .offset((21 as libc::c_int * c + i) as isize)
                        < 0.5f32
                    {
                        *((*st).energy_masking).offset((21 as libc::c_int * c + i) as isize)
                    } else {
                        0.5f32
                    }) > -2.0f32
                    {
                        if *((*st).energy_masking).offset((21 as libc::c_int * c + i) as isize)
                            < 0.5f32
                        {
                            *((*st).energy_masking).offset((21 as libc::c_int * c + i) as isize)
                        } else {
                            0.5f32
                        }
                    } else {
                        -2.0f32
                    };
                    if mask > 0 as libc::c_int as libc::c_float {
                        mask = 0.5f32 * mask;
                    }
                    mask_sum += mask;
                    i += 1;
                }
                c += 1;
            }
            masking_depth = mask_sum / end as libc::c_float * (*st).channels as libc::c_float;
            masking_depth += 0.2f32;
            rate_offset = (srate as opus_val32 * masking_depth) as opus_int32;
            rate_offset =
                if rate_offset > -(2 as libc::c_int) * (*st).silk_mode.bitRate / 3 as libc::c_int {
                    rate_offset
                } else {
                    -(2 as libc::c_int) * (*st).silk_mode.bitRate / 3 as libc::c_int
                };
            if (*st).bandwidth == 1104 as libc::c_int || (*st).bandwidth == 1105 as libc::c_int {
                (*st).silk_mode.bitRate += 3 as libc::c_int * rate_offset / 5 as libc::c_int;
            } else {
                (*st).silk_mode.bitRate += rate_offset;
            }
        }
        (*st).silk_mode.payloadSize_ms = 1000 as libc::c_int * frame_size / (*st).Fs;
        (*st).silk_mode.nChannelsAPI = (*st).channels;
        (*st).silk_mode.nChannelsInternal = (*st).stream_channels;
        if curr_bandwidth == 1101 as libc::c_int {
            (*st).silk_mode.desiredInternalSampleRate = 8000 as libc::c_int;
        } else if curr_bandwidth == 1102 as libc::c_int {
            (*st).silk_mode.desiredInternalSampleRate = 12000 as libc::c_int;
        } else {
            (*st).silk_mode.desiredInternalSampleRate = 16000 as libc::c_int;
        }
        if (*st).mode == 1001 as libc::c_int {
            (*st).silk_mode.minInternalSampleRate = 16000 as libc::c_int;
        } else {
            (*st).silk_mode.minInternalSampleRate = 8000 as libc::c_int;
        }
        (*st).silk_mode.maxInternalSampleRate = 16000 as libc::c_int;
        if (*st).mode == 1000 as libc::c_int {
            let mut effective_max_rate: opus_int32 = max_rate;
            if frame_rate > 50 as libc::c_int {
                effective_max_rate = effective_max_rate * 2 as libc::c_int / 3 as libc::c_int;
            }
            if effective_max_rate < 8000 as libc::c_int {
                (*st).silk_mode.maxInternalSampleRate = 12000 as libc::c_int;
                (*st).silk_mode.desiredInternalSampleRate =
                    if (12000 as libc::c_int) < (*st).silk_mode.desiredInternalSampleRate {
                        12000 as libc::c_int
                    } else {
                        (*st).silk_mode.desiredInternalSampleRate
                    };
            }
            if effective_max_rate < 7000 as libc::c_int {
                (*st).silk_mode.maxInternalSampleRate = 8000 as libc::c_int;
                (*st).silk_mode.desiredInternalSampleRate =
                    if (8000 as libc::c_int) < (*st).silk_mode.desiredInternalSampleRate {
                        8000 as libc::c_int
                    } else {
                        (*st).silk_mode.desiredInternalSampleRate
                    };
            }
        }
        (*st).silk_mode.useCBR = ((*st).use_vbr == 0) as libc::c_int;
        (*st).silk_mode.maxBits = (max_data_bytes - 1 as libc::c_int) * 8 as libc::c_int;
        if redundancy != 0 && redundancy_bytes >= 2 as libc::c_int {
            (*st).silk_mode.maxBits -= redundancy_bytes * 8 as libc::c_int + 1 as libc::c_int;
            if (*st).mode == 1001 as libc::c_int {
                (*st).silk_mode.maxBits -= 20 as libc::c_int;
            }
        }
        if (*st).silk_mode.useCBR != 0 {
            if (*st).mode == 1001 as libc::c_int {
                (*st).silk_mode.maxBits =
                    if (*st).silk_mode.maxBits < (*st).silk_mode.bitRate * frame_size / (*st).Fs {
                        (*st).silk_mode.maxBits
                    } else {
                        (*st).silk_mode.bitRate * frame_size / (*st).Fs
                    };
            }
        } else if (*st).mode == 1001 as libc::c_int {
            let mut maxBitRate: opus_int32 = compute_silk_rate_for_hybrid(
                (*st).silk_mode.maxBits * (*st).Fs / frame_size,
                curr_bandwidth,
                ((*st).Fs == 50 as libc::c_int * frame_size) as libc::c_int,
                (*st).use_vbr,
                (*st).silk_mode.LBRR_coded,
                (*st).stream_channels,
            );
            (*st).silk_mode.maxBits = maxBitRate * frame_size / (*st).Fs;
        }
        if prefill != 0 {
            let mut zero: opus_int32 = 0 as libc::c_int;
            let mut prefill_offset: libc::c_int = 0;
            prefill_offset = (*st).channels
                * ((*st).encoder_buffer - (*st).delay_compensation - (*st).Fs / 400 as libc::c_int);
            gain_fade(
                ((*st).delay_buffer)
                    .as_mut_ptr()
                    .offset(prefill_offset as isize),
                ((*st).delay_buffer)
                    .as_mut_ptr()
                    .offset(prefill_offset as isize),
                0 as libc::c_int as opus_val16,
                1.0f32,
                (*celt_mode).overlap,
                (*st).Fs / 400 as libc::c_int,
                (*st).channels,
                (*celt_mode).window,
                (*st).Fs,
            );
            memset(
                ((*st).delay_buffer).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                (prefill_offset as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong),
            );
            i = 0 as libc::c_int;
            while i < (*st).encoder_buffer * (*st).channels {
                *pcm_silk.as_mut_ptr().offset(i as isize) =
                    FLOAT2INT16((*st).delay_buffer[i as usize]);
                i += 1;
            }
            silk_Encode(
                silk_enc,
                &mut (*st).silk_mode,
                pcm_silk.as_mut_ptr(),
                (*st).encoder_buffer,
                0 as *mut ec_enc,
                &mut zero,
                prefill,
                activity,
            );
            (*st).silk_mode.opusCanSwitch = 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < frame_size * (*st).channels {
            *pcm_silk.as_mut_ptr().offset(i as isize) = FLOAT2INT16(
                *pcm_buf
                    .as_mut_ptr()
                    .offset((total_buffer * (*st).channels + i) as isize),
            );
            i += 1;
        }
        ret = silk_Encode(
            silk_enc,
            &mut (*st).silk_mode,
            pcm_silk.as_mut_ptr(),
            frame_size,
            &mut enc,
            &mut nBytes,
            0 as libc::c_int,
            activity,
        );
        if ret != 0 {
            return -(3 as libc::c_int);
        }
        if (*st).mode == 1000 as libc::c_int {
            if (*st).silk_mode.internalSampleRate == 8000 as libc::c_int {
                curr_bandwidth = 1101 as libc::c_int;
            } else if (*st).silk_mode.internalSampleRate == 12000 as libc::c_int {
                curr_bandwidth = 1102 as libc::c_int;
            } else if (*st).silk_mode.internalSampleRate == 16000 as libc::c_int {
                curr_bandwidth = 1103 as libc::c_int;
            }
        }
        (*st).silk_mode.opusCanSwitch =
            ((*st).silk_mode.switchReady != 0 && (*st).nonfinal_frame == 0) as libc::c_int;
        if nBytes == 0 as libc::c_int {
            (*st).rangeFinal = 0 as libc::c_int as opus_uint32;
            *data.offset(-(1 as libc::c_int) as isize) = gen_toc(
                (*st).mode,
                (*st).Fs / frame_size,
                curr_bandwidth,
                (*st).stream_channels,
            );
            return 1 as libc::c_int;
        }
        if (*st).silk_mode.opusCanSwitch != 0 {
            redundancy_bytes = compute_redundancy_bytes(
                max_data_bytes,
                (*st).bitrate_bps,
                frame_rate,
                (*st).stream_channels,
            );
            redundancy = (redundancy_bytes != 0 as libc::c_int) as libc::c_int;
            celt_to_silk = 0 as libc::c_int;
            (*st).silk_bw_switch = 1 as libc::c_int;
        }
    }
    let mut endband: libc::c_int = 21 as libc::c_int;
    match curr_bandwidth {
        1101 => {
            endband = 13 as libc::c_int;
        }
        1102 | 1103 => {
            endband = 17 as libc::c_int;
        }
        1104 => {
            endband = 19 as libc::c_int;
        }
        1105 => {
            endband = 21 as libc::c_int;
        }
        _ => {}
    }
    opus_custom_encoder_ctl(celt_enc, 10012 as libc::c_int, endband);
    opus_custom_encoder_ctl(celt_enc, 10008 as libc::c_int, (*st).stream_channels);
    opus_custom_encoder_ctl(celt_enc, 4002 as libc::c_int, -(1 as libc::c_int));
    if (*st).mode != 1000 as libc::c_int {
        let mut celt_pred: opus_val32 = 2 as libc::c_int as opus_val32;
        opus_custom_encoder_ctl(celt_enc, 4006 as libc::c_int, 0 as libc::c_int);
        if (*st).silk_mode.reducedDependency != 0 {
            celt_pred = 0 as libc::c_int as opus_val32;
        }
        opus_custom_encoder_ctl(celt_enc, 10002 as libc::c_int, celt_pred as opus_int32);
        if (*st).mode == 1001 as libc::c_int {
            if (*st).use_vbr != 0 {
                opus_custom_encoder_ctl(
                    celt_enc,
                    4002 as libc::c_int,
                    (*st).bitrate_bps - (*st).silk_mode.bitRate,
                );
                opus_custom_encoder_ctl(celt_enc, 4020 as libc::c_int, 0 as libc::c_int);
            }
        } else if (*st).use_vbr != 0 {
            opus_custom_encoder_ctl(celt_enc, 4006 as libc::c_int, 1 as libc::c_int);
            opus_custom_encoder_ctl(celt_enc, 4020 as libc::c_int, (*st).vbr_constraint);
            opus_custom_encoder_ctl(celt_enc, 4002 as libc::c_int, (*st).bitrate_bps);
        }
    }
    let vla_1 = ((*st).channels * (*st).Fs / 400 as libc::c_int) as usize;
    let mut tmp_prefill: Vec<opus_val16> = ::std::vec::from_elem(0., vla_1);
    if (*st).mode != 1000 as libc::c_int
        && (*st).mode != (*st).prev_mode
        && (*st).prev_mode > 0 as libc::c_int
    {
        memcpy(
            tmp_prefill.as_mut_ptr() as *mut libc::c_void,
            &mut *((*st).delay_buffer).as_mut_ptr().offset(
                (((*st).encoder_buffer - total_buffer - (*st).Fs / 400 as libc::c_int)
                    * (*st).channels) as isize,
            ) as *mut opus_val16 as *const libc::c_void,
            (((*st).channels * (*st).Fs / 400 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * tmp_prefill.as_mut_ptr().offset_from(
                            &mut *((*st).delay_buffer).as_mut_ptr().offset(
                                (((*st).encoder_buffer
                                    - total_buffer
                                    - (*st).Fs / 400 as libc::c_int)
                                    * (*st).channels) as isize,
                            ),
                        ) as libc::c_long) as libc::c_ulong,
                ),
        );
    }
    if (*st).channels * ((*st).encoder_buffer - (frame_size + total_buffer)) > 0 as libc::c_int {
        memmove(
            ((*st).delay_buffer).as_mut_ptr() as *mut libc::c_void,
            &mut *((*st).delay_buffer)
                .as_mut_ptr()
                .offset(((*st).channels * frame_size) as isize) as *mut opus_val16
                as *const libc::c_void,
            (((*st).channels * ((*st).encoder_buffer - frame_size - total_buffer))
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * ((*st).delay_buffer).as_mut_ptr().offset_from(
                            &mut *((*st).delay_buffer)
                                .as_mut_ptr()
                                .offset(((*st).channels * frame_size) as isize),
                        ) as libc::c_long) as libc::c_ulong,
                ),
        );
        memcpy(
            &mut *((*st).delay_buffer).as_mut_ptr().offset(
                ((*st).channels * ((*st).encoder_buffer - frame_size - total_buffer)) as isize,
            ) as *mut opus_val16 as *mut libc::c_void,
            &mut *pcm_buf.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut opus_val16
                as *const libc::c_void,
            (((frame_size + total_buffer) * (*st).channels) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (&mut *((*st).delay_buffer).as_mut_ptr().offset(
                            ((*st).channels * ((*st).encoder_buffer - frame_size - total_buffer))
                                as isize,
                        ) as *mut opus_val16)
                            .offset_from(
                                &mut *pcm_buf.as_mut_ptr().offset(0 as libc::c_int as isize),
                            ) as libc::c_long) as libc::c_ulong,
                ),
        );
    } else {
        memcpy(
            ((*st).delay_buffer).as_mut_ptr() as *mut libc::c_void,
            &mut *pcm_buf.as_mut_ptr().offset(
                ((frame_size + total_buffer - (*st).encoder_buffer) * (*st).channels) as isize,
            ) as *mut opus_val16 as *const libc::c_void,
            (((*st).encoder_buffer * (*st).channels) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * ((*st).delay_buffer).as_mut_ptr().offset_from(
                            &mut *pcm_buf.as_mut_ptr().offset(
                                ((frame_size + total_buffer - (*st).encoder_buffer)
                                    * (*st).channels) as isize,
                            ),
                        ) as libc::c_long) as libc::c_ulong,
                ),
        );
    }
    if (*st).prev_HB_gain < 1.0f32 || HB_gain < 1.0f32 {
        gain_fade(
            pcm_buf.as_mut_ptr(),
            pcm_buf.as_mut_ptr(),
            (*st).prev_HB_gain,
            HB_gain,
            (*celt_mode).overlap,
            frame_size,
            (*st).channels,
            (*celt_mode).window,
            (*st).Fs,
        );
    }
    (*st).prev_HB_gain = HB_gain;
    if (*st).mode != 1001 as libc::c_int || (*st).stream_channels == 1 as libc::c_int {
        if equiv_rate > 32000 as libc::c_int {
            (*st).silk_mode.stereoWidth_Q14 = 16384 as libc::c_int;
        } else if equiv_rate < 16000 as libc::c_int {
            (*st).silk_mode.stereoWidth_Q14 = 0 as libc::c_int;
        } else {
            (*st).silk_mode.stereoWidth_Q14 = 16384 as libc::c_int
                - 2048 as libc::c_int * (32000 as libc::c_int - equiv_rate)
                    / (equiv_rate - 14000 as libc::c_int);
        }
    }
    if ((*st).energy_masking).is_null() && (*st).channels == 2 as libc::c_int {
        if ((*st).hybrid_stereo_width_Q14 as libc::c_int) < (1 as libc::c_int) << 14 as libc::c_int
            || (*st).silk_mode.stereoWidth_Q14 < (1 as libc::c_int) << 14 as libc::c_int
        {
            let mut g1: opus_val16 = 0.;
            let mut g2: opus_val16 = 0.;
            g1 = (*st).hybrid_stereo_width_Q14 as opus_val16;
            g2 = (*st).silk_mode.stereoWidth_Q14 as opus_val16;
            g1 *= 1.0f32 / 16384 as libc::c_int as libc::c_float;
            g2 *= 1.0f32 / 16384 as libc::c_int as libc::c_float;
            stereo_fade(
                pcm_buf.as_mut_ptr(),
                pcm_buf.as_mut_ptr(),
                g1,
                g2,
                (*celt_mode).overlap,
                frame_size,
                (*st).channels,
                (*celt_mode).window,
                (*st).Fs,
            );
            (*st).hybrid_stereo_width_Q14 = (*st).silk_mode.stereoWidth_Q14 as opus_int16;
        }
    }
    if (*st).mode != 1002 as libc::c_int
        && ec_tell(&mut enc)
            + 17 as libc::c_int
            + 20 as libc::c_int * ((*st).mode == 1001 as libc::c_int) as libc::c_int
            <= 8 as libc::c_int * (max_data_bytes - 1 as libc::c_int)
    {
        if (*st).mode == 1001 as libc::c_int {
            ec_enc_bit_logp(&mut enc, redundancy, 12 as libc::c_int as libc::c_uint);
        }
        if redundancy != 0 {
            let mut max_redundancy: libc::c_int = 0;
            ec_enc_bit_logp(&mut enc, celt_to_silk, 1 as libc::c_int as libc::c_uint);
            if (*st).mode == 1001 as libc::c_int {
                max_redundancy = max_data_bytes
                    - 1 as libc::c_int
                    - (ec_tell(&mut enc) + 8 as libc::c_int + 3 as libc::c_int + 7 as libc::c_int
                        >> 3 as libc::c_int);
            } else {
                max_redundancy = max_data_bytes
                    - 1 as libc::c_int
                    - (ec_tell(&mut enc) + 7 as libc::c_int >> 3 as libc::c_int);
            }
            redundancy_bytes = if max_redundancy < redundancy_bytes {
                max_redundancy
            } else {
                redundancy_bytes
            };
            redundancy_bytes = if (257 as libc::c_int)
                < (if 2 as libc::c_int > redundancy_bytes {
                    2 as libc::c_int
                } else {
                    redundancy_bytes
                }) {
                257 as libc::c_int
            } else if 2 as libc::c_int > redundancy_bytes {
                2 as libc::c_int
            } else {
                redundancy_bytes
            };
            if (*st).mode == 1001 as libc::c_int {
                ec_enc_uint(
                    &mut enc,
                    (redundancy_bytes - 2 as libc::c_int) as opus_uint32,
                    256 as libc::c_int as opus_uint32,
                );
            }
        }
    } else {
        redundancy = 0 as libc::c_int;
    }
    if redundancy == 0 {
        (*st).silk_bw_switch = 0 as libc::c_int;
        redundancy_bytes = 0 as libc::c_int;
    }
    if (*st).mode != 1002 as libc::c_int {
        start_band = 17 as libc::c_int;
    }
    if (*st).mode == 1000 as libc::c_int {
        ret = ec_tell(&mut enc) + 7 as libc::c_int >> 3 as libc::c_int;
        ec_enc_done(&mut enc);
        nb_compr_bytes = ret;
    } else {
        nb_compr_bytes = max_data_bytes - 1 as libc::c_int - redundancy_bytes;
        ec_enc_shrink(&mut enc, nb_compr_bytes as opus_uint32);
    }
    if redundancy != 0 || (*st).mode != 1000 as libc::c_int {
        opus_custom_encoder_ctl(
            celt_enc,
            10022 as libc::c_int,
            (&mut analysis_info as *mut AnalysisInfo).offset(
                (&mut analysis_info as *mut AnalysisInfo)
                    .offset_from(&mut analysis_info as *mut AnalysisInfo as *const AnalysisInfo)
                    as libc::c_long as isize,
            ),
        );
    }
    if (*st).mode == 1001 as libc::c_int {
        let mut info: SILKInfo = SILKInfo {
            signalType: 0,
            offset: 0,
        };
        info.signalType = (*st).silk_mode.signalType;
        info.offset = (*st).silk_mode.offset;
        opus_custom_encoder_ctl(
            celt_enc,
            10028 as libc::c_int,
            (&mut info as *mut SILKInfo).offset(
                (&mut info as *mut SILKInfo)
                    .offset_from(&mut info as *mut SILKInfo as *const SILKInfo)
                    as libc::c_long as isize,
            ),
        );
    }
    if redundancy != 0 && celt_to_silk != 0 {
        let mut err: libc::c_int = 0;
        opus_custom_encoder_ctl(celt_enc, 10010 as libc::c_int, 0 as libc::c_int);
        opus_custom_encoder_ctl(celt_enc, 4006 as libc::c_int, 0 as libc::c_int);
        opus_custom_encoder_ctl(celt_enc, 4002 as libc::c_int, -(1 as libc::c_int));
        err = celt_encode_with_ec(
            celt_enc,
            pcm_buf.as_mut_ptr(),
            (*st).Fs / 200 as libc::c_int,
            data.offset(nb_compr_bytes as isize),
            redundancy_bytes,
            0 as *mut ec_enc,
        );
        if err < 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        opus_custom_encoder_ctl(
            celt_enc,
            4031 as libc::c_int,
            (&mut redundant_rng as *mut opus_uint32).offset(
                (&mut redundant_rng as *mut opus_uint32)
                    .offset_from(&mut redundant_rng as *mut opus_uint32)
                    as libc::c_long as isize,
            ),
        );
        opus_custom_encoder_ctl(celt_enc, 4028 as libc::c_int);
    }
    opus_custom_encoder_ctl(celt_enc, 10010 as libc::c_int, start_band);
    if (*st).mode != 1000 as libc::c_int {
        if (*st).mode != (*st).prev_mode && (*st).prev_mode > 0 as libc::c_int {
            let mut dummy_0: [libc::c_uchar; 2] = [0; 2];
            opus_custom_encoder_ctl(celt_enc, 4028 as libc::c_int);
            celt_encode_with_ec(
                celt_enc,
                tmp_prefill.as_mut_ptr(),
                (*st).Fs / 400 as libc::c_int,
                dummy_0.as_mut_ptr(),
                2 as libc::c_int,
                0 as *mut ec_enc,
            );
            opus_custom_encoder_ctl(celt_enc, 10002 as libc::c_int, 0 as libc::c_int);
        }
        if ec_tell(&mut enc) <= 8 as libc::c_int * nb_compr_bytes {
            if redundancy != 0
                && celt_to_silk != 0
                && (*st).mode == 1001 as libc::c_int
                && (*st).use_vbr != 0
            {
                opus_custom_encoder_ctl(
                    celt_enc,
                    4002 as libc::c_int,
                    (*st).bitrate_bps - (*st).silk_mode.bitRate,
                );
            }
            opus_custom_encoder_ctl(celt_enc, 4006 as libc::c_int, (*st).use_vbr);
            ret = celt_encode_with_ec(
                celt_enc,
                pcm_buf.as_mut_ptr(),
                frame_size,
                0 as *mut libc::c_uchar,
                nb_compr_bytes,
                &mut enc,
            );
            if ret < 0 as libc::c_int {
                return -(3 as libc::c_int);
            }
            if redundancy != 0
                && celt_to_silk != 0
                && (*st).mode == 1001 as libc::c_int
                && (*st).use_vbr != 0
            {
                memmove(
                    data.offset(ret as isize) as *mut libc::c_void,
                    data.offset(nb_compr_bytes as isize) as *const libc::c_void,
                    (redundancy_bytes as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                        .wrapping_add(
                            (0 as libc::c_int as libc::c_long
                                * data
                                    .offset(ret as isize)
                                    .offset_from(data.offset(nb_compr_bytes as isize))
                                    as libc::c_long) as libc::c_ulong,
                        ),
                );
                nb_compr_bytes = nb_compr_bytes + redundancy_bytes;
            }
        }
    }
    if redundancy != 0 && celt_to_silk == 0 {
        let mut err_0: libc::c_int = 0;
        let mut dummy_1: [libc::c_uchar; 2] = [0; 2];
        let mut N2: libc::c_int = 0;
        let mut N4: libc::c_int = 0;
        N2 = (*st).Fs / 200 as libc::c_int;
        N4 = (*st).Fs / 400 as libc::c_int;
        opus_custom_encoder_ctl(celt_enc, 4028 as libc::c_int);
        opus_custom_encoder_ctl(celt_enc, 10010 as libc::c_int, 0 as libc::c_int);
        opus_custom_encoder_ctl(celt_enc, 10002 as libc::c_int, 0 as libc::c_int);
        opus_custom_encoder_ctl(celt_enc, 4006 as libc::c_int, 0 as libc::c_int);
        opus_custom_encoder_ctl(celt_enc, 4002 as libc::c_int, -(1 as libc::c_int));
        if (*st).mode == 1001 as libc::c_int {
            nb_compr_bytes = ret;
            ec_enc_shrink(&mut enc, nb_compr_bytes as opus_uint32);
        }
        celt_encode_with_ec(
            celt_enc,
            pcm_buf
                .as_mut_ptr()
                .offset(((*st).channels * (frame_size - N2 - N4)) as isize),
            N4,
            dummy_1.as_mut_ptr(),
            2 as libc::c_int,
            0 as *mut ec_enc,
        );
        err_0 = celt_encode_with_ec(
            celt_enc,
            pcm_buf
                .as_mut_ptr()
                .offset(((*st).channels * (frame_size - N2)) as isize),
            N2,
            data.offset(nb_compr_bytes as isize),
            redundancy_bytes,
            0 as *mut ec_enc,
        );
        if err_0 < 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        opus_custom_encoder_ctl(
            celt_enc,
            4031 as libc::c_int,
            (&mut redundant_rng as *mut opus_uint32).offset(
                (&mut redundant_rng as *mut opus_uint32)
                    .offset_from(&mut redundant_rng as *mut opus_uint32)
                    as libc::c_long as isize,
            ),
        );
    }
    data = data.offset(-1);
    *data.offset(0 as libc::c_int as isize) = gen_toc(
        (*st).mode,
        (*st).Fs / frame_size,
        curr_bandwidth,
        (*st).stream_channels,
    );
    (*st).rangeFinal = enc.rng ^ redundant_rng;
    if to_celt != 0 {
        (*st).prev_mode = 1002 as libc::c_int;
    } else {
        (*st).prev_mode = (*st).mode;
    }
    (*st).prev_channels = (*st).stream_channels;
    (*st).prev_framesize = frame_size;
    (*st).first = 0 as libc::c_int;
    if (*st).use_dtx != 0 && (analysis_info.valid != 0 || is_silence != 0) {
        if decide_dtx_mode(
            analysis_info.activity_probability,
            &mut (*st).nb_no_activity_frames,
            (*st).peak_signal_energy,
            pcm,
            frame_size,
            (*st).channels,
            is_silence,
            (*st).arch,
        ) != 0
        {
            (*st).rangeFinal = 0 as libc::c_int as opus_uint32;
            *data.offset(0 as libc::c_int as isize) = gen_toc(
                (*st).mode,
                (*st).Fs / frame_size,
                curr_bandwidth,
                (*st).stream_channels,
            );
            return 1 as libc::c_int;
        }
    } else {
        (*st).nb_no_activity_frames = 0 as libc::c_int;
    }
    if ec_tell(&mut enc) > (max_data_bytes - 1 as libc::c_int) * 8 as libc::c_int {
        if max_data_bytes < 2 as libc::c_int {
            return -(2 as libc::c_int);
        }
        *data.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
        ret = 1 as libc::c_int;
        (*st).rangeFinal = 0 as libc::c_int as opus_uint32;
    } else if (*st).mode == 1000 as libc::c_int && redundancy == 0 {
        while ret > 2 as libc::c_int
            && *data.offset(ret as isize) as libc::c_int == 0 as libc::c_int
        {
            ret -= 1;
        }
    }
    ret += 1 as libc::c_int + redundancy_bytes;
    if (*st).use_vbr == 0 {
        if opus_packet_pad(data, ret, max_data_bytes) != 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        ret = max_data_bytes;
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2235:1"]
pub unsafe extern "C" fn opus_encode(
    mut st: *mut OpusEncoder,
    mut pcm: *const opus_int16,
    mut analysis_frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut max_data_bytes: opus_int32,
) -> opus_int32 {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut frame_size: libc::c_int = 0;
    frame_size = frame_size_select(analysis_frame_size, (*st).variable_duration, (*st).Fs);
    if frame_size <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let vla = (frame_size * (*st).channels) as usize;
    let mut in_0: Vec<libc::c_float> = ::std::vec::from_elem(0., vla);
    i = 0 as libc::c_int;
    while i < frame_size * (*st).channels {
        *in_0.as_mut_ptr().offset(i as isize) = 1.0f32 / 32768 as libc::c_int as libc::c_float
            * *pcm.offset(i as isize) as libc::c_int as libc::c_float;
        i += 1;
    }
    ret = opus_encode_native(
        st,
        in_0.as_mut_ptr(),
        frame_size,
        data,
        max_data_bytes,
        16 as libc::c_int,
        pcm as *const libc::c_void,
        analysis_frame_size,
        0 as libc::c_int,
        -(2 as libc::c_int),
        (*st).channels,
        Some(
            downmix_int
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut opus_val32,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> (),
        ),
        0 as libc::c_int,
    );
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2258:1"]
pub unsafe extern "C" fn opus_encode_float(
    mut st: *mut OpusEncoder,
    mut pcm: *const libc::c_float,
    mut analysis_frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut out_data_bytes: opus_int32,
) -> opus_int32 {
    let mut frame_size: libc::c_int = 0;
    frame_size = frame_size_select(analysis_frame_size, (*st).variable_duration, (*st).Fs);
    return opus_encode_native(
        st,
        pcm,
        frame_size,
        data,
        out_data_bytes,
        24 as libc::c_int,
        pcm as *const libc::c_void,
        analysis_frame_size,
        0 as libc::c_int,
        -(2 as libc::c_int),
        (*st).channels,
        Some(
            downmix_float
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut opus_val32,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> (),
        ),
        1 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "2269:1"]
pub unsafe extern "C" fn opus_encoder_ctl(
    mut st: *mut OpusEncoder,
    mut request: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut celt_enc: *mut OpusCustomEncoder = 0 as *mut OpusCustomEncoder;
    let mut ap: ::core::ffi::VaListImpl;
    ret = 0 as libc::c_int;
    ap = args.clone();
    celt_enc =
        (st as *mut libc::c_char).offset((*st).celt_enc_offset as isize) as *mut OpusCustomEncoder;
    match request {
        4000 => {
            let mut value: opus_int32 = ap.arg::<opus_int32>();
            if value != 2048 as libc::c_int
                && value != 2049 as libc::c_int
                && value != 2051 as libc::c_int
                || (*st).first == 0 && (*st).application != value
            {
                ret = -(1 as libc::c_int);
            } else {
                (*st).application = value;
                (*st).analysis.application = value;
            }
            current_block = 16167632229894708628;
        }
        4001 => {
            let mut value_0: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_0.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_0 = (*st).application;
                current_block = 16167632229894708628;
            }
        }
        4002 => {
            let mut value_1: opus_int32 = ap.arg::<opus_int32>();
            if value_1 != -(1000 as libc::c_int) && value_1 != -(1 as libc::c_int) {
                if value_1 <= 0 as libc::c_int {
                    current_block = 12343738388509029619;
                } else {
                    if value_1 <= 500 as libc::c_int {
                        value_1 = 500 as libc::c_int;
                    } else if value_1 > 300000 as libc::c_int * (*st).channels {
                        value_1 = 300000 as libc::c_int * (*st).channels;
                    }
                    current_block = 6057473163062296781;
                }
            } else {
                current_block = 6057473163062296781;
            }
            match current_block {
                12343738388509029619 => {}
                _ => {
                    (*st).user_bitrate_bps = value_1;
                    current_block = 16167632229894708628;
                }
            }
        }
        4003 => {
            let mut value_2: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_2.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_2 = user_bitrate_to_bitrate(st, (*st).prev_framesize, 1276 as libc::c_int);
                current_block = 16167632229894708628;
            }
        }
        4022 => {
            let mut value_3: opus_int32 = ap.arg::<opus_int32>();
            if (value_3 < 1 as libc::c_int || value_3 > (*st).channels)
                && value_3 != -(1000 as libc::c_int)
            {
                current_block = 12343738388509029619;
            } else {
                (*st).force_channels = value_3;
                current_block = 16167632229894708628;
            }
        }
        4023 => {
            let mut value_4: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_4.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_4 = (*st).force_channels;
                current_block = 16167632229894708628;
            }
        }
        4004 => {
            let mut value_5: opus_int32 = ap.arg::<opus_int32>();
            if value_5 < 1101 as libc::c_int || value_5 > 1105 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).max_bandwidth = value_5;
                if (*st).max_bandwidth == 1101 as libc::c_int {
                    (*st).silk_mode.maxInternalSampleRate = 8000 as libc::c_int;
                } else if (*st).max_bandwidth == 1102 as libc::c_int {
                    (*st).silk_mode.maxInternalSampleRate = 12000 as libc::c_int;
                } else {
                    (*st).silk_mode.maxInternalSampleRate = 16000 as libc::c_int;
                }
                current_block = 16167632229894708628;
            }
        }
        4005 => {
            let mut value_6: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_6.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_6 = (*st).max_bandwidth;
                current_block = 16167632229894708628;
            }
        }
        4008 => {
            let mut value_7: opus_int32 = ap.arg::<opus_int32>();
            if (value_7 < 1101 as libc::c_int || value_7 > 1105 as libc::c_int)
                && value_7 != -(1000 as libc::c_int)
            {
                current_block = 12343738388509029619;
            } else {
                (*st).user_bandwidth = value_7;
                if (*st).user_bandwidth == 1101 as libc::c_int {
                    (*st).silk_mode.maxInternalSampleRate = 8000 as libc::c_int;
                } else if (*st).user_bandwidth == 1102 as libc::c_int {
                    (*st).silk_mode.maxInternalSampleRate = 12000 as libc::c_int;
                } else {
                    (*st).silk_mode.maxInternalSampleRate = 16000 as libc::c_int;
                }
                current_block = 16167632229894708628;
            }
        }
        4009 => {
            let mut value_8: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_8.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_8 = (*st).bandwidth;
                current_block = 16167632229894708628;
            }
        }
        4016 => {
            let mut value_9: opus_int32 = ap.arg::<opus_int32>();
            if value_9 < 0 as libc::c_int || value_9 > 1 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).use_dtx = value_9;
                current_block = 16167632229894708628;
            }
        }
        4017 => {
            let mut value_10: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_10.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_10 = (*st).use_dtx;
                current_block = 16167632229894708628;
            }
        }
        4010 => {
            let mut value_11: opus_int32 = ap.arg::<opus_int32>();
            if value_11 < 0 as libc::c_int || value_11 > 10 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).silk_mode.complexity = value_11;
                opus_custom_encoder_ctl(celt_enc, 4010 as libc::c_int, value_11);
                current_block = 16167632229894708628;
            }
        }
        4011 => {
            let mut value_12: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_12.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_12 = (*st).silk_mode.complexity;
                current_block = 16167632229894708628;
            }
        }
        4012 => {
            let mut value_13: opus_int32 = ap.arg::<opus_int32>();
            if value_13 < 0 as libc::c_int || value_13 > 1 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).silk_mode.useInBandFEC = value_13;
                current_block = 16167632229894708628;
            }
        }
        4013 => {
            let mut value_14: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_14.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_14 = (*st).silk_mode.useInBandFEC;
                current_block = 16167632229894708628;
            }
        }
        4014 => {
            let mut value_15: opus_int32 = ap.arg::<opus_int32>();
            if value_15 < 0 as libc::c_int || value_15 > 100 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).silk_mode.packetLossPercentage = value_15;
                opus_custom_encoder_ctl(celt_enc, 4014 as libc::c_int, value_15);
                current_block = 16167632229894708628;
            }
        }
        4015 => {
            let mut value_16: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_16.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_16 = (*st).silk_mode.packetLossPercentage;
                current_block = 16167632229894708628;
            }
        }
        4006 => {
            let mut value_17: opus_int32 = ap.arg::<opus_int32>();
            if value_17 < 0 as libc::c_int || value_17 > 1 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).use_vbr = value_17;
                (*st).silk_mode.useCBR = 1 as libc::c_int - value_17;
                current_block = 16167632229894708628;
            }
        }
        4007 => {
            let mut value_18: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_18.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_18 = (*st).use_vbr;
                current_block = 16167632229894708628;
            }
        }
        11018 => {
            let mut value_19: opus_int32 = ap.arg::<opus_int32>();
            if value_19 < -(1 as libc::c_int) || value_19 > 100 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).voice_ratio = value_19;
                current_block = 16167632229894708628;
            }
        }
        11019 => {
            let mut value_20: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_20.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_20 = (*st).voice_ratio;
                current_block = 16167632229894708628;
            }
        }
        4020 => {
            let mut value_21: opus_int32 = ap.arg::<opus_int32>();
            if value_21 < 0 as libc::c_int || value_21 > 1 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).vbr_constraint = value_21;
                current_block = 16167632229894708628;
            }
        }
        4021 => {
            let mut value_22: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_22.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_22 = (*st).vbr_constraint;
                current_block = 16167632229894708628;
            }
        }
        4024 => {
            let mut value_23: opus_int32 = ap.arg::<opus_int32>();
            if value_23 != -(1000 as libc::c_int)
                && value_23 != 3001 as libc::c_int
                && value_23 != 3002 as libc::c_int
            {
                current_block = 12343738388509029619;
            } else {
                (*st).signal_type = value_23;
                current_block = 16167632229894708628;
            }
        }
        4025 => {
            let mut value_24: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_24.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_24 = (*st).signal_type;
                current_block = 16167632229894708628;
            }
        }
        4027 => {
            let mut value_25: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_25.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_25 = (*st).Fs / 400 as libc::c_int;
                if (*st).application != 2051 as libc::c_int {
                    *value_25 += (*st).delay_compensation;
                }
                current_block = 16167632229894708628;
            }
        }
        4029 => {
            let mut value_26: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_26.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_26 = (*st).Fs;
                current_block = 16167632229894708628;
            }
        }
        4031 => {
            let mut value_27: *mut opus_uint32 = ap.arg::<*mut opus_uint32>();
            if value_27.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_27 = (*st).rangeFinal;
                current_block = 16167632229894708628;
            }
        }
        4036 => {
            let mut value_28: opus_int32 = ap.arg::<opus_int32>();
            if value_28 < 8 as libc::c_int || value_28 > 24 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).lsb_depth = value_28;
                current_block = 16167632229894708628;
            }
        }
        4037 => {
            let mut value_29: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_29.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_29 = (*st).lsb_depth;
                current_block = 16167632229894708628;
            }
        }
        4040 => {
            let mut value_30: opus_int32 = ap.arg::<opus_int32>();
            if value_30 != 5000 as libc::c_int
                && value_30 != 5001 as libc::c_int
                && value_30 != 5002 as libc::c_int
                && value_30 != 5003 as libc::c_int
                && value_30 != 5004 as libc::c_int
                && value_30 != 5005 as libc::c_int
                && value_30 != 5006 as libc::c_int
                && value_30 != 5007 as libc::c_int
                && value_30 != 5008 as libc::c_int
                && value_30 != 5009 as libc::c_int
            {
                current_block = 12343738388509029619;
            } else {
                (*st).variable_duration = value_30;
                current_block = 16167632229894708628;
            }
        }
        4041 => {
            let mut value_31: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_31.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_31 = (*st).variable_duration;
                current_block = 16167632229894708628;
            }
        }
        4042 => {
            let mut value_32: opus_int32 = ap.arg::<opus_int32>();
            if value_32 > 1 as libc::c_int || value_32 < 0 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).silk_mode.reducedDependency = value_32;
                current_block = 16167632229894708628;
            }
        }
        4043 => {
            let mut value_33: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_33.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_33 = (*st).silk_mode.reducedDependency;
                current_block = 16167632229894708628;
            }
        }
        4046 => {
            let mut value_34: opus_int32 = ap.arg::<opus_int32>();
            if value_34 < 0 as libc::c_int || value_34 > 1 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                opus_custom_encoder_ctl(celt_enc, 4046 as libc::c_int, value_34);
                current_block = 16167632229894708628;
            }
        }
        4047 => {
            let mut value_35: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_35.is_null() {
                current_block = 12343738388509029619;
            } else {
                opus_custom_encoder_ctl(
                    celt_enc,
                    4047 as libc::c_int,
                    value_35.offset(value_35.offset_from(value_35) as libc::c_long as isize),
                );
                current_block = 16167632229894708628;
            }
        }
        4028 => {
            let mut silk_enc: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut dummy: silk_EncControlStruct = silk_EncControlStruct {
                nChannelsAPI: 0,
                nChannelsInternal: 0,
                API_sampleRate: 0,
                maxInternalSampleRate: 0,
                minInternalSampleRate: 0,
                desiredInternalSampleRate: 0,
                payloadSize_ms: 0,
                bitRate: 0,
                packetLossPercentage: 0,
                complexity: 0,
                useInBandFEC: 0,
                LBRR_coded: 0,
                useDTX: 0,
                useCBR: 0,
                maxBits: 0,
                toMono: 0,
                opusCanSwitch: 0,
                reducedDependency: 0,
                internalSampleRate: 0,
                allowBandwidthSwitch: 0,
                inWBmodeWithoutVariableLP: 0,
                stereoWidth_Q14: 0,
                switchReady: 0,
                signalType: 0,
                offset: 0,
            };
            let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
            silk_enc = (st as *mut libc::c_char).offset((*st).silk_enc_offset as isize)
                as *mut libc::c_void;
            tonality_analysis_reset(&mut (*st).analysis);
            start = &mut (*st).stream_channels as *mut libc::c_int as *mut libc::c_char;
            memset(
                start as *mut libc::c_void,
                0 as libc::c_int,
                (::core::mem::size_of::<OpusEncoder>() as libc::c_ulong)
                    .wrapping_sub(
                        start.offset_from(st as *mut libc::c_char) as libc::c_long as libc::c_ulong
                    )
                    .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            opus_custom_encoder_ctl(celt_enc, 4028 as libc::c_int);
            silk_InitEncoder(silk_enc, (*st).arch, &mut dummy);
            (*st).stream_channels = (*st).channels;
            (*st).hybrid_stereo_width_Q14 = ((1 as libc::c_int) << 14 as libc::c_int) as opus_int16;
            (*st).prev_HB_gain = 1.0f32;
            (*st).first = 1 as libc::c_int;
            (*st).mode = 1001 as libc::c_int;
            (*st).bandwidth = 1105 as libc::c_int;
            (*st).variable_HP_smth2_Q15 = ((silk_lin2log(60 as libc::c_int) as opus_uint32)
                << 8 as libc::c_int) as opus_int32;
            current_block = 16167632229894708628;
        }
        11002 => {
            let mut value_36: opus_int32 = ap.arg::<opus_int32>();
            if (value_36 < 1000 as libc::c_int || value_36 > 1002 as libc::c_int)
                && value_36 != -(1000 as libc::c_int)
            {
                current_block = 12343738388509029619;
            } else {
                (*st).user_forced_mode = value_36;
                current_block = 16167632229894708628;
            }
        }
        10024 => {
            let mut value_37: opus_int32 = ap.arg::<opus_int32>();
            (*st).lfe = value_37;
            ret = opus_custom_encoder_ctl(celt_enc, 10024 as libc::c_int, value_37);
            current_block = 16167632229894708628;
        }
        10026 => {
            let mut value_38: *mut opus_val16 = ap.arg::<*mut opus_val16>();
            (*st).energy_masking = value_38;
            ret = opus_custom_encoder_ctl(
                celt_enc,
                10026 as libc::c_int,
                value_38.offset(value_38.offset_from(value_38) as libc::c_long as isize),
            );
            current_block = 16167632229894708628;
        }
        4049 => {
            let mut value_39: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_39.is_null() {
                current_block = 12343738388509029619;
            } else {
                if (*st).silk_mode.useDTX != 0
                    && ((*st).prev_mode == 1000 as libc::c_int
                        || (*st).prev_mode == 1001 as libc::c_int)
                {
                    let mut n: libc::c_int = 0;
                    let mut silk_enc_0: *mut libc::c_void = (st as *mut libc::c_char)
                        .offset((*st).silk_enc_offset as isize)
                        as *mut libc::c_void;
                    *value_39 = 1 as libc::c_int;
                    n = 0 as libc::c_int;
                    while n < (*st).silk_mode.nChannelsInternal {
                        *value_39 = (*value_39 != 0
                            && (*(silk_enc_0 as *mut silk_encoder)).state_Fxx[n as usize]
                                .sCmn
                                .noSpeechCounter
                                >= 10 as libc::c_int)
                            as libc::c_int;
                        n += 1;
                    }
                } else if (*st).use_dtx != 0 {
                    *value_39 = ((*st).nb_no_activity_frames >= 10 as libc::c_int) as libc::c_int;
                } else {
                    *value_39 = 0 as libc::c_int;
                }
                current_block = 16167632229894708628;
            }
        }
        10015 => {
            let mut value_40: *mut *const OpusCustomMode = ap.arg::<*mut *const OpusCustomMode>();
            if value_40.is_null() {
                current_block = 12343738388509029619;
            } else {
                ret = opus_custom_encoder_ctl(
                    celt_enc,
                    10015 as libc::c_int,
                    value_40.offset(value_40.offset_from(value_40) as libc::c_long as isize),
                );
                current_block = 16167632229894708628;
            }
        }
        _ => {
            ret = -(5 as libc::c_int);
            current_block = 16167632229894708628;
        }
    }
    match current_block {
        12343738388509029619 => return -(1 as libc::c_int),
        _ => return ret,
    };
}
#[no_mangle]
#[c2rust::src_loc = "2780:1"]
pub unsafe extern "C" fn opus_encoder_destroy(mut st: *mut OpusEncoder) {
    opus_free(st as *mut libc::c_void);
}
