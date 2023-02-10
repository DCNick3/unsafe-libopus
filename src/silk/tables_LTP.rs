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
pub use self::stdint_intn_h::int8_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::types_h::{__int8_t, __uint8_t};
#[no_mangle]
#[c2rust::src_loc = "34:18"]
pub static mut silk_LTP_per_index_iCDF: [u8; 3] = [
    179 as libc::c_int as u8,
    99 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[c2rust::src_loc = "38:25"]
static mut silk_LTP_gain_iCDF_0: [u8; 8] = [
    71 as libc::c_int as u8,
    56 as libc::c_int as u8,
    43 as libc::c_int as u8,
    30 as libc::c_int as u8,
    21 as libc::c_int as u8,
    12 as libc::c_int as u8,
    6 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[c2rust::src_loc = "42:25"]
static mut silk_LTP_gain_iCDF_1: [u8; 16] = [
    199 as libc::c_int as u8,
    165 as libc::c_int as u8,
    144 as libc::c_int as u8,
    124 as libc::c_int as u8,
    109 as libc::c_int as u8,
    96 as libc::c_int as u8,
    84 as libc::c_int as u8,
    71 as libc::c_int as u8,
    61 as libc::c_int as u8,
    51 as libc::c_int as u8,
    42 as libc::c_int as u8,
    32 as libc::c_int as u8,
    23 as libc::c_int as u8,
    15 as libc::c_int as u8,
    8 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[c2rust::src_loc = "47:25"]
static mut silk_LTP_gain_iCDF_2: [u8; 32] = [
    241 as libc::c_int as u8,
    225 as libc::c_int as u8,
    211 as libc::c_int as u8,
    199 as libc::c_int as u8,
    187 as libc::c_int as u8,
    175 as libc::c_int as u8,
    164 as libc::c_int as u8,
    153 as libc::c_int as u8,
    142 as libc::c_int as u8,
    132 as libc::c_int as u8,
    123 as libc::c_int as u8,
    114 as libc::c_int as u8,
    105 as libc::c_int as u8,
    96 as libc::c_int as u8,
    88 as libc::c_int as u8,
    80 as libc::c_int as u8,
    72 as libc::c_int as u8,
    64 as libc::c_int as u8,
    57 as libc::c_int as u8,
    50 as libc::c_int as u8,
    44 as libc::c_int as u8,
    38 as libc::c_int as u8,
    33 as libc::c_int as u8,
    29 as libc::c_int as u8,
    24 as libc::c_int as u8,
    20 as libc::c_int as u8,
    16 as libc::c_int as u8,
    12 as libc::c_int as u8,
    9 as libc::c_int as u8,
    5 as libc::c_int as u8,
    2 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[c2rust::src_loc = "54:25"]
static mut silk_LTP_gain_BITS_Q5_0: [u8; 8] = [
    15 as libc::c_int as u8,
    131 as libc::c_int as u8,
    138 as libc::c_int as u8,
    138 as libc::c_int as u8,
    155 as libc::c_int as u8,
    155 as libc::c_int as u8,
    173 as libc::c_int as u8,
    173 as libc::c_int as u8,
];
#[c2rust::src_loc = "58:25"]
static mut silk_LTP_gain_BITS_Q5_1: [u8; 16] = [
    69 as libc::c_int as u8,
    93 as libc::c_int as u8,
    115 as libc::c_int as u8,
    118 as libc::c_int as u8,
    131 as libc::c_int as u8,
    138 as libc::c_int as u8,
    141 as libc::c_int as u8,
    138 as libc::c_int as u8,
    150 as libc::c_int as u8,
    150 as libc::c_int as u8,
    155 as libc::c_int as u8,
    150 as libc::c_int as u8,
    155 as libc::c_int as u8,
    160 as libc::c_int as u8,
    166 as libc::c_int as u8,
    160 as libc::c_int as u8,
];
#[c2rust::src_loc = "63:25"]
static mut silk_LTP_gain_BITS_Q5_2: [u8; 32] = [
    131 as libc::c_int as u8,
    128 as libc::c_int as u8,
    134 as libc::c_int as u8,
    141 as libc::c_int as u8,
    141 as libc::c_int as u8,
    141 as libc::c_int as u8,
    145 as libc::c_int as u8,
    145 as libc::c_int as u8,
    145 as libc::c_int as u8,
    150 as libc::c_int as u8,
    155 as libc::c_int as u8,
    155 as libc::c_int as u8,
    155 as libc::c_int as u8,
    155 as libc::c_int as u8,
    160 as libc::c_int as u8,
    160 as libc::c_int as u8,
    160 as libc::c_int as u8,
    160 as libc::c_int as u8,
    166 as libc::c_int as u8,
    166 as libc::c_int as u8,
    173 as libc::c_int as u8,
    173 as libc::c_int as u8,
    182 as libc::c_int as u8,
    192 as libc::c_int as u8,
    182 as libc::c_int as u8,
    192 as libc::c_int as u8,
    192 as libc::c_int as u8,
    192 as libc::c_int as u8,
    205 as libc::c_int as u8,
    192 as libc::c_int as u8,
    205 as libc::c_int as u8,
    224 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "70:26"]
pub static mut silk_LTP_gain_iCDF_ptrs: [*const u8; 3] = unsafe {
    [
        silk_LTP_gain_iCDF_0.as_ptr(),
        silk_LTP_gain_iCDF_1.as_ptr(),
        silk_LTP_gain_iCDF_2.as_ptr(),
    ]
};
#[no_mangle]
#[c2rust::src_loc = "76:26"]
pub static mut silk_LTP_gain_BITS_Q5_ptrs: [*const u8; 3] = unsafe {
    [
        silk_LTP_gain_BITS_Q5_0.as_ptr(),
        silk_LTP_gain_BITS_Q5_1.as_ptr(),
        silk_LTP_gain_BITS_Q5_2.as_ptr(),
    ]
};
#[c2rust::src_loc = "82:24"]
static mut silk_LTP_gain_vq_0: [[i8; 5]; 8] = [
    [
        4 as libc::c_int as i8,
        6 as libc::c_int as i8,
        24 as libc::c_int as i8,
        7 as libc::c_int as i8,
        5 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        2 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
    ],
    [
        12 as libc::c_int as i8,
        28 as libc::c_int as i8,
        41 as libc::c_int as i8,
        13 as libc::c_int as i8,
        -(4 as libc::c_int) as i8,
    ],
    [
        -(9 as libc::c_int) as i8,
        15 as libc::c_int as i8,
        42 as libc::c_int as i8,
        25 as libc::c_int as i8,
        14 as libc::c_int as i8,
    ],
    [
        1 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        62 as libc::c_int as i8,
        41 as libc::c_int as i8,
        -(9 as libc::c_int) as i8,
    ],
    [
        -(10 as libc::c_int) as i8,
        37 as libc::c_int as i8,
        65 as libc::c_int as i8,
        -(4 as libc::c_int) as i8,
        3 as libc::c_int as i8,
    ],
    [
        -(6 as libc::c_int) as i8,
        4 as libc::c_int as i8,
        66 as libc::c_int as i8,
        7 as libc::c_int as i8,
        -(8 as libc::c_int) as i8,
    ],
    [
        16 as libc::c_int as i8,
        14 as libc::c_int as i8,
        38 as libc::c_int as i8,
        -(3 as libc::c_int) as i8,
        33 as libc::c_int as i8,
    ],
];
#[c2rust::src_loc = "110:24"]
static mut silk_LTP_gain_vq_1: [[i8; 5]; 16] = [
    [
        13 as libc::c_int as i8,
        22 as libc::c_int as i8,
        39 as libc::c_int as i8,
        23 as libc::c_int as i8,
        12 as libc::c_int as i8,
    ],
    [
        -(1 as libc::c_int) as i8,
        36 as libc::c_int as i8,
        64 as libc::c_int as i8,
        27 as libc::c_int as i8,
        -(6 as libc::c_int) as i8,
    ],
    [
        -(7 as libc::c_int) as i8,
        10 as libc::c_int as i8,
        55 as libc::c_int as i8,
        43 as libc::c_int as i8,
        17 as libc::c_int as i8,
    ],
    [
        1 as libc::c_int as i8,
        1 as libc::c_int as i8,
        8 as libc::c_int as i8,
        1 as libc::c_int as i8,
        1 as libc::c_int as i8,
    ],
    [
        6 as libc::c_int as i8,
        -(11 as libc::c_int) as i8,
        74 as libc::c_int as i8,
        53 as libc::c_int as i8,
        -(9 as libc::c_int) as i8,
    ],
    [
        -(12 as libc::c_int) as i8,
        55 as libc::c_int as i8,
        76 as libc::c_int as i8,
        -(12 as libc::c_int) as i8,
        8 as libc::c_int as i8,
    ],
    [
        -(3 as libc::c_int) as i8,
        3 as libc::c_int as i8,
        93 as libc::c_int as i8,
        27 as libc::c_int as i8,
        -(4 as libc::c_int) as i8,
    ],
    [
        26 as libc::c_int as i8,
        39 as libc::c_int as i8,
        59 as libc::c_int as i8,
        3 as libc::c_int as i8,
        -(8 as libc::c_int) as i8,
    ],
    [
        2 as libc::c_int as i8,
        0 as libc::c_int as i8,
        77 as libc::c_int as i8,
        11 as libc::c_int as i8,
        9 as libc::c_int as i8,
    ],
    [
        -(8 as libc::c_int) as i8,
        22 as libc::c_int as i8,
        44 as libc::c_int as i8,
        -(6 as libc::c_int) as i8,
        7 as libc::c_int as i8,
    ],
    [
        40 as libc::c_int as i8,
        9 as libc::c_int as i8,
        26 as libc::c_int as i8,
        3 as libc::c_int as i8,
        9 as libc::c_int as i8,
    ],
    [
        -(7 as libc::c_int) as i8,
        20 as libc::c_int as i8,
        101 as libc::c_int as i8,
        -(7 as libc::c_int) as i8,
        4 as libc::c_int as i8,
    ],
    [
        3 as libc::c_int as i8,
        -(8 as libc::c_int) as i8,
        42 as libc::c_int as i8,
        26 as libc::c_int as i8,
        0 as libc::c_int as i8,
    ],
    [
        -(15 as libc::c_int) as i8,
        33 as libc::c_int as i8,
        68 as libc::c_int as i8,
        2 as libc::c_int as i8,
        23 as libc::c_int as i8,
    ],
    [
        -(2 as libc::c_int) as i8,
        55 as libc::c_int as i8,
        46 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        15 as libc::c_int as i8,
    ],
    [
        3 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        21 as libc::c_int as i8,
        16 as libc::c_int as i8,
        41 as libc::c_int as i8,
    ],
];
#[c2rust::src_loc = "162:24"]
static mut silk_LTP_gain_vq_2: [[i8; 5]; 32] = [
    [
        -(6 as libc::c_int) as i8,
        27 as libc::c_int as i8,
        61 as libc::c_int as i8,
        39 as libc::c_int as i8,
        5 as libc::c_int as i8,
    ],
    [
        -(11 as libc::c_int) as i8,
        42 as libc::c_int as i8,
        88 as libc::c_int as i8,
        4 as libc::c_int as i8,
        1 as libc::c_int as i8,
    ],
    [
        -(2 as libc::c_int) as i8,
        60 as libc::c_int as i8,
        65 as libc::c_int as i8,
        6 as libc::c_int as i8,
        -(4 as libc::c_int) as i8,
    ],
    [
        -(1 as libc::c_int) as i8,
        -(5 as libc::c_int) as i8,
        73 as libc::c_int as i8,
        56 as libc::c_int as i8,
        1 as libc::c_int as i8,
    ],
    [
        -(9 as libc::c_int) as i8,
        19 as libc::c_int as i8,
        94 as libc::c_int as i8,
        29 as libc::c_int as i8,
        -(9 as libc::c_int) as i8,
    ],
    [
        0 as libc::c_int as i8,
        12 as libc::c_int as i8,
        99 as libc::c_int as i8,
        6 as libc::c_int as i8,
        4 as libc::c_int as i8,
    ],
    [
        8 as libc::c_int as i8,
        -(19 as libc::c_int) as i8,
        102 as libc::c_int as i8,
        46 as libc::c_int as i8,
        -(13 as libc::c_int) as i8,
    ],
    [
        3 as libc::c_int as i8,
        2 as libc::c_int as i8,
        13 as libc::c_int as i8,
        3 as libc::c_int as i8,
        2 as libc::c_int as i8,
    ],
    [
        9 as libc::c_int as i8,
        -(21 as libc::c_int) as i8,
        84 as libc::c_int as i8,
        72 as libc::c_int as i8,
        -(18 as libc::c_int) as i8,
    ],
    [
        -(11 as libc::c_int) as i8,
        46 as libc::c_int as i8,
        104 as libc::c_int as i8,
        -(22 as libc::c_int) as i8,
        8 as libc::c_int as i8,
    ],
    [
        18 as libc::c_int as i8,
        38 as libc::c_int as i8,
        48 as libc::c_int as i8,
        23 as libc::c_int as i8,
        0 as libc::c_int as i8,
    ],
    [
        -(16 as libc::c_int) as i8,
        70 as libc::c_int as i8,
        83 as libc::c_int as i8,
        -(21 as libc::c_int) as i8,
        11 as libc::c_int as i8,
    ],
    [
        5 as libc::c_int as i8,
        -(11 as libc::c_int) as i8,
        117 as libc::c_int as i8,
        22 as libc::c_int as i8,
        -(8 as libc::c_int) as i8,
    ],
    [
        -(6 as libc::c_int) as i8,
        23 as libc::c_int as i8,
        117 as libc::c_int as i8,
        -(12 as libc::c_int) as i8,
        3 as libc::c_int as i8,
    ],
    [
        3 as libc::c_int as i8,
        -(8 as libc::c_int) as i8,
        95 as libc::c_int as i8,
        28 as libc::c_int as i8,
        4 as libc::c_int as i8,
    ],
    [
        -(10 as libc::c_int) as i8,
        15 as libc::c_int as i8,
        77 as libc::c_int as i8,
        60 as libc::c_int as i8,
        -(15 as libc::c_int) as i8,
    ],
    [
        -(1 as libc::c_int) as i8,
        4 as libc::c_int as i8,
        124 as libc::c_int as i8,
        2 as libc::c_int as i8,
        -(4 as libc::c_int) as i8,
    ],
    [
        3 as libc::c_int as i8,
        38 as libc::c_int as i8,
        84 as libc::c_int as i8,
        24 as libc::c_int as i8,
        -(25 as libc::c_int) as i8,
    ],
    [
        2 as libc::c_int as i8,
        13 as libc::c_int as i8,
        42 as libc::c_int as i8,
        13 as libc::c_int as i8,
        31 as libc::c_int as i8,
    ],
    [
        21 as libc::c_int as i8,
        -(4 as libc::c_int) as i8,
        56 as libc::c_int as i8,
        46 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
    ],
    [
        -(1 as libc::c_int) as i8,
        35 as libc::c_int as i8,
        79 as libc::c_int as i8,
        -(13 as libc::c_int) as i8,
        19 as libc::c_int as i8,
    ],
    [
        -(7 as libc::c_int) as i8,
        65 as libc::c_int as i8,
        88 as libc::c_int as i8,
        -(9 as libc::c_int) as i8,
        -(14 as libc::c_int) as i8,
    ],
    [
        20 as libc::c_int as i8,
        4 as libc::c_int as i8,
        81 as libc::c_int as i8,
        49 as libc::c_int as i8,
        -(29 as libc::c_int) as i8,
    ],
    [
        20 as libc::c_int as i8,
        0 as libc::c_int as i8,
        75 as libc::c_int as i8,
        3 as libc::c_int as i8,
        -(17 as libc::c_int) as i8,
    ],
    [
        5 as libc::c_int as i8,
        -(9 as libc::c_int) as i8,
        44 as libc::c_int as i8,
        92 as libc::c_int as i8,
        -(8 as libc::c_int) as i8,
    ],
    [
        1 as libc::c_int as i8,
        -(3 as libc::c_int) as i8,
        22 as libc::c_int as i8,
        69 as libc::c_int as i8,
        31 as libc::c_int as i8,
    ],
    [
        -(6 as libc::c_int) as i8,
        95 as libc::c_int as i8,
        41 as libc::c_int as i8,
        -(12 as libc::c_int) as i8,
        5 as libc::c_int as i8,
    ],
    [
        39 as libc::c_int as i8,
        67 as libc::c_int as i8,
        16 as libc::c_int as i8,
        -(4 as libc::c_int) as i8,
        1 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        -(6 as libc::c_int) as i8,
        120 as libc::c_int as i8,
        55 as libc::c_int as i8,
        -(36 as libc::c_int) as i8,
    ],
    [
        -(13 as libc::c_int) as i8,
        44 as libc::c_int as i8,
        122 as libc::c_int as i8,
        4 as libc::c_int as i8,
        -(24 as libc::c_int) as i8,
    ],
    [
        81 as libc::c_int as i8,
        5 as libc::c_int as i8,
        11 as libc::c_int as i8,
        3 as libc::c_int as i8,
        7 as libc::c_int as i8,
    ],
    [
        2 as libc::c_int as i8,
        0 as libc::c_int as i8,
        9 as libc::c_int as i8,
        10 as libc::c_int as i8,
        88 as libc::c_int as i8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "262:25"]
pub static mut silk_LTP_vq_ptrs_Q7: [*const i8; 3] = [0 as *const i8; 3];
#[c2rust::src_loc = "270:25"]
static mut silk_LTP_gain_vq_0_gain: [u8; 8] = [
    46 as libc::c_int as u8,
    2 as libc::c_int as u8,
    90 as libc::c_int as u8,
    87 as libc::c_int as u8,
    93 as libc::c_int as u8,
    91 as libc::c_int as u8,
    82 as libc::c_int as u8,
    98 as libc::c_int as u8,
];
#[c2rust::src_loc = "274:25"]
static mut silk_LTP_gain_vq_1_gain: [u8; 16] = [
    109 as libc::c_int as u8,
    120 as libc::c_int as u8,
    118 as libc::c_int as u8,
    12 as libc::c_int as u8,
    113 as libc::c_int as u8,
    115 as libc::c_int as u8,
    117 as libc::c_int as u8,
    119 as libc::c_int as u8,
    99 as libc::c_int as u8,
    59 as libc::c_int as u8,
    87 as libc::c_int as u8,
    111 as libc::c_int as u8,
    63 as libc::c_int as u8,
    111 as libc::c_int as u8,
    112 as libc::c_int as u8,
    80 as libc::c_int as u8,
];
#[c2rust::src_loc = "279:25"]
static mut silk_LTP_gain_vq_2_gain: [u8; 32] = [
    126 as libc::c_int as u8,
    124 as libc::c_int as u8,
    125 as libc::c_int as u8,
    124 as libc::c_int as u8,
    129 as libc::c_int as u8,
    121 as libc::c_int as u8,
    126 as libc::c_int as u8,
    23 as libc::c_int as u8,
    132 as libc::c_int as u8,
    127 as libc::c_int as u8,
    127 as libc::c_int as u8,
    127 as libc::c_int as u8,
    126 as libc::c_int as u8,
    127 as libc::c_int as u8,
    122 as libc::c_int as u8,
    133 as libc::c_int as u8,
    130 as libc::c_int as u8,
    134 as libc::c_int as u8,
    101 as libc::c_int as u8,
    118 as libc::c_int as u8,
    119 as libc::c_int as u8,
    145 as libc::c_int as u8,
    126 as libc::c_int as u8,
    86 as libc::c_int as u8,
    124 as libc::c_int as u8,
    120 as libc::c_int as u8,
    123 as libc::c_int as u8,
    119 as libc::c_int as u8,
    170 as libc::c_int as u8,
    173 as libc::c_int as u8,
    107 as libc::c_int as u8,
    109 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "286:26"]
pub static mut silk_LTP_vq_gain_ptrs_Q7: [*const u8; 3] = [0 as *const u8; 3];
#[no_mangle]
#[c2rust::src_loc = "292:17"]
pub static mut silk_LTP_vq_sizes: [i8; 3] = [
    8 as libc::c_int as i8,
    16 as libc::c_int as i8,
    32 as libc::c_int as i8,
];
unsafe extern "C" fn run_static_initializers() {
    silk_LTP_vq_ptrs_Q7 = [
        &*(*silk_LTP_gain_vq_0
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8 as *mut i8 as *const i8,
        &*(*silk_LTP_gain_vq_1
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8 as *mut i8 as *const i8,
        &*(*silk_LTP_gain_vq_2
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8 as *mut i8 as *const i8,
    ];
    silk_LTP_vq_gain_ptrs_Q7 = [
        &*silk_LTP_gain_vq_0_gain
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const u8,
        &*silk_LTP_gain_vq_1_gain
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const u8,
        &*silk_LTP_gain_vq_2_gain
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const u8,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
