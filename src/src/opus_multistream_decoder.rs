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
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus.h:32"]
pub mod opus_h {
    extern "C" {
        #[c2rust::src_loc = "399:16"]
        pub type OpusDecoder;
        #[c2rust::src_loc = "406:1"]
        pub fn opus_decoder_get_size(channels: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "440:1"]
        pub fn opus_decoder_init(
            st: *mut OpusDecoder,
            Fs: i32,
            channels: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "507:1"]
        pub fn opus_decoder_ctl(st: *mut OpusDecoder, request: libc::c_int, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "584:1"]
        pub fn opus_packet_get_nb_samples(
            packet: *const libc::c_uchar,
            len: i32,
            Fs: i32,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/opus_private.h:34"]
pub mod opus_private_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:16"]
    pub struct ChannelLayout {
        pub nb_channels: libc::c_int,
        pub nb_streams: libc::c_int,
        pub nb_coupled_streams: libc::c_int,
        pub mapping: [libc::c_uchar; 256],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "73:8"]
    pub struct OpusMSDecoder {
        pub layout: ChannelLayout,
    }
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
        pub i: i32,
        pub v: opus_val32,
    }
    #[c2rust::src_loc = "98:1"]
    pub type opus_copy_channel_out_func = Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            *const opus_val16,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >;
    #[inline]
    #[c2rust::src_loc = "154:1"]
    pub unsafe extern "C" fn align(i: libc::c_int) -> libc::c_int {
        let alignment: libc::c_uint = 8 as libc::c_ulong as libc::c_uint;
        return (i as libc::c_uint)
            .wrapping_add(alignment)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(alignment)
            .wrapping_mul(alignment) as libc::c_int;
    }
    use super::arch_h::{opus_val16, opus_val32};
    use super::opus_h::OpusDecoder;
    extern "C" {
        #[c2rust::src_loc = "165:1"]
        pub fn opus_packet_parse_impl(
            data: *const libc::c_uchar,
            len: i32,
            self_delimited: libc::c_int,
            out_toc: *mut libc::c_uchar,
            frames: *mut *const libc::c_uchar,
            size: *mut i16,
            payload_offset: *mut libc::c_int,
            packet_offset: *mut i32,
        ) -> libc::c_int;
        #[c2rust::src_loc = "149:1"]
        pub fn opus_decode_native(
            st: *mut OpusDecoder,
            data: *const libc::c_uchar,
            len: i32,
            pcm: *mut opus_val16,
            frame_size: libc::c_int,
            decode_fec: libc::c_int,
            self_delimited: libc::c_int,
            packet_offset: *mut i32,
            soft_clip: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "84:1"]
        pub fn get_left_channel(
            layout: *const ChannelLayout,
            stream_id: libc::c_int,
            prev: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "85:1"]
        pub fn get_right_channel(
            layout: *const ChannelLayout,
            stream_id: libc::c_int,
            prev: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "86:1"]
        pub fn get_mono_channel(
            layout: *const ChannelLayout,
            stream_id: libc::c_int,
            prev: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "83:1"]
        pub fn validate_layout(layout: *const ChannelLayout) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:34"]
pub mod arch_h {
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "57:9"]
    pub const CELT_SIG_SCALE: libc::c_float = 32768.0f32;
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:34"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/xmmintrin.h:37"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stdarg.h:34"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_defines.h:32"]
pub mod opus_defines_h {
    #[c2rust::src_loc = "139:9"]
    pub const OPUS_GET_BANDWIDTH_REQUEST: libc::c_int = 4009;
    #[c2rust::src_loc = "156:9"]
    pub const OPUS_GET_SAMPLE_RATE_REQUEST: libc::c_int = 4029;
    #[c2rust::src_loc = "160:9"]
    pub const OPUS_GET_GAIN_REQUEST: libc::c_int = 4045;
    #[c2rust::src_loc = "163:9"]
    pub const OPUS_GET_LAST_PACKET_DURATION_REQUEST: libc::c_int = 4039;
    #[c2rust::src_loc = "170:9"]
    pub const OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST: libc::c_int = 4047;
    #[c2rust::src_loc = "157:9"]
    pub const OPUS_GET_FINAL_RANGE_REQUEST: libc::c_int = 4031;
    #[c2rust::src_loc = "662:9"]
    pub const OPUS_RESET_STATE: libc::c_int = 4028 as libc::c_int;
    #[c2rust::src_loc = "159:9"]
    pub const OPUS_SET_GAIN_REQUEST: libc::c_int = 4034;
    #[c2rust::src_loc = "169:9"]
    pub const OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST: libc::c_int = 4046;
    #[c2rust::src_loc = "56:9"]
    pub const OPUS_UNIMPLEMENTED: libc::c_int = -(5 as libc::c_int);
    #[c2rust::src_loc = "54:9"]
    pub const OPUS_INVALID_PACKET: libc::c_int = -(4 as libc::c_int);
    #[c2rust::src_loc = "46:9"]
    pub const OPUS_OK: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "50:9"]
    pub const OPUS_BUFFER_TOO_SMALL: libc::c_int = -(2 as libc::c_int);
    #[c2rust::src_loc = "48:9"]
    pub const OPUS_BAD_ARG: libc::c_int = -(1 as libc::c_int);
    #[c2rust::src_loc = "52:9"]
    pub const OPUS_INTERNAL_ERROR: libc::c_int = -(3 as libc::c_int);
    #[c2rust::src_loc = "60:9"]
    pub const OPUS_ALLOC_FAIL: libc::c_int = -(7 as libc::c_int);
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_multistream.h:32"]
pub mod opus_multistream_h {
    #[c2rust::src_loc = "56:9"]
    pub const OPUS_MULTISTREAM_GET_DECODER_STATE_REQUEST: libc::c_int = 5122;
}
#[c2rust::header_src = "/usr/include/stdlib.h:37"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut libc::c_void);
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/float_cast.h:37"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/os_support.h:38"]
pub mod os_support_h {
    #[inline]
    #[c2rust::src_loc = "64:1"]
    pub unsafe extern "C" fn opus_free(ptr: *mut libc::c_void) {
        free(ptr);
    }
    #[inline]
    #[c2rust::src_loc = "47:1"]
    pub unsafe extern "C" fn opus_alloc(size: size_t) -> *mut libc::c_void {
        return malloc(size);
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::{free, malloc};
}
pub use self::arch_h::{celt_fatal, opus_val16, opus_val32, CELT_SIG_SCALE};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::opus_defines_h::{
    OPUS_ALLOC_FAIL, OPUS_BAD_ARG, OPUS_BUFFER_TOO_SMALL, OPUS_GET_BANDWIDTH_REQUEST,
    OPUS_GET_FINAL_RANGE_REQUEST, OPUS_GET_GAIN_REQUEST, OPUS_GET_LAST_PACKET_DURATION_REQUEST,
    OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST, OPUS_GET_SAMPLE_RATE_REQUEST, OPUS_INTERNAL_ERROR,
    OPUS_INVALID_PACKET, OPUS_OK, OPUS_RESET_STATE, OPUS_SET_GAIN_REQUEST,
    OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST, OPUS_UNIMPLEMENTED,
};
use self::opus_h::{
    opus_decoder_ctl, opus_decoder_get_size, opus_decoder_init, opus_packet_get_nb_samples,
    OpusDecoder,
};
pub use self::opus_multistream_h::OPUS_MULTISTREAM_GET_DECODER_STATE_REQUEST;
pub use self::opus_private_h::{
    align, foo, get_left_channel, get_mono_channel, get_right_channel, opus_copy_channel_out_func,
    opus_decode_native, opus_packet_parse_impl, validate_layout, C2RustUnnamed, ChannelLayout,
    OpusMSDecoder,
};
pub use self::stdarg_h::va_list;
pub use self::stddef_h::{size_t, NULL};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::types_h::{__int16_t, __int32_t, __uint32_t};

pub use self::float_cast_h::{float2int, FLOAT2INT16};
pub use self::os_support_h::{opus_alloc, opus_free};
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn validate_ms_decoder(st: *mut OpusMSDecoder) {
    validate_layout(&mut (*st).layout);
}
#[no_mangle]
#[c2rust::src_loc = "53:1"]
pub unsafe extern "C" fn opus_multistream_decoder_get_size(
    nb_streams: libc::c_int,
    nb_coupled_streams: libc::c_int,
) -> i32 {
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    if nb_streams < 1 as libc::c_int
        || nb_coupled_streams > nb_streams
        || nb_coupled_streams < 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    coupled_size = opus_decoder_get_size(2 as libc::c_int);
    mono_size = opus_decoder_get_size(1 as libc::c_int);
    return align(::core::mem::size_of::<OpusMSDecoder>() as libc::c_ulong as libc::c_int)
        + nb_coupled_streams * align(coupled_size)
        + (nb_streams - nb_coupled_streams) * align(mono_size);
}
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn opus_multistream_decoder_init(
    mut st: *mut OpusMSDecoder,
    Fs: i32,
    channels: libc::c_int,
    streams: libc::c_int,
    coupled_streams: libc::c_int,
    mapping: *const libc::c_uchar,
) -> libc::c_int {
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if channels > 255 as libc::c_int
        || channels < 1 as libc::c_int
        || coupled_streams > streams
        || streams < 1 as libc::c_int
        || coupled_streams < 0 as libc::c_int
        || streams > 255 as libc::c_int - coupled_streams
    {
        return OPUS_BAD_ARG;
    }
    (*st).layout.nb_channels = channels;
    (*st).layout.nb_streams = streams;
    (*st).layout.nb_coupled_streams = coupled_streams;
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_channels {
        (*st).layout.mapping[i as usize] = *mapping.offset(i as isize);
        i += 1;
    }
    if validate_layout(&mut (*st).layout) == 0 {
        return OPUS_BAD_ARG;
    }
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSDecoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    coupled_size = opus_decoder_get_size(2 as libc::c_int);
    mono_size = opus_decoder_get_size(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_coupled_streams {
        ret = opus_decoder_init(ptr as *mut OpusDecoder, Fs, 2 as libc::c_int);
        if ret != OPUS_OK {
            return ret;
        }
        ptr = ptr.offset(align(coupled_size) as isize);
        i += 1;
    }
    while i < (*st).layout.nb_streams {
        ret = opus_decoder_init(ptr as *mut OpusDecoder, Fs, 1 as libc::c_int);
        if ret != OPUS_OK {
            return ret;
        }
        ptr = ptr.offset(align(mono_size) as isize);
        i += 1;
    }
    return OPUS_OK;
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn opus_multistream_decoder_create(
    Fs: i32,
    channels: libc::c_int,
    streams: libc::c_int,
    coupled_streams: libc::c_int,
    mapping: *const libc::c_uchar,
    error: *mut libc::c_int,
) -> *mut OpusMSDecoder {
    let mut ret: libc::c_int = 0;
    let mut st: *mut OpusMSDecoder = 0 as *mut OpusMSDecoder;
    if channels > 255 as libc::c_int
        || channels < 1 as libc::c_int
        || coupled_streams > streams
        || streams < 1 as libc::c_int
        || coupled_streams < 0 as libc::c_int
        || streams > 255 as libc::c_int - coupled_streams
    {
        if !error.is_null() {
            *error = OPUS_BAD_ARG;
        }
        return NULL as *mut OpusMSDecoder;
    }
    st = opus_alloc(opus_multistream_decoder_get_size(streams, coupled_streams) as size_t)
        as *mut OpusMSDecoder;
    if st.is_null() {
        if !error.is_null() {
            *error = OPUS_ALLOC_FAIL;
        }
        return NULL as *mut OpusMSDecoder;
    }
    ret = opus_multistream_decoder_init(st, Fs, channels, streams, coupled_streams, mapping);
    if !error.is_null() {
        *error = ret;
    }
    if ret != OPUS_OK {
        opus_free(st as *mut libc::c_void);
        st = NULL as *mut OpusMSDecoder;
    }
    return st;
}
#[c2rust::src_loc = "149:1"]
unsafe extern "C" fn opus_multistream_packet_validate(
    mut data: *const libc::c_uchar,
    mut len: i32,
    nb_streams: libc::c_int,
    Fs: i32,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut toc: libc::c_uchar = 0;
    let mut size: [i16; 48] = [0; 48];
    let mut samples: libc::c_int = 0 as libc::c_int;
    let mut packet_offset: i32 = 0;
    s = 0 as libc::c_int;
    while s < nb_streams {
        let mut tmp_samples: libc::c_int = 0;
        if len <= 0 as libc::c_int {
            return OPUS_INVALID_PACKET;
        }
        count = opus_packet_parse_impl(
            data,
            len,
            (s != nb_streams - 1 as libc::c_int) as libc::c_int,
            &mut toc,
            NULL as *mut *const libc::c_uchar,
            size.as_mut_ptr(),
            NULL as *mut libc::c_int,
            &mut packet_offset,
        );
        if count < 0 as libc::c_int {
            return count;
        }
        tmp_samples = opus_packet_get_nb_samples(data, packet_offset, Fs);
        if s != 0 as libc::c_int && samples != tmp_samples {
            return OPUS_INVALID_PACKET;
        }
        samples = tmp_samples;
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        s += 1;
    }
    return samples;
}
#[no_mangle]
#[c2rust::src_loc = "178:1"]
pub unsafe extern "C" fn opus_multistream_decode_native(
    st: *mut OpusMSDecoder,
    mut data: *const libc::c_uchar,
    mut len: i32,
    pcm: *mut libc::c_void,
    copy_channel_out: opus_copy_channel_out_func,
    mut frame_size: libc::c_int,
    decode_fec: libc::c_int,
    soft_clip: libc::c_int,
    user_data: *mut libc::c_void,
) -> libc::c_int {
    let mut Fs: i32 = 0;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut do_plc: libc::c_int = 0 as libc::c_int;
    validate_ms_decoder(st);
    if frame_size <= 0 as libc::c_int {
        return OPUS_BAD_ARG;
    }
    if !(opus_multistream_decoder_ctl(
        st,
        4029 as libc::c_int,
        (&mut Fs as *mut i32).offset(
            (&mut Fs as *mut i32).offset_from(&mut Fs as *mut i32) as libc::c_long as isize
        ),
    ) == 0 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: (opus_multistream_decoder_ctl(st, 4029, ((&Fs) + ((&Fs) - (i32*)(&Fs))))) == OPUS_OK\0"
                as *const u8 as *const libc::c_char,
            b"src/opus_multistream_decoder.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
        );
    }
    frame_size = if frame_size < Fs / 25 as libc::c_int * 3 as libc::c_int {
        frame_size
    } else {
        Fs / 25 as libc::c_int * 3 as libc::c_int
    };
    let vla = (2 as libc::c_int * frame_size) as usize;
    let mut buf: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSDecoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    coupled_size = opus_decoder_get_size(2 as libc::c_int);
    mono_size = opus_decoder_get_size(1 as libc::c_int);
    if len == 0 as libc::c_int {
        do_plc = 1 as libc::c_int;
    }
    if len < 0 as libc::c_int {
        return OPUS_BAD_ARG;
    }
    if do_plc == 0 && len < 2 as libc::c_int * (*st).layout.nb_streams - 1 as libc::c_int {
        return OPUS_INVALID_PACKET;
    }
    if do_plc == 0 {
        let ret: libc::c_int =
            opus_multistream_packet_validate(data, len, (*st).layout.nb_streams, Fs);
        if ret < 0 as libc::c_int {
            return ret;
        } else {
            if ret > frame_size {
                return OPUS_BUFFER_TOO_SMALL;
            }
        }
    }
    s = 0 as libc::c_int;
    while s < (*st).layout.nb_streams {
        let mut dec: *mut OpusDecoder = 0 as *mut OpusDecoder;
        let mut packet_offset: i32 = 0;
        let mut ret_0: libc::c_int = 0;
        dec = ptr as *mut OpusDecoder;
        ptr = ptr.offset(
            (if s < (*st).layout.nb_coupled_streams {
                align(coupled_size)
            } else {
                align(mono_size)
            }) as isize,
        );
        if do_plc == 0 && len <= 0 as libc::c_int {
            return OPUS_INTERNAL_ERROR;
        }
        packet_offset = 0 as libc::c_int;
        ret_0 = opus_decode_native(
            dec,
            data,
            len,
            buf.as_mut_ptr(),
            frame_size,
            decode_fec,
            (s != (*st).layout.nb_streams - 1 as libc::c_int) as libc::c_int,
            &mut packet_offset,
            soft_clip,
        );
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        if ret_0 <= 0 as libc::c_int {
            return ret_0;
        }
        frame_size = ret_0;
        if s < (*st).layout.nb_coupled_streams {
            let mut chan: libc::c_int = 0;
            let mut prev: libc::c_int = 0;
            prev = -(1 as libc::c_int);
            loop {
                chan = get_left_channel(&mut (*st).layout, s, prev);
                if !(chan != -(1 as libc::c_int)) {
                    break;
                }
                (Some(copy_channel_out.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    pcm,
                    (*st).layout.nb_channels,
                    chan,
                    buf.as_mut_ptr(),
                    2 as libc::c_int,
                    frame_size,
                    user_data,
                );
                prev = chan;
            }
            prev = -(1 as libc::c_int);
            loop {
                chan = get_right_channel(&mut (*st).layout, s, prev);
                if !(chan != -(1 as libc::c_int)) {
                    break;
                }
                (Some(copy_channel_out.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    pcm,
                    (*st).layout.nb_channels,
                    chan,
                    buf.as_mut_ptr().offset(1 as libc::c_int as isize),
                    2 as libc::c_int,
                    frame_size,
                    user_data,
                );
                prev = chan;
            }
        } else {
            let mut chan_0: libc::c_int = 0;
            let mut prev_0: libc::c_int = 0;
            prev_0 = -(1 as libc::c_int);
            loop {
                chan_0 = get_mono_channel(&mut (*st).layout, s, prev_0);
                if !(chan_0 != -(1 as libc::c_int)) {
                    break;
                }
                (Some(copy_channel_out.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    pcm,
                    (*st).layout.nb_channels,
                    chan_0,
                    buf.as_mut_ptr(),
                    1 as libc::c_int,
                    frame_size,
                    user_data,
                );
                prev_0 = chan_0;
            }
        }
        s += 1;
    }
    c = 0 as libc::c_int;
    while c < (*st).layout.nb_channels {
        if (*st).layout.mapping[c as usize] as libc::c_int == 255 as libc::c_int {
            (Some(copy_channel_out.expect("non-null function pointer")))
                .expect("non-null function pointer")(
                pcm,
                (*st).layout.nb_channels,
                c,
                NULL as *const opus_val16,
                0 as libc::c_int,
                frame_size,
                user_data,
            );
        }
        c += 1;
    }
    return frame_size;
}
#[c2rust::src_loc = "307:1"]
unsafe extern "C" fn opus_copy_channel_out_float(
    dst: *mut libc::c_void,
    dst_stride: libc::c_int,
    dst_channel: libc::c_int,
    src: *const opus_val16,
    src_stride: libc::c_int,
    frame_size: libc::c_int,
    _user_data: *mut libc::c_void,
) {
    let mut float_dst: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: i32 = 0;
    float_dst = dst as *mut libc::c_float;
    if !src.is_null() {
        i = 0 as libc::c_int;
        while i < frame_size {
            *float_dst.offset((i * dst_stride + dst_channel) as isize) =
                *src.offset((i * src_stride) as isize);
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < frame_size {
            *float_dst.offset((i * dst_stride + dst_channel) as isize) =
                0 as libc::c_int as libc::c_float;
            i += 1;
        }
    };
}
#[c2rust::src_loc = "338:1"]
unsafe extern "C" fn opus_copy_channel_out_short(
    dst: *mut libc::c_void,
    dst_stride: libc::c_int,
    dst_channel: libc::c_int,
    src: *const opus_val16,
    src_stride: libc::c_int,
    frame_size: libc::c_int,
    _user_data: *mut libc::c_void,
) {
    let mut short_dst: *mut i16 = 0 as *mut i16;
    let mut i: i32 = 0;
    short_dst = dst as *mut i16;
    if !src.is_null() {
        i = 0 as libc::c_int;
        while i < frame_size {
            *short_dst.offset((i * dst_stride + dst_channel) as isize) =
                FLOAT2INT16(*src.offset((i * src_stride) as isize));
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < frame_size {
            *short_dst.offset((i * dst_stride + dst_channel) as isize) = 0 as libc::c_int as i16;
            i += 1;
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "395:1"]
pub unsafe extern "C" fn opus_multistream_decode(
    st: *mut OpusMSDecoder,
    data: *const libc::c_uchar,
    len: i32,
    pcm: *mut i16,
    frame_size: libc::c_int,
    decode_fec: libc::c_int,
) -> libc::c_int {
    return opus_multistream_decode_native(
        st,
        data,
        len,
        pcm as *mut libc::c_void,
        Some(
            opus_copy_channel_out_short
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                    *const opus_val16,
                    libc::c_int,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
        frame_size,
        decode_fec,
        1 as libc::c_int,
        NULL as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "402:1"]
pub unsafe extern "C" fn opus_multistream_decode_float(
    st: *mut OpusMSDecoder,
    data: *const libc::c_uchar,
    len: i32,
    pcm: *mut opus_val16,
    frame_size: libc::c_int,
    decode_fec: libc::c_int,
) -> libc::c_int {
    return opus_multistream_decode_native(
        st,
        data,
        len,
        pcm as *mut libc::c_void,
        Some(
            opus_copy_channel_out_float
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                    *const opus_val16,
                    libc::c_int,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
        frame_size,
        decode_fec,
        0 as libc::c_int,
        NULL as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "416:1"]
pub unsafe extern "C" fn opus_multistream_decoder_ctl_va_list(
    st: *mut OpusMSDecoder,
    request: libc::c_int,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let current_block: u64;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = OPUS_OK;
    coupled_size = opus_decoder_get_size(2 as libc::c_int);
    mono_size = opus_decoder_get_size(1 as libc::c_int);
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSDecoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    match request {
        OPUS_GET_BANDWIDTH_REQUEST
        | OPUS_GET_SAMPLE_RATE_REQUEST
        | OPUS_GET_GAIN_REQUEST
        | OPUS_GET_LAST_PACKET_DURATION_REQUEST
        | OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST => {
            let mut dec: *mut OpusDecoder = 0 as *mut OpusDecoder;
            let value: *mut i32 = ap.arg::<*mut i32>();
            dec = ptr as *mut OpusDecoder;
            ret = opus_decoder_ctl(dec, request, value);
            current_block = 7343950298149844727;
        }
        OPUS_GET_FINAL_RANGE_REQUEST => {
            let mut s: libc::c_int = 0;
            let value_0: *mut u32 = ap.arg::<*mut u32>();
            let mut tmp: u32 = 0;
            if value_0.is_null() {
                current_block = 15933654591644784431;
            } else {
                *value_0 = 0 as libc::c_int as u32;
                s = 0 as libc::c_int;
                while s < (*st).layout.nb_streams {
                    let mut dec_0: *mut OpusDecoder = 0 as *mut OpusDecoder;
                    dec_0 = ptr as *mut OpusDecoder;
                    if s < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize);
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize);
                    }
                    ret = opus_decoder_ctl(dec_0, request, &mut tmp as *mut u32);
                    if ret != OPUS_OK {
                        break;
                    }
                    *value_0 ^= tmp;
                    s += 1;
                }
                current_block = 7343950298149844727;
            }
        }
        OPUS_RESET_STATE => {
            let mut s_0: libc::c_int = 0;
            s_0 = 0 as libc::c_int;
            while s_0 < (*st).layout.nb_streams {
                let mut dec_1: *mut OpusDecoder = 0 as *mut OpusDecoder;
                dec_1 = ptr as *mut OpusDecoder;
                if s_0 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize);
                } else {
                    ptr = ptr.offset(align(mono_size) as isize);
                }
                ret = opus_decoder_ctl(dec_1, OPUS_RESET_STATE);
                if ret != OPUS_OK {
                    break;
                }
                s_0 += 1;
            }
            current_block = 7343950298149844727;
        }
        OPUS_MULTISTREAM_GET_DECODER_STATE_REQUEST => {
            let mut s_1: libc::c_int = 0;
            let mut stream_id: i32 = 0;
            let mut value_1: *mut *mut OpusDecoder = 0 as *mut *mut OpusDecoder;
            stream_id = ap.arg::<i32>();
            if stream_id < 0 as libc::c_int || stream_id >= (*st).layout.nb_streams {
                current_block = 15933654591644784431;
            } else {
                value_1 = ap.arg::<*mut *mut OpusDecoder>();
                if value_1.is_null() {
                    current_block = 15933654591644784431;
                } else {
                    s_1 = 0 as libc::c_int;
                    while s_1 < stream_id {
                        if s_1 < (*st).layout.nb_coupled_streams {
                            ptr = ptr.offset(align(coupled_size) as isize);
                        } else {
                            ptr = ptr.offset(align(mono_size) as isize);
                        }
                        s_1 += 1;
                    }
                    *value_1 = ptr as *mut OpusDecoder;
                    current_block = 7343950298149844727;
                }
            }
        }
        OPUS_SET_GAIN_REQUEST | OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST => {
            let mut s_2: libc::c_int = 0;
            let value_2: i32 = ap.arg::<i32>();
            s_2 = 0 as libc::c_int;
            while s_2 < (*st).layout.nb_streams {
                let mut dec_2: *mut OpusDecoder = 0 as *mut OpusDecoder;
                dec_2 = ptr as *mut OpusDecoder;
                if s_2 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize);
                } else {
                    ptr = ptr.offset(align(mono_size) as isize);
                }
                ret = opus_decoder_ctl(dec_2, request, value_2);
                if ret != OPUS_OK {
                    break;
                }
                s_2 += 1;
            }
            current_block = 7343950298149844727;
        }
        _ => {
            ret = OPUS_UNIMPLEMENTED;
            current_block = 7343950298149844727;
        }
    }
    match current_block {
        15933654591644784431 => return OPUS_BAD_ARG,
        _ => return ret,
    };
}
#[no_mangle]
#[c2rust::src_loc = "536:1"]
pub unsafe extern "C" fn opus_multistream_decoder_ctl(
    st: *mut OpusMSDecoder,
    request: libc::c_int,
    args: ...
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    ret = opus_multistream_decoder_ctl_va_list(st, request, ap.as_va_list());
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "546:1"]
pub unsafe extern "C" fn opus_multistream_decoder_destroy(st: *mut OpusMSDecoder) {
    opus_free(st as *mut libc::c_void);
}
