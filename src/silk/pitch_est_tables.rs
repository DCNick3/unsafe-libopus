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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/pitch_est_defines.h:33"]
pub mod pitch_est_defines_h {
    #[c2rust::src_loc = "63:9"]
    pub const PE_NB_CBKS_STAGE3_MIN: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "62:9"]
    pub const PE_NB_CBKS_STAGE3_MID: libc::c_int = 24 as libc::c_int;
    #[c2rust::src_loc = "61:9"]
    pub const PE_NB_CBKS_STAGE3_MAX: libc::c_int = 34 as libc::c_int;
}
pub use self::pitch_est_defines_h::{
    PE_NB_CBKS_STAGE3_MAX, PE_NB_CBKS_STAGE3_MID, PE_NB_CBKS_STAGE3_MIN,
};
pub use self::stdint_intn_h::int8_t;
pub use self::types_h::__int8_t;
#[no_mangle]
#[c2rust::src_loc = "35:17"]
pub static mut silk_CB_lags_stage2_10_ms: [[i8; 3]; 2] = [
    [
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "41:17"]
pub static mut silk_CB_lags_stage3_10_ms: [[i8; 12]; 2] = [
    [
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        3 as libc::c_int as i8,
        -(3 as libc::c_int) as i8,
    ],
    [
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        3 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        3 as libc::c_int as i8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "47:17"]
pub static mut silk_Lag_range_stage3_10_ms: [[i8; 2]; 2] = [
    [-(3 as libc::c_int) as i8, 7 as libc::c_int as i8],
    [-(2 as libc::c_int) as i8, 7 as libc::c_int as i8],
];
#[no_mangle]
#[c2rust::src_loc = "53:17"]
pub static mut silk_CB_lags_stage2: [[i8; 11]; 4] = [
    [
        0 as libc::c_int as i8,
        2 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        -(1 as libc::c_int) as i8,
        -(1 as libc::c_int) as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        -(1 as libc::c_int) as i8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "61:17"]
pub static mut silk_CB_lags_stage3: [[i8; 34]; 4] = [
    [
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        0 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        1 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        -(2 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(3 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        3 as libc::c_int as i8,
        -(3 as libc::c_int) as i8,
        -(4 as libc::c_int) as i8,
        3 as libc::c_int as i8,
        -(4 as libc::c_int) as i8,
        4 as libc::c_int as i8,
        4 as libc::c_int as i8,
        -(5 as libc::c_int) as i8,
        5 as libc::c_int as i8,
        -(6 as libc::c_int) as i8,
        -(5 as libc::c_int) as i8,
        6 as libc::c_int as i8,
        -(7 as libc::c_int) as i8,
        6 as libc::c_int as i8,
        5 as libc::c_int as i8,
        8 as libc::c_int as i8,
        -(9 as libc::c_int) as i8,
    ],
    [
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        -(1 as libc::c_int) as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        -(2 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        2 as libc::c_int as i8,
        3 as libc::c_int as i8,
        -(3 as libc::c_int) as i8,
    ],
    [
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        2 as libc::c_int as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        2 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        3 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        -(2 as libc::c_int) as i8,
        -(2 as libc::c_int) as i8,
        3 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        -(1 as libc::c_int) as i8,
        2 as libc::c_int as i8,
        3 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        3 as libc::c_int as i8,
        -(2 as libc::c_int) as i8,
        -(2 as libc::c_int) as i8,
        4 as libc::c_int as i8,
        4 as libc::c_int as i8,
        -(3 as libc::c_int) as i8,
        5 as libc::c_int as i8,
        -(3 as libc::c_int) as i8,
        -(4 as libc::c_int) as i8,
        6 as libc::c_int as i8,
        -(4 as libc::c_int) as i8,
        6 as libc::c_int as i8,
        5 as libc::c_int as i8,
        -(5 as libc::c_int) as i8,
        8 as libc::c_int as i8,
        -(6 as libc::c_int) as i8,
        -(5 as libc::c_int) as i8,
        -(7 as libc::c_int) as i8,
        9 as libc::c_int as i8,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "69:17"]
pub static mut silk_Lag_range_stage3: [[[i8; 2]; 4]; 3] = [
    [
        [-(5 as libc::c_int) as i8, 8 as libc::c_int as i8],
        [-(1 as libc::c_int) as i8, 6 as libc::c_int as i8],
        [-(1 as libc::c_int) as i8, 6 as libc::c_int as i8],
        [-(4 as libc::c_int) as i8, 10 as libc::c_int as i8],
    ],
    [
        [-(6 as libc::c_int) as i8, 10 as libc::c_int as i8],
        [-(2 as libc::c_int) as i8, 6 as libc::c_int as i8],
        [-(1 as libc::c_int) as i8, 6 as libc::c_int as i8],
        [-(5 as libc::c_int) as i8, 10 as libc::c_int as i8],
    ],
    [
        [-(9 as libc::c_int) as i8, 12 as libc::c_int as i8],
        [-(3 as libc::c_int) as i8, 7 as libc::c_int as i8],
        [-(2 as libc::c_int) as i8, 7 as libc::c_int as i8],
        [-(7 as libc::c_int) as i8, 13 as libc::c_int as i8],
    ],
];
#[no_mangle]
#[c2rust::src_loc = "94:17"]
pub static mut silk_nb_cbk_searchs_stage3: [i8; 3] = [
    PE_NB_CBKS_STAGE3_MIN as i8,
    PE_NB_CBKS_STAGE3_MID as i8,
    PE_NB_CBKS_STAGE3_MAX as i8,
];
