#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/mapping_matrix.h:42"]
pub mod mapping_matrix_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:16"]
    pub struct MappingMatrix {
        pub rows: i32,
        pub cols: i32,
        pub gain: i32,
    }

    use crate::opus_val16;
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn mapping_matrix_get_size(rows: i32, cols: i32) -> i32;
        #[c2rust::src_loc = "55:1"]
        pub fn mapping_matrix_init(
            matrix: *mut MappingMatrix,
            rows: i32,
            cols: i32,
            gain: i32,
            data: *const i16,
            data_size: i32,
        );
        #[c2rust::src_loc = "65:1"]
        pub fn mapping_matrix_multiply_channel_in_float(
            matrix: *const MappingMatrix,
            input: *const f32,
            input_rows: i32,
            output: *mut opus_val16,
            output_row: i32,
            output_rows: i32,
            frame_size: i32,
        );
        #[c2rust::src_loc = "75:1"]
        pub fn mapping_matrix_multiply_channel_out_float(
            matrix: *const MappingMatrix,
            input: *const opus_val16,
            input_row: i32,
            input_rows: i32,
            output: *mut f32,
            output_rows: i32,
            frame_size: i32,
        );
        #[c2rust::src_loc = "86:1"]
        pub fn mapping_matrix_multiply_channel_in_short(
            matrix: *const MappingMatrix,
            input: *const i16,
            input_rows: i32,
            output: *mut opus_val16,
            output_row: i32,
            output_rows: i32,
            frame_size: i32,
        );
        #[c2rust::src_loc = "96:1"]
        pub fn mapping_matrix_multiply_channel_out_short(
            matrix: *const MappingMatrix,
            input: *const opus_val16,
            input_row: i32,
            input_rows: i32,
            output: *mut i16,
            output_rows: i32,
            frame_size: i32,
        );
    }
}

pub use self::mapping_matrix_h::{
    mapping_matrix_get_size, mapping_matrix_init, mapping_matrix_multiply_channel_in_float,
    mapping_matrix_multiply_channel_in_short, mapping_matrix_multiply_channel_out_float,
    mapping_matrix_multiply_channel_out_short, MappingMatrix,
};
use crate::{_test_failed, assert_is_equal, assert_is_equal_short, opus_val16};
use unsafe_libopus::externs::{free, malloc};

