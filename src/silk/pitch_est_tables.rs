use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    use super::types_h::__int8_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    use super::stdint_intn_h::int8_t;
}
pub use self::opus_types_h::opus_int8;
pub use self::stdint_intn_h::int8_t;
pub use self::types_h::__int8_t;
#[no_mangle]
#[c2rust::src_loc = "35:17"]
pub static mut silk_CB_lags_stage2_10_ms: [[opus_int8; 3]; 2] = [
    [
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
    ],
    [
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "41:17"]
pub static mut silk_CB_lags_stage3_10_ms: [[opus_int8; 12]; 2] = [
    [
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        3 as libc::c_int as opus_int8,
        -(3 as libc::c_int) as opus_int8,
    ],
    [
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        3 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        3 as libc::c_int as opus_int8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "47:17"]
pub static mut silk_Lag_range_stage3_10_ms: [[opus_int8; 2]; 2] = [
    [
        -(3 as libc::c_int) as opus_int8,
        7 as libc::c_int as opus_int8,
    ],
    [
        -(2 as libc::c_int) as opus_int8,
        7 as libc::c_int as opus_int8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "53:17"]
pub static mut silk_CB_lags_stage2: [[opus_int8; 11]; 4] = [
    [
        0 as libc::c_int as opus_int8,
        2 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
    ],
    [
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
    ],
    [
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
    ],
    [
        0 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        -(1 as libc::c_int) as opus_int8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "61:17"]
pub static mut silk_CB_lags_stage3: [[opus_int8; 34]; 4] = [
    [
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        0 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        1 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(3 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        3 as libc::c_int as opus_int8,
        -(3 as libc::c_int) as opus_int8,
        -(4 as libc::c_int) as opus_int8,
        3 as libc::c_int as opus_int8,
        -(4 as libc::c_int) as opus_int8,
        4 as libc::c_int as opus_int8,
        4 as libc::c_int as opus_int8,
        -(5 as libc::c_int) as opus_int8,
        5 as libc::c_int as opus_int8,
        -(6 as libc::c_int) as opus_int8,
        -(5 as libc::c_int) as opus_int8,
        6 as libc::c_int as opus_int8,
        -(7 as libc::c_int) as opus_int8,
        6 as libc::c_int as opus_int8,
        5 as libc::c_int as opus_int8,
        8 as libc::c_int as opus_int8,
        -(9 as libc::c_int) as opus_int8,
    ],
    [
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        2 as libc::c_int as opus_int8,
        3 as libc::c_int as opus_int8,
        -(3 as libc::c_int) as opus_int8,
    ],
    [
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        2 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        2 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        3 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        3 as libc::c_int as opus_int8,
    ],
    [
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        0 as libc::c_int as opus_int8,
        1 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        -(1 as libc::c_int) as opus_int8,
        2 as libc::c_int as opus_int8,
        3 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        3 as libc::c_int as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        -(2 as libc::c_int) as opus_int8,
        4 as libc::c_int as opus_int8,
        4 as libc::c_int as opus_int8,
        -(3 as libc::c_int) as opus_int8,
        5 as libc::c_int as opus_int8,
        -(3 as libc::c_int) as opus_int8,
        -(4 as libc::c_int) as opus_int8,
        6 as libc::c_int as opus_int8,
        -(4 as libc::c_int) as opus_int8,
        6 as libc::c_int as opus_int8,
        5 as libc::c_int as opus_int8,
        -(5 as libc::c_int) as opus_int8,
        8 as libc::c_int as opus_int8,
        -(6 as libc::c_int) as opus_int8,
        -(5 as libc::c_int) as opus_int8,
        -(7 as libc::c_int) as opus_int8,
        9 as libc::c_int as opus_int8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "69:17"]
pub static mut silk_Lag_range_stage3: [[[opus_int8; 2]; 4]; 3] = [
    [
        [
            -(5 as libc::c_int) as opus_int8,
            8 as libc::c_int as opus_int8,
        ],
        [
            -(1 as libc::c_int) as opus_int8,
            6 as libc::c_int as opus_int8,
        ],
        [
            -(1 as libc::c_int) as opus_int8,
            6 as libc::c_int as opus_int8,
        ],
        [
            -(4 as libc::c_int) as opus_int8,
            10 as libc::c_int as opus_int8,
        ],
    ],
    [
        [
            -(6 as libc::c_int) as opus_int8,
            10 as libc::c_int as opus_int8,
        ],
        [
            -(2 as libc::c_int) as opus_int8,
            6 as libc::c_int as opus_int8,
        ],
        [
            -(1 as libc::c_int) as opus_int8,
            6 as libc::c_int as opus_int8,
        ],
        [
            -(5 as libc::c_int) as opus_int8,
            10 as libc::c_int as opus_int8,
        ],
    ],
    [
        [
            -(9 as libc::c_int) as opus_int8,
            12 as libc::c_int as opus_int8,
        ],
        [
            -(3 as libc::c_int) as opus_int8,
            7 as libc::c_int as opus_int8,
        ],
        [
            -(2 as libc::c_int) as opus_int8,
            7 as libc::c_int as opus_int8,
        ],
        [
            -(7 as libc::c_int) as opus_int8,
            13 as libc::c_int as opus_int8,
        ],
    ],
];
#[no_mangle]
#[c2rust::src_loc = "94:17"]
pub static mut silk_nb_cbk_searchs_stage3: [opus_int8; 3] = [
    16 as libc::c_int as opus_int8,
    24 as libc::c_int as opus_int8,
    34 as libc::c_int as opus_int8,
];
