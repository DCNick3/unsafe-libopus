use ::libc;
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
    pub use core::arch::x86::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
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
        pub i: i32,
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
pub use self::arch_h::{celt_fatal, opus_val16, opus_val32, CELT_SIG_SCALE};
pub use self::float_cast_h::{float2int, FLOAT2INT16};
pub use self::mapping_matrix_h::MappingMatrix;
pub use self::opus_private_h::{align, foo, C2RustUnnamed};

#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn mapping_matrix_get_size(rows: libc::c_int, cols: libc::c_int) -> i32 {
    let mut size: i32 = 0;
    if rows > 255 as libc::c_int || cols > 255 as libc::c_int {
        return 0 as libc::c_int;
    }
    size = ((rows * cols) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong) as i32;
    if size > 65004 as libc::c_int {
        return 0 as libc::c_int;
    }
    return align(::core::mem::size_of::<MappingMatrix>() as libc::c_ulong as libc::c_int)
        + align(size);
}
#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn mapping_matrix_get_data(matrix: *const MappingMatrix) -> *mut i16 {
    return (matrix as *mut libc::c_char).offset(align(
        ::core::mem::size_of::<MappingMatrix>() as libc::c_ulong as libc::c_int
    ) as isize) as *mut libc::c_void as *mut i16;
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn mapping_matrix_init(
    matrix: *mut MappingMatrix,
    rows: libc::c_int,
    cols: libc::c_int,
    gain: libc::c_int,
    data: *const i16,
    data_size: i32,
) {
    let mut i: libc::c_int = 0;
    let mut ptr: *mut i16 = 0 as *mut i16;
    if !(align(data_size)
        == align(
            ((rows * cols) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong)
                as libc::c_int,
        ))
    {
        celt_fatal(
            b"assertion failed: align(data_size) == align(rows * cols * sizeof(i16))\0" as *const u8
                as *const libc::c_char,
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
    matrix: *const MappingMatrix,
    input: *const libc::c_float,
    input_rows: libc::c_int,
    output: *mut opus_val16,
    output_row: libc::c_int,
    output_rows: libc::c_int,
    frame_size: libc::c_int,
) {
    let mut matrix_data: *mut i16 = 0 as *mut i16;
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
            tmp += *matrix_data.offset(((*matrix).rows * col + output_row) as isize) as libc::c_int
                as libc::c_float
                * *input.offset((input_rows * i + col) as isize);
            col += 1;
        }
        *output.offset((output_rows * i) as isize) =
            1 as libc::c_int as libc::c_float / 32768.0f32 * tmp;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn mapping_matrix_multiply_channel_out_float(
    matrix: *const MappingMatrix,
    input: *const opus_val16,
    input_row: libc::c_int,
    input_rows: libc::c_int,
    output: *mut libc::c_float,
    output_rows: libc::c_int,
    frame_size: libc::c_int,
) {
    let mut matrix_data: *mut i16 = 0 as *mut i16;
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
            let tmp: libc::c_float = 1 as libc::c_int as libc::c_float / 32768.0f32
                * *matrix_data.offset(((*matrix).rows * input_row + row) as isize) as libc::c_int
                    as libc::c_float
                * input_sample;
            *output.offset((output_rows * i + row) as isize) += tmp;
            row += 1;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "156:1"]
pub unsafe extern "C" fn mapping_matrix_multiply_channel_in_short(
    matrix: *const MappingMatrix,
    input: *const i16,
    input_rows: libc::c_int,
    output: *mut opus_val16,
    output_row: libc::c_int,
    output_rows: libc::c_int,
    frame_size: libc::c_int,
) {
    let mut matrix_data: *mut i16 = 0 as *mut i16;
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
            tmp += (*matrix_data.offset(((*matrix).rows * col + output_row) as isize)
                as libc::c_int
                * *input.offset((input_rows * i + col) as isize) as libc::c_int)
                as libc::c_float;
            col += 1;
        }
        *output.offset((output_rows * i) as isize) =
            1 as libc::c_int as libc::c_float / (32768.0f32 * 32768.0f32) * tmp;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "196:1"]
pub unsafe extern "C" fn mapping_matrix_multiply_channel_out_short(
    matrix: *const MappingMatrix,
    input: *const opus_val16,
    input_row: libc::c_int,
    input_rows: libc::c_int,
    output: *mut i16,
    output_rows: libc::c_int,
    frame_size: libc::c_int,
) {
    let mut matrix_data: *mut i16 = 0 as *mut i16;
    let mut i: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut input_sample: i32 = 0;
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
        input_sample = FLOAT2INT16(*input.offset((input_rows * i) as isize)) as i32;
        row = 0 as libc::c_int;
        while row < output_rows {
            let tmp: i32 = *matrix_data.offset(((*matrix).rows * input_row + row) as isize) as i32
                * input_sample;
            let ref mut fresh0 = *output.offset((output_rows * i + row) as isize);
            *fresh0 =
                (*fresh0 as libc::c_int + (tmp + 16384 as libc::c_int >> 15 as libc::c_int)) as i16;
            row += 1;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "231:21"]
pub static mut mapping_matrix_foa_mixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 6 as libc::c_int,
        cols: 6 as libc::c_int,
        gain: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "232:18"]
pub static mut mapping_matrix_foa_mixing_data: [i16; 36] = [
    16384 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(16384 as libc::c_int) as i16,
    23170 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    16384 as libc::c_int as i16,
    23170 as libc::c_int as i16,
    16384 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    16384 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(16384 as libc::c_int) as i16,
    -(23170 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    16384 as libc::c_int as i16,
    -(23170 as libc::c_int) as i16,
    16384 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    32767 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    32767 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "240:21"]
pub static mut mapping_matrix_soa_mixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 11 as libc::c_int,
        cols: 11 as libc::c_int,
        gain: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "241:18"]
pub static mut mapping_matrix_soa_mixing_data: [i16; 121] = [
    10923 as libc::c_int as i16,
    7723 as libc::c_int as i16,
    13377 as libc::c_int as i16,
    -(13377 as libc::c_int) as i16,
    11585 as libc::c_int as i16,
    9459 as libc::c_int as i16,
    7723 as libc::c_int as i16,
    -(16384 as libc::c_int) as i16,
    -(6689 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10923 as libc::c_int as i16,
    7723 as libc::c_int as i16,
    13377 as libc::c_int as i16,
    13377 as libc::c_int as i16,
    -(11585 as libc::c_int) as i16,
    9459 as libc::c_int as i16,
    7723 as libc::c_int as i16,
    16384 as libc::c_int as i16,
    -(6689 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10923 as libc::c_int as i16,
    -(15447 as libc::c_int) as i16,
    13377 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(18919 as libc::c_int) as i16,
    7723 as libc::c_int as i16,
    0 as libc::c_int as i16,
    13377 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10923 as libc::c_int as i16,
    7723 as libc::c_int as i16,
    -(13377 as libc::c_int) as i16,
    -(13377 as libc::c_int) as i16,
    11585 as libc::c_int as i16,
    -(9459 as libc::c_int) as i16,
    7723 as libc::c_int as i16,
    16384 as libc::c_int as i16,
    -(6689 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10923 as libc::c_int as i16,
    -(7723 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    13377 as libc::c_int as i16,
    -(16384 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(15447 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    9459 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10923 as libc::c_int as i16,
    -(7723 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(13377 as libc::c_int) as i16,
    16384 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(15447 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    9459 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10923 as libc::c_int as i16,
    15447 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(15447 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(18919 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10923 as libc::c_int as i16,
    7723 as libc::c_int as i16,
    -(13377 as libc::c_int) as i16,
    13377 as libc::c_int as i16,
    -(11585 as libc::c_int) as i16,
    -(9459 as libc::c_int) as i16,
    7723 as libc::c_int as i16,
    -(16384 as libc::c_int) as i16,
    -(6689 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10923 as libc::c_int as i16,
    -(15447 as libc::c_int) as i16,
    -(13377 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    18919 as libc::c_int as i16,
    7723 as libc::c_int as i16,
    0 as libc::c_int as i16,
    13377 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    32767 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    32767 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "260:21"]
pub static mut mapping_matrix_toa_mixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 18 as libc::c_int,
        cols: 18 as libc::c_int,
        gain: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "261:18"]
pub static mut mapping_matrix_toa_mixing_data: [i16; 324] = [
    8208 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(881 as libc::c_int) as i16,
    14369 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(8192 as libc::c_int) as i16,
    -(4163 as libc::c_int) as i16,
    13218 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    11095 as libc::c_int as i16,
    -(8836 as libc::c_int) as i16,
    -(6218 as libc::c_int) as i16,
    14833 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    -(10161 as libc::c_int) as i16,
    881 as libc::c_int as i16,
    10161 as libc::c_int as i16,
    -(13218 as libc::c_int) as i16,
    -(2944 as libc::c_int) as i16,
    -(8192 as libc::c_int) as i16,
    2944 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(10488 as libc::c_int) as i16,
    -(6218 as libc::c_int) as i16,
    6248 as libc::c_int as i16,
    -(11095 as libc::c_int) as i16,
    -(6248 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(10488 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    10161 as libc::c_int as i16,
    881 as libc::c_int as i16,
    -(10161 as libc::c_int) as i16,
    -(13218 as libc::c_int) as i16,
    2944 as libc::c_int as i16,
    -(8192 as libc::c_int) as i16,
    -(2944 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    10488 as libc::c_int as i16,
    -(6218 as libc::c_int) as i16,
    -(6248 as libc::c_int) as i16,
    -(11095 as libc::c_int) as i16,
    6248 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10488 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8176 as libc::c_int as i16,
    5566 as libc::c_int as i16,
    -(11552 as libc::c_int) as i16,
    5566 as libc::c_int as i16,
    9681 as libc::c_int as i16,
    -(11205 as libc::c_int) as i16,
    8192 as libc::c_int as i16,
    -(11205 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    4920 as libc::c_int as i16,
    -(15158 as libc::c_int) as i16,
    9756 as libc::c_int as i16,
    -(3334 as libc::c_int) as i16,
    9756 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(4920 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8176 as libc::c_int as i16,
    7871 as libc::c_int as i16,
    11552 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    15846 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(9681 as libc::c_int) as i16,
    -(6958 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    13797 as libc::c_int as i16,
    3334 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(15158 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8176 as libc::c_int as i16,
    0 as libc::c_int as i16,
    11552 as libc::c_int as i16,
    7871 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    15846 as libc::c_int as i16,
    9681 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    3334 as libc::c_int as i16,
    13797 as libc::c_int as i16,
    15158 as libc::c_int as i16,
    6958 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8176 as libc::c_int as i16,
    5566 as libc::c_int as i16,
    -(11552 as libc::c_int) as i16,
    -(5566 as libc::c_int) as i16,
    -(9681 as libc::c_int) as i16,
    -(11205 as libc::c_int) as i16,
    8192 as libc::c_int as i16,
    11205 as libc::c_int as i16,
    0 as libc::c_int as i16,
    4920 as libc::c_int as i16,
    15158 as libc::c_int as i16,
    9756 as libc::c_int as i16,
    -(3334 as libc::c_int) as i16,
    -(9756 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    4920 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    14369 as libc::c_int as i16,
    -(881 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(4163 as libc::c_int) as i16,
    -(8192 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(13218 as libc::c_int) as i16,
    -(14833 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(8836 as libc::c_int) as i16,
    11095 as libc::c_int as i16,
    0 as libc::c_int as i16,
    6218 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    10161 as libc::c_int as i16,
    881 as libc::c_int as i16,
    10161 as libc::c_int as i16,
    13218 as libc::c_int as i16,
    2944 as libc::c_int as i16,
    -(8192 as libc::c_int) as i16,
    2944 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10488 as libc::c_int as i16,
    6218 as libc::c_int as i16,
    -(6248 as libc::c_int) as i16,
    -(11095 as libc::c_int) as i16,
    -(6248 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(10488 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    -(14369 as libc::c_int) as i16,
    -(881 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    4163 as libc::c_int as i16,
    -(8192 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(13218 as libc::c_int) as i16,
    14833 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8836 as libc::c_int as i16,
    11095 as libc::c_int as i16,
    0 as libc::c_int as i16,
    6218 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(881 as libc::c_int) as i16,
    -(14369 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(8192 as libc::c_int) as i16,
    4163 as libc::c_int as i16,
    13218 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    11095 as libc::c_int as i16,
    8836 as libc::c_int as i16,
    -(6218 as libc::c_int) as i16,
    -(14833 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8176 as libc::c_int as i16,
    -(5566 as libc::c_int) as i16,
    -(11552 as libc::c_int) as i16,
    5566 as libc::c_int as i16,
    -(9681 as libc::c_int) as i16,
    11205 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    -(11205 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(4920 as libc::c_int) as i16,
    15158 as libc::c_int as i16,
    -(9756 as libc::c_int) as i16,
    -(3334 as libc::c_int) as i16,
    9756 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(4920 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8176 as libc::c_int as i16,
    0 as libc::c_int as i16,
    11552 as libc::c_int as i16,
    -(7871 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    -(15846 as libc::c_int) as i16,
    9681 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    3334 as libc::c_int as i16,
    -(13797 as libc::c_int) as i16,
    15158 as libc::c_int as i16,
    -(6958 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8176 as libc::c_int as i16,
    -(7871 as libc::c_int) as i16,
    11552 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(15846 as libc::c_int) as i16,
    8192 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(9681 as libc::c_int) as i16,
    6958 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(13797 as libc::c_int) as i16,
    3334 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(15158 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8176 as libc::c_int as i16,
    -(5566 as libc::c_int) as i16,
    -(11552 as libc::c_int) as i16,
    -(5566 as libc::c_int) as i16,
    9681 as libc::c_int as i16,
    11205 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    11205 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(4920 as libc::c_int) as i16,
    -(15158 as libc::c_int) as i16,
    -(9756 as libc::c_int) as i16,
    -(3334 as libc::c_int) as i16,
    -(9756 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    4920 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    -(10161 as libc::c_int) as i16,
    881 as libc::c_int as i16,
    -(10161 as libc::c_int) as i16,
    13218 as libc::c_int as i16,
    -(2944 as libc::c_int) as i16,
    -(8192 as libc::c_int) as i16,
    -(2944 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(10488 as libc::c_int) as i16,
    6218 as libc::c_int as i16,
    6248 as libc::c_int as i16,
    -(11095 as libc::c_int) as i16,
    6248 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10488 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    32767 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    32767 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "305:21"]
pub static mut mapping_matrix_foa_demixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 6 as libc::c_int,
        cols: 6 as libc::c_int,
        gain: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "306:18"]
pub static mut mapping_matrix_foa_demixing_data: [i16; 36] = [
    16384 as libc::c_int as i16,
    16384 as libc::c_int as i16,
    16384 as libc::c_int as i16,
    16384 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    23170 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(23170 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(16384 as libc::c_int) as i16,
    16384 as libc::c_int as i16,
    -(16384 as libc::c_int) as i16,
    16384 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    23170 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(23170 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    32767 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    32767 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "314:21"]
pub static mut mapping_matrix_soa_demixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 11 as libc::c_int,
        cols: 11 as libc::c_int,
        gain: 3050 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "315:18"]
pub static mut mapping_matrix_soa_demixing_data: [i16; 121] = [
    2771 as libc::c_int as i16,
    2771 as libc::c_int as i16,
    2771 as libc::c_int as i16,
    2771 as libc::c_int as i16,
    2771 as libc::c_int as i16,
    2771 as libc::c_int as i16,
    2771 as libc::c_int as i16,
    2771 as libc::c_int as i16,
    2771 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10033 as libc::c_int as i16,
    10033 as libc::c_int as i16,
    -(20066 as libc::c_int) as i16,
    10033 as libc::c_int as i16,
    14189 as libc::c_int as i16,
    14189 as libc::c_int as i16,
    -(28378 as libc::c_int) as i16,
    10033 as libc::c_int as i16,
    -(20066 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    3393 as libc::c_int as i16,
    3393 as libc::c_int as i16,
    3393 as libc::c_int as i16,
    -(3393 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(3393 as libc::c_int) as i16,
    -(3393 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(17378 as libc::c_int) as i16,
    17378 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(17378 as libc::c_int) as i16,
    -(24576 as libc::c_int) as i16,
    24576 as libc::c_int as i16,
    0 as libc::c_int as i16,
    17378 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(14189 as libc::c_int) as i16,
    14189 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(14189 as libc::c_int) as i16,
    -(28378 as libc::c_int) as i16,
    28378 as libc::c_int as i16,
    0 as libc::c_int as i16,
    14189 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    2399 as libc::c_int as i16,
    2399 as libc::c_int as i16,
    -(4799 as libc::c_int) as i16,
    -(2399 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(2399 as libc::c_int) as i16,
    4799 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    1959 as libc::c_int as i16,
    1959 as libc::c_int as i16,
    1959 as libc::c_int as i16,
    1959 as libc::c_int as i16,
    -(3918 as libc::c_int) as i16,
    -(3918 as libc::c_int) as i16,
    -(3918 as libc::c_int) as i16,
    1959 as libc::c_int as i16,
    1959 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(4156 as libc::c_int) as i16,
    4156 as libc::c_int as i16,
    0 as libc::c_int as i16,
    4156 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(4156 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    -(16384 as libc::c_int) as i16,
    8192 as libc::c_int as i16,
    16384 as libc::c_int as i16,
    16384 as libc::c_int as i16,
    -(32768 as libc::c_int) as i16,
    8192 as libc::c_int as i16,
    -(16384 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8312 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8312 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "334:21"]
pub static mut mapping_matrix_toa_demixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 18 as libc::c_int,
        cols: 18 as libc::c_int,
        gain: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "335:18"]
pub static mut mapping_matrix_toa_demixing_data: [i16; 324] = [
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    8192 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(9779 as libc::c_int) as i16,
    9779 as libc::c_int as i16,
    6263 as libc::c_int as i16,
    8857 as libc::c_int as i16,
    0 as libc::c_int as i16,
    6263 as libc::c_int as i16,
    13829 as libc::c_int as i16,
    9779 as libc::c_int as i16,
    -(13829 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(6263 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(8857 as libc::c_int) as i16,
    -(6263 as libc::c_int) as i16,
    -(9779 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(3413 as libc::c_int) as i16,
    3413 as libc::c_int as i16,
    3413 as libc::c_int as i16,
    -(11359 as libc::c_int) as i16,
    11359 as libc::c_int as i16,
    11359 as libc::c_int as i16,
    -(11359 as libc::c_int) as i16,
    -(3413 as libc::c_int) as i16,
    3413 as libc::c_int as i16,
    -(3413 as libc::c_int) as i16,
    -(3413 as libc::c_int) as i16,
    -(11359 as libc::c_int) as i16,
    11359 as libc::c_int as i16,
    11359 as libc::c_int as i16,
    -(11359 as libc::c_int) as i16,
    3413 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    13829 as libc::c_int as i16,
    9779 as libc::c_int as i16,
    -(9779 as libc::c_int) as i16,
    6263 as libc::c_int as i16,
    0 as libc::c_int as i16,
    8857 as libc::c_int as i16,
    -(6263 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    9779 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(13829 as libc::c_int) as i16,
    6263 as libc::c_int as i16,
    -(8857 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(6263 as libc::c_int) as i16,
    -(9779 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(15617 as libc::c_int) as i16,
    -(15617 as libc::c_int) as i16,
    6406 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(6406 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    15617 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(6406 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    6406 as libc::c_int as i16,
    15617 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(5003 as libc::c_int) as i16,
    5003 as libc::c_int as i16,
    -(10664 as libc::c_int) as i16,
    15081 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(10664 as libc::c_int) as i16,
    -(7075 as libc::c_int) as i16,
    5003 as libc::c_int as i16,
    7075 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10664 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(15081 as libc::c_int) as i16,
    10664 as libc::c_int as i16,
    -(5003 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(8176 as libc::c_int) as i16,
    -(8176 as libc::c_int) as i16,
    -(8176 as libc::c_int) as i16,
    8208 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    -(8176 as libc::c_int) as i16,
    -(8176 as libc::c_int) as i16,
    -(8176 as libc::c_int) as i16,
    -(8176 as libc::c_int) as i16,
    8208 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    8208 as libc::c_int as i16,
    -(8176 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(7075 as libc::c_int) as i16,
    5003 as libc::c_int as i16,
    -(5003 as libc::c_int) as i16,
    -(10664 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    15081 as libc::c_int as i16,
    10664 as libc::c_int as i16,
    0 as libc::c_int as i16,
    5003 as libc::c_int as i16,
    0 as libc::c_int as i16,
    7075 as libc::c_int as i16,
    -(10664 as libc::c_int) as i16,
    -(15081 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    10664 as libc::c_int as i16,
    -(5003 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    15617 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(6406 as libc::c_int) as i16,
    6406 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(15617 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(15617 as libc::c_int) as i16,
    15617 as libc::c_int as i16,
    0 as libc::c_int as i16,
    6406 as libc::c_int as i16,
    -(6406 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(11393 as libc::c_int) as i16,
    11393 as libc::c_int as i16,
    2993 as libc::c_int as i16,
    -(4233 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    2993 as libc::c_int as i16,
    -(16112 as libc::c_int) as i16,
    11393 as libc::c_int as i16,
    16112 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(2993 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    4233 as libc::c_int as i16,
    -(2993 as libc::c_int) as i16,
    -(11393 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(9974 as libc::c_int) as i16,
    -(9974 as libc::c_int) as i16,
    -(13617 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    13617 as libc::c_int as i16,
    0 as libc::c_int as i16,
    9974 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    13617 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(13617 as libc::c_int) as i16,
    9974 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    5579 as libc::c_int as i16,
    -(5579 as libc::c_int) as i16,
    10185 as libc::c_int as i16,
    14403 as libc::c_int as i16,
    0 as libc::c_int as i16,
    10185 as libc::c_int as i16,
    -(7890 as libc::c_int) as i16,
    -(5579 as libc::c_int) as i16,
    7890 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(10185 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(14403 as libc::c_int) as i16,
    -(10185 as libc::c_int) as i16,
    5579 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    11826 as libc::c_int as i16,
    -(11826 as libc::c_int) as i16,
    -(11826 as libc::c_int) as i16,
    -(901 as libc::c_int) as i16,
    901 as libc::c_int as i16,
    901 as libc::c_int as i16,
    -(901 as libc::c_int) as i16,
    11826 as libc::c_int as i16,
    -(11826 as libc::c_int) as i16,
    11826 as libc::c_int as i16,
    11826 as libc::c_int as i16,
    -(901 as libc::c_int) as i16,
    901 as libc::c_int as i16,
    901 as libc::c_int as i16,
    -(901 as libc::c_int) as i16,
    -(11826 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(7890 as libc::c_int) as i16,
    -(5579 as libc::c_int) as i16,
    5579 as libc::c_int as i16,
    10185 as libc::c_int as i16,
    0 as libc::c_int as i16,
    14403 as libc::c_int as i16,
    -(10185 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(5579 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    7890 as libc::c_int as i16,
    10185 as libc::c_int as i16,
    -(14403 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(10185 as libc::c_int) as i16,
    5579 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(9974 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(13617 as libc::c_int) as i16,
    13617 as libc::c_int as i16,
    0 as libc::c_int as i16,
    9974 as libc::c_int as i16,
    0 as libc::c_int as i16,
    9974 as libc::c_int as i16,
    -(9974 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    13617 as libc::c_int as i16,
    -(13617 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    16112 as libc::c_int as i16,
    -(11393 as libc::c_int) as i16,
    11393 as libc::c_int as i16,
    -(2993 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    4233 as libc::c_int as i16,
    2993 as libc::c_int as i16,
    0 as libc::c_int as i16,
    -(11393 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    -(16112 as libc::c_int) as i16,
    -(2993 as libc::c_int) as i16,
    -(4233 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    2993 as libc::c_int as i16,
    11393 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    32767 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    0 as libc::c_int as i16,
    32767 as libc::c_int as i16,
];
