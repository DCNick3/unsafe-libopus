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
    use super::types_h::{__int16_t, __int32_t, __int64_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
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
#[c2rust::header_src = "/usr/include/string.h:32"]
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
    }
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "155:9"]
    pub const MAX_SHAPE_LPC_ORDER: libc::c_int = 24 as libc::c_int;
    #[c2rust::src_loc = "157:9"]
    pub const HARM_SHAPE_FIR_TAPS: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "146:9"]
    pub const LTP_ORDER: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "180:10"]
    pub const NSQ_LPC_BUF_LENGTH: libc::c_int = MAX_LPC_ORDER;
    #[c2rust::src_loc = "142:9"]
    pub const MAX_LPC_ORDER: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "101:26"]
        pub static silk_Quantization_Offsets_Q10: [[i16; 2]; 2];
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
    pub const EC_CLZ0: libc::c_int =
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/macros.h:32"]
pub mod macros_h {
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn silk_CLZ32(in32: i32) -> i32 {
        return if in32 != 0 {
            32 as libc::c_int - (EC_CLZ0 - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }
    use super::ecintrin_h::EC_CLZ0;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    extern "C" {
        #[c2rust::src_loc = "123:1"]
        pub fn silk_LPC_analysis_filter(
            out: *mut i16,
            in_0: *const i16,
            B: *const i16,
            len: i32,
            d: i32,
            arch: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/Inlines.h:32"]
pub mod Inlines_h {
    #[inline]
    #[c2rust::src_loc = "97:1"]
    pub unsafe extern "C" fn silk_DIV32_varQ(a32: i32, b32: i32, Qres: libc::c_int) -> i32 {
        let mut a_headrm: libc::c_int = 0;
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: i32 = 0;
        let mut a32_nrm: i32 = 0;
        let mut b32_nrm: i32 = 0;
        let mut result: i32 = 0;
        a_headrm = silk_CLZ32(if a32 > 0 as libc::c_int { a32 } else { -a32 }) - 1 as libc::c_int;
        a32_nrm = ((a32 as u32) << a_headrm) as i32;
        b_headrm = silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 }) - 1 as libc::c_int;
        b32_nrm = ((b32 as u32) << b_headrm) as i32;
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int) / (b32_nrm >> 16 as libc::c_int);
        result = (a32_nrm as libc::c_long * b32_inv as i16 as i64 >> 16 as libc::c_int) as i32;
        a32_nrm = (a32_nrm as u32).wrapping_sub(
            (((b32_nrm as i64 * result as libc::c_long >> 32 as libc::c_int) as i32 as u32)
                << 3 as libc::c_int) as i32 as u32,
        ) as i32;
        result = (result as libc::c_long
            + (a32_nrm as libc::c_long * b32_inv as i16 as i64 >> 16 as libc::c_int))
            as i32;
        lshift = 29 as libc::c_int + a_headrm - b_headrm - Qres;
        if lshift < 0 as libc::c_int {
            return (((if 0x80000000 as libc::c_uint as i32 >> -lshift
                > 0x7fffffff as libc::c_int >> -lshift
            {
                if result > 0x80000000 as libc::c_uint as i32 >> -lshift {
                    0x80000000 as libc::c_uint as i32 >> -lshift
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
                    if result < 0x80000000 as libc::c_uint as i32 >> -lshift {
                        0x80000000 as libc::c_uint as i32 >> -lshift
                    } else {
                        result
                    }
                }
            }) as u32)
                << -lshift) as i32;
        } else if lshift < 32 as libc::c_int {
            return result >> lshift;
        } else {
            return 0 as libc::c_int;
        };
    }
    #[inline]
    #[c2rust::src_loc = "143:1"]
    pub unsafe extern "C" fn silk_INVERSE32_varQ(b32: i32, Qres: libc::c_int) -> i32 {
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: i32 = 0;
        let mut b32_nrm: i32 = 0;
        let mut err_Q32: i32 = 0;
        let mut result: i32 = 0;
        b_headrm = silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 }) - 1 as libc::c_int;
        b32_nrm = ((b32 as u32) << b_headrm) as i32;
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int) / (b32_nrm >> 16 as libc::c_int);
        result = ((b32_inv as u32) << 16 as libc::c_int) as i32;
        err_Q32 = (((((1 as libc::c_int) << 29 as libc::c_int)
            - (b32_nrm as libc::c_long * b32_inv as i16 as i64 >> 16 as libc::c_int) as i32)
            as u32)
            << 3 as libc::c_int) as i32;
        result = (result as libc::c_long
            + (err_Q32 as i64 * b32_inv as libc::c_long >> 16 as libc::c_int))
            as i32;
        lshift = 61 as libc::c_int - b_headrm - Qres;
        if lshift <= 0 as libc::c_int {
            return (((if 0x80000000 as libc::c_uint as i32 >> -lshift
                > 0x7fffffff as libc::c_int >> -lshift
            {
                if result > 0x80000000 as libc::c_uint as i32 >> -lshift {
                    0x80000000 as libc::c_uint as i32 >> -lshift
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
                    if result < 0x80000000 as libc::c_uint as i32 >> -lshift {
                        0x80000000 as libc::c_uint as i32 >> -lshift
                    } else {
                        result
                    }
                }
            }) as u32)
                << -lshift) as i32;
        } else if lshift < 32 as libc::c_int {
            return result >> lshift;
        } else {
            return 0 as libc::c_int;
        };
    }
    use super::macros_h::silk_CLZ32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/NSQ.h:34"]
