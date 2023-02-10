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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus.h:32"]
pub mod opus_h {
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "399:16"]
        pub type OpusDecoder;
        #[c2rust::src_loc = "406:1"]
        pub fn opus_decoder_get_size(channels: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "440:1"]
        pub fn opus_decoder_init(
            st: *mut OpusDecoder,
            Fs: opus_int32,
            channels: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "507:1"]
        pub fn opus_decoder_ctl(st: *mut OpusDecoder, request: libc::c_int, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "584:1"]
        pub fn opus_packet_get_nb_samples(
            packet: *const libc::c_uchar,
            len: opus_int32,
            Fs: opus_int32,
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
        pub i: opus_int32,
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
    pub unsafe extern "C" fn align(mut i: libc::c_int) -> libc::c_int {
        let mut alignment: libc::c_uint = 8 as libc::c_ulong as libc::c_uint;
        return (i as libc::c_uint)
            .wrapping_add(alignment)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(alignment)
            .wrapping_mul(alignment) as libc::c_int;
    }
    use super::arch_h::{opus_val16, opus_val32};
    use super::opus_h::OpusDecoder;
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "149:1"]
        pub fn opus_decode_native(
            st: *mut OpusDecoder,
            data: *const libc::c_uchar,
            len: opus_int32,
            pcm: *mut opus_val16,
            frame_size: libc::c_int,
            decode_fec: libc::c_int,
            self_delimited: libc::c_int,
            packet_offset: *mut opus_int32,
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:34"]
pub mod arch_h {
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:34"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/os_support.h:38"]
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
pub use self::arch_h::{opus_val16, opus_val32};
pub use self::float_cast_h::{float2int, FLOAT2INT16};
pub use self::internal::{__builtin_va_list, __va_list_tag};
use self::opus_h::{
    opus_decoder_ctl, opus_decoder_get_size, opus_decoder_init, opus_packet_get_nb_samples,
    OpusDecoder,
};
pub use self::opus_private_h::{
    align, foo, get_left_channel, get_mono_channel, get_right_channel, opus_copy_channel_out_func,
    opus_decode_native, opus_packet_parse_impl, validate_layout, C2RustUnnamed, ChannelLayout,
    OpusMSDecoder,
};
pub use self::opus_types_h::{opus_int16, opus_int32, opus_uint32};
pub use self::os_support_h::{opus_alloc, opus_free};
pub use self::stdarg_h::va_list;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint32_t;
use self::stdlib_h::{free, malloc};
pub use self::types_h::{__int16_t, __int32_t, __uint32_t};
#[no_mangle]
#[c2rust::src_loc = "53:1"]
pub unsafe extern "C" fn opus_multistream_decoder_get_size(
    mut nb_streams: libc::c_int,
    mut nb_coupled_streams: libc::c_int,
) -> opus_int32 {
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
    mut Fs: opus_int32,
    mut channels: libc::c_int,
    mut streams: libc::c_int,
    mut coupled_streams: libc::c_int,
    mut mapping: *const libc::c_uchar,
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
        return -(1 as libc::c_int);
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
        return -(1 as libc::c_int);
    }
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSDecoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    coupled_size = opus_decoder_get_size(2 as libc::c_int);
    mono_size = opus_decoder_get_size(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_coupled_streams {
        ret = opus_decoder_init(ptr as *mut OpusDecoder, Fs, 2 as libc::c_int);
        if ret != 0 as libc::c_int {
            return ret;
        }
        ptr = ptr.offset(align(coupled_size) as isize);
        i += 1;
    }
    while i < (*st).layout.nb_streams {
        ret = opus_decoder_init(ptr as *mut OpusDecoder, Fs, 1 as libc::c_int);
        if ret != 0 as libc::c_int {
            return ret;
        }
        ptr = ptr.offset(align(mono_size) as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn opus_multistream_decoder_create(
    mut Fs: opus_int32,
    mut channels: libc::c_int,
    mut streams: libc::c_int,
    mut coupled_streams: libc::c_int,
    mut mapping: *const libc::c_uchar,
    mut error: *mut libc::c_int,
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
            *error = -(1 as libc::c_int);
        }
        return 0 as *mut OpusMSDecoder;
    }
    st = opus_alloc(opus_multistream_decoder_get_size(streams, coupled_streams) as size_t)
        as *mut OpusMSDecoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as libc::c_int);
        }
        return 0 as *mut OpusMSDecoder;
    }
    ret = opus_multistream_decoder_init(st, Fs, channels, streams, coupled_streams, mapping);
    if !error.is_null() {
        *error = ret;
    }
    if ret != 0 as libc::c_int {
        opus_free(st as *mut libc::c_void);
        st = 0 as *mut OpusMSDecoder;
    }
    return st;
}
#[c2rust::src_loc = "149:1"]
unsafe extern "C" fn opus_multistream_packet_validate(
    mut data: *const libc::c_uchar,
    mut len: opus_int32,
    mut nb_streams: libc::c_int,
    mut Fs: opus_int32,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut toc: libc::c_uchar = 0;
    let mut size: [opus_int16; 48] = [0; 48];
    let mut samples: libc::c_int = 0 as libc::c_int;
    let mut packet_offset: opus_int32 = 0;
    s = 0 as libc::c_int;
    while s < nb_streams {
        let mut tmp_samples: libc::c_int = 0;
        if len <= 0 as libc::c_int {
            return -(4 as libc::c_int);
        }
        count = opus_packet_parse_impl(
            data,
            len,
            (s != nb_streams - 1 as libc::c_int) as libc::c_int,
            &mut toc,
            0 as *mut *const libc::c_uchar,
            size.as_mut_ptr(),
            0 as *mut libc::c_int,
            &mut packet_offset,
        );
        if count < 0 as libc::c_int {
            return count;
        }
        tmp_samples = opus_packet_get_nb_samples(data, packet_offset, Fs);
        if s != 0 as libc::c_int && samples != tmp_samples {
            return -(4 as libc::c_int);
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
    mut st: *mut OpusMSDecoder,
    mut data: *const libc::c_uchar,
    mut len: opus_int32,
    mut pcm: *mut libc::c_void,
    mut copy_channel_out: opus_copy_channel_out_func,
    mut frame_size: libc::c_int,
    mut decode_fec: libc::c_int,
    mut soft_clip: libc::c_int,
    mut user_data: *mut libc::c_void,
) -> libc::c_int {
    let mut Fs: opus_int32 = 0;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut do_plc: libc::c_int = 0 as libc::c_int;
    if frame_size <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if opus_multistream_decoder_ctl(
        st,
        4029 as libc::c_int,
        (&mut Fs as *mut opus_int32).offset(
            (&mut Fs as *mut opus_int32).offset_from(&mut Fs as *mut opus_int32) as libc::c_long
                as isize,
        ),
    ) != 0 as libc::c_int
    {
        return -(3 as libc::c_int);
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
        return -(1 as libc::c_int);
    }
    if do_plc == 0 && len < 2 as libc::c_int * (*st).layout.nb_streams - 1 as libc::c_int {
        return -(4 as libc::c_int);
    }
    if do_plc == 0 {
        let mut ret: libc::c_int =
            opus_multistream_packet_validate(data, len, (*st).layout.nb_streams, Fs);
        if ret < 0 as libc::c_int {
            return ret;
        } else {
            if ret > frame_size {
                return -(2 as libc::c_int);
            }
        }
    }
    s = 0 as libc::c_int;
    while s < (*st).layout.nb_streams {
        let mut dec: *mut OpusDecoder = 0 as *mut OpusDecoder;
        let mut packet_offset: opus_int32 = 0;
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
            return -(3 as libc::c_int);
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
                0 as *const opus_val16,
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
    mut dst: *mut libc::c_void,
    mut dst_stride: libc::c_int,
    mut dst_channel: libc::c_int,
    mut src: *const opus_val16,
    mut src_stride: libc::c_int,
    mut frame_size: libc::c_int,
    mut user_data: *mut libc::c_void,
) {
    let mut float_dst: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: opus_int32 = 0;
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
    mut dst: *mut libc::c_void,
    mut dst_stride: libc::c_int,
    mut dst_channel: libc::c_int,
    mut src: *const opus_val16,
    mut src_stride: libc::c_int,
    mut frame_size: libc::c_int,
    mut user_data: *mut libc::c_void,
) {
    let mut short_dst: *mut opus_int16 = 0 as *mut opus_int16;
    let mut i: opus_int32 = 0;
    short_dst = dst as *mut opus_int16;
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
            *short_dst.offset((i * dst_stride + dst_channel) as isize) =
                0 as libc::c_int as opus_int16;
            i += 1;
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "395:1"]
pub unsafe extern "C" fn opus_multistream_decode(
    mut st: *mut OpusMSDecoder,
    mut data: *const libc::c_uchar,
    mut len: opus_int32,
    mut pcm: *mut opus_int16,
    mut frame_size: libc::c_int,
    mut decode_fec: libc::c_int,
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
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "402:1"]
pub unsafe extern "C" fn opus_multistream_decode_float(
    mut st: *mut OpusMSDecoder,
    mut data: *const libc::c_uchar,
    mut len: opus_int32,
    mut pcm: *mut opus_val16,
    mut frame_size: libc::c_int,
    mut decode_fec: libc::c_int,
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
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "416:1"]
pub unsafe extern "C" fn opus_multistream_decoder_ctl_va_list(
    mut st: *mut OpusMSDecoder,
    mut request: libc::c_int,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut current_block: u64;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0 as libc::c_int;
    coupled_size = opus_decoder_get_size(2 as libc::c_int);
    mono_size = opus_decoder_get_size(1 as libc::c_int);
    ptr = (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusMSDecoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    match request {
        4009 | 4029 | 4045 | 4039 | 4047 => {
            let mut dec: *mut OpusDecoder = 0 as *mut OpusDecoder;
            let mut value: *mut opus_int32 = ap.arg::<*mut opus_int32>();
            dec = ptr as *mut OpusDecoder;
            ret = opus_decoder_ctl(dec, request, value);
            current_block = 7343950298149844727;
        }
        4031 => {
            let mut s: libc::c_int = 0;
            let mut value_0: *mut opus_uint32 = ap.arg::<*mut opus_uint32>();
            let mut tmp: opus_uint32 = 0;
            if value_0.is_null() {
                current_block = 4177471793300891699;
            } else {
                *value_0 = 0 as libc::c_int as opus_uint32;
                s = 0 as libc::c_int;
                while s < (*st).layout.nb_streams {
                    let mut dec_0: *mut OpusDecoder = 0 as *mut OpusDecoder;
                    dec_0 = ptr as *mut OpusDecoder;
                    if s < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize);
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize);
                    }
                    ret = opus_decoder_ctl(dec_0, request, &mut tmp as *mut opus_uint32);
                    if ret != 0 as libc::c_int {
                        break;
                    }
                    *value_0 ^= tmp;
                    s += 1;
                }
                current_block = 7343950298149844727;
            }
        }
        4028 => {
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
                ret = opus_decoder_ctl(dec_1, 4028 as libc::c_int);
                if ret != 0 as libc::c_int {
                    break;
                }
                s_0 += 1;
            }
            current_block = 7343950298149844727;
        }
        5122 => {
            let mut s_1: libc::c_int = 0;
            let mut stream_id: opus_int32 = 0;
            let mut value_1: *mut *mut OpusDecoder = 0 as *mut *mut OpusDecoder;
            stream_id = ap.arg::<opus_int32>();
            if stream_id < 0 as libc::c_int || stream_id >= (*st).layout.nb_streams {
                current_block = 4177471793300891699;
            } else {
                value_1 = ap.arg::<*mut *mut OpusDecoder>();
                if value_1.is_null() {
                    current_block = 4177471793300891699;
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
        4034 | 4046 => {
            let mut s_2: libc::c_int = 0;
            let mut value_2: opus_int32 = ap.arg::<opus_int32>();
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
                if ret != 0 as libc::c_int {
                    break;
                }
                s_2 += 1;
            }
            current_block = 7343950298149844727;
        }
        _ => {
            ret = -(5 as libc::c_int);
            current_block = 7343950298149844727;
        }
    }
    match current_block {
        4177471793300891699 => return -(1 as libc::c_int),
        _ => return ret,
    };
}
#[no_mangle]
#[c2rust::src_loc = "536:1"]
pub unsafe extern "C" fn opus_multistream_decoder_ctl(
    mut st: *mut OpusMSDecoder,
    mut request: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    ret = opus_multistream_decoder_ctl_va_list(st, request, ap.as_va_list());
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "546:1"]
pub unsafe extern "C" fn opus_multistream_decoder_destroy(mut st: *mut OpusMSDecoder) {
    opus_free(st as *mut libc::c_void);
}
