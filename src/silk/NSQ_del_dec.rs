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
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:32"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:32"]
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
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(
            _: *mut libc::c_void,
            _: libc::c_int,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:32"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:32"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int = ::core::mem::size_of::<libc::c_uint>()
        as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/macros.h:32"]
pub mod macros_h {
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn silk_CLZ32(in32: opus_int32) -> opus_int32 {
        return if in32 != 0 {
            32 as libc::c_int - (EC_CLZ0 - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }
    use super::opus_types_h::opus_int32;
    use super::ecintrin_h::EC_CLZ0;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "546:1"]
    pub unsafe extern "C" fn silk_min_int(
        a: libc::c_int,
        b: libc::c_int,
    ) -> libc::c_int {
        return if a < b { a } else { b };
    }
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "123:1"]
        pub fn silk_LPC_analysis_filter(
            out: *mut opus_int16,
            in_0: *const opus_int16,
            B: *const opus_int16,
            len: opus_int32,
            d: opus_int32,
            arch: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/Inlines.h:32"]
pub mod Inlines_h {
    #[inline]
    #[c2rust::src_loc = "97:1"]
    pub unsafe extern "C" fn silk_DIV32_varQ(
        a32: opus_int32,
        b32: opus_int32,
        Qres: libc::c_int,
    ) -> opus_int32 {
        let mut a_headrm: libc::c_int = 0;
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: opus_int32 = 0;
        let mut a32_nrm: opus_int32 = 0;
        let mut b32_nrm: opus_int32 = 0;
        let mut result: opus_int32 = 0;
        a_headrm = silk_CLZ32(if a32 > 0 as libc::c_int { a32 } else { -a32 })
            - 1 as libc::c_int;
        a32_nrm = ((a32 as opus_uint32) << a_headrm) as opus_int32;
        b_headrm = silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 })
            - 1 as libc::c_int;
        b32_nrm = ((b32 as opus_uint32) << b_headrm) as opus_int32;
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int)
            / (b32_nrm >> 16 as libc::c_int);
        result = (a32_nrm as libc::c_long * b32_inv as opus_int16 as opus_int64
            >> 16 as libc::c_int) as opus_int32;
        a32_nrm = (a32_nrm as opus_uint32)
            .wrapping_sub(
                (((b32_nrm as opus_int64 * result as libc::c_long >> 32 as libc::c_int)
                    as opus_int32 as opus_uint32) << 3 as libc::c_int) as opus_int32
                    as opus_uint32,
            ) as opus_int32;
        result = (result as libc::c_long
            + (a32_nrm as libc::c_long * b32_inv as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        lshift = 29 as libc::c_int + a_headrm - b_headrm - Qres;
        if lshift < 0 as libc::c_int {
            return (((if 0x80000000 as libc::c_uint as opus_int32 >> -lshift
                > 0x7fffffff as libc::c_int >> -lshift
            {
                if result > 0x80000000 as libc::c_uint as opus_int32 >> -lshift {
                    0x80000000 as libc::c_uint as opus_int32 >> -lshift
                } else {
                    if result < 0x7fffffff as libc::c_int >> -lshift {
                        0x7fffffff as libc::c_int >> -lshift
                    } else {
                        result
                    }
                }
            } else {
                if result > 0x7fffffff as libc::c_int >> -lshift {
                    0x7fffffff as libc::c_int >> -lshift
                } else {
                    if result < 0x80000000 as libc::c_uint as opus_int32 >> -lshift {
                        0x80000000 as libc::c_uint as opus_int32 >> -lshift
                    } else {
                        result
                    }
                }
            }) as opus_uint32) << -lshift) as opus_int32
        } else if lshift < 32 as libc::c_int {
            return result >> lshift
        } else {
            return 0 as libc::c_int
        };
    }
    #[inline]
    #[c2rust::src_loc = "143:1"]
    pub unsafe extern "C" fn silk_INVERSE32_varQ(
        b32: opus_int32,
        Qres: libc::c_int,
    ) -> opus_int32 {
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: opus_int32 = 0;
        let mut b32_nrm: opus_int32 = 0;
        let mut err_Q32: opus_int32 = 0;
        let mut result: opus_int32 = 0;
        b_headrm = silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 })
            - 1 as libc::c_int;
        b32_nrm = ((b32 as opus_uint32) << b_headrm) as opus_int32;
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int)
            / (b32_nrm >> 16 as libc::c_int);
        result = ((b32_inv as opus_uint32) << 16 as libc::c_int) as opus_int32;
        err_Q32 = (((((1 as libc::c_int) << 29 as libc::c_int)
            - (b32_nrm as libc::c_long * b32_inv as opus_int16 as opus_int64
                >> 16 as libc::c_int) as opus_int32) as opus_uint32) << 3 as libc::c_int)
            as opus_int32;
        result = (result as libc::c_long
            + (err_Q32 as opus_int64 * b32_inv as libc::c_long >> 16 as libc::c_int))
            as opus_int32;
        lshift = 61 as libc::c_int - b_headrm - Qres;
        if lshift <= 0 as libc::c_int {
            return (((if 0x80000000 as libc::c_uint as opus_int32 >> -lshift
                > 0x7fffffff as libc::c_int >> -lshift
            {
                if result > 0x80000000 as libc::c_uint as opus_int32 >> -lshift {
                    0x80000000 as libc::c_uint as opus_int32 >> -lshift
                } else {
                    if result < 0x7fffffff as libc::c_int >> -lshift {
                        0x7fffffff as libc::c_int >> -lshift
                    } else {
                        result
                    }
                }
            } else {
                if result > 0x7fffffff as libc::c_int >> -lshift {
                    0x7fffffff as libc::c_int >> -lshift
                } else {
                    if result < 0x80000000 as libc::c_uint as opus_int32 >> -lshift {
                        0x80000000 as libc::c_uint as opus_int32 >> -lshift
                    } else {
                        result
                    }
                }
            }) as opus_uint32) << -lshift) as opus_int32
        } else if lshift < 32 as libc::c_int {
            return result >> lshift
        } else {
            return 0 as libc::c_int
        };
    }
    use super::opus_types_h::{opus_int32, opus_uint32, opus_int16, opus_int64};
    use super::macros_h::silk_CLZ32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    use super::opus_types_h::opus_int16;
    extern "C" {
        #[c2rust::src_loc = "101:26"]
        pub static silk_Quantization_Offsets_Q10: [[opus_int16; 2]; 2];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "155:9"]
    pub const MAX_SHAPE_LPC_ORDER: libc::c_int = 24 as libc::c_int;
    #[c2rust::src_loc = "157:9"]
    pub const HARM_SHAPE_FIR_TAPS: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "165:9"]
    pub const DECISION_DELAY: libc::c_int = 40 as libc::c_int;
    #[c2rust::src_loc = "146:9"]
    pub const LTP_ORDER: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "142:9"]
    pub const MAX_LPC_ORDER: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "180:10"]
    pub const NSQ_LPC_BUF_LENGTH: libc::c_int = MAX_LPC_ORDER;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/NSQ.h:34"]
