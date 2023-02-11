#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/src/mapping_matrix.h:42"]
pub mod mapping_matrix_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:16"]
    pub struct MappingMatrix {
        pub rows: libc::c_int,
        pub cols: libc::c_int,
        pub gain: libc::c_int,
    }

    use crate::opus_val16;
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn mapping_matrix_get_size(rows: libc::c_int, cols: libc::c_int) -> i32;
        #[c2rust::src_loc = "55:1"]
        pub fn mapping_matrix_init(
            matrix: *mut MappingMatrix,
            rows: libc::c_int,
            cols: libc::c_int,
            gain: libc::c_int,
            data: *const i16,
            data_size: i32,
        );
        #[c2rust::src_loc = "65:1"]
        pub fn mapping_matrix_multiply_channel_in_float(
            matrix: *const MappingMatrix,
            input: *const libc::c_float,
            input_rows: libc::c_int,
            output: *mut opus_val16,
            output_row: libc::c_int,
            output_rows: libc::c_int,
            frame_size: libc::c_int,
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
        #[c2rust::src_loc = "86:1"]
        pub fn mapping_matrix_multiply_channel_in_short(
            matrix: *const MappingMatrix,
            input: *const i16,
            input_rows: libc::c_int,
            output: *mut opus_val16,
            output_row: libc::c_int,
            output_rows: libc::c_int,
            frame_size: libc::c_int,
        );
        #[c2rust::src_loc = "96:1"]
        pub fn mapping_matrix_multiply_channel_out_short(
            matrix: *const MappingMatrix,
            input: *const opus_val16,
            input_row: libc::c_int,
            input_rows: libc::c_int,
            output: *mut i16,
            output_rows: libc::c_int,
            frame_size: libc::c_int,
        );
    }
}

pub use self::mapping_matrix_h::{
    mapping_matrix_get_size, mapping_matrix_init, mapping_matrix_multiply_channel_in_float,
    mapping_matrix_multiply_channel_in_short, mapping_matrix_multiply_channel_out_float,
    mapping_matrix_multiply_channel_out_short, MappingMatrix,
};
use crate::{_test_failed, assert_is_equal, assert_is_equal_short, opus_val16};
use libopus_unsafe::externs::{free, malloc};

