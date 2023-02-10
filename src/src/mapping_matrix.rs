use ::libc;
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
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/xmmintrin.h:33"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvtss_si32, _mm_cvt_ss2si, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvtss_si32, _mm_cvt_ss2si, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/opus_private.h:34"]
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
    use super::opus_types_h::opus_int32;
    use super::arch_h::opus_val32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/mapping_matrix.h:36"]
pub mod mapping_matrix_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:16"]
    pub struct MappingMatrix {
        pub rows: libc::c_int,
        pub cols: libc::c_int,
        pub gain: libc::c_int,
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/float_cast.h:33"]
pub mod float_cast_h {
    #[inline]
    #[c2rust::src_loc = "137:1"]
    pub unsafe extern "C" fn FLOAT2INT16(mut x: libc::c_float) -> opus_int16 {
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
        return float2int(x) as opus_int16;
    }
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn float2int(mut x: libc::c_float) -> opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }
    use super::opus_types_h::{opus_int16, opus_int32};
    use super::arch_h::CELT_SIG_SCALE;
    use super::xmmintrin_h::{_mm_cvt_ss2si, _mm_set_ss};
}
pub use self::types_h::{__int16_t, __int32_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::opus_types_h::{opus_int16, opus_int32};
pub use self::arch_h::{opus_val16, opus_val32, CELT_SIG_SCALE, celt_fatal};
pub use self::opus_private_h::{foo, C2RustUnnamed, align};
pub use self::mapping_matrix_h::MappingMatrix;
pub use self::float_cast_h::{FLOAT2INT16, float2int};
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn mapping_matrix_get_size(
    mut rows: libc::c_int,
    mut cols: libc::c_int,
) -> opus_int32 {
    let mut size: opus_int32 = 0;
    if rows > 255 as libc::c_int || cols > 255 as libc::c_int {
        return 0 as libc::c_int;
    }
    size = ((rows * cols) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong)
        as opus_int32;
    if size > 65004 as libc::c_int {
        return 0 as libc::c_int;
    }
    return align(::core::mem::size_of::<MappingMatrix>() as libc::c_ulong as libc::c_int)
        + align(size);
}
#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn mapping_matrix_get_data(
    mut matrix: *const MappingMatrix,
) -> *mut opus_int16 {
    return (matrix as *mut libc::c_char)
        .offset(
            align(
                ::core::mem::size_of::<MappingMatrix>() as libc::c_ulong as libc::c_int,
            ) as isize,
        ) as *mut libc::c_void as *mut opus_int16;
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn mapping_matrix_init(
    matrix: *mut MappingMatrix,
    mut rows: libc::c_int,
    mut cols: libc::c_int,
    mut gain: libc::c_int,
    mut data: *const opus_int16,
    mut data_size: opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut ptr: *mut opus_int16 = 0 as *mut opus_int16;
    if !(align(data_size)
        == align(
            ((rows * cols) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong)
                as libc::c_int,
        ))
    {
        celt_fatal(
            b"assertion failed: align(data_size) == align(rows * cols * sizeof(opus_int16))\0"
                as *const u8 as *const libc::c_char,
            b"src/mapping_matrix.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
        );
    }
    (*matrix).rows = rows;
    (*matrix).cols = cols;
    (*matrix).gain = gain;
    ptr = mapping_matrix_get_data(matrix);
    i = 0 as libc::c_int;
    while i < rows * cols {
        *ptr.offset(i as isize) = *data.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn mapping_matrix_multiply_channel_in_float(
    mut matrix: *const MappingMatrix,
    mut input: *const libc::c_float,
    mut input_rows: libc::c_int,
    mut output: *mut opus_val16,
    mut output_row: libc::c_int,
    mut output_rows: libc::c_int,
    mut frame_size: libc::c_int,
) {
    let mut matrix_data: *mut opus_int16 = 0 as *mut opus_int16;
    let mut i: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    if !(input_rows <= (*matrix).cols && output_rows <= (*matrix).rows) {
        celt_fatal(
            b"assertion failed: input_rows <= matrix->cols && output_rows <= matrix->rows\0"
                as *const u8 as *const libc::c_char,
            b"src/mapping_matrix.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
        );
    }
    matrix_data = mapping_matrix_get_data(matrix);
    i = 0 as libc::c_int;
    while i < frame_size {
        let mut tmp: libc::c_float = 0 as libc::c_int as libc::c_float;
        col = 0 as libc::c_int;
        while col < input_rows {
            tmp
                += *matrix_data.offset(((*matrix).rows * col + output_row) as isize)
                    as libc::c_int as libc::c_float
                    * *input.offset((input_rows * i + col) as isize);
            col += 1;
        }
        *output
            .offset(
                (output_rows * i) as isize,
            ) = 1 as libc::c_int as libc::c_float / 32768.0f32 * tmp;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn mapping_matrix_multiply_channel_out_float(
    mut matrix: *const MappingMatrix,
    mut input: *const opus_val16,
    mut input_row: libc::c_int,
    mut input_rows: libc::c_int,
    mut output: *mut libc::c_float,
    mut output_rows: libc::c_int,
    mut frame_size: libc::c_int,
) {
    let mut matrix_data: *mut opus_int16 = 0 as *mut opus_int16;
    let mut i: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut input_sample: libc::c_float = 0.;
    if !(input_rows <= (*matrix).cols && output_rows <= (*matrix).rows) {
        celt_fatal(
            b"assertion failed: input_rows <= matrix->cols && output_rows <= matrix->rows\0"
                as *const u8 as *const libc::c_char,
            b"src/mapping_matrix.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
        );
    }
    matrix_data = mapping_matrix_get_data(matrix);
    i = 0 as libc::c_int;
    while i < frame_size {
        input_sample = *input.offset((input_rows * i) as isize);
        row = 0 as libc::c_int;
        while row < output_rows {
            let mut tmp: libc::c_float = 1 as libc::c_int as libc::c_float / 32768.0f32
                * *matrix_data.offset(((*matrix).rows * input_row + row) as isize)
                    as libc::c_int as libc::c_float * input_sample;
            *output.offset((output_rows * i + row) as isize) += tmp;
            row += 1;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "156:1"]
pub unsafe extern "C" fn mapping_matrix_multiply_channel_in_short(
    mut matrix: *const MappingMatrix,
    mut input: *const opus_int16,
    mut input_rows: libc::c_int,
    mut output: *mut opus_val16,
    mut output_row: libc::c_int,
    mut output_rows: libc::c_int,
    mut frame_size: libc::c_int,
) {
    let mut matrix_data: *mut opus_int16 = 0 as *mut opus_int16;
    let mut i: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    if !(input_rows <= (*matrix).cols && output_rows <= (*matrix).rows) {
        celt_fatal(
            b"assertion failed: input_rows <= matrix->cols && output_rows <= matrix->rows\0"
                as *const u8 as *const libc::c_char,
            b"src/mapping_matrix.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
        );
    }
    matrix_data = mapping_matrix_get_data(matrix);
    i = 0 as libc::c_int;
    while i < frame_size {
        let mut tmp: opus_val32 = 0 as libc::c_int as opus_val32;
        col = 0 as libc::c_int;
        while col < input_rows {
            tmp
                += (*matrix_data.offset(((*matrix).rows * col + output_row) as isize)
                    as libc::c_int
                    * *input.offset((input_rows * i + col) as isize) as libc::c_int)
                    as libc::c_float;
            col += 1;
        }
        *output
            .offset(
                (output_rows * i) as isize,
            ) = 1 as libc::c_int as libc::c_float / (32768.0f32 * 32768.0f32) * tmp;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "196:1"]
pub unsafe extern "C" fn mapping_matrix_multiply_channel_out_short(
    mut matrix: *const MappingMatrix,
    mut input: *const opus_val16,
    mut input_row: libc::c_int,
    mut input_rows: libc::c_int,
    mut output: *mut opus_int16,
    mut output_rows: libc::c_int,
    mut frame_size: libc::c_int,
) {
    let mut matrix_data: *mut opus_int16 = 0 as *mut opus_int16;
    let mut i: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut input_sample: opus_int32 = 0;
    if !(input_rows <= (*matrix).cols && output_rows <= (*matrix).rows) {
        celt_fatal(
            b"assertion failed: input_rows <= matrix->cols && output_rows <= matrix->rows\0"
                as *const u8 as *const libc::c_char,
            b"src/mapping_matrix.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int,
        );
    }
    matrix_data = mapping_matrix_get_data(matrix);
    i = 0 as libc::c_int;
    while i < frame_size {
        input_sample = FLOAT2INT16(*input.offset((input_rows * i) as isize))
            as opus_int32;
        row = 0 as libc::c_int;
        while row < output_rows {
            let mut tmp: opus_int32 = *matrix_data
                .offset(((*matrix).rows * input_row + row) as isize) as opus_int32
                * input_sample;
            let ref mut fresh0 = *output.offset((output_rows * i + row) as isize);
            *fresh0 = (*fresh0 as libc::c_int
                + (tmp + 16384 as libc::c_int >> 15 as libc::c_int)) as opus_int16;
            row += 1;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "231:21"]
pub static mut mapping_matrix_foa_mixing: MappingMatrix = {
    let mut init = MappingMatrix {
        rows: 6 as libc::c_int,
        cols: 6 as libc::c_int,
        gain: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "232:18"]
pub static mut mapping_matrix_foa_mixing_data: [opus_int16; 36] = [
    16384 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(16384 as libc::c_int) as opus_int16,
    23170 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    16384 as libc::c_int as opus_int16,
    23170 as libc::c_int as opus_int16,
    16384 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    16384 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(16384 as libc::c_int) as opus_int16,
    -(23170 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    16384 as libc::c_int as opus_int16,
    -(23170 as libc::c_int) as opus_int16,
    16384 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    32767 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    32767 as libc::c_int as opus_int16,
];
#[no_mangle]
#[c2rust::src_loc = "240:21"]
pub static mut mapping_matrix_soa_mixing: MappingMatrix = {
    let mut init = MappingMatrix {
        rows: 11 as libc::c_int,
        cols: 11 as libc::c_int,
        gain: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "241:18"]
pub static mut mapping_matrix_soa_mixing_data: [opus_int16; 121] = [
    10923 as libc::c_int as opus_int16,
    7723 as libc::c_int as opus_int16,
    13377 as libc::c_int as opus_int16,
    -(13377 as libc::c_int) as opus_int16,
    11585 as libc::c_int as opus_int16,
    9459 as libc::c_int as opus_int16,
    7723 as libc::c_int as opus_int16,
    -(16384 as libc::c_int) as opus_int16,
    -(6689 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10923 as libc::c_int as opus_int16,
    7723 as libc::c_int as opus_int16,
    13377 as libc::c_int as opus_int16,
    13377 as libc::c_int as opus_int16,
    -(11585 as libc::c_int) as opus_int16,
    9459 as libc::c_int as opus_int16,
    7723 as libc::c_int as opus_int16,
    16384 as libc::c_int as opus_int16,
    -(6689 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10923 as libc::c_int as opus_int16,
    -(15447 as libc::c_int) as opus_int16,
    13377 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(18919 as libc::c_int) as opus_int16,
    7723 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    13377 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10923 as libc::c_int as opus_int16,
    7723 as libc::c_int as opus_int16,
    -(13377 as libc::c_int) as opus_int16,
    -(13377 as libc::c_int) as opus_int16,
    11585 as libc::c_int as opus_int16,
    -(9459 as libc::c_int) as opus_int16,
    7723 as libc::c_int as opus_int16,
    16384 as libc::c_int as opus_int16,
    -(6689 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10923 as libc::c_int as opus_int16,
    -(7723 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    13377 as libc::c_int as opus_int16,
    -(16384 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(15447 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    9459 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10923 as libc::c_int as opus_int16,
    -(7723 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(13377 as libc::c_int) as opus_int16,
    16384 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(15447 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    9459 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10923 as libc::c_int as opus_int16,
    15447 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(15447 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(18919 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10923 as libc::c_int as opus_int16,
    7723 as libc::c_int as opus_int16,
    -(13377 as libc::c_int) as opus_int16,
    13377 as libc::c_int as opus_int16,
    -(11585 as libc::c_int) as opus_int16,
    -(9459 as libc::c_int) as opus_int16,
    7723 as libc::c_int as opus_int16,
    -(16384 as libc::c_int) as opus_int16,
    -(6689 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10923 as libc::c_int as opus_int16,
    -(15447 as libc::c_int) as opus_int16,
    -(13377 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    18919 as libc::c_int as opus_int16,
    7723 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    13377 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    32767 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    32767 as libc::c_int as opus_int16,
];
#[no_mangle]
#[c2rust::src_loc = "260:21"]
pub static mut mapping_matrix_toa_mixing: MappingMatrix = {
    let mut init = MappingMatrix {
        rows: 18 as libc::c_int,
        cols: 18 as libc::c_int,
        gain: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "261:18"]
pub static mut mapping_matrix_toa_mixing_data: [opus_int16; 324] = [
    8208 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(881 as libc::c_int) as opus_int16,
    14369 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(8192 as libc::c_int) as opus_int16,
    -(4163 as libc::c_int) as opus_int16,
    13218 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    11095 as libc::c_int as opus_int16,
    -(8836 as libc::c_int) as opus_int16,
    -(6218 as libc::c_int) as opus_int16,
    14833 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    -(10161 as libc::c_int) as opus_int16,
    881 as libc::c_int as opus_int16,
    10161 as libc::c_int as opus_int16,
    -(13218 as libc::c_int) as opus_int16,
    -(2944 as libc::c_int) as opus_int16,
    -(8192 as libc::c_int) as opus_int16,
    2944 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(10488 as libc::c_int) as opus_int16,
    -(6218 as libc::c_int) as opus_int16,
    6248 as libc::c_int as opus_int16,
    -(11095 as libc::c_int) as opus_int16,
    -(6248 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(10488 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    10161 as libc::c_int as opus_int16,
    881 as libc::c_int as opus_int16,
    -(10161 as libc::c_int) as opus_int16,
    -(13218 as libc::c_int) as opus_int16,
    2944 as libc::c_int as opus_int16,
    -(8192 as libc::c_int) as opus_int16,
    -(2944 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    10488 as libc::c_int as opus_int16,
    -(6218 as libc::c_int) as opus_int16,
    -(6248 as libc::c_int) as opus_int16,
    -(11095 as libc::c_int) as opus_int16,
    6248 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10488 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8176 as libc::c_int as opus_int16,
    5566 as libc::c_int as opus_int16,
    -(11552 as libc::c_int) as opus_int16,
    5566 as libc::c_int as opus_int16,
    9681 as libc::c_int as opus_int16,
    -(11205 as libc::c_int) as opus_int16,
    8192 as libc::c_int as opus_int16,
    -(11205 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    4920 as libc::c_int as opus_int16,
    -(15158 as libc::c_int) as opus_int16,
    9756 as libc::c_int as opus_int16,
    -(3334 as libc::c_int) as opus_int16,
    9756 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(4920 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8176 as libc::c_int as opus_int16,
    7871 as libc::c_int as opus_int16,
    11552 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    15846 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(9681 as libc::c_int) as opus_int16,
    -(6958 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    13797 as libc::c_int as opus_int16,
    3334 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(15158 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8176 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    11552 as libc::c_int as opus_int16,
    7871 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    15846 as libc::c_int as opus_int16,
    9681 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    3334 as libc::c_int as opus_int16,
    13797 as libc::c_int as opus_int16,
    15158 as libc::c_int as opus_int16,
    6958 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8176 as libc::c_int as opus_int16,
    5566 as libc::c_int as opus_int16,
    -(11552 as libc::c_int) as opus_int16,
    -(5566 as libc::c_int) as opus_int16,
    -(9681 as libc::c_int) as opus_int16,
    -(11205 as libc::c_int) as opus_int16,
    8192 as libc::c_int as opus_int16,
    11205 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    4920 as libc::c_int as opus_int16,
    15158 as libc::c_int as opus_int16,
    9756 as libc::c_int as opus_int16,
    -(3334 as libc::c_int) as opus_int16,
    -(9756 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    4920 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    14369 as libc::c_int as opus_int16,
    -(881 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(4163 as libc::c_int) as opus_int16,
    -(8192 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(13218 as libc::c_int) as opus_int16,
    -(14833 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(8836 as libc::c_int) as opus_int16,
    11095 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    6218 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    10161 as libc::c_int as opus_int16,
    881 as libc::c_int as opus_int16,
    10161 as libc::c_int as opus_int16,
    13218 as libc::c_int as opus_int16,
    2944 as libc::c_int as opus_int16,
    -(8192 as libc::c_int) as opus_int16,
    2944 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10488 as libc::c_int as opus_int16,
    6218 as libc::c_int as opus_int16,
    -(6248 as libc::c_int) as opus_int16,
    -(11095 as libc::c_int) as opus_int16,
    -(6248 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(10488 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    -(14369 as libc::c_int) as opus_int16,
    -(881 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    4163 as libc::c_int as opus_int16,
    -(8192 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(13218 as libc::c_int) as opus_int16,
    14833 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8836 as libc::c_int as opus_int16,
    11095 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    6218 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(881 as libc::c_int) as opus_int16,
    -(14369 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(8192 as libc::c_int) as opus_int16,
    4163 as libc::c_int as opus_int16,
    13218 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    11095 as libc::c_int as opus_int16,
    8836 as libc::c_int as opus_int16,
    -(6218 as libc::c_int) as opus_int16,
    -(14833 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8176 as libc::c_int as opus_int16,
    -(5566 as libc::c_int) as opus_int16,
    -(11552 as libc::c_int) as opus_int16,
    5566 as libc::c_int as opus_int16,
    -(9681 as libc::c_int) as opus_int16,
    11205 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    -(11205 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(4920 as libc::c_int) as opus_int16,
    15158 as libc::c_int as opus_int16,
    -(9756 as libc::c_int) as opus_int16,
    -(3334 as libc::c_int) as opus_int16,
    9756 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(4920 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8176 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    11552 as libc::c_int as opus_int16,
    -(7871 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    -(15846 as libc::c_int) as opus_int16,
    9681 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    3334 as libc::c_int as opus_int16,
    -(13797 as libc::c_int) as opus_int16,
    15158 as libc::c_int as opus_int16,
    -(6958 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8176 as libc::c_int as opus_int16,
    -(7871 as libc::c_int) as opus_int16,
    11552 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(15846 as libc::c_int) as opus_int16,
    8192 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(9681 as libc::c_int) as opus_int16,
    6958 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(13797 as libc::c_int) as opus_int16,
    3334 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(15158 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8176 as libc::c_int as opus_int16,
    -(5566 as libc::c_int) as opus_int16,
    -(11552 as libc::c_int) as opus_int16,
    -(5566 as libc::c_int) as opus_int16,
    9681 as libc::c_int as opus_int16,
    11205 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    11205 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(4920 as libc::c_int) as opus_int16,
    -(15158 as libc::c_int) as opus_int16,
    -(9756 as libc::c_int) as opus_int16,
    -(3334 as libc::c_int) as opus_int16,
    -(9756 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    4920 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    -(10161 as libc::c_int) as opus_int16,
    881 as libc::c_int as opus_int16,
    -(10161 as libc::c_int) as opus_int16,
    13218 as libc::c_int as opus_int16,
    -(2944 as libc::c_int) as opus_int16,
    -(8192 as libc::c_int) as opus_int16,
    -(2944 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(10488 as libc::c_int) as opus_int16,
    6218 as libc::c_int as opus_int16,
    6248 as libc::c_int as opus_int16,
    -(11095 as libc::c_int) as opus_int16,
    6248 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10488 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    32767 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    32767 as libc::c_int as opus_int16,
];
#[no_mangle]
#[c2rust::src_loc = "305:21"]
pub static mut mapping_matrix_foa_demixing: MappingMatrix = {
    let mut init = MappingMatrix {
        rows: 6 as libc::c_int,
        cols: 6 as libc::c_int,
        gain: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "306:18"]
pub static mut mapping_matrix_foa_demixing_data: [opus_int16; 36] = [
    16384 as libc::c_int as opus_int16,
    16384 as libc::c_int as opus_int16,
    16384 as libc::c_int as opus_int16,
    16384 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    23170 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(23170 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(16384 as libc::c_int) as opus_int16,
    16384 as libc::c_int as opus_int16,
    -(16384 as libc::c_int) as opus_int16,
    16384 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    23170 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(23170 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    32767 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    32767 as libc::c_int as opus_int16,
];
#[no_mangle]
#[c2rust::src_loc = "314:21"]
pub static mut mapping_matrix_soa_demixing: MappingMatrix = {
    let mut init = MappingMatrix {
        rows: 11 as libc::c_int,
        cols: 11 as libc::c_int,
        gain: 3050 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "315:18"]
pub static mut mapping_matrix_soa_demixing_data: [opus_int16; 121] = [
    2771 as libc::c_int as opus_int16,
    2771 as libc::c_int as opus_int16,
    2771 as libc::c_int as opus_int16,
    2771 as libc::c_int as opus_int16,
    2771 as libc::c_int as opus_int16,
    2771 as libc::c_int as opus_int16,
    2771 as libc::c_int as opus_int16,
    2771 as libc::c_int as opus_int16,
    2771 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10033 as libc::c_int as opus_int16,
    10033 as libc::c_int as opus_int16,
    -(20066 as libc::c_int) as opus_int16,
    10033 as libc::c_int as opus_int16,
    14189 as libc::c_int as opus_int16,
    14189 as libc::c_int as opus_int16,
    -(28378 as libc::c_int) as opus_int16,
    10033 as libc::c_int as opus_int16,
    -(20066 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    3393 as libc::c_int as opus_int16,
    3393 as libc::c_int as opus_int16,
    3393 as libc::c_int as opus_int16,
    -(3393 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(3393 as libc::c_int) as opus_int16,
    -(3393 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(17378 as libc::c_int) as opus_int16,
    17378 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(17378 as libc::c_int) as opus_int16,
    -(24576 as libc::c_int) as opus_int16,
    24576 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    17378 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(14189 as libc::c_int) as opus_int16,
    14189 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(14189 as libc::c_int) as opus_int16,
    -(28378 as libc::c_int) as opus_int16,
    28378 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    14189 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    2399 as libc::c_int as opus_int16,
    2399 as libc::c_int as opus_int16,
    -(4799 as libc::c_int) as opus_int16,
    -(2399 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(2399 as libc::c_int) as opus_int16,
    4799 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    1959 as libc::c_int as opus_int16,
    1959 as libc::c_int as opus_int16,
    1959 as libc::c_int as opus_int16,
    1959 as libc::c_int as opus_int16,
    -(3918 as libc::c_int) as opus_int16,
    -(3918 as libc::c_int) as opus_int16,
    -(3918 as libc::c_int) as opus_int16,
    1959 as libc::c_int as opus_int16,
    1959 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(4156 as libc::c_int) as opus_int16,
    4156 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    4156 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(4156 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    -(16384 as libc::c_int) as opus_int16,
    8192 as libc::c_int as opus_int16,
    16384 as libc::c_int as opus_int16,
    16384 as libc::c_int as opus_int16,
    -(32768 as libc::c_int) as opus_int16,
    8192 as libc::c_int as opus_int16,
    -(16384 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8312 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8312 as libc::c_int as opus_int16,
];
#[no_mangle]
#[c2rust::src_loc = "334:21"]
pub static mut mapping_matrix_toa_demixing: MappingMatrix = {
    let mut init = MappingMatrix {
        rows: 18 as libc::c_int,
        cols: 18 as libc::c_int,
        gain: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "335:18"]
pub static mut mapping_matrix_toa_demixing_data: [opus_int16; 324] = [
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(9779 as libc::c_int) as opus_int16,
    9779 as libc::c_int as opus_int16,
    6263 as libc::c_int as opus_int16,
    8857 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    6263 as libc::c_int as opus_int16,
    13829 as libc::c_int as opus_int16,
    9779 as libc::c_int as opus_int16,
    -(13829 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(6263 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(8857 as libc::c_int) as opus_int16,
    -(6263 as libc::c_int) as opus_int16,
    -(9779 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(3413 as libc::c_int) as opus_int16,
    3413 as libc::c_int as opus_int16,
    3413 as libc::c_int as opus_int16,
    -(11359 as libc::c_int) as opus_int16,
    11359 as libc::c_int as opus_int16,
    11359 as libc::c_int as opus_int16,
    -(11359 as libc::c_int) as opus_int16,
    -(3413 as libc::c_int) as opus_int16,
    3413 as libc::c_int as opus_int16,
    -(3413 as libc::c_int) as opus_int16,
    -(3413 as libc::c_int) as opus_int16,
    -(11359 as libc::c_int) as opus_int16,
    11359 as libc::c_int as opus_int16,
    11359 as libc::c_int as opus_int16,
    -(11359 as libc::c_int) as opus_int16,
    3413 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    13829 as libc::c_int as opus_int16,
    9779 as libc::c_int as opus_int16,
    -(9779 as libc::c_int) as opus_int16,
    6263 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    8857 as libc::c_int as opus_int16,
    -(6263 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    9779 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(13829 as libc::c_int) as opus_int16,
    6263 as libc::c_int as opus_int16,
    -(8857 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(6263 as libc::c_int) as opus_int16,
    -(9779 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(15617 as libc::c_int) as opus_int16,
    -(15617 as libc::c_int) as opus_int16,
    6406 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(6406 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    15617 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(6406 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    6406 as libc::c_int as opus_int16,
    15617 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(5003 as libc::c_int) as opus_int16,
    5003 as libc::c_int as opus_int16,
    -(10664 as libc::c_int) as opus_int16,
    15081 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(10664 as libc::c_int) as opus_int16,
    -(7075 as libc::c_int) as opus_int16,
    5003 as libc::c_int as opus_int16,
    7075 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10664 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(15081 as libc::c_int) as opus_int16,
    10664 as libc::c_int as opus_int16,
    -(5003 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(8176 as libc::c_int) as opus_int16,
    -(8176 as libc::c_int) as opus_int16,
    -(8176 as libc::c_int) as opus_int16,
    8208 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    -(8176 as libc::c_int) as opus_int16,
    -(8176 as libc::c_int) as opus_int16,
    -(8176 as libc::c_int) as opus_int16,
    -(8176 as libc::c_int) as opus_int16,
    8208 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    8208 as libc::c_int as opus_int16,
    -(8176 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(7075 as libc::c_int) as opus_int16,
    5003 as libc::c_int as opus_int16,
    -(5003 as libc::c_int) as opus_int16,
    -(10664 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    15081 as libc::c_int as opus_int16,
    10664 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    5003 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    7075 as libc::c_int as opus_int16,
    -(10664 as libc::c_int) as opus_int16,
    -(15081 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    10664 as libc::c_int as opus_int16,
    -(5003 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    15617 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(6406 as libc::c_int) as opus_int16,
    6406 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(15617 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(15617 as libc::c_int) as opus_int16,
    15617 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    6406 as libc::c_int as opus_int16,
    -(6406 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(11393 as libc::c_int) as opus_int16,
    11393 as libc::c_int as opus_int16,
    2993 as libc::c_int as opus_int16,
    -(4233 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    2993 as libc::c_int as opus_int16,
    -(16112 as libc::c_int) as opus_int16,
    11393 as libc::c_int as opus_int16,
    16112 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(2993 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    4233 as libc::c_int as opus_int16,
    -(2993 as libc::c_int) as opus_int16,
    -(11393 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(9974 as libc::c_int) as opus_int16,
    -(9974 as libc::c_int) as opus_int16,
    -(13617 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    13617 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    9974 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    13617 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(13617 as libc::c_int) as opus_int16,
    9974 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    5579 as libc::c_int as opus_int16,
    -(5579 as libc::c_int) as opus_int16,
    10185 as libc::c_int as opus_int16,
    14403 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    10185 as libc::c_int as opus_int16,
    -(7890 as libc::c_int) as opus_int16,
    -(5579 as libc::c_int) as opus_int16,
    7890 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(10185 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(14403 as libc::c_int) as opus_int16,
    -(10185 as libc::c_int) as opus_int16,
    5579 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    11826 as libc::c_int as opus_int16,
    -(11826 as libc::c_int) as opus_int16,
    -(11826 as libc::c_int) as opus_int16,
    -(901 as libc::c_int) as opus_int16,
    901 as libc::c_int as opus_int16,
    901 as libc::c_int as opus_int16,
    -(901 as libc::c_int) as opus_int16,
    11826 as libc::c_int as opus_int16,
    -(11826 as libc::c_int) as opus_int16,
    11826 as libc::c_int as opus_int16,
    11826 as libc::c_int as opus_int16,
    -(901 as libc::c_int) as opus_int16,
    901 as libc::c_int as opus_int16,
    901 as libc::c_int as opus_int16,
    -(901 as libc::c_int) as opus_int16,
    -(11826 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(7890 as libc::c_int) as opus_int16,
    -(5579 as libc::c_int) as opus_int16,
    5579 as libc::c_int as opus_int16,
    10185 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    14403 as libc::c_int as opus_int16,
    -(10185 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(5579 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    7890 as libc::c_int as opus_int16,
    10185 as libc::c_int as opus_int16,
    -(14403 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(10185 as libc::c_int) as opus_int16,
    5579 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(9974 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(13617 as libc::c_int) as opus_int16,
    13617 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    9974 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    9974 as libc::c_int as opus_int16,
    -(9974 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    13617 as libc::c_int as opus_int16,
    -(13617 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    16112 as libc::c_int as opus_int16,
    -(11393 as libc::c_int) as opus_int16,
    11393 as libc::c_int as opus_int16,
    -(2993 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    4233 as libc::c_int as opus_int16,
    2993 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(11393 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    -(16112 as libc::c_int) as opus_int16,
    -(2993 as libc::c_int) as opus_int16,
    -(4233 as libc::c_int) as opus_int16,
    0 as libc::c_int as opus_int16,
    2993 as libc::c_int as opus_int16,
    11393 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    32767 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    32767 as libc::c_int as opus_int16,
];
