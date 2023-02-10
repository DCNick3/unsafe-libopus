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
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stdarg.h:40"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/types.h:41"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:41"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:41"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:41"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_custom.h:41"]
pub mod opus_custom_h {
    extern "C" {
        #[c2rust::src_loc = "102:16"]
        pub type OpusCustomDecoder;
        #[c2rust::src_loc = "334:20"]
        pub fn opus_custom_decoder_ctl(
            st: *mut OpusCustomDecoder,
            request: libc::c_int,
            _: ...
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/modes.h:44"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mdct.h:44"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/kiss_fft.h:44"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:41"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:41"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:41"]
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
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/control.h:45"]
pub mod control_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:9"]
    pub struct silk_DecControlStruct {
        pub nChannelsAPI: opus_int32,
        pub nChannelsInternal: opus_int32,
        pub API_sampleRate: opus_int32,
        pub internalSampleRate: opus_int32,
        pub payloadSize_ms: libc::c_int,
        pub prevPitchLag: libc::c_int,
    }
    use super::opus_types_h::opus_int32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/opus_private.h:48"]
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
        #[c2rust::src_loc = "165:1"]
        pub fn opus_packet_parse_impl(
            data: *const libc::c_uchar,
            len: opus_int32,
            self_delimited: libc::c_int,
            out_toc: *mut libc::c_uchar,
            frames: *mut *const libc::c_uchar,
            size: *mut opus_int16,
            payload_offset: *mut libc::c_int,
            packet_offset: *mut opus_int32,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/xmmintrin.h:47"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:41"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "95:17"]
        pub fn exp(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/celt.h:41"]
pub mod celt_h {
    use super::arch_h::opus_val16;
    use super::entcode_h::ec_dec;
    use super::opus_custom_h::OpusCustomDecoder;
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "152:1"]
        pub fn celt_decode_with_ec(
            st: *mut OpusCustomDecoder,
            data: *const libc::c_uchar,
            len: libc::c_int,
            pcm: *mut opus_val16,
            frame_size: libc::c_int,
            dec: *mut ec_dec,
            accum: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "147:1"]
        pub fn celt_decoder_get_size(channels: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "150:1"]
        pub fn celt_decoder_init(
            st: *mut OpusCustomDecoder,
            sampling_rate: opus_int32,
            channels: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.h:41"]
pub mod entdec_h {
    use super::entcode_h::ec_dec;
    use super::opus_types_h::opus_uint32;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn ec_dec_init(_this: *mut ec_dec, _buf: *mut libc::c_uchar, _storage: opus_uint32);
        #[c2rust::src_loc = "72:1"]
        pub fn ec_dec_bit_logp(_this: *mut ec_dec, _logp: libc::c_uint) -> libc::c_int;
        #[c2rust::src_loc = "90:1"]
        pub fn ec_dec_uint(_this: *mut ec_dec, _ft: opus_uint32) -> opus_uint32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus.h:42"]
pub mod opus_h {
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "556:1"]
        pub fn opus_packet_get_samples_per_frame(
            data: *const libc::c_uchar,
            Fs: opus_int32,
        ) -> libc::c_int;
        #[c2rust::src_loc = "606:1"]
        pub fn opus_pcm_soft_clip(
            pcm: *mut libc::c_float,
            frame_size: libc::c_int,
            channels: libc::c_int,
            softclip_mem: *mut libc::c_float,
        );
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:44"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cpu_support.h:44"]
pub mod cpu_support_h {
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn opus_select_arch() -> libc::c_int {
        return 0 as libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/API.h:45"]
pub mod API_h {
    use super::control_h::silk_DecControlStruct;
    use super::entcode_h::ec_dec;
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "94:1"]
        pub fn silk_Get_Decoder_Size(decSizeBytes: *mut libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "101:1"]
        pub fn silk_InitDecoder(decState: *mut libc::c_void) -> libc::c_int;
        #[c2rust::src_loc = "108:1"]
        pub fn silk_Decode(
            decState: *mut libc::c_void,
            decControl: *mut silk_DecControlStruct,
            lostFlag: libc::c_int,
            newPacketFlag: libc::c_int,
            psRangeDec: *mut ec_dec,
            samplesOut: *mut opus_int16,
            nSamplesOut: *mut opus_int32,
            arch: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/stack_alloc.h:46"]
pub mod stack_alloc_h {
    #[inline]
    #[c2rust::src_loc = "175:1"]
    pub unsafe extern "C" fn _opus_false() -> libc::c_int {
        return 0 as libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/float_cast.h:47"]
pub mod float_cast_h {
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
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn float2int(mut x: libc::c_float) -> opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }
    use super::opus_types_h::{opus_int16, opus_int32};
    use super::xmmintrin_h::{_mm_cvt_ss2si, _mm_set_ss};
}
#[c2rust::header_src = "/usr/include/string.h:49"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/os_support.h:49"]
pub mod os_support_h {
    #[inline]
    #[c2rust::src_loc = "47:1"]
    pub unsafe extern "C" fn opus_alloc(mut size: size_t) -> *mut libc::c_void {
        return malloc(size);
    }
    #[inline]
    #[c2rust::src_loc = "64:1"]
    pub unsafe extern "C" fn opus_free(mut ptr: *mut libc::c_void) {
        free(ptr);
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::{free, malloc};
}
pub use self::arch_h::{celt_fatal, opus_val16, opus_val32};
use self::celt_h::{celt_decode_with_ec, celt_decoder_get_size, celt_decoder_init};
pub use self::control_h::silk_DecControlStruct;
pub use self::cpu_support_h::opus_select_arch;
pub use self::entcode_h::{ec_ctx, ec_dec, ec_tell, ec_window};
use self::entdec_h::{ec_dec_bit_logp, ec_dec_init, ec_dec_uint};
pub use self::float_cast_h::{float2int, FLOAT2INT16};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::kiss_fft_h::{arch_fft_state, kiss_fft_state, kiss_twiddle_cpx};
use self::mathcalls_h::exp;
pub use self::mdct_h::mdct_lookup;
pub use self::modes_h::{OpusCustomMode, PulseCache};
use self::opus_custom_h::{opus_custom_decoder_ctl, OpusCustomDecoder};
use self::opus_h::{opus_packet_get_samples_per_frame, opus_pcm_soft_clip};
pub use self::opus_private_h::{align, foo, opus_packet_parse_impl, C2RustUnnamed};
pub use self::opus_types_h::{opus_int16, opus_int32, opus_uint32};
pub use self::os_support_h::{opus_alloc, opus_free};
pub use self::stack_alloc_h::_opus_false;
pub use self::stdarg_h::va_list;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint32_t;
use self::stdlib_h::{free, malloc};
use self::string_h::memset;
pub use self::types_h::{__int16_t, __int32_t, __uint32_t};
use self::API_h::{silk_Decode, silk_Get_Decoder_Size, silk_InitDecoder};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "55:8"]
pub struct OpusDecoder {
    pub celt_dec_offset: libc::c_int,
    pub silk_dec_offset: libc::c_int,
    pub channels: libc::c_int,
    pub Fs: opus_int32,
    pub DecControl: silk_DecControlStruct,
    pub decode_gain: libc::c_int,
    pub arch: libc::c_int,
    pub stream_channels: libc::c_int,
    pub bandwidth: libc::c_int,
    pub mode: libc::c_int,
    pub prev_mode: libc::c_int,
    pub frame_size: libc::c_int,
    pub prev_redundancy: libc::c_int,
    pub last_packet_duration: libc::c_int,
    pub softclip_mem: [opus_val16; 2],
    pub rangeFinal: opus_uint32,
}
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn validate_opus_decoder(mut st: *mut OpusDecoder) {
    if !((*st).channels == 1 as libc::c_int || (*st).channels == 2 as libc::c_int) {
        celt_fatal(
            b"assertion failed: st->channels == 1 || st->channels == 2\0" as *const u8
                as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
        );
    }
    if !((*st).Fs == 48000 as libc::c_int
        || (*st).Fs == 24000 as libc::c_int
        || (*st).Fs == 16000 as libc::c_int
        || (*st).Fs == 12000 as libc::c_int
        || (*st).Fs == 8000 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: st->Fs == 48000 || st->Fs == 24000 || st->Fs == 16000 || st->Fs == 12000 || st->Fs == 8000\0"
                as *const u8 as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
        );
    }
    if !((*st).DecControl.API_sampleRate == (*st).Fs) {
        celt_fatal(
            b"assertion failed: st->DecControl.API_sampleRate == st->Fs\0" as *const u8
                as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
        );
    }
    if !((*st).DecControl.internalSampleRate == 0 as libc::c_int
        || (*st).DecControl.internalSampleRate == 16000 as libc::c_int
        || (*st).DecControl.internalSampleRate == 12000 as libc::c_int
        || (*st).DecControl.internalSampleRate == 8000 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: st->DecControl.internalSampleRate == 0 || st->DecControl.internalSampleRate == 16000 || st->DecControl.internalSampleRate == 12000 || st->DecControl.internalSampleRate == 8000\0"
                as *const u8 as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
        );
    }
    if !((*st).DecControl.nChannelsAPI == (*st).channels) {
        celt_fatal(
            b"assertion failed: st->DecControl.nChannelsAPI == st->channels\0" as *const u8
                as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
        );
    }
    if !((*st).DecControl.nChannelsInternal == 0 as libc::c_int
        || (*st).DecControl.nChannelsInternal == 1 as libc::c_int
        || (*st).DecControl.nChannelsInternal == 2 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: st->DecControl.nChannelsInternal == 0 || st->DecControl.nChannelsInternal == 1 || st->DecControl.nChannelsInternal == 2\0"
                as *const u8 as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
        );
    }
    if !((*st).DecControl.payloadSize_ms == 0 as libc::c_int
        || (*st).DecControl.payloadSize_ms == 10 as libc::c_int
        || (*st).DecControl.payloadSize_ms == 20 as libc::c_int
        || (*st).DecControl.payloadSize_ms == 40 as libc::c_int
        || (*st).DecControl.payloadSize_ms == 60 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: st->DecControl.payloadSize_ms == 0 || st->DecControl.payloadSize_ms == 10 || st->DecControl.payloadSize_ms == 20 || st->DecControl.payloadSize_ms == 40 || st->DecControl.payloadSize_ms == 60\0"
                as *const u8 as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
        );
    }
    if !((*st).arch >= 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: st->arch >= 0\0" as *const u8 as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
        );
    }
    if !((*st).arch <= 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: st->arch <= OPUS_ARCHMASK\0" as *const u8 as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
        );
    }
    if !((*st).stream_channels == 1 as libc::c_int || (*st).stream_channels == 2 as libc::c_int) {
        celt_fatal(
            b"assertion failed: st->stream_channels == 1 || st->stream_channels == 2\0" as *const u8
                as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "102:1"]
pub unsafe extern "C" fn opus_decoder_get_size(mut channels: libc::c_int) -> libc::c_int {
    let mut silkDecSizeBytes: libc::c_int = 0;
    let mut celtDecSizeBytes: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if channels < 1 as libc::c_int || channels > 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    ret = silk_Get_Decoder_Size(&mut silkDecSizeBytes);
    if ret != 0 {
        return 0 as libc::c_int;
    }
    silkDecSizeBytes = align(silkDecSizeBytes);
    celtDecSizeBytes = celt_decoder_get_size(channels);
    return align(::core::mem::size_of::<OpusDecoder>() as libc::c_ulong as libc::c_int)
        + silkDecSizeBytes
        + celtDecSizeBytes;
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn opus_decoder_init(
    mut st: *mut OpusDecoder,
    mut Fs: opus_int32,
    mut channels: libc::c_int,
) -> libc::c_int {
    let mut silk_dec: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut celt_dec: *mut OpusCustomDecoder = 0 as *mut OpusCustomDecoder;
    let mut ret: libc::c_int = 0;
    let mut silkDecSizeBytes: libc::c_int = 0;
    if Fs != 48000 as libc::c_int
        && Fs != 24000 as libc::c_int
        && Fs != 16000 as libc::c_int
        && Fs != 12000 as libc::c_int
        && Fs != 8000 as libc::c_int
        || channels != 1 as libc::c_int && channels != 2 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    memset(
        st as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
        (opus_decoder_get_size(channels) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    ret = silk_Get_Decoder_Size(&mut silkDecSizeBytes);
    if ret != 0 {
        return -(3 as libc::c_int);
    }
    silkDecSizeBytes = align(silkDecSizeBytes);
    (*st).silk_dec_offset =
        align(::core::mem::size_of::<OpusDecoder>() as libc::c_ulong as libc::c_int);
    (*st).celt_dec_offset = (*st).silk_dec_offset + silkDecSizeBytes;
    silk_dec =
        (st as *mut libc::c_char).offset((*st).silk_dec_offset as isize) as *mut libc::c_void;
    celt_dec =
        (st as *mut libc::c_char).offset((*st).celt_dec_offset as isize) as *mut OpusCustomDecoder;
    (*st).channels = channels;
    (*st).stream_channels = (*st).channels;
    (*st).Fs = Fs;
    (*st).DecControl.API_sampleRate = (*st).Fs;
    (*st).DecControl.nChannelsAPI = (*st).channels;
    ret = silk_InitDecoder(silk_dec);
    if ret != 0 {
        return -(3 as libc::c_int);
    }
    ret = celt_decoder_init(celt_dec, Fs, channels);
    if ret != 0 as libc::c_int {
        return -(3 as libc::c_int);
    }
    opus_custom_decoder_ctl(celt_dec, 10016 as libc::c_int, 0 as libc::c_int);
    (*st).prev_mode = 0 as libc::c_int;
    (*st).frame_size = Fs / 400 as libc::c_int;
    (*st).arch = opus_select_arch();
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "159:1"]
pub unsafe extern "C" fn opus_decoder_create(
    mut Fs: opus_int32,
    mut channels: libc::c_int,
    mut error: *mut libc::c_int,
) -> *mut OpusDecoder {
    let mut ret: libc::c_int = 0;
    let mut st: *mut OpusDecoder = 0 as *mut OpusDecoder;
    if Fs != 48000 as libc::c_int
        && Fs != 24000 as libc::c_int
        && Fs != 16000 as libc::c_int
        && Fs != 12000 as libc::c_int
        && Fs != 8000 as libc::c_int
        || channels != 1 as libc::c_int && channels != 2 as libc::c_int
    {
        if !error.is_null() {
            *error = -(1 as libc::c_int);
        }
        return 0 as *mut OpusDecoder;
    }
    st = opus_alloc(opus_decoder_get_size(channels) as size_t) as *mut OpusDecoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as libc::c_int);
        }
        return 0 as *mut OpusDecoder;
    }
    ret = opus_decoder_init(st, Fs, channels);
    if !error.is_null() {
        *error = ret;
    }
    if ret != 0 as libc::c_int {
        opus_free(st as *mut libc::c_void);
        st = 0 as *mut OpusDecoder;
    }
    return st;
}
#[c2rust::src_loc = "188:1"]
unsafe extern "C" fn smooth_fade(
    mut in1: *const opus_val16,
    mut in2: *const opus_val16,
    mut out: *mut opus_val16,
    mut overlap: libc::c_int,
    mut channels: libc::c_int,
    mut window: *const opus_val16,
    mut Fs: opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut inc: libc::c_int = 48000 as libc::c_int / Fs;
    c = 0 as libc::c_int;
    while c < channels {
        i = 0 as libc::c_int;
        while i < overlap {
            let mut w: opus_val16 =
                *window.offset((i * inc) as isize) * *window.offset((i * inc) as isize);
            *out.offset((i * channels + c) as isize) = w * *in2.offset((i * channels + c) as isize)
                + (1.0f32 - w) * *in1.offset((i * channels + c) as isize);
            i += 1;
        }
        c += 1;
    }
}
#[c2rust::src_loc = "205:1"]
unsafe extern "C" fn opus_packet_get_mode(mut data: *const libc::c_uchar) -> libc::c_int {
    let mut mode: libc::c_int = 0;
    if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int != 0 {
        mode = 1002 as libc::c_int;
    } else if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x60 as libc::c_int
        == 0x60 as libc::c_int
    {
        mode = 1001 as libc::c_int;
    } else {
        mode = 1000 as libc::c_int;
    }
    return mode;
}
#[c2rust::src_loc = "220:1"]
unsafe extern "C" fn opus_decode_frame(
    mut st: *mut OpusDecoder,
    mut data: *const libc::c_uchar,
    mut len: opus_int32,
    mut pcm: *mut opus_val16,
    mut frame_size: libc::c_int,
    mut decode_fec: libc::c_int,
) -> libc::c_int {
    let mut silk_dec: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut celt_dec: *mut OpusCustomDecoder = 0 as *mut OpusCustomDecoder;
    let mut i: libc::c_int = 0;
    let mut silk_ret: libc::c_int = 0 as libc::c_int;
    let mut celt_ret: libc::c_int = 0 as libc::c_int;
    let mut dec: ec_dec = ec_dec {
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
    let mut silk_frame_size: opus_int32 = 0;
    let mut pcm_silk_size: libc::c_int = 0;
    let mut pcm_transition_silk_size: libc::c_int = 0;
    let mut pcm_transition_celt_size: libc::c_int = 0;
    let mut pcm_transition: *mut opus_val16 = 0 as *mut opus_val16;
    let mut redundant_audio_size: libc::c_int = 0;
    let mut audiosize: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    let mut bandwidth: libc::c_int = 0;
    let mut transition: libc::c_int = 0 as libc::c_int;
    let mut start_band: libc::c_int = 0;
    let mut redundancy: libc::c_int = 0 as libc::c_int;
    let mut redundancy_bytes: libc::c_int = 0 as libc::c_int;
    let mut celt_to_silk: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut F2_5: libc::c_int = 0;
    let mut F5: libc::c_int = 0;
    let mut F10: libc::c_int = 0;
    let mut F20: libc::c_int = 0;
    let mut window: *const opus_val16 = 0 as *const opus_val16;
    let mut redundant_rng: opus_uint32 = 0 as libc::c_int as opus_uint32;
    let mut celt_accum: libc::c_int = 0;
    silk_dec =
        (st as *mut libc::c_char).offset((*st).silk_dec_offset as isize) as *mut libc::c_void;
    celt_dec =
        (st as *mut libc::c_char).offset((*st).celt_dec_offset as isize) as *mut OpusCustomDecoder;
    F20 = (*st).Fs / 50 as libc::c_int;
    F10 = F20 >> 1 as libc::c_int;
    F5 = F10 >> 1 as libc::c_int;
    F2_5 = F5 >> 1 as libc::c_int;
    if frame_size < F2_5 {
        return -(2 as libc::c_int);
    }
    frame_size = if frame_size < (*st).Fs / 25 as libc::c_int * 3 as libc::c_int {
        frame_size
    } else {
        (*st).Fs / 25 as libc::c_int * 3 as libc::c_int
    };
    if len <= 1 as libc::c_int {
        data = 0 as *const libc::c_uchar;
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
        ec_dec_init(&mut dec, data as *mut libc::c_uchar, len as opus_uint32);
    } else {
        audiosize = frame_size;
        mode = (*st).prev_mode;
        bandwidth = 0 as libc::c_int;
        if mode == 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < audiosize * (*st).channels {
                *pcm.offset(i as isize) = 0 as libc::c_int as opus_val16;
                i += 1;
            }
            return audiosize;
        }
        if audiosize > F20 {
            loop {
                let mut ret: libc::c_int = opus_decode_frame(
                    st,
                    0 as *const libc::c_uchar,
                    0 as libc::c_int,
                    pcm,
                    if audiosize < F20 { audiosize } else { F20 },
                    0 as libc::c_int,
                );
                if ret < 0 as libc::c_int {
                    return ret;
                }
                pcm = pcm.offset((ret * (*st).channels) as isize);
                audiosize -= ret;
                if !(audiosize > 0 as libc::c_int) {
                    break;
                }
            }
            return frame_size;
        } else {
            if audiosize < F20 {
                if audiosize > F10 {
                    audiosize = F10;
                } else if mode != 1000 as libc::c_int && audiosize > F5 && audiosize < F10 {
                    audiosize = F5;
                }
            }
        }
    }
    celt_accum = 0 as libc::c_int;
    pcm_transition_silk_size = 1 as libc::c_int;
    pcm_transition_celt_size = 1 as libc::c_int;
    if !data.is_null()
        && (*st).prev_mode > 0 as libc::c_int
        && (mode == 1002 as libc::c_int
            && (*st).prev_mode != 1002 as libc::c_int
            && (*st).prev_redundancy == 0
            || mode != 1002 as libc::c_int && (*st).prev_mode == 1002 as libc::c_int)
    {
        transition = 1 as libc::c_int;
        if mode == 1002 as libc::c_int {
            pcm_transition_celt_size = F5 * (*st).channels;
        } else {
            pcm_transition_silk_size = F5 * (*st).channels;
        }
    }
    let vla = pcm_transition_celt_size as usize;
    let mut pcm_transition_celt: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    if transition != 0 && mode == 1002 as libc::c_int {
        pcm_transition = pcm_transition_celt.as_mut_ptr();
        opus_decode_frame(
            st,
            0 as *const libc::c_uchar,
            0 as libc::c_int,
            pcm_transition,
            if F5 < audiosize { F5 } else { audiosize },
            0 as libc::c_int,
        );
    }
    if audiosize > frame_size {
        return -(1 as libc::c_int);
    } else {
        frame_size = audiosize;
    }
    pcm_silk_size = if mode != 1002 as libc::c_int && celt_accum == 0 {
        (if F10 > frame_size { F10 } else { frame_size }) * (*st).channels
    } else {
        1 as libc::c_int
    };
    let vla_0 = pcm_silk_size as usize;
    let mut pcm_silk: Vec<opus_int16> = ::std::vec::from_elem(0, vla_0);
    if mode != 1002 as libc::c_int {
        let mut lost_flag: libc::c_int = 0;
        let mut decoded_samples: libc::c_int = 0;
        let mut pcm_ptr: *mut opus_int16 = 0 as *mut opus_int16;
        pcm_ptr = pcm_silk.as_mut_ptr();
        if (*st).prev_mode == 1002 as libc::c_int {
            silk_InitDecoder(silk_dec);
        }
        (*st).DecControl.payloadSize_ms =
            if 10 as libc::c_int > 1000 as libc::c_int * audiosize / (*st).Fs {
                10 as libc::c_int
            } else {
                1000 as libc::c_int * audiosize / (*st).Fs
            };
        if !data.is_null() {
            (*st).DecControl.nChannelsInternal = (*st).stream_channels;
            if mode == 1000 as libc::c_int {
                if bandwidth == 1101 as libc::c_int {
                    (*st).DecControl.internalSampleRate = 8000 as libc::c_int;
                } else if bandwidth == 1102 as libc::c_int {
                    (*st).DecControl.internalSampleRate = 12000 as libc::c_int;
                } else if bandwidth == 1103 as libc::c_int {
                    (*st).DecControl.internalSampleRate = 16000 as libc::c_int;
                } else {
                    (*st).DecControl.internalSampleRate = 16000 as libc::c_int;
                    if 0 as libc::c_int == 0 {
                        celt_fatal(
                            b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                            389 as libc::c_int,
                        );
                    }
                }
            } else {
                (*st).DecControl.internalSampleRate = 16000 as libc::c_int;
            }
        }
        lost_flag = if data.is_null() {
            1 as libc::c_int
        } else {
            2 as libc::c_int * decode_fec
        };
        decoded_samples = 0 as libc::c_int;
        loop {
            let mut first_frame: libc::c_int = (decoded_samples == 0 as libc::c_int) as libc::c_int;
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
                    i = 0 as libc::c_int;
                    while i < frame_size * (*st).channels {
                        *pcm_ptr.offset(i as isize) = 0 as libc::c_int as opus_int16;
                        i += 1;
                    }
                } else {
                    return -(3 as libc::c_int);
                }
            }
            pcm_ptr = pcm_ptr.offset((silk_frame_size * (*st).channels) as isize);
            decoded_samples += silk_frame_size;
            if !(decoded_samples < frame_size) {
                break;
            }
        }
    }
    start_band = 0 as libc::c_int;
    if decode_fec == 0
        && mode != 1002 as libc::c_int
        && !data.is_null()
        && ec_tell(&mut dec)
            + 17 as libc::c_int
            + 20 as libc::c_int * ((*st).mode == 1001 as libc::c_int) as libc::c_int
            <= 8 as libc::c_int * len
    {
        if mode == 1001 as libc::c_int {
            redundancy = ec_dec_bit_logp(&mut dec, 12 as libc::c_int as libc::c_uint);
        } else {
            redundancy = 1 as libc::c_int;
        }
        if redundancy != 0 {
            celt_to_silk = ec_dec_bit_logp(&mut dec, 1 as libc::c_int as libc::c_uint);
            redundancy_bytes = if mode == 1001 as libc::c_int {
                ec_dec_uint(&mut dec, 256 as libc::c_int as opus_uint32) as opus_int32
                    + 2 as libc::c_int
            } else {
                len - (ec_tell(&mut dec) + 7 as libc::c_int >> 3 as libc::c_int)
            };
            len -= redundancy_bytes;
            if (len * 8 as libc::c_int) < ec_tell(&mut dec) {
                len = 0 as libc::c_int;
                redundancy_bytes = 0 as libc::c_int;
                redundancy = 0 as libc::c_int;
            }
            dec.storage = (dec.storage as libc::c_uint)
                .wrapping_sub(redundancy_bytes as libc::c_uint)
                as opus_uint32 as opus_uint32;
        }
    }
    if mode != 1002 as libc::c_int {
        start_band = 17 as libc::c_int;
    }
    if redundancy != 0 {
        transition = 0 as libc::c_int;
        pcm_transition_silk_size = 1 as libc::c_int;
    }
    let vla_1 = pcm_transition_silk_size as usize;
    let mut pcm_transition_silk: Vec<opus_val16> = ::std::vec::from_elem(0., vla_1);
    if transition != 0 && mode != 1002 as libc::c_int {
        pcm_transition = pcm_transition_silk.as_mut_ptr();
        opus_decode_frame(
            st,
            0 as *const libc::c_uchar,
            0 as libc::c_int,
            pcm_transition,
            if F5 < audiosize { F5 } else { audiosize },
            0 as libc::c_int,
        );
    }
    if bandwidth != 0 {
        let mut endband: libc::c_int = 21 as libc::c_int;
        match bandwidth {
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
            _ => {
                if 0 as libc::c_int == 0 {
                    celt_fatal(
                        b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                        b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                        488 as libc::c_int,
                    );
                }
            }
        }
        if !(opus_custom_decoder_ctl(celt_dec, 10012 as libc::c_int, endband) == 0 as libc::c_int) {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl(celt_dec, 10012, (((void)((endband) == (opus_int32)0)), (opus_int32)(endband)))) == OPUS_OK\0"
                    as *const u8 as *const libc::c_char,
                b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                491 as libc::c_int,
            );
        }
    }
    if !(opus_custom_decoder_ctl(celt_dec, 10008 as libc::c_int, (*st).stream_channels)
        == 0 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: (opus_custom_decoder_ctl(celt_dec, 10008, (((void)((st->stream_channels) == (opus_int32)0)), (opus_int32)(st->stream_channels)))) == OPUS_OK\0"
                as *const u8 as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            493 as libc::c_int,
        );
    }
    redundant_audio_size = if redundancy != 0 {
        F5 * (*st).channels
    } else {
        1 as libc::c_int
    };
    let vla_2 = redundant_audio_size as usize;
    let mut redundant_audio: Vec<opus_val16> = ::std::vec::from_elem(0., vla_2);
    if redundancy != 0 && celt_to_silk != 0 {
        if !(opus_custom_decoder_ctl(celt_dec, 10010 as libc::c_int, 0 as libc::c_int)
            == 0 as libc::c_int)
        {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl(celt_dec, 10010, (((void)((0) == (opus_int32)0)), (opus_int32)(0)))) == OPUS_OK\0"
                    as *const u8 as *const libc::c_char,
                b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                502 as libc::c_int,
            );
        }
        celt_decode_with_ec(
            celt_dec,
            data.offset(len as isize),
            redundancy_bytes,
            redundant_audio.as_mut_ptr(),
            F5,
            0 as *mut ec_dec,
            0 as libc::c_int,
        );
        if !(opus_custom_decoder_ctl(
            celt_dec,
            4031 as libc::c_int,
            (&mut redundant_rng as *mut opus_uint32).offset(
                (&mut redundant_rng as *mut opus_uint32)
                    .offset_from(&mut redundant_rng as *mut opus_uint32)
                    as libc::c_long as isize,
            ),
        ) == 0 as libc::c_int)
        {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl(celt_dec, 4031, ((&redundant_rng) + ((&redundant_rng) - (opus_uint32*)(&redundant_rng))))) == OPUS_OK\0"
                    as *const u8 as *const libc::c_char,
                b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                505 as libc::c_int,
            );
        }
    }
    if !(opus_custom_decoder_ctl(celt_dec, 10010 as libc::c_int, start_band) == 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: (opus_custom_decoder_ctl(celt_dec, 10010, (((void)((start_band) == (opus_int32)0)), (opus_int32)(start_band)))) == OPUS_OK\0"
                as *const u8 as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            509 as libc::c_int,
        );
    }
    if mode != 1000 as libc::c_int {
        let mut celt_frame_size: libc::c_int = if F20 < frame_size { F20 } else { frame_size };
        if mode != (*st).prev_mode
            && (*st).prev_mode > 0 as libc::c_int
            && (*st).prev_redundancy == 0
        {
            if !(opus_custom_decoder_ctl(celt_dec, 4028 as libc::c_int) == 0 as libc::c_int) {
                celt_fatal(
                    b"assertion failed: (opus_custom_decoder_ctl(celt_dec, 4028)) == OPUS_OK\0"
                        as *const u8 as *const libc::c_char,
                    b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                    516 as libc::c_int,
                );
            }
        }
        celt_ret = celt_decode_with_ec(
            celt_dec,
            if decode_fec != 0 {
                0 as *const libc::c_uchar
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
        let mut silence: [libc::c_uchar; 2] = [
            0xff as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
        ];
        if celt_accum == 0 {
            i = 0 as libc::c_int;
            while i < frame_size * (*st).channels {
                *pcm.offset(i as isize) = 0 as libc::c_int as opus_val16;
                i += 1;
            }
        }
        if (*st).prev_mode == 1001 as libc::c_int
            && !(redundancy != 0 && celt_to_silk != 0 && (*st).prev_redundancy != 0)
        {
            if !(opus_custom_decoder_ctl(celt_dec, 10010 as libc::c_int, 0 as libc::c_int)
                == 0 as libc::c_int)
            {
                celt_fatal(
                    b"assertion failed: (opus_custom_decoder_ctl(celt_dec, 10010, (((void)((0) == (opus_int32)0)), (opus_int32)(0)))) == OPUS_OK\0"
                        as *const u8 as *const libc::c_char,
                    b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                    531 as libc::c_int,
                );
            }
            celt_decode_with_ec(
                celt_dec,
                silence.as_mut_ptr(),
                2 as libc::c_int,
                pcm,
                F2_5,
                0 as *mut ec_dec,
                celt_accum,
            );
        }
    }
    if mode != 1002 as libc::c_int && celt_accum == 0 {
        i = 0 as libc::c_int;
        while i < frame_size * (*st).channels {
            *pcm.offset(i as isize) = *pcm.offset(i as isize)
                + 1.0f32 / 32768.0f32
                    * *pcm_silk.as_mut_ptr().offset(i as isize) as libc::c_int as libc::c_float;
            i += 1;
        }
    }
    let mut celt_mode: *const OpusCustomMode = 0 as *const OpusCustomMode;
    if !(opus_custom_decoder_ctl(
        celt_dec,
        10015 as libc::c_int,
        (&mut celt_mode as *mut *const OpusCustomMode).offset(
            (&mut celt_mode as *mut *const OpusCustomMode)
                .offset_from(&mut celt_mode as *mut *const OpusCustomMode)
                as libc::c_long as isize,
        ),
    ) == 0 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: (opus_custom_decoder_ctl(celt_dec, 10015, ((&celt_mode) + ((&celt_mode) - (const OpusCustomMode**)(&celt_mode))))) == OPUS_OK\0"
                as *const u8 as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            549 as libc::c_int,
        );
    }
    window = (*celt_mode).window;
    if redundancy != 0 && celt_to_silk == 0 {
        if !(opus_custom_decoder_ctl(celt_dec, 4028 as libc::c_int) == 0 as libc::c_int) {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl(celt_dec, 4028)) == OPUS_OK\0"
                    as *const u8 as *const libc::c_char,
                b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                556 as libc::c_int,
            );
        }
        if !(opus_custom_decoder_ctl(celt_dec, 10010 as libc::c_int, 0 as libc::c_int)
            == 0 as libc::c_int)
        {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl(celt_dec, 10010, (((void)((0) == (opus_int32)0)), (opus_int32)(0)))) == OPUS_OK\0"
                    as *const u8 as *const libc::c_char,
                b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                557 as libc::c_int,
            );
        }
        celt_decode_with_ec(
            celt_dec,
            data.offset(len as isize),
            redundancy_bytes,
            redundant_audio.as_mut_ptr(),
            F5,
            0 as *mut ec_dec,
            0 as libc::c_int,
        );
        if !(opus_custom_decoder_ctl(
            celt_dec,
            4031 as libc::c_int,
            (&mut redundant_rng as *mut opus_uint32).offset(
                (&mut redundant_rng as *mut opus_uint32)
                    .offset_from(&mut redundant_rng as *mut opus_uint32)
                    as libc::c_long as isize,
            ),
        ) == 0 as libc::c_int)
        {
            celt_fatal(
                b"assertion failed: (opus_custom_decoder_ctl(celt_dec, 4031, ((&redundant_rng) + ((&redundant_rng) - (opus_uint32*)(&redundant_rng))))) == OPUS_OK\0"
                    as *const u8 as *const libc::c_char,
                b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                560 as libc::c_int,
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
        c = 0 as libc::c_int;
        while c < (*st).channels {
            i = 0 as libc::c_int;
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
            i = 0 as libc::c_int;
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
        gain = exp(0.6931471805599453094f64
            * (6.48814081e-4f32 * (*st).decode_gain as libc::c_float) as libc::c_double)
            as libc::c_float;
        i = 0 as libc::c_int;
        while i < frame_size * (*st).channels {
            let mut x: opus_val32 = 0.;
            x = *pcm.offset(i as isize) * gain;
            *pcm.offset(i as isize) = x;
            i += 1;
        }
    }
    if len <= 1 as libc::c_int {
        (*st).rangeFinal = 0 as libc::c_int as opus_uint32;
    } else {
        (*st).rangeFinal = dec.rng ^ redundant_rng;
    }
    (*st).prev_mode = mode;
    (*st).prev_redundancy = (redundancy != 0 && celt_to_silk == 0) as libc::c_int;
    if celt_ret >= 0 as libc::c_int {
        _opus_false() != 0;
    }
    return if celt_ret < 0 as libc::c_int {
        celt_ret
    } else {
        audiosize
    };
}
#[no_mangle]
#[c2rust::src_loc = "626:1"]
pub unsafe extern "C" fn opus_decode_native(
    mut st: *mut OpusDecoder,
    mut data: *const libc::c_uchar,
    mut len: opus_int32,
    mut pcm: *mut opus_val16,
    mut frame_size: libc::c_int,
    mut decode_fec: libc::c_int,
    mut self_delimited: libc::c_int,
    mut packet_offset: *mut opus_int32,
    mut soft_clip: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nb_samples: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut toc: libc::c_uchar = 0;
    let mut packet_frame_size: libc::c_int = 0;
    let mut packet_bandwidth: libc::c_int = 0;
    let mut packet_mode: libc::c_int = 0;
    let mut packet_stream_channels: libc::c_int = 0;
    let mut size: [opus_int16; 48] = [0; 48];
    validate_opus_decoder(st);
    if decode_fec < 0 as libc::c_int || decode_fec > 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (decode_fec != 0 || len == 0 as libc::c_int || data.is_null())
        && frame_size % ((*st).Fs / 400 as libc::c_int) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if len == 0 as libc::c_int || data.is_null() {
        let mut pcm_count: libc::c_int = 0 as libc::c_int;
        loop {
            let mut ret: libc::c_int = 0;
            ret = opus_decode_frame(
                st,
                0 as *const libc::c_uchar,
                0 as libc::c_int,
                pcm.offset((pcm_count * (*st).channels) as isize),
                frame_size - pcm_count,
                0 as libc::c_int,
            );
            if ret < 0 as libc::c_int {
                return ret;
            }
            pcm_count += ret;
            if !(pcm_count < frame_size) {
                break;
            }
        }
        if !(pcm_count == frame_size) {
            celt_fatal(
                b"assertion failed: pcm_count == frame_size\0" as *const u8 as *const libc::c_char,
                b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                652 as libc::c_int,
            );
        }
        _opus_false() != 0;
        (*st).last_packet_duration = pcm_count;
        return pcm_count;
    } else {
        if len < 0 as libc::c_int {
            return -(1 as libc::c_int);
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
        0 as *mut *const libc::c_uchar,
        size.as_mut_ptr(),
        &mut offset,
        packet_offset,
    );
    if count < 0 as libc::c_int {
        return count;
    }
    data = data.offset(offset as isize);
    if decode_fec != 0 {
        let mut duration_copy: libc::c_int = 0;
        let mut ret_0: libc::c_int = 0;
        if frame_size < packet_frame_size
            || packet_mode == 1002 as libc::c_int
            || (*st).mode == 1002 as libc::c_int
        {
            return opus_decode_native(
                st,
                0 as *const libc::c_uchar,
                0 as libc::c_int,
                pcm,
                frame_size,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut opus_int32,
                soft_clip,
            );
        }
        duration_copy = (*st).last_packet_duration;
        if frame_size - packet_frame_size != 0 as libc::c_int {
            ret_0 = opus_decode_native(
                st,
                0 as *const libc::c_uchar,
                0 as libc::c_int,
                pcm,
                frame_size - packet_frame_size,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut opus_int32,
                soft_clip,
            );
            if ret_0 < 0 as libc::c_int {
                (*st).last_packet_duration = duration_copy;
                return ret_0;
            }
            if !(ret_0 == frame_size - packet_frame_size) {
                celt_fatal(
                    b"assertion failed: ret==frame_size-packet_frame_size\0" as *const u8
                        as *const libc::c_char,
                    b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                    689 as libc::c_int,
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
            size[0 as libc::c_int as usize] as opus_int32,
            pcm.offset(((*st).channels * (frame_size - packet_frame_size)) as isize),
            packet_frame_size,
            1 as libc::c_int,
        );
        if ret_0 < 0 as libc::c_int {
            return ret_0;
        } else {
            _opus_false() != 0;
            (*st).last_packet_duration = frame_size;
            return frame_size;
        }
    }
    if count * packet_frame_size > frame_size {
        return -(2 as libc::c_int);
    }
    (*st).mode = packet_mode;
    (*st).bandwidth = packet_bandwidth;
    (*st).frame_size = packet_frame_size;
    (*st).stream_channels = packet_stream_channels;
    nb_samples = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        let mut ret_1: libc::c_int = 0;
        ret_1 = opus_decode_frame(
            st,
            data,
            size[i as usize] as opus_int32,
            pcm.offset((nb_samples * (*st).channels) as isize),
            frame_size - nb_samples,
            0 as libc::c_int,
        );
        if ret_1 < 0 as libc::c_int {
            return ret_1;
        }
        if !(ret_1 == packet_frame_size) {
            celt_fatal(
                b"assertion failed: ret==packet_frame_size\0" as *const u8 as *const libc::c_char,
                b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
                724 as libc::c_int,
            );
        }
        data = data.offset(size[i as usize] as libc::c_int as isize);
        nb_samples += ret_1;
        i += 1;
    }
    (*st).last_packet_duration = nb_samples;
    _opus_false() != 0;
    if soft_clip != 0 {
        opus_pcm_soft_clip(
            pcm,
            nb_samples,
            (*st).channels,
            ((*st).softclip_mem).as_mut_ptr(),
        );
    } else {
        (*st).softclip_mem[1 as libc::c_int as usize] = 0 as libc::c_int as opus_val16;
        (*st).softclip_mem[0 as libc::c_int as usize] =
            (*st).softclip_mem[1 as libc::c_int as usize];
    }
    return nb_samples;
}
#[no_mangle]
#[c2rust::src_loc = "788:1"]
pub unsafe extern "C" fn opus_decode(
    mut st: *mut OpusDecoder,
    mut data: *const libc::c_uchar,
    mut len: opus_int32,
    mut pcm: *mut opus_int16,
    mut frame_size: libc::c_int,
    mut decode_fec: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nb_samples: libc::c_int = 0;
    if frame_size <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !data.is_null() && len > 0 as libc::c_int && decode_fec == 0 {
        nb_samples = opus_decoder_get_nb_samples(st, data, len);
        if nb_samples > 0 as libc::c_int {
            frame_size = if frame_size < nb_samples {
                frame_size
            } else {
                nb_samples
            };
        } else {
            return -(4 as libc::c_int);
        }
    }
    if !((*st).channels == 1 as libc::c_int || (*st).channels == 2 as libc::c_int) {
        celt_fatal(
            b"assertion failed: st->channels == 1 || st->channels == 2\0" as *const u8
                as *const libc::c_char,
            b"src/opus_decoder.c\0" as *const u8 as *const libc::c_char,
            810 as libc::c_int,
        );
    }
    let vla = (frame_size * (*st).channels) as usize;
    let mut out: Vec<libc::c_float> = ::std::vec::from_elem(0., vla);
    ret = opus_decode_native(
        st,
        data,
        len,
        out.as_mut_ptr(),
        frame_size,
        decode_fec,
        0 as libc::c_int,
        0 as *mut opus_int32,
        1 as libc::c_int,
    );
    if ret > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < ret * (*st).channels {
            *pcm.offset(i as isize) = FLOAT2INT16(*out.as_mut_ptr().offset(i as isize));
            i += 1;
        }
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "823:1"]
pub unsafe extern "C" fn opus_decode_float(
    mut st: *mut OpusDecoder,
    mut data: *const libc::c_uchar,
    mut len: opus_int32,
    mut pcm: *mut opus_val16,
    mut frame_size: libc::c_int,
    mut decode_fec: libc::c_int,
) -> libc::c_int {
    if frame_size <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return opus_decode_native(
        st,
        data,
        len,
        pcm,
        frame_size,
        decode_fec,
        0 as libc::c_int,
        0 as *mut opus_int32,
        0 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "833:1"]
pub unsafe extern "C" fn opus_decoder_ctl(
    mut st: *mut OpusDecoder,
    mut request: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ap: ::core::ffi::VaListImpl;
    let mut silk_dec: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut celt_dec: *mut OpusCustomDecoder = 0 as *mut OpusCustomDecoder;
    silk_dec =
        (st as *mut libc::c_char).offset((*st).silk_dec_offset as isize) as *mut libc::c_void;
    celt_dec =
        (st as *mut libc::c_char).offset((*st).celt_dec_offset as isize) as *mut OpusCustomDecoder;
    ap = args.clone();
    match request {
        4009 => {
            let mut value: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value.is_null() {
                current_block = 2989495919056355252;
            } else {
                *value = (*st).bandwidth;
                current_block = 3160140712158701372;
            }
        }
        4031 => {
            let mut value_0: *mut opus_uint32 = ap.arg::<*mut opus_uint32>();
            if value_0.is_null() {
                current_block = 2989495919056355252;
            } else {
                *value_0 = (*st).rangeFinal;
                current_block = 3160140712158701372;
            }
        }
        4028 => {
            memset(
                &mut (*st).stream_channels as *mut libc::c_int as *mut libc::c_char
                    as *mut libc::c_void,
                0 as libc::c_int,
                (::core::mem::size_of::<OpusDecoder>() as libc::c_ulong)
                    .wrapping_sub(
                        (&mut (*st).stream_channels as *mut libc::c_int as *mut libc::c_char)
                            .offset_from(st as *mut libc::c_char)
                            as libc::c_long as libc::c_ulong,
                    )
                    .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            opus_custom_decoder_ctl(celt_dec, 4028 as libc::c_int);
            silk_InitDecoder(silk_dec);
            (*st).stream_channels = (*st).channels;
            (*st).frame_size = (*st).Fs / 400 as libc::c_int;
            current_block = 3160140712158701372;
        }
        4029 => {
            let mut value_1: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_1.is_null() {
                current_block = 2989495919056355252;
            } else {
                *value_1 = (*st).Fs;
                current_block = 3160140712158701372;
            }
        }
        4033 => {
            let mut value_2: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_2.is_null() {
                current_block = 2989495919056355252;
            } else {
                if (*st).prev_mode == 1002 as libc::c_int {
                    ret = opus_custom_decoder_ctl(
                        celt_dec,
                        4033 as libc::c_int,
                        value_2.offset(value_2.offset_from(value_2) as libc::c_long as isize),
                    );
                } else {
                    *value_2 = (*st).DecControl.prevPitchLag;
                }
                current_block = 3160140712158701372;
            }
        }
        4045 => {
            let mut value_3: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_3.is_null() {
                current_block = 2989495919056355252;
            } else {
                *value_3 = (*st).decode_gain;
                current_block = 3160140712158701372;
            }
        }
        4034 => {
            let mut value_4: opus_int32 = ap.arg::<opus_int32>();
            if value_4 < -(32768 as libc::c_int) || value_4 > 32767 as libc::c_int {
                current_block = 2989495919056355252;
            } else {
                (*st).decode_gain = value_4;
                current_block = 3160140712158701372;
            }
        }
        4039 => {
            let mut value_5: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_5.is_null() {
                current_block = 2989495919056355252;
            } else {
                *value_5 = (*st).last_packet_duration;
                current_block = 3160140712158701372;
            }
        }
        4046 => {
            let mut value_6: opus_int32 = ap.arg::<opus_int32>();
            if value_6 < 0 as libc::c_int || value_6 > 1 as libc::c_int {
                current_block = 2989495919056355252;
            } else {
                ret = opus_custom_decoder_ctl(celt_dec, 4046 as libc::c_int, value_6);
                current_block = 3160140712158701372;
            }
        }
        4047 => {
            let mut value_7: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            if value_7.is_null() {
                current_block = 2989495919056355252;
            } else {
                ret = opus_custom_decoder_ctl(
                    celt_dec,
                    4047 as libc::c_int,
                    value_7.offset(value_7.offset_from(value_7) as libc::c_long as isize),
                );
                current_block = 3160140712158701372;
            }
        }
        _ => {
            ret = -(5 as libc::c_int);
            current_block = 3160140712158701372;
        }
    }
    match current_block {
        2989495919056355252 => return -(1 as libc::c_int),
        _ => return ret,
    };
}
#[no_mangle]
#[c2rust::src_loc = "966:1"]
pub unsafe extern "C" fn opus_decoder_destroy(mut st: *mut OpusDecoder) {
    opus_free(st as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "972:1"]
pub unsafe extern "C" fn opus_packet_get_bandwidth(mut data: *const libc::c_uchar) -> libc::c_int {
    let mut bandwidth: libc::c_int = 0;
    if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int != 0 {
        bandwidth = 1102 as libc::c_int
            + (*data.offset(0 as libc::c_int as isize) as libc::c_int >> 5 as libc::c_int
                & 0x3 as libc::c_int);
        if bandwidth == 1102 as libc::c_int {
            bandwidth = 1101 as libc::c_int;
        }
    } else if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x60 as libc::c_int
        == 0x60 as libc::c_int
    {
        bandwidth =
            if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x10 as libc::c_int != 0 {
                1105 as libc::c_int
            } else {
                1104 as libc::c_int
            };
    } else {
        bandwidth = 1101 as libc::c_int
            + (*data.offset(0 as libc::c_int as isize) as libc::c_int >> 5 as libc::c_int
                & 0x3 as libc::c_int);
    }
    return bandwidth;
}
#[no_mangle]
#[c2rust::src_loc = "990:1"]
pub unsafe extern "C" fn opus_packet_get_nb_channels(
    mut data: *const libc::c_uchar,
) -> libc::c_int {
    return if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x4 as libc::c_int != 0 {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
#[no_mangle]
#[c2rust::src_loc = "995:1"]
pub unsafe extern "C" fn opus_packet_get_nb_frames(
    mut packet: *const libc::c_uchar,
    mut len: opus_int32,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    if len < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    count = *packet.offset(0 as libc::c_int as isize) as libc::c_int & 0x3 as libc::c_int;
    if count == 0 as libc::c_int {
        return 1 as libc::c_int;
    } else if count != 3 as libc::c_int {
        return 2 as libc::c_int;
    } else if len < 2 as libc::c_int {
        return -(4 as libc::c_int);
    } else {
        return *packet.offset(1 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int;
    };
}
#[no_mangle]
#[c2rust::src_loc = "1011:1"]
pub unsafe extern "C" fn opus_packet_get_nb_samples(
    mut packet: *const libc::c_uchar,
    mut len: opus_int32,
    mut Fs: opus_int32,
) -> libc::c_int {
    let mut samples: libc::c_int = 0;
    let mut count: libc::c_int = opus_packet_get_nb_frames(packet, len);
    if count < 0 as libc::c_int {
        return count;
    }
    samples = count * opus_packet_get_samples_per_frame(packet, Fs);
    if samples * 25 as libc::c_int > Fs * 3 as libc::c_int {
        return -(4 as libc::c_int);
    } else {
        return samples;
    };
}
#[no_mangle]
#[c2rust::src_loc = "1028:1"]
pub unsafe extern "C" fn opus_decoder_get_nb_samples(
    mut dec: *const OpusDecoder,
    mut packet: *const libc::c_uchar,
    mut len: opus_int32,
) -> libc::c_int {
    return opus_packet_get_nb_samples(packet, len, (*dec).Fs);
}
