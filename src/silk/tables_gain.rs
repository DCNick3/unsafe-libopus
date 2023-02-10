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
#[c2rust::src_loc = "39:18"]
pub static mut silk_gain_iCDF: [[u8; 8]; 3] = [
    [
        224 as libc::c_int as u8,
        112 as libc::c_int as u8,
        44 as libc::c_int as u8,
        15 as libc::c_int as u8,
        3 as libc::c_int as u8,
        2 as libc::c_int as u8,
        1 as libc::c_int as u8,
        0 as libc::c_int as u8,
    ],
    [
        254 as libc::c_int as u8,
        237 as libc::c_int as u8,
        192 as libc::c_int as u8,
        132 as libc::c_int as u8,
        70 as libc::c_int as u8,
        23 as libc::c_int as u8,
        4 as libc::c_int as u8,
        0 as libc::c_int as u8,
    ],
    [
        255 as libc::c_int as u8,
        252 as libc::c_int as u8,
        226 as libc::c_int as u8,
        155 as libc::c_int as u8,
        61 as libc::c_int as u8,
        11 as libc::c_int as u8,
        2 as libc::c_int as u8,
        0 as libc::c_int as u8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "52:18"]
pub static mut silk_delta_gain_iCDF: [u8; 41] = [
    250 as libc::c_int as u8,
    245 as libc::c_int as u8,
    234 as libc::c_int as u8,
    203 as libc::c_int as u8,
    71 as libc::c_int as u8,
    50 as libc::c_int as u8,
    42 as libc::c_int as u8,
    38 as libc::c_int as u8,
    35 as libc::c_int as u8,
    33 as libc::c_int as u8,
    31 as libc::c_int as u8,
    29 as libc::c_int as u8,
    28 as libc::c_int as u8,
    27 as libc::c_int as u8,
    26 as libc::c_int as u8,
    25 as libc::c_int as u8,
    24 as libc::c_int as u8,
    23 as libc::c_int as u8,
    22 as libc::c_int as u8,
    21 as libc::c_int as u8,
    20 as libc::c_int as u8,
    19 as libc::c_int as u8,
    18 as libc::c_int as u8,
    17 as libc::c_int as u8,
    16 as libc::c_int as u8,
    15 as libc::c_int as u8,
    14 as libc::c_int as u8,
    13 as libc::c_int as u8,
    12 as libc::c_int as u8,
    11 as libc::c_int as u8,
    10 as libc::c_int as u8,
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