#[c2rust::src_loc = "82:1"]
pub unsafe fn test_simple_matrix() {
    let simple_matrix_params: MappingMatrix = {
        MappingMatrix {
            rows: 4 as i32,
            cols: 3 as i32,
            gain: 0 as i32,
        }
    };
    let simple_matrix_data: [i16; 12] = [
        0 as i32 as i16,
        32767 as i32 as i16,
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
    let input_int16: [i16; 30] = [
        32767 as i32 as i16,
        0 as i32 as i16,
        -(32768 as i32) as i16,
        29491 as i32 as i16,
        -(3277 as i32) as i16,
        -(29491 as i32) as i16,
        26214 as i32 as i16,
        -(6554 as i32) as i16,
        -(26214 as i32) as i16,
        22938 as i32 as i16,
        -(9830 as i32) as i16,
        -(22938 as i32) as i16,
        19661 as i32 as i16,
        -(13107 as i32) as i16,
        -(19661 as i32) as i16,
        16384 as i32 as i16,
        -(16384 as i32) as i16,
        -(16384 as i32) as i16,
        13107 as i32 as i16,
        -(19661 as i32) as i16,
        -(13107 as i32) as i16,
        9830 as i32 as i16,
        -(22938 as i32) as i16,
        -(9830 as i32) as i16,
        6554 as i32 as i16,
        -(26214 as i32) as i16,
        -(6554 as i32) as i16,
        3277 as i32 as i16,
        -(29491 as i32) as i16,
        -(3277 as i32) as i16,
    ];
    let expected_output_int16: [i16; 40] = [
        0 as i32 as i16,
        32767 as i32 as i16,
        0 as i32 as i16,
        -(32768 as i32) as i16,
        -(3277 as i32) as i16,
        29491 as i32 as i16,
        0 as i32 as i16,
        -(29491 as i32) as i16,
        -(6554 as i32) as i16,
        26214 as i32 as i16,
        0 as i32 as i16,
        -(26214 as i32) as i16,
        -(9830 as i32) as i16,
        22938 as i32 as i16,
        0 as i32 as i16,
        -(22938 as i32) as i16,
        -(13107 as i32) as i16,
        19661 as i32 as i16,
        0 as i32 as i16,
        -(19661 as i32) as i16,
        -(16384 as i32) as i16,
        16384 as i32 as i16,
        0 as i32 as i16,
        -(16384 as i32) as i16,
        -(19661 as i32) as i16,
        13107 as i32 as i16,
        0 as i32 as i16,
        -(13107 as i32) as i16,
        -(22938 as i32) as i16,
        9830 as i32 as i16,
        0 as i32 as i16,
        -(9830 as i32) as i16,
        -(26214 as i32) as i16,
        6554 as i32 as i16,
        0 as i32 as i16,
        -(6554 as i32) as i16,
        -(29491 as i32) as i16,
        3277 as i32 as i16,
        0 as i32 as i16,
        -(3277 as i32) as i16,
    ];
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    let mut simple_matrix_size: i32 = 0;
    let mut input_val16: *mut opus_val16 = std::ptr::null_mut::<opus_val16>();
    let mut output_val16: *mut opus_val16 = std::ptr::null_mut::<opus_val16>();
    let mut output_int16: *mut i16 = std::ptr::null_mut::<i16>();
    let mut simple_matrix: *mut MappingMatrix = std::ptr::null_mut::<MappingMatrix>();
    input_val16 =
        malloc((::core::mem::size_of::<opus_val16>() as u64).wrapping_mul(30 as i32 as u64))
            as *mut opus_val16;
    output_int16 =
        malloc((::core::mem::size_of::<i16>() as u64).wrapping_mul(40 as i32 as u64)) as *mut i16;
    output_val16 =
        malloc((::core::mem::size_of::<opus_val16>() as u64).wrapping_mul(40 as i32 as u64))
            as *mut opus_val16;
    simple_matrix_size =
        mapping_matrix_get_size(simple_matrix_params.rows, simple_matrix_params.cols);
    if simple_matrix_size == 0 {
        _test_failed(
            b"tests/test_opus_projection.c\0" as *const u8 as *const i8,
            112 as i32,
        );
    }
    simple_matrix = malloc(simple_matrix_size as _) as *mut MappingMatrix;
    mapping_matrix_init(
        simple_matrix,
        simple_matrix_params.rows,
        simple_matrix_params.cols,
        simple_matrix_params.gain,
        simple_matrix_data.as_ptr(),
        ::core::mem::size_of::<[i16; 12]>() as u64 as i32,
    );
    i = 0 as i32;
    while i < 30 as i32 {
        *input_val16.offset(i as isize) =
            1 as i32 as f32 / 32768.0f32 * input_int16[i as usize] as i32 as f32;
        i += 1;
    }
    i = 0 as i32;
    while i < 40 as i32 {
        *output_val16.offset(i as isize) = 0 as i32 as opus_val16;
        i += 1;
    }
    i = 0 as i32;
    while i < (*simple_matrix).rows {
        mapping_matrix_multiply_channel_in_short(
            simple_matrix,
            input_int16.as_ptr(),
            (*simple_matrix).cols,
            &mut *output_val16.offset(i as isize),
            i,
            (*simple_matrix).rows,
            10 as i32,
        );
        i += 1;
    }
    ret = assert_is_equal(
        output_val16,
        expected_output_int16.as_ptr(),
        40 as i32,
        1 as i32 as i16,
    );
    if ret != 0 {
        _test_failed(
            b"tests/test_opus_projection.c\0" as *const u8 as *const i8,
            140 as i32,
        );
    }
    i = 0 as i32;
    while i < 40 as i32 {
        *output_int16.offset(i as isize) = 0 as i32 as i16;
        i += 1;
    }
    i = 0 as i32;
    while i < (*simple_matrix).cols {
        mapping_matrix_multiply_channel_out_short(
            simple_matrix,
            &mut *input_val16.offset(i as isize),
            i,
            (*simple_matrix).cols,
            output_int16,
            (*simple_matrix).rows,
            10 as i32,
        );
        i += 1;
    }
    ret = assert_is_equal_short(
        output_int16,
        expected_output_int16.as_ptr(),
        40 as i32,
        1 as i32 as i16,
    );
    if ret != 0 {
        _test_failed(
            b"tests/test_opus_projection.c\0" as *const u8 as *const i8,
            153 as i32,
        );
    }
    i = 0 as i32;
    while i < 40 as i32 {
        *output_val16.offset(i as isize) = 0 as i32 as opus_val16;
        i += 1;
    }
    i = 0 as i32;
    while i < (*simple_matrix).rows {
        mapping_matrix_multiply_channel_in_float(
            simple_matrix,
            input_val16,
            (*simple_matrix).cols,
            &mut *output_val16.offset(i as isize),
            i,
            (*simple_matrix).rows,
            10 as i32,
        );
        i += 1;
    }
    ret = assert_is_equal(
        output_val16,
        expected_output_int16.as_ptr(),
        40 as i32,
        1 as i32 as i16,
    );
    if ret != 0 {
        _test_failed(
            b"tests/test_opus_projection.c\0" as *const u8 as *const i8,
            167 as i32,
        );
    }
    i = 0 as i32;
    while i < 40 as i32 {
        *output_val16.offset(i as isize) = 0 as i32 as opus_val16;
        i += 1;
    }
    i = 0 as i32;
    while i < (*simple_matrix).cols {
        mapping_matrix_multiply_channel_out_float(
            simple_matrix,
            &mut *input_val16.offset(i as isize),
            i,
            (*simple_matrix).cols,
            output_val16,
            (*simple_matrix).rows,
            10 as i32,
        );
        i += 1;
    }
    ret = assert_is_equal(
        output_val16,
        expected_output_int16.as_ptr(),
        40 as i32,
        1 as i32 as i16,
    );
    if ret != 0 {
        _test_failed(
            b"tests/test_opus_projection.c\0" as *const u8 as *const i8,
            180 as i32,
        );
    }
    free(input_val16 as *mut core::ffi::c_void);
    free(output_int16 as *mut core::ffi::c_void);
    free(output_val16 as *mut core::ffi::c_void);
    free(simple_matrix as *mut core::ffi::c_void);
}
