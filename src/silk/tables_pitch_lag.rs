use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "52:4"]
    pub type opus_uint8 = uint8_t;
    use super::stdint_uintn_h::uint8_t;
}
pub use self::opus_types_h::opus_uint8;
pub use self::stdint_uintn_h::uint8_t;
pub use self::types_h::__uint8_t;
#[no_mangle]
#[c2rust::src_loc = "34:18"]
pub static mut silk_pitch_lag_iCDF: [opus_uint8; 32] = [
    253 as libc::c_int as opus_uint8,
    250 as libc::c_int as opus_uint8,
    244 as libc::c_int as opus_uint8,
    233 as libc::c_int as opus_uint8,
    212 as libc::c_int as opus_uint8,
    182 as libc::c_int as opus_uint8,
    150 as libc::c_int as opus_uint8,
    131 as libc::c_int as opus_uint8,
    120 as libc::c_int as opus_uint8,
    110 as libc::c_int as opus_uint8,
    98 as libc::c_int as opus_uint8,
    85 as libc::c_int as opus_uint8,
    72 as libc::c_int as opus_uint8,
    60 as libc::c_int as opus_uint8,
    49 as libc::c_int as opus_uint8,
    40 as libc::c_int as opus_uint8,
    32 as libc::c_int as opus_uint8,
    25 as libc::c_int as opus_uint8,
    19 as libc::c_int as opus_uint8,
    15 as libc::c_int as opus_uint8,
    13 as libc::c_int as opus_uint8,
    11 as libc::c_int as opus_uint8,
    9 as libc::c_int as opus_uint8,
    8 as libc::c_int as opus_uint8,
    7 as libc::c_int as opus_uint8,
    6 as libc::c_int as opus_uint8,
    5 as libc::c_int as opus_uint8,
    4 as libc::c_int as opus_uint8,
    3 as libc::c_int as opus_uint8,
    2 as libc::c_int as opus_uint8,
    1 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "41:18"]
pub static mut silk_pitch_delta_iCDF: [opus_uint8; 21] = [
    210 as libc::c_int as opus_uint8,
    208 as libc::c_int as opus_uint8,
    206 as libc::c_int as opus_uint8,
    203 as libc::c_int as opus_uint8,
    199 as libc::c_int as opus_uint8,
    193 as libc::c_int as opus_uint8,
    183 as libc::c_int as opus_uint8,
    168 as libc::c_int as opus_uint8,
    142 as libc::c_int as opus_uint8,
    104 as libc::c_int as opus_uint8,
    74 as libc::c_int as opus_uint8,
    52 as libc::c_int as opus_uint8,
    37 as libc::c_int as opus_uint8,
    27 as libc::c_int as opus_uint8,
    20 as libc::c_int as opus_uint8,
    14 as libc::c_int as opus_uint8,
    10 as libc::c_int as opus_uint8,
    6 as libc::c_int as opus_uint8,
    4 as libc::c_int as opus_uint8,
    2 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "47:18"]
pub static mut silk_pitch_contour_iCDF: [opus_uint8; 34] = [
    223 as libc::c_int as opus_uint8,
    201 as libc::c_int as opus_uint8,
    183 as libc::c_int as opus_uint8,
    167 as libc::c_int as opus_uint8,
    152 as libc::c_int as opus_uint8,
    138 as libc::c_int as opus_uint8,
    124 as libc::c_int as opus_uint8,
    111 as libc::c_int as opus_uint8,
    98 as libc::c_int as opus_uint8,
    88 as libc::c_int as opus_uint8,
    79 as libc::c_int as opus_uint8,
    70 as libc::c_int as opus_uint8,
    62 as libc::c_int as opus_uint8,
    56 as libc::c_int as opus_uint8,
    50 as libc::c_int as opus_uint8,
    44 as libc::c_int as opus_uint8,
    39 as libc::c_int as opus_uint8,
    35 as libc::c_int as opus_uint8,
    31 as libc::c_int as opus_uint8,
    27 as libc::c_int as opus_uint8,
    24 as libc::c_int as opus_uint8,
    21 as libc::c_int as opus_uint8,
    18 as libc::c_int as opus_uint8,
    16 as libc::c_int as opus_uint8,
    14 as libc::c_int as opus_uint8,
    12 as libc::c_int as opus_uint8,
    10 as libc::c_int as opus_uint8,
    8 as libc::c_int as opus_uint8,
    6 as libc::c_int as opus_uint8,
    4 as libc::c_int as opus_uint8,
    3 as libc::c_int as opus_uint8,
    2 as libc::c_int as opus_uint8,
    1 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "55:18"]
pub static mut silk_pitch_contour_NB_iCDF: [opus_uint8; 11] = [
    188 as libc::c_int as opus_uint8,
    176 as libc::c_int as opus_uint8,
    155 as libc::c_int as opus_uint8,
    138 as libc::c_int as opus_uint8,
    119 as libc::c_int as opus_uint8,
    97 as libc::c_int as opus_uint8,
    67 as libc::c_int as opus_uint8,
    43 as libc::c_int as opus_uint8,
    26 as libc::c_int as opus_uint8,
    10 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "60:18"]
pub static mut silk_pitch_contour_10_ms_iCDF: [opus_uint8; 12] = [
    165 as libc::c_int as opus_uint8,
    119 as libc::c_int as opus_uint8,
    80 as libc::c_int as opus_uint8,
    61 as libc::c_int as opus_uint8,
    47 as libc::c_int as opus_uint8,
    35 as libc::c_int as opus_uint8,
    27 as libc::c_int as opus_uint8,
    20 as libc::c_int as opus_uint8,
    14 as libc::c_int as opus_uint8,
    9 as libc::c_int as opus_uint8,
    4 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "65:18"]
pub static mut silk_pitch_contour_10_ms_NB_iCDF: [opus_uint8; 3] = [
    113 as libc::c_int as opus_uint8,
    63 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
