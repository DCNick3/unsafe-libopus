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
pub use self::stdint_uintn_h::uint8_t;
pub use self::types_h::__uint8_t;
#[no_mangle]
#[c2rust::src_loc = "34:18"]
pub static mut silk_pitch_lag_iCDF: [u8; 32] = [
    253 as libc::c_int as u8,
    250 as libc::c_int as u8,
    244 as libc::c_int as u8,
    233 as libc::c_int as u8,
    212 as libc::c_int as u8,
    182 as libc::c_int as u8,
    150 as libc::c_int as u8,
    131 as libc::c_int as u8,
    120 as libc::c_int as u8,
    110 as libc::c_int as u8,
    98 as libc::c_int as u8,
    85 as libc::c_int as u8,
    72 as libc::c_int as u8,
    60 as libc::c_int as u8,
    49 as libc::c_int as u8,
    40 as libc::c_int as u8,
    32 as libc::c_int as u8,
    25 as libc::c_int as u8,
    19 as libc::c_int as u8,
    15 as libc::c_int as u8,
    13 as libc::c_int as u8,
    11 as libc::c_int as u8,
    9 as libc::c_int as u8,
    8 as libc::c_int as u8,
    7 as libc::c_int as u8,
    6 as libc::c_int as u8,
    5 as libc::c_int as u8,
    4 as libc::c_int as u8,
    3 as libc::c_int as u8,
    2 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "41:18"]
pub static mut silk_pitch_delta_iCDF: [u8; 21] = [
    210 as libc::c_int as u8,
    208 as libc::c_int as u8,
    206 as libc::c_int as u8,
    203 as libc::c_int as u8,
    199 as libc::c_int as u8,
    193 as libc::c_int as u8,
    183 as libc::c_int as u8,
    168 as libc::c_int as u8,
    142 as libc::c_int as u8,
    104 as libc::c_int as u8,
    74 as libc::c_int as u8,
    52 as libc::c_int as u8,
    37 as libc::c_int as u8,
    27 as libc::c_int as u8,
    20 as libc::c_int as u8,
    14 as libc::c_int as u8,
    10 as libc::c_int as u8,
    6 as libc::c_int as u8,
    4 as libc::c_int as u8,
    2 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "47:18"]
pub static mut silk_pitch_contour_iCDF: [u8; 34] = [
    223 as libc::c_int as u8,
    201 as libc::c_int as u8,
    183 as libc::c_int as u8,
    167 as libc::c_int as u8,
    152 as libc::c_int as u8,
    138 as libc::c_int as u8,
    124 as libc::c_int as u8,
    111 as libc::c_int as u8,
    98 as libc::c_int as u8,
    88 as libc::c_int as u8,
    79 as libc::c_int as u8,
    70 as libc::c_int as u8,
    62 as libc::c_int as u8,
    56 as libc::c_int as u8,
    50 as libc::c_int as u8,
    44 as libc::c_int as u8,
    39 as libc::c_int as u8,
    35 as libc::c_int as u8,
    31 as libc::c_int as u8,
    27 as libc::c_int as u8,
    24 as libc::c_int as u8,
    21 as libc::c_int as u8,
    18 as libc::c_int as u8,
    16 as libc::c_int as u8,
    14 as libc::c_int as u8,
    12 as libc::c_int as u8,
    10 as libc::c_int as u8,
    8 as libc::c_int as u8,
    6 as libc::c_int as u8,
    4 as libc::c_int as u8,
    3 as libc::c_int as u8,
    2 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "55:18"]
pub static mut silk_pitch_contour_NB_iCDF: [u8; 11] = [
    188 as libc::c_int as u8,
    176 as libc::c_int as u8,
    155 as libc::c_int as u8,
    138 as libc::c_int as u8,
    119 as libc::c_int as u8,
    97 as libc::c_int as u8,
    67 as libc::c_int as u8,
    43 as libc::c_int as u8,
    26 as libc::c_int as u8,
    10 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "60:18"]
pub static mut silk_pitch_contour_10_ms_iCDF: [u8; 12] = [
    165 as libc::c_int as u8,
    119 as libc::c_int as u8,
    80 as libc::c_int as u8,
    61 as libc::c_int as u8,
    47 as libc::c_int as u8,
    35 as libc::c_int as u8,
    27 as libc::c_int as u8,
    20 as libc::c_int as u8,
    14 as libc::c_int as u8,
    9 as libc::c_int as u8,
    4 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "65:18"]
pub static mut silk_pitch_contour_10_ms_NB_iCDF: [u8; 3] = [
    113 as libc::c_int as u8,
    63 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
