use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "132:9"]
    pub const OFFSET_UVL_Q10: libc::c_int = 100 as libc::c_int;
    #[c2rust::src_loc = "133:9"]
    pub const OFFSET_UVH_Q10: libc::c_int = 240 as libc::c_int;
    #[c2rust::src_loc = "130:9"]
    pub const OFFSET_VL_Q10: libc::c_int = 32 as libc::c_int;
    #[c2rust::src_loc = "131:9"]
    pub const OFFSET_VH_Q10: libc::c_int = 100 as libc::c_int;
}
pub use self::define_h::{OFFSET_UVH_Q10, OFFSET_UVL_Q10, OFFSET_VH_Q10, OFFSET_VL_Q10};

#[no_mangle]
#[c2rust::src_loc = "42:18"]
pub static mut silk_stereo_pred_quant_Q13: [i16; 16] = [
    -(13732 as libc::c_int) as i16,
    -(10050 as libc::c_int) as i16,
    -(8266 as libc::c_int) as i16,
    -(7526 as libc::c_int) as i16,
    -(6500 as libc::c_int) as i16,
    -(5000 as libc::c_int) as i16,
    -(2950 as libc::c_int) as i16,
    -(820 as libc::c_int) as i16,
    820 as libc::c_int as i16,
    2950 as libc::c_int as i16,
    5000 as libc::c_int as i16,
    6500 as libc::c_int as i16,
    7526 as libc::c_int as i16,
    8266 as libc::c_int as i16,
    10050 as libc::c_int as i16,
    13732 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "46:19"]
pub static mut silk_stereo_pred_joint_iCDF: [u8; 25] = [
    249 as libc::c_int as u8,
    247 as libc::c_int as u8,
    246 as libc::c_int as u8,
    245 as libc::c_int as u8,
    244 as libc::c_int as u8,
    234 as libc::c_int as u8,
    210 as libc::c_int as u8,
    202 as libc::c_int as u8,
    201 as libc::c_int as u8,
    200 as libc::c_int as u8,
    197 as libc::c_int as u8,
    174 as libc::c_int as u8,
    82 as libc::c_int as u8,
    59 as libc::c_int as u8,
    56 as libc::c_int as u8,
    55 as libc::c_int as u8,
    54 as libc::c_int as u8,
    46 as libc::c_int as u8,
    22 as libc::c_int as u8,
    12 as libc::c_int as u8,
    11 as libc::c_int as u8,
    10 as libc::c_int as u8,
    9 as libc::c_int as u8,
    7 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "53:19"]
pub static mut silk_stereo_only_code_mid_iCDF: [u8; 2] =
    [64 as libc::c_int as u8, 0 as libc::c_int as u8];
#[c2rust::src_loc = "56:25"]
static mut silk_LBRR_flags_2_iCDF: [u8; 3] = [
    203 as libc::c_int as u8,
    150 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[c2rust::src_loc = "57:25"]
static mut silk_LBRR_flags_3_iCDF: [u8; 7] = [
    215 as libc::c_int as u8,
    195 as libc::c_int as u8,
    166 as libc::c_int as u8,
    125 as libc::c_int as u8,
    110 as libc::c_int as u8,
    82 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "58:26"]
pub static mut silk_LBRR_flags_iCDF_ptr: [*const u8; 2] = unsafe {
    [
        silk_LBRR_flags_2_iCDF.as_ptr(),
        silk_LBRR_flags_3_iCDF.as_ptr(),
    ]
};
#[no_mangle]
#[c2rust::src_loc = "64:18"]
pub static mut silk_lsb_iCDF: [u8; 2] = [120 as libc::c_int as u8, 0 as libc::c_int as u8];
#[no_mangle]
#[c2rust::src_loc = "67:18"]
pub static mut silk_LTPscale_iCDF: [u8; 3] = [
    128 as libc::c_int as u8,
    64 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "70:18"]
pub static mut silk_type_offset_VAD_iCDF: [u8; 4] = [
    232 as libc::c_int as u8,
    158 as libc::c_int as u8,
    10 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "73:18"]
pub static mut silk_type_offset_no_VAD_iCDF: [u8; 2] =
    [230 as libc::c_int as u8, 0 as libc::c_int as u8];
#[no_mangle]
#[c2rust::src_loc = "78:18"]
pub static mut silk_NLSF_interpolation_factor_iCDF: [u8; 5] = [
    243 as libc::c_int as u8,
    221 as libc::c_int as u8,
    192 as libc::c_int as u8,
    181 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "81:19"]
pub static mut silk_Quantization_Offsets_Q10: [[i16; 2]; 2] = [
    [OFFSET_UVL_Q10 as i16, OFFSET_UVH_Q10 as i16],
    [OFFSET_VL_Q10 as i16, OFFSET_VH_Q10 as i16],
];
#[no_mangle]
#[c2rust::src_loc = "86:18"]
pub static mut silk_LTPScales_table_Q14: [i16; 3] = [
    15565 as libc::c_int as i16,
    12288 as libc::c_int as i16,
    8192 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "89:18"]
pub static mut silk_uniform3_iCDF: [u8; 3] = [
    171 as libc::c_int as u8,
    85 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "90:18"]
pub static mut silk_uniform4_iCDF: [u8; 4] = [
    192 as libc::c_int as u8,
    128 as libc::c_int as u8,
    64 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "91:18"]
pub static mut silk_uniform5_iCDF: [u8; 5] = [
    205 as libc::c_int as u8,
    154 as libc::c_int as u8,
    102 as libc::c_int as u8,
    51 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "92:18"]
pub static mut silk_uniform6_iCDF: [u8; 6] = [
    213 as libc::c_int as u8,
    171 as libc::c_int as u8,
    128 as libc::c_int as u8,
    85 as libc::c_int as u8,
    43 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "93:18"]
pub static mut silk_uniform8_iCDF: [u8; 8] = [
    224 as libc::c_int as u8,
    192 as libc::c_int as u8,
    160 as libc::c_int as u8,
    128 as libc::c_int as u8,
    96 as libc::c_int as u8,
    64 as libc::c_int as u8,
    32 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "95:18"]
pub static mut silk_NLSF_EXT_iCDF: [u8; 7] = [
    100 as libc::c_int as u8,
    40 as libc::c_int as u8,
    16 as libc::c_int as u8,
    7 as libc::c_int as u8,
    3 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[no_mangle]
#[c2rust::src_loc = "102:18"]
pub static mut silk_Transition_LP_B_Q28: [[i32; 3]; 5] = [
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
pub static mut silk_Transition_LP_A_Q28: [[i32; 2]; 5] = [
    [506393414 as libc::c_int, 239854379 as libc::c_int],
    [411067935 as libc::c_int, 169683996 as libc::c_int],
    [306733530 as libc::c_int, 116694253 as libc::c_int],
    [185807084 as libc::c_int, 77959395 as libc::c_int],
    [35497197 as libc::c_int, 57401098 as libc::c_int],
];
