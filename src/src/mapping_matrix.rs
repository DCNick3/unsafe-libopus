pub mod arch_h {
    pub type opus_val16 = f32;
    pub type opus_val32 = f32;
}
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
}
pub use self::arch_h::{opus_val16, opus_val32};
use crate::celt::celt::celt_fatal;
use crate::celt::float_cast::FLOAT2INT16;
use crate::src::opus_private::align;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MappingMatrix {
    pub rows: i32,
    pub cols: i32,
    pub gain: i32,
}

pub unsafe fn mapping_matrix_get_size(rows: i32, cols: i32) -> i32 {
    let mut size: i32 = 0;
    if rows > 255 as i32 || cols > 255 as i32 {
        return 0 as i32;
    }
    size = ((rows * cols) as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64) as i32;
    if size > 65004 as i32 {
        return 0 as i32;
    }
    return align(::core::mem::size_of::<MappingMatrix>() as u64 as i32) + align(size);
}
pub unsafe fn mapping_matrix_get_data(matrix: *const MappingMatrix) -> *mut i16 {
    return (matrix as *mut i8)
        .offset(align(::core::mem::size_of::<MappingMatrix>() as u64 as i32) as isize)
        as *mut core::ffi::c_void as *mut i16;
}
pub unsafe fn mapping_matrix_init(
    matrix: *mut MappingMatrix,
    rows: i32,
    cols: i32,
    gain: i32,
    data: *const i16,
    data_size: i32,
) {
    let mut i: i32 = 0;
    let mut ptr: *mut i16 = 0 as *mut i16;
    if !(align(data_size)
        == align(((rows * cols) as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64) as i32))
    {
        celt_fatal(
            b"assertion failed: align(data_size) == align(rows * cols * sizeof(i16))\0" as *const u8
                as *const i8,
            b"src/mapping_matrix.c\0" as *const u8 as *const i8,
            72 as i32,
        );
    }
    (*matrix).rows = rows;
    (*matrix).cols = cols;
    (*matrix).gain = gain;
    ptr = mapping_matrix_get_data(matrix);
    i = 0 as i32;
    while i < rows * cols {
        *ptr.offset(i as isize) = *data.offset(i as isize);
        i += 1;
    }
}
pub unsafe fn mapping_matrix_multiply_channel_in_float(
    matrix: *const MappingMatrix,
    input: *const f32,
    input_rows: i32,
    output: *mut opus_val16,
    output_row: i32,
    output_rows: i32,
    frame_size: i32,
) {
    let mut matrix_data: *mut i16 = 0 as *mut i16;
    let mut i: i32 = 0;
    let mut col: i32 = 0;
    if !(input_rows <= (*matrix).cols && output_rows <= (*matrix).rows) {
        celt_fatal(
            b"assertion failed: input_rows <= matrix->cols && output_rows <= matrix->rows\0"
                as *const u8 as *const i8,
            b"src/mapping_matrix.c\0" as *const u8 as *const i8,
            98 as i32,
        );
    }
    matrix_data = mapping_matrix_get_data(matrix);
    i = 0 as i32;
    while i < frame_size {
        let mut tmp: f32 = 0 as i32 as f32;
        col = 0 as i32;
        while col < input_rows {
            tmp += *matrix_data.offset(((*matrix).rows * col + output_row) as isize) as i32 as f32
                * *input.offset((input_rows * i + col) as isize);
            col += 1;
        }
        *output.offset((output_rows * i) as isize) = 1 as i32 as f32 / 32768.0f32 * tmp;
        i += 1;
    }
}
pub unsafe fn mapping_matrix_multiply_channel_out_float(
    matrix: *const MappingMatrix,
    input: *const opus_val16,
    input_row: i32,
    input_rows: i32,
    output: *mut f32,
    output_rows: i32,
    frame_size: i32,
) {
    let mut matrix_data: *mut i16 = 0 as *mut i16;
    let mut i: i32 = 0;
    let mut row: i32 = 0;
    let mut input_sample: f32 = 0.;
    if !(input_rows <= (*matrix).cols && output_rows <= (*matrix).rows) {
        celt_fatal(
            b"assertion failed: input_rows <= matrix->cols && output_rows <= matrix->rows\0"
                as *const u8 as *const i8,
            b"src/mapping_matrix.c\0" as *const u8 as *const i8,
            134 as i32,
        );
    }
    matrix_data = mapping_matrix_get_data(matrix);
    i = 0 as i32;
    while i < frame_size {
        input_sample = *input.offset((input_rows * i) as isize);
        row = 0 as i32;
        while row < output_rows {
            let tmp: f32 = 1 as i32 as f32 / 32768.0f32
                * *matrix_data.offset(((*matrix).rows * input_row + row) as isize) as i32 as f32
                * input_sample;
            *output.offset((output_rows * i + row) as isize) += tmp;
            row += 1;
        }
        i += 1;
    }
}
pub unsafe fn mapping_matrix_multiply_channel_in_short(
    matrix: *const MappingMatrix,
    input: *const i16,
    input_rows: i32,
    output: *mut opus_val16,
    output_row: i32,
    output_rows: i32,
    frame_size: i32,
) {
    let mut matrix_data: *mut i16 = 0 as *mut i16;
    let mut i: i32 = 0;
    let mut col: i32 = 0;
    if !(input_rows <= (*matrix).cols && output_rows <= (*matrix).rows) {
        celt_fatal(
            b"assertion failed: input_rows <= matrix->cols && output_rows <= matrix->rows\0"
                as *const u8 as *const i8,
            b"src/mapping_matrix.c\0" as *const u8 as *const i8,
            169 as i32,
        );
    }
    matrix_data = mapping_matrix_get_data(matrix);
    i = 0 as i32;
    while i < frame_size {
        let mut tmp: opus_val32 = 0 as i32 as opus_val32;
        col = 0 as i32;
        while col < input_rows {
            tmp += (*matrix_data.offset(((*matrix).rows * col + output_row) as isize) as i32
                * *input.offset((input_rows * i + col) as isize) as i32) as f32;
            col += 1;
        }
        *output.offset((output_rows * i) as isize) =
            1 as i32 as f32 / (32768.0f32 * 32768.0f32) * tmp;
        i += 1;
    }
}
pub unsafe fn mapping_matrix_multiply_channel_out_short(
    matrix: *const MappingMatrix,
    input: *const opus_val16,
    input_row: i32,
    input_rows: i32,
    output: *mut i16,
    output_rows: i32,
    frame_size: i32,
) {
    let mut matrix_data: *mut i16 = 0 as *mut i16;
    let mut i: i32 = 0;
    let mut row: i32 = 0;
    let mut input_sample: i32 = 0;
    if !(input_rows <= (*matrix).cols && output_rows <= (*matrix).rows) {
        celt_fatal(
            b"assertion failed: input_rows <= matrix->cols && output_rows <= matrix->rows\0"
                as *const u8 as *const i8,
            b"src/mapping_matrix.c\0" as *const u8 as *const i8,
            210 as i32,
        );
    }
    matrix_data = mapping_matrix_get_data(matrix);
    i = 0 as i32;
    while i < frame_size {
        input_sample = FLOAT2INT16(*input.offset((input_rows * i) as isize)) as i32;
        row = 0 as i32;
        while row < output_rows {
            let tmp: i32 = *matrix_data.offset(((*matrix).rows * input_row + row) as isize) as i32
                * input_sample;
            let ref mut fresh0 = *output.offset((output_rows * i + row) as isize);
            *fresh0 = (*fresh0 as i32 + (tmp + 16384 as i32 >> 15 as i32)) as i16;
            row += 1;
        }
        i += 1;
    }
}
pub static mut mapping_matrix_foa_mixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 6 as i32,
        cols: 6 as i32,
        gain: 0 as i32,
    };
    init
};
pub static mut mapping_matrix_foa_mixing_data: [i16; 36] = [
    16384 as i32 as i16,
    0 as i32 as i16,
    -(16384 as i32) as i16,
    23170 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    16384 as i32 as i16,
    23170 as i32 as i16,
    16384 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    16384 as i32 as i16,
    0 as i32 as i16,
    -(16384 as i32) as i16,
    -(23170 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    16384 as i32 as i16,
    -(23170 as i32) as i16,
    16384 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    32767 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    32767 as i32 as i16,
];
pub static mut mapping_matrix_soa_mixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 11 as i32,
        cols: 11 as i32,
        gain: 0 as i32,
    };
    init
};
pub static mut mapping_matrix_soa_mixing_data: [i16; 121] = [
    10923 as i32 as i16,
    7723 as i32 as i16,
    13377 as i32 as i16,
    -(13377 as i32) as i16,
    11585 as i32 as i16,
    9459 as i32 as i16,
    7723 as i32 as i16,
    -(16384 as i32) as i16,
    -(6689 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    10923 as i32 as i16,
    7723 as i32 as i16,
    13377 as i32 as i16,
    13377 as i32 as i16,
    -(11585 as i32) as i16,
    9459 as i32 as i16,
    7723 as i32 as i16,
    16384 as i32 as i16,
    -(6689 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    10923 as i32 as i16,
    -(15447 as i32) as i16,
    13377 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(18919 as i32) as i16,
    7723 as i32 as i16,
    0 as i32 as i16,
    13377 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    10923 as i32 as i16,
    7723 as i32 as i16,
    -(13377 as i32) as i16,
    -(13377 as i32) as i16,
    11585 as i32 as i16,
    -(9459 as i32) as i16,
    7723 as i32 as i16,
    16384 as i32 as i16,
    -(6689 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    10923 as i32 as i16,
    -(7723 as i32) as i16,
    0 as i32 as i16,
    13377 as i32 as i16,
    -(16384 as i32) as i16,
    0 as i32 as i16,
    -(15447 as i32) as i16,
    0 as i32 as i16,
    9459 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    10923 as i32 as i16,
    -(7723 as i32) as i16,
    0 as i32 as i16,
    -(13377 as i32) as i16,
    16384 as i32 as i16,
    0 as i32 as i16,
    -(15447 as i32) as i16,
    0 as i32 as i16,
    9459 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    10923 as i32 as i16,
    15447 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(15447 as i32) as i16,
    0 as i32 as i16,
    -(18919 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    10923 as i32 as i16,
    7723 as i32 as i16,
    -(13377 as i32) as i16,
    13377 as i32 as i16,
    -(11585 as i32) as i16,
    -(9459 as i32) as i16,
    7723 as i32 as i16,
    -(16384 as i32) as i16,
    -(6689 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    10923 as i32 as i16,
    -(15447 as i32) as i16,
    -(13377 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    18919 as i32 as i16,
    7723 as i32 as i16,
    0 as i32 as i16,
    13377 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    32767 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    32767 as i32 as i16,
];
pub static mut mapping_matrix_toa_mixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 18 as i32,
        cols: 18 as i32,
        gain: 0 as i32,
    };
    init
};
pub static mut mapping_matrix_toa_mixing_data: [i16; 324] = [
    8208 as i32 as i16,
    0 as i32 as i16,
    -(881 as i32) as i16,
    14369 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(8192 as i32) as i16,
    -(4163 as i32) as i16,
    13218 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    11095 as i32 as i16,
    -(8836 as i32) as i16,
    -(6218 as i32) as i16,
    14833 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8208 as i32 as i16,
    -(10161 as i32) as i16,
    881 as i32 as i16,
    10161 as i32 as i16,
    -(13218 as i32) as i16,
    -(2944 as i32) as i16,
    -(8192 as i32) as i16,
    2944 as i32 as i16,
    0 as i32 as i16,
    -(10488 as i32) as i16,
    -(6218 as i32) as i16,
    6248 as i32 as i16,
    -(11095 as i32) as i16,
    -(6248 as i32) as i16,
    0 as i32 as i16,
    -(10488 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8208 as i32 as i16,
    10161 as i32 as i16,
    881 as i32 as i16,
    -(10161 as i32) as i16,
    -(13218 as i32) as i16,
    2944 as i32 as i16,
    -(8192 as i32) as i16,
    -(2944 as i32) as i16,
    0 as i32 as i16,
    10488 as i32 as i16,
    -(6218 as i32) as i16,
    -(6248 as i32) as i16,
    -(11095 as i32) as i16,
    6248 as i32 as i16,
    0 as i32 as i16,
    10488 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8176 as i32 as i16,
    5566 as i32 as i16,
    -(11552 as i32) as i16,
    5566 as i32 as i16,
    9681 as i32 as i16,
    -(11205 as i32) as i16,
    8192 as i32 as i16,
    -(11205 as i32) as i16,
    0 as i32 as i16,
    4920 as i32 as i16,
    -(15158 as i32) as i16,
    9756 as i32 as i16,
    -(3334 as i32) as i16,
    9756 as i32 as i16,
    0 as i32 as i16,
    -(4920 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8176 as i32 as i16,
    7871 as i32 as i16,
    11552 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    15846 as i32 as i16,
    8192 as i32 as i16,
    0 as i32 as i16,
    -(9681 as i32) as i16,
    -(6958 as i32) as i16,
    0 as i32 as i16,
    13797 as i32 as i16,
    3334 as i32 as i16,
    0 as i32 as i16,
    -(15158 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8176 as i32 as i16,
    0 as i32 as i16,
    11552 as i32 as i16,
    7871 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8192 as i32 as i16,
    15846 as i32 as i16,
    9681 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    3334 as i32 as i16,
    13797 as i32 as i16,
    15158 as i32 as i16,
    6958 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8176 as i32 as i16,
    5566 as i32 as i16,
    -(11552 as i32) as i16,
    -(5566 as i32) as i16,
    -(9681 as i32) as i16,
    -(11205 as i32) as i16,
    8192 as i32 as i16,
    11205 as i32 as i16,
    0 as i32 as i16,
    4920 as i32 as i16,
    15158 as i32 as i16,
    9756 as i32 as i16,
    -(3334 as i32) as i16,
    -(9756 as i32) as i16,
    0 as i32 as i16,
    4920 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8208 as i32 as i16,
    14369 as i32 as i16,
    -(881 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(4163 as i32) as i16,
    -(8192 as i32) as i16,
    0 as i32 as i16,
    -(13218 as i32) as i16,
    -(14833 as i32) as i16,
    0 as i32 as i16,
    -(8836 as i32) as i16,
    11095 as i32 as i16,
    0 as i32 as i16,
    6218 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8208 as i32 as i16,
    10161 as i32 as i16,
    881 as i32 as i16,
    10161 as i32 as i16,
    13218 as i32 as i16,
    2944 as i32 as i16,
    -(8192 as i32) as i16,
    2944 as i32 as i16,
    0 as i32 as i16,
    10488 as i32 as i16,
    6218 as i32 as i16,
    -(6248 as i32) as i16,
    -(11095 as i32) as i16,
    -(6248 as i32) as i16,
    0 as i32 as i16,
    -(10488 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8208 as i32 as i16,
    -(14369 as i32) as i16,
    -(881 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    4163 as i32 as i16,
    -(8192 as i32) as i16,
    0 as i32 as i16,
    -(13218 as i32) as i16,
    14833 as i32 as i16,
    0 as i32 as i16,
    8836 as i32 as i16,
    11095 as i32 as i16,
    0 as i32 as i16,
    6218 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8208 as i32 as i16,
    0 as i32 as i16,
    -(881 as i32) as i16,
    -(14369 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(8192 as i32) as i16,
    4163 as i32 as i16,
    13218 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    11095 as i32 as i16,
    8836 as i32 as i16,
    -(6218 as i32) as i16,
    -(14833 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8176 as i32 as i16,
    -(5566 as i32) as i16,
    -(11552 as i32) as i16,
    5566 as i32 as i16,
    -(9681 as i32) as i16,
    11205 as i32 as i16,
    8192 as i32 as i16,
    -(11205 as i32) as i16,
    0 as i32 as i16,
    -(4920 as i32) as i16,
    15158 as i32 as i16,
    -(9756 as i32) as i16,
    -(3334 as i32) as i16,
    9756 as i32 as i16,
    0 as i32 as i16,
    -(4920 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8176 as i32 as i16,
    0 as i32 as i16,
    11552 as i32 as i16,
    -(7871 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8192 as i32 as i16,
    -(15846 as i32) as i16,
    9681 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    3334 as i32 as i16,
    -(13797 as i32) as i16,
    15158 as i32 as i16,
    -(6958 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8176 as i32 as i16,
    -(7871 as i32) as i16,
    11552 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(15846 as i32) as i16,
    8192 as i32 as i16,
    0 as i32 as i16,
    -(9681 as i32) as i16,
    6958 as i32 as i16,
    0 as i32 as i16,
    -(13797 as i32) as i16,
    3334 as i32 as i16,
    0 as i32 as i16,
    -(15158 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8176 as i32 as i16,
    -(5566 as i32) as i16,
    -(11552 as i32) as i16,
    -(5566 as i32) as i16,
    9681 as i32 as i16,
    11205 as i32 as i16,
    8192 as i32 as i16,
    11205 as i32 as i16,
    0 as i32 as i16,
    -(4920 as i32) as i16,
    -(15158 as i32) as i16,
    -(9756 as i32) as i16,
    -(3334 as i32) as i16,
    -(9756 as i32) as i16,
    0 as i32 as i16,
    4920 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8208 as i32 as i16,
    -(10161 as i32) as i16,
    881 as i32 as i16,
    -(10161 as i32) as i16,
    13218 as i32 as i16,
    -(2944 as i32) as i16,
    -(8192 as i32) as i16,
    -(2944 as i32) as i16,
    0 as i32 as i16,
    -(10488 as i32) as i16,
    6218 as i32 as i16,
    6248 as i32 as i16,
    -(11095 as i32) as i16,
    6248 as i32 as i16,
    0 as i32 as i16,
    10488 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    32767 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    32767 as i32 as i16,
];
pub static mut mapping_matrix_foa_demixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 6 as i32,
        cols: 6 as i32,
        gain: 0 as i32,
    };
    init
};
pub static mut mapping_matrix_foa_demixing_data: [i16; 36] = [
    16384 as i32 as i16,
    16384 as i32 as i16,
    16384 as i32 as i16,
    16384 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    23170 as i32 as i16,
    0 as i32 as i16,
    -(23170 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(16384 as i32) as i16,
    16384 as i32 as i16,
    -(16384 as i32) as i16,
    16384 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    23170 as i32 as i16,
    0 as i32 as i16,
    -(23170 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    32767 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    32767 as i32 as i16,
];
pub static mut mapping_matrix_soa_demixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 11 as i32,
        cols: 11 as i32,
        gain: 3050 as i32,
    };
    init
};
pub static mut mapping_matrix_soa_demixing_data: [i16; 121] = [
    2771 as i32 as i16,
    2771 as i32 as i16,
    2771 as i32 as i16,
    2771 as i32 as i16,
    2771 as i32 as i16,
    2771 as i32 as i16,
    2771 as i32 as i16,
    2771 as i32 as i16,
    2771 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    10033 as i32 as i16,
    10033 as i32 as i16,
    -(20066 as i32) as i16,
    10033 as i32 as i16,
    14189 as i32 as i16,
    14189 as i32 as i16,
    -(28378 as i32) as i16,
    10033 as i32 as i16,
    -(20066 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    3393 as i32 as i16,
    3393 as i32 as i16,
    3393 as i32 as i16,
    -(3393 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(3393 as i32) as i16,
    -(3393 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(17378 as i32) as i16,
    17378 as i32 as i16,
    0 as i32 as i16,
    -(17378 as i32) as i16,
    -(24576 as i32) as i16,
    24576 as i32 as i16,
    0 as i32 as i16,
    17378 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(14189 as i32) as i16,
    14189 as i32 as i16,
    0 as i32 as i16,
    -(14189 as i32) as i16,
    -(28378 as i32) as i16,
    28378 as i32 as i16,
    0 as i32 as i16,
    14189 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    2399 as i32 as i16,
    2399 as i32 as i16,
    -(4799 as i32) as i16,
    -(2399 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(2399 as i32) as i16,
    4799 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    1959 as i32 as i16,
    1959 as i32 as i16,
    1959 as i32 as i16,
    1959 as i32 as i16,
    -(3918 as i32) as i16,
    -(3918 as i32) as i16,
    -(3918 as i32) as i16,
    1959 as i32 as i16,
    1959 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(4156 as i32) as i16,
    4156 as i32 as i16,
    0 as i32 as i16,
    4156 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(4156 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    -(16384 as i32) as i16,
    8192 as i32 as i16,
    16384 as i32 as i16,
    16384 as i32 as i16,
    -(32768 as i32) as i16,
    8192 as i32 as i16,
    -(16384 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8312 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    8312 as i32 as i16,
];
pub static mut mapping_matrix_toa_demixing: MappingMatrix = {
    let init = MappingMatrix {
        rows: 18 as i32,
        cols: 18 as i32,
        gain: 0 as i32,
    };
    init
};
pub static mut mapping_matrix_toa_demixing_data: [i16; 324] = [
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    8192 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(9779 as i32) as i16,
    9779 as i32 as i16,
    6263 as i32 as i16,
    8857 as i32 as i16,
    0 as i32 as i16,
    6263 as i32 as i16,
    13829 as i32 as i16,
    9779 as i32 as i16,
    -(13829 as i32) as i16,
    0 as i32 as i16,
    -(6263 as i32) as i16,
    0 as i32 as i16,
    -(8857 as i32) as i16,
    -(6263 as i32) as i16,
    -(9779 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(3413 as i32) as i16,
    3413 as i32 as i16,
    3413 as i32 as i16,
    -(11359 as i32) as i16,
    11359 as i32 as i16,
    11359 as i32 as i16,
    -(11359 as i32) as i16,
    -(3413 as i32) as i16,
    3413 as i32 as i16,
    -(3413 as i32) as i16,
    -(3413 as i32) as i16,
    -(11359 as i32) as i16,
    11359 as i32 as i16,
    11359 as i32 as i16,
    -(11359 as i32) as i16,
    3413 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    13829 as i32 as i16,
    9779 as i32 as i16,
    -(9779 as i32) as i16,
    6263 as i32 as i16,
    0 as i32 as i16,
    8857 as i32 as i16,
    -(6263 as i32) as i16,
    0 as i32 as i16,
    9779 as i32 as i16,
    0 as i32 as i16,
    -(13829 as i32) as i16,
    6263 as i32 as i16,
    -(8857 as i32) as i16,
    0 as i32 as i16,
    -(6263 as i32) as i16,
    -(9779 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(15617 as i32) as i16,
    -(15617 as i32) as i16,
    6406 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(6406 as i32) as i16,
    0 as i32 as i16,
    15617 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(6406 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    6406 as i32 as i16,
    15617 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(5003 as i32) as i16,
    5003 as i32 as i16,
    -(10664 as i32) as i16,
    15081 as i32 as i16,
    0 as i32 as i16,
    -(10664 as i32) as i16,
    -(7075 as i32) as i16,
    5003 as i32 as i16,
    7075 as i32 as i16,
    0 as i32 as i16,
    10664 as i32 as i16,
    0 as i32 as i16,
    -(15081 as i32) as i16,
    10664 as i32 as i16,
    -(5003 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(8176 as i32) as i16,
    -(8176 as i32) as i16,
    -(8176 as i32) as i16,
    8208 as i32 as i16,
    8208 as i32 as i16,
    8208 as i32 as i16,
    8208 as i32 as i16,
    -(8176 as i32) as i16,
    -(8176 as i32) as i16,
    -(8176 as i32) as i16,
    -(8176 as i32) as i16,
    8208 as i32 as i16,
    8208 as i32 as i16,
    8208 as i32 as i16,
    8208 as i32 as i16,
    -(8176 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(7075 as i32) as i16,
    5003 as i32 as i16,
    -(5003 as i32) as i16,
    -(10664 as i32) as i16,
    0 as i32 as i16,
    15081 as i32 as i16,
    10664 as i32 as i16,
    0 as i32 as i16,
    5003 as i32 as i16,
    0 as i32 as i16,
    7075 as i32 as i16,
    -(10664 as i32) as i16,
    -(15081 as i32) as i16,
    0 as i32 as i16,
    10664 as i32 as i16,
    -(5003 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    15617 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(6406 as i32) as i16,
    6406 as i32 as i16,
    0 as i32 as i16,
    -(15617 as i32) as i16,
    0 as i32 as i16,
    -(15617 as i32) as i16,
    15617 as i32 as i16,
    0 as i32 as i16,
    6406 as i32 as i16,
    -(6406 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(11393 as i32) as i16,
    11393 as i32 as i16,
    2993 as i32 as i16,
    -(4233 as i32) as i16,
    0 as i32 as i16,
    2993 as i32 as i16,
    -(16112 as i32) as i16,
    11393 as i32 as i16,
    16112 as i32 as i16,
    0 as i32 as i16,
    -(2993 as i32) as i16,
    0 as i32 as i16,
    4233 as i32 as i16,
    -(2993 as i32) as i16,
    -(11393 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(9974 as i32) as i16,
    -(9974 as i32) as i16,
    -(13617 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    13617 as i32 as i16,
    0 as i32 as i16,
    9974 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    13617 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(13617 as i32) as i16,
    9974 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    5579 as i32 as i16,
    -(5579 as i32) as i16,
    10185 as i32 as i16,
    14403 as i32 as i16,
    0 as i32 as i16,
    10185 as i32 as i16,
    -(7890 as i32) as i16,
    -(5579 as i32) as i16,
    7890 as i32 as i16,
    0 as i32 as i16,
    -(10185 as i32) as i16,
    0 as i32 as i16,
    -(14403 as i32) as i16,
    -(10185 as i32) as i16,
    5579 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    11826 as i32 as i16,
    -(11826 as i32) as i16,
    -(11826 as i32) as i16,
    -(901 as i32) as i16,
    901 as i32 as i16,
    901 as i32 as i16,
    -(901 as i32) as i16,
    11826 as i32 as i16,
    -(11826 as i32) as i16,
    11826 as i32 as i16,
    11826 as i32 as i16,
    -(901 as i32) as i16,
    901 as i32 as i16,
    901 as i32 as i16,
    -(901 as i32) as i16,
    -(11826 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(7890 as i32) as i16,
    -(5579 as i32) as i16,
    5579 as i32 as i16,
    10185 as i32 as i16,
    0 as i32 as i16,
    14403 as i32 as i16,
    -(10185 as i32) as i16,
    0 as i32 as i16,
    -(5579 as i32) as i16,
    0 as i32 as i16,
    7890 as i32 as i16,
    10185 as i32 as i16,
    -(14403 as i32) as i16,
    0 as i32 as i16,
    -(10185 as i32) as i16,
    5579 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(9974 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    -(13617 as i32) as i16,
    13617 as i32 as i16,
    0 as i32 as i16,
    9974 as i32 as i16,
    0 as i32 as i16,
    9974 as i32 as i16,
    -(9974 as i32) as i16,
    0 as i32 as i16,
    13617 as i32 as i16,
    -(13617 as i32) as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    16112 as i32 as i16,
    -(11393 as i32) as i16,
    11393 as i32 as i16,
    -(2993 as i32) as i16,
    0 as i32 as i16,
    4233 as i32 as i16,
    2993 as i32 as i16,
    0 as i32 as i16,
    -(11393 as i32) as i16,
    0 as i32 as i16,
    -(16112 as i32) as i16,
    -(2993 as i32) as i16,
    -(4233 as i32) as i16,
    0 as i32 as i16,
    2993 as i32 as i16,
    11393 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    32767 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    0 as i32 as i16,
    32767 as i32 as i16,
];
