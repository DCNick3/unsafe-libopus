use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:35"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:35"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    use super::types_h::__int16_t;
}
pub use self::stdint_intn_h::int16_t;
pub use self::types_h::__int16_t;
#[no_mangle]
#[c2rust::src_loc = "43:35"]
pub static mut silk_Resampler_3_4_COEFS: [i16; 29] = [
    -(20694 as libc::c_int) as i16,
    -(13867 as libc::c_int) as i16,
    -(49 as libc::c_int) as i16,
    64 as libc::c_int as i16,
    17 as libc::c_int as i16,
    -(157 as libc::c_int) as i16,
    353 as libc::c_int as i16,
    -(496 as libc::c_int) as i16,
    163 as libc::c_int as i16,
    11047 as libc::c_int as i16,
    22205 as libc::c_int as i16,
    -(39 as libc::c_int) as i16,
    6 as libc::c_int as i16,
    91 as libc::c_int as i16,
    -(170 as libc::c_int) as i16,
    186 as libc::c_int as i16,
    23 as libc::c_int as i16,
    -(896 as libc::c_int) as i16,
    6336 as libc::c_int as i16,
    19928 as libc::c_int as i16,
    -(19 as libc::c_int) as i16,
    -(36 as libc::c_int) as i16,
    102 as libc::c_int as i16,
    -(89 as libc::c_int) as i16,
    -(24 as libc::c_int) as i16,
    328 as libc::c_int as i16,
    -(951 as libc::c_int) as i16,
    2568 as libc::c_int as i16,
    15909 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "50:35"]
pub static mut silk_Resampler_2_3_COEFS: [i16; 20] = [
    -(14457 as libc::c_int) as i16,
    -(14019 as libc::c_int) as i16,
    64 as libc::c_int as i16,
    128 as libc::c_int as i16,
    -(122 as libc::c_int) as i16,
    36 as libc::c_int as i16,
    310 as libc::c_int as i16,
    -(768 as libc::c_int) as i16,
    584 as libc::c_int as i16,
    9267 as libc::c_int as i16,
    17733 as libc::c_int as i16,
    12 as libc::c_int as i16,
    128 as libc::c_int as i16,
    18 as libc::c_int as i16,
    -(142 as libc::c_int) as i16,
    288 as libc::c_int as i16,
    -(117 as libc::c_int) as i16,
    -(865 as libc::c_int) as i16,
    4123 as libc::c_int as i16,
    14459 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "56:35"]
pub static mut silk_Resampler_1_2_COEFS: [i16; 14] = [
    616 as libc::c_int as i16,
    -(14323 as libc::c_int) as i16,
    -(10 as libc::c_int) as i16,
    39 as libc::c_int as i16,
    58 as libc::c_int as i16,
    -(46 as libc::c_int) as i16,
    -(84 as libc::c_int) as i16,
    120 as libc::c_int as i16,
    184 as libc::c_int as i16,
    -(315 as libc::c_int) as i16,
    -(541 as libc::c_int) as i16,
    1284 as libc::c_int as i16,
    5380 as libc::c_int as i16,
    9024 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "61:35"]
pub static mut silk_Resampler_1_3_COEFS: [i16; 20] = [
    16102 as libc::c_int as i16,
    -(15162 as libc::c_int) as i16,
    -(13 as libc::c_int) as i16,
    0 as libc::c_int as i16,
    20 as libc::c_int as i16,
    26 as libc::c_int as i16,
    5 as libc::c_int as i16,
    -(31 as libc::c_int) as i16,
    -(43 as libc::c_int) as i16,
    -(4 as libc::c_int) as i16,
    65 as libc::c_int as i16,
    90 as libc::c_int as i16,
    7 as libc::c_int as i16,
    -(157 as libc::c_int) as i16,
    -(248 as libc::c_int) as i16,
    -(44 as libc::c_int) as i16,
    593 as libc::c_int as i16,
    1583 as libc::c_int as i16,
    2612 as libc::c_int as i16,
    3271 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "66:35"]
