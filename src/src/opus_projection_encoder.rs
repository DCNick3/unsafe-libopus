use crate::externs::{free, malloc};
use crate::src::opus_encoder::{downmix_float, downmix_int};
use crate::src::opus_multistream_encoder::{
    opus_multistream_encode_native, opus_multistream_encoder_ctl_va_list,
};
use crate::src::opus_private::align;
use crate::{opus_multistream_encoder_get_size, opus_multistream_encoder_init, OpusMSEncoder};
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_defines.h:32"]
pub mod opus_defines_h {
    #[c2rust::src_loc = "60:9"]
    pub const OPUS_ALLOC_FAIL: libc::c_int = -(7 as libc::c_int);
    #[c2rust::src_loc = "56:9"]
    pub const OPUS_UNIMPLEMENTED: libc::c_int = -(5 as libc::c_int);
    #[c2rust::src_loc = "48:9"]
    pub const OPUS_BAD_ARG: libc::c_int = -(1 as libc::c_int);
    #[c2rust::src_loc = "46:9"]
    pub const OPUS_OK: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_projection.h:36"]
pub mod opus_projection_h {
    #[c2rust::src_loc = "48:9"]
    pub const OPUS_PROJECTION_GET_DEMIXING_MATRIX_GAIN_REQUEST: libc::c_int = 6001;
    #[c2rust::src_loc = "49:9"]
    pub const OPUS_PROJECTION_GET_DEMIXING_MATRIX_SIZE_REQUEST: libc::c_int = 6003;
    #[c2rust::src_loc = "50:9"]
    pub const OPUS_PROJECTION_GET_DEMIXING_MATRIX_REQUEST: libc::c_int = 6005;
}
pub use self::arch_h::{opus_val16, opus_val32};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::opus_defines_h::{OPUS_ALLOC_FAIL, OPUS_BAD_ARG, OPUS_OK, OPUS_UNIMPLEMENTED};
pub use self::opus_projection_h::{
    OPUS_PROJECTION_GET_DEMIXING_MATRIX_GAIN_REQUEST, OPUS_PROJECTION_GET_DEMIXING_MATRIX_REQUEST,
    OPUS_PROJECTION_GET_DEMIXING_MATRIX_SIZE_REQUEST,
};
pub use self::stdarg_h::va_list;
pub use self::stddef_h::{size_t, NULL};
use crate::celt::mathops::isqrt32;
use crate::src::mapping_matrix::{
    mapping_matrix_foa_demixing, mapping_matrix_foa_demixing_data, mapping_matrix_foa_mixing,
    mapping_matrix_foa_mixing_data, mapping_matrix_get_data, mapping_matrix_get_size,
    mapping_matrix_init, mapping_matrix_multiply_channel_in_float,
    mapping_matrix_multiply_channel_in_short, mapping_matrix_soa_demixing,
    mapping_matrix_soa_demixing_data, mapping_matrix_soa_mixing, mapping_matrix_soa_mixing_data,
    mapping_matrix_toa_demixing, mapping_matrix_toa_demixing_data, mapping_matrix_toa_mixing,
    mapping_matrix_toa_mixing_data, MappingMatrix,
};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "41:8"]
pub struct OpusProjectionEncoder {
    pub mixing_matrix_size_in_bytes: i32,
    pub demixing_matrix_size_in_bytes: i32,
}
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn opus_projection_copy_channel_in_float(
    dst: *mut opus_val16,
    dst_stride: libc::c_int,
    src: *const libc::c_void,
    src_stride: libc::c_int,
    src_channel: libc::c_int,
    frame_size: libc::c_int,
    user_data: *mut libc::c_void,
) {
    mapping_matrix_multiply_channel_in_float(
        user_data as *const MappingMatrix,
        src as *const libc::c_float,
        src_stride,
        dst,
        src_channel,
        dst_stride,
        frame_size,
    );
}
#[c2rust::src_loc = "64:1"]
unsafe extern "C" fn opus_projection_copy_channel_in_short(
    dst: *mut opus_val16,
    dst_stride: libc::c_int,
    src: *const libc::c_void,
    src_stride: libc::c_int,
    src_channel: libc::c_int,
    frame_size: libc::c_int,
    user_data: *mut libc::c_void,
) {
    mapping_matrix_multiply_channel_in_short(
        user_data as *const MappingMatrix,
        src as *const i16,
        src_stride,
        dst,
        src_channel,
        dst_stride,
        frame_size,
    );
}
#[c2rust::src_loc = "78:1"]
unsafe extern "C" fn get_order_plus_one_from_channels(
    channels: libc::c_int,
    order_plus_one: *mut libc::c_int,
) -> libc::c_int {
    let mut order_plus_one_: libc::c_int = 0;
    let mut acn_channels: libc::c_int = 0;
    let mut nondiegetic_channels: libc::c_int = 0;
    if channels < 1 as libc::c_int || channels > 227 as libc::c_int {
        return OPUS_BAD_ARG;
    }
    order_plus_one_ = isqrt32(channels as u32) as libc::c_int;
    acn_channels = order_plus_one_ * order_plus_one_;
    nondiegetic_channels = channels - acn_channels;
    if nondiegetic_channels != 0 as libc::c_int && nondiegetic_channels != 2 as libc::c_int {
        return OPUS_BAD_ARG;
    }
    if !order_plus_one.is_null() {
        *order_plus_one = order_plus_one_;
    }
    return OPUS_OK;
}
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn get_streams_from_channels(
    channels: libc::c_int,
    mapping_family: libc::c_int,
    streams: *mut libc::c_int,
    coupled_streams: *mut libc::c_int,
    order_plus_one: *mut libc::c_int,
) -> libc::c_int {
    if mapping_family == 3 as libc::c_int {
        if get_order_plus_one_from_channels(channels, order_plus_one) != OPUS_OK {
            return OPUS_BAD_ARG;
        }
        if !streams.is_null() {
            *streams = (channels + 1 as libc::c_int) / 2 as libc::c_int;
        }
        if !coupled_streams.is_null() {
            *coupled_streams = channels / 2 as libc::c_int;
        }
        return OPUS_OK;
    }
    return OPUS_BAD_ARG;
}
#[c2rust::src_loc = "118:1"]
unsafe extern "C" fn get_mixing_matrix(st: *mut OpusProjectionEncoder) -> *mut MappingMatrix {
    return (st as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<OpusProjectionEncoder>() as libc::c_ulong as libc::c_int
    ) as isize) as *mut libc::c_void as *mut MappingMatrix;
}
#[c2rust::src_loc = "125:1"]
unsafe extern "C" fn get_enc_demixing_matrix(st: *mut OpusProjectionEncoder) -> *mut MappingMatrix {
    return (st as *mut libc::c_char).offset(align(
        (::core::mem::size_of::<OpusProjectionEncoder>() as libc::c_ulong)
            .wrapping_add((*st).mixing_matrix_size_in_bytes as libc::c_ulong)
            as libc::c_int,
    ) as isize) as *mut libc::c_void as *mut MappingMatrix;
}
#[c2rust::src_loc = "133:1"]
unsafe extern "C" fn get_multistream_encoder(st: *mut OpusProjectionEncoder) -> *mut OpusMSEncoder {
    return (st as *mut libc::c_char).offset(align(
        (::core::mem::size_of::<OpusProjectionEncoder>() as libc::c_ulong)
            .wrapping_add((*st).mixing_matrix_size_in_bytes as libc::c_ulong)
            .wrapping_add((*st).demixing_matrix_size_in_bytes as libc::c_ulong)
            as libc::c_int,
    ) as isize) as *mut libc::c_void as *mut OpusMSEncoder;
}
#[no_mangle]
#[c2rust::src_loc = "142:1"]
pub unsafe extern "C" fn opus_projection_ambisonics_encoder_get_size(
    channels: libc::c_int,
    mapping_family: libc::c_int,
) -> i32 {
    let mut nb_streams: libc::c_int = 0;
    let mut nb_coupled_streams: libc::c_int = 0;
    let mut order_plus_one: libc::c_int = 0;
    let mut mixing_matrix_rows: libc::c_int = 0;
    let mut mixing_matrix_cols: libc::c_int = 0;
    let mut demixing_matrix_rows: libc::c_int = 0;
    let mut demixing_matrix_cols: libc::c_int = 0;
    let mut mixing_matrix_size: i32 = 0;
    let mut demixing_matrix_size: i32 = 0;
    let mut encoder_size: i32 = 0;
    let mut ret: libc::c_int = 0;
    ret = get_streams_from_channels(
        channels,
        mapping_family,
        &mut nb_streams,
        &mut nb_coupled_streams,
        &mut order_plus_one,
    );
    if ret != OPUS_OK {
        return 0 as libc::c_int;
    }
    if order_plus_one == 2 as libc::c_int {
        mixing_matrix_rows = mapping_matrix_foa_mixing.rows;
        mixing_matrix_cols = mapping_matrix_foa_mixing.cols;
        demixing_matrix_rows = mapping_matrix_foa_demixing.rows;
        demixing_matrix_cols = mapping_matrix_foa_demixing.cols;
    } else if order_plus_one == 3 as libc::c_int {
        mixing_matrix_rows = mapping_matrix_soa_mixing.rows;
        mixing_matrix_cols = mapping_matrix_soa_mixing.cols;
        demixing_matrix_rows = mapping_matrix_soa_demixing.rows;
        demixing_matrix_cols = mapping_matrix_soa_demixing.cols;
    } else if order_plus_one == 4 as libc::c_int {
        mixing_matrix_rows = mapping_matrix_toa_mixing.rows;
        mixing_matrix_cols = mapping_matrix_toa_mixing.cols;
        demixing_matrix_rows = mapping_matrix_toa_demixing.rows;
        demixing_matrix_cols = mapping_matrix_toa_demixing.cols;
    } else {
        return 0 as libc::c_int;
    }
    mixing_matrix_size = mapping_matrix_get_size(mixing_matrix_rows, mixing_matrix_cols);
    if mixing_matrix_size == 0 {
        return 0 as libc::c_int;
    }
    demixing_matrix_size = mapping_matrix_get_size(demixing_matrix_rows, demixing_matrix_cols);
    if demixing_matrix_size == 0 {
        return 0 as libc::c_int;
    }
    encoder_size = opus_multistream_encoder_get_size(nb_streams, nb_coupled_streams);
    if encoder_size == 0 {
        return 0 as libc::c_int;
    }
    return align(::core::mem::size_of::<OpusProjectionEncoder>() as libc::c_ulong as libc::c_int)
        + mixing_matrix_size
        + demixing_matrix_size
        + encoder_size;
}
#[no_mangle]
#[c2rust::src_loc = "202:1"]
pub unsafe extern "C" fn opus_projection_ambisonics_encoder_init(
    mut st: *mut OpusProjectionEncoder,
    Fs: i32,
    channels: libc::c_int,
    mapping_family: libc::c_int,
    streams: *mut libc::c_int,
    coupled_streams: *mut libc::c_int,
    application: libc::c_int,
) -> libc::c_int {
    let mut mixing_matrix: *mut MappingMatrix = 0 as *mut MappingMatrix;
    let mut demixing_matrix: *mut MappingMatrix = 0 as *mut MappingMatrix;
    let mut ms_encoder: *mut OpusMSEncoder = 0 as *mut OpusMSEncoder;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut order_plus_one: libc::c_int = 0;
    let mut mapping: [libc::c_uchar; 255] = [0; 255];
    if streams.is_null() || coupled_streams.is_null() {
        return OPUS_BAD_ARG;
    }
    if get_streams_from_channels(
        channels,
        mapping_family,
        streams,
        coupled_streams,
        &mut order_plus_one,
    ) != OPUS_OK
    {
        return OPUS_BAD_ARG;
    }
    if mapping_family == 3 as libc::c_int {
        mixing_matrix = get_mixing_matrix(st);
        if order_plus_one == 2 as libc::c_int {
            mapping_matrix_init(
                mixing_matrix,
                mapping_matrix_foa_mixing.rows,
                mapping_matrix_foa_mixing.cols,
                mapping_matrix_foa_mixing.gain,
                mapping_matrix_foa_mixing_data.as_ptr(),
                ::core::mem::size_of::<[i16; 36]>() as libc::c_ulong as i32,
            );
        } else if order_plus_one == 3 as libc::c_int {
            mapping_matrix_init(
                mixing_matrix,
                mapping_matrix_soa_mixing.rows,
                mapping_matrix_soa_mixing.cols,
                mapping_matrix_soa_mixing.gain,
                mapping_matrix_soa_mixing_data.as_ptr(),
                ::core::mem::size_of::<[i16; 121]>() as libc::c_ulong as i32,
            );
        } else if order_plus_one == 4 as libc::c_int {
            mapping_matrix_init(
                mixing_matrix,
                mapping_matrix_toa_mixing.rows,
                mapping_matrix_toa_mixing.cols,
                mapping_matrix_toa_mixing.gain,
                mapping_matrix_toa_mixing_data.as_ptr(),
                ::core::mem::size_of::<[i16; 324]>() as libc::c_ulong as i32,
            );
        } else {
            return OPUS_BAD_ARG;
        }
        (*st).mixing_matrix_size_in_bytes =
            mapping_matrix_get_size((*mixing_matrix).rows, (*mixing_matrix).cols);
        if (*st).mixing_matrix_size_in_bytes == 0 {
            return OPUS_BAD_ARG;
        }
        demixing_matrix = get_enc_demixing_matrix(st);
        if order_plus_one == 2 as libc::c_int {
            mapping_matrix_init(
                demixing_matrix,
                mapping_matrix_foa_demixing.rows,
                mapping_matrix_foa_demixing.cols,
                mapping_matrix_foa_demixing.gain,
                mapping_matrix_foa_demixing_data.as_ptr(),
                ::core::mem::size_of::<[i16; 36]>() as libc::c_ulong as i32,
            );
        } else if order_plus_one == 3 as libc::c_int {
            mapping_matrix_init(
                demixing_matrix,
                mapping_matrix_soa_demixing.rows,
                mapping_matrix_soa_demixing.cols,
                mapping_matrix_soa_demixing.gain,
                mapping_matrix_soa_demixing_data.as_ptr(),
                ::core::mem::size_of::<[i16; 121]>() as libc::c_ulong as i32,
            );
        } else if order_plus_one == 4 as libc::c_int {
            mapping_matrix_init(
                demixing_matrix,
                mapping_matrix_toa_demixing.rows,
                mapping_matrix_toa_demixing.cols,
                mapping_matrix_toa_demixing.gain,
                mapping_matrix_toa_demixing_data.as_ptr(),
                ::core::mem::size_of::<[i16; 324]>() as libc::c_ulong as i32,
            );
        } else {
            return OPUS_BAD_ARG;
        }
        (*st).demixing_matrix_size_in_bytes =
            mapping_matrix_get_size((*demixing_matrix).rows, (*demixing_matrix).cols);
        if (*st).demixing_matrix_size_in_bytes == 0 {
            return OPUS_BAD_ARG;
        }
    } else {
        return OPUS_UNIMPLEMENTED;
    }
    if *streams + *coupled_streams > (*mixing_matrix).rows
        || channels > (*mixing_matrix).cols
        || channels > (*demixing_matrix).rows
        || *streams + *coupled_streams > (*demixing_matrix).cols
    {
        return OPUS_BAD_ARG;
    }
    i = 0 as libc::c_int;
    while i < channels {
        mapping[i as usize] = i as libc::c_uchar;
        i += 1;
    }
    ms_encoder = get_multistream_encoder(st);
    ret = opus_multistream_encoder_init(
        ms_encoder,
        Fs,
        channels,
        *streams,
        *coupled_streams,
        mapping.as_mut_ptr(),
        application,
    );
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "308:1"]
pub unsafe extern "C" fn opus_projection_ambisonics_encoder_create(
    Fs: i32,
    channels: libc::c_int,
    mapping_family: libc::c_int,
    streams: *mut libc::c_int,
    coupled_streams: *mut libc::c_int,
    application: libc::c_int,
    error: *mut libc::c_int,
) -> *mut OpusProjectionEncoder {
    let mut size: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut st: *mut OpusProjectionEncoder = 0 as *mut OpusProjectionEncoder;
    size = opus_projection_ambisonics_encoder_get_size(channels, mapping_family);
    if size == 0 {
        if !error.is_null() {
            *error = OPUS_ALLOC_FAIL;
        }
        return NULL as *mut OpusProjectionEncoder;
    }
    st = malloc(size as size_t) as *mut OpusProjectionEncoder;
    if st.is_null() {
        if !error.is_null() {
            *error = OPUS_ALLOC_FAIL;
        }
        return NULL as *mut OpusProjectionEncoder;
    }
    ret = opus_projection_ambisonics_encoder_init(
        st,
        Fs,
        channels,
        mapping_family,
        streams,
        coupled_streams,
        application,
    );
    if ret != OPUS_OK {
        free(st as *mut libc::c_void);
        st = NULL as *mut OpusProjectionEncoder;
    }
    if !error.is_null() {
        *error = ret;
    }
    return st;
}
#[no_mangle]
#[c2rust::src_loc = "344:1"]
pub unsafe extern "C" fn opus_projection_encode(
    st: *mut OpusProjectionEncoder,
    pcm: *const i16,
    frame_size: libc::c_int,
    data: *mut libc::c_uchar,
    max_data_bytes: i32,
) -> libc::c_int {
    return opus_multistream_encode_native(
        get_multistream_encoder(st),
        Some(
            opus_projection_copy_channel_in_short
                as unsafe extern "C" fn(
                    *mut opus_val16,
                    libc::c_int,
                    *const libc::c_void,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
        pcm as *const libc::c_void,
        frame_size,
        data,
        max_data_bytes,
        16 as libc::c_int,
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
        get_mixing_matrix(st) as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "364:1"]
pub unsafe extern "C" fn opus_projection_encode_float(
    st: *mut OpusProjectionEncoder,
    pcm: *const libc::c_float,
    frame_size: libc::c_int,
    data: *mut libc::c_uchar,
    max_data_bytes: i32,
) -> libc::c_int {
    return opus_multistream_encode_native(
        get_multistream_encoder(st),
        Some(
            opus_projection_copy_channel_in_float
                as unsafe extern "C" fn(
                    *mut opus_val16,
                    libc::c_int,
                    *const libc::c_void,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
        pcm as *const libc::c_void,
        frame_size,
        data,
        max_data_bytes,
        24 as libc::c_int,
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
        get_mixing_matrix(st) as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "375:1"]
pub unsafe extern "C" fn opus_projection_encoder_destroy(st: *mut OpusProjectionEncoder) {
    free(st as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "380:1"]
pub unsafe extern "C" fn opus_projection_encoder_ctl(
    st: *mut OpusProjectionEncoder,
    request: libc::c_int,
    args: ...
) -> libc::c_int {
    let current_block: u64;
    let mut ap: ::core::ffi::VaListImpl;
    let mut demixing_matrix: *mut MappingMatrix = 0 as *mut MappingMatrix;
    let mut ms_encoder: *mut OpusMSEncoder = 0 as *mut OpusMSEncoder;
    let mut ret: libc::c_int = OPUS_OK;
    ms_encoder = get_multistream_encoder(st);
    demixing_matrix = get_enc_demixing_matrix(st);
    ap = args.clone();
    match request {
        OPUS_PROJECTION_GET_DEMIXING_MATRIX_SIZE_REQUEST => {
            let value: *mut i32 = ap.arg::<*mut i32>();
            if value.is_null() {
                current_block = 17184638872671510253;
            } else {
                *value = (((*ms_encoder).layout.nb_channels
                    * ((*ms_encoder).layout.nb_streams + (*ms_encoder).layout.nb_coupled_streams))
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong)
                    as i32;
                current_block = 18153031941552419006;
            }
        }
        OPUS_PROJECTION_GET_DEMIXING_MATRIX_GAIN_REQUEST => {
            let value_0: *mut i32 = ap.arg::<*mut i32>();
            if value_0.is_null() {
                current_block = 17184638872671510253;
            } else {
                *value_0 = (*demixing_matrix).gain;
                current_block = 18153031941552419006;
            }
        }
        OPUS_PROJECTION_GET_DEMIXING_MATRIX_REQUEST => {
            let mut i: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            let mut k: libc::c_int = 0;
            let mut l: libc::c_int = 0;
            let mut nb_input_streams: libc::c_int = 0;
            let mut nb_output_streams: libc::c_int = 0;
            let mut external_char: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut internal_short: *mut i16 = 0 as *mut i16;
            let mut external_size: i32 = 0;
            let mut internal_size: i32 = 0;
            nb_input_streams =
                (*ms_encoder).layout.nb_streams + (*ms_encoder).layout.nb_coupled_streams;
            nb_output_streams = (*ms_encoder).layout.nb_channels;
            external_char = ap.arg::<*mut libc::c_uchar>();
            external_size = ap.arg::<i32>();
            if external_char.is_null() {
                current_block = 17184638872671510253;
            } else {
                internal_short = mapping_matrix_get_data(demixing_matrix);
                internal_size = ((nb_input_streams * nb_output_streams) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong)
                    as i32;
                if external_size != internal_size {
                    current_block = 17184638872671510253;
                } else {
                    l = 0 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < nb_input_streams {
                        j = 0 as libc::c_int;
                        while j < nb_output_streams {
                            k = (*demixing_matrix).rows * i + j;
                            *external_char.offset((2 as libc::c_int * l) as isize) =
                                *internal_short.offset(k as isize) as libc::c_uchar;
                            *external_char
                                .offset((2 as libc::c_int * l + 1 as libc::c_int) as isize) =
                                (*internal_short.offset(k as isize) as libc::c_int
                                    >> 8 as libc::c_int)
                                    as libc::c_uchar;
                            l += 1;
                            j += 1;
                        }
                        i += 1;
                    }
                    current_block = 18153031941552419006;
                }
            }
        }
        _ => {
            ret = opus_multistream_encoder_ctl_va_list(ms_encoder, request, ap.as_va_list());
            current_block = 18153031941552419006;
        }
    }
    match current_block {
        17184638872671510253 => return OPUS_BAD_ARG,
        _ => return ret,
    };
}
