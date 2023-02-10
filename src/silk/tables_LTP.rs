use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    use super::types_h::__int8_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    #[c2rust::src_loc = "52:4"]
    pub type opus_uint8 = uint8_t;
    use super::stdint_intn_h::int8_t;
    use super::stdint_uintn_h::uint8_t;
}
pub use self::opus_types_h::{opus_int8, opus_uint8};
pub use self::stdint_intn_h::int8_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::types_h::{__int8_t, __uint8_t};
#[no_mangle]
#[c2rust::src_loc = "34:18"]
pub static mut silk_LTP_per_index_iCDF: [opus_uint8; 3] = [
    179 as libc::c_int as opus_uint8,
    99 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[c2rust::src_loc = "38:25"]
static mut silk_LTP_gain_iCDF_0: [opus_uint8; 8] = [
    71 as libc::c_int as opus_uint8,
    56 as libc::c_int as opus_uint8,
    43 as libc::c_int as opus_uint8,
    30 as libc::c_int as opus_uint8,
    21 as libc::c_int as opus_uint8,
    12 as libc::c_int as opus_uint8,
    6 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[c2rust::src_loc = "42:25"]
static mut silk_LTP_gain_iCDF_1: [opus_uint8; 16] = [
    199 as libc::c_int as opus_uint8,
    165 as libc::c_int as opus_uint8,
    144 as libc::c_int as opus_uint8,
    124 as libc::c_int as opus_uint8,
    109 as libc::c_int as opus_uint8,
    96 as libc::c_int as opus_uint8,
    84 as libc::c_int as opus_uint8,
    71 as libc::c_int as opus_uint8,
    61 as libc::c_int as opus_uint8,
    51 as libc::c_int as opus_uint8,
    42 as libc::c_int as opus_uint8,
    32 as libc::c_int as opus_uint8,
    23 as libc::c_int as opus_uint8,
    15 as libc::c_int as opus_uint8,
    8 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[c2rust::src_loc = "47:25"]
static mut silk_LTP_gain_iCDF_2: [opus_uint8; 32] = [
    241 as libc::c_int as opus_uint8,
    225 as libc::c_int as opus_uint8,
    211 as libc::c_int as opus_uint8,
    199 as libc::c_int as opus_uint8,
    187 as libc::c_int as opus_uint8,
    175 as libc::c_int as opus_uint8,
    164 as libc::c_int as opus_uint8,
    153 as libc::c_int as opus_uint8,
    142 as libc::c_int as opus_uint8,
    132 as libc::c_int as opus_uint8,
    123 as libc::c_int as opus_uint8,
    114 as libc::c_int as opus_uint8,
    105 as libc::c_int as opus_uint8,
    96 as libc::c_int as opus_uint8,
    88 as libc::c_int as opus_uint8,
    80 as libc::c_int as opus_uint8,
    72 as libc::c_int as opus_uint8,
    64 as libc::c_int as opus_uint8,
    57 as libc::c_int as opus_uint8,
    50 as libc::c_int as opus_uint8,
    44 as libc::c_int as opus_uint8,
    38 as libc::c_int as opus_uint8,
    33 as libc::c_int as opus_uint8,
    29 as libc::c_int as opus_uint8,
    24 as libc::c_int as opus_uint8,
    20 as libc::c_int as opus_uint8,
    16 as libc::c_int as opus_uint8,
    12 as libc::c_int as opus_uint8,
    9 as libc::c_int as opus_uint8,
    5 as libc::c_int as opus_uint8,
    2 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[c2rust::src_loc = "54:25"]
static mut silk_LTP_gain_BITS_Q5_0: [opus_uint8; 8] = [
    15 as libc::c_int as opus_uint8,
    131 as libc::c_int as opus_uint8,
    138 as libc::c_int as opus_uint8,
    138 as libc::c_int as opus_uint8,
    155 as libc::c_int as opus_uint8,
    155 as libc::c_int as opus_uint8,
    173 as libc::c_int as opus_uint8,
    173 as libc::c_int as opus_uint8,
];
#[c2rust::src_loc = "58:25"]
static mut silk_LTP_gain_BITS_Q5_1: [opus_uint8; 16] = [
    69 as libc::c_int as opus_uint8,
    93 as libc::c_int as opus_uint8,
    115 as libc::c_int as opus_uint8,
    118 as libc::c_int as opus_uint8,
    131 as libc::c_int as opus_uint8,
    138 as libc::c_int as opus_uint8,
    141 as libc::c_int as opus_uint8,
    138 as libc::c_int as opus_uint8,
    150 as libc::c_int as opus_uint8,
    150 as libc::c_int as opus_uint8,
    155 as libc::c_int as opus_uint8,
    150 as libc::c_int as opus_uint8,
    155 as libc::c_int as opus_uint8,
    160 as libc::c_int as opus_uint8,
    166 as libc::c_int as opus_uint8,
    160 as libc::c_int as opus_uint8,
];
#[c2rust::src_loc = "63:25"]
static mut silk_LTP_gain_BITS_Q5_2: [opus_uint8; 32] = [
    131 as libc::c_int as opus_uint8,
    128 as libc::c_int as opus_uint8,
    134 as libc::c_int as opus_uint8,
    141 as libc::c_int as opus_uint8,
    141 as libc::c_int as opus_uint8,
    141 as libc::c_int as opus_uint8,
    145 as libc::c_int as opus_uint8,
    145 as libc::c_int as opus_uint8,
    145 as libc::c_int as opus_uint8,
    150 as libc::c_int as opus_uint8,
    155 as libc::c_int as opus_uint8,
    155 as libc::c_int as opus_uint8,
    155 as libc::c_int as opus_uint8,
    155 as libc::c_int as opus_uint8,
    160 as libc::c_int as opus_uint8,
    160 as libc::c_int as opus_uint8,
    160 as libc::c_int as opus_uint8,
    160 as libc::c_int as opus_uint8,
    166 as libc::c_int as opus_uint8,
    166 as libc::c_int as opus_uint8,
    173 as libc::c_int as opus_uint8,
    173 as libc::c_int as opus_uint8,
    182 as libc::c_int as opus_uint8,
    192 as libc::c_int as opus_uint8,
    182 as libc::c_int as opus_uint8,
    192 as libc::c_int as opus_uint8,
    192 as libc::c_int as opus_uint8,
    192 as libc::c_int as opus_uint8,
    205 as libc::c_int as opus_uint8,
    192 as libc::c_int as opus_uint8,
    205 as libc::c_int as opus_uint8,
    224 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "70:26"]
pub static mut silk_LTP_gain_iCDF_ptrs: [*const opus_uint8; 3] = unsafe {
    [
        silk_LTP_gain_iCDF_0.as_ptr(),
        silk_LTP_gain_iCDF_1.as_ptr(),
        silk_LTP_gain_iCDF_2.as_ptr(),
    ]
};
#[no_mangle]
#[c2rust::src_loc = "76:26"]
pub static mut silk_LTP_gain_BITS_Q5_ptrs: [*const opus_uint8; 3] = unsafe {
    [
        silk_LTP_gain_BITS_Q5_0.as_ptr(),
        silk_LTP_gain_BITS_Q5_1.as_ptr(),
        silk_LTP_gain_BITS_Q5_2.as_ptr(),
    ]
};
#[c2rust::src_loc = "82:24"]
static mut silk_LTP_gain_vq_0: [[opus_int8; 5]; 8] = [
    [
        4 as libc::c_int as opus_int8,
        6 as libc::c_int as opus_int8,
        24 as libc::c_int as opus_int8,
        7 as libc::c_int as opus_int8,
        5 as libc::c_int as opus_int8,
    ],
    [
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        2 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
    ],
    [
        12 as libc::c_int as opus_int8,
        28 as libc::c_int as opus_int8,
        41 as libc::c_int as opus_int8,
        13 as libc::c_int as opus_int8,
        -(4 as libc::c_int) as opus_int8,
    ],
    [
        -(9 as libc::c_int) as opus_int8,
        15 as libc::c_int as opus_int8,
        42 as libc::c_int as opus_int8,
        25 as libc::c_int as opus_int8,
        14 as libc::c_int as opus_int8,
    ],
    [
        1 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        62 as libc::c_int as opus_int8,
        41 as libc::c_int as opus_int8,
        -(9 as libc::c_int) as opus_int8,
    ],
    [
        -(10 as libc::c_int) as opus_int8,
        37 as libc::c_int as opus_int8,
        65 as libc::c_int as opus_int8,
        -(4 as libc::c_int) as opus_int8,
        3 as libc::c_int as opus_int8,
    ],
    [
        -(6 as libc::c_int) as opus_int8,
        4 as libc::c_int as opus_int8,
        66 as libc::c_int as opus_int8,
        7 as libc::c_int as opus_int8,
        -(8 as libc::c_int) as opus_int8,
    ],
    [
        16 as libc::c_int as opus_int8,
        14 as libc::c_int as opus_int8,
        38 as libc::c_int as opus_int8,
        -(3 as libc::c_int) as opus_int8,
        33 as libc::c_int as opus_int8,
    ],
];
#[c2rust::src_loc = "110:24"]
static mut silk_LTP_gain_vq_1: [[opus_int8; 5]; 16] = [
    [
        13 as libc::c_int as opus_int8,
        22 as libc::c_int as opus_int8,
        39 as libc::c_int as opus_int8,
        23 as libc::c_int as opus_int8,
        12 as libc::c_int as opus_int8,
    ],
    [
        -(1 as libc::c_int) as opus_int8,
        36 as libc::c_int as opus_int8,
        64 as libc::c_int as opus_int8,
        27 as libc::c_int as opus_int8,
        -(6 as libc::c_int) as opus_int8,
    ],
    [
        -(7 as libc::c_int) as opus_int8,
        10 as libc::c_int as opus_int8,
        55 as libc::c_int as opus_int8,
        43 as libc::c_int as opus_int8,
        17 as libc::c_int as opus_int8,
    ],
    [
        1 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        8 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
    ],
    [
        6 as libc::c_int as opus_int8,
        -(11 as libc::c_int) as opus_int8,
        74 as libc::c_int as opus_int8,
        53 as libc::c_int as opus_int8,
        -(9 as libc::c_int) as opus_int8,
    ],
    [
        -(12 as libc::c_int) as opus_int8,
        55 as libc::c_int as opus_int8,
        76 as libc::c_int as opus_int8,
        -(12 as libc::c_int) as opus_int8,
        8 as libc::c_int as opus_int8,
    ],
    [
        -(3 as libc::c_int) as opus_int8,
        3 as libc::c_int as opus_int8,
        93 as libc::c_int as opus_int8,
        27 as libc::c_int as opus_int8,
        -(4 as libc::c_int) as opus_int8,
    ],
    [
        26 as libc::c_int as opus_int8,
        39 as libc::c_int as opus_int8,
        59 as libc::c_int as opus_int8,
        3 as libc::c_int as opus_int8,
        -(8 as libc::c_int) as opus_int8,
    ],
    [
        2 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        77 as libc::c_int as opus_int8,
        11 as libc::c_int as opus_int8,
        9 as libc::c_int as opus_int8,
    ],
    [
        -(8 as libc::c_int) as opus_int8,
        22 as libc::c_int as opus_int8,
        44 as libc::c_int as opus_int8,
        -(6 as libc::c_int) as opus_int8,
        7 as libc::c_int as opus_int8,
    ],
    [
        40 as libc::c_int as opus_int8,
        9 as libc::c_int as opus_int8,
        26 as libc::c_int as opus_int8,
        3 as libc::c_int as opus_int8,
        9 as libc::c_int as opus_int8,
    ],
    [
        -(7 as libc::c_int) as opus_int8,
        20 as libc::c_int as opus_int8,
        101 as libc::c_int as opus_int8,
        -(7 as libc::c_int) as opus_int8,
        4 as libc::c_int as opus_int8,
    ],
    [
        3 as libc::c_int as opus_int8,
        -(8 as libc::c_int) as opus_int8,
        42 as libc::c_int as opus_int8,
        26 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
    ],
    [
        -(15 as libc::c_int) as opus_int8,
        33 as libc::c_int as opus_int8,
        68 as libc::c_int as opus_int8,
        2 as libc::c_int as opus_int8,
        23 as libc::c_int as opus_int8,
    ],
    [
        -(2 as libc::c_int) as opus_int8,
        55 as libc::c_int as opus_int8,
        46 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        15 as libc::c_int as opus_int8,
    ],
    [
        3 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        21 as libc::c_int as opus_int8,
        16 as libc::c_int as opus_int8,
        41 as libc::c_int as opus_int8,
    ],
];
#[c2rust::src_loc = "162:24"]
static mut silk_LTP_gain_vq_2: [[opus_int8; 5]; 32] = [
    [
        -(6 as libc::c_int) as opus_int8,
        27 as libc::c_int as opus_int8,
        61 as libc::c_int as opus_int8,
        39 as libc::c_int as opus_int8,
        5 as libc::c_int as opus_int8,
    ],
    [
        -(11 as libc::c_int) as opus_int8,
        42 as libc::c_int as opus_int8,
        88 as libc::c_int as opus_int8,
        4 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
    ],
    [
        -(2 as libc::c_int) as opus_int8,
        60 as libc::c_int as opus_int8,
        65 as libc::c_int as opus_int8,
        6 as libc::c_int as opus_int8,
        -(4 as libc::c_int) as opus_int8,
    ],
    [
        -(1 as libc::c_int) as opus_int8,
        -(5 as libc::c_int) as opus_int8,
        73 as libc::c_int as opus_int8,
        56 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
    ],
    [
        -(9 as libc::c_int) as opus_int8,
        19 as libc::c_int as opus_int8,
        94 as libc::c_int as opus_int8,
        29 as libc::c_int as opus_int8,
        -(9 as libc::c_int) as opus_int8,
    ],
    [
        0 as libc::c_int as opus_int8,
        12 as libc::c_int as opus_int8,
        99 as libc::c_int as opus_int8,
        6 as libc::c_int as opus_int8,
        4 as libc::c_int as opus_int8,
    ],
    [
        8 as libc::c_int as opus_int8,
        -(19 as libc::c_int) as opus_int8,
        102 as libc::c_int as opus_int8,
        46 as libc::c_int as opus_int8,
        -(13 as libc::c_int) as opus_int8,
    ],
    [
        3 as libc::c_int as opus_int8,
        2 as libc::c_int as opus_int8,
        13 as libc::c_int as opus_int8,
        3 as libc::c_int as opus_int8,
        2 as libc::c_int as opus_int8,
    ],
    [
        9 as libc::c_int as opus_int8,
        -(21 as libc::c_int) as opus_int8,
        84 as libc::c_int as opus_int8,
        72 as libc::c_int as opus_int8,
        -(18 as libc::c_int) as opus_int8,
    ],
    [
        -(11 as libc::c_int) as opus_int8,
        46 as libc::c_int as opus_int8,
        104 as libc::c_int as opus_int8,
        -(22 as libc::c_int) as opus_int8,
        8 as libc::c_int as opus_int8,
    ],
    [
        18 as libc::c_int as opus_int8,
        38 as libc::c_int as opus_int8,
        48 as libc::c_int as opus_int8,
        23 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
    ],
    [
        -(16 as libc::c_int) as opus_int8,
        70 as libc::c_int as opus_int8,
        83 as libc::c_int as opus_int8,
        -(21 as libc::c_int) as opus_int8,
        11 as libc::c_int as opus_int8,
    ],
    [
        5 as libc::c_int as opus_int8,
        -(11 as libc::c_int) as opus_int8,
        117 as libc::c_int as opus_int8,
        22 as libc::c_int as opus_int8,
        -(8 as libc::c_int) as opus_int8,
    ],
    [
        -(6 as libc::c_int) as opus_int8,
        23 as libc::c_int as opus_int8,
        117 as libc::c_int as opus_int8,
        -(12 as libc::c_int) as opus_int8,
        3 as libc::c_int as opus_int8,
    ],
    [
        3 as libc::c_int as opus_int8,
        -(8 as libc::c_int) as opus_int8,
        95 as libc::c_int as opus_int8,
        28 as libc::c_int as opus_int8,
        4 as libc::c_int as opus_int8,
    ],
    [
        -(10 as libc::c_int) as opus_int8,
        15 as libc::c_int as opus_int8,
        77 as libc::c_int as opus_int8,
        60 as libc::c_int as opus_int8,
        -(15 as libc::c_int) as opus_int8,
    ],
    [
        -(1 as libc::c_int) as opus_int8,
        4 as libc::c_int as opus_int8,
        124 as libc::c_int as opus_int8,
        2 as libc::c_int as opus_int8,
        -(4 as libc::c_int) as opus_int8,
    ],
    [
        3 as libc::c_int as opus_int8,
        38 as libc::c_int as opus_int8,
        84 as libc::c_int as opus_int8,
        24 as libc::c_int as opus_int8,
        -(25 as libc::c_int) as opus_int8,
    ],
    [
        2 as libc::c_int as opus_int8,
        13 as libc::c_int as opus_int8,
        42 as libc::c_int as opus_int8,
        13 as libc::c_int as opus_int8,
        31 as libc::c_int as opus_int8,
    ],
    [
        21 as libc::c_int as opus_int8,
        -(4 as libc::c_int) as opus_int8,
        56 as libc::c_int as opus_int8,
        46 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
    ],
    [
        -(1 as libc::c_int) as opus_int8,
        35 as libc::c_int as opus_int8,
        79 as libc::c_int as opus_int8,
        -(13 as libc::c_int) as opus_int8,
        19 as libc::c_int as opus_int8,
    ],
    [
        -(7 as libc::c_int) as opus_int8,
        65 as libc::c_int as opus_int8,
        88 as libc::c_int as opus_int8,
        -(9 as libc::c_int) as opus_int8,
        -(14 as libc::c_int) as opus_int8,
    ],
    [
        20 as libc::c_int as opus_int8,
        4 as libc::c_int as opus_int8,
        81 as libc::c_int as opus_int8,
        49 as libc::c_int as opus_int8,
        -(29 as libc::c_int) as opus_int8,
    ],
    [
        20 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        75 as libc::c_int as opus_int8,
        3 as libc::c_int as opus_int8,
        -(17 as libc::c_int) as opus_int8,
    ],
    [
        5 as libc::c_int as opus_int8,
        -(9 as libc::c_int) as opus_int8,
        44 as libc::c_int as opus_int8,
        92 as libc::c_int as opus_int8,
        -(8 as libc::c_int) as opus_int8,
    ],
    [
        1 as libc::c_int as opus_int8,
        -(3 as libc::c_int) as opus_int8,
        22 as libc::c_int as opus_int8,
        69 as libc::c_int as opus_int8,
        31 as libc::c_int as opus_int8,
    ],
    [
        -(6 as libc::c_int) as opus_int8,
        95 as libc::c_int as opus_int8,
        41 as libc::c_int as opus_int8,
        -(12 as libc::c_int) as opus_int8,
        5 as libc::c_int as opus_int8,
    ],
    [
        39 as libc::c_int as opus_int8,
        67 as libc::c_int as opus_int8,
        16 as libc::c_int as opus_int8,
        -(4 as libc::c_int) as opus_int8,
        1 as libc::c_int as opus_int8,
    ],
    [
        0 as libc::c_int as opus_int8,
        -(6 as libc::c_int) as opus_int8,
        120 as libc::c_int as opus_int8,
        55 as libc::c_int as opus_int8,
        -(36 as libc::c_int) as opus_int8,
    ],
    [
        -(13 as libc::c_int) as opus_int8,
        44 as libc::c_int as opus_int8,
        122 as libc::c_int as opus_int8,
        4 as libc::c_int as opus_int8,
        -(24 as libc::c_int) as opus_int8,
    ],
    [
        81 as libc::c_int as opus_int8,
        5 as libc::c_int as opus_int8,
        11 as libc::c_int as opus_int8,
        3 as libc::c_int as opus_int8,
        7 as libc::c_int as opus_int8,
    ],
    [
        2 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        9 as libc::c_int as opus_int8,
        10 as libc::c_int as opus_int8,
        88 as libc::c_int as opus_int8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "262:25"]
pub static mut silk_LTP_vq_ptrs_Q7: [*const opus_int8; 3] = [0 as *const opus_int8; 3];
#[c2rust::src_loc = "270:25"]
static mut silk_LTP_gain_vq_0_gain: [opus_uint8; 8] = [
    46 as libc::c_int as opus_uint8,
    2 as libc::c_int as opus_uint8,
    90 as libc::c_int as opus_uint8,
    87 as libc::c_int as opus_uint8,
    93 as libc::c_int as opus_uint8,
    91 as libc::c_int as opus_uint8,
    82 as libc::c_int as opus_uint8,
    98 as libc::c_int as opus_uint8,
];
#[c2rust::src_loc = "274:25"]
static mut silk_LTP_gain_vq_1_gain: [opus_uint8; 16] = [
    109 as libc::c_int as opus_uint8,
    120 as libc::c_int as opus_uint8,
    118 as libc::c_int as opus_uint8,
    12 as libc::c_int as opus_uint8,
    113 as libc::c_int as opus_uint8,
    115 as libc::c_int as opus_uint8,
    117 as libc::c_int as opus_uint8,
    119 as libc::c_int as opus_uint8,
    99 as libc::c_int as opus_uint8,
    59 as libc::c_int as opus_uint8,
    87 as libc::c_int as opus_uint8,
    111 as libc::c_int as opus_uint8,
    63 as libc::c_int as opus_uint8,
    111 as libc::c_int as opus_uint8,
    112 as libc::c_int as opus_uint8,
    80 as libc::c_int as opus_uint8,
];
#[c2rust::src_loc = "279:25"]
static mut silk_LTP_gain_vq_2_gain: [opus_uint8; 32] = [
    126 as libc::c_int as opus_uint8,
    124 as libc::c_int as opus_uint8,
    125 as libc::c_int as opus_uint8,
    124 as libc::c_int as opus_uint8,
    129 as libc::c_int as opus_uint8,
    121 as libc::c_int as opus_uint8,
    126 as libc::c_int as opus_uint8,
    23 as libc::c_int as opus_uint8,
    132 as libc::c_int as opus_uint8,
    127 as libc::c_int as opus_uint8,
    127 as libc::c_int as opus_uint8,
    127 as libc::c_int as opus_uint8,
    126 as libc::c_int as opus_uint8,
    127 as libc::c_int as opus_uint8,
    122 as libc::c_int as opus_uint8,
    133 as libc::c_int as opus_uint8,
    130 as libc::c_int as opus_uint8,
    134 as libc::c_int as opus_uint8,
    101 as libc::c_int as opus_uint8,
    118 as libc::c_int as opus_uint8,
    119 as libc::c_int as opus_uint8,
    145 as libc::c_int as opus_uint8,
    126 as libc::c_int as opus_uint8,
    86 as libc::c_int as opus_uint8,
    124 as libc::c_int as opus_uint8,
    120 as libc::c_int as opus_uint8,
    123 as libc::c_int as opus_uint8,
    119 as libc::c_int as opus_uint8,
    170 as libc::c_int as opus_uint8,
    173 as libc::c_int as opus_uint8,
    107 as libc::c_int as opus_uint8,
    109 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "286:26"]
pub static mut silk_LTP_vq_gain_ptrs_Q7: [*const opus_uint8; 3] = [0 as *const opus_uint8; 3];
#[no_mangle]
#[c2rust::src_loc = "292:17"]
pub static mut silk_LTP_vq_sizes: [opus_int8; 3] = [
    8 as libc::c_int as opus_int8,
    16 as libc::c_int as opus_int8,
    32 as libc::c_int as opus_int8,
];
unsafe extern "C" fn run_static_initializers() {
    silk_LTP_vq_ptrs_Q7 = [
        &*(*silk_LTP_gain_vq_0
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const opus_int8 as *mut opus_int8
            as *const opus_int8,
        &*(*silk_LTP_gain_vq_1
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const opus_int8 as *mut opus_int8
            as *const opus_int8,
        &*(*silk_LTP_gain_vq_2
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const opus_int8 as *mut opus_int8
            as *const opus_int8,
    ];
    silk_LTP_vq_gain_ptrs_Q7 = [
        &*silk_LTP_gain_vq_0_gain
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const opus_uint8,
        &*silk_LTP_gain_vq_1_gain
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const opus_uint8,
        &*silk_LTP_gain_vq_2_gain
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const opus_uint8,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