pub mod NSQ_h {
    #[inline]
    #[c2rust::src_loc = "35:1"]
    pub unsafe extern "C" fn silk_noise_shape_quantizer_short_prediction_c(
        buf32: *const opus_int32,
        coef16: *const opus_int16,
        order: libc::c_int,
    ) -> opus_int32 {
        let mut out: opus_int32 = 0;
        out = order >> 1 as libc::c_int;
        out = (out as libc::c_long
            + (*buf32.offset(0 as libc::c_int as isize) as libc::c_long
                * *coef16.offset(0 as libc::c_int as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        out = (out as libc::c_long
            + (*buf32.offset(-(1 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(1 as libc::c_int as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        out = (out as libc::c_long
            + (*buf32.offset(-(2 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(2 as libc::c_int as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        out = (out as libc::c_long
            + (*buf32.offset(-(3 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(3 as libc::c_int as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        out = (out as libc::c_long
            + (*buf32.offset(-(4 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(4 as libc::c_int as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        out = (out as libc::c_long
            + (*buf32.offset(-(5 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(5 as libc::c_int as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        out = (out as libc::c_long
            + (*buf32.offset(-(6 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(6 as libc::c_int as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        out = (out as libc::c_long
            + (*buf32.offset(-(7 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(7 as libc::c_int as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        out = (out as libc::c_long
            + (*buf32.offset(-(8 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(8 as libc::c_int as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        out = (out as libc::c_long
            + (*buf32.offset(-(9 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(9 as libc::c_int as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        if order == 16 as libc::c_int {
            out = (out as libc::c_long
                + (*buf32.offset(-(10 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(10 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            out = (out as libc::c_long
                + (*buf32.offset(-(11 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(11 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            out = (out as libc::c_long
                + (*buf32.offset(-(12 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(12 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            out = (out as libc::c_long
                + (*buf32.offset(-(13 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(13 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            out = (out as libc::c_long
                + (*buf32.offset(-(14 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(14 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            out = (out as libc::c_long
                + (*buf32.offset(-(15 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(15 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
        }
        return out;
    }
    use super::opus_types_h::{opus_int32, opus_int16, opus_int64};
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
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
pub use self::typedef_h::{silk_int32_MAX, silk_int16_MIN, silk_int16_MAX};
use self::arch_h::celt_fatal;
use self::string_h::{memmove, memcpy, memset};
pub use self::limits_h::CHAR_BIT;
pub use self::ecintrin_h::EC_CLZ0;
pub use self::macros_h::silk_CLZ32;
pub use self::SigProc_FIX_h::{silk_min_int, silk_LPC_analysis_filter};
pub use self::Inlines_h::{silk_DIV32_varQ, silk_INVERSE32_varQ};
use self::tables_h::silk_Quantization_Offsets_Q10;
pub use self::define_h::{
    MAX_SHAPE_LPC_ORDER, HARM_SHAPE_FIR_TAPS, DECISION_DELAY, LTP_ORDER, TYPE_VOICED,
    MAX_LPC_ORDER, NSQ_LPC_BUF_LENGTH,
};
pub use self::NSQ_h::silk_noise_shape_quantizer_short_prediction_c;
pub use self::internal::__CHAR_BIT__;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "37:9"]
pub struct NSQ_del_dec_struct {
    pub sLPC_Q14: [opus_int32; 96],
    pub RandState: [opus_int32; 40],
    pub Q_Q10: [opus_int32; 40],
    pub Xq_Q14: [opus_int32; 40],
    pub Pred_Q15: [opus_int32; 40],
    pub Shape_Q14: [opus_int32; 40],
    pub sAR2_Q14: [opus_int32; 24],
    pub LF_AR_Q14: opus_int32,
    pub Diff_Q14: opus_int32,
    pub Seed: opus_int32,
    pub SeedInit: opus_int32,
    pub RD_Q10: opus_int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "52:9"]
pub struct NSQ_sample_struct {
    pub Q_Q10: opus_int32,
    pub RD_Q10: opus_int32,
    pub xq_Q14: opus_int32,
    pub LF_AR_Q14: opus_int32,
    pub Diff_Q14: opus_int32,
    pub sLTP_shp_Q14: opus_int32,
    pub LPC_exc_Q14: opus_int32,
}
#[c2rust::src_loc = "62:1"]
pub type NSQ_sample_pair = [NSQ_sample_struct; 2];
#[no_mangle]
#[c2rust::src_loc = "117:1"]
pub unsafe extern "C" fn silk_NSQ_del_dec_c(
    psEncC: *const silk_encoder_state,
    mut NSQ: *mut silk_nsq_state,
    mut psIndices: *mut SideInfoIndices,
    mut x16: *const opus_int16,
    mut pulses: *mut opus_int8,
    PredCoef_Q12: *const opus_int16,
    LTPCoef_Q14: *const opus_int16,
    AR_Q13: *const opus_int16,
    HarmShapeGain_Q14: *const libc::c_int,
    Tilt_Q14: *const libc::c_int,
    LF_shp_Q14: *const opus_int32,
    Gains_Q16: *const opus_int32,
    pitchL: *const libc::c_int,
    Lambda_Q10: libc::c_int,
    LTP_scale_Q14: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut start_idx: libc::c_int = 0;
    let mut LSF_interpolation_flag: libc::c_int = 0;
    let mut Winner_ind: libc::c_int = 0;
    let mut subfr: libc::c_int = 0;
    let mut last_smple_idx: libc::c_int = 0;
    let mut smpl_buf_idx: libc::c_int = 0;
    let mut decisionDelay: libc::c_int = 0;
    let mut A_Q12: *const opus_int16 = 0 as *const opus_int16;
    let mut B_Q14: *const opus_int16 = 0 as *const opus_int16;
    let mut AR_shp_Q13: *const opus_int16 = 0 as *const opus_int16;
    let mut pxq: *mut opus_int16 = 0 as *mut opus_int16;
    let mut HarmShapeFIRPacked_Q14: opus_int32 = 0;
    let mut offset_Q10: libc::c_int = 0;
    let mut RDmin_Q10: opus_int32 = 0;
    let mut Gain_Q10: opus_int32 = 0;
    let mut psDD: *mut NSQ_del_dec_struct = 0 as *mut NSQ_del_dec_struct;
    lag = (*NSQ).lagPrev;
    let vla = (*psEncC).nStatesDelayedDecision as usize;
    let mut psDelDec: Vec::<NSQ_del_dec_struct> = ::std::vec::from_elem(
        NSQ_del_dec_struct {
            sLPC_Q14: [0; 96],
            RandState: [0; 40],
            Q_Q10: [0; 40],
            Xq_Q14: [0; 40],
            Pred_Q15: [0; 40],
            Shape_Q14: [0; 40],
            sAR2_Q14: [0; 24],
            LF_AR_Q14: 0,
            Diff_Q14: 0,
            Seed: 0,
            SeedInit: 0,
            RD_Q10: 0,
        },
        vla,
    );
    memset(
        psDelDec.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((*psEncC).nStatesDelayedDecision as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<NSQ_del_dec_struct>() as libc::c_ulong),
    );
    k = 0 as libc::c_int;
    while k < (*psEncC).nStatesDelayedDecision {
        psDD = &mut *psDelDec.as_mut_ptr().offset(k as isize) as *mut NSQ_del_dec_struct;
        (*psDD).Seed = k + (*psIndices).Seed as libc::c_int & 3 as libc::c_int;
        (*psDD).SeedInit = (*psDD).Seed;
        (*psDD).RD_Q10 = 0 as libc::c_int;
        (*psDD).LF_AR_Q14 = (*NSQ).sLF_AR_shp_Q14;
        (*psDD).Diff_Q14 = (*NSQ).sDiff_shp_Q14;
        (*psDD)
            .Shape_Q14[0 as libc::c_int
            as usize] = (*NSQ)
            .sLTP_shp_Q14[((*psEncC).ltp_mem_length - 1 as libc::c_int) as usize];
        memcpy(
            ((*psDD).sLPC_Q14).as_mut_ptr() as *mut libc::c_void,
            ((*NSQ).sLPC_Q14).as_mut_ptr() as *const libc::c_void,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_int32>() as libc::c_ulong),
        );
        memcpy(
            ((*psDD).sAR2_Q14).as_mut_ptr() as *mut libc::c_void,
            ((*NSQ).sAR2_Q14).as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[opus_int32; 24]>() as libc::c_ulong,
        );
        k += 1;
    }
    offset_Q10 = silk_Quantization_Offsets_Q10[((*psIndices).signalType as libc::c_int
        >> 1 as libc::c_int) as usize][(*psIndices).quantOffsetType as usize]
        as libc::c_int;
    smpl_buf_idx = 0 as libc::c_int;
    decisionDelay = silk_min_int(DECISION_DELAY, (*psEncC).subfr_length);
    if (*psIndices).signalType as libc::c_int == TYPE_VOICED {
        k = 0 as libc::c_int;
        while k < (*psEncC).nb_subfr {
            decisionDelay = silk_min_int(
                decisionDelay,
                *pitchL.offset(k as isize) - LTP_ORDER / 2 as libc::c_int
                    - 1 as libc::c_int,
            );
            k += 1;
        }
    } else if lag > 0 as libc::c_int {
        decisionDelay = silk_min_int(
            decisionDelay,
            lag - LTP_ORDER / 2 as libc::c_int - 1 as libc::c_int,
        );
    }
    if (*psIndices).NLSFInterpCoef_Q2 as libc::c_int == 4 as libc::c_int {
        LSF_interpolation_flag = 0 as libc::c_int;
    } else {
        LSF_interpolation_flag = 1 as libc::c_int;
    }
    let vla_0 = ((*psEncC).ltp_mem_length + (*psEncC).frame_length) as usize;
    let mut sLTP_Q15: Vec::<opus_int32> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = ((*psEncC).ltp_mem_length + (*psEncC).frame_length) as usize;
    let mut sLTP: Vec::<opus_int16> = ::std::vec::from_elem(0, vla_1);
    let vla_2 = (*psEncC).subfr_length as usize;
    let mut x_sc_Q10: Vec::<opus_int32> = ::std::vec::from_elem(0, vla_2);
    let mut delayedGain_Q10: [opus_int32; 40] = [0; 40];
    pxq = &mut *((*NSQ).xq).as_mut_ptr().offset((*psEncC).ltp_mem_length as isize)
        as *mut opus_int16;
    (*NSQ).sLTP_shp_buf_idx = (*psEncC).ltp_mem_length;
    (*NSQ).sLTP_buf_idx = (*psEncC).ltp_mem_length;
    subfr = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < (*psEncC).nb_subfr {
        A_Q12 = &*PredCoef_Q12
            .offset(
                ((k >> 1 as libc::c_int | 1 as libc::c_int - LSF_interpolation_flag)
                    * MAX_LPC_ORDER) as isize,
            ) as *const opus_int16;
        B_Q14 = &*LTPCoef_Q14.offset((k * LTP_ORDER) as isize) as *const opus_int16;
        AR_shp_Q13 = &*AR_Q13.offset((k * MAX_SHAPE_LPC_ORDER) as isize)
            as *const opus_int16;
        HarmShapeFIRPacked_Q14 = *HarmShapeGain_Q14.offset(k as isize)
            >> 2 as libc::c_int;
        HarmShapeFIRPacked_Q14
            |= (((*HarmShapeGain_Q14.offset(k as isize) >> 1 as libc::c_int)
                as opus_uint32) << 16 as libc::c_int) as opus_int32;
        (*NSQ).rewhite_flag = 0 as libc::c_int;
        if (*psIndices).signalType as libc::c_int == TYPE_VOICED {
            lag = *pitchL.offset(k as isize);
            if k
                & 3 as libc::c_int
                    - ((LSF_interpolation_flag as opus_uint32) << 1 as libc::c_int)
                        as opus_int32 == 0 as libc::c_int
            {
                if k == 2 as libc::c_int {
                    RDmin_Q10 = (*psDelDec
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .RD_Q10;
                    Winner_ind = 0 as libc::c_int;
                    i = 1 as libc::c_int;
                    while i < (*psEncC).nStatesDelayedDecision {
                        if (*psDelDec.as_mut_ptr().offset(i as isize)).RD_Q10 < RDmin_Q10
                        {
                            RDmin_Q10 = (*psDelDec.as_mut_ptr().offset(i as isize))
                                .RD_Q10;
                            Winner_ind = i;
                        }
                        i += 1;
                    }
                    i = 0 as libc::c_int;
                    while i < (*psEncC).nStatesDelayedDecision {
                        if i != Winner_ind {
                            let ref mut fresh0 = (*psDelDec
                                .as_mut_ptr()
                                .offset(i as isize))
                                .RD_Q10;
                            *fresh0 += silk_int32_MAX >> 4 as libc::c_int;
                        }
                        i += 1;
                    }
                    psDD = &mut *psDelDec.as_mut_ptr().offset(Winner_ind as isize)
                        as *mut NSQ_del_dec_struct;
                    last_smple_idx = smpl_buf_idx + decisionDelay;
                    i = 0 as libc::c_int;
                    while i < decisionDelay {
                        last_smple_idx = (last_smple_idx - 1 as libc::c_int)
                            % DECISION_DELAY;
                        if last_smple_idx < 0 as libc::c_int {
                            last_smple_idx += DECISION_DELAY;
                        }
                        *pulses
                            .offset(
                                (i - decisionDelay) as isize,
                            ) = (if 10 as libc::c_int == 1 as libc::c_int {
                            ((*psDD).Q_Q10[last_smple_idx as usize] >> 1 as libc::c_int)
                                + ((*psDD).Q_Q10[last_smple_idx as usize]
                                    & 1 as libc::c_int)
                        } else {
                            ((*psDD).Q_Q10[last_smple_idx as usize]
                                >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) as opus_int8;
                        *pxq
                            .offset(
                                (i - decisionDelay) as isize,
                            ) = (if (if 14 as libc::c_int == 1 as libc::c_int {
                            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                                * *Gains_Q16.offset(1 as libc::c_int as isize)
                                    as libc::c_long >> 16 as libc::c_int) as opus_int32
                                >> 1 as libc::c_int)
                                + (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                                    * *Gains_Q16.offset(1 as libc::c_int as isize)
                                        as libc::c_long >> 16 as libc::c_int) as opus_int32
                                    & 1 as libc::c_int)
                        } else {
                            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                                * *Gains_Q16.offset(1 as libc::c_int as isize)
                                    as libc::c_long >> 16 as libc::c_int) as opus_int32
                                >> 14 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) > silk_int16_MAX
                        {
                            silk_int16_MAX
                        } else if (if 14 as libc::c_int == 1 as libc::c_int {
                            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                                * *Gains_Q16.offset(1 as libc::c_int as isize)
                                    as libc::c_long >> 16 as libc::c_int) as opus_int32
                                >> 1 as libc::c_int)
                                + (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                                    * *Gains_Q16.offset(1 as libc::c_int as isize)
                                        as libc::c_long >> 16 as libc::c_int) as opus_int32
                                    & 1 as libc::c_int)
                        } else {
                            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                                * *Gains_Q16.offset(1 as libc::c_int as isize)
                                    as libc::c_long >> 16 as libc::c_int) as opus_int32
                                >> 14 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) < silk_int16_MIN
                        {
                            silk_int16_MIN
                        } else if 14 as libc::c_int == 1 as libc::c_int {
                            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                                * *Gains_Q16.offset(1 as libc::c_int as isize)
                                    as libc::c_long >> 16 as libc::c_int) as opus_int32
                                >> 1 as libc::c_int)
                                + (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                                    * *Gains_Q16.offset(1 as libc::c_int as isize)
                                        as libc::c_long >> 16 as libc::c_int) as opus_int32
                                    & 1 as libc::c_int)
                        } else {
                            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                                * *Gains_Q16.offset(1 as libc::c_int as isize)
                                    as libc::c_long >> 16 as libc::c_int) as opus_int32
                                >> 14 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) as opus_int16;
                        (*NSQ)
                            .sLTP_shp_Q14[((*NSQ).sLTP_shp_buf_idx - decisionDelay + i)
                            as usize] = (*psDD).Shape_Q14[last_smple_idx as usize];
                        i += 1;
                    }
                    subfr = 0 as libc::c_int;
                }
                start_idx = (*psEncC).ltp_mem_length - lag - (*psEncC).predictLPCOrder
                    - LTP_ORDER / 2 as libc::c_int;
                if !(start_idx > 0 as libc::c_int) {
                    celt_fatal(
                        b"assertion failed: start_idx > 0\0" as *const u8
                            as *const libc::c_char,
                        b"silk/NSQ_del_dec.c\0" as *const u8 as *const libc::c_char,
                        253 as libc::c_int,
                    );
                }
                silk_LPC_analysis_filter(
                    &mut *sLTP.as_mut_ptr().offset(start_idx as isize),
                    &mut *((*NSQ).xq)
                        .as_mut_ptr()
                        .offset((start_idx + k * (*psEncC).subfr_length) as isize),
                    A_Q12,
                    (*psEncC).ltp_mem_length - start_idx,
                    (*psEncC).predictLPCOrder,
                    (*psEncC).arch,
                );
                (*NSQ).sLTP_buf_idx = (*psEncC).ltp_mem_length;
                (*NSQ).rewhite_flag = 1 as libc::c_int;
            }
        }
        silk_nsq_del_dec_scale_states(
            psEncC,
            NSQ,
            psDelDec.as_mut_ptr(),
            x16,
            x_sc_Q10.as_mut_ptr(),
            sLTP.as_mut_ptr() as *const opus_int16,
            sLTP_Q15.as_mut_ptr(),
            k,
            (*psEncC).nStatesDelayedDecision,
            LTP_scale_Q14,
            Gains_Q16,
            pitchL,
            (*psIndices).signalType as libc::c_int,
            decisionDelay,
        );
        let fresh1 = subfr;
        subfr = subfr + 1;
        silk_noise_shape_quantizer_del_dec(
            NSQ,
            psDelDec.as_mut_ptr(),
            (*psIndices).signalType as libc::c_int,
            x_sc_Q10.as_mut_ptr() as *const opus_int32,
            pulses,
            pxq,
            sLTP_Q15.as_mut_ptr(),
            delayedGain_Q10.as_mut_ptr(),
            A_Q12,
            B_Q14,
            AR_shp_Q13,
            lag,
            HarmShapeFIRPacked_Q14,
            *Tilt_Q14.offset(k as isize),
            *LF_shp_Q14.offset(k as isize),
            *Gains_Q16.offset(k as isize),
            Lambda_Q10,
            offset_Q10,
            (*psEncC).subfr_length,
            fresh1,
            (*psEncC).shapingLPCOrder,
            (*psEncC).predictLPCOrder,
            (*psEncC).warping_Q16,
            (*psEncC).nStatesDelayedDecision,
            &mut smpl_buf_idx,
            decisionDelay,
            (*psEncC).arch,
        );
        x16 = x16.offset((*psEncC).subfr_length as isize);
        pulses = pulses.offset((*psEncC).subfr_length as isize);
        pxq = pxq.offset((*psEncC).subfr_length as isize);
        k += 1;
    }
    RDmin_Q10 = (*psDelDec.as_mut_ptr().offset(0 as libc::c_int as isize)).RD_Q10;
    Winner_ind = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k < (*psEncC).nStatesDelayedDecision {
        if (*psDelDec.as_mut_ptr().offset(k as isize)).RD_Q10 < RDmin_Q10 {
            RDmin_Q10 = (*psDelDec.as_mut_ptr().offset(k as isize)).RD_Q10;
            Winner_ind = k;
        }
        k += 1;
    }
    psDD = &mut *psDelDec.as_mut_ptr().offset(Winner_ind as isize)
        as *mut NSQ_del_dec_struct;
    (*psIndices).Seed = (*psDD).SeedInit as opus_int8;
    last_smple_idx = smpl_buf_idx + decisionDelay;
    Gain_Q10 = *Gains_Q16.offset(((*psEncC).nb_subfr - 1 as libc::c_int) as isize)
        >> 6 as libc::c_int;
    i = 0 as libc::c_int;
    while i < decisionDelay {
        last_smple_idx = (last_smple_idx - 1 as libc::c_int) % DECISION_DELAY;
        if last_smple_idx < 0 as libc::c_int {
            last_smple_idx += DECISION_DELAY;
        }
        *pulses
            .offset(
                (i - decisionDelay) as isize,
            ) = (if 10 as libc::c_int == 1 as libc::c_int {
            ((*psDD).Q_Q10[last_smple_idx as usize] >> 1 as libc::c_int)
                + ((*psDD).Q_Q10[last_smple_idx as usize] & 1 as libc::c_int)
        } else {
            ((*psDD).Q_Q10[last_smple_idx as usize]
                >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as opus_int8;
        *pxq
            .offset(
                (i - decisionDelay) as isize,
            ) = (if (if 8 as libc::c_int == 1 as libc::c_int {
            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as opus_int32
                >> 1 as libc::c_int)
                + (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                    * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as opus_int32
                    & 1 as libc::c_int)
        } else {
            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as opus_int32
                >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as opus_int32
                >> 1 as libc::c_int)
                + (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                    * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as opus_int32
                    & 1 as libc::c_int)
        } else {
            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as opus_int32
                >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 8 as libc::c_int == 1 as libc::c_int {
            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as opus_int32
                >> 1 as libc::c_int)
                + (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                    * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as opus_int32
                    & 1 as libc::c_int)
        } else {
            (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as opus_int32
                >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as opus_int16;
        (*NSQ)
            .sLTP_shp_Q14[((*NSQ).sLTP_shp_buf_idx - decisionDelay + i)
            as usize] = (*psDD).Shape_Q14[last_smple_idx as usize];
        i += 1;
    }
    memcpy(
        ((*NSQ).sLPC_Q14).as_mut_ptr() as *mut libc::c_void,
        &mut *((*psDD).sLPC_Q14).as_mut_ptr().offset((*psEncC).subfr_length as isize)
            as *mut opus_int32 as *const libc::c_void,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int32>() as libc::c_ulong),
    );
    memcpy(
        ((*NSQ).sAR2_Q14).as_mut_ptr() as *mut libc::c_void,
        ((*psDD).sAR2_Q14).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[opus_int32; 24]>() as libc::c_ulong,
    );
    (*NSQ).sLF_AR_shp_Q14 = (*psDD).LF_AR_Q14;
    (*NSQ).sDiff_shp_Q14 = (*psDD).Diff_Q14;
    (*NSQ).lagPrev = *pitchL.offset(((*psEncC).nb_subfr - 1 as libc::c_int) as isize);
    memmove(
        ((*NSQ).xq).as_mut_ptr() as *mut libc::c_void,
        &mut *((*NSQ).xq).as_mut_ptr().offset((*psEncC).frame_length as isize)
            as *mut opus_int16 as *const libc::c_void,
        ((*psEncC).ltp_mem_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
    memmove(
        ((*NSQ).sLTP_shp_Q14).as_mut_ptr() as *mut libc::c_void,
        &mut *((*NSQ).sLTP_shp_Q14).as_mut_ptr().offset((*psEncC).frame_length as isize)
            as *mut opus_int32 as *const libc::c_void,
        ((*psEncC).ltp_mem_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int32>() as libc::c_ulong),
    );
}
#[inline]
#[c2rust::src_loc = "318:1"]
unsafe extern "C" fn silk_noise_shape_quantizer_del_dec(
    mut NSQ: *mut silk_nsq_state,
    psDelDec: *mut NSQ_del_dec_struct,
    signalType: libc::c_int,
    x_Q10: *const opus_int32,
    pulses: *mut opus_int8,
    xq: *mut opus_int16,
    sLTP_Q15: *mut opus_int32,
    delayedGain_Q10: *mut opus_int32,
    a_Q12: *const opus_int16,
    b_Q14: *const opus_int16,
    AR_shp_Q13: *const opus_int16,
    lag: libc::c_int,
    HarmShapeFIRPacked_Q14: opus_int32,
    Tilt_Q14: libc::c_int,
    LF_shp_Q14: opus_int32,
    Gain_Q16: opus_int32,
    Lambda_Q10: libc::c_int,
    offset_Q10: libc::c_int,
    length: libc::c_int,
    subfr: libc::c_int,
    shapingLPCOrder: libc::c_int,
    predictLPCOrder: libc::c_int,
    warping_Q16: libc::c_int,
    nStatesDelayedDecision: libc::c_int,
    smpl_buf_idx: *mut libc::c_int,
    decisionDelay: libc::c_int,
    _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut Winner_ind: libc::c_int = 0;
    let mut RDmin_ind: libc::c_int = 0;
    let mut RDmax_ind: libc::c_int = 0;
    let mut last_smple_idx: libc::c_int = 0;
    let mut Winner_rand_state: opus_int32 = 0;
    let mut LTP_pred_Q14: opus_int32 = 0;
    let mut LPC_pred_Q14: opus_int32 = 0;
    let mut n_AR_Q14: opus_int32 = 0;
    let mut n_LTP_Q14: opus_int32 = 0;
    let mut n_LF_Q14: opus_int32 = 0;
    let mut r_Q10: opus_int32 = 0;
    let mut rr_Q10: opus_int32 = 0;
    let mut rd1_Q10: opus_int32 = 0;
    let mut rd2_Q10: opus_int32 = 0;
    let mut RDmin_Q10: opus_int32 = 0;
    let mut RDmax_Q10: opus_int32 = 0;
    let mut q1_Q0: opus_int32 = 0;
    let mut q1_Q10: opus_int32 = 0;
    let mut q2_Q10: opus_int32 = 0;
    let mut exc_Q14: opus_int32 = 0;
    let mut LPC_exc_Q14: opus_int32 = 0;
    let mut xq_Q14: opus_int32 = 0;
    let mut Gain_Q10: opus_int32 = 0;
    let mut tmp1: opus_int32 = 0;
    let mut tmp2: opus_int32 = 0;
    let mut sLF_AR_shp_Q14: opus_int32 = 0;
    let mut pred_lag_ptr: *mut opus_int32 = 0 as *mut opus_int32;
    let mut shp_lag_ptr: *mut opus_int32 = 0 as *mut opus_int32;
    let mut psLPC_Q14: *mut opus_int32 = 0 as *mut opus_int32;
    let mut psDD: *mut NSQ_del_dec_struct = 0 as *mut NSQ_del_dec_struct;
    let mut psSS: *mut NSQ_sample_struct = 0 as *mut NSQ_sample_struct;
    if !(nStatesDelayedDecision > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: nStatesDelayedDecision > 0\0" as *const u8
                as *const libc::c_char,
            b"silk/NSQ_del_dec.c\0" as *const u8 as *const libc::c_char,
            364 as libc::c_int,
        );
    }
    let vla = nStatesDelayedDecision as usize;
    let mut psSampleState: Vec::<NSQ_sample_pair> = ::std::vec::from_elem(
        [NSQ_sample_struct {
            Q_Q10: 0,
            RD_Q10: 0,
            xq_Q14: 0,
            LF_AR_Q14: 0,
            Diff_Q14: 0,
            sLTP_shp_Q14: 0,
            LPC_exc_Q14: 0,
        }; 2],
        vla,
    );
    shp_lag_ptr = &mut *((*NSQ).sLTP_shp_Q14)
        .as_mut_ptr()
        .offset(
            ((*NSQ).sLTP_shp_buf_idx - lag + HARM_SHAPE_FIR_TAPS / 2 as libc::c_int)
                as isize,
        ) as *mut opus_int32;
    pred_lag_ptr = &mut *sLTP_Q15
        .offset(((*NSQ).sLTP_buf_idx - lag + LTP_ORDER / 2 as libc::c_int) as isize)
        as *mut opus_int32;
    Gain_Q10 = Gain_Q16 >> 6 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        if signalType == TYPE_VOICED {
            LTP_pred_Q14 = 2 as libc::c_int;
            LTP_pred_Q14 = (LTP_pred_Q14 as libc::c_long
                + (*pred_lag_ptr.offset(0 as libc::c_int as isize) as libc::c_long
                    * *b_Q14.offset(0 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            LTP_pred_Q14 = (LTP_pred_Q14 as libc::c_long
                + (*pred_lag_ptr.offset(-(1 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(1 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            LTP_pred_Q14 = (LTP_pred_Q14 as libc::c_long
                + (*pred_lag_ptr.offset(-(2 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(2 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            LTP_pred_Q14 = (LTP_pred_Q14 as libc::c_long
                + (*pred_lag_ptr.offset(-(3 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(3 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            LTP_pred_Q14 = (LTP_pred_Q14 as libc::c_long
                + (*pred_lag_ptr.offset(-(4 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(4 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            LTP_pred_Q14 = ((LTP_pred_Q14 as opus_uint32) << 1 as libc::c_int)
                as opus_int32;
            pred_lag_ptr = pred_lag_ptr.offset(1);
        } else {
            LTP_pred_Q14 = 0 as libc::c_int;
        }
        if lag > 0 as libc::c_int {
            n_LTP_Q14 = ((*shp_lag_ptr.offset(0 as libc::c_int as isize)
                + *shp_lag_ptr.offset(-(2 as libc::c_int) as isize)) as libc::c_long
                * HarmShapeFIRPacked_Q14 as opus_int16 as opus_int64
                >> 16 as libc::c_int) as opus_int32;
            n_LTP_Q14 = (n_LTP_Q14 as libc::c_long
                + (*shp_lag_ptr.offset(-(1 as libc::c_int) as isize) as libc::c_long
                    * (HarmShapeFIRPacked_Q14 as opus_int64 >> 16 as libc::c_int)
                    >> 16 as libc::c_int)) as opus_int32;
            n_LTP_Q14 = LTP_pred_Q14
                - ((n_LTP_Q14 as opus_uint32) << 2 as libc::c_int) as opus_int32;
            shp_lag_ptr = shp_lag_ptr.offset(1);
        } else {
            n_LTP_Q14 = 0 as libc::c_int;
        }
        k = 0 as libc::c_int;
        while k < nStatesDelayedDecision {
            psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
            psSS = (*psSampleState.as_mut_ptr().offset(k as isize)).as_mut_ptr();
            (*psDD)
                .Seed = (907633515 as libc::c_int as opus_uint32)
                .wrapping_add(
                    ((*psDD).Seed as opus_uint32)
                        .wrapping_mul(196314165 as libc::c_int as opus_uint32),
                ) as opus_int32;
            psLPC_Q14 = &mut *((*psDD).sLPC_Q14)
                .as_mut_ptr()
                .offset((NSQ_LPC_BUF_LENGTH - 1 as libc::c_int + i) as isize)
                as *mut opus_int32;
            LPC_pred_Q14 = silk_noise_shape_quantizer_short_prediction_c(
                psLPC_Q14,
                a_Q12,
                predictLPCOrder,
            );
            LPC_pred_Q14 = ((LPC_pred_Q14 as opus_uint32) << 4 as libc::c_int)
                as opus_int32;
            if !(shapingLPCOrder & 1 as libc::c_int == 0 as libc::c_int) {
                celt_fatal(
                    b"assertion failed: ( shapingLPCOrder & 1 ) == 0\0" as *const u8
                        as *const libc::c_char,
                    b"silk/NSQ_del_dec.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                );
            }
            tmp2 = ((*psDD).Diff_Q14 as libc::c_long
                + ((*psDD).sAR2_Q14[0 as libc::c_int as usize] as libc::c_long
                    * warping_Q16 as opus_int16 as opus_int64 >> 16 as libc::c_int))
                as opus_int32;
            tmp1 = ((*psDD).sAR2_Q14[0 as libc::c_int as usize] as libc::c_long
                + (((*psDD).sAR2_Q14[1 as libc::c_int as usize] - tmp2) as libc::c_long
                    * warping_Q16 as opus_int16 as opus_int64 >> 16 as libc::c_int))
                as opus_int32;
            (*psDD).sAR2_Q14[0 as libc::c_int as usize] = tmp2;
            n_AR_Q14 = shapingLPCOrder >> 1 as libc::c_int;
            n_AR_Q14 = (n_AR_Q14 as libc::c_long
                + (tmp2 as libc::c_long
                    * *AR_shp_Q13.offset(0 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            j = 2 as libc::c_int;
            while j < shapingLPCOrder {
                tmp2 = ((*psDD).sAR2_Q14[(j - 1 as libc::c_int) as usize] as libc::c_long
                    + (((*psDD).sAR2_Q14[(j + 0 as libc::c_int) as usize] - tmp1)
                        as libc::c_long * warping_Q16 as opus_int16 as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                (*psDD).sAR2_Q14[(j - 1 as libc::c_int) as usize] = tmp1;
                n_AR_Q14 = (n_AR_Q14 as libc::c_long
                    + (tmp1 as libc::c_long
                        * *AR_shp_Q13.offset((j - 1 as libc::c_int) as isize)
                            as opus_int64 >> 16 as libc::c_int)) as opus_int32;
                tmp1 = ((*psDD).sAR2_Q14[(j + 0 as libc::c_int) as usize] as libc::c_long
                    + (((*psDD).sAR2_Q14[(j + 1 as libc::c_int) as usize] - tmp2)
                        as libc::c_long * warping_Q16 as opus_int16 as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                (*psDD).sAR2_Q14[(j + 0 as libc::c_int) as usize] = tmp2;
                n_AR_Q14 = (n_AR_Q14 as libc::c_long
                    + (tmp2 as libc::c_long
                        * *AR_shp_Q13.offset(j as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                j += 2 as libc::c_int;
            }
            (*psDD).sAR2_Q14[(shapingLPCOrder - 1 as libc::c_int) as usize] = tmp1;
            n_AR_Q14 = (n_AR_Q14 as libc::c_long
                + (tmp1 as libc::c_long
                    * *AR_shp_Q13.offset((shapingLPCOrder - 1 as libc::c_int) as isize)
                        as opus_int64 >> 16 as libc::c_int)) as opus_int32;
            n_AR_Q14 = ((n_AR_Q14 as opus_uint32) << 1 as libc::c_int) as opus_int32;
            n_AR_Q14 = (n_AR_Q14 as libc::c_long
                + ((*psDD).LF_AR_Q14 as libc::c_long
                    * Tilt_Q14 as opus_int16 as opus_int64 >> 16 as libc::c_int))
                as opus_int32;
            n_AR_Q14 = ((n_AR_Q14 as opus_uint32) << 2 as libc::c_int) as opus_int32;
            n_LF_Q14 = ((*psDD).Shape_Q14[*smpl_buf_idx as usize] as libc::c_long
                * LF_shp_Q14 as opus_int16 as opus_int64 >> 16 as libc::c_int)
                as opus_int32;
            n_LF_Q14 = (n_LF_Q14 as libc::c_long
                + ((*psDD).LF_AR_Q14 as libc::c_long
                    * (LF_shp_Q14 as opus_int64 >> 16 as libc::c_int)
                    >> 16 as libc::c_int)) as opus_int32;
            n_LF_Q14 = ((n_LF_Q14 as opus_uint32) << 2 as libc::c_int) as opus_int32;
            tmp1 = n_AR_Q14 + n_LF_Q14;
            tmp2 = n_LTP_Q14 + LPC_pred_Q14;
            tmp1 = tmp2 - tmp1;
            tmp1 = if 4 as libc::c_int == 1 as libc::c_int {
                (tmp1 >> 1 as libc::c_int) + (tmp1 & 1 as libc::c_int)
            } else {
                (tmp1 >> 4 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            };
            r_Q10 = *x_Q10.offset(i as isize) - tmp1;
            if (*psDD).Seed < 0 as libc::c_int {
                r_Q10 = -r_Q10;
            }
            r_Q10 = if -((31 as libc::c_int) << 10 as libc::c_int)
                > (30 as libc::c_int) << 10 as libc::c_int
            {
                if r_Q10 > -((31 as libc::c_int) << 10 as libc::c_int) {
                    -((31 as libc::c_int) << 10 as libc::c_int)
                } else if r_Q10 < (30 as libc::c_int) << 10 as libc::c_int {
                    (30 as libc::c_int) << 10 as libc::c_int
                } else {
                    r_Q10
                }
            } else if r_Q10 > (30 as libc::c_int) << 10 as libc::c_int {
                (30 as libc::c_int) << 10 as libc::c_int
            } else if r_Q10 < -((31 as libc::c_int) << 10 as libc::c_int) {
                -((31 as libc::c_int) << 10 as libc::c_int)
            } else {
                r_Q10
            };
            q1_Q10 = r_Q10 - offset_Q10;
            q1_Q0 = q1_Q10 >> 10 as libc::c_int;
            if Lambda_Q10 > 2048 as libc::c_int {
                let rdo_offset: libc::c_int = Lambda_Q10 / 2 as libc::c_int
                    - 512 as libc::c_int;
                if q1_Q10 > rdo_offset {
                    q1_Q0 = q1_Q10 - rdo_offset >> 10 as libc::c_int;
                } else if q1_Q10 < -rdo_offset {
                    q1_Q0 = q1_Q10 + rdo_offset >> 10 as libc::c_int;
                } else if q1_Q10 < 0 as libc::c_int {
                    q1_Q0 = -(1 as libc::c_int);
                } else {
                    q1_Q0 = 0 as libc::c_int;
                }
            }
            if q1_Q0 > 0 as libc::c_int {
                q1_Q10 = ((q1_Q0 as opus_uint32) << 10 as libc::c_int) as opus_int32
                    - 80 as libc::c_int;
                q1_Q10 = q1_Q10 + offset_Q10;
                q2_Q10 = q1_Q10 + 1024 as libc::c_int;
                rd1_Q10 = q1_Q10 as opus_int16 as opus_int32
                    * Lambda_Q10 as opus_int16 as opus_int32;
                rd2_Q10 = q2_Q10 as opus_int16 as opus_int32
                    * Lambda_Q10 as opus_int16 as opus_int32;
            } else if q1_Q0 == 0 as libc::c_int {
                q1_Q10 = offset_Q10;
                q2_Q10 = q1_Q10 + (1024 as libc::c_int - 80 as libc::c_int);
                rd1_Q10 = q1_Q10 as opus_int16 as opus_int32
                    * Lambda_Q10 as opus_int16 as opus_int32;
                rd2_Q10 = q2_Q10 as opus_int16 as opus_int32
                    * Lambda_Q10 as opus_int16 as opus_int32;
            } else if q1_Q0 == -(1 as libc::c_int) {
                q2_Q10 = offset_Q10;
                q1_Q10 = q2_Q10 - (1024 as libc::c_int - 80 as libc::c_int);
                rd1_Q10 = -q1_Q10 as opus_int16 as opus_int32
                    * Lambda_Q10 as opus_int16 as opus_int32;
                rd2_Q10 = q2_Q10 as opus_int16 as opus_int32
                    * Lambda_Q10 as opus_int16 as opus_int32;
            } else {
                q1_Q10 = ((q1_Q0 as opus_uint32) << 10 as libc::c_int) as opus_int32
                    + 80 as libc::c_int;
                q1_Q10 = q1_Q10 + offset_Q10;
                q2_Q10 = q1_Q10 + 1024 as libc::c_int;
                rd1_Q10 = -q1_Q10 as opus_int16 as opus_int32
                    * Lambda_Q10 as opus_int16 as opus_int32;
                rd2_Q10 = -q2_Q10 as opus_int16 as opus_int32
                    * Lambda_Q10 as opus_int16 as opus_int32;
            }
            rr_Q10 = r_Q10 - q1_Q10;
            rd1_Q10 = rd1_Q10
                + rr_Q10 as opus_int16 as opus_int32 * rr_Q10 as opus_int16 as opus_int32
                >> 10 as libc::c_int;
            rr_Q10 = r_Q10 - q2_Q10;
            rd2_Q10 = rd2_Q10
                + rr_Q10 as opus_int16 as opus_int32 * rr_Q10 as opus_int16 as opus_int32
                >> 10 as libc::c_int;
            if rd1_Q10 < rd2_Q10 {
                (*psSS.offset(0 as libc::c_int as isize))
                    .RD_Q10 = (*psDD).RD_Q10 + rd1_Q10;
                (*psSS.offset(1 as libc::c_int as isize))
                    .RD_Q10 = (*psDD).RD_Q10 + rd2_Q10;
                (*psSS.offset(0 as libc::c_int as isize)).Q_Q10 = q1_Q10;
                (*psSS.offset(1 as libc::c_int as isize)).Q_Q10 = q2_Q10;
            } else {
                (*psSS.offset(0 as libc::c_int as isize))
                    .RD_Q10 = (*psDD).RD_Q10 + rd2_Q10;
                (*psSS.offset(1 as libc::c_int as isize))
                    .RD_Q10 = (*psDD).RD_Q10 + rd1_Q10;
                (*psSS.offset(0 as libc::c_int as isize)).Q_Q10 = q2_Q10;
                (*psSS.offset(1 as libc::c_int as isize)).Q_Q10 = q1_Q10;
            }
            exc_Q14 = (((*psSS.offset(0 as libc::c_int as isize)).Q_Q10 as opus_uint32)
                << 4 as libc::c_int) as opus_int32;
            if (*psDD).Seed < 0 as libc::c_int {
                exc_Q14 = -exc_Q14;
            }
            LPC_exc_Q14 = exc_Q14 + LTP_pred_Q14;
            xq_Q14 = LPC_exc_Q14 + LPC_pred_Q14;
            (*psSS.offset(0 as libc::c_int as isize))
                .Diff_Q14 = xq_Q14
                - ((*x_Q10.offset(i as isize) as opus_uint32) << 4 as libc::c_int)
                    as opus_int32;
            sLF_AR_shp_Q14 = (*psSS.offset(0 as libc::c_int as isize)).Diff_Q14
                - n_AR_Q14;
            (*psSS.offset(0 as libc::c_int as isize))
                .sLTP_shp_Q14 = sLF_AR_shp_Q14 - n_LF_Q14;
            (*psSS.offset(0 as libc::c_int as isize)).LF_AR_Q14 = sLF_AR_shp_Q14;
            (*psSS.offset(0 as libc::c_int as isize)).LPC_exc_Q14 = LPC_exc_Q14;
            (*psSS.offset(0 as libc::c_int as isize)).xq_Q14 = xq_Q14;
            exc_Q14 = (((*psSS.offset(1 as libc::c_int as isize)).Q_Q10 as opus_uint32)
                << 4 as libc::c_int) as opus_int32;
            if (*psDD).Seed < 0 as libc::c_int {
                exc_Q14 = -exc_Q14;
            }
            LPC_exc_Q14 = exc_Q14 + LTP_pred_Q14;
            xq_Q14 = LPC_exc_Q14 + LPC_pred_Q14;
            (*psSS.offset(1 as libc::c_int as isize))
                .Diff_Q14 = xq_Q14
                - ((*x_Q10.offset(i as isize) as opus_uint32) << 4 as libc::c_int)
                    as opus_int32;
            sLF_AR_shp_Q14 = (*psSS.offset(1 as libc::c_int as isize)).Diff_Q14
                - n_AR_Q14;
            (*psSS.offset(1 as libc::c_int as isize))
                .sLTP_shp_Q14 = sLF_AR_shp_Q14 - n_LF_Q14;
            (*psSS.offset(1 as libc::c_int as isize)).LF_AR_Q14 = sLF_AR_shp_Q14;
            (*psSS.offset(1 as libc::c_int as isize)).LPC_exc_Q14 = LPC_exc_Q14;
            (*psSS.offset(1 as libc::c_int as isize)).xq_Q14 = xq_Q14;
            k += 1;
        }
        *smpl_buf_idx = (*smpl_buf_idx - 1 as libc::c_int) % DECISION_DELAY;
        if *smpl_buf_idx < 0 as libc::c_int {
            *smpl_buf_idx += DECISION_DELAY;
        }
        last_smple_idx = (*smpl_buf_idx + decisionDelay) % DECISION_DELAY;
        RDmin_Q10 = (*psSampleState
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            .RD_Q10;
        Winner_ind = 0 as libc::c_int;
        k = 1 as libc::c_int;
        while k < nStatesDelayedDecision {
            if (*psSampleState
                .as_mut_ptr()
                .offset(k as isize))[0 as libc::c_int as usize]
                .RD_Q10 < RDmin_Q10
            {
                RDmin_Q10 = (*psSampleState
                    .as_mut_ptr()
                    .offset(k as isize))[0 as libc::c_int as usize]
                    .RD_Q10;
                Winner_ind = k;
            }
            k += 1;
        }
        Winner_rand_state = (*psDelDec.offset(Winner_ind as isize))
            .RandState[last_smple_idx as usize];
        k = 0 as libc::c_int;
        while k < nStatesDelayedDecision {
            if (*psDelDec.offset(k as isize)).RandState[last_smple_idx as usize]
                != Winner_rand_state
            {
                (*psSampleState
                    .as_mut_ptr()
                    .offset(k as isize))[0 as libc::c_int as usize]
                    .RD_Q10 = (*psSampleState
                    .as_mut_ptr()
                    .offset(k as isize))[0 as libc::c_int as usize]
                    .RD_Q10 + (0x7fffffff as libc::c_int >> 4 as libc::c_int);
                (*psSampleState
                    .as_mut_ptr()
                    .offset(k as isize))[1 as libc::c_int as usize]
                    .RD_Q10 = (*psSampleState
                    .as_mut_ptr()
                    .offset(k as isize))[1 as libc::c_int as usize]
                    .RD_Q10 + (0x7fffffff as libc::c_int >> 4 as libc::c_int);
            }
            k += 1;
        }
        RDmax_Q10 = (*psSampleState
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            .RD_Q10;
        RDmin_Q10 = (*psSampleState
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            .RD_Q10;
        RDmax_ind = 0 as libc::c_int;
        RDmin_ind = 0 as libc::c_int;
        k = 1 as libc::c_int;
        while k < nStatesDelayedDecision {
            if (*psSampleState
                .as_mut_ptr()
                .offset(k as isize))[0 as libc::c_int as usize]
                .RD_Q10 > RDmax_Q10
            {
                RDmax_Q10 = (*psSampleState
                    .as_mut_ptr()
                    .offset(k as isize))[0 as libc::c_int as usize]
                    .RD_Q10;
                RDmax_ind = k;
            }
            if (*psSampleState
                .as_mut_ptr()
                .offset(k as isize))[1 as libc::c_int as usize]
                .RD_Q10 < RDmin_Q10
            {
                RDmin_Q10 = (*psSampleState
                    .as_mut_ptr()
                    .offset(k as isize))[1 as libc::c_int as usize]
                    .RD_Q10;
                RDmin_ind = k;
            }
            k += 1;
        }
        if RDmin_Q10 < RDmax_Q10 {
            memcpy(
                (&mut *psDelDec.offset(RDmax_ind as isize) as *mut NSQ_del_dec_struct
                    as *mut opus_int32)
                    .offset(i as isize) as *mut libc::c_void,
                (&mut *psDelDec.offset(RDmin_ind as isize) as *mut NSQ_del_dec_struct
                    as *mut opus_int32)
                    .offset(i as isize) as *const libc::c_void,
                (::core::mem::size_of::<NSQ_del_dec_struct>() as libc::c_ulong)
                    .wrapping_sub(
                        (i as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<opus_int32>() as libc::c_ulong,
                            ),
                    ),
            );
            memcpy(
                &mut *(*psSampleState.as_mut_ptr().offset(RDmax_ind as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut NSQ_sample_struct
                    as *mut libc::c_void,
                &mut *(*psSampleState.as_mut_ptr().offset(RDmin_ind as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut NSQ_sample_struct
                    as *const libc::c_void,
                ::core::mem::size_of::<NSQ_sample_struct>() as libc::c_ulong,
            );
        }
        psDD = &mut *psDelDec.offset(Winner_ind as isize) as *mut NSQ_del_dec_struct;
        if subfr > 0 as libc::c_int || i >= decisionDelay {
            *pulses
                .offset(
                    (i - decisionDelay) as isize,
                ) = (if 10 as libc::c_int == 1 as libc::c_int {
                ((*psDD).Q_Q10[last_smple_idx as usize] >> 1 as libc::c_int)
                    + ((*psDD).Q_Q10[last_smple_idx as usize] & 1 as libc::c_int)
            } else {
                ((*psDD).Q_Q10[last_smple_idx as usize]
                    >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as opus_int8;
            *xq
                .offset(
                    (i - decisionDelay) as isize,
                ) = (if (if 8 as libc::c_int == 1 as libc::c_int {
                (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as libc::c_long
                    >> 16 as libc::c_int) as opus_int32 >> 1 as libc::c_int)
                    + (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                        * *delayedGain_Q10.offset(last_smple_idx as isize)
                            as libc::c_long >> 16 as libc::c_int) as opus_int32
                        & 1 as libc::c_int)
            } else {
                (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as libc::c_long
                    >> 16 as libc::c_int) as opus_int32
                    >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > silk_int16_MAX
            {
                silk_int16_MAX
            } else if (if 8 as libc::c_int == 1 as libc::c_int {
                (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as libc::c_long
                    >> 16 as libc::c_int) as opus_int32 >> 1 as libc::c_int)
                    + (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                        * *delayedGain_Q10.offset(last_smple_idx as isize)
                            as libc::c_long >> 16 as libc::c_int) as opus_int32
                        & 1 as libc::c_int)
            } else {
                (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as libc::c_long
                    >> 16 as libc::c_int) as opus_int32
                    >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < silk_int16_MIN
            {
                silk_int16_MIN
            } else if 8 as libc::c_int == 1 as libc::c_int {
                (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as libc::c_long
                    >> 16 as libc::c_int) as opus_int32 >> 1 as libc::c_int)
                    + (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                        * *delayedGain_Q10.offset(last_smple_idx as isize)
                            as libc::c_long >> 16 as libc::c_int) as opus_int32
                        & 1 as libc::c_int)
            } else {
                (((*psDD).Xq_Q14[last_smple_idx as usize] as opus_int64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as libc::c_long
                    >> 16 as libc::c_int) as opus_int32
                    >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as opus_int16;
            (*NSQ)
                .sLTP_shp_Q14[((*NSQ).sLTP_shp_buf_idx - decisionDelay)
                as usize] = (*psDD).Shape_Q14[last_smple_idx as usize];
            *sLTP_Q15
                .offset(
                    ((*NSQ).sLTP_buf_idx - decisionDelay) as isize,
                ) = (*psDD).Pred_Q15[last_smple_idx as usize];
        }
        (*NSQ).sLTP_shp_buf_idx += 1;
        (*NSQ).sLTP_buf_idx += 1;
        k = 0 as libc::c_int;
        while k < nStatesDelayedDecision {
            psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
            psSS = &mut *(*psSampleState.as_mut_ptr().offset(k as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut NSQ_sample_struct;
            (*psDD).LF_AR_Q14 = (*psSS).LF_AR_Q14;
            (*psDD).Diff_Q14 = (*psSS).Diff_Q14;
            (*psDD).sLPC_Q14[(NSQ_LPC_BUF_LENGTH + i) as usize] = (*psSS).xq_Q14;
            (*psDD).Xq_Q14[*smpl_buf_idx as usize] = (*psSS).xq_Q14;
            (*psDD).Q_Q10[*smpl_buf_idx as usize] = (*psSS).Q_Q10;
            (*psDD)
                .Pred_Q15[*smpl_buf_idx
                as usize] = (((*psSS).LPC_exc_Q14 as opus_uint32) << 1 as libc::c_int)
                as opus_int32;
            (*psDD).Shape_Q14[*smpl_buf_idx as usize] = (*psSS).sLTP_shp_Q14;
            (*psDD)
                .Seed = ((*psDD).Seed as opus_uint32)
                .wrapping_add(
                    (if 10 as libc::c_int == 1 as libc::c_int {
                        ((*psSS).Q_Q10 >> 1 as libc::c_int)
                            + ((*psSS).Q_Q10 & 1 as libc::c_int)
                    } else {
                        ((*psSS).Q_Q10 >> 10 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int >> 1 as libc::c_int
                    }) as opus_uint32,
                ) as opus_int32;
            (*psDD).RandState[*smpl_buf_idx as usize] = (*psDD).Seed;
            (*psDD).RD_Q10 = (*psSS).RD_Q10;
            k += 1;
        }
        *delayedGain_Q10.offset(*smpl_buf_idx as isize) = Gain_Q10;
        i += 1;
    }
    k = 0 as libc::c_int;
    while k < nStatesDelayedDecision {
        psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
        memcpy(
            ((*psDD).sLPC_Q14).as_mut_ptr() as *mut libc::c_void,
            &mut *((*psDD).sLPC_Q14).as_mut_ptr().offset(length as isize)
                as *mut opus_int32 as *const libc::c_void,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_int32>() as libc::c_ulong),
        );
        k += 1;
    }
}
#[inline]
#[c2rust::src_loc = "651:1"]
unsafe extern "C" fn silk_nsq_del_dec_scale_states(
    psEncC: *const silk_encoder_state,
    mut NSQ: *mut silk_nsq_state,
    psDelDec: *mut NSQ_del_dec_struct,
    x16: *const opus_int16,
    x_sc_Q10: *mut opus_int32,
    sLTP: *const opus_int16,
    sLTP_Q15: *mut opus_int32,
    subfr: libc::c_int,
    nStatesDelayedDecision: libc::c_int,
    LTP_scale_Q14: libc::c_int,
    Gains_Q16: *const opus_int32,
    pitchL: *const libc::c_int,
    signal_type: libc::c_int,
    decisionDelay: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut gain_adj_Q16: opus_int32 = 0;
    let mut inv_gain_Q31: opus_int32 = 0;
    let mut inv_gain_Q26: opus_int32 = 0;
    let mut psDD: *mut NSQ_del_dec_struct = 0 as *mut NSQ_del_dec_struct;
    lag = *pitchL.offset(subfr as isize);
    inv_gain_Q31 = silk_INVERSE32_varQ(
        if *Gains_Q16.offset(subfr as isize) > 1 as libc::c_int {
            *Gains_Q16.offset(subfr as isize)
        } else {
            1 as libc::c_int
        },
        47 as libc::c_int,
    );
    inv_gain_Q26 = if 5 as libc::c_int == 1 as libc::c_int {
        (inv_gain_Q31 >> 1 as libc::c_int) + (inv_gain_Q31 & 1 as libc::c_int)
    } else {
        (inv_gain_Q31 >> 5 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
            >> 1 as libc::c_int
    };
    i = 0 as libc::c_int;
    while i < (*psEncC).subfr_length {
        *x_sc_Q10
            .offset(
                i as isize,
            ) = (*x16.offset(i as isize) as opus_int64 * inv_gain_Q26 as libc::c_long
            >> 16 as libc::c_int) as opus_int32;
        i += 1;
    }
    if (*NSQ).rewhite_flag != 0 {
        if subfr == 0 as libc::c_int {
            inv_gain_Q31 = (((inv_gain_Q31 as libc::c_long
                * LTP_scale_Q14 as opus_int16 as opus_int64 >> 16 as libc::c_int)
                as opus_int32 as opus_uint32) << 2 as libc::c_int) as opus_int32;
        }
        i = (*NSQ).sLTP_buf_idx - lag - LTP_ORDER / 2 as libc::c_int;
        while i < (*NSQ).sLTP_buf_idx {
            *sLTP_Q15
                .offset(
                    i as isize,
                ) = (inv_gain_Q31 as libc::c_long
                * *sLTP.offset(i as isize) as opus_int64 >> 16 as libc::c_int)
                as opus_int32;
            i += 1;
        }
    }
    if *Gains_Q16.offset(subfr as isize) != (*NSQ).prev_gain_Q16 {
        gain_adj_Q16 = silk_DIV32_varQ(
            (*NSQ).prev_gain_Q16,
            *Gains_Q16.offset(subfr as isize),
            16 as libc::c_int,
        );
        i = (*NSQ).sLTP_shp_buf_idx - (*psEncC).ltp_mem_length;
        while i < (*NSQ).sLTP_shp_buf_idx {
            (*NSQ)
                .sLTP_shp_Q14[i
                as usize] = (gain_adj_Q16 as opus_int64
                * (*NSQ).sLTP_shp_Q14[i as usize] as libc::c_long >> 16 as libc::c_int)
                as opus_int32;
            i += 1;
        }
        if signal_type == TYPE_VOICED && (*NSQ).rewhite_flag == 0 as libc::c_int {
            i = (*NSQ).sLTP_buf_idx - lag - LTP_ORDER / 2 as libc::c_int;
            while i < (*NSQ).sLTP_buf_idx - decisionDelay {
                *sLTP_Q15
                    .offset(
                        i as isize,
                    ) = (gain_adj_Q16 as opus_int64
                    * *sLTP_Q15.offset(i as isize) as libc::c_long >> 16 as libc::c_int)
                    as opus_int32;
                i += 1;
            }
        }
        k = 0 as libc::c_int;
        while k < nStatesDelayedDecision {
            psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
            (*psDD)
                .LF_AR_Q14 = (gain_adj_Q16 as opus_int64
                * (*psDD).LF_AR_Q14 as libc::c_long >> 16 as libc::c_int) as opus_int32;
            (*psDD)
                .Diff_Q14 = (gain_adj_Q16 as opus_int64
                * (*psDD).Diff_Q14 as libc::c_long >> 16 as libc::c_int) as opus_int32;
            i = 0 as libc::c_int;
            while i < NSQ_LPC_BUF_LENGTH {
                (*psDD)
                    .sLPC_Q14[i
                    as usize] = (gain_adj_Q16 as opus_int64
                    * (*psDD).sLPC_Q14[i as usize] as libc::c_long >> 16 as libc::c_int)
                    as opus_int32;
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < MAX_SHAPE_LPC_ORDER {
                (*psDD)
                    .sAR2_Q14[i
                    as usize] = (gain_adj_Q16 as opus_int64
                    * (*psDD).sAR2_Q14[i as usize] as libc::c_long >> 16 as libc::c_int)
                    as opus_int32;
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < DECISION_DELAY {
                (*psDD)
                    .Pred_Q15[i
                    as usize] = (gain_adj_Q16 as opus_int64
                    * (*psDD).Pred_Q15[i as usize] as libc::c_long >> 16 as libc::c_int)
                    as opus_int32;
                (*psDD)
                    .Shape_Q14[i
                    as usize] = (gain_adj_Q16 as opus_int64
                    * (*psDD).Shape_Q14[i as usize] as libc::c_long >> 16 as libc::c_int)
                    as opus_int32;
                i += 1;
            }
            k += 1;
        }
        (*NSQ).prev_gain_Q16 = *Gains_Q16.offset(subfr as isize);
    }
}
