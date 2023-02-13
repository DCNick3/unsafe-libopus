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

#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "73:8"]
pub struct OpusMSDecoder {
    pub layout: ChannelLayout,
}

pub type opus_copy_channel_out_func = Option<
    unsafe fn(
        *mut core::ffi::c_void,
        i32,
        i32,
        *const opus_val16,
        i32,
        i32,
        *mut core::ffi::c_void,
    ) -> (),
>;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:34"]
pub mod arch_h {
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = f32;
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = f32;
    #[c2rust::src_loc = "57:9"]
    pub const CELT_SIG_SCALE: f32 = 32768.0f32;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:34"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = u64;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: i32 = 0 as i32;
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
    pub const OPUS_GET_BANDWIDTH_REQUEST: i32 = 4009;
    #[c2rust::src_loc = "156:9"]
    pub const OPUS_GET_SAMPLE_RATE_REQUEST: i32 = 4029;
    #[c2rust::src_loc = "160:9"]
    pub const OPUS_GET_GAIN_REQUEST: i32 = 4045;
    #[c2rust::src_loc = "163:9"]
    pub const OPUS_GET_LAST_PACKET_DURATION_REQUEST: i32 = 4039;
    #[c2rust::src_loc = "170:9"]
    pub const OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST: i32 = 4047;
    #[c2rust::src_loc = "157:9"]
    pub const OPUS_GET_FINAL_RANGE_REQUEST: i32 = 4031;
    #[c2rust::src_loc = "662:9"]
    pub const OPUS_RESET_STATE: i32 = 4028 as i32;
    #[c2rust::src_loc = "159:9"]
    pub const OPUS_SET_GAIN_REQUEST: i32 = 4034;
    #[c2rust::src_loc = "169:9"]
    pub const OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST: i32 = 4046;
    #[c2rust::src_loc = "56:9"]
    pub const OPUS_UNIMPLEMENTED: i32 = -(5 as i32);
    #[c2rust::src_loc = "54:9"]
    pub const OPUS_INVALID_PACKET: i32 = -(4 as i32);
    #[c2rust::src_loc = "46:9"]
    pub const OPUS_OK: i32 = 0 as i32;
    #[c2rust::src_loc = "50:9"]
    pub const OPUS_BUFFER_TOO_SMALL: i32 = -(2 as i32);
    #[c2rust::src_loc = "48:9"]
    pub const OPUS_BAD_ARG: i32 = -(1 as i32);
    #[c2rust::src_loc = "52:9"]
    pub const OPUS_INTERNAL_ERROR: i32 = -(3 as i32);
    #[c2rust::src_loc = "60:9"]
    pub const OPUS_ALLOC_FAIL: i32 = -(7 as i32);
}
use self::arch_h::opus_val16;
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::opus_defines_h::{
    OPUS_ALLOC_FAIL, OPUS_BAD_ARG, OPUS_BUFFER_TOO_SMALL, OPUS_GET_BANDWIDTH_REQUEST,
    OPUS_GET_FINAL_RANGE_REQUEST, OPUS_GET_GAIN_REQUEST, OPUS_GET_LAST_PACKET_DURATION_REQUEST,
    OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST, OPUS_GET_SAMPLE_RATE_REQUEST, OPUS_INTERNAL_ERROR,
    OPUS_INVALID_PACKET, OPUS_OK, OPUS_RESET_STATE, OPUS_SET_GAIN_REQUEST,
    OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST, OPUS_UNIMPLEMENTED,
};
pub use self::stdarg_h::va_list;
pub use self::stddef_h::{size_t, NULL};
use crate::celt::celt::celt_fatal;
use crate::celt::float_cast::FLOAT2INT16;
use crate::src::opus::opus_packet_parse_impl;
use crate::src::opus_decoder::opus_decode_native;
use crate::src::opus_multistream::{
    get_left_channel, get_mono_channel, get_right_channel, validate_layout, ChannelLayout,
};
use crate::src::opus_private::align;
use crate::varargs::VarArgs;
use crate::{
    opus_decoder_ctl, opus_decoder_get_size, opus_decoder_init, opus_multistream_decoder_ctl,
    opus_packet_get_nb_samples, OpusDecoder,
};

