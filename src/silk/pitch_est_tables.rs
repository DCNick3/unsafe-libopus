use ::libc;

pub const PE_NB_CBKS_STAGE3_MIN: libc::c_int = 16;
pub const PE_NB_CBKS_STAGE3_MID: libc::c_int = 24;
pub const PE_NB_CBKS_STAGE3_MAX: libc::c_int = 34;
pub const PE_NB_CBKS_STAGE3_10MS: libc::c_int = 12;
pub const PE_NB_STAGE3_LAGS: libc::c_int = 5 as libc::c_int;
pub const PE_NB_CBKS_STAGE2_EXT: libc::c_int = 11;
pub const PE_NB_CBKS_STAGE2_10MS: libc::c_int = 3;
pub const PE_MAX_NB_SUBFR: libc::c_int = 4;
pub const PE_SUBFR_LENGTH_MS: libc::c_int = 5 as libc::c_int;
pub const PE_LTP_MEM_LENGTH_MS: libc::c_int = 4 as libc::c_int * PE_SUBFR_LENGTH_MS;
pub const PE_MAX_LAG_MS: libc::c_int = 18 as libc::c_int;
pub const PE_MIN_LAG_MS: libc::c_int = 2 as libc::c_int;
pub const PE_NB_CBKS_STAGE2: libc::c_int = 3 as libc::c_int;

pub const SILK_PE_MIN_COMPLEX: libc::c_int = 0 as libc::c_int;
pub const SILK_PE_MID_COMPLEX: libc::c_int = 1 as libc::c_int;
pub const SILK_PE_MAX_COMPLEX: libc::c_int = 2 as libc::c_int;

pub const PE_SHORTLAG_BIAS: libc::c_float = 0.2f32;
pub const PE_PREVLAG_BIAS: libc::c_float = 0.2f32;
pub const PE_FLATCONTOUR_BIAS: libc::c_float = 0.05f32;

#[no_mangle]
#[c2rust::src_loc = "35:17"]
pub static silk_CB_lags_stage2_10_ms: [[i8; 3]; 2] = [[0, 1, 0], [0, 0, 1]];
#[no_mangle]
#[c2rust::src_loc = "41:17"]
pub static silk_CB_lags_stage3_10_ms: [[i8; 12]; 2] = [
    [0, 0, 1, -1, 1, -1, 2, -2, 2, -2, 3, -3],
    [0, 1, 0, 1, -1, 2, -1, 2, -2, 3, -2, 3],
];
#[no_mangle]
#[c2rust::src_loc = "47:17"]
pub static silk_Lag_range_stage3_10_ms: [[i8; 2]; 2] = [[-3, 7], [-2, 7]];
#[no_mangle]
#[c2rust::src_loc = "53:17"]
pub static silk_CB_lags_stage2: [[i8; 11]; 4] = [
    [0, 2, -1, -1, -1, 0, 0, 1, 1, 0, 1],
    [0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, -1, 2, 1, 0, 1, 1, 0, 0, -1, -1],
];
#[no_mangle]
#[c2rust::src_loc = "61:17"]
pub static silk_CB_lags_stage3: [[i8; 34]; 4] = [
    [
        0, 0, 1, -1, 0, 1, -1, 0, -1, 1, -2, 2, -2, -2, 2, -3, 2, 3, -3, -4, 3, -4, 4, 4, -5, 5,
        -6, -5, 6, -7, 6, 5, 8, -9,
    ],
    [
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 1, -1, 0, 1, -1, -1, 1, -1, 2, 1, -1, 2, -2, -2,
        2, -2, 2, 2, 3, -3,
    ],
    [
        0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, -1, 1, 0, 0, 2, 1, -1, 2, -1, -1, 2, -1, 2, 2,
        -1, 3, -2, -2, -2, 3,
    ],
    [
        0, 1, 0, 0, 1, 0, 1, -1, 2, -1, 2, -1, 2, 3, -2, 3, -2, -2, 4, 4, -3, 5, -3, -4, 6, -4, 6,
        5, -5, 8, -6, -5, -7, 9,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "69:17"]
pub static silk_Lag_range_stage3: [[[i8; 2]; 4]; 3] = [
    [[-5, 8], [-1, 6], [-1, 6], [-4, 10]],
    [[-6, 10], [-2, 6], [-1, 6], [-5, 10]],
    [[-9, 12], [-3, 7], [-2, 7], [-7, 13]],
];
#[no_mangle]
#[c2rust::src_loc = "94:17"]
pub static silk_nb_cbk_searchs_stage3: [i8; 3] = [
    PE_NB_CBKS_STAGE3_MIN as i8,
    PE_NB_CBKS_STAGE3_MID as i8,
    PE_NB_CBKS_STAGE3_MAX as i8,
];
