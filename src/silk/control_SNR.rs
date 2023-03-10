pub mod errors_h {
    pub const SILK_NO_ERROR: i32 = 0 as i32;
}
pub use self::errors_h::SILK_NO_ERROR;
use crate::silk::structs::silk_encoder_state;

static mut silk_TargetRate_NB_21: [u8; 107] = [
    0 as i32 as u8,
    15 as i32 as u8,
    39 as i32 as u8,
    52 as i32 as u8,
    61 as i32 as u8,
    68 as i32 as u8,
    74 as i32 as u8,
    79 as i32 as u8,
    84 as i32 as u8,
    88 as i32 as u8,
    92 as i32 as u8,
    95 as i32 as u8,
    99 as i32 as u8,
    102 as i32 as u8,
    105 as i32 as u8,
    108 as i32 as u8,
    111 as i32 as u8,
    114 as i32 as u8,
    117 as i32 as u8,
    119 as i32 as u8,
    122 as i32 as u8,
    124 as i32 as u8,
    126 as i32 as u8,
    129 as i32 as u8,
    131 as i32 as u8,
    133 as i32 as u8,
    135 as i32 as u8,
    137 as i32 as u8,
    139 as i32 as u8,
    142 as i32 as u8,
    143 as i32 as u8,
    145 as i32 as u8,
    147 as i32 as u8,
    149 as i32 as u8,
    151 as i32 as u8,
    153 as i32 as u8,
    155 as i32 as u8,
    157 as i32 as u8,
    158 as i32 as u8,
    160 as i32 as u8,
    162 as i32 as u8,
    163 as i32 as u8,
    165 as i32 as u8,
    167 as i32 as u8,
    168 as i32 as u8,
    170 as i32 as u8,
    171 as i32 as u8,
    173 as i32 as u8,
    174 as i32 as u8,
    176 as i32 as u8,
    177 as i32 as u8,
    179 as i32 as u8,
    180 as i32 as u8,
    182 as i32 as u8,
    183 as i32 as u8,
    185 as i32 as u8,
    186 as i32 as u8,
    187 as i32 as u8,
    189 as i32 as u8,
    190 as i32 as u8,
    192 as i32 as u8,
    193 as i32 as u8,
    194 as i32 as u8,
    196 as i32 as u8,
    197 as i32 as u8,
    199 as i32 as u8,
    200 as i32 as u8,
    201 as i32 as u8,
    203 as i32 as u8,
    204 as i32 as u8,
    205 as i32 as u8,
    207 as i32 as u8,
    208 as i32 as u8,
    209 as i32 as u8,
    211 as i32 as u8,
    212 as i32 as u8,
    213 as i32 as u8,
    215 as i32 as u8,
    216 as i32 as u8,
    217 as i32 as u8,
    219 as i32 as u8,
    220 as i32 as u8,
    221 as i32 as u8,
    223 as i32 as u8,
    224 as i32 as u8,
    225 as i32 as u8,
    227 as i32 as u8,
    228 as i32 as u8,
    230 as i32 as u8,
    231 as i32 as u8,
    232 as i32 as u8,
    234 as i32 as u8,
    235 as i32 as u8,
    236 as i32 as u8,
    238 as i32 as u8,
    239 as i32 as u8,
    241 as i32 as u8,
    242 as i32 as u8,
    243 as i32 as u8,
    245 as i32 as u8,
    246 as i32 as u8,
    248 as i32 as u8,
    249 as i32 as u8,
    250 as i32 as u8,
    252 as i32 as u8,
    253 as i32 as u8,
    255 as i32 as u8,
];
static mut silk_TargetRate_MB_21: [u8; 155] = [
    0 as i32 as u8,
    0 as i32 as u8,
    28 as i32 as u8,
    43 as i32 as u8,
    52 as i32 as u8,
    59 as i32 as u8,
    65 as i32 as u8,
    70 as i32 as u8,
    74 as i32 as u8,
    78 as i32 as u8,
    81 as i32 as u8,
    85 as i32 as u8,
    87 as i32 as u8,
    90 as i32 as u8,
    93 as i32 as u8,
    95 as i32 as u8,
    98 as i32 as u8,
    100 as i32 as u8,
    102 as i32 as u8,
    105 as i32 as u8,
    107 as i32 as u8,
    109 as i32 as u8,
    111 as i32 as u8,
    113 as i32 as u8,
    115 as i32 as u8,
    116 as i32 as u8,
    118 as i32 as u8,
    120 as i32 as u8,
    122 as i32 as u8,
    123 as i32 as u8,
    125 as i32 as u8,
    127 as i32 as u8,
    128 as i32 as u8,
    130 as i32 as u8,
    131 as i32 as u8,
    133 as i32 as u8,
    134 as i32 as u8,
    136 as i32 as u8,
    137 as i32 as u8,
    138 as i32 as u8,
    140 as i32 as u8,
    141 as i32 as u8,
    143 as i32 as u8,
    144 as i32 as u8,
    145 as i32 as u8,
    147 as i32 as u8,
    148 as i32 as u8,
    149 as i32 as u8,
    151 as i32 as u8,
    152 as i32 as u8,
    153 as i32 as u8,
    154 as i32 as u8,
    156 as i32 as u8,
    157 as i32 as u8,
    158 as i32 as u8,
    159 as i32 as u8,
    160 as i32 as u8,
    162 as i32 as u8,
    163 as i32 as u8,
    164 as i32 as u8,
    165 as i32 as u8,
    166 as i32 as u8,
    167 as i32 as u8,
    168 as i32 as u8,
    169 as i32 as u8,
    171 as i32 as u8,
    172 as i32 as u8,
    173 as i32 as u8,
    174 as i32 as u8,
    175 as i32 as u8,
    176 as i32 as u8,
    177 as i32 as u8,
    178 as i32 as u8,
    179 as i32 as u8,
    180 as i32 as u8,
    181 as i32 as u8,
    182 as i32 as u8,
    183 as i32 as u8,
    184 as i32 as u8,
    185 as i32 as u8,
    186 as i32 as u8,
    187 as i32 as u8,
    188 as i32 as u8,
    188 as i32 as u8,
    189 as i32 as u8,
    190 as i32 as u8,
    191 as i32 as u8,
    192 as i32 as u8,
    193 as i32 as u8,
    194 as i32 as u8,
    195 as i32 as u8,
    196 as i32 as u8,
    197 as i32 as u8,
    198 as i32 as u8,
    199 as i32 as u8,
    200 as i32 as u8,
    201 as i32 as u8,
    202 as i32 as u8,
    203 as i32 as u8,
    203 as i32 as u8,
    204 as i32 as u8,
    205 as i32 as u8,
    206 as i32 as u8,
    207 as i32 as u8,
    208 as i32 as u8,
    209 as i32 as u8,
    210 as i32 as u8,
    211 as i32 as u8,
    212 as i32 as u8,
    213 as i32 as u8,
    214 as i32 as u8,
    214 as i32 as u8,
    215 as i32 as u8,
    216 as i32 as u8,
    217 as i32 as u8,
    218 as i32 as u8,
    219 as i32 as u8,
    220 as i32 as u8,
    221 as i32 as u8,
    222 as i32 as u8,
    223 as i32 as u8,
    224 as i32 as u8,
    224 as i32 as u8,
    225 as i32 as u8,
    226 as i32 as u8,
    227 as i32 as u8,
    228 as i32 as u8,
    229 as i32 as u8,
    230 as i32 as u8,
    231 as i32 as u8,
    232 as i32 as u8,
    233 as i32 as u8,
    234 as i32 as u8,
    235 as i32 as u8,
    236 as i32 as u8,
    236 as i32 as u8,
    237 as i32 as u8,
    238 as i32 as u8,
    239 as i32 as u8,
    240 as i32 as u8,
    241 as i32 as u8,
    242 as i32 as u8,
    243 as i32 as u8,
    244 as i32 as u8,
    245 as i32 as u8,
    246 as i32 as u8,
    247 as i32 as u8,
    248 as i32 as u8,
    249 as i32 as u8,
    250 as i32 as u8,
    251 as i32 as u8,
    252 as i32 as u8,
    253 as i32 as u8,
    254 as i32 as u8,
    255 as i32 as u8,
];
static mut silk_TargetRate_WB_21: [u8; 191] = [
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    8 as i32 as u8,
    29 as i32 as u8,
    41 as i32 as u8,
    49 as i32 as u8,
    56 as i32 as u8,
    62 as i32 as u8,
    66 as i32 as u8,
    70 as i32 as u8,
    74 as i32 as u8,
    77 as i32 as u8,
    80 as i32 as u8,
    83 as i32 as u8,
    86 as i32 as u8,
    88 as i32 as u8,
    91 as i32 as u8,
    93 as i32 as u8,
    95 as i32 as u8,
    97 as i32 as u8,
    99 as i32 as u8,
    101 as i32 as u8,
    103 as i32 as u8,
    105 as i32 as u8,
    107 as i32 as u8,
    108 as i32 as u8,
    110 as i32 as u8,
    112 as i32 as u8,
    113 as i32 as u8,
    115 as i32 as u8,
    116 as i32 as u8,
    118 as i32 as u8,
    119 as i32 as u8,
    121 as i32 as u8,
    122 as i32 as u8,
    123 as i32 as u8,
    125 as i32 as u8,
    126 as i32 as u8,
    127 as i32 as u8,
    129 as i32 as u8,
    130 as i32 as u8,
    131 as i32 as u8,
    132 as i32 as u8,
    134 as i32 as u8,
    135 as i32 as u8,
    136 as i32 as u8,
    137 as i32 as u8,
    138 as i32 as u8,
    140 as i32 as u8,
    141 as i32 as u8,
    142 as i32 as u8,
    143 as i32 as u8,
    144 as i32 as u8,
    145 as i32 as u8,
    146 as i32 as u8,
    147 as i32 as u8,
    148 as i32 as u8,
    149 as i32 as u8,
    150 as i32 as u8,
    151 as i32 as u8,
    152 as i32 as u8,
    153 as i32 as u8,
    154 as i32 as u8,
    156 as i32 as u8,
    157 as i32 as u8,
    158 as i32 as u8,
    159 as i32 as u8,
    159 as i32 as u8,
    160 as i32 as u8,
    161 as i32 as u8,
    162 as i32 as u8,
    163 as i32 as u8,
    164 as i32 as u8,
    165 as i32 as u8,
    166 as i32 as u8,
    167 as i32 as u8,
    168 as i32 as u8,
    169 as i32 as u8,
    170 as i32 as u8,
    171 as i32 as u8,
    171 as i32 as u8,
    172 as i32 as u8,
    173 as i32 as u8,
    174 as i32 as u8,
    175 as i32 as u8,
    176 as i32 as u8,
    177 as i32 as u8,
    177 as i32 as u8,
    178 as i32 as u8,
    179 as i32 as u8,
    180 as i32 as u8,
    181 as i32 as u8,
    181 as i32 as u8,
    182 as i32 as u8,
    183 as i32 as u8,
    184 as i32 as u8,
    185 as i32 as u8,
    185 as i32 as u8,
    186 as i32 as u8,
    187 as i32 as u8,
    188 as i32 as u8,
    189 as i32 as u8,
    189 as i32 as u8,
    190 as i32 as u8,
    191 as i32 as u8,
    192 as i32 as u8,
    192 as i32 as u8,
    193 as i32 as u8,
    194 as i32 as u8,
    195 as i32 as u8,
    195 as i32 as u8,
    196 as i32 as u8,
    197 as i32 as u8,
    198 as i32 as u8,
    198 as i32 as u8,
    199 as i32 as u8,
    200 as i32 as u8,
    200 as i32 as u8,
    201 as i32 as u8,
    202 as i32 as u8,
    203 as i32 as u8,
    203 as i32 as u8,
    204 as i32 as u8,
    205 as i32 as u8,
    206 as i32 as u8,
    206 as i32 as u8,
    207 as i32 as u8,
    208 as i32 as u8,
    209 as i32 as u8,
    209 as i32 as u8,
    210 as i32 as u8,
    211 as i32 as u8,
    211 as i32 as u8,
    212 as i32 as u8,
    213 as i32 as u8,
    214 as i32 as u8,
    214 as i32 as u8,
    215 as i32 as u8,
    216 as i32 as u8,
    216 as i32 as u8,
    217 as i32 as u8,
    218 as i32 as u8,
    219 as i32 as u8,
    219 as i32 as u8,
    220 as i32 as u8,
    221 as i32 as u8,
    221 as i32 as u8,
    222 as i32 as u8,
    223 as i32 as u8,
    224 as i32 as u8,
    224 as i32 as u8,
    225 as i32 as u8,
    226 as i32 as u8,
    226 as i32 as u8,
    227 as i32 as u8,
    228 as i32 as u8,
    229 as i32 as u8,
    229 as i32 as u8,
    230 as i32 as u8,
    231 as i32 as u8,
    232 as i32 as u8,
    232 as i32 as u8,
    233 as i32 as u8,
    234 as i32 as u8,
    234 as i32 as u8,
    235 as i32 as u8,
    236 as i32 as u8,
    237 as i32 as u8,
    237 as i32 as u8,
    238 as i32 as u8,
    239 as i32 as u8,
    240 as i32 as u8,
    240 as i32 as u8,
    241 as i32 as u8,
    242 as i32 as u8,
    243 as i32 as u8,
    243 as i32 as u8,
    244 as i32 as u8,
    245 as i32 as u8,
    246 as i32 as u8,
    246 as i32 as u8,
    247 as i32 as u8,
    248 as i32 as u8,
    249 as i32 as u8,
    249 as i32 as u8,
    250 as i32 as u8,
    251 as i32 as u8,
    252 as i32 as u8,
    253 as i32 as u8,
    255 as i32 as u8,
];
pub unsafe fn silk_control_SNR(
    mut psEncC: *mut silk_encoder_state,
    mut TargetRate_bps: i32,
) -> i32 {
    let mut id: i32 = 0;
    let mut bound: i32 = 0;
    let mut snr_table: *const u8 = 0 as *const u8;
    (*psEncC).TargetRate_bps = TargetRate_bps;
    if (*psEncC).nb_subfr == 2 as i32 {
        TargetRate_bps -= 2000 as i32 + (*psEncC).fs_kHz / 16 as i32;
    }
    if (*psEncC).fs_kHz == 8 as i32 {
        bound = ::core::mem::size_of::<[u8; 107]>() as u64 as i32;
        snr_table = silk_TargetRate_NB_21.as_ptr();
    } else if (*psEncC).fs_kHz == 12 as i32 {
        bound = ::core::mem::size_of::<[u8; 155]>() as u64 as i32;
        snr_table = silk_TargetRate_MB_21.as_ptr();
    } else {
        bound = ::core::mem::size_of::<[u8; 191]>() as u64 as i32;
        snr_table = silk_TargetRate_WB_21.as_ptr();
    }
    id = (TargetRate_bps + 200 as i32) / 400 as i32;
    id = if (id - 10 as i32) < bound - 1 as i32 {
        id - 10 as i32
    } else {
        bound - 1 as i32
    };
    if id <= 0 as i32 {
        (*psEncC).SNR_dB_Q7 = 0 as i32;
    } else {
        (*psEncC).SNR_dB_Q7 = *snr_table.offset(id as isize) as i32 * 21 as i32;
    }
    return SILK_NO_ERROR;
}
