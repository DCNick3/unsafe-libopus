use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
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
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::stdint_uintn_h::uint8_t;
}
pub use self::opus_types_h::{opus_int16, opus_int32, opus_uint8};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint8_t;
pub use self::types_h::{__int16_t, __int32_t, __uint8_t};
#[no_mangle]
#[c2rust::src_loc = "42:18"]
pub static mut silk_stereo_pred_quant_Q13: [opus_int16; 16] = [
    -(13732 as libc::c_int) as opus_int16,
    -(10050 as libc::c_int) as opus_int16,
    -(8266 as libc::c_int) as opus_int16,
    -(7526 as libc::c_int) as opus_int16,
    -(6500 as libc::c_int) as opus_int16,
    -(5000 as libc::c_int) as opus_int16,
    -(2950 as libc::c_int) as opus_int16,
    -(820 as libc::c_int) as opus_int16,
    820 as libc::c_int as opus_int16,
    2950 as libc::c_int as opus_int16,
    5000 as libc::c_int as opus_int16,
    6500 as libc::c_int as opus_int16,
    7526 as libc::c_int as opus_int16,
    8266 as libc::c_int as opus_int16,
    10050 as libc::c_int as opus_int16,
    13732 as libc::c_int as opus_int16,
];
#[no_mangle]
#[c2rust::src_loc = "46:19"]
pub static mut silk_stereo_pred_joint_iCDF: [opus_uint8; 25] = [
    249 as libc::c_int as opus_uint8,
    247 as libc::c_int as opus_uint8,
    246 as libc::c_int as opus_uint8,
    245 as libc::c_int as opus_uint8,
    244 as libc::c_int as opus_uint8,
    234 as libc::c_int as opus_uint8,
    210 as libc::c_int as opus_uint8,
    202 as libc::c_int as opus_uint8,
    201 as libc::c_int as opus_uint8,
    200 as libc::c_int as opus_uint8,
    197 as libc::c_int as opus_uint8,
    174 as libc::c_int as opus_uint8,
    82 as libc::c_int as opus_uint8,
    59 as libc::c_int as opus_uint8,
    56 as libc::c_int as opus_uint8,
    55 as libc::c_int as opus_uint8,
    54 as libc::c_int as opus_uint8,
    46 as libc::c_int as opus_uint8,
    22 as libc::c_int as opus_uint8,
    12 as libc::c_int as opus_uint8,
    11 as libc::c_int as opus_uint8,
    10 as libc::c_int as opus_uint8,
    9 as libc::c_int as opus_uint8,
    7 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "53:19"]