pub mod NSQ_h {
    #[inline]
    #[c2rust::src_loc = "35:1"]
    pub unsafe extern "C" fn silk_noise_shape_quantizer_short_prediction_c(
        buf32: *const i32,
        coef16: *const i16,
        order: libc::c_int,
    ) -> i32 {
        let mut out: i32 = 0;
        out = order >> 1 as libc::c_int;
        out = (out as libc::c_long
            + (*buf32.offset(0 as libc::c_int as isize) as libc::c_long
                * *coef16.offset(0 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(1 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(1 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(2 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(2 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(3 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(3 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(4 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(4 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(5 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(5 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(6 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(6 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(7 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(7 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(8 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(8 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(9 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(9 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        if order == 16 as libc::c_int {
            out = (out as libc::c_long
                + (*buf32.offset(-(10 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(10 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            out = (out as libc::c_long
                + (*buf32.offset(-(11 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(11 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            out = (out as libc::c_long
                + (*buf32.offset(-(12 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(12 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            out = (out as libc::c_long
                + (*buf32.offset(-(13 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(13 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            out = (out as libc::c_long
                + (*buf32.offset(-(14 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(14 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            out = (out as libc::c_long
                + (*buf32.offset(-(15 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(15 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
        }
        return out;
    }
    #[inline]
    #[c2rust::src_loc = "67:1"]
    pub unsafe extern "C" fn silk_NSQ_noise_shape_feedback_loop_c(
        data0: *const i32,
        data1: *mut i32,
        coef: *const i16,
        order: libc::c_int,
    ) -> i32 {
        let mut out: i32 = 0;
        let mut tmp1: i32 = 0;
        let mut tmp2: i32 = 0;
        let mut j: libc::c_int = 0;
        tmp2 = *data0.offset(0 as libc::c_int as isize);
        tmp1 = *data1.offset(0 as libc::c_int as isize);
        *data1.offset(0 as libc::c_int as isize) = tmp2;
        out = order >> 1 as libc::c_int;
        out = (out as libc::c_long
            + (tmp2 as libc::c_long * *coef.offset(0 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        j = 2 as libc::c_int;
        while j < order {
            tmp2 = *data1.offset((j - 1 as libc::c_int) as isize);
            *data1.offset((j - 1 as libc::c_int) as isize) = tmp1;
            out = (out as libc::c_long
                + (tmp1 as libc::c_long * *coef.offset((j - 1 as libc::c_int) as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            tmp1 = *data1.offset((j + 0 as libc::c_int) as isize);
            *data1.offset((j + 0 as libc::c_int) as isize) = tmp2;
            out = (out as libc::c_long
                + (tmp2 as libc::c_long * *coef.offset(j as isize) as i64 >> 16 as libc::c_int))
                as i32;
            j += 2 as libc::c_int;
        }
        *data1.offset((order - 1 as libc::c_int) as isize) = tmp1;
        out = (out as libc::c_long
            + (tmp1 as libc::c_long * *coef.offset((order - 1 as libc::c_int) as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = ((out as u32) << 1 as libc::c_int) as i32;
        return out;
    }
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
use self::arch_h::celt_fatal;
pub use self::define_h::{
    HARM_SHAPE_FIR_TAPS, LTP_ORDER, MAX_LPC_ORDER, MAX_SHAPE_LPC_ORDER, NSQ_LPC_BUF_LENGTH,
    TYPE_VOICED,
};
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
pub use self::macros_h::silk_CLZ32;
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::string_h::{memcpy, memmove};
pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    SideInfoIndices,
};
use self::tables_h::silk_Quantization_Offsets_Q10;
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __int8_t, __uint32_t, __uint8_t};
pub use self::Inlines_h::{silk_DIV32_varQ, silk_INVERSE32_varQ};
pub use self::NSQ_h::{
    silk_NSQ_noise_shape_feedback_loop_c, silk_noise_shape_quantizer_short_prediction_c,
};
use self::SigProc_FIX_h::silk_LPC_analysis_filter;
#[no_mangle]
#[c2rust::src_loc = "76:1"]
pub unsafe extern "C" fn silk_NSQ_c(
    psEncC: *const silk_encoder_state,
    mut NSQ: *mut silk_nsq_state,
    psIndices: *mut SideInfoIndices,
    mut x16: *const i16,
    mut pulses: *mut i8,
    PredCoef_Q12: *const i16,
    LTPCoef_Q14: *const i16,
    AR_Q13: *const i16,
    HarmShapeGain_Q14: *const libc::c_int,
    Tilt_Q14: *const libc::c_int,
    LF_shp_Q14: *const i32,
    Gains_Q16: *const i32,
    pitchL: *const libc::c_int,
    Lambda_Q10: libc::c_int,
    LTP_scale_Q14: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut start_idx: libc::c_int = 0;
    let mut LSF_interpolation_flag: libc::c_int = 0;
    let mut A_Q12: *const i16 = 0 as *const i16;
    let mut B_Q14: *const i16 = 0 as *const i16;
    let mut AR_shp_Q13: *const i16 = 0 as *const i16;
    let mut pxq: *mut i16 = 0 as *mut i16;
    let mut HarmShapeFIRPacked_Q14: i32 = 0;
    let mut offset_Q10: libc::c_int = 0;
    (*NSQ).rand_seed = (*psIndices).Seed as i32;
    lag = (*NSQ).lagPrev;
    offset_Q10 = silk_Quantization_Offsets_Q10
        [((*psIndices).signalType as libc::c_int >> 1 as libc::c_int) as usize]
        [(*psIndices).quantOffsetType as usize] as libc::c_int;
    if (*psIndices).NLSFInterpCoef_Q2 as libc::c_int == 4 as libc::c_int {
        LSF_interpolation_flag = 0 as libc::c_int;
    } else {
        LSF_interpolation_flag = 1 as libc::c_int;
    }
    let vla = ((*psEncC).ltp_mem_length + (*psEncC).frame_length) as usize;
    let mut sLTP_Q15: Vec<i32> = ::std::vec::from_elem(0, vla);
    let vla_0 = ((*psEncC).ltp_mem_length + (*psEncC).frame_length) as usize;
    let mut sLTP: Vec<i16> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = (*psEncC).subfr_length as usize;
    let mut x_sc_Q10: Vec<i32> = ::std::vec::from_elem(0, vla_1);
    (*NSQ).sLTP_shp_buf_idx = (*psEncC).ltp_mem_length;
    (*NSQ).sLTP_buf_idx = (*psEncC).ltp_mem_length;
    pxq = &mut *((*NSQ).xq)
        .as_mut_ptr()
        .offset((*psEncC).ltp_mem_length as isize) as *mut i16;
    k = 0 as libc::c_int;
    while k < (*psEncC).nb_subfr {
        A_Q12 = &*PredCoef_Q12.offset(
            ((k >> 1 as libc::c_int | 1 as libc::c_int - LSF_interpolation_flag) * MAX_LPC_ORDER)
                as isize,
        ) as *const i16;
        B_Q14 = &*LTPCoef_Q14.offset((k * LTP_ORDER) as isize) as *const i16;
        AR_shp_Q13 = &*AR_Q13.offset((k * MAX_SHAPE_LPC_ORDER) as isize) as *const i16;
        HarmShapeFIRPacked_Q14 = *HarmShapeGain_Q14.offset(k as isize) >> 2 as libc::c_int;
        HarmShapeFIRPacked_Q14 |= (((*HarmShapeGain_Q14.offset(k as isize) >> 1 as libc::c_int)
            as u32)
            << 16 as libc::c_int) as i32;
        (*NSQ).rewhite_flag = 0 as libc::c_int;
        if (*psIndices).signalType as libc::c_int == TYPE_VOICED {
            lag = *pitchL.offset(k as isize);
            if k & 3 as libc::c_int - ((LSF_interpolation_flag as u32) << 1 as libc::c_int) as i32
                == 0 as libc::c_int
            {
                start_idx = (*psEncC).ltp_mem_length
                    - lag
                    - (*psEncC).predictLPCOrder
                    - LTP_ORDER / 2 as libc::c_int;
                if !(start_idx > 0 as libc::c_int) {
                    celt_fatal(
                        b"assertion failed: start_idx > 0\0" as *const u8 as *const libc::c_char,
                        b"silk/NSQ.c\0" as *const u8 as *const libc::c_char,
                        146 as libc::c_int,
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
                (*NSQ).rewhite_flag = 1 as libc::c_int;
                (*NSQ).sLTP_buf_idx = (*psEncC).ltp_mem_length;
            }
        }
        silk_nsq_scale_states(
            psEncC,
            NSQ,
            x16,
            x_sc_Q10.as_mut_ptr(),
            sLTP.as_mut_ptr() as *const i16,
            sLTP_Q15.as_mut_ptr(),
            k,
            LTP_scale_Q14,
            Gains_Q16,
            pitchL,
            (*psIndices).signalType as libc::c_int,
        );
        silk_noise_shape_quantizer(
            NSQ,
            (*psIndices).signalType as libc::c_int,
            x_sc_Q10.as_mut_ptr() as *const i32,
            pulses,
            pxq,
            sLTP_Q15.as_mut_ptr(),
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
            (*psEncC).shapingLPCOrder,
            (*psEncC).predictLPCOrder,
            (*psEncC).arch,
        );
        x16 = x16.offset((*psEncC).subfr_length as isize);
        pulses = pulses.offset((*psEncC).subfr_length as isize);
        pxq = pxq.offset((*psEncC).subfr_length as isize);
        k += 1;
    }
    (*NSQ).lagPrev = *pitchL.offset(((*psEncC).nb_subfr - 1 as libc::c_int) as isize);
    memmove(
        ((*NSQ).xq).as_mut_ptr() as *mut libc::c_void,
        &mut *((*NSQ).xq)
            .as_mut_ptr()
            .offset((*psEncC).frame_length as isize) as *mut i16 as *const libc::c_void,
        ((*psEncC).ltp_mem_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    memmove(
        ((*NSQ).sLTP_shp_Q14).as_mut_ptr() as *mut libc::c_void,
        &mut *((*NSQ).sLTP_shp_Q14)
            .as_mut_ptr()
            .offset((*psEncC).frame_length as isize) as *mut i32 as *const libc::c_void,
        ((*psEncC).ltp_mem_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
}
#[inline]
#[c2rust::src_loc = "181:1"]
unsafe extern "C" fn silk_noise_shape_quantizer(
    mut NSQ: *mut silk_nsq_state,
    signalType: libc::c_int,
    x_sc_Q10: *const i32,
    pulses: *mut i8,
    xq: *mut i16,
    sLTP_Q15: *mut i32,
    a_Q12: *const i16,
    b_Q14: *const i16,
    AR_shp_Q13: *const i16,
    lag: libc::c_int,
    HarmShapeFIRPacked_Q14: i32,
    Tilt_Q14: libc::c_int,
    LF_shp_Q14: i32,
    Gain_Q16: i32,
    Lambda_Q10: libc::c_int,
    offset_Q10: libc::c_int,
    length: libc::c_int,
    shapingLPCOrder: libc::c_int,
    predictLPCOrder: libc::c_int,
    _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut LTP_pred_Q13: i32 = 0;
    let mut LPC_pred_Q10: i32 = 0;
    let mut n_AR_Q12: i32 = 0;
    let mut n_LTP_Q13: i32 = 0;
    let mut n_LF_Q12: i32 = 0;
    let mut r_Q10: i32 = 0;
    let mut rr_Q10: i32 = 0;
    let mut q1_Q0: i32 = 0;
    let mut q1_Q10: i32 = 0;
    let mut q2_Q10: i32 = 0;
    let mut rd1_Q20: i32 = 0;
    let mut rd2_Q20: i32 = 0;
    let mut exc_Q14: i32 = 0;
    let mut LPC_exc_Q14: i32 = 0;
    let mut xq_Q14: i32 = 0;
    let mut Gain_Q10: i32 = 0;
    let mut tmp1: i32 = 0;
    let mut tmp2: i32 = 0;
    let mut sLF_AR_shp_Q14: i32 = 0;
    let mut psLPC_Q14: *mut i32 = 0 as *mut i32;
    let mut shp_lag_ptr: *mut i32 = 0 as *mut i32;
    let mut pred_lag_ptr: *mut i32 = 0 as *mut i32;
    shp_lag_ptr = &mut *((*NSQ).sLTP_shp_Q14)
        .as_mut_ptr()
        .offset(((*NSQ).sLTP_shp_buf_idx - lag + HARM_SHAPE_FIR_TAPS / 2 as libc::c_int) as isize)
        as *mut i32;
    pred_lag_ptr = &mut *sLTP_Q15
        .offset(((*NSQ).sLTP_buf_idx - lag + LTP_ORDER / 2 as libc::c_int) as isize)
        as *mut i32;
    Gain_Q10 = Gain_Q16 >> 6 as libc::c_int;
    psLPC_Q14 = &mut *((*NSQ).sLPC_Q14)
        .as_mut_ptr()
        .offset((NSQ_LPC_BUF_LENGTH - 1 as libc::c_int) as isize) as *mut i32;
    i = 0 as libc::c_int;
    while i < length {
        (*NSQ).rand_seed = (907633515 as libc::c_int as u32)
            .wrapping_add(((*NSQ).rand_seed as u32).wrapping_mul(196314165 as libc::c_int as u32))
            as i32;
        LPC_pred_Q10 =
            silk_noise_shape_quantizer_short_prediction_c(psLPC_Q14, a_Q12, predictLPCOrder);
        if signalType == TYPE_VOICED {
            LTP_pred_Q13 = 2 as libc::c_int;
            LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                + (*pred_lag_ptr.offset(0 as libc::c_int as isize) as libc::c_long
                    * *b_Q14.offset(0 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                + (*pred_lag_ptr.offset(-(1 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(1 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                + (*pred_lag_ptr.offset(-(2 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(2 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                + (*pred_lag_ptr.offset(-(3 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(3 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                + (*pred_lag_ptr.offset(-(4 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(4 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            pred_lag_ptr = pred_lag_ptr.offset(1);
        } else {
            LTP_pred_Q13 = 0 as libc::c_int;
        }
        if !(shapingLPCOrder & 1 as libc::c_int == 0 as libc::c_int) {
            celt_fatal(
                b"assertion failed: ( shapingLPCOrder & 1 ) == 0\0" as *const u8
                    as *const libc::c_char,
                b"silk/NSQ.c\0" as *const u8 as *const libc::c_char,
                250 as libc::c_int,
            );
        }
        n_AR_Q12 = silk_NSQ_noise_shape_feedback_loop_c(
            &mut (*NSQ).sDiff_shp_Q14,
            ((*NSQ).sAR2_Q14).as_mut_ptr(),
            AR_shp_Q13,
            shapingLPCOrder,
        );
        n_AR_Q12 = (n_AR_Q12 as libc::c_long
            + ((*NSQ).sLF_AR_shp_Q14 as libc::c_long * Tilt_Q14 as i16 as i64 >> 16 as libc::c_int))
            as i32;
        n_LF_Q12 = ((*NSQ).sLTP_shp_Q14[((*NSQ).sLTP_shp_buf_idx - 1 as libc::c_int) as usize]
            as libc::c_long
            * LF_shp_Q14 as i16 as i64
            >> 16 as libc::c_int) as i32;
        n_LF_Q12 = (n_LF_Q12 as libc::c_long
            + ((*NSQ).sLF_AR_shp_Q14 as libc::c_long * (LF_shp_Q14 as i64 >> 16 as libc::c_int)
                >> 16 as libc::c_int)) as i32;
        if !(lag > 0 as libc::c_int || signalType != 2 as libc::c_int) {
            celt_fatal(
                b"assertion failed: lag > 0 || signalType != TYPE_VOICED\0" as *const u8
                    as *const libc::c_char,
                b"silk/NSQ.c\0" as *const u8 as *const libc::c_char,
                258 as libc::c_int,
            );
        }
        tmp1 = ((LPC_pred_Q10 as u32) << 2 as libc::c_int) as i32 - n_AR_Q12;
        tmp1 = tmp1 - n_LF_Q12;
        if lag > 0 as libc::c_int {
            n_LTP_Q13 = ((*shp_lag_ptr.offset(0 as libc::c_int as isize)
                + *shp_lag_ptr.offset(-(2 as libc::c_int) as isize))
                as libc::c_long
                * HarmShapeFIRPacked_Q14 as i16 as i64
                >> 16 as libc::c_int) as i32;
            n_LTP_Q13 = (n_LTP_Q13 as libc::c_long
                + (*shp_lag_ptr.offset(-(1 as libc::c_int) as isize) as libc::c_long
                    * (HarmShapeFIRPacked_Q14 as i64 >> 16 as libc::c_int)
                    >> 16 as libc::c_int)) as i32;
            n_LTP_Q13 = ((n_LTP_Q13 as u32) << 1 as libc::c_int) as i32;
            shp_lag_ptr = shp_lag_ptr.offset(1);
            tmp2 = LTP_pred_Q13 - n_LTP_Q13;
            tmp1 = tmp2 + ((tmp1 as u32) << 1 as libc::c_int) as i32;
            tmp1 = if 3 as libc::c_int == 1 as libc::c_int {
                (tmp1 >> 1 as libc::c_int) + (tmp1 & 1 as libc::c_int)
            } else {
                (tmp1 >> 3 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
            };
        } else {
            tmp1 = if 2 as libc::c_int == 1 as libc::c_int {
                (tmp1 >> 1 as libc::c_int) + (tmp1 & 1 as libc::c_int)
            } else {
                (tmp1 >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
            };
        }
        r_Q10 = *x_sc_Q10.offset(i as isize) - tmp1;
        if (*NSQ).rand_seed < 0 as libc::c_int {
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
            let rdo_offset: libc::c_int = Lambda_Q10 / 2 as libc::c_int - 512 as libc::c_int;
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
            q1_Q10 = ((q1_Q0 as u32) << 10 as libc::c_int) as i32 - 80 as libc::c_int;
            q1_Q10 = q1_Q10 + offset_Q10;
            q2_Q10 = q1_Q10 + 1024 as libc::c_int;
            rd1_Q20 = q1_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
            rd2_Q20 = q2_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
        } else if q1_Q0 == 0 as libc::c_int {
            q1_Q10 = offset_Q10;
            q2_Q10 = q1_Q10 + (1024 as libc::c_int - 80 as libc::c_int);
            rd1_Q20 = q1_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
            rd2_Q20 = q2_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
        } else if q1_Q0 == -(1 as libc::c_int) {
            q2_Q10 = offset_Q10;
            q1_Q10 = q2_Q10 - (1024 as libc::c_int - 80 as libc::c_int);
            rd1_Q20 = -q1_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
            rd2_Q20 = q2_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
        } else {
            q1_Q10 = ((q1_Q0 as u32) << 10 as libc::c_int) as i32 + 80 as libc::c_int;
            q1_Q10 = q1_Q10 + offset_Q10;
            q2_Q10 = q1_Q10 + 1024 as libc::c_int;
            rd1_Q20 = -q1_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
            rd2_Q20 = -q2_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
        }
        rr_Q10 = r_Q10 - q1_Q10;
        rd1_Q20 = rd1_Q20 + rr_Q10 as i16 as i32 * rr_Q10 as i16 as i32;
        rr_Q10 = r_Q10 - q2_Q10;
        rd2_Q20 = rd2_Q20 + rr_Q10 as i16 as i32 * rr_Q10 as i16 as i32;
        if rd2_Q20 < rd1_Q20 {
            q1_Q10 = q2_Q10;
        }
        *pulses.offset(i as isize) = (if 10 as libc::c_int == 1 as libc::c_int {
            (q1_Q10 >> 1 as libc::c_int) + (q1_Q10 & 1 as libc::c_int)
        } else {
            (q1_Q10 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i8;
        exc_Q14 = ((q1_Q10 as u32) << 4 as libc::c_int) as i32;
        if (*NSQ).rand_seed < 0 as libc::c_int {
            exc_Q14 = -exc_Q14;
        }
        LPC_exc_Q14 = exc_Q14 + ((LTP_pred_Q13 as u32) << 1 as libc::c_int) as i32;
        xq_Q14 = LPC_exc_Q14 + ((LPC_pred_Q10 as u32) << 4 as libc::c_int) as i32;
        *xq.offset(i as isize) = (if (if 8 as libc::c_int == 1 as libc::c_int {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 8 as libc::c_int == 1 as libc::c_int {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as i16;
        psLPC_Q14 = psLPC_Q14.offset(1);
        *psLPC_Q14 = xq_Q14;
        (*NSQ).sDiff_shp_Q14 =
            xq_Q14 - ((*x_sc_Q10.offset(i as isize) as u32) << 4 as libc::c_int) as i32;
        sLF_AR_shp_Q14 = (*NSQ).sDiff_shp_Q14 - ((n_AR_Q12 as u32) << 2 as libc::c_int) as i32;
        (*NSQ).sLF_AR_shp_Q14 = sLF_AR_shp_Q14;
        (*NSQ).sLTP_shp_Q14[(*NSQ).sLTP_shp_buf_idx as usize] =
            sLF_AR_shp_Q14 - ((n_LF_Q12 as u32) << 2 as libc::c_int) as i32;
        *sLTP_Q15.offset((*NSQ).sLTP_buf_idx as isize) =
            ((LPC_exc_Q14 as u32) << 1 as libc::c_int) as i32;
        (*NSQ).sLTP_shp_buf_idx += 1;
        (*NSQ).sLTP_buf_idx += 1;
        (*NSQ).rand_seed =
            ((*NSQ).rand_seed as u32).wrapping_add(*pulses.offset(i as isize) as u32) as i32;
        i += 1;
    }
    memcpy(
        ((*NSQ).sLPC_Q14).as_mut_ptr() as *mut libc::c_void,
        &mut *((*NSQ).sLPC_Q14).as_mut_ptr().offset(length as isize) as *mut i32
            as *const libc::c_void,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
}
#[inline]
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn silk_nsq_scale_states(
    psEncC: *const silk_encoder_state,
    mut NSQ: *mut silk_nsq_state,
    x16: *const i16,
    x_sc_Q10: *mut i32,
    sLTP: *const i16,
    sLTP_Q15: *mut i32,
    subfr: libc::c_int,
    LTP_scale_Q14: libc::c_int,
    Gains_Q16: *const i32,
    pitchL: *const libc::c_int,
    signal_type: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut gain_adj_Q16: i32 = 0;
    let mut inv_gain_Q31: i32 = 0;
    let mut inv_gain_Q26: i32 = 0;
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
        (inv_gain_Q31 >> 5 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
    };
    i = 0 as libc::c_int;
    while i < (*psEncC).subfr_length {
        *x_sc_Q10.offset(i as isize) = (*x16.offset(i as isize) as i64
            * inv_gain_Q26 as libc::c_long
            >> 16 as libc::c_int) as i32;
        i += 1;
    }
    if (*NSQ).rewhite_flag != 0 {
        if subfr == 0 as libc::c_int {
            inv_gain_Q31 = (((inv_gain_Q31 as libc::c_long * LTP_scale_Q14 as i16 as i64
                >> 16 as libc::c_int) as i32 as u32)
                << 2 as libc::c_int) as i32;
        }
        i = (*NSQ).sLTP_buf_idx - lag - LTP_ORDER / 2 as libc::c_int;
        while i < (*NSQ).sLTP_buf_idx {
            *sLTP_Q15.offset(i as isize) = (inv_gain_Q31 as libc::c_long
                * *sLTP.offset(i as isize) as i64
                >> 16 as libc::c_int) as i32;
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
            (*NSQ).sLTP_shp_Q14[i as usize] = (gain_adj_Q16 as i64
                * (*NSQ).sLTP_shp_Q14[i as usize] as libc::c_long
                >> 16 as libc::c_int) as i32;
            i += 1;
        }
        if signal_type == TYPE_VOICED && (*NSQ).rewhite_flag == 0 as libc::c_int {
            i = (*NSQ).sLTP_buf_idx - lag - LTP_ORDER / 2 as libc::c_int;
            while i < (*NSQ).sLTP_buf_idx {
                *sLTP_Q15.offset(i as isize) = (gain_adj_Q16 as i64
                    * *sLTP_Q15.offset(i as isize) as libc::c_long
                    >> 16 as libc::c_int) as i32;
                i += 1;
            }
        }
        (*NSQ).sLF_AR_shp_Q14 = (gain_adj_Q16 as i64 * (*NSQ).sLF_AR_shp_Q14 as libc::c_long
            >> 16 as libc::c_int) as i32;
        (*NSQ).sDiff_shp_Q14 = (gain_adj_Q16 as i64 * (*NSQ).sDiff_shp_Q14 as libc::c_long
            >> 16 as libc::c_int) as i32;
        i = 0 as libc::c_int;
        while i < NSQ_LPC_BUF_LENGTH {
            (*NSQ).sLPC_Q14[i as usize] = (gain_adj_Q16 as i64
                * (*NSQ).sLPC_Q14[i as usize] as libc::c_long
                >> 16 as libc::c_int) as i32;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < MAX_SHAPE_LPC_ORDER {
            (*NSQ).sAR2_Q14[i as usize] = (gain_adj_Q16 as i64
                * (*NSQ).sAR2_Q14[i as usize] as libc::c_long
                >> 16 as libc::c_int) as i32;
            i += 1;
        }
        (*NSQ).prev_gain_Q16 = *Gains_Q16.offset(subfr as isize);
    }
}
