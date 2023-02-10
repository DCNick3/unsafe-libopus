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
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:32"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stdarg.h:32"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
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
    #[c2rust::src_loc = "98:1"]
    pub type opus_copy_channel_out_func = Option::<
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
    pub unsafe extern "C" fn align(i: libc::c_int) -> libc::c_int {
        let alignment: libc::c_uint = 8 as libc::c_ulong as libc::c_uint;
        return (i as libc::c_uint)
            .wrapping_add(alignment)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(alignment)
            .wrapping_mul(alignment) as libc::c_int;
    }
    use super::arch_h::{opus_val16, opus_val32};
    use super::opus_types_h::opus_int32;
    
    extern "C" {
        #[c2rust::src_loc = "189:1"]
        pub fn opus_multistream_decode_native(
            st: *mut OpusMSDecoder,
            data: *const libc::c_uchar,
            len: opus_int32,
            pcm: *mut libc::c_void,
            copy_channel_out: opus_copy_channel_out_func,
            frame_size: libc::c_int,
            decode_fec: libc::c_int,
            soft_clip: libc::c_int,
            user_data: *mut libc::c_void,
        ) -> libc::c_int;
        #[c2rust::src_loc = "80:1"]
        pub fn opus_multistream_decoder_ctl_va_list(
            st: *mut OpusMSDecoder,
            request: libc::c_int,
            ap: ::core::ffi::VaList,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/mapping_matrix.h:38"]
pub mod mapping_matrix_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:16"]
    pub struct MappingMatrix {
        pub rows: libc::c_int,
        pub cols: libc::c_int,
        pub gain: libc::c_int,
    }
    use super::opus_types_h::{opus_int32, opus_int16};
    use super::arch_h::opus_val16;
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn mapping_matrix_get_size(
            rows: libc::c_int,
            cols: libc::c_int,
        ) -> opus_int32;
        #[c2rust::src_loc = "55:1"]
        pub fn mapping_matrix_init(
            matrix: *mut MappingMatrix,
            rows: libc::c_int,
            cols: libc::c_int,
            gain: libc::c_int,
            data: *const opus_int16,
            data_size: opus_int32,
        );
        #[c2rust::src_loc = "75:1"]
        pub fn mapping_matrix_multiply_channel_out_float(
            matrix: *const MappingMatrix,
            input: *const opus_val16,
            input_row: libc::c_int,
            input_rows: libc::c_int,
            output: *mut libc::c_float,
            output_rows: libc::c_int,
            frame_size: libc::c_int,
        );
        #[c2rust::src_loc = "96:1"]
        pub fn mapping_matrix_multiply_channel_out_short(
            matrix: *const MappingMatrix,
            input: *const opus_val16,
            input_row: libc::c_int,
            input_rows: libc::c_int,
            output: *mut opus_int16,
            output_rows: libc::c_int,
            frame_size: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_defines.h:32"]
pub mod opus_defines_h {
    #[c2rust::src_loc = "60:9"]
    pub const OPUS_ALLOC_FAIL: libc::c_int = -(7 as libc::c_int);
    #[c2rust::src_loc = "48:9"]
    pub const OPUS_BAD_ARG: libc::c_int = -(1 as libc::c_int);
    #[c2rust::src_loc = "46:9"]
    pub const OPUS_OK: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/string.h:32"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/os_support.h:32"]
pub mod os_support_h {
    #[inline]
    #[c2rust::src_loc = "47:1"]
    pub unsafe extern "C" fn opus_alloc(size: size_t) -> *mut libc::c_void {
        return malloc(size);
    }
    #[inline]
    #[c2rust::src_loc = "64:1"]
    pub unsafe extern "C" fn opus_free(ptr: *mut libc::c_void) {
        free(ptr);
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::{malloc, free};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_multistream.h:36"]
pub mod opus_multistream_h {
    use super::opus_types_h::opus_int32;
    use super::opus_private_h::OpusMSDecoder;
    extern "C" {
        #[c2rust::src_loc = "470:1"]
        pub fn opus_multistream_decoder_get_size(
            streams: libc::c_int,
            coupled_streams: libc::c_int,
        ) -> opus_int32;
        #[c2rust::src_loc = "547:1"]
        pub fn opus_multistream_decoder_init(
            st: *mut OpusMSDecoder,
            Fs: opus_int32,
            channels: libc::c_int,
            streams: libc::c_int,
            coupled_streams: libc::c_int,
            mapping: *const libc::c_uchar,
        ) -> libc::c_int;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__int16_t, __int32_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::opus_types_h::{opus_int16, opus_int32};
pub use self::arch_h::{opus_val16, opus_val32};
pub use self::stddef_h::{size_t, NULL};
pub use self::stdarg_h::va_list;
pub use self::opus_private_h::{
    ChannelLayout, OpusMSDecoder, opus_copy_channel_out_func, foo, C2RustUnnamed, align,
    opus_multistream_decode_native, opus_multistream_decoder_ctl_va_list,
};
pub use self::mapping_matrix_h::{
    MappingMatrix, mapping_matrix_get_size, mapping_matrix_init,
    mapping_matrix_multiply_channel_out_float, mapping_matrix_multiply_channel_out_short,
};
pub use self::opus_defines_h::{OPUS_ALLOC_FAIL, OPUS_BAD_ARG, OPUS_OK};
use self::string_h::memset;

pub use self::os_support_h::{opus_alloc, opus_free};
use self::opus_multistream_h::{
    opus_multistream_decoder_get_size, opus_multistream_decoder_init,
};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "41:8"]
pub struct OpusProjectionDecoder {
    pub demixing_matrix_size_in_bytes: opus_int32,
}
#[c2rust::src_loc = "48:1"]
unsafe extern "C" fn opus_projection_copy_channel_out_float(
    dst: *mut libc::c_void,
    dst_stride: libc::c_int,
    dst_channel: libc::c_int,
    src: *const opus_val16,
    src_stride: libc::c_int,
    frame_size: libc::c_int,
    user_data: *mut libc::c_void,
) {
    let mut float_dst: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut matrix: *const MappingMatrix = 0 as *const MappingMatrix;
    float_dst = dst as *mut libc::c_float;
    matrix = user_data as *const MappingMatrix;
    if dst_channel == 0 as libc::c_int {
        memset(
            float_dst as *mut libc::c_void,
            0 as libc::c_int,
            ((frame_size * dst_stride) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
    }
    if !src.is_null() {
        mapping_matrix_multiply_channel_out_float(
            matrix,
            src,
            dst_channel,
            src_stride,
            float_dst,
            dst_stride,
            frame_size,
        );
    }
}
#[c2rust::src_loc = "71:1"]
unsafe extern "C" fn opus_projection_copy_channel_out_short(
    dst: *mut libc::c_void,
    dst_stride: libc::c_int,
    dst_channel: libc::c_int,
    src: *const opus_val16,
    src_stride: libc::c_int,
    frame_size: libc::c_int,
    user_data: *mut libc::c_void,
) {
    let mut short_dst: *mut opus_int16 = 0 as *mut opus_int16;
    let mut matrix: *const MappingMatrix = 0 as *const MappingMatrix;
    short_dst = dst as *mut opus_int16;
    matrix = user_data as *const MappingMatrix;
    if dst_channel == 0 as libc::c_int {
        memset(
            short_dst as *mut libc::c_void,
            0 as libc::c_int,
            ((frame_size * dst_stride) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
        );
    }
    if !src.is_null() {
        mapping_matrix_multiply_channel_out_short(
            matrix,
            src,
            dst_channel,
            src_stride,
            short_dst,
            dst_stride,
            frame_size,
        );
    }
}
#[c2rust::src_loc = "92:1"]
unsafe extern "C" fn get_dec_demixing_matrix(
    st: *mut OpusProjectionDecoder,
) -> *mut MappingMatrix {
    return (st as *mut libc::c_char)
        .offset(
            align(
                ::core::mem::size_of::<OpusProjectionDecoder>() as libc::c_ulong
                    as libc::c_int,
            ) as isize,
        ) as *mut libc::c_void as *mut MappingMatrix;
}
#[c2rust::src_loc = "99:1"]
unsafe extern "C" fn get_multistream_decoder(
    st: *mut OpusProjectionDecoder,
) -> *mut OpusMSDecoder {
    return (st as *mut libc::c_char)
        .offset(
            align(
                (::core::mem::size_of::<OpusProjectionDecoder>() as libc::c_ulong)
                    .wrapping_add((*st).demixing_matrix_size_in_bytes as libc::c_ulong)
                    as libc::c_int,
            ) as isize,
        ) as *mut libc::c_void as *mut OpusMSDecoder;
}
#[no_mangle]
#[c2rust::src_loc = "107:1"]
pub unsafe extern "C" fn opus_projection_decoder_get_size(
    channels: libc::c_int,
    streams: libc::c_int,
    coupled_streams: libc::c_int,
) -> opus_int32 {
    let mut matrix_size: opus_int32 = 0;
    let mut decoder_size: opus_int32 = 0;
    matrix_size = mapping_matrix_get_size(streams + coupled_streams, channels);
    if matrix_size == 0 {
        return 0 as libc::c_int;
    }
    decoder_size = opus_multistream_decoder_get_size(streams, coupled_streams);
    if decoder_size == 0 {
        return 0 as libc::c_int;
    }
    return align(
        ::core::mem::size_of::<OpusProjectionDecoder>() as libc::c_ulong as libc::c_int,
    ) + matrix_size + decoder_size;
}
#[no_mangle]
#[c2rust::src_loc = "125:1"]
pub unsafe extern "C" fn opus_projection_decoder_init(
    mut st: *mut OpusProjectionDecoder,
    Fs: opus_int32,
    channels: libc::c_int,
    streams: libc::c_int,
    coupled_streams: libc::c_int,
    demixing_matrix: *mut libc::c_uchar,
    demixing_matrix_size: opus_int32,
) -> libc::c_int {
    let mut nb_input_streams: libc::c_int = 0;
    let mut expected_matrix_size: opus_int32 = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut mapping: [libc::c_uchar; 255] = [0; 255];
    nb_input_streams = streams + coupled_streams;
    expected_matrix_size = ((nb_input_streams * channels) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong)
        as opus_int32;
    if expected_matrix_size != demixing_matrix_size {
        return OPUS_BAD_ARG;
    }
    let vla = (nb_input_streams * channels) as usize;
    let mut buf: Vec::<opus_int16> = ::std::vec::from_elem(0, vla);
    i = 0 as libc::c_int;
    while i < nb_input_streams * channels {
        let mut s: libc::c_int = (*demixing_matrix
            .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) as libc::c_int)
            << 8 as libc::c_int
            | *demixing_matrix.offset((2 as libc::c_int * i) as isize) as libc::c_int;
        s = (s & 0xffff as libc::c_int ^ 0x8000 as libc::c_int) - 0x8000 as libc::c_int;
        *buf.as_mut_ptr().offset(i as isize) = s as opus_int16;
        i += 1;
    }
    (*st)
        .demixing_matrix_size_in_bytes = mapping_matrix_get_size(
        channels,
        nb_input_streams,
    );
    if (*st).demixing_matrix_size_in_bytes == 0 {
        return OPUS_BAD_ARG;
    }
    mapping_matrix_init(
        get_dec_demixing_matrix(st),
        channels,
        nb_input_streams,
        0 as libc::c_int,
        buf.as_mut_ptr(),
        demixing_matrix_size,
    );
    i = 0 as libc::c_int;
    while i < channels {
        mapping[i as usize] = i as libc::c_uchar;
        i += 1;
    }
    ret = opus_multistream_decoder_init(
        get_multistream_decoder(st),
        Fs,
        channels,
        streams,
        coupled_streams,
        mapping.as_mut_ptr(),
    );
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "176:1"]
pub unsafe extern "C" fn opus_projection_decoder_create(
    Fs: opus_int32,
    channels: libc::c_int,
    streams: libc::c_int,
    coupled_streams: libc::c_int,
    demixing_matrix: *mut libc::c_uchar,
    demixing_matrix_size: opus_int32,
    error: *mut libc::c_int,
) -> *mut OpusProjectionDecoder {
    let mut size: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut st: *mut OpusProjectionDecoder = 0 as *mut OpusProjectionDecoder;
    size = opus_projection_decoder_get_size(channels, streams, coupled_streams);
    if size == 0 {
        if !error.is_null() {
            *error = OPUS_ALLOC_FAIL;
        }
        return NULL as *mut OpusProjectionDecoder;
    }
    st = opus_alloc(size as size_t) as *mut OpusProjectionDecoder;
    if st.is_null() {
        if !error.is_null() {
            *error = OPUS_ALLOC_FAIL;
        }
        return NULL as *mut OpusProjectionDecoder;
    }
    ret = opus_projection_decoder_init(
        st,
        Fs,
        channels,
        streams,
        coupled_streams,
        demixing_matrix,
        demixing_matrix_size,
    );
    if ret != OPUS_OK {
        opus_free(st as *mut libc::c_void);
        st = NULL as *mut OpusProjectionDecoder;
    }
    if !error.is_null() {
        *error = ret;
    }
    return st;
}
#[no_mangle]
#[c2rust::src_loc = "222:1"]
pub unsafe extern "C" fn opus_projection_decode(
    st: *mut OpusProjectionDecoder,
    data: *const libc::c_uchar,
    len: opus_int32,
    pcm: *mut opus_int16,
    frame_size: libc::c_int,
    decode_fec: libc::c_int,
) -> libc::c_int {
    return opus_multistream_decode_native(
        get_multistream_decoder(st),
        data,
        len,
        pcm as *mut libc::c_void,
        Some(
            opus_projection_copy_channel_out_short
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
        get_dec_demixing_matrix(st) as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "233:1"]
pub unsafe extern "C" fn opus_projection_decode_float(
    st: *mut OpusProjectionDecoder,
    data: *const libc::c_uchar,
    len: opus_int32,
    pcm: *mut libc::c_float,
    frame_size: libc::c_int,
    decode_fec: libc::c_int,
) -> libc::c_int {
    return opus_multistream_decode_native(
        get_multistream_decoder(st),
        data,
        len,
        pcm as *mut libc::c_void,
        Some(
            opus_projection_copy_channel_out_float
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
        get_dec_demixing_matrix(st) as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "242:1"]
pub unsafe extern "C" fn opus_projection_decoder_ctl(
    st: *mut OpusProjectionDecoder,
    request: libc::c_int,
    args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut ret: libc::c_int = OPUS_OK;
    ap = args.clone();
    ret = opus_multistream_decoder_ctl_va_list(
        get_multistream_decoder(st),
        request,
        ap.as_va_list(),
    );
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "254:1"]
pub unsafe extern "C" fn opus_projection_decoder_destroy(
    st: *mut OpusProjectionDecoder,
) {
    opus_free(st as *mut libc::c_void);
}