pub static mut silk_stereo_only_code_mid_iCDF: [opus_uint8; 2] = [
    64 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[c2rust::src_loc = "56:25"]
static mut silk_LBRR_flags_2_iCDF: [opus_uint8; 3] = [
    203 as libc::c_int as opus_uint8,
    150 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[c2rust::src_loc = "57:25"]
static mut silk_LBRR_flags_3_iCDF: [opus_uint8; 7] = [
    215 as libc::c_int as opus_uint8,
    195 as libc::c_int as opus_uint8,
    166 as libc::c_int as opus_uint8,
    125 as libc::c_int as opus_uint8,
    110 as libc::c_int as opus_uint8,
    82 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "58:26"]
pub static mut silk_LBRR_flags_iCDF_ptr: [*const opus_uint8; 2] = unsafe {
    [
        silk_LBRR_flags_2_iCDF.as_ptr(),
        silk_LBRR_flags_3_iCDF.as_ptr(),
    ]
};
#[no_mangle]
#[c2rust::src_loc = "64:18"]
pub static mut silk_lsb_iCDF: [opus_uint8; 2] = [
    120 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "67:18"]
pub static mut silk_LTPscale_iCDF: [opus_uint8; 3] = [
    128 as libc::c_int as opus_uint8,
    64 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "70:18"]
pub static mut silk_type_offset_VAD_iCDF: [opus_uint8; 4] = [
    232 as libc::c_int as opus_uint8,
    158 as libc::c_int as opus_uint8,
    10 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "73:18"]
pub static mut silk_type_offset_no_VAD_iCDF: [opus_uint8; 2] = [
    230 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "78:18"]
pub static mut silk_NLSF_interpolation_factor_iCDF: [opus_uint8; 5] = [
    243 as libc::c_int as opus_uint8,
    221 as libc::c_int as opus_uint8,
    192 as libc::c_int as opus_uint8,
    181 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "81:19"]
pub static mut silk_Quantization_Offsets_Q10: [[opus_int16; 2]; 2] = [
    [
        100 as libc::c_int as opus_int16,
        240 as libc::c_int as opus_int16,
    ],
    [
        32 as libc::c_int as opus_int16,
        100 as libc::c_int as opus_int16,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "86:18"]
pub static mut silk_LTPScales_table_Q14: [opus_int16; 3] = [
    15565 as libc::c_int as opus_int16,
    12288 as libc::c_int as opus_int16,
    8192 as libc::c_int as opus_int16,
];
#[no_mangle]
#[c2rust::src_loc = "89:18"]
pub static mut silk_uniform3_iCDF: [opus_uint8; 3] = [
    171 as libc::c_int as opus_uint8,
    85 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "90:18"]
pub static mut silk_uniform4_iCDF: [opus_uint8; 4] = [
    192 as libc::c_int as opus_uint8,
    128 as libc::c_int as opus_uint8,
    64 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "91:18"]
pub static mut silk_uniform5_iCDF: [opus_uint8; 5] = [
    205 as libc::c_int as opus_uint8,
    154 as libc::c_int as opus_uint8,
    102 as libc::c_int as opus_uint8,
    51 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "92:18"]
pub static mut silk_uniform6_iCDF: [opus_uint8; 6] = [
    213 as libc::c_int as opus_uint8,
    171 as libc::c_int as opus_uint8,
    128 as libc::c_int as opus_uint8,
    85 as libc::c_int as opus_uint8,
    43 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "93:18"]
pub static mut silk_uniform8_iCDF: [opus_uint8; 8] = [
    224 as libc::c_int as opus_uint8,
    192 as libc::c_int as opus_uint8,
    160 as libc::c_int as opus_uint8,
    128 as libc::c_int as opus_uint8,
    96 as libc::c_int as opus_uint8,
    64 as libc::c_int as opus_uint8,
    32 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "95:18"]
pub static mut silk_NLSF_EXT_iCDF: [opus_uint8; 7] = [
    100 as libc::c_int as opus_uint8,
    40 as libc::c_int as opus_uint8,
    16 as libc::c_int as opus_uint8,
    7 as libc::c_int as opus_uint8,
    3 as libc::c_int as opus_uint8,
    1 as libc::c_int as opus_uint8,
    0 as libc::c_int as opus_uint8,
];
#[no_mangle]
#[c2rust::src_loc = "102:18"]
pub static mut silk_Transition_LP_B_Q28: [[opus_int32; 3]; 5] = [
    [
        250767114 as libc::c_int,
        501534038 as libc::c_int,
        250767114 as libc::c_int,
    ],
    [
        209867381 as libc::c_int,
        419732057 as libc::c_int,
        209867381 as libc::c_int,
    ],
    [
        170987846 as libc::c_int,
        341967853 as libc::c_int,
        170987846 as libc::c_int,
    ],
    [
        131531482 as libc::c_int,
        263046905 as libc::c_int,
        131531482 as libc::c_int,
    ],
    [
        89306658 as libc::c_int,
        178584282 as libc::c_int,
        89306658 as libc::c_int,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "112:18"]
pub static mut silk_Transition_LP_A_Q28: [[opus_int32; 2]; 5] = [
    [506393414 as libc::c_int, 239854379 as libc::c_int],
    [411067935 as libc::c_int, 169683996 as libc::c_int],
    [306733530 as libc::c_int, 116694253 as libc::c_int],
    [185807084 as libc::c_int, 77959395 as libc::c_int],
    [35497197 as libc::c_int, 57401098 as libc::c_int],
];