#[no_mangle]
#[c2rust::src_loc = "82:1"]
pub unsafe extern "C" fn test_simple_matrix() {
    let simple_matrix_params: MappingMatrix = {
        MappingMatrix {
            rows: 4 as libc::c_int,
            cols: 3 as libc::c_int,
            gain: 0 as libc::c_int,
        }
    };
    let simple_matrix_data: [i16; 12] = [
        0 as libc::c_int as i16,
        32767 as libc::c_int as i16,
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
    let input_int16: [i16; 30] = [
        32767 as libc::c_int as i16,
        0 as libc::c_int as i16,
        -(32768 as libc::c_int) as i16,
        29491 as libc::c_int as i16,
        -(3277 as libc::c_int) as i16,
        -(29491 as libc::c_int) as i16,
        26214 as libc::c_int as i16,
        -(6554 as libc::c_int) as i16,
        -(26214 as libc::c_int) as i16,
        22938 as libc::c_int as i16,
        -(9830 as libc::c_int) as i16,
        -(22938 as libc::c_int) as i16,
        19661 as libc::c_int as i16,
        -(13107 as libc::c_int) as i16,
        -(19661 as libc::c_int) as i16,
        16384 as libc::c_int as i16,
        -(16384 as libc::c_int) as i16,
        -(16384 as libc::c_int) as i16,
        13107 as libc::c_int as i16,
        -(19661 as libc::c_int) as i16,
        -(13107 as libc::c_int) as i16,
        9830 as libc::c_int as i16,
        -(22938 as libc::c_int) as i16,
        -(9830 as libc::c_int) as i16,
        6554 as libc::c_int as i16,
        -(26214 as libc::c_int) as i16,
        -(6554 as libc::c_int) as i16,
        3277 as libc::c_int as i16,
        -(29491 as libc::c_int) as i16,
        -(3277 as libc::c_int) as i16,
    ];
    let expected_output_int16: [i16; 40] = [
        0 as libc::c_int as i16,
        32767 as libc::c_int as i16,
        0 as libc::c_int as i16,
        -(32768 as libc::c_int) as i16,
        -(3277 as libc::c_int) as i16,
        29491 as libc::c_int as i16,
        0 as libc::c_int as i16,
        -(29491 as libc::c_int) as i16,
        -(6554 as libc::c_int) as i16,
        26214 as libc::c_int as i16,
        0 as libc::c_int as i16,
        -(26214 as libc::c_int) as i16,
        -(9830 as libc::c_int) as i16,
        22938 as libc::c_int as i16,
        0 as libc::c_int as i16,
        -(22938 as libc::c_int) as i16,
        -(13107 as libc::c_int) as i16,
        19661 as libc::c_int as i16,
        0 as libc::c_int as i16,
        -(19661 as libc::c_int) as i16,
        -(16384 as libc::c_int) as i16,
        16384 as libc::c_int as i16,
        0 as libc::c_int as i16,
        -(16384 as libc::c_int) as i16,
        -(19661 as libc::c_int) as i16,
        13107 as libc::c_int as i16,
        0 as libc::c_int as i16,
        -(13107 as libc::c_int) as i16,
        -(22938 as libc::c_int) as i16,
        9830 as libc::c_int as i16,
        0 as libc::c_int as i16,
        -(9830 as libc::c_int) as i16,
        -(26214 as libc::c_int) as i16,
        6554 as libc::c_int as i16,
        0 as libc::c_int as i16,
        -(6554 as libc::c_int) as i16,
        -(29491 as libc::c_int) as i16,
        3277 as libc::c_int as i16,
        0 as libc::c_int as i16,
        -(3277 as libc::c_int) as i16,
    ];
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut simple_matrix_size: i32 = 0;
    let mut input_val16: *mut opus_val16 = std::ptr::null_mut::<opus_val16>();
    let mut output_val16: *mut opus_val16 = std::ptr::null_mut::<opus_val16>();
    let mut output_int16: *mut i16 = std::ptr::null_mut::<i16>();
    let mut simple_matrix: *mut MappingMatrix = std::ptr::null_mut::<MappingMatrix>();
    input_val16 = malloc(
        (::core::mem::size_of::<opus_val16>() as libc::c_ulong)
            .wrapping_mul(30 as libc::c_int as libc::c_ulong),
    ) as *mut opus_val16;
    output_int16 = malloc(
        (::core::mem::size_of::<i16>() as libc::c_ulong)
            .wrapping_mul(40 as libc::c_int as libc::c_ulong),
    ) as *mut i16;
    output_val16 = malloc(
        (::core::mem::size_of::<opus_val16>() as libc::c_ulong)
            .wrapping_mul(40 as libc::c_int as libc::c_ulong),
    ) as *mut opus_val16;
    simple_matrix_size =
        mapping_matrix_get_size(simple_matrix_params.rows, simple_matrix_params.cols);
    if simple_matrix_size == 0 {
        _test_failed(
            b"tests/test_opus_projection.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
        );
    }
    simple_matrix = malloc(simple_matrix_size as _) as *mut MappingMatrix;
    mapping_matrix_init(
        simple_matrix,
        simple_matrix_params.rows,
        simple_matrix_params.cols,
        simple_matrix_params.gain,
        simple_matrix_data.as_ptr(),
        ::core::mem::size_of::<[i16; 12]>() as libc::c_ulong as i32,
    );
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        *input_val16.offset(i as isize) = 1 as libc::c_int as libc::c_float / 32768.0f32
            * input_int16[i as usize] as libc::c_int as libc::c_float;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 40 as libc::c_int {
        *output_val16.offset(i as isize) = 0 as libc::c_int as opus_val16;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*simple_matrix).rows {
        mapping_matrix_multiply_channel_in_short(
            simple_matrix,
            input_int16.as_ptr(),
            (*simple_matrix).cols,
            &mut *output_val16.offset(i as isize),
            i,
            (*simple_matrix).rows,
            10 as libc::c_int,
        );
        i += 1;
    }
    ret = assert_is_equal(
        output_val16,
        expected_output_int16.as_ptr(),
        40 as libc::c_int,
        1 as libc::c_int as i16,
    );
    if ret != 0 {
        _test_failed(
            b"tests/test_opus_projection.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < 40 as libc::c_int {
        *output_int16.offset(i as isize) = 0 as libc::c_int as i16;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*simple_matrix).cols {
        mapping_matrix_multiply_channel_out_short(
            simple_matrix,
            &mut *input_val16.offset(i as isize),
            i,
            (*simple_matrix).cols,
            output_int16,
            (*simple_matrix).rows,
            10 as libc::c_int,
        );
        i += 1;
    }
    ret = assert_is_equal_short(
        output_int16,
        expected_output_int16.as_ptr(),
        40 as libc::c_int,
        1 as libc::c_int as i16,
    );
    if ret != 0 {
        _test_failed(
            b"tests/test_opus_projection.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < 40 as libc::c_int {
        *output_val16.offset(i as isize) = 0 as libc::c_int as opus_val16;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*simple_matrix).rows {
        mapping_matrix_multiply_channel_in_float(
            simple_matrix,
            input_val16,
            (*simple_matrix).cols,
            &mut *output_val16.offset(i as isize),
            i,
            (*simple_matrix).rows,
            10 as libc::c_int,
        );
        i += 1;
    }
    ret = assert_is_equal(
        output_val16,
        expected_output_int16.as_ptr(),
        40 as libc::c_int,
        1 as libc::c_int as i16,
    );
    if ret != 0 {
        _test_failed(
            b"tests/test_opus_projection.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < 40 as libc::c_int {
        *output_val16.offset(i as isize) = 0 as libc::c_int as opus_val16;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*simple_matrix).cols {
        mapping_matrix_multiply_channel_out_float(
            simple_matrix,
            &mut *input_val16.offset(i as isize),
            i,
            (*simple_matrix).cols,
            output_val16,
            (*simple_matrix).rows,
            10 as libc::c_int,
        );
        i += 1;
    }
    ret = assert_is_equal(
        output_val16,
        expected_output_int16.as_ptr(),
        40 as libc::c_int,
        1 as libc::c_int as i16,
    );
    if ret != 0 {
        _test_failed(
            b"tests/test_opus_projection.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
        );
    }
    free(input_val16 as *mut libc::c_void);
    free(output_int16 as *mut libc::c_void);
    free(output_val16 as *mut libc::c_void);
    free(simple_matrix as *mut libc::c_void);
}
