use crate::externs::{free, malloc};

#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: u32,
        pub fp_offset: u32,
        pub overflow_arg_area: *mut core::ffi::c_void,
        pub reg_save_area: *mut core::ffi::c_void,
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stdarg.h:40"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:41"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = f32;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = f32;
    #[c2rust::src_loc = "57:9"]
    pub const CELT_SIG_SCALE: f32 = 32768.0f32;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:41"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = u64;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: i32 = 0 as i32;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/xmmintrin.h:47"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_defines.h:41"]
pub mod opus_defines_h {
    #[c2rust::src_loc = "139:9"]
    pub const OPUS_GET_BANDWIDTH_REQUEST: i32 = 4009;
    #[c2rust::src_loc = "157:9"]
    pub const OPUS_GET_FINAL_RANGE_REQUEST: i32 = 4031;
    #[c2rust::src_loc = "662:9"]
    pub const OPUS_RESET_STATE: i32 = 4028;
    #[c2rust::src_loc = "156:9"]
    pub const OPUS_GET_SAMPLE_RATE_REQUEST: i32 = 4029;
    #[c2rust::src_loc = "158:9"]
    pub const OPUS_GET_PITCH_REQUEST: i32 = 4033;
    #[c2rust::src_loc = "160:9"]
    pub const OPUS_GET_GAIN_REQUEST: i32 = 4045;
    #[c2rust::src_loc = "159:9"]
    pub const OPUS_SET_GAIN_REQUEST: i32 = 4034;
    #[c2rust::src_loc = "163:9"]
    pub const OPUS_GET_LAST_PACKET_DURATION_REQUEST: i32 = 4039;
    #[c2rust::src_loc = "169:9"]
    pub const OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST: i32 = 4046 as i32;
    #[c2rust::src_loc = "170:9"]
    pub const OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST: i32 = 4047;
    #[c2rust::src_loc = "56:9"]
    pub const OPUS_UNIMPLEMENTED: i32 = -(5 as i32);
    #[c2rust::src_loc = "54:9"]
    pub const OPUS_INVALID_PACKET: i32 = -(4 as i32);
    #[c2rust::src_loc = "50:9"]
    pub const OPUS_BUFFER_TOO_SMALL: i32 = -(2 as i32);
    #[c2rust::src_loc = "203:9"]
    pub const OPUS_BANDWIDTH_NARROWBAND: i32 = 1101 as i32;
    #[c2rust::src_loc = "204:9"]
    pub const OPUS_BANDWIDTH_MEDIUMBAND: i32 = 1102 as i32;
    #[c2rust::src_loc = "205:9"]
    pub const OPUS_BANDWIDTH_WIDEBAND: i32 = 1103;
    #[c2rust::src_loc = "206:9"]
    pub const OPUS_BANDWIDTH_SUPERWIDEBAND: i32 = 1104 as i32;
    #[c2rust::src_loc = "207:9"]
    pub const OPUS_BANDWIDTH_FULLBAND: i32 = 1105 as i32;
    #[c2rust::src_loc = "60:9"]
    pub const OPUS_ALLOC_FAIL: i32 = -(7 as i32);
    #[c2rust::src_loc = "48:9"]
    pub const OPUS_BAD_ARG: i32 = -(1 as i32);
    #[c2rust::src_loc = "52:9"]
    pub const OPUS_INTERNAL_ERROR: i32 = -(3 as i32);
    #[c2rust::src_loc = "46:9"]
    pub const OPUS_OK: i32 = 0 as i32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cpu_support.h:44"]
pub mod cpu_support_h {
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe fn opus_select_arch() -> i32 {
        return 0 as i32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/stack_alloc.h:46"]
pub mod stack_alloc_h {
    #[c2rust::src_loc = "99:9"]
    pub const ALLOC_NONE: i32 = 1 as i32;
    #[inline]
    #[c2rust::src_loc = "175:1"]
    pub unsafe fn _opus_false() -> i32 {
        return 0 as i32;
    }
}
pub use self::arch_h::{opus_val16, opus_val32, CELT_SIG_SCALE};
pub use self::cpu_support_h::opus_select_arch;
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::opus_defines_h::{
    OPUS_ALLOC_FAIL, OPUS_BAD_ARG, OPUS_BANDWIDTH_FULLBAND, OPUS_BANDWIDTH_MEDIUMBAND,
    OPUS_BANDWIDTH_NARROWBAND, OPUS_BANDWIDTH_SUPERWIDEBAND, OPUS_BANDWIDTH_WIDEBAND,
    OPUS_BUFFER_TOO_SMALL, OPUS_GET_BANDWIDTH_REQUEST, OPUS_GET_FINAL_RANGE_REQUEST,
    OPUS_GET_GAIN_REQUEST, OPUS_GET_LAST_PACKET_DURATION_REQUEST,
    OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST, OPUS_GET_PITCH_REQUEST,
    OPUS_GET_SAMPLE_RATE_REQUEST, OPUS_INTERNAL_ERROR, OPUS_INVALID_PACKET, OPUS_OK,
    OPUS_RESET_STATE, OPUS_SET_GAIN_REQUEST, OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST,
    OPUS_UNIMPLEMENTED,
};
pub use self::stack_alloc_h::{_opus_false, ALLOC_NONE};
pub use self::stdarg_h::va_list;
pub use self::stddef_h::{size_t, NULL};
use crate::celt::celt::celt_fatal;
use crate::celt::celt::CELT_SET_SIGNALLING_REQUEST;
use crate::celt::celt_decoder::{
    celt_decode_with_ec, celt_decoder_get_size, celt_decoder_init, OpusCustomDecoder,
};
use crate::celt::entcode::ec_tell;
use crate::celt::entdec::ec_dec;
use crate::celt::entdec::{ec_dec_bit_logp, ec_dec_init, ec_dec_uint};
use crate::celt::float_cast::FLOAT2INT16;
use crate::celt::modes::OpusCustomMode;
use crate::externs::memset;
use crate::silk::dec_API::silk_DecControlStruct;
use crate::silk::dec_API::{silk_Decode, silk_Get_Decoder_Size, silk_InitDecoder};
use crate::src::opus::opus_packet_parse_impl;
use crate::src::opus_private::{align, MODE_CELT_ONLY, MODE_HYBRID, MODE_SILK_ONLY};
use crate::varargs::VarArgs;
use crate::{opus_custom_decoder_ctl, opus_packet_get_samples_per_frame, opus_pcm_soft_clip};

#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "55:8"]
pub struct OpusDecoder {
    pub(crate) celt_dec_offset: i32,
    pub(crate) silk_dec_offset: i32,
    pub(crate) channels: i32,
    pub(crate) Fs: i32,
    pub(crate) DecControl: silk_DecControlStruct,
    pub(crate) decode_gain: i32,
    pub(crate) arch: i32,
    pub(crate) stream_channels: i32,
    pub(crate) bandwidth: i32,
    pub(crate) mode: i32,
    pub(crate) prev_mode: i32,
    pub(crate) frame_size: i32,
    pub(crate) prev_redundancy: i32,
    pub(crate) last_packet_duration: i32,
    pub(crate) softclip_mem: [opus_val16; 2],
    pub(crate) rangeFinal: u32,
}
#[c2rust::src_loc = "82:1"]
unsafe fn validate_opus_decoder(st: *mut OpusDecoder) {
    if !((*st).channels == 1 as i32 || (*st).channels == 2 as i32) {
        celt_fatal(
            b"assertion failed: st->channels == 1 || st->channels == 2\0" as *const u8 as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            84 as i32,
        );
    }
    if !((*st).Fs == 48000 as i32
        || (*st).Fs == 24000 as i32
        || (*st).Fs == 16000 as i32
        || (*st).Fs == 12000 as i32
        || (*st).Fs == 8000 as i32)
    {
        celt_fatal(
            b"assertion failed: st->Fs == 48000 || st->Fs == 24000 || st->Fs == 16000 || st->Fs == 12000 || st->Fs == 8000\0"
                as *const u8 as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            85 as i32,
        );
    }
    if !((*st).DecControl.API_sampleRate == (*st).Fs) {
        celt_fatal(
            b"assertion failed: st->DecControl.API_sampleRate == st->Fs\0" as *const u8
                as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            86 as i32,
        );
    }
    if !((*st).DecControl.internalSampleRate == 0 as i32
        || (*st).DecControl.internalSampleRate == 16000 as i32
        || (*st).DecControl.internalSampleRate == 12000 as i32
        || (*st).DecControl.internalSampleRate == 8000 as i32)
    {
        celt_fatal(
            b"assertion failed: st->DecControl.internalSampleRate == 0 || st->DecControl.internalSampleRate == 16000 || st->DecControl.internalSampleRate == 12000 || st->DecControl.internalSampleRate == 8000\0"
                as *const u8 as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            87 as i32,
        );
    }
    if !((*st).DecControl.nChannelsAPI == (*st).channels) {
        celt_fatal(
            b"assertion failed: st->DecControl.nChannelsAPI == st->channels\0" as *const u8
                as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            88 as i32,
        );
    }
    if !((*st).DecControl.nChannelsInternal == 0 as i32
        || (*st).DecControl.nChannelsInternal == 1 as i32
        || (*st).DecControl.nChannelsInternal == 2 as i32)
    {
        celt_fatal(
            b"assertion failed: st->DecControl.nChannelsInternal == 0 || st->DecControl.nChannelsInternal == 1 || st->DecControl.nChannelsInternal == 2\0"
                as *const u8 as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            89 as i32,
        );
    }
    if !((*st).DecControl.payloadSize_ms == 0 as i32
        || (*st).DecControl.payloadSize_ms == 10 as i32
        || (*st).DecControl.payloadSize_ms == 20 as i32
        || (*st).DecControl.payloadSize_ms == 40 as i32
        || (*st).DecControl.payloadSize_ms == 60 as i32)
    {
        celt_fatal(
            b"assertion failed: st->DecControl.payloadSize_ms == 0 || st->DecControl.payloadSize_ms == 10 || st->DecControl.payloadSize_ms == 20 || st->DecControl.payloadSize_ms == 40 || st->DecControl.payloadSize_ms == 60\0"
                as *const u8 as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            90 as i32,
        );
    }
    if !((*st).arch >= 0 as i32) {
        celt_fatal(
            b"assertion failed: st->arch >= 0\0" as *const u8 as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            92 as i32,
        );
    }
    if !((*st).arch <= 0 as i32) {
        celt_fatal(
            b"assertion failed: st->arch <= OPUS_ARCHMASK\0" as *const u8 as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            93 as i32,
        );
    }
    if !((*st).stream_channels == 1 as i32 || (*st).stream_channels == 2 as i32) {
        celt_fatal(
            b"assertion failed: st->stream_channels == 1 || st->stream_channels == 2\0" as *const u8
                as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            95 as i32,
        );
    }
}
#[c2rust::src_loc = "102:1"]
pub unsafe fn opus_decoder_get_size(channels: i32) -> i32 {
    let mut silkDecSizeBytes: i32 = 0;
    let mut celtDecSizeBytes: i32 = 0;
    let mut ret: i32 = 0;
    if channels < 1 as i32 || channels > 2 as i32 {
        return 0 as i32;
    }
    ret = silk_Get_Decoder_Size(&mut silkDecSizeBytes);
    if ret != 0 {
        return 0 as i32;
    }
    silkDecSizeBytes = align(silkDecSizeBytes);
    celtDecSizeBytes = celt_decoder_get_size(channels);
    return align(::core::mem::size_of::<OpusDecoder>() as u64 as i32)
        + silkDecSizeBytes
        + celtDecSizeBytes;
}
#[c2rust::src_loc = "116:1"]
pub unsafe fn opus_decoder_init(mut st: *mut OpusDecoder, Fs: i32, channels: i32) -> i32 {
    let mut silk_dec: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut celt_dec: *mut OpusCustomDecoder = 0 as *mut OpusCustomDecoder;
    let mut ret: i32 = 0;
    let mut silkDecSizeBytes: i32 = 0;
    if Fs != 48000 as i32
        && Fs != 24000 as i32
        && Fs != 16000 as i32
        && Fs != 12000 as i32
        && Fs != 8000 as i32
        || channels != 1 as i32 && channels != 2 as i32
    {
        return OPUS_BAD_ARG;
    }
    memset(
        st as *mut i8 as *mut core::ffi::c_void,
        0 as i32,
        (opus_decoder_get_size(channels) as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    ret = silk_Get_Decoder_Size(&mut silkDecSizeBytes);
    if ret != 0 {
        return OPUS_INTERNAL_ERROR;
    }
    silkDecSizeBytes = align(silkDecSizeBytes);
    (*st).silk_dec_offset = align(::core::mem::size_of::<OpusDecoder>() as u64 as i32);
    (*st).celt_dec_offset = (*st).silk_dec_offset + silkDecSizeBytes;
    silk_dec = (st as *mut i8).offset((*st).silk_dec_offset as isize) as *mut core::ffi::c_void;
    celt_dec = (st as *mut i8).offset((*st).celt_dec_offset as isize) as *mut OpusCustomDecoder;
    (*st).channels = channels;
    (*st).stream_channels = (*st).channels;
    (*st).Fs = Fs;
    (*st).DecControl.API_sampleRate = (*st).Fs;
    (*st).DecControl.nChannelsAPI = (*st).channels;
    ret = silk_InitDecoder(silk_dec);
    if ret != 0 {
        return OPUS_INTERNAL_ERROR;
    }
    ret = celt_decoder_init(celt_dec, Fs, channels);
    if ret != OPUS_OK {
        return OPUS_INTERNAL_ERROR;
    }
    opus_custom_decoder_ctl!(celt_dec, CELT_SET_SIGNALLING_REQUEST, 0 as i32);
    (*st).prev_mode = 0 as i32;
    (*st).frame_size = Fs / 400 as i32;
    (*st).arch = opus_select_arch();
    return OPUS_OK;
}
#[c2rust::src_loc = "159:1"]
pub unsafe fn opus_decoder_create(Fs: i32, channels: i32, error: *mut i32) -> *mut OpusDecoder {
    let mut ret: i32 = 0;
    let mut st: *mut OpusDecoder = 0 as *mut OpusDecoder;
    if Fs != 48000 as i32
        && Fs != 24000 as i32
        && Fs != 16000 as i32
        && Fs != 12000 as i32
        && Fs != 8000 as i32
        || channels != 1 as i32 && channels != 2 as i32
    {
        if !error.is_null() {
            *error = OPUS_BAD_ARG;
        }
        return NULL as *mut OpusDecoder;
    }
    st = malloc(opus_decoder_get_size(channels) as size_t) as *mut OpusDecoder;
    if st.is_null() {
        if !error.is_null() {
            *error = OPUS_ALLOC_FAIL;
        }
        return NULL as *mut OpusDecoder;
    }
    ret = opus_decoder_init(st, Fs, channels);
    if !error.is_null() {
        *error = ret;
    }
    if ret != OPUS_OK {
        free(st as *mut core::ffi::c_void);
        st = NULL as *mut OpusDecoder;
    }
    return st;
}
#[c2rust::src_loc = "188:1"]
unsafe fn smooth_fade(
    in1: *const opus_val16,
    in2: *const opus_val16,
    out: *mut opus_val16,
    overlap: i32,
    channels: i32,
    window: *const opus_val16,
    Fs: i32,
) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let inc: i32 = 48000 as i32 / Fs;
    c = 0 as i32;
    while c < channels {
        i = 0 as i32;
        while i < overlap {
            let w: opus_val16 =
                *window.offset((i * inc) as isize) * *window.offset((i * inc) as isize);
            *out.offset((i * channels + c) as isize) = w * *in2.offset((i * channels + c) as isize)
                + (1.0f32 - w) * *in1.offset((i * channels + c) as isize);
            i += 1;
        }
        c += 1;
    }
}
#[c2rust::src_loc = "205:1"]
unsafe fn opus_packet_get_mode(data: *const u8) -> i32 {
    let mut mode: i32 = 0;
    if *data.offset(0 as i32 as isize) as i32 & 0x80 as i32 != 0 {
        mode = MODE_CELT_ONLY;
    } else if *data.offset(0 as i32 as isize) as i32 & 0x60 as i32 == 0x60 as i32 {
        mode = MODE_HYBRID;
    } else {
        mode = MODE_SILK_ONLY;
    }
    return mode;
}
#[c2rust::src_loc = "220:1"]
unsafe fn opus_decode_frame(
    mut st: *mut OpusDecoder,
    mut data: *const u8,
    mut len: i32,
    mut pcm: *mut opus_val16,
    mut frame_size: i32,
    decode_fec: i32,
) -> i32 {
    let mut silk_dec: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut celt_dec: *mut OpusCustomDecoder = 0 as *mut OpusCustomDecoder;
    let mut i: i32 = 0;
    let mut silk_ret: i32 = 0 as i32;
    let mut celt_ret: i32 = 0 as i32;
    let mut dec: ec_dec = ec_dec {
        buf: 0 as *mut u8,
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
    let mut silk_frame_size: i32 = 0;
    let mut pcm_silk_size: i32 = 0;
    let mut pcm_transition_silk_size: i32 = 0;
    let mut pcm_transition_celt_size: i32 = 0;
    let mut pcm_transition: *mut opus_val16 = NULL as *mut opus_val16;
    let mut redundant_audio_size: i32 = 0;
    let mut audiosize: i32 = 0;
    let mut mode: i32 = 0;
    let mut bandwidth: i32 = 0;
    let mut transition: i32 = 0 as i32;
    let mut start_band: i32 = 0;
    let mut redundancy: i32 = 0 as i32;
    let mut redundancy_bytes: i32 = 0 as i32;
    let mut celt_to_silk: i32 = 0 as i32;
    let mut c: i32 = 0;
    let mut F2_5: i32 = 0;
    let mut F5: i32 = 0;
    let mut F10: i32 = 0;
    let mut F20: i32 = 0;
    let mut window: *const opus_val16 = 0 as *const opus_val16;
    let mut redundant_rng: u32 = 0 as i32 as u32;
    let mut celt_accum: i32 = 0;
    silk_dec = (st as *mut i8).offset((*st).silk_dec_offset as isize) as *mut core::ffi::c_void;
    celt_dec = (st as *mut i8).offset((*st).celt_dec_offset as isize) as *mut OpusCustomDecoder;
    F20 = (*st).Fs / 50 as i32;
    F10 = F20 >> 1 as i32;
    F5 = F10 >> 1 as i32;
    F2_5 = F5 >> 1 as i32;
    if frame_size < F2_5 {
        return OPUS_BUFFER_TOO_SMALL;
    }
    frame_size = if frame_size < (*st).Fs / 25 as i32 * 3 as i32 {
        frame_size
    } else {
        (*st).Fs / 25 as i32 * 3 as i32
    };
    if len <= 1 as i32 {
        data = NULL as *const u8;
        frame_size = if frame_size < (*st).frame_size {
            frame_size
        } else {
            (*st).frame_size
        };
    }
    if !data.is_null() {
        audiosize = (*st).frame_size;
        mode = (*st).mode;
        bandwidth = (*st).bandwidth;
        ec_dec_init(&mut dec, data as *mut u8, len as u32);
    } else {
        audiosize = frame_size;
        mode = (*st).prev_mode;
        bandwidth = 0 as i32;
        if mode == 0 as i32 {
            i = 0 as i32;
            while i < audiosize * (*st).channels {
                *pcm.offset(i as isize) = 0 as i32 as opus_val16;
                i += 1;
            }
            return audiosize;
        }
        if audiosize > F20 {
            loop {
                let ret: i32 = opus_decode_frame(
                    st,
                    NULL as *const u8,
                    0 as i32,
                    pcm,
                    if audiosize < F20 { audiosize } else { F20 },
                    0 as i32,
                );
                if ret < 0 as i32 {
                    return ret;
                }
                pcm = pcm.offset((ret * (*st).channels) as isize);
                audiosize -= ret;
                if !(audiosize > 0 as i32) {
                    break;
                }
            }
            return frame_size;
        } else {
            if audiosize < F20 {
                if audiosize > F10 {
                    audiosize = F10;
                } else if mode != MODE_SILK_ONLY && audiosize > F5 && audiosize < F10 {
                    audiosize = F5;
                }
            }
        }
    }
    celt_accum = 0 as i32;
    pcm_transition_silk_size = ALLOC_NONE;
    pcm_transition_celt_size = ALLOC_NONE;
    if !data.is_null()
        && (*st).prev_mode > 0 as i32
        && (mode == MODE_CELT_ONLY
            && (*st).prev_mode != MODE_CELT_ONLY
            && (*st).prev_redundancy == 0
            || mode != MODE_CELT_ONLY && (*st).prev_mode == MODE_CELT_ONLY)
    {
        transition = 1 as i32;
        if mode == MODE_CELT_ONLY {
            pcm_transition_celt_size = F5 * (*st).channels;
        } else {
            pcm_transition_silk_size = F5 * (*st).channels;
        }
    }
    let vla = pcm_transition_celt_size as usize;
    let mut pcm_transition_celt: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    if transition != 0 && mode == MODE_CELT_ONLY {
        pcm_transition = pcm_transition_celt.as_mut_ptr();
        opus_decode_frame(
            st,
            NULL as *const u8,
            0 as i32,
            pcm_transition,
            if F5 < audiosize { F5 } else { audiosize },
            0 as i32,
        );
    }
    if audiosize > frame_size {
        return OPUS_BAD_ARG;
    } else {
        frame_size = audiosize;
    }
    pcm_silk_size = if mode != MODE_CELT_ONLY && celt_accum == 0 {
        (if F10 > frame_size { F10 } else { frame_size }) * (*st).channels
    } else {
        ALLOC_NONE
    };
    let vla_0 = pcm_silk_size as usize;
    let mut pcm_silk: Vec<i16> = ::std::vec::from_elem(0, vla_0);
    if mode != MODE_CELT_ONLY {
        let mut lost_flag: i32 = 0;
        let mut decoded_samples: i32 = 0;
        let mut pcm_ptr: *mut i16 = 0 as *mut i16;
        pcm_ptr = pcm_silk.as_mut_ptr();
        if (*st).prev_mode == MODE_CELT_ONLY {
            silk_InitDecoder(silk_dec);
        }
        (*st).DecControl.payloadSize_ms = if 10 as i32 > 1000 as i32 * audiosize / (*st).Fs {
            10 as i32
        } else {
            1000 as i32 * audiosize / (*st).Fs
        };
        if !data.is_null() {
            (*st).DecControl.nChannelsInternal = (*st).stream_channels;
            if mode == MODE_SILK_ONLY {
                if bandwidth == OPUS_BANDWIDTH_NARROWBAND {
                    (*st).DecControl.internalSampleRate = 8000 as i32;
                } else if bandwidth == OPUS_BANDWIDTH_MEDIUMBAND {
                    (*st).DecControl.internalSampleRate = 12000 as i32;
                } else if bandwidth == OPUS_BANDWIDTH_WIDEBAND {
                    (*st).DecControl.internalSampleRate = 16000 as i32;
                } else {
                    (*st).DecControl.internalSampleRate = 16000 as i32;
                    if 0 as i32 == 0 {
                        celt_fatal(
                            b"assertion failed: 0\0" as *const u8 as *const i8,
                            b"src/opus_decoder.c\0" as *const u8 as *const i8,
                            389 as i32,
                        );
                    }
                }
            } else {
                (*st).DecControl.internalSampleRate = 16000 as i32;
            }
        }
        lost_flag = if data.is_null() {
            1 as i32
        } else {
            2 as i32 * decode_fec
        };
        decoded_samples = 0 as i32;
        loop {
            let first_frame: i32 = (decoded_samples == 0 as i32) as i32;
            silk_ret = silk_Decode(
                silk_dec,
                &mut (*st).DecControl,
                lost_flag,
                first_frame,
                &mut dec,
                pcm_ptr,
                &mut silk_frame_size,
                (*st).arch,
            );
            if silk_ret != 0 {
                if lost_flag != 0 {
                    silk_frame_size = frame_size;
                    i = 0 as i32;
                    while i < frame_size * (*st).channels {
                        *pcm_ptr.offset(i as isize) = 0 as i32 as i16;
                        i += 1;
                    }
                } else {
                    return OPUS_INTERNAL_ERROR;
                }
            }
            pcm_ptr = pcm_ptr.offset((silk_frame_size * (*st).channels) as isize);
            decoded_samples += silk_frame_size;
            if !(decoded_samples < frame_size) {
                break;
            }
        }
    }
    start_band = 0 as i32;
    if decode_fec == 0
        && mode != MODE_CELT_ONLY
        && !data.is_null()
        && ec_tell(&mut dec) + 17 as i32 + 20 as i32 * ((*st).mode == MODE_HYBRID) as i32
            <= 8 as i32 * len
    {
        if mode == MODE_HYBRID {
            redundancy = ec_dec_bit_logp(&mut dec, 12 as i32 as u32);
        } else {
            redundancy = 1 as i32;
        }
        if redundancy != 0 {
            celt_to_silk = ec_dec_bit_logp(&mut dec, 1 as i32 as u32);
            redundancy_bytes = if mode == MODE_HYBRID {
                ec_dec_uint(&mut dec, 256 as i32 as u32) as i32 + 2 as i32
            } else {
                len - (ec_tell(&mut dec) + 7 as i32 >> 3 as i32)
            };
            len -= redundancy_bytes;
            if (len * 8 as i32) < ec_tell(&mut dec) {
                len = 0 as i32;
                redundancy_bytes = 0 as i32;
                redundancy = 0 as i32;
            }
            dec.storage = (dec.storage as u32).wrapping_sub(redundancy_bytes as u32) as u32 as u32;
        }
    }
    if mode != MODE_CELT_ONLY {
        start_band = 17 as i32;
    }
    if redundancy != 0 {
        transition = 0 as i32;
        pcm_transition_silk_size = ALLOC_NONE;
    }
    let vla_1 = pcm_transition_silk_size as usize;
    let mut pcm_transition_silk: Vec<opus_val16> = ::std::vec::from_elem(0., vla_1);
    if transition != 0 && mode != MODE_CELT_ONLY {
        pcm_transition = pcm_transition_silk.as_mut_ptr();
        opus_decode_frame(
            st,
            NULL as *const u8,
            0 as i32,
            pcm_transition,
            if F5 < audiosize { F5 } else { audiosize },
            0 as i32,
        );
    }
    if bandwidth != 0 {
        let mut endband: i32 = 21 as i32;
        match bandwidth {
            OPUS_BANDWIDTH_NARROWBAND => {
                endband = 13 as i32;
            }
            OPUS_BANDWIDTH_MEDIUMBAND | OPUS_BANDWIDTH_WIDEBAND => {
                endband = 17 as i32;
            }
            OPUS_BANDWIDTH_SUPERWIDEBAND => {
                endband = 19 as i32;
            }
            OPUS_BANDWIDTH_FULLBAND => {
                endband = 21 as i32;
            }
            _ => {
                if 0 as i32 == 0 {
                    celt_fatal(
                        b"assertion failed: 0\0" as *const u8 as *const i8,
                        b"src/opus_decoder.c\0" as *const u8 as *const i8,
                        488 as i32,
                    );
                }
            }
        }
        if !(opus_custom_decoder_ctl!(celt_dec, 10012 as i32, endband) == 0 as i32) {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl!(celt_dec, 10012, (((void)((endband) == (i32)0)), (i32)(endband)))) == OPUS_OK\0"
                    as *const u8 as *const i8,
                b"src/opus_decoder.c\0" as *const u8 as *const i8,
                491 as i32,
            );
        }
    }
    if !(opus_custom_decoder_ctl!(celt_dec, 10008 as i32, (*st).stream_channels) == 0 as i32) {
        celt_fatal(
            b"assertion failed: (opus_custom_decoder_ctl!(celt_dec, 10008, (((void)((st->stream_channels) == (i32)0)), (i32)(st->stream_channels)))) == OPUS_OK\0"
                as *const u8 as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            493 as i32,
        );
    }
    redundant_audio_size = if redundancy != 0 {
        F5 * (*st).channels
    } else {
        ALLOC_NONE
    };
    let vla_2 = redundant_audio_size as usize;
    let mut redundant_audio: Vec<opus_val16> = ::std::vec::from_elem(0., vla_2);
    if redundancy != 0 && celt_to_silk != 0 {
        if !(opus_custom_decoder_ctl!(celt_dec, 10010 as i32, 0 as i32) == 0 as i32) {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl!(celt_dec, 10010, (((void)((0) == (i32)0)), (i32)(0)))) == OPUS_OK\0"
                    as *const u8 as *const i8,
                b"src/opus_decoder.c\0" as *const u8 as *const i8,
                502 as i32,
            );
        }
        celt_decode_with_ec(
            celt_dec,
            data.offset(len as isize),
            redundancy_bytes,
            redundant_audio.as_mut_ptr(),
            F5,
            NULL as *mut ec_dec,
            0 as i32,
        );
        if !(opus_custom_decoder_ctl!(
            celt_dec,
            4031 as i32,
            (&mut redundant_rng as *mut u32).offset(
                (&mut redundant_rng as *mut u32).offset_from(&mut redundant_rng as *mut u32) as i64
                    as isize,
            ),
        ) == 0 as i32)
        {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl!(celt_dec, 4031, ((&redundant_rng) + ((&redundant_rng) - (u32*)(&redundant_rng))))) == OPUS_OK\0"
                    as *const u8 as *const i8,
                b"src/opus_decoder.c\0" as *const u8 as *const i8,
                505 as i32,
            );
        }
    }
    if !(opus_custom_decoder_ctl!(celt_dec, 10010 as i32, start_band) == 0 as i32) {
        celt_fatal(
            b"assertion failed: (opus_custom_decoder_ctl!(celt_dec, 10010, (((void)((start_band) == (i32)0)), (i32)(start_band)))) == OPUS_OK\0"
                as *const u8 as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            509 as i32,
        );
    }
    if mode != MODE_SILK_ONLY {
        let celt_frame_size: i32 = if F20 < frame_size { F20 } else { frame_size };
        if mode != (*st).prev_mode && (*st).prev_mode > 0 as i32 && (*st).prev_redundancy == 0 {
            if !(opus_custom_decoder_ctl!(celt_dec, 4028 as i32) == 0 as i32) {
                celt_fatal(
                    b"assertion failed: (opus_custom_decoder_ctl!(celt_dec, 4028)) == OPUS_OK\0"
                        as *const u8 as *const i8,
                    b"src/opus_decoder.c\0" as *const u8 as *const i8,
                    516 as i32,
                );
            }
        }
        celt_ret = celt_decode_with_ec(
            celt_dec,
            if decode_fec != 0 {
                NULL as *const u8
            } else {
                data
            },
            len,
            pcm,
            celt_frame_size,
            &mut dec,
            celt_accum,
        );
    } else {
        let mut silence: [u8; 2] = [0xff as i32 as u8, 0xff as i32 as u8];
        if celt_accum == 0 {
            i = 0 as i32;
            while i < frame_size * (*st).channels {
                *pcm.offset(i as isize) = 0 as i32 as opus_val16;
                i += 1;
            }
        }
        if (*st).prev_mode == MODE_HYBRID
            && !(redundancy != 0 && celt_to_silk != 0 && (*st).prev_redundancy != 0)
        {
            if !(opus_custom_decoder_ctl!(celt_dec, 10010 as i32, 0 as i32) == 0 as i32) {
                celt_fatal(
                    b"assertion failed: (opus_custom_decoder_ctl!(celt_dec, 10010, (((void)((0) == (i32)0)), (i32)(0)))) == OPUS_OK\0"
                        as *const u8 as *const i8,
                    b"src/opus_decoder.c\0" as *const u8 as *const i8,
                    531 as i32,
                );
            }
            celt_decode_with_ec(
                celt_dec,
                silence.as_mut_ptr(),
                2 as i32,
                pcm,
                F2_5,
                NULL as *mut ec_dec,
                celt_accum,
            );
        }
    }
    if mode != MODE_CELT_ONLY && celt_accum == 0 {
        i = 0 as i32;
        while i < frame_size * (*st).channels {
            *pcm.offset(i as isize) = *pcm.offset(i as isize)
                + 1.0f32 / 32768.0f32 * *pcm_silk.as_mut_ptr().offset(i as isize) as i32 as f32;
            i += 1;
        }
    }
    let mut celt_mode: *const OpusCustomMode = 0 as *const OpusCustomMode;
    if !(opus_custom_decoder_ctl!(
        celt_dec,
        10015 as i32,
        (&mut celt_mode as *mut *const OpusCustomMode).offset(
            (&mut celt_mode as *mut *const OpusCustomMode)
                .offset_from(&mut celt_mode as *mut *const OpusCustomMode) as i64
                as isize,
        ),
    ) == 0 as i32)
    {
        celt_fatal(
            b"assertion failed: (opus_custom_decoder_ctl!(celt_dec, 10015, ((&celt_mode) + ((&celt_mode) - (const OpusCustomMode**)(&celt_mode))))) == OPUS_OK\0"
                as *const u8 as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            549 as i32,
        );
    }
    window = (*celt_mode).window;
    if redundancy != 0 && celt_to_silk == 0 {
        if !(opus_custom_decoder_ctl!(celt_dec, 4028 as i32) == 0 as i32) {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl!(celt_dec, 4028)) == OPUS_OK\0"
                    as *const u8 as *const i8,
                b"src/opus_decoder.c\0" as *const u8 as *const i8,
                556 as i32,
            );
        }
        if !(opus_custom_decoder_ctl!(celt_dec, 10010 as i32, 0 as i32) == 0 as i32) {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl!(celt_dec, 10010, (((void)((0) == (i32)0)), (i32)(0)))) == OPUS_OK\0"
                    as *const u8 as *const i8,
                b"src/opus_decoder.c\0" as *const u8 as *const i8,
                557 as i32,
            );
        }
        celt_decode_with_ec(
            celt_dec,
            data.offset(len as isize),
            redundancy_bytes,
            redundant_audio.as_mut_ptr(),
            F5,
            NULL as *mut ec_dec,
            0 as i32,
        );
        if !(opus_custom_decoder_ctl!(
            celt_dec,
            4031 as i32,
            (&mut redundant_rng as *mut u32).offset(
                (&mut redundant_rng as *mut u32).offset_from(&mut redundant_rng as *mut u32) as i64
                    as isize,
            ),
        ) == 0 as i32)
        {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl!(celt_dec, 4031, ((&redundant_rng) + ((&redundant_rng) - (u32*)(&redundant_rng))))) == OPUS_OK\0"
                    as *const u8 as *const i8,
                b"src/opus_decoder.c\0" as *const u8 as *const i8,
                560 as i32,
            );
        }
        smooth_fade(
            pcm.offset(((*st).channels * (frame_size - F2_5)) as isize),
            redundant_audio
                .as_mut_ptr()
                .offset(((*st).channels * F2_5) as isize),
            pcm.offset(((*st).channels * (frame_size - F2_5)) as isize),
            F2_5,
            (*st).channels,
            window,
            (*st).Fs,
        );
    }
    if redundancy != 0 && celt_to_silk != 0 {
        c = 0 as i32;
        while c < (*st).channels {
            i = 0 as i32;
            while i < F2_5 {
                *pcm.offset(((*st).channels * i + c) as isize) = *redundant_audio
                    .as_mut_ptr()
                    .offset(((*st).channels * i + c) as isize);
                i += 1;
            }
            c += 1;
        }
        smooth_fade(
            redundant_audio
                .as_mut_ptr()
                .offset(((*st).channels * F2_5) as isize),
            pcm.offset(((*st).channels * F2_5) as isize),
            pcm.offset(((*st).channels * F2_5) as isize),
            F2_5,
            (*st).channels,
            window,
            (*st).Fs,
        );
    }
    if transition != 0 {
        if audiosize >= F5 {
            i = 0 as i32;
            while i < (*st).channels * F2_5 {
                *pcm.offset(i as isize) = *pcm_transition.offset(i as isize);
                i += 1;
            }
            smooth_fade(
                pcm_transition.offset(((*st).channels * F2_5) as isize),
                pcm.offset(((*st).channels * F2_5) as isize),
                pcm.offset(((*st).channels * F2_5) as isize),
                F2_5,
                (*st).channels,
                window,
                (*st).Fs,
            );
        } else {
            smooth_fade(
                pcm_transition,
                pcm,
                pcm,
                F2_5,
                (*st).channels,
                window,
                (*st).Fs,
            );
        }
    }
    if (*st).decode_gain != 0 {
        let mut gain: opus_val32 = 0.;
        gain = (std::f32::consts::LN_2 * (6.48814081e-4f32 * (*st).decode_gain as f32)).exp();
        i = 0 as i32;
        while i < frame_size * (*st).channels {
            let mut x: opus_val32 = 0.;
            x = *pcm.offset(i as isize) * gain;
            *pcm.offset(i as isize) = x;
            i += 1;
        }
    }
    if len <= 1 as i32 {
        (*st).rangeFinal = 0 as i32 as u32;
    } else {
        (*st).rangeFinal = dec.rng ^ redundant_rng;
    }
    (*st).prev_mode = mode;
    (*st).prev_redundancy = (redundancy != 0 && celt_to_silk == 0) as i32;
    if celt_ret >= 0 as i32 {
        let _ = _opus_false() != 0;
    }
    return if celt_ret < 0 as i32 {
        celt_ret
    } else {
        audiosize
    };
}
#[c2rust::src_loc = "626:1"]
pub unsafe fn opus_decode_native(
    mut st: *mut OpusDecoder,
    mut data: *const u8,
    len: i32,
    pcm: *mut opus_val16,
    frame_size: i32,
    decode_fec: i32,
    self_delimited: i32,
    packet_offset: *mut i32,
    soft_clip: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut nb_samples: i32 = 0;
    let mut count: i32 = 0;
    let mut offset: i32 = 0;
    let mut toc: u8 = 0;
    let mut packet_frame_size: i32 = 0;
    let mut packet_bandwidth: i32 = 0;
    let mut packet_mode: i32 = 0;
    let mut packet_stream_channels: i32 = 0;
    let mut size: [i16; 48] = [0; 48];
    validate_opus_decoder(st);
    if decode_fec < 0 as i32 || decode_fec > 1 as i32 {
        return OPUS_BAD_ARG;
    }
    if (decode_fec != 0 || len == 0 as i32 || data.is_null())
        && frame_size % ((*st).Fs / 400 as i32) != 0 as i32
    {
        return OPUS_BAD_ARG;
    }
    if len == 0 as i32 || data.is_null() {
        let mut pcm_count: i32 = 0 as i32;
        loop {
            let mut ret: i32 = 0;
            ret = opus_decode_frame(
                st,
                NULL as *const u8,
                0 as i32,
                pcm.offset((pcm_count * (*st).channels) as isize),
                frame_size - pcm_count,
                0 as i32,
            );
            if ret < 0 as i32 {
                return ret;
            }
            pcm_count += ret;
            if !(pcm_count < frame_size) {
                break;
            }
        }
        if !(pcm_count == frame_size) {
            celt_fatal(
                b"assertion failed: pcm_count == frame_size\0" as *const u8 as *const i8,
                b"src/opus_decoder.c\0" as *const u8 as *const i8,
                652 as i32,
            );
        }
        let _ = _opus_false() != 0;
        (*st).last_packet_duration = pcm_count;
        return pcm_count;
    } else {
        if len < 0 as i32 {
            return OPUS_BAD_ARG;
        }
    }
    packet_mode = opus_packet_get_mode(data);
    packet_bandwidth = opus_packet_get_bandwidth(data);
    packet_frame_size = opus_packet_get_samples_per_frame(data, (*st).Fs);
    packet_stream_channels = opus_packet_get_nb_channels(data);
    count = opus_packet_parse_impl(
        data,
        len,
        self_delimited,
        &mut toc,
        NULL as *mut *const u8,
        size.as_mut_ptr(),
        &mut offset,
        packet_offset,
    );
    if count < 0 as i32 {
        return count;
    }
    data = data.offset(offset as isize);
    if decode_fec != 0 {
        let mut duration_copy: i32 = 0;
        let mut ret_0: i32 = 0;
        if frame_size < packet_frame_size
            || packet_mode == MODE_CELT_ONLY
            || (*st).mode == MODE_CELT_ONLY
        {
            return opus_decode_native(
                st,
                NULL as *const u8,
                0 as i32,
                pcm,
                frame_size,
                0 as i32,
                0 as i32,
                NULL as *mut i32,
                soft_clip,
            );
        }
        duration_copy = (*st).last_packet_duration;
        if frame_size - packet_frame_size != 0 as i32 {
            ret_0 = opus_decode_native(
                st,
                NULL as *const u8,
                0 as i32,
                pcm,
                frame_size - packet_frame_size,
                0 as i32,
                0 as i32,
                NULL as *mut i32,
                soft_clip,
            );
            if ret_0 < 0 as i32 {
                (*st).last_packet_duration = duration_copy;
                return ret_0;
            }
            if !(ret_0 == frame_size - packet_frame_size) {
                celt_fatal(
                    b"assertion failed: ret==frame_size-packet_frame_size\0" as *const u8
                        as *const i8,
                    b"src/opus_decoder.c\0" as *const u8 as *const i8,
                    689 as i32,
                );
            }
        }
        (*st).mode = packet_mode;
        (*st).bandwidth = packet_bandwidth;
        (*st).frame_size = packet_frame_size;
        (*st).stream_channels = packet_stream_channels;
        ret_0 = opus_decode_frame(
            st,
            data,
            size[0 as i32 as usize] as i32,
            pcm.offset(((*st).channels * (frame_size - packet_frame_size)) as isize),
            packet_frame_size,
            1 as i32,
        );
        if ret_0 < 0 as i32 {
            return ret_0;
        } else {
            let _ = _opus_false() != 0;
            (*st).last_packet_duration = frame_size;
            return frame_size;
        }
    }
    if count * packet_frame_size > frame_size {
        return OPUS_BUFFER_TOO_SMALL;
    }
    (*st).mode = packet_mode;
    (*st).bandwidth = packet_bandwidth;
    (*st).frame_size = packet_frame_size;
    (*st).stream_channels = packet_stream_channels;
    nb_samples = 0 as i32;
    i = 0 as i32;
    while i < count {
        let mut ret_1: i32 = 0;
        ret_1 = opus_decode_frame(
            st,
            data,
            size[i as usize] as i32,
            pcm.offset((nb_samples * (*st).channels) as isize),
            frame_size - nb_samples,
            0 as i32,
        );
        if ret_1 < 0 as i32 {
            return ret_1;
        }
        if !(ret_1 == packet_frame_size) {
            celt_fatal(
                b"assertion failed: ret==packet_frame_size\0" as *const u8 as *const i8,
                b"src/opus_decoder.c\0" as *const u8 as *const i8,
                724 as i32,
            );
        }
        data = data.offset(size[i as usize] as i32 as isize);
        nb_samples += ret_1;
        i += 1;
    }
    (*st).last_packet_duration = nb_samples;
    let _ = _opus_false() != 0;
    if soft_clip != 0 {
        opus_pcm_soft_clip(
            pcm,
            nb_samples,
            (*st).channels,
            ((*st).softclip_mem).as_mut_ptr(),
        );
    } else {
        (*st).softclip_mem[1 as i32 as usize] = 0 as i32 as opus_val16;
        (*st).softclip_mem[0 as i32 as usize] = (*st).softclip_mem[1 as i32 as usize];
    }
    return nb_samples;
}
#[c2rust::src_loc = "788:1"]
pub unsafe fn opus_decode(
    st: *mut OpusDecoder,
    data: *const u8,
    len: i32,
    pcm: *mut i16,
    mut frame_size: i32,
    decode_fec: i32,
) -> i32 {
    let mut ret: i32 = 0;
    let mut i: i32 = 0;
    let mut nb_samples: i32 = 0;
    if frame_size <= 0 as i32 {
        return OPUS_BAD_ARG;
    }
    if !data.is_null() && len > 0 as i32 && decode_fec == 0 {
        nb_samples = opus_decoder_get_nb_samples(st, data, len);
        if nb_samples > 0 as i32 {
            frame_size = if frame_size < nb_samples {
                frame_size
            } else {
                nb_samples
            };
        } else {
            return OPUS_INVALID_PACKET;
        }
    }
    if !((*st).channels == 1 as i32 || (*st).channels == 2 as i32) {
        celt_fatal(
            b"assertion failed: st->channels == 1 || st->channels == 2\0" as *const u8 as *const i8,
            b"src/opus_decoder.c\0" as *const u8 as *const i8,
            810 as i32,
        );
    }
    let vla = (frame_size * (*st).channels) as usize;
    let mut out: Vec<f32> = ::std::vec::from_elem(0., vla);
    ret = opus_decode_native(
        st,
        data,
        len,
        out.as_mut_ptr(),
        frame_size,
        decode_fec,
        0 as i32,
        NULL as *mut i32,
        1 as i32,
    );
    if ret > 0 as i32 {
        i = 0 as i32;
        while i < ret * (*st).channels {
            *pcm.offset(i as isize) = FLOAT2INT16(*out.as_mut_ptr().offset(i as isize));
            i += 1;
        }
    }
    return ret;
}
#[c2rust::src_loc = "823:1"]
pub unsafe fn opus_decode_float(
    st: *mut OpusDecoder,
    data: *const u8,
    len: i32,
    pcm: *mut opus_val16,
    frame_size: i32,
    decode_fec: i32,
) -> i32 {
    if frame_size <= 0 as i32 {
        return OPUS_BAD_ARG;
    }
    return opus_decode_native(
        st,
        data,
        len,
        pcm,
        frame_size,
        decode_fec,
        0 as i32,
        NULL as *mut i32,
        0 as i32,
    );
}
#[c2rust::src_loc = "833:1"]
pub unsafe fn opus_decoder_ctl_impl(mut st: *mut OpusDecoder, request: i32, args: VarArgs) -> i32 {
    let mut silk_dec: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut celt_dec: *mut OpusCustomDecoder = 0 as *mut OpusCustomDecoder;
    silk_dec = (st as *mut i8).offset((*st).silk_dec_offset as isize) as *mut core::ffi::c_void;
    celt_dec = (st as *mut i8).offset((*st).celt_dec_offset as isize) as *mut OpusCustomDecoder;

    let mut ap = args;

    match request {
        OPUS_GET_BANDWIDTH_REQUEST => {
            let value: *mut i32 = ap.arg::<*mut i32>();
            if value.is_null() {
                OPUS_BAD_ARG
            } else {
                *value = (*st).bandwidth;
                OPUS_OK
            }
        }
        OPUS_GET_FINAL_RANGE_REQUEST => {
            let value_0: *mut u32 = ap.arg::<*mut u32>();
            if value_0.is_null() {
                OPUS_BAD_ARG
            } else {
                *value_0 = (*st).rangeFinal;
                OPUS_OK
            }
        }
        OPUS_RESET_STATE => {
            memset(
                &mut (*st).stream_channels as *mut i32 as *mut i8 as *mut core::ffi::c_void,
                0 as i32,
                (::core::mem::size_of::<OpusDecoder>() as u64)
                    .wrapping_sub(
                        (&mut (*st).stream_channels as *mut i32 as *mut i8)
                            .offset_from(st as *mut i8) as i64 as u64,
                    )
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            opus_custom_decoder_ctl!(celt_dec, OPUS_RESET_STATE);
            silk_InitDecoder(silk_dec);
            (*st).stream_channels = (*st).channels;
            (*st).frame_size = (*st).Fs / 400 as i32;
            OPUS_OK
        }
        OPUS_GET_SAMPLE_RATE_REQUEST => {
            let value_1: *mut i32 = ap.arg::<*mut i32>();
            if value_1.is_null() {
                OPUS_BAD_ARG
            } else {
                *value_1 = (*st).Fs;
                OPUS_OK
            }
        }
        OPUS_GET_PITCH_REQUEST => {
            let value_2: *mut i32 = ap.arg::<*mut i32>();
            if value_2.is_null() {
                OPUS_BAD_ARG
            } else {
                if (*st).prev_mode == MODE_CELT_ONLY {
                    opus_custom_decoder_ctl!(
                        celt_dec,
                        OPUS_GET_PITCH_REQUEST,
                        value_2.offset(value_2.offset_from(value_2) as i64 as isize),
                    )
                } else {
                    *value_2 = (*st).DecControl.prevPitchLag;
                    OPUS_OK
                }
            }
        }
        OPUS_GET_GAIN_REQUEST => {
            let value_3: *mut i32 = ap.arg::<*mut i32>();
            if value_3.is_null() {
                OPUS_BAD_ARG
            } else {
                *value_3 = (*st).decode_gain;
                OPUS_OK
            }
        }
        OPUS_SET_GAIN_REQUEST => {
            let value_4: i32 = ap.arg::<i32>();
            if value_4 < -(32768 as i32) || value_4 > 32767 as i32 {
                OPUS_BAD_ARG
            } else {
                (*st).decode_gain = value_4;
                OPUS_OK
            }
        }
        OPUS_GET_LAST_PACKET_DURATION_REQUEST => {
            let value_5: *mut i32 = ap.arg::<*mut i32>();
            if value_5.is_null() {
                OPUS_BAD_ARG
            } else {
                *value_5 = (*st).last_packet_duration;
                OPUS_OK
            }
        }
        OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST => {
            let value_6: i32 = ap.arg::<i32>();
            if value_6 < 0 as i32 || value_6 > 1 as i32 {
                OPUS_BAD_ARG
            } else {
                opus_custom_decoder_ctl!(
                    celt_dec,
                    OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST,
                    value_6,
                )
            }
        }
        OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST => {
            let value_7: *mut i32 = ap.arg::<*mut i32>();
            if value_7.is_null() {
                OPUS_BAD_ARG
            } else {
                opus_custom_decoder_ctl!(
                    celt_dec,
                    OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST,
                    value_7.offset(value_7.offset_from(value_7) as i64 as isize),
                )
            }
        }
        _ => OPUS_UNIMPLEMENTED,
    }
}
#[macro_export]
macro_rules! opus_decoder_ctl {
    ($st:expr,$request:expr, $($arg:expr),*) => {
        $crate::opus_decoder_ctl_impl(
            $st,
            $request,
            $crate::varargs!($($arg),*)
        )
    };
    ($st:expr,$request:expr, $($arg:expr),*,) => {
        opus_decoder_ctl!($st, $request, $($arg),*)
    };
    ($st:expr,$request:expr) => {
        opus_decoder_ctl!($st, $request,)
    };
}
#[c2rust::src_loc = "966:1"]
pub unsafe fn opus_decoder_destroy(st: *mut OpusDecoder) {
    free(st as *mut core::ffi::c_void);
}
#[c2rust::src_loc = "972:1"]
pub unsafe fn opus_packet_get_bandwidth(data: *const u8) -> i32 {
    let mut bandwidth: i32 = 0;
    if *data.offset(0 as i32 as isize) as i32 & 0x80 as i32 != 0 {
        bandwidth = OPUS_BANDWIDTH_MEDIUMBAND
            + (*data.offset(0 as i32 as isize) as i32 >> 5 as i32 & 0x3 as i32);
        if bandwidth == OPUS_BANDWIDTH_MEDIUMBAND {
            bandwidth = OPUS_BANDWIDTH_NARROWBAND;
        }
    } else if *data.offset(0 as i32 as isize) as i32 & 0x60 as i32 == 0x60 as i32 {
        bandwidth = if *data.offset(0 as i32 as isize) as i32 & 0x10 as i32 != 0 {
            OPUS_BANDWIDTH_FULLBAND
        } else {
            OPUS_BANDWIDTH_SUPERWIDEBAND
        };
    } else {
        bandwidth = OPUS_BANDWIDTH_NARROWBAND
            + (*data.offset(0 as i32 as isize) as i32 >> 5 as i32 & 0x3 as i32);
    }
    return bandwidth;
}
#[c2rust::src_loc = "990:1"]
pub unsafe fn opus_packet_get_nb_channels(data: *const u8) -> i32 {
    return if *data.offset(0 as i32 as isize) as i32 & 0x4 as i32 != 0 {
        2 as i32
    } else {
        1 as i32
    };
}
#[c2rust::src_loc = "995:1"]
pub unsafe fn opus_packet_get_nb_frames(packet: *const u8, len: i32) -> i32 {
    let mut count: i32 = 0;
    if len < 1 as i32 {
        return OPUS_BAD_ARG;
    }
    count = *packet.offset(0 as i32 as isize) as i32 & 0x3 as i32;
    if count == 0 as i32 {
        return 1 as i32;
    } else if count != 3 as i32 {
        return 2 as i32;
    } else if len < 2 as i32 {
        return OPUS_INVALID_PACKET;
    } else {
        return *packet.offset(1 as i32 as isize) as i32 & 0x3f as i32;
    };
}
#[c2rust::src_loc = "1011:1"]
pub unsafe fn opus_packet_get_nb_samples(packet: *const u8, len: i32, Fs: i32) -> i32 {
    let mut samples: i32 = 0;
    let count: i32 = opus_packet_get_nb_frames(packet, len);
    if count < 0 as i32 {
        return count;
    }
    samples = count * opus_packet_get_samples_per_frame(packet, Fs);
    if samples * 25 as i32 > Fs * 3 as i32 {
        return OPUS_INVALID_PACKET;
    } else {
        return samples;
    };
}
#[c2rust::src_loc = "1028:1"]
pub unsafe fn opus_decoder_get_nb_samples(
    dec: *const OpusDecoder,
    packet: *const u8,
    len: i32,
) -> i32 {
    return opus_packet_get_nb_samples(packet, len, (*dec).Fs);
}
