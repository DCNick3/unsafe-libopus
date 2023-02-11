use crate::externs::{free, malloc};
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

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[c2rust::src_loc = "57:9"]
    pub const CELT_SIG_SCALE: libc::c_float = 32768.0f32;
    #[c2rust::src_loc = "203:9"]
    pub const Q15ONE: libc::c_float = 1.0f32;
    #[c2rust::src_loc = "207:9"]
    pub const EPSILON: libc::c_float = 1e-15f32;
    #[c2rust::src_loc = "208:9"]
    pub const VERY_SMALL: libc::c_float = 1e-30f32;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/celt.h:33"]
pub mod celt_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:9"]
    pub struct SILKInfo {
        pub signalType: libc::c_int,
        pub offset: libc::c_int,
    }
    #[c2rust::src_loc = "125:9"]
    pub const OPUS_SET_LFE_REQUEST: libc::c_int = 10024;
    #[c2rust::src_loc = "128:9"]
    pub const OPUS_SET_ENERGY_MASK_REQUEST: libc::c_int = 10026;
    #[c2rust::src_loc = "110:9"]
    pub const CELT_GET_MODE_REQUEST: libc::c_int = 10015 as libc::c_int;
    #[c2rust::src_loc = "107:9"]
    pub const CELT_SET_END_BAND_REQUEST: libc::c_int = 10012 as libc::c_int;
    #[c2rust::src_loc = "99:9"]
    pub const CELT_SET_CHANNELS_REQUEST: libc::c_int = 10008 as libc::c_int;
    #[c2rust::src_loc = "122:9"]
    pub const CELT_SET_ANALYSIS_REQUEST: libc::c_int = 10022 as libc::c_int;
    #[c2rust::src_loc = "131:9"]
    pub const CELT_SET_SILK_INFO_REQUEST: libc::c_int = 10028 as libc::c_int;
    #[c2rust::src_loc = "104:9"]
    pub const CELT_SET_START_BAND_REQUEST: libc::c_int = 10010 as libc::c_int;
    #[c2rust::src_loc = "85:9"]
    pub const CELT_SET_PREDICTION_REQUEST: libc::c_int = 10002 as libc::c_int;
    #[c2rust::src_loc = "114:9"]
    pub const CELT_SET_SIGNALLING_REQUEST: libc::c_int = 10016 as libc::c_int;
    #[c2rust::src_loc = "155:9"]
    pub const celt_encoder_ctl: unsafe extern "C" fn(
        *mut OpusCustomEncoder,
        libc::c_int,
        ...
    ) -> libc::c_int = opus_custom_encoder_ctl;

    use super::arch_h::opus_val16;
    use crate::celt::celt_encoder::{opus_custom_encoder_ctl, OpusCustomEncoder};
    use crate::celt::entenc::ec_enc;

    extern "C" {
        #[c2rust::src_loc = "140:1"]
        pub fn celt_encoder_init(
            st: *mut OpusCustomEncoder,
            sampling_rate: i32,
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
        #[c2rust::src_loc = "136:1"]
        pub fn celt_encoder_get_size(channels: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/xmmintrin.h:38"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
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
        pub LastGainIndex: i8,
        pub HarmShapeGain_smth: libc::c_float,
        pub Tilt_smth: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:9"]
    pub struct silk_encoder {
        pub state_Fxx: [silk_encoder_state_FLP; 2],
        pub sStereo: stereo_enc_state,
        pub nBitsUsedLBRR: i32,
        pub nBitsExceeded: i32,
        pub nChannelsAPI: libc::c_int,
        pub nChannelsInternal: libc::c_int,
        pub nPrevChannelsInternal: libc::c_int,
        pub timeSinceSwitchAllowed_ms: libc::c_int,
        pub allowBandwidthSwitch: libc::c_int,
        pub prev_decode_only_middle: libc::c_int,
    }
    use crate::silk::structs::{silk_encoder_state, stereo_enc_state};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_defines.h:33"]
pub mod opus_defines_h {
    #[c2rust::src_loc = "130:9"]
    pub const OPUS_SET_APPLICATION_REQUEST: libc::c_int = 4000;
    #[c2rust::src_loc = "131:9"]
    pub const OPUS_GET_APPLICATION_REQUEST: libc::c_int = 4001;
    #[c2rust::src_loc = "133:9"]
    pub const OPUS_GET_BITRATE_REQUEST: libc::c_int = 4003;
    #[c2rust::src_loc = "150:9"]
    pub const OPUS_SET_FORCE_CHANNELS_REQUEST: libc::c_int = 4022;
    #[c2rust::src_loc = "151:9"]
    pub const OPUS_GET_FORCE_CHANNELS_REQUEST: libc::c_int = 4023;
    #[c2rust::src_loc = "134:9"]
    pub const OPUS_SET_MAX_BANDWIDTH_REQUEST: libc::c_int = 4004;
    #[c2rust::src_loc = "135:9"]
    pub const OPUS_GET_MAX_BANDWIDTH_REQUEST: libc::c_int = 4005;
    #[c2rust::src_loc = "138:9"]
    pub const OPUS_SET_BANDWIDTH_REQUEST: libc::c_int = 4008;
    #[c2rust::src_loc = "139:9"]
    pub const OPUS_GET_BANDWIDTH_REQUEST: libc::c_int = 4009;
    #[c2rust::src_loc = "146:9"]
    pub const OPUS_SET_DTX_REQUEST: libc::c_int = 4016;
    #[c2rust::src_loc = "147:9"]
    pub const OPUS_GET_DTX_REQUEST: libc::c_int = 4017;
    #[c2rust::src_loc = "141:9"]
    pub const OPUS_GET_COMPLEXITY_REQUEST: libc::c_int = 4011;
    #[c2rust::src_loc = "142:9"]
    pub const OPUS_SET_INBAND_FEC_REQUEST: libc::c_int = 4012;
    #[c2rust::src_loc = "143:9"]
    pub const OPUS_GET_INBAND_FEC_REQUEST: libc::c_int = 4013;
    #[c2rust::src_loc = "144:9"]
    pub const OPUS_SET_PACKET_LOSS_PERC_REQUEST: libc::c_int = 4014 as libc::c_int;
    #[c2rust::src_loc = "145:9"]
    pub const OPUS_GET_PACKET_LOSS_PERC_REQUEST: libc::c_int = 4015;
    #[c2rust::src_loc = "137:9"]
    pub const OPUS_GET_VBR_REQUEST: libc::c_int = 4007;
    #[c2rust::src_loc = "149:9"]
    pub const OPUS_GET_VBR_CONSTRAINT_REQUEST: libc::c_int = 4021;
    #[c2rust::src_loc = "152:9"]
    pub const OPUS_SET_SIGNAL_REQUEST: libc::c_int = 4024;
    #[c2rust::src_loc = "153:9"]
    pub const OPUS_GET_SIGNAL_REQUEST: libc::c_int = 4025;
    #[c2rust::src_loc = "154:9"]
    pub const OPUS_GET_LOOKAHEAD_REQUEST: libc::c_int = 4027;
    #[c2rust::src_loc = "156:9"]
    pub const OPUS_GET_SAMPLE_RATE_REQUEST: libc::c_int = 4029;
    #[c2rust::src_loc = "162:9"]
    pub const OPUS_GET_LSB_DEPTH_REQUEST: libc::c_int = 4037;
    #[c2rust::src_loc = "164:9"]
    pub const OPUS_SET_EXPERT_FRAME_DURATION_REQUEST: libc::c_int = 4040;
    #[c2rust::src_loc = "211:9"]
    pub const OPUS_FRAMESIZE_5_MS: libc::c_int = 5002 as libc::c_int;
    #[c2rust::src_loc = "212:9"]
    pub const OPUS_FRAMESIZE_10_MS: libc::c_int = 5003 as libc::c_int;
    #[c2rust::src_loc = "213:9"]
    pub const OPUS_FRAMESIZE_20_MS: libc::c_int = 5004 as libc::c_int;
    #[c2rust::src_loc = "215:9"]
    pub const OPUS_FRAMESIZE_60_MS: libc::c_int = 5006 as libc::c_int;
    #[c2rust::src_loc = "216:9"]
    pub const OPUS_FRAMESIZE_80_MS: libc::c_int = 5007 as libc::c_int;
    #[c2rust::src_loc = "217:9"]
    pub const OPUS_FRAMESIZE_100_MS: libc::c_int = 5008 as libc::c_int;
    #[c2rust::src_loc = "165:9"]
    pub const OPUS_GET_EXPERT_FRAME_DURATION_REQUEST: libc::c_int = 4041;
    #[c2rust::src_loc = "166:9"]
    pub const OPUS_SET_PREDICTION_DISABLED_REQUEST: libc::c_int = 4042;
    #[c2rust::src_loc = "167:9"]
    pub const OPUS_GET_PREDICTION_DISABLED_REQUEST: libc::c_int = 4043;
    #[c2rust::src_loc = "169:9"]
    pub const OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST: libc::c_int = 4046 as libc::c_int;
    #[c2rust::src_loc = "170:9"]
    pub const OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST: libc::c_int = 4047;
    #[c2rust::src_loc = "171:9"]
    pub const OPUS_GET_IN_DTX_REQUEST: libc::c_int = 4049;
    #[c2rust::src_loc = "56:9"]
    pub const OPUS_UNIMPLEMENTED: libc::c_int = -(5 as libc::c_int);
    #[c2rust::src_loc = "218:9"]
    pub const OPUS_FRAMESIZE_120_MS: libc::c_int = 5009 as libc::c_int;
    #[c2rust::src_loc = "214:9"]
    pub const OPUS_FRAMESIZE_40_MS: libc::c_int = 5005 as libc::c_int;
    #[c2rust::src_loc = "210:9"]
    pub const OPUS_FRAMESIZE_2_5_MS: libc::c_int = 5001 as libc::c_int;
    #[c2rust::src_loc = "201:9"]
    pub const OPUS_SIGNAL_VOICE: libc::c_int = 3001 as libc::c_int;
    #[c2rust::src_loc = "202:9"]
    pub const OPUS_SIGNAL_MUSIC: libc::c_int = 3002 as libc::c_int;
    #[c2rust::src_loc = "161:9"]
    pub const OPUS_SET_LSB_DEPTH_REQUEST: libc::c_int = 4036 as libc::c_int;
    #[c2rust::src_loc = "189:9"]
    pub const OPUS_BITRATE_MAX: libc::c_int = -(1 as libc::c_int);
    #[c2rust::src_loc = "205:9"]
    pub const OPUS_BANDWIDTH_WIDEBAND: libc::c_int = 1103 as libc::c_int;
    #[c2rust::src_loc = "148:9"]
    pub const OPUS_SET_VBR_CONSTRAINT_REQUEST: libc::c_int = 4020 as libc::c_int;
    #[c2rust::src_loc = "662:9"]
    pub const OPUS_RESET_STATE: libc::c_int = 4028 as libc::c_int;
    #[c2rust::src_loc = "136:9"]
    pub const OPUS_SET_VBR_REQUEST: libc::c_int = 4006 as libc::c_int;
    #[c2rust::src_loc = "132:9"]
    pub const OPUS_SET_BITRATE_REQUEST: libc::c_int = 4002 as libc::c_int;
    #[c2rust::src_loc = "157:9"]
    pub const OPUS_GET_FINAL_RANGE_REQUEST: libc::c_int = 4031 as libc::c_int;
    #[c2rust::src_loc = "203:9"]
    pub const OPUS_BANDWIDTH_NARROWBAND: libc::c_int = 1101 as libc::c_int;
    #[c2rust::src_loc = "204:9"]
    pub const OPUS_BANDWIDTH_MEDIUMBAND: libc::c_int = 1102 as libc::c_int;
    #[c2rust::src_loc = "206:9"]
    pub const OPUS_BANDWIDTH_SUPERWIDEBAND: libc::c_int = 1104 as libc::c_int;
    #[c2rust::src_loc = "50:9"]
    pub const OPUS_BUFFER_TOO_SMALL: libc::c_int = -(2 as libc::c_int);
    #[c2rust::src_loc = "60:9"]
    pub const OPUS_ALLOC_FAIL: libc::c_int = -(7 as libc::c_int);
    #[c2rust::src_loc = "193:9"]
    pub const OPUS_APPLICATION_VOIP: libc::c_int = 2048 as libc::c_int;
    #[c2rust::src_loc = "196:9"]
    pub const OPUS_APPLICATION_AUDIO: libc::c_int = 2049 as libc::c_int;
    #[c2rust::src_loc = "199:9"]
    pub const OPUS_APPLICATION_RESTRICTED_LOWDELAY: libc::c_int = 2051 as libc::c_int;
    #[c2rust::src_loc = "48:9"]
    pub const OPUS_BAD_ARG: libc::c_int = -(1 as libc::c_int);
    #[c2rust::src_loc = "52:9"]
    pub const OPUS_INTERNAL_ERROR: libc::c_int = -(3 as libc::c_int);
    #[c2rust::src_loc = "140:9"]
    pub const OPUS_SET_COMPLEXITY_REQUEST: libc::c_int = 4010 as libc::c_int;
    #[c2rust::src_loc = "188:9"]
    pub const OPUS_AUTO: libc::c_int = -(1000 as libc::c_int);
    #[c2rust::src_loc = "209:9"]
    pub const OPUS_FRAMESIZE_ARG: libc::c_int = 5000 as libc::c_int;
    #[c2rust::src_loc = "207:9"]
    pub const OPUS_BANDWIDTH_FULLBAND: libc::c_int = 1105 as libc::c_int;
    #[c2rust::src_loc = "46:9"]
    pub const OPUS_OK: libc::c_int = 0 as libc::c_int;
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
    use crate::celt::entenc::ec_enc;
    use crate::silk::enc_API::silk_EncControlStruct;
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
            samplesIn: *const i16,
            nSamplesIn: libc::c_int,
            psRangeEnc: *mut ec_enc,
            nBytesOut: *mut i32,
            prefillFlag: libc::c_int,
            activity: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/float_cast.h:38"]
pub mod float_cast_h {
    #[inline]
    #[c2rust::src_loc = "137:1"]
    pub unsafe extern "C" fn FLOAT2INT16(mut x: libc::c_float) -> i16 {
        x = x * CELT_SIG_SCALE;
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
        return float2int(x) as i16;
    }
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn float2int(x: libc::c_float) -> i32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }
    use super::arch_h::CELT_SIG_SCALE;
    use super::xmmintrin_h::{_mm_cvt_ss2si, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/pitch.h:41"]
pub mod pitch_h {
    #[inline]
    #[c2rust::src_loc = "159:1"]
    pub unsafe extern "C" fn celt_inner_prod_c(
        x: *const opus_val16,
        y: *const opus_val16,
        N: libc::c_int,
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mathops.h:46"]
pub mod mathops_h {
    #[inline]
    #[c2rust::src_loc = "80:1"]
    pub unsafe extern "C" fn celt_maxabs16(x: *const opus_val16, len: libc::c_int) -> opus_val32 {
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:51"]
pub mod define_h {
    #[c2rust::src_loc = "56:9"]
    pub const NB_SPEECH_FRAMES_BEFORE_DTX: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "57:9"]
    pub const MAX_CONSECUTIVE_DTX: libc::c_int = 20 as libc::c_int;
    #[c2rust::src_loc = "61:9"]
    pub const VAD_NO_DECISION: libc::c_int = -(1 as libc::c_int);
    #[c2rust::src_loc = "58:9"]
    pub const DTX_ACTIVITY_THRESHOLD: libc::c_float = 0.1f32;
}
pub use self::arch_h::{opus_val16, opus_val32, CELT_SIG_SCALE, EPSILON, Q15ONE, VERY_SMALL};
pub use self::celt_h::{
    celt_encode_with_ec, celt_encoder_ctl, celt_encoder_get_size, celt_encoder_init, SILKInfo,
    CELT_GET_MODE_REQUEST, CELT_SET_ANALYSIS_REQUEST, CELT_SET_CHANNELS_REQUEST,
    CELT_SET_END_BAND_REQUEST, CELT_SET_PREDICTION_REQUEST, CELT_SET_SIGNALLING_REQUEST,
    CELT_SET_SILK_INFO_REQUEST, CELT_SET_START_BAND_REQUEST, OPUS_SET_ENERGY_MASK_REQUEST,
    OPUS_SET_LFE_REQUEST,
};
pub use self::cpu_support_h::opus_select_arch;
pub use self::define_h::{
    DTX_ACTIVITY_THRESHOLD, MAX_CONSECUTIVE_DTX, NB_SPEECH_FRAMES_BEFORE_DTX, VAD_NO_DECISION,
};
pub use self::float_cast_h::{float2int, FLOAT2INT16};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::mathops_h::celt_maxabs16;
pub use self::opus_defines_h::{
    OPUS_ALLOC_FAIL, OPUS_APPLICATION_AUDIO, OPUS_APPLICATION_RESTRICTED_LOWDELAY,
    OPUS_APPLICATION_VOIP, OPUS_AUTO, OPUS_BAD_ARG, OPUS_BANDWIDTH_FULLBAND,
    OPUS_BANDWIDTH_MEDIUMBAND, OPUS_BANDWIDTH_NARROWBAND, OPUS_BANDWIDTH_SUPERWIDEBAND,
    OPUS_BANDWIDTH_WIDEBAND, OPUS_BITRATE_MAX, OPUS_BUFFER_TOO_SMALL, OPUS_FRAMESIZE_100_MS,
    OPUS_FRAMESIZE_10_MS, OPUS_FRAMESIZE_120_MS, OPUS_FRAMESIZE_20_MS, OPUS_FRAMESIZE_2_5_MS,
    OPUS_FRAMESIZE_40_MS, OPUS_FRAMESIZE_5_MS, OPUS_FRAMESIZE_60_MS, OPUS_FRAMESIZE_80_MS,
    OPUS_FRAMESIZE_ARG, OPUS_GET_APPLICATION_REQUEST, OPUS_GET_BANDWIDTH_REQUEST,
    OPUS_GET_BITRATE_REQUEST, OPUS_GET_COMPLEXITY_REQUEST, OPUS_GET_DTX_REQUEST,
    OPUS_GET_EXPERT_FRAME_DURATION_REQUEST, OPUS_GET_FINAL_RANGE_REQUEST,
    OPUS_GET_FORCE_CHANNELS_REQUEST, OPUS_GET_INBAND_FEC_REQUEST, OPUS_GET_IN_DTX_REQUEST,
    OPUS_GET_LOOKAHEAD_REQUEST, OPUS_GET_LSB_DEPTH_REQUEST, OPUS_GET_MAX_BANDWIDTH_REQUEST,
    OPUS_GET_PACKET_LOSS_PERC_REQUEST, OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST,
    OPUS_GET_PREDICTION_DISABLED_REQUEST, OPUS_GET_SAMPLE_RATE_REQUEST, OPUS_GET_SIGNAL_REQUEST,
    OPUS_GET_VBR_CONSTRAINT_REQUEST, OPUS_GET_VBR_REQUEST, OPUS_INTERNAL_ERROR, OPUS_OK,
    OPUS_RESET_STATE, OPUS_SET_APPLICATION_REQUEST, OPUS_SET_BANDWIDTH_REQUEST,
    OPUS_SET_BITRATE_REQUEST, OPUS_SET_COMPLEXITY_REQUEST, OPUS_SET_DTX_REQUEST,
    OPUS_SET_EXPERT_FRAME_DURATION_REQUEST, OPUS_SET_FORCE_CHANNELS_REQUEST,
    OPUS_SET_INBAND_FEC_REQUEST, OPUS_SET_LSB_DEPTH_REQUEST, OPUS_SET_MAX_BANDWIDTH_REQUEST,
    OPUS_SET_PACKET_LOSS_PERC_REQUEST, OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST,
    OPUS_SET_PREDICTION_DISABLED_REQUEST, OPUS_SET_SIGNAL_REQUEST, OPUS_SET_VBR_CONSTRAINT_REQUEST,
    OPUS_SET_VBR_REQUEST, OPUS_SIGNAL_MUSIC, OPUS_SIGNAL_VOICE, OPUS_UNIMPLEMENTED,
};
pub use self::pitch_h::celt_inner_prod_c;
pub use self::stdarg_h::va_list;
pub use self::stddef_h::{size_t, NULL};
pub use self::structs_FLP_h::{silk_encoder, silk_encoder_state_FLP, silk_shape_state_FLP};
use self::API_h::{silk_Encode, silk_Get_Encoder_Size, silk_InitEncoder};
use crate::celt::celt::celt_fatal;
use crate::celt::celt_encoder::{opus_custom_encoder_ctl, OpusCustomEncoder};
use crate::celt::entcode::ec_tell;
use crate::celt::entenc::ec_enc;
use crate::celt::entenc::{ec_enc_bit_logp, ec_enc_done, ec_enc_init, ec_enc_shrink, ec_enc_uint};
use crate::celt::modes::OpusCustomMode;
use crate::externs::{memcpy, memmove, memset};
use crate::silk::enc_API::silk_EncControlStruct;
use crate::silk::lin2log::silk_lin2log;
use crate::silk::log2lin::silk_log2lin;
use crate::src::analysis::{
    downmix_func, run_analysis, tonality_analysis_init, tonality_analysis_reset, AnalysisInfo,
    TonalityAnalysisState,
};
use crate::src::opus_private::{
    align, MODE_CELT_ONLY, MODE_HYBRID, MODE_SILK_ONLY, OPUS_GET_VOICE_RATIO_REQUEST,
    OPUS_SET_FORCE_MODE_REQUEST, OPUS_SET_VOICE_RATIO_REQUEST,
};
use crate::{
    opus_packet_pad, opus_repacketizer_cat, opus_repacketizer_init,
    opus_repacketizer_out_range_impl, OpusRepacketizer,
};

#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "66:8"]
pub struct OpusEncoder {
    pub(crate) celt_enc_offset: libc::c_int,
    pub(crate) silk_enc_offset: libc::c_int,
    pub(crate) silk_mode: silk_EncControlStruct,
    pub(crate) application: libc::c_int,
    pub(crate) channels: libc::c_int,
    pub(crate) delay_compensation: libc::c_int,
    pub(crate) force_channels: libc::c_int,
    pub(crate) signal_type: libc::c_int,
    pub(crate) user_bandwidth: libc::c_int,
    pub(crate) max_bandwidth: libc::c_int,
    pub(crate) user_forced_mode: libc::c_int,
    pub(crate) voice_ratio: libc::c_int,
    pub(crate) Fs: i32,
    pub(crate) use_vbr: libc::c_int,
    pub(crate) vbr_constraint: libc::c_int,
    pub(crate) variable_duration: libc::c_int,
    pub(crate) bitrate_bps: i32,
    pub(crate) user_bitrate_bps: i32,
    pub(crate) lsb_depth: libc::c_int,
    pub(crate) encoder_buffer: libc::c_int,
    pub(crate) lfe: libc::c_int,
    pub(crate) arch: libc::c_int,
    pub(crate) use_dtx: libc::c_int,
    pub(crate) analysis: TonalityAnalysisState,
    pub(crate) stream_channels: libc::c_int,
    pub(crate) hybrid_stereo_width_Q14: i16,
    pub(crate) variable_HP_smth2_Q15: i32,
    pub(crate) prev_HB_gain: opus_val16,
    pub(crate) hp_mem: [opus_val32; 4],
    pub(crate) mode: libc::c_int,
    pub(crate) prev_mode: libc::c_int,
    pub(crate) prev_channels: libc::c_int,
    pub(crate) prev_framesize: libc::c_int,
    pub(crate) bandwidth: libc::c_int,
    pub(crate) auto_bandwidth: libc::c_int,
    pub(crate) silk_bw_switch: libc::c_int,
    pub(crate) first: libc::c_int,
    pub(crate) energy_masking: *mut opus_val16,
    pub(crate) width_mem: StereoWidthState,
    pub(crate) delay_buffer: [opus_val16; 960],
    pub(crate) detected_bandwidth: libc::c_int,
    pub(crate) nb_no_activity_frames: libc::c_int,
    pub(crate) peak_signal_energy: opus_val32,
    pub(crate) nonfinal_frame: libc::c_int,
    pub(crate) rangeFinal: u32,
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
#[c2rust::src_loc = "57:9"]
pub const PSEUDO_SNR_THRESHOLD: libc::c_float = 316.23f32;
#[c2rust::src_loc = "125:25"]
static mut mono_voice_bandwidth_thresholds: [i32; 8] = [
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
static mut mono_music_bandwidth_thresholds: [i32; 8] = [
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
static mut stereo_voice_bandwidth_thresholds: [i32; 8] = [
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
static mut stereo_music_bandwidth_thresholds: [i32; 8] = [
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
static mut stereo_voice_threshold: i32 = 19000 as libc::c_int;
#[c2rust::src_loc = "151:25"]
static mut stereo_music_threshold: i32 = 17000 as libc::c_int;
#[c2rust::src_loc = "154:25"]
static mut mode_thresholds: [[i32; 2]; 2] = [
    [64000 as libc::c_int, 10000 as libc::c_int],
    [44000 as libc::c_int, 10000 as libc::c_int],
];
#[c2rust::src_loc = "160:25"]
static mut fec_thresholds: [i32; 10] = [
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
pub unsafe extern "C" fn opus_encoder_get_size(channels: libc::c_int) -> libc::c_int {
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
    Fs: i32,
    channels: libc::c_int,
    application: libc::c_int,
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
        || application != OPUS_APPLICATION_VOIP
            && application != OPUS_APPLICATION_AUDIO
            && application != OPUS_APPLICATION_RESTRICTED_LOWDELAY
    {
        return OPUS_BAD_ARG;
    }
    memset(
        st as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
        (opus_encoder_get_size(channels) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    ret = silk_Get_Encoder_Size(&mut silkEncSizeBytes);
    if ret != 0 {
        return OPUS_BAD_ARG;
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
        return OPUS_INTERNAL_ERROR;
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
    if err != OPUS_OK {
        return OPUS_INTERNAL_ERROR;
    }
    opus_custom_encoder_ctl(celt_enc, CELT_SET_SIGNALLING_REQUEST, 0 as libc::c_int);
    opus_custom_encoder_ctl(
        celt_enc,
        OPUS_SET_COMPLEXITY_REQUEST,
        (*st).silk_mode.complexity,
    );
    (*st).use_vbr = 1 as libc::c_int;
    (*st).vbr_constraint = 1 as libc::c_int;
    (*st).user_bitrate_bps = OPUS_AUTO;
    (*st).bitrate_bps = 3000 as libc::c_int + Fs * channels;
    (*st).application = application;
    (*st).signal_type = OPUS_AUTO;
    (*st).user_bandwidth = OPUS_AUTO;
    (*st).max_bandwidth = OPUS_BANDWIDTH_FULLBAND;
    (*st).force_channels = OPUS_AUTO;
    (*st).user_forced_mode = OPUS_AUTO;
    (*st).voice_ratio = -(1 as libc::c_int);
    (*st).encoder_buffer = (*st).Fs / 100 as libc::c_int;
    (*st).lsb_depth = 24 as libc::c_int;
    (*st).variable_duration = OPUS_FRAMESIZE_ARG;
    (*st).delay_compensation = (*st).Fs / 250 as libc::c_int;
    (*st).hybrid_stereo_width_Q14 = ((1 as libc::c_int) << 14 as libc::c_int) as i16;
    (*st).prev_HB_gain = Q15ONE;
    (*st).variable_HP_smth2_Q15 =
        ((silk_lin2log(60 as libc::c_int) as u32) << 8 as libc::c_int) as i32;
    (*st).first = 1 as libc::c_int;
    (*st).mode = MODE_HYBRID;
    (*st).bandwidth = OPUS_BANDWIDTH_FULLBAND;
    tonality_analysis_init(&mut (*st).analysis, (*st).Fs);
    (*st).analysis.application = (*st).application;
    return OPUS_OK;
}
#[c2rust::src_loc = "273:1"]
unsafe extern "C" fn gen_toc(
    mode: libc::c_int,
    mut framerate: libc::c_int,
    bandwidth: libc::c_int,
    channels: libc::c_int,
) -> libc::c_uchar {
    let mut period: libc::c_int = 0;
    let mut toc: libc::c_uchar = 0;
    period = 0 as libc::c_int;
    while framerate < 400 as libc::c_int {
        framerate <<= 1 as libc::c_int;
        period += 1;
    }
    if mode == MODE_SILK_ONLY {
        toc = (bandwidth - OPUS_BANDWIDTH_NARROWBAND << 5 as libc::c_int) as libc::c_uchar;
        toc =
            (toc as libc::c_int | (period - 2 as libc::c_int) << 3 as libc::c_int) as libc::c_uchar;
    } else if mode == MODE_CELT_ONLY {
        let mut tmp: libc::c_int = bandwidth - OPUS_BANDWIDTH_MEDIUMBAND;
        if tmp < 0 as libc::c_int {
            tmp = 0 as libc::c_int;
        }
        toc = 0x80 as libc::c_int as libc::c_uchar;
        toc = (toc as libc::c_int | tmp << 5 as libc::c_int) as libc::c_uchar;
        toc = (toc as libc::c_int | period << 3 as libc::c_int) as libc::c_uchar;
    } else {
        toc = 0x60 as libc::c_int as libc::c_uchar;
        toc = (toc as libc::c_int | bandwidth - OPUS_BANDWIDTH_SUPERWIDEBAND << 4 as libc::c_int)
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
    in_0: *const opus_val16,
    B_Q28: *const i32,
    A_Q28: *const i32,
    S: *mut opus_val32,
    out: *mut opus_val16,
    len: i32,
    stride: libc::c_int,
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
        *S.offset(1 as libc::c_int as isize) = -vout * A[1 as libc::c_int as usize]
            + B[2 as libc::c_int as usize] * inval
            + VERY_SMALL;
        *out.offset((k * stride) as isize) = vout;
        k += 1;
    }
}
#[c2rust::src_loc = "345:1"]
unsafe extern "C" fn hp_cutoff(
    in_0: *const opus_val16,
    cutoff_Hz: i32,
    out: *mut opus_val16,
    hp_mem: *mut opus_val32,
    len: libc::c_int,
    channels: libc::c_int,
    Fs: i32,
    _arch: libc::c_int,
) {
    let mut B_Q28: [i32; 3] = [0; 3];
    let mut A_Q28: [i32; 2] = [0; 2];
    let mut Fc_Q19: i32 = 0;
    let mut r_Q28: i32 = 0;
    let mut r_Q22: i32 = 0;
    Fc_Q19 = (1.5f64 * 3.14159f64 / 1000 as libc::c_int as libc::c_double
        * ((1 as libc::c_int as i64) << 19 as libc::c_int) as libc::c_double
        + 0.5f64) as i32 as i16 as i32
        * cutoff_Hz as i16 as i32
        / (Fs / 1000 as libc::c_int);
    r_Q28 = (1.0f64 * ((1 as libc::c_int as i64) << 28 as libc::c_int) as libc::c_double + 0.5f64)
        as i32
        - (0.92f64 * ((1 as libc::c_int as i64) << 9 as libc::c_int) as libc::c_double + 0.5f64)
            as i32
            * Fc_Q19;
    B_Q28[0 as libc::c_int as usize] = r_Q28;
    B_Q28[1 as libc::c_int as usize] = ((-r_Q28 as u32) << 1 as libc::c_int) as i32;
    B_Q28[2 as libc::c_int as usize] = r_Q28;
    r_Q22 = r_Q28 >> 6 as libc::c_int;
    A_Q28[0 as libc::c_int as usize] = (r_Q22 as i64
        * ((Fc_Q19 as i64 * Fc_Q19 as libc::c_long >> 16 as libc::c_int) as i32
            - (2.0f64 * ((1 as libc::c_int as i64) << 22 as libc::c_int) as libc::c_double + 0.5f64)
                as i32) as libc::c_long
        >> 16 as libc::c_int) as i32;
    A_Q28[1 as libc::c_int as usize] =
        (r_Q22 as i64 * r_Q22 as libc::c_long >> 16 as libc::c_int) as i32;
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
    in_0: *const opus_val16,
    cutoff_Hz: i32,
    out: *mut opus_val16,
    hp_mem: *mut opus_val32,
    len: libc::c_int,
    channels: libc::c_int,
    Fs: i32,
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
            m0 = coef * x0 + VERY_SMALL + coef2 * m0;
            m2 = coef * x1 + VERY_SMALL + coef2 * m2;
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
            m0_0 = coef * x + VERY_SMALL + coef2 * m0_0;
            *out.offset(i as isize) = y;
            i += 1;
        }
        *hp_mem.offset(0 as libc::c_int as isize) = m0_0;
    };
}
#[c2rust::src_loc = "445:1"]
unsafe extern "C" fn stereo_fade(
    in_0: *const opus_val16,
    out: *mut opus_val16,
    mut g1: opus_val16,
    mut g2: opus_val16,
    overlap48: libc::c_int,
    frame_size: libc::c_int,
    channels: libc::c_int,
    window: *const opus_val16,
    Fs: i32,
) {
    let mut i: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    let mut inc: libc::c_int = 0;
    inc = 48000 as libc::c_int / Fs;
    overlap = overlap48 / inc;
    g1 = Q15ONE - g1;
    g2 = Q15ONE - g2;
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
    in_0: *const opus_val16,
    out: *mut opus_val16,
    g1: opus_val16,
    g2: opus_val16,
    overlap48: libc::c_int,
    frame_size: libc::c_int,
    channels: libc::c_int,
    window: *const opus_val16,
    Fs: i32,
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
    Fs: i32,
    channels: libc::c_int,
    application: libc::c_int,
    error: *mut libc::c_int,
) -> *mut OpusEncoder {
    let mut ret: libc::c_int = 0;
    let mut st: *mut OpusEncoder = 0 as *mut OpusEncoder;
    if Fs != 48000 as libc::c_int
        && Fs != 24000 as libc::c_int
        && Fs != 16000 as libc::c_int
        && Fs != 12000 as libc::c_int
        && Fs != 8000 as libc::c_int
        || channels != 1 as libc::c_int && channels != 2 as libc::c_int
        || application != OPUS_APPLICATION_VOIP
            && application != OPUS_APPLICATION_AUDIO
            && application != OPUS_APPLICATION_RESTRICTED_LOWDELAY
    {
        if !error.is_null() {
            *error = OPUS_BAD_ARG;
        }
        return NULL as *mut OpusEncoder;
    }
    st = malloc(opus_encoder_get_size(channels) as size_t) as *mut OpusEncoder;
    if st.is_null() {
        if !error.is_null() {
            *error = OPUS_ALLOC_FAIL;
        }
        return NULL as *mut OpusEncoder;
    }
    ret = opus_encoder_init(st, Fs, channels, application);
    if !error.is_null() {
        *error = ret;
    }
    if ret != OPUS_OK {
        free(st as *mut libc::c_void);
        st = NULL as *mut OpusEncoder;
    }
    return st;
}
#[c2rust::src_loc = "546:1"]
unsafe extern "C" fn user_bitrate_to_bitrate(
    st: *mut OpusEncoder,
    mut frame_size: libc::c_int,
    max_data_bytes: libc::c_int,
) -> i32 {
    if frame_size == 0 {
        frame_size = (*st).Fs / 400 as libc::c_int;
    }
    if (*st).user_bitrate_bps == OPUS_AUTO {
        return 60 as libc::c_int * (*st).Fs / frame_size + (*st).Fs * (*st).channels;
    } else if (*st).user_bitrate_bps == OPUS_BITRATE_MAX {
        return max_data_bytes * 8 as libc::c_int * (*st).Fs / frame_size;
    } else {
        return (*st).user_bitrate_bps;
    };
}
#[no_mangle]
#[c2rust::src_loc = "564:1"]
pub unsafe extern "C" fn downmix_float(
    mut _x: *const libc::c_void,
    y: *mut opus_val32,
    subframe: libc::c_int,
    offset: libc::c_int,
    c1: libc::c_int,
    c2: libc::c_int,
    C: libc::c_int,
) {
    let mut x: *const libc::c_float = 0 as *const libc::c_float;
    let mut j: libc::c_int = 0;
    x = _x as *const libc::c_float;
    j = 0 as libc::c_int;
    while j < subframe {
        *y.offset(j as isize) = *x.offset(((j + offset) * C + c1) as isize) * CELT_SIG_SCALE;
        j += 1;
    }
    if c2 > -(1 as libc::c_int) {
        j = 0 as libc::c_int;
        while j < subframe {
            let ref mut fresh0 = *y.offset(j as isize);
            *fresh0 += *x.offset(((j + offset) * C + c2) as isize) * CELT_SIG_SCALE;
            j += 1;
        }
    } else if c2 == -(2 as libc::c_int) {
        let mut c: libc::c_int = 0;
        c = 1 as libc::c_int;
        while c < C {
            j = 0 as libc::c_int;
            while j < subframe {
                let ref mut fresh1 = *y.offset(j as isize);
                *fresh1 += *x.offset(((j + offset) * C + c) as isize) * CELT_SIG_SCALE;
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
    y: *mut opus_val32,
    subframe: libc::c_int,
    offset: libc::c_int,
    c1: libc::c_int,
    c2: libc::c_int,
    C: libc::c_int,
) {
    let mut x: *const i16 = 0 as *const i16;
    let mut j: libc::c_int = 0;
    x = _x as *const i16;
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
    frame_size: i32,
    variable_duration: libc::c_int,
    Fs: i32,
) -> i32 {
    let mut new_size: libc::c_int = 0;
    if frame_size < Fs / 400 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if variable_duration == OPUS_FRAMESIZE_ARG {
        new_size = frame_size;
    } else if variable_duration >= OPUS_FRAMESIZE_2_5_MS
        && variable_duration <= OPUS_FRAMESIZE_120_MS
    {
        if variable_duration <= OPUS_FRAMESIZE_40_MS {
            new_size = (Fs / 400 as libc::c_int) << variable_duration - OPUS_FRAMESIZE_2_5_MS;
        } else {
            new_size = (variable_duration - OPUS_FRAMESIZE_2_5_MS - 2 as libc::c_int) * Fs
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
    pcm: *const opus_val16,
    frame_size: libc::c_int,
    Fs: i32,
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
    short_alpha = Q15ONE
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
        sqrt_xx = ((*mem).XX).sqrt();
        sqrt_yy = ((*mem).YY).sqrt();
        qrrt_xx = (sqrt_xx).sqrt();
        qrrt_yy = (sqrt_yy).sqrt();
        (*mem).XY = if (*mem).XY < sqrt_xx * sqrt_yy {
            (*mem).XY
        } else {
            sqrt_xx * sqrt_yy
        };
        corr = (*mem).XY / (1e-15f32 + sqrt_xx * sqrt_yy);
        ldiff = 1.0f32 * (qrrt_xx - qrrt_yy).abs() / (EPSILON + qrrt_xx + qrrt_yy);
        width = (1.0f32 - corr * corr).sqrt() * ldiff;
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
    useInBandFEC: libc::c_int,
    PacketLoss_perc: libc::c_int,
    last_fec: libc::c_int,
    mode: libc::c_int,
    bandwidth: *mut libc::c_int,
    rate: i32,
) -> libc::c_int {
    let mut orig_bandwidth: libc::c_int = 0;
    if useInBandFEC == 0 || PacketLoss_perc == 0 as libc::c_int || mode == MODE_CELT_ONLY {
        return 0 as libc::c_int;
    }
    orig_bandwidth = *bandwidth;
    loop {
        let mut hysteresis: i32 = 0;
        let mut LBRR_rate_thres_bps: i32 = 0;
        LBRR_rate_thres_bps =
            fec_thresholds[(2 as libc::c_int * (*bandwidth - OPUS_BANDWIDTH_NARROWBAND)) as usize];
        hysteresis = fec_thresholds[(2 as libc::c_int * (*bandwidth - OPUS_BANDWIDTH_NARROWBAND)
            + 1 as libc::c_int) as usize];
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
            * (0.01f64 * ((1 as libc::c_int as i64) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as i32 as i16 as i64
            >> 16 as libc::c_int) as i32;
        if rate > LBRR_rate_thres_bps {
            return 1 as libc::c_int;
        } else if PacketLoss_perc <= 5 as libc::c_int {
            return 0 as libc::c_int;
        } else {
            if !(*bandwidth > OPUS_BANDWIDTH_NARROWBAND) {
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
    bandwidth: libc::c_int,
    frame20ms: libc::c_int,
    vbr: libc::c_int,
    fec: libc::c_int,
    channels: libc::c_int,
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
        let mut lo: i32 = 0;
        let mut hi: i32 = 0;
        let mut x0: i32 = 0;
        let mut x1: i32 = 0;
        lo = rate_table[(i - 1 as libc::c_int) as usize][entry as usize];
        hi = rate_table[i as usize][entry as usize];
        x0 = rate_table[(i - 1 as libc::c_int) as usize][0 as libc::c_int as usize];
        x1 = rate_table[i as usize][0 as libc::c_int as usize];
        silk_rate = (lo * (x1 - rate) + hi * (rate - x0)) / (x1 - x0);
    }
    if vbr == 0 {
        silk_rate += 100 as libc::c_int;
    }
    if bandwidth == OPUS_BANDWIDTH_SUPERWIDEBAND {
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
    bitrate: i32,
    channels: libc::c_int,
    frame_rate: libc::c_int,
    vbr: libc::c_int,
    mode: libc::c_int,
    complexity: libc::c_int,
    loss: libc::c_int,
) -> i32 {
    let mut equiv: i32 = 0;
    equiv = bitrate;
    if frame_rate > 50 as libc::c_int {
        equiv -=
            (40 as libc::c_int * channels + 20 as libc::c_int) * (frame_rate - 50 as libc::c_int);
    }
    if vbr == 0 {
        equiv -= equiv / 12 as libc::c_int;
    }
    equiv = equiv * (90 as libc::c_int + complexity) / 100 as libc::c_int;
    if mode == MODE_SILK_ONLY || mode == MODE_HYBRID {
        if complexity < 2 as libc::c_int {
            equiv = equiv * 4 as libc::c_int / 5 as libc::c_int;
        }
        equiv -= equiv * loss / (6 as libc::c_int * loss + 10 as libc::c_int);
    } else if mode == MODE_CELT_ONLY {
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
    pcm: *const opus_val16,
    frame_size: libc::c_int,
    channels: libc::c_int,
    lsb_depth: libc::c_int,
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
    pcm: *const opus_val16,
    frame_size: libc::c_int,
    channels: libc::c_int,
    _arch: libc::c_int,
) -> opus_val32 {
    let len: libc::c_int = frame_size * channels;
    return celt_inner_prod_c(pcm, pcm, len) / len as libc::c_float;
}
#[c2rust::src_loc = "895:1"]
unsafe extern "C" fn decide_dtx_mode(
    activity_probability: libc::c_float,
    nb_no_activity_frames: *mut libc::c_int,
    peak_signal_energy: opus_val32,
    pcm: *const opus_val16,
    frame_size: libc::c_int,
    channels: libc::c_int,
    mut is_silence: libc::c_int,
    arch: libc::c_int,
) -> libc::c_int {
    let mut noise_energy: opus_val32 = 0.;
    if is_silence == 0 {
        if activity_probability < DTX_ACTIVITY_THRESHOLD {
            noise_energy = compute_frame_energy(pcm, frame_size, channels, arch);
            is_silence = (peak_signal_energy >= PSEUDO_SNR_THRESHOLD * noise_energy) as libc::c_int;
        }
    }
    if is_silence != 0 {
        *nb_no_activity_frames += 1;
        if *nb_no_activity_frames > NB_SPEECH_FRAMES_BEFORE_DTX {
            if *nb_no_activity_frames <= NB_SPEECH_FRAMES_BEFORE_DTX + MAX_CONSECUTIVE_DTX {
                return 1 as libc::c_int;
            } else {
                *nb_no_activity_frames = NB_SPEECH_FRAMES_BEFORE_DTX;
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
    pcm: *const opus_val16,
    nb_frames: libc::c_int,
    frame_size: libc::c_int,
    data: *mut libc::c_uchar,
    out_data_bytes: i32,
    to_celt: libc::c_int,
    lsb_depth: libc::c_int,
    float_api: libc::c_int,
) -> i32 {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut bak_mode: libc::c_int = 0;
    let mut bak_bandwidth: libc::c_int = 0;
    let mut bak_channels: libc::c_int = 0;
    let mut bak_to_mono: libc::c_int = 0;
    let mut max_header_bytes: libc::c_int = 0;
    let mut bytes_per_frame: i32 = 0;
    let mut cbr_bytes: i32 = 0;
    let mut repacketize_len: i32 = 0;
    let mut tmp_len: libc::c_int = 0;
    max_header_bytes = if nb_frames == 2 as libc::c_int {
        3 as libc::c_int
    } else {
        2 as libc::c_int + (nb_frames - 1 as libc::c_int) * 2 as libc::c_int
    };
    if (*st).use_vbr != 0 || (*st).user_bitrate_bps == OPUS_BITRATE_MAX {
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
            (*st).user_forced_mode = MODE_CELT_ONLY;
        }
        tmp_len = opus_encode_native(
            st,
            pcm.offset((i * ((*st).channels * frame_size)) as isize),
            frame_size,
            tmp_data.as_mut_ptr().offset((i * bytes_per_frame) as isize),
            bytes_per_frame,
            lsb_depth,
            NULL as *const libc::c_void,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            ::core::mem::transmute::<libc::intptr_t, downmix_func>(NULL as libc::intptr_t),
            float_api,
        );
        if tmp_len < 0 as libc::c_int {
            return OPUS_INTERNAL_ERROR;
        }
        ret = opus_repacketizer_cat(
            rp.as_mut_ptr(),
            tmp_data.as_mut_ptr().offset((i * bytes_per_frame) as isize),
            tmp_len,
        );
        if ret < 0 as libc::c_int {
            return OPUS_INTERNAL_ERROR;
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
        return OPUS_INTERNAL_ERROR;
    }
    (*st).user_forced_mode = bak_mode;
    (*st).user_bandwidth = bak_bandwidth;
    (*st).force_channels = bak_channels;
    (*st).silk_mode.toMono = bak_to_mono;
    return ret;
}
#[c2rust::src_loc = "1038:1"]
unsafe extern "C" fn compute_redundancy_bytes(
    max_data_bytes: i32,
    bitrate_bps: i32,
    frame_rate: libc::c_int,
    channels: libc::c_int,
) -> libc::c_int {
    let mut redundancy_bytes_cap: libc::c_int = 0;
    let mut redundancy_bytes: libc::c_int = 0;
    let mut redundancy_rate: i32 = 0;
    let mut base_bits: libc::c_int = 0;
    let mut available_bits: i32 = 0;
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
    pcm: *const opus_val16,
    frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    out_data_bytes: i32,
    mut lsb_depth: libc::c_int,
    analysis_pcm: *const libc::c_void,
    analysis_size: i32,
    c1: libc::c_int,
    c2: libc::c_int,
    analysis_channels: libc::c_int,
    downmix: downmix_func,
    float_api: libc::c_int,
) -> i32 {
    let mut silk_enc: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut celt_enc: *mut OpusCustomEncoder = 0 as *mut OpusCustomEncoder;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut nBytes: i32 = 0;
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
    let mut redundant_rng: u32 = 0 as libc::c_int as u32;
    let mut cutoff_Hz: libc::c_int = 0;
    let mut hp_freq_smth1: libc::c_int = 0;
    let mut voice_est: libc::c_int = 0;
    let mut equiv_rate: i32 = 0;
    let mut delay_compensation: libc::c_int = 0;
    let mut frame_rate: libc::c_int = 0;
    let mut max_rate: i32 = 0;
    let mut curr_bandwidth: libc::c_int = 0;
    let mut HB_gain: opus_val16 = 0.;
    let mut max_data_bytes: i32 = 0;
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
    (*st).rangeFinal = 0 as libc::c_int as u32;
    if frame_size <= 0 as libc::c_int || max_data_bytes <= 0 as libc::c_int {
        return OPUS_BAD_ARG;
    }
    if max_data_bytes == 1 as libc::c_int && (*st).Fs == frame_size * 10 as libc::c_int {
        return OPUS_BUFFER_TOO_SMALL;
    }
    silk_enc =
        (st as *mut libc::c_char).offset((*st).silk_enc_offset as isize) as *mut libc::c_void;
    celt_enc =
        (st as *mut libc::c_char).offset((*st).celt_enc_offset as isize) as *mut OpusCustomEncoder;
    if (*st).application == OPUS_APPLICATION_RESTRICTED_LOWDELAY {
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
        CELT_GET_MODE_REQUEST,
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
        if is_silence == 0 && analysis_info.activity_probability > DTX_ACTIVITY_THRESHOLD {
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
        if (*st).signal_type == OPUS_AUTO {
            let mut prob: libc::c_float = 0.;
            if (*st).prev_mode == 0 as libc::c_int {
                prob = analysis_info.music_prob;
            } else if (*st).prev_mode == MODE_CELT_ONLY {
                prob = analysis_info.music_prob_max;
            } else {
                prob = analysis_info.music_prob_min;
            }
            (*st).voice_ratio = (0.5 + (100.0 * (1.0 - prob))).floor() as libc::c_int;
        }
        analysis_bandwidth = analysis_info.bandwidth;
        if analysis_bandwidth <= 12 as libc::c_int {
            (*st).detected_bandwidth = OPUS_BANDWIDTH_NARROWBAND;
        } else if analysis_bandwidth <= 14 as libc::c_int {
            (*st).detected_bandwidth = OPUS_BANDWIDTH_MEDIUMBAND;
        } else if analysis_bandwidth <= 16 as libc::c_int {
            (*st).detected_bandwidth = OPUS_BANDWIDTH_WIDEBAND;
        } else if analysis_bandwidth <= 18 as libc::c_int {
            (*st).detected_bandwidth = OPUS_BANDWIDTH_SUPERWIDEBAND;
        } else {
            (*st).detected_bandwidth = OPUS_BANDWIDTH_FULLBAND;
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
        let frame_rate12: libc::c_int = 12 as libc::c_int * (*st).Fs / frame_size;
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
            OPUS_BANDWIDTH_NARROWBAND
        } else {
            (*st).bandwidth
        };
        let mut packet_code: libc::c_int = 0 as libc::c_int;
        let mut num_multiframes: libc::c_int = 0 as libc::c_int;
        if tocmode == 0 as libc::c_int {
            tocmode = MODE_SILK_ONLY;
        }
        if frame_rate > 100 as libc::c_int {
            tocmode = MODE_CELT_ONLY;
        }
        if frame_rate == 25 as libc::c_int && tocmode != MODE_SILK_ONLY {
            frame_rate = 50 as libc::c_int;
            packet_code = 1 as libc::c_int;
        }
        if frame_rate <= 16 as libc::c_int {
            if out_data_bytes == 1 as libc::c_int
                || tocmode == MODE_SILK_ONLY && frame_rate != 10 as libc::c_int
            {
                tocmode = MODE_SILK_ONLY;
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
        if tocmode == MODE_SILK_ONLY && bw > OPUS_BANDWIDTH_WIDEBAND {
            bw = OPUS_BANDWIDTH_WIDEBAND;
        } else if tocmode == MODE_CELT_ONLY && bw == OPUS_BANDWIDTH_MEDIUMBAND {
            bw = OPUS_BANDWIDTH_NARROWBAND;
        } else if tocmode == MODE_HYBRID && bw <= OPUS_BANDWIDTH_SUPERWIDEBAND {
            bw = OPUS_BANDWIDTH_SUPERWIDEBAND;
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
            if ret == OPUS_OK {
                ret = max_data_bytes;
            } else {
                ret = OPUS_INTERNAL_ERROR;
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
    if (*st).signal_type == OPUS_SIGNAL_VOICE {
        voice_est = 127 as libc::c_int;
    } else if (*st).signal_type == OPUS_SIGNAL_MUSIC {
        voice_est = 0 as libc::c_int;
    } else if (*st).voice_ratio >= 0 as libc::c_int {
        voice_est = (*st).voice_ratio * 327 as libc::c_int >> 8 as libc::c_int;
        if (*st).application == OPUS_APPLICATION_AUDIO {
            voice_est = if voice_est < 115 as libc::c_int {
                voice_est
            } else {
                115 as libc::c_int
            };
        }
    } else if (*st).application == OPUS_APPLICATION_VOIP {
        voice_est = 115 as libc::c_int;
    } else {
        voice_est = 48 as libc::c_int;
    }
    if (*st).force_channels != OPUS_AUTO && (*st).channels == 2 as libc::c_int {
        (*st).stream_channels = (*st).force_channels;
    } else if (*st).channels == 2 as libc::c_int {
        let mut stereo_threshold: i32 = 0;
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
    if (*st).application == OPUS_APPLICATION_RESTRICTED_LOWDELAY {
        (*st).mode = MODE_CELT_ONLY;
    } else if (*st).user_forced_mode == OPUS_AUTO {
        let mut mode_voice: i32 = 0;
        let mut mode_music: i32 = 0;
        let mut threshold: i32 = 0;
        mode_voice = ((1.0f32 - stereo_width)
            * mode_thresholds[0 as libc::c_int as usize][0 as libc::c_int as usize]
                as libc::c_float
            + stereo_width
                * mode_thresholds[1 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_float) as i32;
        mode_music = ((1.0f32 - stereo_width)
            * mode_thresholds[1 as libc::c_int as usize][1 as libc::c_int as usize]
                as libc::c_float
            + stereo_width
                * mode_thresholds[1 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_float) as i32;
        threshold =
            mode_music + (voice_est * voice_est * (mode_voice - mode_music) >> 14 as libc::c_int);
        if (*st).application == OPUS_APPLICATION_VOIP {
            threshold += 8000 as libc::c_int;
        }
        if (*st).prev_mode == MODE_CELT_ONLY {
            threshold -= 4000 as libc::c_int;
        } else if (*st).prev_mode > 0 as libc::c_int {
            threshold += 4000 as libc::c_int;
        }
        (*st).mode = if equiv_rate >= threshold {
            MODE_CELT_ONLY
        } else {
            MODE_SILK_ONLY
        };
        if (*st).silk_mode.useInBandFEC != 0
            && (*st).silk_mode.packetLossPercentage
                > 128 as libc::c_int - voice_est >> 4 as libc::c_int
        {
            (*st).mode = MODE_SILK_ONLY;
        }
        if (*st).silk_mode.useDTX != 0 && voice_est > 100 as libc::c_int {
            (*st).mode = MODE_SILK_ONLY;
        }
        if max_data_bytes
            < (if frame_rate > 50 as libc::c_int {
                9000 as libc::c_int
            } else {
                6000 as libc::c_int
            }) * frame_size
                / ((*st).Fs * 8 as libc::c_int)
        {
            (*st).mode = MODE_CELT_ONLY;
        }
    } else {
        (*st).mode = (*st).user_forced_mode;
    }
    if (*st).mode != MODE_CELT_ONLY && frame_size < (*st).Fs / 100 as libc::c_int {
        (*st).mode = MODE_CELT_ONLY;
    }
    if (*st).lfe != 0 {
        (*st).mode = MODE_CELT_ONLY;
    }
    if (*st).prev_mode > 0 as libc::c_int
        && ((*st).mode != MODE_CELT_ONLY && (*st).prev_mode == MODE_CELT_ONLY
            || (*st).mode == MODE_CELT_ONLY && (*st).prev_mode != MODE_CELT_ONLY)
    {
        redundancy = 1 as libc::c_int;
        celt_to_silk = ((*st).mode != MODE_CELT_ONLY) as libc::c_int;
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
        && (*st).mode != MODE_CELT_ONLY
        && (*st).prev_mode != MODE_CELT_ONLY
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
    if (*st).mode != MODE_CELT_ONLY && (*st).prev_mode == MODE_CELT_ONLY {
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
    if (*st).mode == MODE_CELT_ONLY || (*st).first != 0 || (*st).silk_mode.allowBandwidthSwitch != 0
    {
        let mut voice_bandwidth_thresholds: *const i32 = 0 as *const i32;
        let mut music_bandwidth_thresholds: *const i32 = 0 as *const i32;
        let mut bandwidth_thresholds: [i32; 8] = [0; 8];
        let mut bandwidth: libc::c_int = OPUS_BANDWIDTH_FULLBAND;
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
                [(2 as libc::c_int * (bandwidth - OPUS_BANDWIDTH_MEDIUMBAND)) as usize];
            hysteresis = bandwidth_thresholds[(2 as libc::c_int
                * (bandwidth - OPUS_BANDWIDTH_MEDIUMBAND)
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
            if !(bandwidth > OPUS_BANDWIDTH_NARROWBAND) {
                break;
            }
        }
        if bandwidth == OPUS_BANDWIDTH_MEDIUMBAND {
            bandwidth = OPUS_BANDWIDTH_WIDEBAND;
        }
        (*st).auto_bandwidth = bandwidth;
        (*st).bandwidth = (*st).auto_bandwidth;
        if (*st).first == 0
            && (*st).mode != MODE_CELT_ONLY
            && (*st).silk_mode.inWBmodeWithoutVariableLP == 0
            && (*st).bandwidth > OPUS_BANDWIDTH_WIDEBAND
        {
            (*st).bandwidth = OPUS_BANDWIDTH_WIDEBAND;
        }
    }
    if (*st).bandwidth > (*st).max_bandwidth {
        (*st).bandwidth = (*st).max_bandwidth;
    }
    if (*st).user_bandwidth != OPUS_AUTO {
        (*st).bandwidth = (*st).user_bandwidth;
    }
    if (*st).mode != MODE_CELT_ONLY && max_rate < 15000 as libc::c_int {
        (*st).bandwidth = if (*st).bandwidth < 1103 as libc::c_int {
            (*st).bandwidth
        } else {
            1103 as libc::c_int
        };
    }
    if (*st).Fs <= 24000 as libc::c_int && (*st).bandwidth > OPUS_BANDWIDTH_SUPERWIDEBAND {
        (*st).bandwidth = OPUS_BANDWIDTH_SUPERWIDEBAND;
    }
    if (*st).Fs <= 16000 as libc::c_int && (*st).bandwidth > OPUS_BANDWIDTH_WIDEBAND {
        (*st).bandwidth = OPUS_BANDWIDTH_WIDEBAND;
    }
    if (*st).Fs <= 12000 as libc::c_int && (*st).bandwidth > OPUS_BANDWIDTH_MEDIUMBAND {
        (*st).bandwidth = OPUS_BANDWIDTH_MEDIUMBAND;
    }
    if (*st).Fs <= 8000 as libc::c_int && (*st).bandwidth > OPUS_BANDWIDTH_NARROWBAND {
        (*st).bandwidth = OPUS_BANDWIDTH_NARROWBAND;
    }
    if (*st).detected_bandwidth != 0 && (*st).user_bandwidth == OPUS_AUTO {
        let mut min_detected_bandwidth: libc::c_int = 0;
        if equiv_rate <= 18000 as libc::c_int * (*st).stream_channels
            && (*st).mode == MODE_CELT_ONLY
        {
            min_detected_bandwidth = OPUS_BANDWIDTH_NARROWBAND;
        } else if equiv_rate <= 24000 as libc::c_int * (*st).stream_channels
            && (*st).mode == MODE_CELT_ONLY
        {
            min_detected_bandwidth = OPUS_BANDWIDTH_MEDIUMBAND;
        } else if equiv_rate <= 30000 as libc::c_int * (*st).stream_channels {
            min_detected_bandwidth = OPUS_BANDWIDTH_WIDEBAND;
        } else if equiv_rate <= 44000 as libc::c_int * (*st).stream_channels {
            min_detected_bandwidth = OPUS_BANDWIDTH_SUPERWIDEBAND;
        } else {
            min_detected_bandwidth = OPUS_BANDWIDTH_FULLBAND;
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
    opus_custom_encoder_ctl(celt_enc, OPUS_SET_LSB_DEPTH_REQUEST, lsb_depth);
    if (*st).mode == MODE_CELT_ONLY && (*st).bandwidth == OPUS_BANDWIDTH_MEDIUMBAND {
        (*st).bandwidth = OPUS_BANDWIDTH_WIDEBAND;
    }
    if (*st).lfe != 0 {
        (*st).bandwidth = OPUS_BANDWIDTH_NARROWBAND;
    }
    curr_bandwidth = (*st).bandwidth;
    if (*st).mode == MODE_SILK_ONLY && curr_bandwidth > OPUS_BANDWIDTH_WIDEBAND {
        (*st).mode = MODE_HYBRID;
    }
    if (*st).mode == MODE_HYBRID && curr_bandwidth <= OPUS_BANDWIDTH_WIDEBAND {
        (*st).mode = MODE_SILK_ONLY;
    }
    if frame_size > (*st).Fs / 50 as libc::c_int && (*st).mode != MODE_SILK_ONLY
        || frame_size > 3 as libc::c_int * (*st).Fs / 50 as libc::c_int
    {
        let mut enc_frame_size: libc::c_int = 0;
        let mut nb_frames: libc::c_int = 0;
        if (*st).mode == MODE_SILK_ONLY {
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
    if (*st).mode == MODE_CELT_ONLY {
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
    ec_enc_init(&mut enc, data, (max_data_bytes - 1 as libc::c_int) as u32);
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
    if (*st).mode == MODE_CELT_ONLY {
        hp_freq_smth1 = ((silk_lin2log(60 as libc::c_int) as u32) << 8 as libc::c_int) as i32;
    } else {
        hp_freq_smth1 = (*(silk_enc as *mut silk_encoder)).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .variable_HP_smth1_Q15;
    }
    (*st).variable_HP_smth2_Q15 = ((*st).variable_HP_smth2_Q15 as libc::c_long
        + ((hp_freq_smth1 - (*st).variable_HP_smth2_Q15) as libc::c_long
            * ((0.015f32 * ((1 as libc::c_int as i64) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32 as i16 as i64
            >> 16 as libc::c_int)) as i32;
    cutoff_Hz = silk_log2lin((*st).variable_HP_smth2_Q15 >> 8 as libc::c_int);
    if (*st).application == OPUS_APPLICATION_VOIP {
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
    HB_gain = Q15ONE;
    if (*st).mode != MODE_CELT_ONLY {
        let mut total_bitRate: i32 = 0;
        let mut celt_rate: i32 = 0;
        let mut activity: libc::c_int = 0;
        let vla_0 = ((*st).channels * frame_size) as usize;
        let mut pcm_silk: Vec<i16> = ::std::vec::from_elem(0, vla_0);
        activity = VAD_NO_DECISION;
        if analysis_info.valid != 0 {
            activity =
                (analysis_info.activity_probability >= DTX_ACTIVITY_THRESHOLD) as libc::c_int;
        }
        total_bitRate = 8 as libc::c_int * bytes_target * frame_rate;
        if (*st).mode == MODE_HYBRID {
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
                HB_gain = Q15ONE
                    - (std::f32::consts::LN_2 * (-celt_rate as f32 * (1.0f32 / 1024f32))).exp();
            }
        } else {
            (*st).silk_mode.bitRate = total_bitRate;
        }
        if !((*st).energy_masking).is_null() && (*st).use_vbr != 0 && (*st).lfe == 0 {
            let mut mask_sum: opus_val32 = 0 as libc::c_int as opus_val32;
            let mut masking_depth: opus_val16 = 0.;
            let mut rate_offset: i32 = 0;
            let mut c: libc::c_int = 0;
            let mut end: libc::c_int = 17 as libc::c_int;
            let mut srate: i16 = 16000 as libc::c_int as i16;
            if (*st).bandwidth == OPUS_BANDWIDTH_NARROWBAND {
                end = 13 as libc::c_int;
                srate = 8000 as libc::c_int as i16;
            } else if (*st).bandwidth == OPUS_BANDWIDTH_MEDIUMBAND {
                end = 15 as libc::c_int;
                srate = 12000 as libc::c_int as i16;
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
            rate_offset = (srate as opus_val32 * masking_depth) as i32;
            rate_offset =
                if rate_offset > -(2 as libc::c_int) * (*st).silk_mode.bitRate / 3 as libc::c_int {
                    rate_offset
                } else {
                    -(2 as libc::c_int) * (*st).silk_mode.bitRate / 3 as libc::c_int
                };
            if (*st).bandwidth == OPUS_BANDWIDTH_SUPERWIDEBAND
                || (*st).bandwidth == OPUS_BANDWIDTH_FULLBAND
            {
                (*st).silk_mode.bitRate += 3 as libc::c_int * rate_offset / 5 as libc::c_int;
            } else {
                (*st).silk_mode.bitRate += rate_offset;
            }
        }
        (*st).silk_mode.payloadSize_ms = 1000 as libc::c_int * frame_size / (*st).Fs;
        (*st).silk_mode.nChannelsAPI = (*st).channels;
        (*st).silk_mode.nChannelsInternal = (*st).stream_channels;
        if curr_bandwidth == OPUS_BANDWIDTH_NARROWBAND {
            (*st).silk_mode.desiredInternalSampleRate = 8000 as libc::c_int;
        } else if curr_bandwidth == OPUS_BANDWIDTH_MEDIUMBAND {
            (*st).silk_mode.desiredInternalSampleRate = 12000 as libc::c_int;
        } else {
            if !((*st).mode == 1001 as libc::c_int || curr_bandwidth == 1103 as libc::c_int) {
                celt_fatal(
                    b"assertion failed: st->mode == MODE_HYBRID || curr_bandwidth == OPUS_BANDWIDTH_WIDEBAND\0"
                        as *const u8 as *const libc::c_char,
                    b"src/opus_encoder.c\0" as *const u8 as *const libc::c_char,
                    1755 as libc::c_int,
                );
            }
            (*st).silk_mode.desiredInternalSampleRate = 16000 as libc::c_int;
        }
        if (*st).mode == MODE_HYBRID {
            (*st).silk_mode.minInternalSampleRate = 16000 as libc::c_int;
        } else {
            (*st).silk_mode.minInternalSampleRate = 8000 as libc::c_int;
        }
        (*st).silk_mode.maxInternalSampleRate = 16000 as libc::c_int;
        if (*st).mode == MODE_SILK_ONLY {
            let mut effective_max_rate: i32 = max_rate;
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
            if (*st).mode == MODE_HYBRID {
                (*st).silk_mode.maxBits -= 20 as libc::c_int;
            }
        }
        if (*st).silk_mode.useCBR != 0 {
            if (*st).mode == MODE_HYBRID {
                (*st).silk_mode.maxBits =
                    if (*st).silk_mode.maxBits < (*st).silk_mode.bitRate * frame_size / (*st).Fs {
                        (*st).silk_mode.maxBits
                    } else {
                        (*st).silk_mode.bitRate * frame_size / (*st).Fs
                    };
            }
        } else if (*st).mode == MODE_HYBRID {
            let maxBitRate: i32 = compute_silk_rate_for_hybrid(
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
            let mut zero: i32 = 0 as libc::c_int;
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
                Q15ONE,
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
                NULL as *mut ec_enc,
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
            return OPUS_INTERNAL_ERROR;
        }
        if (*st).mode == MODE_SILK_ONLY {
            if (*st).silk_mode.internalSampleRate == 8000 as libc::c_int {
                curr_bandwidth = OPUS_BANDWIDTH_NARROWBAND;
            } else if (*st).silk_mode.internalSampleRate == 12000 as libc::c_int {
                curr_bandwidth = OPUS_BANDWIDTH_MEDIUMBAND;
            } else if (*st).silk_mode.internalSampleRate == 16000 as libc::c_int {
                curr_bandwidth = OPUS_BANDWIDTH_WIDEBAND;
            }
        } else if !((*st).silk_mode.internalSampleRate == 16000 as libc::c_int) {
            celt_fatal(
                b"assertion failed: st->silk_mode.internalSampleRate == 16000\0" as *const u8
                    as *const libc::c_char,
                b"src/opus_encoder.c\0" as *const u8 as *const libc::c_char,
                1863 as libc::c_int,
            );
        }
        (*st).silk_mode.opusCanSwitch =
            ((*st).silk_mode.switchReady != 0 && (*st).nonfinal_frame == 0) as libc::c_int;
        if nBytes == 0 as libc::c_int {
            (*st).rangeFinal = 0 as libc::c_int as u32;
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
        OPUS_BANDWIDTH_NARROWBAND => {
            endband = 13 as libc::c_int;
        }
        OPUS_BANDWIDTH_MEDIUMBAND | OPUS_BANDWIDTH_WIDEBAND => {
            endband = 17 as libc::c_int;
        }
        OPUS_BANDWIDTH_SUPERWIDEBAND => {
            endband = 19 as libc::c_int;
        }
        OPUS_BANDWIDTH_FULLBAND => {
            endband = 21 as libc::c_int;
        }
        _ => {}
    }
    opus_custom_encoder_ctl(celt_enc, CELT_SET_END_BAND_REQUEST, endband);
    opus_custom_encoder_ctl(celt_enc, CELT_SET_CHANNELS_REQUEST, (*st).stream_channels);
    opus_custom_encoder_ctl(celt_enc, OPUS_SET_BITRATE_REQUEST, -(1 as libc::c_int));
    if (*st).mode != MODE_SILK_ONLY {
        let mut celt_pred: opus_val32 = 2 as libc::c_int as opus_val32;
        opus_custom_encoder_ctl(celt_enc, OPUS_SET_VBR_REQUEST, 0 as libc::c_int);
        if (*st).silk_mode.reducedDependency != 0 {
            celt_pred = 0 as libc::c_int as opus_val32;
        }
        opus_custom_encoder_ctl(celt_enc, CELT_SET_PREDICTION_REQUEST, celt_pred as i32);
        if (*st).mode == MODE_HYBRID {
            if (*st).use_vbr != 0 {
                opus_custom_encoder_ctl(
                    celt_enc,
                    OPUS_SET_BITRATE_REQUEST,
                    (*st).bitrate_bps - (*st).silk_mode.bitRate,
                );
                opus_custom_encoder_ctl(
                    celt_enc,
                    OPUS_SET_VBR_CONSTRAINT_REQUEST,
                    0 as libc::c_int,
                );
            }
        } else if (*st).use_vbr != 0 {
            opus_custom_encoder_ctl(celt_enc, OPUS_SET_VBR_REQUEST, 1 as libc::c_int);
            opus_custom_encoder_ctl(
                celt_enc,
                OPUS_SET_VBR_CONSTRAINT_REQUEST,
                (*st).vbr_constraint,
            );
            opus_custom_encoder_ctl(celt_enc, OPUS_SET_BITRATE_REQUEST, (*st).bitrate_bps);
        }
    }
    let vla_1 = ((*st).channels * (*st).Fs / 400 as libc::c_int) as usize;
    let mut tmp_prefill: Vec<opus_val16> = ::std::vec::from_elem(0., vla_1);
    if (*st).mode != MODE_SILK_ONLY
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
    if (*st).prev_HB_gain < Q15ONE || HB_gain < Q15ONE {
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
    if (*st).mode != MODE_HYBRID || (*st).stream_channels == 1 as libc::c_int {
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
            (*st).hybrid_stereo_width_Q14 = (*st).silk_mode.stereoWidth_Q14 as i16;
        }
    }
    if (*st).mode != MODE_CELT_ONLY
        && ec_tell(&mut enc)
            + 17 as libc::c_int
            + 20 as libc::c_int * ((*st).mode == MODE_HYBRID) as libc::c_int
            <= 8 as libc::c_int * (max_data_bytes - 1 as libc::c_int)
    {
        if (*st).mode == MODE_HYBRID {
            ec_enc_bit_logp(&mut enc, redundancy, 12 as libc::c_int as libc::c_uint);
        }
        if redundancy != 0 {
            let mut max_redundancy: libc::c_int = 0;
            ec_enc_bit_logp(&mut enc, celt_to_silk, 1 as libc::c_int as libc::c_uint);
            if (*st).mode == MODE_HYBRID {
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
            if (*st).mode == MODE_HYBRID {
                ec_enc_uint(
                    &mut enc,
                    (redundancy_bytes - 2 as libc::c_int) as u32,
                    256 as libc::c_int as u32,
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
    if (*st).mode != MODE_CELT_ONLY {
        start_band = 17 as libc::c_int;
    }
    if (*st).mode == MODE_SILK_ONLY {
        ret = ec_tell(&mut enc) + 7 as libc::c_int >> 3 as libc::c_int;
        ec_enc_done(&mut enc);
        nb_compr_bytes = ret;
    } else {
        nb_compr_bytes = max_data_bytes - 1 as libc::c_int - redundancy_bytes;
        ec_enc_shrink(&mut enc, nb_compr_bytes as u32);
    }
    if redundancy != 0 || (*st).mode != MODE_SILK_ONLY {
        opus_custom_encoder_ctl(
            celt_enc,
            CELT_SET_ANALYSIS_REQUEST,
            (&mut analysis_info as *mut AnalysisInfo).offset(
                (&mut analysis_info as *mut AnalysisInfo)
                    .offset_from(&mut analysis_info as *mut AnalysisInfo as *const AnalysisInfo)
                    as libc::c_long as isize,
            ),
        );
    }
    if (*st).mode == MODE_HYBRID {
        let mut info: SILKInfo = SILKInfo {
            signalType: 0,
            offset: 0,
        };
        info.signalType = (*st).silk_mode.signalType;
        info.offset = (*st).silk_mode.offset;
        opus_custom_encoder_ctl(
            celt_enc,
            CELT_SET_SILK_INFO_REQUEST,
            (&mut info as *mut SILKInfo).offset(
                (&mut info as *mut SILKInfo)
                    .offset_from(&mut info as *mut SILKInfo as *const SILKInfo)
                    as libc::c_long as isize,
            ),
        );
    }
    if redundancy != 0 && celt_to_silk != 0 {
        let mut err: libc::c_int = 0;
        opus_custom_encoder_ctl(celt_enc, CELT_SET_START_BAND_REQUEST, 0 as libc::c_int);
        opus_custom_encoder_ctl(celt_enc, OPUS_SET_VBR_REQUEST, 0 as libc::c_int);
        opus_custom_encoder_ctl(celt_enc, OPUS_SET_BITRATE_REQUEST, -(1 as libc::c_int));
        err = celt_encode_with_ec(
            celt_enc,
            pcm_buf.as_mut_ptr(),
            (*st).Fs / 200 as libc::c_int,
            data.offset(nb_compr_bytes as isize),
            redundancy_bytes,
            NULL as *mut ec_enc,
        );
        if err < 0 as libc::c_int {
            return OPUS_INTERNAL_ERROR;
        }
        opus_custom_encoder_ctl(
            celt_enc,
            OPUS_GET_FINAL_RANGE_REQUEST,
            (&mut redundant_rng as *mut u32).offset(
                (&mut redundant_rng as *mut u32).offset_from(&mut redundant_rng as *mut u32)
                    as libc::c_long as isize,
            ),
        );
        opus_custom_encoder_ctl(celt_enc, OPUS_RESET_STATE);
    }
    opus_custom_encoder_ctl(celt_enc, CELT_SET_START_BAND_REQUEST, start_band);
    if (*st).mode != MODE_SILK_ONLY {
        if (*st).mode != (*st).prev_mode && (*st).prev_mode > 0 as libc::c_int {
            let mut dummy_0: [libc::c_uchar; 2] = [0; 2];
            opus_custom_encoder_ctl(celt_enc, OPUS_RESET_STATE);
            celt_encode_with_ec(
                celt_enc,
                tmp_prefill.as_mut_ptr(),
                (*st).Fs / 400 as libc::c_int,
                dummy_0.as_mut_ptr(),
                2 as libc::c_int,
                NULL as *mut ec_enc,
            );
            opus_custom_encoder_ctl(celt_enc, CELT_SET_PREDICTION_REQUEST, 0 as libc::c_int);
        }
        if ec_tell(&mut enc) <= 8 as libc::c_int * nb_compr_bytes {
            if redundancy != 0
                && celt_to_silk != 0
                && (*st).mode == MODE_HYBRID
                && (*st).use_vbr != 0
            {
                opus_custom_encoder_ctl(
                    celt_enc,
                    OPUS_SET_BITRATE_REQUEST,
                    (*st).bitrate_bps - (*st).silk_mode.bitRate,
                );
            }
            opus_custom_encoder_ctl(celt_enc, OPUS_SET_VBR_REQUEST, (*st).use_vbr);
            ret = celt_encode_with_ec(
                celt_enc,
                pcm_buf.as_mut_ptr(),
                frame_size,
                NULL as *mut libc::c_uchar,
                nb_compr_bytes,
                &mut enc,
            );
            if ret < 0 as libc::c_int {
                return OPUS_INTERNAL_ERROR;
            }
            if redundancy != 0
                && celt_to_silk != 0
                && (*st).mode == MODE_HYBRID
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
        opus_custom_encoder_ctl(celt_enc, OPUS_RESET_STATE);
        opus_custom_encoder_ctl(celt_enc, CELT_SET_START_BAND_REQUEST, 0 as libc::c_int);
        opus_custom_encoder_ctl(celt_enc, CELT_SET_PREDICTION_REQUEST, 0 as libc::c_int);
        opus_custom_encoder_ctl(celt_enc, OPUS_SET_VBR_REQUEST, 0 as libc::c_int);
        opus_custom_encoder_ctl(celt_enc, OPUS_SET_BITRATE_REQUEST, -(1 as libc::c_int));
        if (*st).mode == MODE_HYBRID {
            nb_compr_bytes = ret;
            ec_enc_shrink(&mut enc, nb_compr_bytes as u32);
        }
        celt_encode_with_ec(
            celt_enc,
            pcm_buf
                .as_mut_ptr()
                .offset(((*st).channels * (frame_size - N2 - N4)) as isize),
            N4,
            dummy_1.as_mut_ptr(),
            2 as libc::c_int,
            NULL as *mut ec_enc,
        );
        err_0 = celt_encode_with_ec(
            celt_enc,
            pcm_buf
                .as_mut_ptr()
                .offset(((*st).channels * (frame_size - N2)) as isize),
            N2,
            data.offset(nb_compr_bytes as isize),
            redundancy_bytes,
            NULL as *mut ec_enc,
        );
        if err_0 < 0 as libc::c_int {
            return OPUS_INTERNAL_ERROR;
        }
        opus_custom_encoder_ctl(
            celt_enc,
            OPUS_GET_FINAL_RANGE_REQUEST,
            (&mut redundant_rng as *mut u32).offset(
                (&mut redundant_rng as *mut u32).offset_from(&mut redundant_rng as *mut u32)
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
        (*st).prev_mode = MODE_CELT_ONLY;
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
            (*st).rangeFinal = 0 as libc::c_int as u32;
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
            return OPUS_BUFFER_TOO_SMALL;
        }
        *data.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
        ret = 1 as libc::c_int;
        (*st).rangeFinal = 0 as libc::c_int as u32;
    } else if (*st).mode == MODE_SILK_ONLY && redundancy == 0 {
        while ret > 2 as libc::c_int
            && *data.offset(ret as isize) as libc::c_int == 0 as libc::c_int
        {
            ret -= 1;
        }
    }
    ret += 1 as libc::c_int + redundancy_bytes;
    if (*st).use_vbr == 0 {
        if opus_packet_pad(data, ret, max_data_bytes) != OPUS_OK {
            return OPUS_INTERNAL_ERROR;
        }
        ret = max_data_bytes;
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2235:1"]
pub unsafe extern "C" fn opus_encode(
    st: *mut OpusEncoder,
    pcm: *const i16,
    analysis_frame_size: libc::c_int,
    data: *mut libc::c_uchar,
    max_data_bytes: i32,
) -> i32 {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut frame_size: libc::c_int = 0;
    frame_size = frame_size_select(analysis_frame_size, (*st).variable_duration, (*st).Fs);
    if frame_size <= 0 as libc::c_int {
        return OPUS_BAD_ARG;
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
    st: *mut OpusEncoder,
    pcm: *const libc::c_float,
    analysis_frame_size: libc::c_int,
    data: *mut libc::c_uchar,
    out_data_bytes: i32,
) -> i32 {
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
    request: libc::c_int,
    args: ...
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut celt_enc: *mut OpusCustomEncoder = 0 as *mut OpusCustomEncoder;
    let mut ap: ::core::ffi::VaListImpl;
    ret = OPUS_OK;
    ap = args.clone();
    celt_enc =
        (st as *mut libc::c_char).offset((*st).celt_enc_offset as isize) as *mut OpusCustomEncoder;
    match request {
        OPUS_SET_APPLICATION_REQUEST => {
            let value: i32 = ap.arg::<i32>();
            if value != OPUS_APPLICATION_VOIP
                && value != OPUS_APPLICATION_AUDIO
                && value != OPUS_APPLICATION_RESTRICTED_LOWDELAY
                || (*st).first == 0 && (*st).application != value
            {
                ret = OPUS_BAD_ARG;
            } else {
                (*st).application = value;
                (*st).analysis.application = value;
            }
            current_block = 16167632229894708628;
        }
        OPUS_GET_APPLICATION_REQUEST => {
            let value_0: *mut i32 = ap.arg::<*mut i32>();
            if value_0.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_0 = (*st).application;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_BITRATE_REQUEST => {
            let mut value_1: i32 = ap.arg::<i32>();
            if value_1 != OPUS_AUTO && value_1 != OPUS_BITRATE_MAX {
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
        OPUS_GET_BITRATE_REQUEST => {
            let value_2: *mut i32 = ap.arg::<*mut i32>();
            if value_2.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_2 = user_bitrate_to_bitrate(st, (*st).prev_framesize, 1276 as libc::c_int);
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_FORCE_CHANNELS_REQUEST => {
            let value_3: i32 = ap.arg::<i32>();
            if (value_3 < 1 as libc::c_int || value_3 > (*st).channels) && value_3 != OPUS_AUTO {
                current_block = 12343738388509029619;
            } else {
                (*st).force_channels = value_3;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_FORCE_CHANNELS_REQUEST => {
            let value_4: *mut i32 = ap.arg::<*mut i32>();
            if value_4.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_4 = (*st).force_channels;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_MAX_BANDWIDTH_REQUEST => {
            let value_5: i32 = ap.arg::<i32>();
            if value_5 < OPUS_BANDWIDTH_NARROWBAND || value_5 > OPUS_BANDWIDTH_FULLBAND {
                current_block = 12343738388509029619;
            } else {
                (*st).max_bandwidth = value_5;
                if (*st).max_bandwidth == OPUS_BANDWIDTH_NARROWBAND {
                    (*st).silk_mode.maxInternalSampleRate = 8000 as libc::c_int;
                } else if (*st).max_bandwidth == OPUS_BANDWIDTH_MEDIUMBAND {
                    (*st).silk_mode.maxInternalSampleRate = 12000 as libc::c_int;
                } else {
                    (*st).silk_mode.maxInternalSampleRate = 16000 as libc::c_int;
                }
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_MAX_BANDWIDTH_REQUEST => {
            let value_6: *mut i32 = ap.arg::<*mut i32>();
            if value_6.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_6 = (*st).max_bandwidth;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_BANDWIDTH_REQUEST => {
            let value_7: i32 = ap.arg::<i32>();
            if (value_7 < OPUS_BANDWIDTH_NARROWBAND || value_7 > OPUS_BANDWIDTH_FULLBAND)
                && value_7 != OPUS_AUTO
            {
                current_block = 12343738388509029619;
            } else {
                (*st).user_bandwidth = value_7;
                if (*st).user_bandwidth == OPUS_BANDWIDTH_NARROWBAND {
                    (*st).silk_mode.maxInternalSampleRate = 8000 as libc::c_int;
                } else if (*st).user_bandwidth == OPUS_BANDWIDTH_MEDIUMBAND {
                    (*st).silk_mode.maxInternalSampleRate = 12000 as libc::c_int;
                } else {
                    (*st).silk_mode.maxInternalSampleRate = 16000 as libc::c_int;
                }
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_BANDWIDTH_REQUEST => {
            let value_8: *mut i32 = ap.arg::<*mut i32>();
            if value_8.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_8 = (*st).bandwidth;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_DTX_REQUEST => {
            let value_9: i32 = ap.arg::<i32>();
            if value_9 < 0 as libc::c_int || value_9 > 1 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).use_dtx = value_9;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_DTX_REQUEST => {
            let value_10: *mut i32 = ap.arg::<*mut i32>();
            if value_10.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_10 = (*st).use_dtx;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_COMPLEXITY_REQUEST => {
            let value_11: i32 = ap.arg::<i32>();
            if value_11 < 0 as libc::c_int || value_11 > 10 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).silk_mode.complexity = value_11;
                opus_custom_encoder_ctl(celt_enc, OPUS_SET_COMPLEXITY_REQUEST, value_11);
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_COMPLEXITY_REQUEST => {
            let value_12: *mut i32 = ap.arg::<*mut i32>();
            if value_12.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_12 = (*st).silk_mode.complexity;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_INBAND_FEC_REQUEST => {
            let value_13: i32 = ap.arg::<i32>();
            if value_13 < 0 as libc::c_int || value_13 > 1 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).silk_mode.useInBandFEC = value_13;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_INBAND_FEC_REQUEST => {
            let value_14: *mut i32 = ap.arg::<*mut i32>();
            if value_14.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_14 = (*st).silk_mode.useInBandFEC;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_PACKET_LOSS_PERC_REQUEST => {
            let value_15: i32 = ap.arg::<i32>();
            if value_15 < 0 as libc::c_int || value_15 > 100 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).silk_mode.packetLossPercentage = value_15;
                opus_custom_encoder_ctl(celt_enc, OPUS_SET_PACKET_LOSS_PERC_REQUEST, value_15);
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_PACKET_LOSS_PERC_REQUEST => {
            let value_16: *mut i32 = ap.arg::<*mut i32>();
            if value_16.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_16 = (*st).silk_mode.packetLossPercentage;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_VBR_REQUEST => {
            let value_17: i32 = ap.arg::<i32>();
            if value_17 < 0 as libc::c_int || value_17 > 1 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).use_vbr = value_17;
                (*st).silk_mode.useCBR = 1 as libc::c_int - value_17;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_VBR_REQUEST => {
            let value_18: *mut i32 = ap.arg::<*mut i32>();
            if value_18.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_18 = (*st).use_vbr;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_VOICE_RATIO_REQUEST => {
            let value_19: i32 = ap.arg::<i32>();
            if value_19 < -(1 as libc::c_int) || value_19 > 100 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).voice_ratio = value_19;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_VOICE_RATIO_REQUEST => {
            let value_20: *mut i32 = ap.arg::<*mut i32>();
            if value_20.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_20 = (*st).voice_ratio;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_VBR_CONSTRAINT_REQUEST => {
            let value_21: i32 = ap.arg::<i32>();
            if value_21 < 0 as libc::c_int || value_21 > 1 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).vbr_constraint = value_21;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_VBR_CONSTRAINT_REQUEST => {
            let value_22: *mut i32 = ap.arg::<*mut i32>();
            if value_22.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_22 = (*st).vbr_constraint;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_SIGNAL_REQUEST => {
            let value_23: i32 = ap.arg::<i32>();
            if value_23 != OPUS_AUTO
                && value_23 != OPUS_SIGNAL_VOICE
                && value_23 != OPUS_SIGNAL_MUSIC
            {
                current_block = 12343738388509029619;
            } else {
                (*st).signal_type = value_23;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_SIGNAL_REQUEST => {
            let value_24: *mut i32 = ap.arg::<*mut i32>();
            if value_24.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_24 = (*st).signal_type;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_LOOKAHEAD_REQUEST => {
            let value_25: *mut i32 = ap.arg::<*mut i32>();
            if value_25.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_25 = (*st).Fs / 400 as libc::c_int;
                if (*st).application != OPUS_APPLICATION_RESTRICTED_LOWDELAY {
                    *value_25 += (*st).delay_compensation;
                }
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_SAMPLE_RATE_REQUEST => {
            let value_26: *mut i32 = ap.arg::<*mut i32>();
            if value_26.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_26 = (*st).Fs;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_FINAL_RANGE_REQUEST => {
            let value_27: *mut u32 = ap.arg::<*mut u32>();
            if value_27.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_27 = (*st).rangeFinal;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_LSB_DEPTH_REQUEST => {
            let value_28: i32 = ap.arg::<i32>();
            if value_28 < 8 as libc::c_int || value_28 > 24 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).lsb_depth = value_28;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_LSB_DEPTH_REQUEST => {
            let value_29: *mut i32 = ap.arg::<*mut i32>();
            if value_29.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_29 = (*st).lsb_depth;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_EXPERT_FRAME_DURATION_REQUEST => {
            let value_30: i32 = ap.arg::<i32>();
            if value_30 != OPUS_FRAMESIZE_ARG
                && value_30 != OPUS_FRAMESIZE_2_5_MS
                && value_30 != OPUS_FRAMESIZE_5_MS
                && value_30 != OPUS_FRAMESIZE_10_MS
                && value_30 != OPUS_FRAMESIZE_20_MS
                && value_30 != OPUS_FRAMESIZE_40_MS
                && value_30 != OPUS_FRAMESIZE_60_MS
                && value_30 != OPUS_FRAMESIZE_80_MS
                && value_30 != OPUS_FRAMESIZE_100_MS
                && value_30 != OPUS_FRAMESIZE_120_MS
            {
                current_block = 12343738388509029619;
            } else {
                (*st).variable_duration = value_30;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_EXPERT_FRAME_DURATION_REQUEST => {
            let value_31: *mut i32 = ap.arg::<*mut i32>();
            if value_31.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_31 = (*st).variable_duration;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_PREDICTION_DISABLED_REQUEST => {
            let value_32: i32 = ap.arg::<i32>();
            if value_32 > 1 as libc::c_int || value_32 < 0 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                (*st).silk_mode.reducedDependency = value_32;
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_PREDICTION_DISABLED_REQUEST => {
            let value_33: *mut i32 = ap.arg::<*mut i32>();
            if value_33.is_null() {
                current_block = 12343738388509029619;
            } else {
                *value_33 = (*st).silk_mode.reducedDependency;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST => {
            let value_34: i32 = ap.arg::<i32>();
            if value_34 < 0 as libc::c_int || value_34 > 1 as libc::c_int {
                current_block = 12343738388509029619;
            } else {
                opus_custom_encoder_ctl(
                    celt_enc,
                    OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST,
                    value_34,
                );
                current_block = 16167632229894708628;
            }
        }
        OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST => {
            let value_35: *mut i32 = ap.arg::<*mut i32>();
            if value_35.is_null() {
                current_block = 12343738388509029619;
            } else {
                opus_custom_encoder_ctl(
                    celt_enc,
                    OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST,
                    value_35.offset(value_35.offset_from(value_35) as libc::c_long as isize),
                );
                current_block = 16167632229894708628;
            }
        }
        OPUS_RESET_STATE => {
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
            opus_custom_encoder_ctl(celt_enc, OPUS_RESET_STATE);
            silk_InitEncoder(silk_enc, (*st).arch, &mut dummy);
            (*st).stream_channels = (*st).channels;
            (*st).hybrid_stereo_width_Q14 = ((1 as libc::c_int) << 14 as libc::c_int) as i16;
            (*st).prev_HB_gain = Q15ONE;
            (*st).first = 1 as libc::c_int;
            (*st).mode = MODE_HYBRID;
            (*st).bandwidth = OPUS_BANDWIDTH_FULLBAND;
            (*st).variable_HP_smth2_Q15 =
                ((silk_lin2log(60 as libc::c_int) as u32) << 8 as libc::c_int) as i32;
            current_block = 16167632229894708628;
        }
        OPUS_SET_FORCE_MODE_REQUEST => {
            let value_36: i32 = ap.arg::<i32>();
            if (value_36 < MODE_SILK_ONLY || value_36 > MODE_CELT_ONLY) && value_36 != OPUS_AUTO {
                current_block = 12343738388509029619;
            } else {
                (*st).user_forced_mode = value_36;
                current_block = 16167632229894708628;
            }
        }
        OPUS_SET_LFE_REQUEST => {
            let value_37: i32 = ap.arg::<i32>();
            (*st).lfe = value_37;
            ret = opus_custom_encoder_ctl(celt_enc, OPUS_SET_LFE_REQUEST, value_37);
            current_block = 16167632229894708628;
        }
        OPUS_SET_ENERGY_MASK_REQUEST => {
            let value_38: *mut opus_val16 = ap.arg::<*mut opus_val16>();
            (*st).energy_masking = value_38;
            ret = opus_custom_encoder_ctl(
                celt_enc,
                OPUS_SET_ENERGY_MASK_REQUEST,
                value_38.offset(value_38.offset_from(value_38) as libc::c_long as isize),
            );
            current_block = 16167632229894708628;
        }
        OPUS_GET_IN_DTX_REQUEST => {
            let value_39: *mut i32 = ap.arg::<*mut i32>();
            if value_39.is_null() {
                current_block = 12343738388509029619;
            } else {
                if (*st).silk_mode.useDTX != 0
                    && ((*st).prev_mode == MODE_SILK_ONLY || (*st).prev_mode == MODE_HYBRID)
                {
                    let mut n: libc::c_int = 0;
                    let silk_enc_0: *mut libc::c_void = (st as *mut libc::c_char)
                        .offset((*st).silk_enc_offset as isize)
                        as *mut libc::c_void;
                    *value_39 = 1 as libc::c_int;
                    n = 0 as libc::c_int;
                    while n < (*st).silk_mode.nChannelsInternal {
                        *value_39 = (*value_39 != 0
                            && (*(silk_enc_0 as *mut silk_encoder)).state_Fxx[n as usize]
                                .sCmn
                                .noSpeechCounter
                                >= NB_SPEECH_FRAMES_BEFORE_DTX)
                            as libc::c_int;
                        n += 1;
                    }
                } else if (*st).use_dtx != 0 {
                    *value_39 =
                        ((*st).nb_no_activity_frames >= NB_SPEECH_FRAMES_BEFORE_DTX) as libc::c_int;
                } else {
                    *value_39 = 0 as libc::c_int;
                }
                current_block = 16167632229894708628;
            }
        }
        CELT_GET_MODE_REQUEST => {
            let value_40: *mut *const OpusCustomMode = ap.arg::<*mut *const OpusCustomMode>();
            if value_40.is_null() {
                current_block = 12343738388509029619;
            } else {
                ret = opus_custom_encoder_ctl(
                    celt_enc,
                    CELT_GET_MODE_REQUEST,
                    value_40.offset(value_40.offset_from(value_40) as libc::c_long as isize),
                );
                current_block = 16167632229894708628;
            }
        }
        _ => {
            ret = OPUS_UNIMPLEMENTED;
            current_block = 16167632229894708628;
        }
    }
    match current_block {
        12343738388509029619 => return OPUS_BAD_ARG,
        _ => return ret,
    };
}
#[no_mangle]
#[c2rust::src_loc = "2780:1"]
pub unsafe extern "C" fn opus_encoder_destroy(st: *mut OpusEncoder) {
    free(st as *mut libc::c_void);
}