pub const OPUS_MULTISTREAM_GET_DECODER_STATE_REQUEST: i32 = 5122;

#[c2rust::src_loc = "43:1"]
unsafe fn validate_ms_decoder(st: *mut OpusMSDecoder) {
    validate_layout(&mut (*st).layout);
}
#[c2rust::src_loc = "53:1"]
pub unsafe fn opus_multistream_decoder_get_size(nb_streams: i32, nb_coupled_streams: i32) -> i32 {
    let mut coupled_size: i32 = 0;
    let mut mono_size: i32 = 0;
    if nb_streams < 1 as i32 || nb_coupled_streams > nb_streams || nb_coupled_streams < 0 as i32 {
        return 0 as i32;
    }
    coupled_size = opus_decoder_get_size(2 as i32);
    mono_size = opus_decoder_get_size(1 as i32);
    return align(::core::mem::size_of::<OpusMSDecoder>() as u64 as i32)
        + nb_coupled_streams * align(coupled_size)
        + (nb_streams - nb_coupled_streams) * align(mono_size);
}
#[c2rust::src_loc = "66:1"]
pub unsafe fn opus_multistream_decoder_init(
    mut st: *mut OpusMSDecoder,
    Fs: i32,
    channels: i32,
    streams: i32,
    coupled_streams: i32,
    mapping: *const u8,
) -> i32 {
    let mut coupled_size: i32 = 0;
    let mut mono_size: i32 = 0;
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    let mut ptr: *mut i8 = 0 as *mut i8;
    if channels > 255 as i32
        || channels < 1 as i32
        || coupled_streams > streams
        || streams < 1 as i32
        || coupled_streams < 0 as i32
        || streams > 255 as i32 - coupled_streams
    {
        return OPUS_BAD_ARG;
    }
    (*st).layout.nb_channels = channels;
    (*st).layout.nb_streams = streams;
    (*st).layout.nb_coupled_streams = coupled_streams;
    i = 0 as i32;
    while i < (*st).layout.nb_channels {
        (*st).layout.mapping[i as usize] = *mapping.offset(i as isize);
        i += 1;
    }
    if validate_layout(&mut (*st).layout) == 0 {
        return OPUS_BAD_ARG;
    }
    ptr = (st as *mut i8)
        .offset(align(::core::mem::size_of::<OpusMSDecoder>() as u64 as i32) as isize);
    coupled_size = opus_decoder_get_size(2 as i32);
    mono_size = opus_decoder_get_size(1 as i32);
    i = 0 as i32;
    while i < (*st).layout.nb_coupled_streams {
        ret = opus_decoder_init(ptr as *mut OpusDecoder, Fs, 2 as i32);
        if ret != OPUS_OK {
            return ret;
        }
        ptr = ptr.offset(align(coupled_size) as isize);
        i += 1;
    }
    while i < (*st).layout.nb_streams {
        ret = opus_decoder_init(ptr as *mut OpusDecoder, Fs, 1 as i32);
        if ret != OPUS_OK {
            return ret;
        }
        ptr = ptr.offset(align(mono_size) as isize);
        i += 1;
    }
    return OPUS_OK;
}
#[c2rust::src_loc = "113:1"]
pub unsafe fn opus_multistream_decoder_create(
    Fs: i32,
    channels: i32,
    streams: i32,
    coupled_streams: i32,
    mapping: *const u8,
    error: *mut i32,
) -> *mut OpusMSDecoder {
    let mut ret: i32 = 0;
    let mut st: *mut OpusMSDecoder = 0 as *mut OpusMSDecoder;
    if channels > 255 as i32
        || channels < 1 as i32
        || coupled_streams > streams
        || streams < 1 as i32
        || coupled_streams < 0 as i32
        || streams > 255 as i32 - coupled_streams
    {
        if !error.is_null() {
            *error = OPUS_BAD_ARG;
        }
        return NULL as *mut OpusMSDecoder;
    }
    st = malloc(opus_multistream_decoder_get_size(streams, coupled_streams) as size_t)
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
        free(st as *mut core::ffi::c_void);
        st = NULL as *mut OpusMSDecoder;
    }
    return st;
}
#[c2rust::src_loc = "149:1"]
unsafe fn opus_multistream_packet_validate(
    mut data: *const u8,
    mut len: i32,
    nb_streams: i32,
    Fs: i32,
) -> i32 {
    let mut s: i32 = 0;
    let mut count: i32 = 0;
    let mut toc: u8 = 0;
    let mut size: [i16; 48] = [0; 48];
    let mut samples: i32 = 0 as i32;
    let mut packet_offset: i32 = 0;
    s = 0 as i32;
    while s < nb_streams {
        let mut tmp_samples: i32 = 0;
        if len <= 0 as i32 {
            return OPUS_INVALID_PACKET;
        }
        count = opus_packet_parse_impl(
            data,
            len,
            (s != nb_streams - 1 as i32) as i32,
            &mut toc,
            NULL as *mut *const u8,
            size.as_mut_ptr(),
            NULL as *mut i32,
            &mut packet_offset,
        );
        if count < 0 as i32 {
            return count;
        }
        tmp_samples = opus_packet_get_nb_samples(data, packet_offset, Fs);
        if s != 0 as i32 && samples != tmp_samples {
            return OPUS_INVALID_PACKET;
        }
        samples = tmp_samples;
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        s += 1;
    }
    return samples;
}
#[c2rust::src_loc = "178:1"]
pub unsafe fn opus_multistream_decode_native(
    st: *mut OpusMSDecoder,
    mut data: *const u8,
    mut len: i32,
    pcm: *mut core::ffi::c_void,
    copy_channel_out: opus_copy_channel_out_func,
    mut frame_size: i32,
    decode_fec: i32,
    soft_clip: i32,
    user_data: *mut core::ffi::c_void,
) -> i32 {
    let mut Fs: i32 = 0;
    let mut coupled_size: i32 = 0;
    let mut mono_size: i32 = 0;
    let mut s: i32 = 0;
    let mut c: i32 = 0;
    let mut ptr: *mut i8 = 0 as *mut i8;
    let mut do_plc: i32 = 0 as i32;
    validate_ms_decoder(st);
    if frame_size <= 0 as i32 {
        return OPUS_BAD_ARG;
    }
    if !(opus_multistream_decoder_ctl!(
        st,
        4029 as i32,
        (&mut Fs as *mut i32)
            .offset((&mut Fs as *mut i32).offset_from(&mut Fs as *mut i32) as i64 as isize),
    ) == 0 as i32)
    {
        celt_fatal(
            b"assertion failed: (opus_multistream_decoder_ctl(st, 4029, ((&Fs) + ((&Fs) - (i32*)(&Fs))))) == OPUS_OK\0"
                as *const u8 as *const i8,
            b"src/opus_multistream_decoder.c\0" as *const u8 as *const i8,
            206 as i32,
        );
    }
    frame_size = if frame_size < Fs / 25 as i32 * 3 as i32 {
        frame_size
    } else {
        Fs / 25 as i32 * 3 as i32
    };
    let vla = (2 as i32 * frame_size) as usize;
    let mut buf: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    ptr = (st as *mut i8)
        .offset(align(::core::mem::size_of::<OpusMSDecoder>() as u64 as i32) as isize);
    coupled_size = opus_decoder_get_size(2 as i32);
    mono_size = opus_decoder_get_size(1 as i32);
    if len == 0 as i32 {
        do_plc = 1 as i32;
    }
    if len < 0 as i32 {
        return OPUS_BAD_ARG;
    }
    if do_plc == 0 && len < 2 as i32 * (*st).layout.nb_streams - 1 as i32 {
        return OPUS_INVALID_PACKET;
    }
    if do_plc == 0 {
        let ret: i32 = opus_multistream_packet_validate(data, len, (*st).layout.nb_streams, Fs);
        if ret < 0 as i32 {
            return ret;
        } else {
            if ret > frame_size {
                return OPUS_BUFFER_TOO_SMALL;
            }
        }
    }
    s = 0 as i32;
    while s < (*st).layout.nb_streams {
        let mut dec: *mut OpusDecoder = 0 as *mut OpusDecoder;
        let mut packet_offset: i32 = 0;
        let mut ret_0: i32 = 0;
        dec = ptr as *mut OpusDecoder;
        ptr = ptr.offset(
            (if s < (*st).layout.nb_coupled_streams {
                align(coupled_size)
            } else {
                align(mono_size)
            }) as isize,
        );
        if do_plc == 0 && len <= 0 as i32 {
            return OPUS_INTERNAL_ERROR;
        }
        packet_offset = 0 as i32;
        ret_0 = opus_decode_native(
            dec,
            data,
            len,
            buf.as_mut_ptr(),
            frame_size,
            decode_fec,
            (s != (*st).layout.nb_streams - 1 as i32) as i32,
            &mut packet_offset,
            soft_clip,
        );
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        if ret_0 <= 0 as i32 {
            return ret_0;
        }
        frame_size = ret_0;
        if s < (*st).layout.nb_coupled_streams {
            let mut chan: i32 = 0;
            let mut prev: i32 = 0;
            prev = -(1 as i32);
            loop {
                chan = get_left_channel(&mut (*st).layout, s, prev);
                if !(chan != -(1 as i32)) {
                    break;
                }
                (Some(copy_channel_out.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    pcm,
                    (*st).layout.nb_channels,
                    chan,
                    buf.as_mut_ptr(),
                    2 as i32,
                    frame_size,
                    user_data,
                );
                prev = chan;
            }
            prev = -(1 as i32);
            loop {
                chan = get_right_channel(&mut (*st).layout, s, prev);
                if !(chan != -(1 as i32)) {
                    break;
                }
                (Some(copy_channel_out.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    pcm,
                    (*st).layout.nb_channels,
                    chan,
                    buf.as_mut_ptr().offset(1 as i32 as isize),
                    2 as i32,
                    frame_size,
                    user_data,
                );
                prev = chan;
            }
        } else {
            let mut chan_0: i32 = 0;
            let mut prev_0: i32 = 0;
            prev_0 = -(1 as i32);
            loop {
                chan_0 = get_mono_channel(&mut (*st).layout, s, prev_0);
                if !(chan_0 != -(1 as i32)) {
                    break;
                }
                (Some(copy_channel_out.expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    pcm,
                    (*st).layout.nb_channels,
                    chan_0,
                    buf.as_mut_ptr(),
                    1 as i32,
                    frame_size,
                    user_data,
                );
                prev_0 = chan_0;
            }
        }
        s += 1;
    }
    c = 0 as i32;
    while c < (*st).layout.nb_channels {
        if (*st).layout.mapping[c as usize] as i32 == 255 as i32 {
            (Some(copy_channel_out.expect("non-null function pointer")))
                .expect("non-null function pointer")(
                pcm,
                (*st).layout.nb_channels,
                c,
                NULL as *const opus_val16,
                0 as i32,
                frame_size,
                user_data,
            );
        }
        c += 1;
    }
    return frame_size;
}
#[c2rust::src_loc = "307:1"]
unsafe fn opus_copy_channel_out_float(
    dst: *mut core::ffi::c_void,
    dst_stride: i32,
    dst_channel: i32,
    src: *const opus_val16,
    src_stride: i32,
    frame_size: i32,
    _user_data: *mut core::ffi::c_void,
) {
    let mut float_dst: *mut f32 = 0 as *mut f32;
    let mut i: i32 = 0;
    float_dst = dst as *mut f32;
    if !src.is_null() {
        i = 0 as i32;
        while i < frame_size {
            *float_dst.offset((i * dst_stride + dst_channel) as isize) =
                *src.offset((i * src_stride) as isize);
            i += 1;
        }
    } else {
        i = 0 as i32;
        while i < frame_size {
            *float_dst.offset((i * dst_stride + dst_channel) as isize) = 0 as i32 as f32;
            i += 1;
        }
    };
}
#[c2rust::src_loc = "338:1"]
unsafe fn opus_copy_channel_out_short(
    dst: *mut core::ffi::c_void,
    dst_stride: i32,
    dst_channel: i32,
    src: *const opus_val16,
    src_stride: i32,
    frame_size: i32,
    _user_data: *mut core::ffi::c_void,
) {
    let mut short_dst: *mut i16 = 0 as *mut i16;
    let mut i: i32 = 0;
    short_dst = dst as *mut i16;
    if !src.is_null() {
        i = 0 as i32;
        while i < frame_size {
            *short_dst.offset((i * dst_stride + dst_channel) as isize) =
                FLOAT2INT16(*src.offset((i * src_stride) as isize));
            i += 1;
        }
    } else {
        i = 0 as i32;
        while i < frame_size {
            *short_dst.offset((i * dst_stride + dst_channel) as isize) = 0 as i32 as i16;
            i += 1;
        }
    };
}
#[c2rust::src_loc = "395:1"]
pub unsafe fn opus_multistream_decode(
    st: *mut OpusMSDecoder,
    data: *const u8,
    len: i32,
    pcm: *mut i16,
    frame_size: i32,
    decode_fec: i32,
) -> i32 {
    return opus_multistream_decode_native(
        st,
        data,
        len,
        pcm as *mut core::ffi::c_void,
        Some(
            opus_copy_channel_out_short
                as unsafe fn(
                    *mut core::ffi::c_void,
                    i32,
                    i32,
                    *const opus_val16,
                    i32,
                    i32,
                    *mut core::ffi::c_void,
                ) -> (),
        ),
        frame_size,
        decode_fec,
        1 as i32,
        NULL as *mut core::ffi::c_void,
    );
}
#[c2rust::src_loc = "402:1"]
pub unsafe fn opus_multistream_decode_float(
    st: *mut OpusMSDecoder,
    data: *const u8,
    len: i32,
    pcm: *mut opus_val16,
    frame_size: i32,
    decode_fec: i32,
) -> i32 {
    return opus_multistream_decode_native(
        st,
        data,
        len,
        pcm as *mut core::ffi::c_void,
        Some(
            opus_copy_channel_out_float
                as unsafe fn(
                    *mut core::ffi::c_void,
                    i32,
                    i32,
                    *const opus_val16,
                    i32,
                    i32,
                    *mut core::ffi::c_void,
                ) -> (),
        ),
        frame_size,
        decode_fec,
        0 as i32,
        NULL as *mut core::ffi::c_void,
    );
}
#[c2rust::src_loc = "416:1"]
pub unsafe fn opus_multistream_decoder_ctl_va_list(
    st: *mut OpusMSDecoder,
    request: i32,
    mut ap: VarArgs,
) -> i32 {
    let current_block: u64;
    let mut coupled_size: i32 = 0;
    let mut mono_size: i32 = 0;
    let mut ptr: *mut i8 = 0 as *mut i8;
    let mut ret: i32 = OPUS_OK;
    coupled_size = opus_decoder_get_size(2 as i32);
    mono_size = opus_decoder_get_size(1 as i32);
    ptr = (st as *mut i8)
        .offset(align(::core::mem::size_of::<OpusMSDecoder>() as u64 as i32) as isize);
    match request {
        OPUS_GET_BANDWIDTH_REQUEST
        | OPUS_GET_SAMPLE_RATE_REQUEST
        | OPUS_GET_GAIN_REQUEST
        | OPUS_GET_LAST_PACKET_DURATION_REQUEST
        | OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST => {
            let mut dec: *mut OpusDecoder = 0 as *mut OpusDecoder;
            let value: *mut i32 = ap.arg::<*mut i32>();
            dec = ptr as *mut OpusDecoder;
            ret = opus_decoder_ctl!(dec, request, value);
            current_block = 7343950298149844727;
        }
        OPUS_GET_FINAL_RANGE_REQUEST => {
            let mut s: i32 = 0;
            let value_0: *mut u32 = ap.arg::<*mut u32>();
            let mut tmp: u32 = 0;
            if value_0.is_null() {
                current_block = 15933654591644784431;
            } else {
                *value_0 = 0 as i32 as u32;
                s = 0 as i32;
                while s < (*st).layout.nb_streams {
                    let mut dec_0: *mut OpusDecoder = 0 as *mut OpusDecoder;
                    dec_0 = ptr as *mut OpusDecoder;
                    if s < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize);
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize);
                    }
                    ret = opus_decoder_ctl!(dec_0, request, &mut tmp as *mut u32);
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
            let mut s_0: i32 = 0;
            s_0 = 0 as i32;
            while s_0 < (*st).layout.nb_streams {
                let mut dec_1: *mut OpusDecoder = 0 as *mut OpusDecoder;
                dec_1 = ptr as *mut OpusDecoder;
                if s_0 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize);
                } else {
                    ptr = ptr.offset(align(mono_size) as isize);
                }
                ret = opus_decoder_ctl!(dec_1, OPUS_RESET_STATE);
                if ret != OPUS_OK {
                    break;
                }
                s_0 += 1;
            }
            current_block = 7343950298149844727;
        }
        OPUS_MULTISTREAM_GET_DECODER_STATE_REQUEST => {
            let mut s_1: i32 = 0;
            let mut stream_id: i32 = 0;
            let mut value_1: *mut *mut OpusDecoder = 0 as *mut *mut OpusDecoder;
            stream_id = ap.arg::<i32>();
            if stream_id < 0 as i32 || stream_id >= (*st).layout.nb_streams {
                current_block = 15933654591644784431;
            } else {
                value_1 = ap.arg::<*mut *mut OpusDecoder>();
                if value_1.is_null() {
                    current_block = 15933654591644784431;
                } else {
                    s_1 = 0 as i32;
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
            let mut s_2: i32 = 0;
            let value_2: i32 = ap.arg::<i32>();
            s_2 = 0 as i32;
            while s_2 < (*st).layout.nb_streams {
                let mut dec_2: *mut OpusDecoder = 0 as *mut OpusDecoder;
                dec_2 = ptr as *mut OpusDecoder;
                if s_2 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize);
                } else {
                    ptr = ptr.offset(align(mono_size) as isize);
                }
                ret = opus_decoder_ctl!(dec_2, request, value_2);
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
#[c2rust::src_loc = "536:1"]
pub unsafe fn opus_multistream_decoder_ctl_impl(
    st: *mut OpusMSDecoder,
    request: i32,
    args: VarArgs,
) -> i32 {
    let mut ret: i32 = 0;
    ret = opus_multistream_decoder_ctl_va_list(st, request, args);
    return ret;
}
#[macro_export]
macro_rules! opus_multistream_decoder_ctl {
    ($st:expr, $request:expr, $($arg:expr),*) => {
        $crate::opus_multistream_decoder_ctl_impl($st, $request, $crate::varargs!($($arg),*))
    };
    ($st:expr, $request:expr) => {
        opus_multistream_decoder_ctl!($st, $request,)
    };
    ($st:expr, $request:expr, $($arg:expr),*,) => {
        opus_multistream_decoder_ctl!($st, $request, $($arg),*)
    };
}
#[c2rust::src_loc = "546:1"]
pub unsafe fn opus_multistream_decoder_destroy(st: *mut OpusMSDecoder) {
    free(st as *mut core::ffi::c_void);
}
