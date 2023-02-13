#[c2rust::src_loc = "34:18"]
pub static silk_LTP_per_index_iCDF: [u8; 3] = [179, 99, 0];
#[c2rust::src_loc = "38:25"]
static silk_LTP_gain_iCDF_0: [u8; 8] = [71, 56, 43, 30, 21, 12, 6, 0];
#[c2rust::src_loc = "42:25"]
static silk_LTP_gain_iCDF_1: [u8; 16] = [
    199, 165, 144, 124, 109, 96, 84, 71, 61, 51, 42, 32, 23, 15, 8, 0,
];
#[c2rust::src_loc = "47:25"]
static silk_LTP_gain_iCDF_2: [u8; 32] = [
    241, 225, 211, 199, 187, 175, 164, 153, 142, 132, 123, 114, 105, 96, 88, 80, 72, 64, 57, 50,
    44, 38, 33, 29, 24, 20, 16, 12, 9, 5, 2, 0,
];
#[c2rust::src_loc = "54:25"]
static silk_LTP_gain_BITS_Q5_0: [u8; 8] = [15, 131, 138, 138, 155, 155, 173, 173];
#[c2rust::src_loc = "58:25"]
static silk_LTP_gain_BITS_Q5_1: [u8; 16] = [
    69, 93, 115, 118, 131, 138, 141, 138, 150, 150, 155, 150, 155, 160, 166, 160,
];
#[c2rust::src_loc = "63:25"]
static silk_LTP_gain_BITS_Q5_2: [u8; 32] = [
    131, 128, 134, 141, 141, 141, 145, 145, 145, 150, 155, 155, 155, 155, 160, 160, 160, 160, 166,
    166, 173, 173, 182, 192, 182, 192, 192, 192, 205, 192, 205, 224,
];
#[c2rust::src_loc = "70:26"]
pub static mut silk_LTP_gain_iCDF_ptrs: [*const u8; 3] = [
    silk_LTP_gain_iCDF_0.as_ptr(),
    silk_LTP_gain_iCDF_1.as_ptr(),
    silk_LTP_gain_iCDF_2.as_ptr(),
];
#[c2rust::src_loc = "76:26"]
pub static mut silk_LTP_gain_BITS_Q5_ptrs: [*const u8; 3] = [
    silk_LTP_gain_BITS_Q5_0.as_ptr(),
    silk_LTP_gain_BITS_Q5_1.as_ptr(),
    silk_LTP_gain_BITS_Q5_2.as_ptr(),
];
#[c2rust::src_loc = "82:24"]
static silk_LTP_gain_vq_0: [[i8; 5]; 8] = [
    [4, 6, 24, 7, 5],
    [0, 0, 2, 0, 0],
    [12, 28, 41, 13, -4],
    [-9, 15, 42, 25, 14],
    [1, -2, 62, 41, -9],
    [-10, 37, 65, -4, 3],
    [-6, 4, 66, 7, -8],
    [16, 14, 38, -3, 33],
];
#[c2rust::src_loc = "110:24"]
static silk_LTP_gain_vq_1: [[i8; 5]; 16] = [
    [13, 22, 39, 23, 12],
    [-1, 36, 64, 27, -6],
    [-7, 10, 55, 43, 17],
    [1, 1, 8, 1, 1],
    [6, -11, 74, 53, -9],
    [-12, 55, 76, -12, 8],
    [-3, 3, 93, 27, -4],
    [26, 39, 59, 3, -8],
    [2, 0, 77, 11, 9],
    [-8, 22, 44, -6, 7],
    [40, 9, 26, 3, 9],
    [-7, 20, 101, -7, 4],
    [3, -8, 42, 26, 0],
    [-15, 33, 68, 2, 23],
    [-2, 55, 46, -2, 15],
    [3, -1, 21, 16, 41],
];
#[c2rust::src_loc = "162:24"]
static silk_LTP_gain_vq_2: [[i8; 5]; 32] = [
    [-6, 27, 61, 39, 5],
    [-11, 42, 88, 4, 1],
    [-2, 60, 65, 6, -4],
    [-1, -5, 73, 56, 1],
    [-9, 19, 94, 29, -9],
    [0, 12, 99, 6, 4],
    [8, -19, 102, 46, -13],
    [3, 2, 13, 3, 2],
    [9, -21, 84, 72, -18],
    [-11, 46, 104, -22, 8],
    [18, 38, 48, 23, 0],
    [-16, 70, 83, -21, 11],
    [5, -11, 117, 22, -8],
    [-6, 23, 117, -12, 3],
    [3, -8, 95, 28, 4],
    [-10, 15, 77, 60, -15],
    [-1, 4, 124, 2, -4],
    [3, 38, 84, 24, -25],
    [2, 13, 42, 13, 31],
    [21, -4, 56, 46, -1],
    [-1, 35, 79, -13, 19],
    [-7, 65, 88, -9, -14],
    [20, 4, 81, 49, -29],
    [20, 0, 75, 3, -17],
    [5, -9, 44, 92, -8],
    [1, -3, 22, 69, 31],
    [-6, 95, 41, -12, 5],
    [39, 67, 16, -4, 1],
    [0, -6, 120, 55, -36],
    [-13, 44, 122, 4, -24],
    [81, 5, 11, 3, 7],
    [2, 0, 9, 10, 88],
];
#[c2rust::src_loc = "262:25"]
pub static mut silk_LTP_vq_ptrs_Q7: [*const i8; 3] = [0 as *const i8; 3];
#[c2rust::src_loc = "270:25"]
static silk_LTP_gain_vq_0_gain: [u8; 8] = [46, 2, 90, 87, 93, 91, 82, 98];
#[c2rust::src_loc = "274:25"]
static silk_LTP_gain_vq_1_gain: [u8; 16] = [
    109, 120, 118, 12, 113, 115, 117, 119, 99, 59, 87, 111, 63, 111, 112, 80,
];
#[c2rust::src_loc = "279:25"]
static silk_LTP_gain_vq_2_gain: [u8; 32] = [
    126, 124, 125, 124, 129, 121, 126, 23, 132, 127, 127, 127, 126, 127, 122, 133, 130, 134, 101,
    118, 119, 145, 126, 86, 124, 120, 123, 119, 170, 173, 107, 109,
];
#[c2rust::src_loc = "286:26"]
pub static mut silk_LTP_vq_gain_ptrs_Q7: [*const u8; 3] = [0 as *const u8; 3];
#[c2rust::src_loc = "292:17"]
pub static silk_LTP_vq_sizes: [i8; 3] = [8, 16, 32];

// some array shenanigans going on here...
unsafe fn run_static_initializers() {
    silk_LTP_vq_ptrs_Q7 = [
        &*(*silk_LTP_gain_vq_0.as_ptr().offset(0 as i32 as isize))
            .as_ptr()
            .offset(0 as i32 as isize) as *const i8 as *mut i8 as *const i8,
        &*(*silk_LTP_gain_vq_1.as_ptr().offset(0 as i32 as isize))
            .as_ptr()
            .offset(0 as i32 as isize) as *const i8 as *mut i8 as *const i8,
        &*(*silk_LTP_gain_vq_2.as_ptr().offset(0 as i32 as isize))
            .as_ptr()
            .offset(0 as i32 as isize) as *const i8 as *mut i8 as *const i8,
    ];
    silk_LTP_vq_gain_ptrs_Q7 = [
        &*silk_LTP_gain_vq_0_gain.as_ptr().offset(0 as i32 as isize) as *const u8,
        &*silk_LTP_gain_vq_1_gain.as_ptr().offset(0 as i32 as isize) as *const u8,
        &*silk_LTP_gain_vq_2_gain.as_ptr().offset(0 as i32 as isize) as *const u8,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
