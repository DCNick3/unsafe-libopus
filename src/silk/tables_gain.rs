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
#[c2rust::src_loc = "39:18"]
pub static mut silk_gain_iCDF: [[opus_uint8; 8]; 3] = [
    [
        224 as libc::c_int as opus_uint8,
        112 as libc::c_int as opus_uint8,
        44 as libc::c_int as opus_uint8,
        15 as libc::c_int as opus_uint8,
        3 as libc::c_int as opus_uint8,
        2 as libc::c_int as opus_uint8,
        1 as libc::c_int as opus_uint8,
        0 as libc::c_int as opus_uint8,
    ],
    [
        254 as libc::c_int as opus_uint8,
        237 as libc::c_int as opus_uint8,
        192 as libc::c_int as opus_uint8,
        132 as libc::c_int as opus_uint8,
        70 as libc::c_int as opus_uint8,
        23 as libc::c_int as opus_uint8,
        4 as libc::c_int as opus_uint8,
        0 as libc::c_int as opus_uint8,
    ],
    [
        255 as libc::c_int as opus_uint8,
        252 as libc::c_int as opus_uint8,
        226 as libc::c_int as opus_uint8,
        155 as libc::c_int as opus_uint8,
        61 as libc::c_int as opus_uint8,
        11 as libc::c_int as opus_uint8,
        2 as libc::c_int as opus_uint8,
        0 as libc::c_int as opus_uint8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "52:18"]
pub static mut silk_delta_gain_iCDF: [opus_uint8; 41] = [
    250 as libc::c_int as opus_uint8,
    245 as libc::c_int as opus_uint8,
    234 as libc::c_int as opus_uint8,
    203 as libc::c_int as opus_uint8,
    71 as libc::c_int as opus_uint8,
    50 as libc::c_int as opus_uint8,
    42 as libc::c_int as opus_uint8,
    38 as libc::c_int as opus_uint8,
    35 as libc::c_int as opus_uint8,
    33 as libc::c_int as opus_uint8,
    31 as libc::c_int as opus_uint8,
    29 as libc::c_int as opus_uint8,
    28 as libc::c_int as opus_uint8,
    27 as libc::c_int as opus_uint8,
    26 as libc::c_int as opus_uint8,
    25 as libc::c_int as opus_uint8,
    24 as libc::c_int as opus_uint8,
    23 as libc::c_int as opus_uint8,
    22 as libc::c_int as opus_uint8,
    21 as libc::c_int as opus_uint8,
    20 as libc::c_int as opus_uint8,
    19 as libc::c_int as opus_uint8,
    18 as libc::c_int as opus_uint8,
    17 as libc::c_int as opus_uint8,
    16 as libc::c_int as opus_uint8,
    15 as libc::c_int as opus_uint8,
    14 as libc::c_int as opus_uint8,
    13 as libc::c_int as opus_uint8,
    12 as libc::c_int as opus_uint8,
    11 as libc::c_int as opus_uint8,
    10 as libc::c_int as opus_uint8,
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