pub static mut silk_Resampler_1_4_COEFS: [i16; 20] = [
    22500 as libc::c_int as i16,
    -(15099 as libc::c_int) as i16,
    3 as libc::c_int as i16,
    -(14 as libc::c_int) as i16,
    -(20 as libc::c_int) as i16,
    -(15 as libc::c_int) as i16,
    2 as libc::c_int as i16,
    25 as libc::c_int as i16,
    37 as libc::c_int as i16,
    25 as libc::c_int as i16,
    -(16 as libc::c_int) as i16,
    -(71 as libc::c_int) as i16,
    -(107 as libc::c_int) as i16,
    -(79 as libc::c_int) as i16,
    50 as libc::c_int as i16,
    292 as libc::c_int as i16,
    623 as libc::c_int as i16,
    982 as libc::c_int as i16,
    1288 as libc::c_int as i16,
    1464 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "71:35"]
pub static mut silk_Resampler_1_6_COEFS: [i16; 20] = [
    27540 as libc::c_int as i16,
    -(15257 as libc::c_int) as i16,
    17 as libc::c_int as i16,
    12 as libc::c_int as i16,
    8 as libc::c_int as i16,
    1 as libc::c_int as i16,
    -(10 as libc::c_int) as i16,
    -(22 as libc::c_int) as i16,
    -(30 as libc::c_int) as i16,
    -(32 as libc::c_int) as i16,
    -(22 as libc::c_int) as i16,
    3 as libc::c_int as i16,
    44 as libc::c_int as i16,
    100 as libc::c_int as i16,
    168 as libc::c_int as i16,
    243 as libc::c_int as i16,
    317 as libc::c_int as i16,
    381 as libc::c_int as i16,
    429 as libc::c_int as i16,
    455 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "76:35"]
pub static mut silk_Resampler_2_3_COEFS_LQ: [i16; 6] = [
    -(2797 as libc::c_int) as i16,
    -(6507 as libc::c_int) as i16,
    4697 as libc::c_int as i16,
    10739 as libc::c_int as i16,
    1567 as libc::c_int as i16,
    8276 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "83:35"]
pub static mut silk_resampler_frac_FIR_12: [[i16; 4]; 12] = [
    [
        189 as libc::c_int as i16,
        -(600 as libc::c_int) as i16,
        617 as libc::c_int as i16,
        30567 as libc::c_int as i16,
    ],
    [
        117 as libc::c_int as i16,
        -(159 as libc::c_int) as i16,
        -(1070 as libc::c_int) as i16,
        29704 as libc::c_int as i16,
    ],
    [
        52 as libc::c_int as i16,
        221 as libc::c_int as i16,
        -(2392 as libc::c_int) as i16,
        28276 as libc::c_int as i16,
    ],
    [
        -(4 as libc::c_int) as i16,
        529 as libc::c_int as i16,
        -(3350 as libc::c_int) as i16,
        26341 as libc::c_int as i16,
    ],
    [
        -(48 as libc::c_int) as i16,
        758 as libc::c_int as i16,
        -(3956 as libc::c_int) as i16,
        23973 as libc::c_int as i16,
    ],
    [
        -(80 as libc::c_int) as i16,
        905 as libc::c_int as i16,
        -(4235 as libc::c_int) as i16,
        21254 as libc::c_int as i16,
    ],
    [
        -(99 as libc::c_int) as i16,
        972 as libc::c_int as i16,
        -(4222 as libc::c_int) as i16,
        18278 as libc::c_int as i16,
    ],
    [
        -(107 as libc::c_int) as i16,
        967 as libc::c_int as i16,
        -(3957 as libc::c_int) as i16,
        15143 as libc::c_int as i16,
    ],
    [
        -(103 as libc::c_int) as i16,
        896 as libc::c_int as i16,
        -(3487 as libc::c_int) as i16,
        11950 as libc::c_int as i16,
    ],
    [
        -(91 as libc::c_int) as i16,
        773 as libc::c_int as i16,
        -(2865 as libc::c_int) as i16,
        8798 as libc::c_int as i16,
    ],
    [
        -(71 as libc::c_int) as i16,
        611 as libc::c_int as i16,
        -(2143 as libc::c_int) as i16,
        5784 as libc::c_int as i16,
    ],
    [
        -(46 as libc::c_int) as i16,
        425 as libc::c_int as i16,
        -(1375 as libc::c_int) as i16,
        2996 as libc::c_int as i16,
    ],
];
