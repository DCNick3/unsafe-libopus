pub mod arch_h {
    pub type opus_val32 = f32;
}
pub use self::arch_h::opus_val32;
use crate::celt::celt::celt_fatal;
use crate::celt::entdec::{ec_dec, ec_dec_uint};
use crate::celt::entenc::{ec_enc, ec_enc_uint};

static mut CELT_PVQ_U_DATA: [u32; 1272] = [
    1 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    0 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    1 as i32 as u32,
    3 as i32 as u32,
    5 as i32 as u32,
    7 as i32 as u32,
    9 as i32 as u32,
    11 as i32 as u32,
    13 as i32 as u32,
    15 as i32 as u32,
    17 as i32 as u32,
    19 as i32 as u32,
    21 as i32 as u32,
    23 as i32 as u32,
    25 as i32 as u32,
    27 as i32 as u32,
    29 as i32 as u32,
    31 as i32 as u32,
    33 as i32 as u32,
    35 as i32 as u32,
    37 as i32 as u32,
    39 as i32 as u32,
    41 as i32 as u32,
    43 as i32 as u32,
    45 as i32 as u32,
    47 as i32 as u32,
    49 as i32 as u32,
    51 as i32 as u32,
    53 as i32 as u32,
    55 as i32 as u32,
    57 as i32 as u32,
    59 as i32 as u32,
    61 as i32 as u32,
    63 as i32 as u32,
    65 as i32 as u32,
    67 as i32 as u32,
    69 as i32 as u32,
    71 as i32 as u32,
    73 as i32 as u32,
    75 as i32 as u32,
    77 as i32 as u32,
    79 as i32 as u32,
    81 as i32 as u32,
    83 as i32 as u32,
    85 as i32 as u32,
    87 as i32 as u32,
    89 as i32 as u32,
    91 as i32 as u32,
    93 as i32 as u32,
    95 as i32 as u32,
    97 as i32 as u32,
    99 as i32 as u32,
    101 as i32 as u32,
    103 as i32 as u32,
    105 as i32 as u32,
    107 as i32 as u32,
    109 as i32 as u32,
    111 as i32 as u32,
    113 as i32 as u32,
    115 as i32 as u32,
    117 as i32 as u32,
    119 as i32 as u32,
    121 as i32 as u32,
    123 as i32 as u32,
    125 as i32 as u32,
    127 as i32 as u32,
    129 as i32 as u32,
    131 as i32 as u32,
    133 as i32 as u32,
    135 as i32 as u32,
    137 as i32 as u32,
    139 as i32 as u32,
    141 as i32 as u32,
    143 as i32 as u32,
    145 as i32 as u32,
    147 as i32 as u32,
    149 as i32 as u32,
    151 as i32 as u32,
    153 as i32 as u32,
    155 as i32 as u32,
    157 as i32 as u32,
    159 as i32 as u32,
    161 as i32 as u32,
    163 as i32 as u32,
    165 as i32 as u32,
    167 as i32 as u32,
    169 as i32 as u32,
    171 as i32 as u32,
    173 as i32 as u32,
    175 as i32 as u32,
    177 as i32 as u32,
    179 as i32 as u32,
    181 as i32 as u32,
    183 as i32 as u32,
    185 as i32 as u32,
    187 as i32 as u32,
    189 as i32 as u32,
    191 as i32 as u32,
    193 as i32 as u32,
    195 as i32 as u32,
    197 as i32 as u32,
    199 as i32 as u32,
    201 as i32 as u32,
    203 as i32 as u32,
    205 as i32 as u32,
    207 as i32 as u32,
    209 as i32 as u32,
    211 as i32 as u32,
    213 as i32 as u32,
    215 as i32 as u32,
    217 as i32 as u32,
    219 as i32 as u32,
    221 as i32 as u32,
    223 as i32 as u32,
    225 as i32 as u32,
    227 as i32 as u32,
    229 as i32 as u32,
    231 as i32 as u32,
    233 as i32 as u32,
    235 as i32 as u32,
    237 as i32 as u32,
    239 as i32 as u32,
    241 as i32 as u32,
    243 as i32 as u32,
    245 as i32 as u32,
    247 as i32 as u32,
    249 as i32 as u32,
    251 as i32 as u32,
    253 as i32 as u32,
    255 as i32 as u32,
    257 as i32 as u32,
    259 as i32 as u32,
    261 as i32 as u32,
    263 as i32 as u32,
    265 as i32 as u32,
    267 as i32 as u32,
    269 as i32 as u32,
    271 as i32 as u32,
    273 as i32 as u32,
    275 as i32 as u32,
    277 as i32 as u32,
    279 as i32 as u32,
    281 as i32 as u32,
    283 as i32 as u32,
    285 as i32 as u32,
    287 as i32 as u32,
    289 as i32 as u32,
    291 as i32 as u32,
    293 as i32 as u32,
    295 as i32 as u32,
    297 as i32 as u32,
    299 as i32 as u32,
    301 as i32 as u32,
    303 as i32 as u32,
    305 as i32 as u32,
    307 as i32 as u32,
    309 as i32 as u32,
    311 as i32 as u32,
    313 as i32 as u32,
    315 as i32 as u32,
    317 as i32 as u32,
    319 as i32 as u32,
    321 as i32 as u32,
    323 as i32 as u32,
    325 as i32 as u32,
    327 as i32 as u32,
    329 as i32 as u32,
    331 as i32 as u32,
    333 as i32 as u32,
    335 as i32 as u32,
    337 as i32 as u32,
    339 as i32 as u32,
    341 as i32 as u32,
    343 as i32 as u32,
    345 as i32 as u32,
    347 as i32 as u32,
    349 as i32 as u32,
    351 as i32 as u32,
    13 as i32 as u32,
    25 as i32 as u32,
    41 as i32 as u32,
    61 as i32 as u32,
    85 as i32 as u32,
    113 as i32 as u32,
    145 as i32 as u32,
    181 as i32 as u32,
    221 as i32 as u32,
    265 as i32 as u32,
    313 as i32 as u32,
    365 as i32 as u32,
    421 as i32 as u32,
    481 as i32 as u32,
    545 as i32 as u32,
    613 as i32 as u32,
    685 as i32 as u32,
    761 as i32 as u32,
    841 as i32 as u32,
    925 as i32 as u32,
    1013 as i32 as u32,
    1105 as i32 as u32,
    1201 as i32 as u32,
    1301 as i32 as u32,
    1405 as i32 as u32,
    1513 as i32 as u32,
    1625 as i32 as u32,
    1741 as i32 as u32,
    1861 as i32 as u32,
    1985 as i32 as u32,
    2113 as i32 as u32,
    2245 as i32 as u32,
    2381 as i32 as u32,
    2521 as i32 as u32,
    2665 as i32 as u32,
    2813 as i32 as u32,
    2965 as i32 as u32,
    3121 as i32 as u32,
    3281 as i32 as u32,
    3445 as i32 as u32,
    3613 as i32 as u32,
    3785 as i32 as u32,
    3961 as i32 as u32,
    4141 as i32 as u32,
    4325 as i32 as u32,
    4513 as i32 as u32,
    4705 as i32 as u32,
    4901 as i32 as u32,
    5101 as i32 as u32,
    5305 as i32 as u32,
    5513 as i32 as u32,
    5725 as i32 as u32,
    5941 as i32 as u32,
    6161 as i32 as u32,
    6385 as i32 as u32,
    6613 as i32 as u32,
    6845 as i32 as u32,
    7081 as i32 as u32,
    7321 as i32 as u32,
    7565 as i32 as u32,
    7813 as i32 as u32,
    8065 as i32 as u32,
    8321 as i32 as u32,
    8581 as i32 as u32,
    8845 as i32 as u32,
    9113 as i32 as u32,
    9385 as i32 as u32,
    9661 as i32 as u32,
    9941 as i32 as u32,
    10225 as i32 as u32,
    10513 as i32 as u32,
    10805 as i32 as u32,
    11101 as i32 as u32,
    11401 as i32 as u32,
    11705 as i32 as u32,
    12013 as i32 as u32,
    12325 as i32 as u32,
    12641 as i32 as u32,
    12961 as i32 as u32,
    13285 as i32 as u32,
    13613 as i32 as u32,
    13945 as i32 as u32,
    14281 as i32 as u32,
    14621 as i32 as u32,
    14965 as i32 as u32,
    15313 as i32 as u32,
    15665 as i32 as u32,
    16021 as i32 as u32,
    16381 as i32 as u32,
    16745 as i32 as u32,
    17113 as i32 as u32,
    17485 as i32 as u32,
    17861 as i32 as u32,
    18241 as i32 as u32,
    18625 as i32 as u32,
    19013 as i32 as u32,
    19405 as i32 as u32,
    19801 as i32 as u32,
    20201 as i32 as u32,
    20605 as i32 as u32,
    21013 as i32 as u32,
    21425 as i32 as u32,
    21841 as i32 as u32,
    22261 as i32 as u32,
    22685 as i32 as u32,
    23113 as i32 as u32,
    23545 as i32 as u32,
    23981 as i32 as u32,
    24421 as i32 as u32,
    24865 as i32 as u32,
    25313 as i32 as u32,
    25765 as i32 as u32,
    26221 as i32 as u32,
    26681 as i32 as u32,
    27145 as i32 as u32,
    27613 as i32 as u32,
    28085 as i32 as u32,
    28561 as i32 as u32,
    29041 as i32 as u32,
    29525 as i32 as u32,
    30013 as i32 as u32,
    30505 as i32 as u32,
    31001 as i32 as u32,
    31501 as i32 as u32,
    32005 as i32 as u32,
    32513 as i32 as u32,
    33025 as i32 as u32,
    33541 as i32 as u32,
    34061 as i32 as u32,
    34585 as i32 as u32,
    35113 as i32 as u32,
    35645 as i32 as u32,
    36181 as i32 as u32,
    36721 as i32 as u32,
    37265 as i32 as u32,
    37813 as i32 as u32,
    38365 as i32 as u32,
    38921 as i32 as u32,
    39481 as i32 as u32,
    40045 as i32 as u32,
    40613 as i32 as u32,
    41185 as i32 as u32,
    41761 as i32 as u32,
    42341 as i32 as u32,
    42925 as i32 as u32,
    43513 as i32 as u32,
    44105 as i32 as u32,
    44701 as i32 as u32,
    45301 as i32 as u32,
    45905 as i32 as u32,
    46513 as i32 as u32,
    47125 as i32 as u32,
    47741 as i32 as u32,
    48361 as i32 as u32,
    48985 as i32 as u32,
    49613 as i32 as u32,
    50245 as i32 as u32,
    50881 as i32 as u32,
    51521 as i32 as u32,
    52165 as i32 as u32,
    52813 as i32 as u32,
    53465 as i32 as u32,
    54121 as i32 as u32,
    54781 as i32 as u32,
    55445 as i32 as u32,
    56113 as i32 as u32,
    56785 as i32 as u32,
    57461 as i32 as u32,
    58141 as i32 as u32,
    58825 as i32 as u32,
    59513 as i32 as u32,
    60205 as i32 as u32,
    60901 as i32 as u32,
    61601 as i32 as u32,
    63 as i32 as u32,
    129 as i32 as u32,
    231 as i32 as u32,
    377 as i32 as u32,
    575 as i32 as u32,
    833 as i32 as u32,
    1159 as i32 as u32,
    1561 as i32 as u32,
    2047 as i32 as u32,
    2625 as i32 as u32,
    3303 as i32 as u32,
    4089 as i32 as u32,
    4991 as i32 as u32,
    6017 as i32 as u32,
    7175 as i32 as u32,
    8473 as i32 as u32,
    9919 as i32 as u32,
    11521 as i32 as u32,
    13287 as i32 as u32,
    15225 as i32 as u32,
    17343 as i32 as u32,
    19649 as i32 as u32,
    22151 as i32 as u32,
    24857 as i32 as u32,
    27775 as i32 as u32,
    30913 as i32 as u32,
    34279 as i32 as u32,
    37881 as i32 as u32,
    41727 as i32 as u32,
    45825 as i32 as u32,
    50183 as i32 as u32,
    54809 as i32 as u32,
    59711 as i32 as u32,
    64897 as i32 as u32,
    70375 as i32 as u32,
    76153 as i32 as u32,
    82239 as i32 as u32,
    88641 as i32 as u32,
    95367 as i32 as u32,
    102425 as i32 as u32,
    109823 as i32 as u32,
    117569 as i32 as u32,
    125671 as i32 as u32,
    134137 as i32 as u32,
    142975 as i32 as u32,
    152193 as i32 as u32,
    161799 as i32 as u32,
    171801 as i32 as u32,
    182207 as i32 as u32,
    193025 as i32 as u32,
    204263 as i32 as u32,
    215929 as i32 as u32,
    228031 as i32 as u32,
    240577 as i32 as u32,
    253575 as i32 as u32,
    267033 as i32 as u32,
    280959 as i32 as u32,
    295361 as i32 as u32,
    310247 as i32 as u32,
    325625 as i32 as u32,
    341503 as i32 as u32,
    357889 as i32 as u32,
    374791 as i32 as u32,
    392217 as i32 as u32,
    410175 as i32 as u32,
    428673 as i32 as u32,
    447719 as i32 as u32,
    467321 as i32 as u32,
    487487 as i32 as u32,
    508225 as i32 as u32,
    529543 as i32 as u32,
    551449 as i32 as u32,
    573951 as i32 as u32,
    597057 as i32 as u32,
    620775 as i32 as u32,
    645113 as i32 as u32,
    670079 as i32 as u32,
    695681 as i32 as u32,
    721927 as i32 as u32,
    748825 as i32 as u32,
    776383 as i32 as u32,
    804609 as i32 as u32,
    833511 as i32 as u32,
    863097 as i32 as u32,
    893375 as i32 as u32,
    924353 as i32 as u32,
    956039 as i32 as u32,
    988441 as i32 as u32,
    1021567 as i32 as u32,
    1055425 as i32 as u32,
    1090023 as i32 as u32,
    1125369 as i32 as u32,
    1161471 as i32 as u32,
    1198337 as i32 as u32,
    1235975 as i32 as u32,
    1274393 as i32 as u32,
    1313599 as i32 as u32,
    1353601 as i32 as u32,
    1394407 as i32 as u32,
    1436025 as i32 as u32,
    1478463 as i32 as u32,
    1521729 as i32 as u32,
    1565831 as i32 as u32,
    1610777 as i32 as u32,
    1656575 as i32 as u32,
    1703233 as i32 as u32,
    1750759 as i32 as u32,
    1799161 as i32 as u32,
    1848447 as i32 as u32,
    1898625 as i32 as u32,
    1949703 as i32 as u32,
    2001689 as i32 as u32,
    2054591 as i32 as u32,
    2108417 as i32 as u32,
    2163175 as i32 as u32,
    2218873 as i32 as u32,
    2275519 as i32 as u32,
    2333121 as i32 as u32,
    2391687 as i32 as u32,
    2451225 as i32 as u32,
    2511743 as i32 as u32,
    2573249 as i32 as u32,
    2635751 as i32 as u32,
    2699257 as i32 as u32,
    2763775 as i32 as u32,
    2829313 as i32 as u32,
    2895879 as i32 as u32,
    2963481 as i32 as u32,
    3032127 as i32 as u32,
    3101825 as i32 as u32,
    3172583 as i32 as u32,
    3244409 as i32 as u32,
    3317311 as i32 as u32,
    3391297 as i32 as u32,
    3466375 as i32 as u32,
    3542553 as i32 as u32,
    3619839 as i32 as u32,
    3698241 as i32 as u32,
    3777767 as i32 as u32,
    3858425 as i32 as u32,
    3940223 as i32 as u32,
    4023169 as i32 as u32,
    4107271 as i32 as u32,
    4192537 as i32 as u32,
    4278975 as i32 as u32,
    4366593 as i32 as u32,
    4455399 as i32 as u32,
    4545401 as i32 as u32,
    4636607 as i32 as u32,
    4729025 as i32 as u32,
    4822663 as i32 as u32,
    4917529 as i32 as u32,
    5013631 as i32 as u32,
    5110977 as i32 as u32,
    5209575 as i32 as u32,
    5309433 as i32 as u32,
    5410559 as i32 as u32,
    5512961 as i32 as u32,
    5616647 as i32 as u32,
    5721625 as i32 as u32,
    5827903 as i32 as u32,
    5935489 as i32 as u32,
    6044391 as i32 as u32,
    6154617 as i32 as u32,
    6266175 as i32 as u32,
    6379073 as i32 as u32,
    6493319 as i32 as u32,
    6608921 as i32 as u32,
    6725887 as i32 as u32,
    6844225 as i32 as u32,
    6963943 as i32 as u32,
    7085049 as i32 as u32,
    7207551 as i32 as u32,
    321 as i32 as u32,
    681 as i32 as u32,
    1289 as i32 as u32,
    2241 as i32 as u32,
    3649 as i32 as u32,
    5641 as i32 as u32,
    8361 as i32 as u32,
    11969 as i32 as u32,
    16641 as i32 as u32,
    22569 as i32 as u32,
    29961 as i32 as u32,
    39041 as i32 as u32,
    50049 as i32 as u32,
    63241 as i32 as u32,
    78889 as i32 as u32,
    97281 as i32 as u32,
    118721 as i32 as u32,
    143529 as i32 as u32,
    172041 as i32 as u32,
    204609 as i32 as u32,
    241601 as i32 as u32,
    283401 as i32 as u32,
    330409 as i32 as u32,
    383041 as i32 as u32,
    441729 as i32 as u32,
    506921 as i32 as u32,
    579081 as i32 as u32,
    658689 as i32 as u32,
    746241 as i32 as u32,
    842249 as i32 as u32,
    947241 as i32 as u32,
    1061761 as i32 as u32,
    1186369 as i32 as u32,
    1321641 as i32 as u32,
    1468169 as i32 as u32,
    1626561 as i32 as u32,
    1797441 as i32 as u32,
    1981449 as i32 as u32,
    2179241 as i32 as u32,
    2391489 as i32 as u32,
    2618881 as i32 as u32,
    2862121 as i32 as u32,
    3121929 as i32 as u32,
    3399041 as i32 as u32,
    3694209 as i32 as u32,
    4008201 as i32 as u32,
    4341801 as i32 as u32,
    4695809 as i32 as u32,
    5071041 as i32 as u32,
    5468329 as i32 as u32,
    5888521 as i32 as u32,
    6332481 as i32 as u32,
    6801089 as i32 as u32,
    7295241 as i32 as u32,
    7815849 as i32 as u32,
    8363841 as i32 as u32,
    8940161 as i32 as u32,
    9545769 as i32 as u32,
    10181641 as i32 as u32,
    10848769 as i32 as u32,
    11548161 as i32 as u32,
    12280841 as i32 as u32,
    13047849 as i32 as u32,
    13850241 as i32 as u32,
    14689089 as i32 as u32,
    15565481 as i32 as u32,
    16480521 as i32 as u32,
    17435329 as i32 as u32,
    18431041 as i32 as u32,
    19468809 as i32 as u32,
    20549801 as i32 as u32,
    21675201 as i32 as u32,
    22846209 as i32 as u32,
    24064041 as i32 as u32,
    25329929 as i32 as u32,
    26645121 as i32 as u32,
    28010881 as i32 as u32,
    29428489 as i32 as u32,
    30899241 as i32 as u32,
    32424449 as i32 as u32,
    34005441 as i32 as u32,
    35643561 as i32 as u32,
    37340169 as i32 as u32,
    39096641 as i32 as u32,
    40914369 as i32 as u32,
    42794761 as i32 as u32,
    44739241 as i32 as u32,
    46749249 as i32 as u32,
    48826241 as i32 as u32,
    50971689 as i32 as u32,
    53187081 as i32 as u32,
    55473921 as i32 as u32,
    57833729 as i32 as u32,
    60268041 as i32 as u32,
    62778409 as i32 as u32,
    65366401 as i32 as u32,
    68033601 as i32 as u32,
    70781609 as i32 as u32,
    73612041 as i32 as u32,
    76526529 as i32 as u32,
    79526721 as i32 as u32,
    82614281 as i32 as u32,
    85790889 as i32 as u32,
    89058241 as i32 as u32,
    92418049 as i32 as u32,
    95872041 as i32 as u32,
    99421961 as i32 as u32,
    103069569 as i32 as u32,
    106816641 as i32 as u32,
    110664969 as i32 as u32,
    114616361 as i32 as u32,
    118672641 as i32 as u32,
    122835649 as i32 as u32,
    127107241 as i32 as u32,
    131489289 as i32 as u32,
    135983681 as i32 as u32,
    140592321 as i32 as u32,
    145317129 as i32 as u32,
    150160041 as i32 as u32,
    155123009 as i32 as u32,
    160208001 as i32 as u32,
    165417001 as i32 as u32,
    170752009 as i32 as u32,
    176215041 as i32 as u32,
    181808129 as i32 as u32,
    187533321 as i32 as u32,
    193392681 as i32 as u32,
    199388289 as i32 as u32,
    205522241 as i32 as u32,
    211796649 as i32 as u32,
    218213641 as i32 as u32,
    224775361 as i32 as u32,
    231483969 as i32 as u32,
    238341641 as i32 as u32,
    245350569 as i32 as u32,
    252512961 as i32 as u32,
    259831041 as i32 as u32,
    267307049 as i32 as u32,
    274943241 as i32 as u32,
    282741889 as i32 as u32,
    290705281 as i32 as u32,
    298835721 as i32 as u32,
    307135529 as i32 as u32,
    315607041 as i32 as u32,
    324252609 as i32 as u32,
    333074601 as i32 as u32,
    342075401 as i32 as u32,
    351257409 as i32 as u32,
    360623041 as i32 as u32,
    370174729 as i32 as u32,
    379914921 as i32 as u32,
    389846081 as i32 as u32,
    399970689 as i32 as u32,
    410291241 as i32 as u32,
    420810249 as i32 as u32,
    431530241 as i32 as u32,
    442453761 as i32 as u32,
    453583369 as i32 as u32,
    464921641 as i32 as u32,
    476471169 as i32 as u32,
    488234561 as i32 as u32,
    500214441 as i32 as u32,
    512413449 as i32 as u32,
    524834241 as i32 as u32,
    537479489 as i32 as u32,
    550351881 as i32 as u32,
    563454121 as i32 as u32,
    576788929 as i32 as u32,
    590359041 as i32 as u32,
    604167209 as i32 as u32,
    618216201 as i32 as u32,
    632508801 as i32 as u32,
    1683 as i32 as u32,
    3653 as i32 as u32,
    7183 as i32 as u32,
    13073 as i32 as u32,
    22363 as i32 as u32,
    36365 as i32 as u32,
    56695 as i32 as u32,
    85305 as i32 as u32,
    124515 as i32 as u32,
    177045 as i32 as u32,
    246047 as i32 as u32,
    335137 as i32 as u32,
    448427 as i32 as u32,
    590557 as i32 as u32,
    766727 as i32 as u32,
    982729 as i32 as u32,
    1244979 as i32 as u32,
    1560549 as i32 as u32,
    1937199 as i32 as u32,
    2383409 as i32 as u32,
    2908411 as i32 as u32,
    3522221 as i32 as u32,
    4235671 as i32 as u32,
    5060441 as i32 as u32,
    6009091 as i32 as u32,
    7095093 as i32 as u32,
    8332863 as i32 as u32,
    9737793 as i32 as u32,
    11326283 as i32 as u32,
    13115773 as i32 as u32,
    15124775 as i32 as u32,
    17372905 as i32 as u32,
    19880915 as i32 as u32,
    22670725 as i32 as u32,
    25765455 as i32 as u32,
    29189457 as i32 as u32,
    32968347 as i32 as u32,
    37129037 as i32 as u32,
    41699767 as i32 as u32,
    46710137 as i32 as u32,
    52191139 as i32 as u32,
    58175189 as i32 as u32,
    64696159 as i32 as u32,
    71789409 as i32 as u32,
    79491819 as i32 as u32,
    87841821 as i32 as u32,
    96879431 as i32 as u32,
    106646281 as i32 as u32,
    117185651 as i32 as u32,
    128542501 as i32 as u32,
    140763503 as i32 as u32,
    153897073 as i32 as u32,
    167993403 as i32 as u32,
    183104493 as i32 as u32,
    199284183 as i32 as u32,
    216588185 as i32 as u32,
    235074115 as i32 as u32,
    254801525 as i32 as u32,
    275831935 as i32 as u32,
    298228865 as i32 as u32,
    322057867 as i32 as u32,
    347386557 as i32 as u32,
    374284647 as i32 as u32,
    402823977 as i32 as u32,
    433078547 as i32 as u32,
    465124549 as i32 as u32,
    499040399 as i32 as u32,
    534906769 as i32 as u32,
    572806619 as i32 as u32,
    612825229 as i32 as u32,
    655050231 as i32 as u32,
    699571641 as i32 as u32,
    746481891 as i32 as u32,
    795875861 as i32 as u32,
    847850911 as i32 as u32,
    902506913 as i32 as u32,
    959946283 as i32 as u32,
    1020274013 as i32 as u32,
    1083597703 as i32 as u32,
    1150027593 as i32 as u32,
    1219676595 as i32 as u32,
    1292660325 as i32 as u32,
    1369097135 as i32 as u32,
    1449108145 as i32 as u32,
    1532817275 as i32 as u32,
    1620351277 as i32 as u32,
    1711839767 as i32 as u32,
    1807415257 as i32 as u32,
    1907213187 as i32 as u32,
    2011371957 as i32 as u32,
    2120032959 as i32 as u32,
    8989 as i32 as u32,
    19825 as i32 as u32,
    40081 as i32 as u32,
    75517 as i32 as u32,
    134245 as i32 as u32,
    227305 as i32 as u32,
    369305 as i32 as u32,
    579125 as i32 as u32,
    880685 as i32 as u32,
    1303777 as i32 as u32,
    1884961 as i32 as u32,
    2668525 as i32 as u32,
    3707509 as i32 as u32,
    5064793 as i32 as u32,
    6814249 as i32 as u32,
    9041957 as i32 as u32,
    11847485 as i32 as u32,
    15345233 as i32 as u32,
    19665841 as i32 as u32,
    24957661 as i32 as u32,
    31388293 as i32 as u32,
    39146185 as i32 as u32,
    48442297 as i32 as u32,
    59511829 as i32 as u32,
    72616013 as i32 as u32,
    88043969 as i32 as u32,
    106114625 as i32 as u32,
    127178701 as i32 as u32,
    151620757 as i32 as u32,
    179861305 as i32 as u32,
    212358985 as i32 as u32,
    249612805 as i32 as u32,
    292164445 as i32 as u32,
    340600625 as i32 as u32,
    395555537 as i32 as u32,
    457713341 as i32 as u32,
    527810725 as i32 as u32,
    606639529 as i32 as u32,
    695049433 as i32 as u32,
    793950709 as i32 as u32,
    904317037 as i32 as u32,
    1027188385 as i32 as u32,
    1163673953 as i32 as u32,
    1314955181 as i32 as u32,
    1482288821 as i32 as u32,
    1667010073 as i32 as u32,
    1870535785 as i32 as u32,
    2094367717 as i32 as u32,
    48639 as i32 as u32,
    108545 as i32 as u32,
    224143 as i32 as u32,
    433905 as i32 as u32,
    795455 as i32 as u32,
    1392065 as i32 as u32,
    2340495 as i32 as u32,
    3800305 as i32 as u32,
    5984767 as i32 as u32,
    9173505 as i32 as u32,
    13726991 as i32 as u32,
    20103025 as i32 as u32,
    28875327 as i32 as u32,
    40754369 as i32 as u32,
    56610575 as i32 as u32,
    77500017 as i32 as u32,
    104692735 as i32 as u32,
    139703809 as i32 as u32,
    184327311 as i32 as u32,
    240673265 as i32 as u32,
    311207743 as i32 as u32,
    398796225 as i32 as u32,
    506750351 as i32 as u32,
    638878193 as i32 as u32,
    799538175 as i32 as u32,
    993696769 as i32 as u32,
    1226990095 as i32 as u32,
    1505789553 as i32 as u32,
    1837271615 as i32 as u32,
    2229491905 as u32,
    265729 as i32 as u32,
    598417 as i32 as u32,
    1256465 as i32 as u32,
    2485825 as i32 as u32,
    4673345 as i32 as u32,
    8405905 as i32 as u32,
    14546705 as i32 as u32,
    24331777 as i32 as u32,
    39490049 as i32 as u32,
    62390545 as i32 as u32,
    96220561 as i32 as u32,
    145198913 as i32 as u32,
    214828609 as i32 as u32,
    312193553 as i32 as u32,
    446304145 as i32 as u32,
    628496897 as i32 as u32,
    872893441 as i32 as u32,
    1196924561 as i32 as u32,
    1621925137 as i32 as u32,
    2173806145 as u32,
    1462563 as i32 as u32,
    3317445 as i32 as u32,
    7059735 as i32 as u32,
    14218905 as i32 as u32,
    27298155 as i32 as u32,
    50250765 as i32 as u32,
    89129247 as i32 as u32,
    152951073 as i32 as u32,
    254831667 as i32 as u32,
    413442773 as i32 as u32,
    654862247 as i32 as u32,
    1014889769 as i32 as u32,
    1541911931 as i32 as u32,
    2300409629 as u32,
    3375210671 as u32,
    8097453 as i32 as u32,
    18474633 as i32 as u32,
    39753273 as i32 as u32,
    81270333 as i32 as u32,
    158819253 as i32 as u32,
    298199265 as i32 as u32,
    540279585 as i32 as u32,
    948062325 as i32 as u32,
    1616336765 as i32 as u32,
    45046719 as i32 as u32,
    103274625 as i32 as u32,
    224298231 as i32 as u32,
    464387817 as i32 as u32,
    921406335 as i32 as u32,
    1759885185 as i32 as u32,
    3248227095 as u32,
    251595969 as i32 as u32,
    579168825 as i32 as u32,
    1267854873 as i32 as u32,
    2653649025 as u32,
    1409933619 as i32 as u32,
];
static mut CELT_PVQ_U_ROW: [*const u32; 15] = [0 as *const u32; 15];
unsafe fn icwrs(mut _n: i32, mut _y: *const i32) -> u32 {
    let mut i: u32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    if !(_n >= 2 as i32) {
        celt_fatal(
            b"assertion failed: _n>=2\0" as *const u8 as *const i8,
            b"celt/cwrs.c\0" as *const u8 as *const i8,
            444 as i32,
        );
    }
    j = _n - 1 as i32;
    i = (*_y.offset(j as isize) < 0 as i32) as i32 as u32;
    k = (*_y.offset(j as isize)).abs();
    loop {
        j -= 1;
        i = (i as u32).wrapping_add(
            *(CELT_PVQ_U_ROW[(if _n - j < k { _n - j } else { k }) as usize])
                .offset((if _n - j > k { _n - j } else { k }) as isize),
        ) as u32 as u32;
        k += (*_y.offset(j as isize)).abs();
        if *_y.offset(j as isize) < 0 as i32 {
            i = (i as u32).wrapping_add(
                *(CELT_PVQ_U_ROW[(if _n - j < k + 1 as i32 {
                    _n - j
                } else {
                    k + 1 as i32
                }) as usize])
                    .offset(
                        (if _n - j > k + 1 as i32 {
                            _n - j
                        } else {
                            k + 1 as i32
                        }) as isize,
                    ),
            ) as u32 as u32;
        }
        if !(j > 0 as i32) {
            break;
        }
    }
    return i;
}
pub unsafe fn encode_pulses(mut _y: *const i32, mut _n: i32, mut _k: i32, mut _enc: *mut ec_enc) {
    if !(_k > 0 as i32) {
        celt_fatal(
            b"assertion failed: _k>0\0" as *const u8 as *const i8,
            b"celt/cwrs.c\0" as *const u8 as *const i8,
            459 as i32,
        );
    }
    ec_enc_uint(
        _enc,
        icwrs(_n, _y),
        (*(CELT_PVQ_U_ROW[(if _n < _k { _n } else { _k }) as usize])
            .offset((if _n > _k { _n } else { _k }) as isize))
        .wrapping_add(
            *(CELT_PVQ_U_ROW[(if _n < _k + 1 as i32 {
                _n
            } else {
                _k + 1 as i32
            }) as usize])
                .offset(
                    (if _n > _k + 1 as i32 {
                        _n
                    } else {
                        _k + 1 as i32
                    }) as isize,
                ),
        ),
    );
}
unsafe fn cwrsi(mut _n: i32, mut _k: i32, mut _i: u32, mut _y: *mut i32) -> opus_val32 {
    let mut p: u32 = 0;
    let mut s: i32 = 0;
    let mut k0: i32 = 0;
    let mut val: i16 = 0;
    let mut yy: opus_val32 = 0 as i32 as opus_val32;
    if !(_k > 0 as i32) {
        celt_fatal(
            b"assertion failed: _k>0\0" as *const u8 as *const i8,
            b"celt/cwrs.c\0" as *const u8 as *const i8,
            469 as i32,
        );
    }
    if !(_n > 1 as i32) {
        celt_fatal(
            b"assertion failed: _n>1\0" as *const u8 as *const i8,
            b"celt/cwrs.c\0" as *const u8 as *const i8,
            470 as i32,
        );
    }
    while _n > 2 as i32 {
        let mut q: u32 = 0;
        if _k >= _n {
            let mut row: *const u32 = 0 as *const u32;
            row = CELT_PVQ_U_ROW[_n as usize];
            p = *row.offset((_k + 1 as i32) as isize);
            s = -((_i >= p) as i32);
            _i = (_i as u32).wrapping_sub(p & s as u32) as u32 as u32;
            k0 = _k;
            q = *row.offset(_n as isize);
            if q > _i {
                _k = _n;
                loop {
                    _k -= 1;
                    p = *(CELT_PVQ_U_ROW[_k as usize]).offset(_n as isize);
                    if !(p > _i) {
                        break;
                    }
                }
            } else {
                p = *row.offset(_k as isize);
                while p > _i {
                    _k -= 1;
                    p = *row.offset(_k as isize);
                }
            }
            _i = (_i as u32).wrapping_sub(p) as u32 as u32;
            val = (k0 - _k + s ^ s) as i16;
            let fresh0 = _y;
            _y = _y.offset(1);
            *fresh0 = val as i32;
            yy = yy + val as opus_val32 * val as opus_val32;
        } else {
            p = *(CELT_PVQ_U_ROW[_k as usize]).offset(_n as isize);
            q = *(CELT_PVQ_U_ROW[(_k + 1 as i32) as usize]).offset(_n as isize);
            if p <= _i && _i < q {
                _i = (_i as u32).wrapping_sub(p) as u32 as u32;
                let fresh1 = _y;
                _y = _y.offset(1);
                *fresh1 = 0 as i32;
            } else {
                s = -((_i >= q) as i32);
                _i = (_i as u32).wrapping_sub(q & s as u32) as u32 as u32;
                k0 = _k;
                loop {
                    _k -= 1;
                    p = *(CELT_PVQ_U_ROW[_k as usize]).offset(_n as isize);
                    if !(p > _i) {
                        break;
                    }
                }
                _i = (_i as u32).wrapping_sub(p) as u32 as u32;
                val = (k0 - _k + s ^ s) as i16;
                let fresh2 = _y;
                _y = _y.offset(1);
                *fresh2 = val as i32;
                yy = yy + val as opus_val32 * val as opus_val32;
            }
        }
        _n -= 1;
    }
    p = (2 as i32 * _k + 1 as i32) as u32;
    s = -((_i >= p) as i32);
    _i = (_i as u32).wrapping_sub(p & s as u32) as u32 as u32;
    k0 = _k;
    _k = (_i.wrapping_add(1 as i32 as u32) >> 1 as i32) as i32;
    if _k != 0 {
        _i = (_i as u32).wrapping_sub((2 as i32 * _k - 1 as i32) as u32) as u32 as u32;
    }
    val = (k0 - _k + s ^ s) as i16;
    let fresh3 = _y;
    _y = _y.offset(1);
    *fresh3 = val as i32;
    yy = yy + val as opus_val32 * val as opus_val32;
    s = -(_i as i32);
    val = (_k + s ^ s) as i16;
    *_y = val as i32;
    yy = yy + val as opus_val32 * val as opus_val32;
    return yy;
}
pub unsafe fn decode_pulses(
    mut _y: *mut i32,
    mut _n: i32,
    mut _k: i32,
    mut _dec: *mut ec_dec,
) -> opus_val32 {
    return cwrsi(
        _n,
        _k,
        ec_dec_uint(
            _dec,
            (*(CELT_PVQ_U_ROW[(if _n < _k { _n } else { _k }) as usize])
                .offset((if _n > _k { _n } else { _k }) as isize))
            .wrapping_add(
                *(CELT_PVQ_U_ROW[(if _n < _k + 1 as i32 {
                    _n
                } else {
                    _k + 1 as i32
                }) as usize])
                    .offset(
                        (if _n > _k + 1 as i32 {
                            _n
                        } else {
                            _k + 1 as i32
                        }) as isize,
                    ),
            ),
        ),
        _y,
    );
}
unsafe fn run_static_initializers() {
    CELT_PVQ_U_ROW = [
        CELT_PVQ_U_DATA.as_ptr().offset(0 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(176 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(351 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(525 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(698 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(870 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1041 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1131 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1178 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1207 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1226 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1240 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1248 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1254 as i32 as isize),
        CELT_PVQ_U_DATA.as_ptr().offset(1257 as i32 as isize),
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
