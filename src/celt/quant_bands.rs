use crate::celt::entcode::{ec_get_buffer, ec_range_bytes, ec_tell, ec_tell_frac};
use crate::celt::entdec::{ec_dec, ec_dec_bit_logp, ec_dec_bits, ec_dec_icdf};
use crate::celt::entenc::{ec_enc, ec_enc_bit_logp, ec_enc_bits, ec_enc_icdf};
use crate::celt::laplace::{ec_laplace_decode, ec_laplace_encode};
use crate::celt::modes::OpusCustomMode;
use crate::celt::rate::MAX_FINE_BITS;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = f32;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = f32;
    #[c2rust::src_loc = "185:1"]
    pub type celt_ener = f32;
}

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/stack_alloc.h:39"]
pub mod stack_alloc_h {
    #[c2rust::src_loc = "99:9"]
    pub const ALLOC_NONE: i32 = 1 as i32;
}
pub use self::arch_h::{celt_ener, opus_val16, opus_val32};
pub use self::stack_alloc_h::ALLOC_NONE;

use crate::externs::memcpy;
#[c2rust::src_loc = "53:18"]
pub static eMeans: [opus_val16; 25] = [
    6.437500f32,
    6.250000f32,
    5.750000f32,
    5.312500f32,
    5.062500f32,
    4.812500f32,
    4.500000f32,
    4.375000f32,
    4.875000f32,
    4.687500f32,
    4.562500f32,
    4.437500f32,
    4.875000f32,
    4.625000f32,
    4.312500f32,
    4.500000f32,
    4.375000f32,
    4.625000f32,
    4.750000f32,
    4.437500f32,
    3.750000f32,
    3.750000f32,
    3.750000f32,
    3.750000f32,
    3.750000f32,
];
#[c2rust::src_loc = "67:25"]
static mut pred_coef: [opus_val16; 4] = [
    (29440 as i32 as f64 / 32768.0f64) as opus_val16,
    (26112 as i32 as f64 / 32768.0f64) as opus_val16,
    (21248 as i32 as f64 / 32768.0f64) as opus_val16,
    (16384 as i32 as f64 / 32768.0f64) as opus_val16,
];
#[c2rust::src_loc = "68:25"]
static mut beta_coef: [opus_val16; 4] = [
    (30147 as i32 as f64 / 32768.0f64) as opus_val16,
    (22282 as i32 as f64 / 32768.0f64) as opus_val16,
    (12124 as i32 as f64 / 32768.0f64) as opus_val16,
    (6554 as i32 as f64 / 32768.0f64) as opus_val16,
];
#[c2rust::src_loc = "69:25"]
static mut beta_intra: opus_val16 = (4915 as i32 as f64 / 32768.0f64) as opus_val16;
#[c2rust::src_loc = "77:28"]
static mut e_prob_model: [[[u8; 42]; 2]; 4] = [
    [
        [
            72 as i32 as u8,
            127 as i32 as u8,
            65 as i32 as u8,
            129 as i32 as u8,
            66 as i32 as u8,
            128 as i32 as u8,
            65 as i32 as u8,
            128 as i32 as u8,
            64 as i32 as u8,
            128 as i32 as u8,
            62 as i32 as u8,
            128 as i32 as u8,
            64 as i32 as u8,
            128 as i32 as u8,
            64 as i32 as u8,
            128 as i32 as u8,
            92 as i32 as u8,
            78 as i32 as u8,
            92 as i32 as u8,
            79 as i32 as u8,
            92 as i32 as u8,
            78 as i32 as u8,
            90 as i32 as u8,
            79 as i32 as u8,
            116 as i32 as u8,
            41 as i32 as u8,
            115 as i32 as u8,
            40 as i32 as u8,
            114 as i32 as u8,
            40 as i32 as u8,
            132 as i32 as u8,
            26 as i32 as u8,
            132 as i32 as u8,
            26 as i32 as u8,
            145 as i32 as u8,
            17 as i32 as u8,
            161 as i32 as u8,
            12 as i32 as u8,
            176 as i32 as u8,
            10 as i32 as u8,
            177 as i32 as u8,
            11 as i32 as u8,
        ],
        [
            24 as i32 as u8,
            179 as i32 as u8,
            48 as i32 as u8,
            138 as i32 as u8,
            54 as i32 as u8,
            135 as i32 as u8,
            54 as i32 as u8,
            132 as i32 as u8,
            53 as i32 as u8,
            134 as i32 as u8,
            56 as i32 as u8,
            133 as i32 as u8,
            55 as i32 as u8,
            132 as i32 as u8,
            55 as i32 as u8,
            132 as i32 as u8,
            61 as i32 as u8,
            114 as i32 as u8,
            70 as i32 as u8,
            96 as i32 as u8,
            74 as i32 as u8,
            88 as i32 as u8,
            75 as i32 as u8,
            88 as i32 as u8,
            87 as i32 as u8,
            74 as i32 as u8,
            89 as i32 as u8,
            66 as i32 as u8,
            91 as i32 as u8,
            67 as i32 as u8,
            100 as i32 as u8,
            59 as i32 as u8,
            108 as i32 as u8,
            50 as i32 as u8,
            120 as i32 as u8,
            40 as i32 as u8,
            122 as i32 as u8,
            37 as i32 as u8,
            97 as i32 as u8,
            43 as i32 as u8,
            78 as i32 as u8,
            50 as i32 as u8,
        ],
    ],
    [
        [
            83 as i32 as u8,
            78 as i32 as u8,
            84 as i32 as u8,
            81 as i32 as u8,
            88 as i32 as u8,
            75 as i32 as u8,
            86 as i32 as u8,
            74 as i32 as u8,
            87 as i32 as u8,
            71 as i32 as u8,
            90 as i32 as u8,
            73 as i32 as u8,
            93 as i32 as u8,
            74 as i32 as u8,
            93 as i32 as u8,
            74 as i32 as u8,
            109 as i32 as u8,
            40 as i32 as u8,
            114 as i32 as u8,
            36 as i32 as u8,
            117 as i32 as u8,
            34 as i32 as u8,
            117 as i32 as u8,
            34 as i32 as u8,
            143 as i32 as u8,
            17 as i32 as u8,
            145 as i32 as u8,
            18 as i32 as u8,
            146 as i32 as u8,
            19 as i32 as u8,
            162 as i32 as u8,
            12 as i32 as u8,
            165 as i32 as u8,
            10 as i32 as u8,
            178 as i32 as u8,
            7 as i32 as u8,
            189 as i32 as u8,
            6 as i32 as u8,
            190 as i32 as u8,
            8 as i32 as u8,
            177 as i32 as u8,
            9 as i32 as u8,
        ],
        [
            23 as i32 as u8,
            178 as i32 as u8,
            54 as i32 as u8,
            115 as i32 as u8,
            63 as i32 as u8,
            102 as i32 as u8,
            66 as i32 as u8,
            98 as i32 as u8,
            69 as i32 as u8,
            99 as i32 as u8,
            74 as i32 as u8,
            89 as i32 as u8,
            71 as i32 as u8,
            91 as i32 as u8,
            73 as i32 as u8,
            91 as i32 as u8,
            78 as i32 as u8,
            89 as i32 as u8,
            86 as i32 as u8,
            80 as i32 as u8,
            92 as i32 as u8,
            66 as i32 as u8,
            93 as i32 as u8,
            64 as i32 as u8,
            102 as i32 as u8,
            59 as i32 as u8,
            103 as i32 as u8,
            60 as i32 as u8,
            104 as i32 as u8,
            60 as i32 as u8,
            117 as i32 as u8,
            52 as i32 as u8,
            123 as i32 as u8,
            44 as i32 as u8,
            138 as i32 as u8,
            35 as i32 as u8,
            133 as i32 as u8,
            31 as i32 as u8,
            97 as i32 as u8,
            38 as i32 as u8,
            77 as i32 as u8,
            45 as i32 as u8,
        ],
    ],
    [
        [
            61 as i32 as u8,
            90 as i32 as u8,
            93 as i32 as u8,
            60 as i32 as u8,
            105 as i32 as u8,
            42 as i32 as u8,
            107 as i32 as u8,
            41 as i32 as u8,
            110 as i32 as u8,
            45 as i32 as u8,
            116 as i32 as u8,
            38 as i32 as u8,
            113 as i32 as u8,
            38 as i32 as u8,
            112 as i32 as u8,
            38 as i32 as u8,
            124 as i32 as u8,
            26 as i32 as u8,
            132 as i32 as u8,
            27 as i32 as u8,
            136 as i32 as u8,
            19 as i32 as u8,
            140 as i32 as u8,
            20 as i32 as u8,
            155 as i32 as u8,
            14 as i32 as u8,
            159 as i32 as u8,
            16 as i32 as u8,
            158 as i32 as u8,
            18 as i32 as u8,
            170 as i32 as u8,
            13 as i32 as u8,
            177 as i32 as u8,
            10 as i32 as u8,
            187 as i32 as u8,
            8 as i32 as u8,
            192 as i32 as u8,
            6 as i32 as u8,
            175 as i32 as u8,
            9 as i32 as u8,
            159 as i32 as u8,
            10 as i32 as u8,
        ],
        [
            21 as i32 as u8,
            178 as i32 as u8,
            59 as i32 as u8,
            110 as i32 as u8,
            71 as i32 as u8,
            86 as i32 as u8,
            75 as i32 as u8,
            85 as i32 as u8,
            84 as i32 as u8,
            83 as i32 as u8,
            91 as i32 as u8,
            66 as i32 as u8,
            88 as i32 as u8,
            73 as i32 as u8,
            87 as i32 as u8,
            72 as i32 as u8,
            92 as i32 as u8,
            75 as i32 as u8,
            98 as i32 as u8,
            72 as i32 as u8,
            105 as i32 as u8,
            58 as i32 as u8,
            107 as i32 as u8,
            54 as i32 as u8,
            115 as i32 as u8,
            52 as i32 as u8,
            114 as i32 as u8,
            55 as i32 as u8,
            112 as i32 as u8,
            56 as i32 as u8,
            129 as i32 as u8,
            51 as i32 as u8,
            132 as i32 as u8,
            40 as i32 as u8,
            150 as i32 as u8,
            33 as i32 as u8,
            140 as i32 as u8,
            29 as i32 as u8,
            98 as i32 as u8,
            35 as i32 as u8,
            77 as i32 as u8,
            42 as i32 as u8,
        ],
    ],
    [
        [
            42 as i32 as u8,
            121 as i32 as u8,
            96 as i32 as u8,
            66 as i32 as u8,
            108 as i32 as u8,
            43 as i32 as u8,
            111 as i32 as u8,
            40 as i32 as u8,
            117 as i32 as u8,
            44 as i32 as u8,
            123 as i32 as u8,
            32 as i32 as u8,
            120 as i32 as u8,
            36 as i32 as u8,
            119 as i32 as u8,
            33 as i32 as u8,
            127 as i32 as u8,
            33 as i32 as u8,
            134 as i32 as u8,
            34 as i32 as u8,
            139 as i32 as u8,
            21 as i32 as u8,
            147 as i32 as u8,
            23 as i32 as u8,
            152 as i32 as u8,
            20 as i32 as u8,
            158 as i32 as u8,
            25 as i32 as u8,
            154 as i32 as u8,
            26 as i32 as u8,
            166 as i32 as u8,
            21 as i32 as u8,
            173 as i32 as u8,
            16 as i32 as u8,
            184 as i32 as u8,
            13 as i32 as u8,
            184 as i32 as u8,
            10 as i32 as u8,
            150 as i32 as u8,
            13 as i32 as u8,
            139 as i32 as u8,
            15 as i32 as u8,
        ],
        [
            22 as i32 as u8,
            178 as i32 as u8,
            63 as i32 as u8,
            114 as i32 as u8,
            74 as i32 as u8,
            82 as i32 as u8,
            84 as i32 as u8,
            83 as i32 as u8,
            92 as i32 as u8,
            82 as i32 as u8,
            103 as i32 as u8,
            62 as i32 as u8,
            96 as i32 as u8,
            72 as i32 as u8,
            96 as i32 as u8,
            67 as i32 as u8,
            101 as i32 as u8,
            73 as i32 as u8,
            107 as i32 as u8,
            72 as i32 as u8,
            113 as i32 as u8,
            55 as i32 as u8,
            118 as i32 as u8,
            52 as i32 as u8,
            125 as i32 as u8,
            52 as i32 as u8,
            118 as i32 as u8,
            52 as i32 as u8,
            117 as i32 as u8,
            55 as i32 as u8,
            135 as i32 as u8,
            49 as i32 as u8,
            137 as i32 as u8,
            39 as i32 as u8,
            157 as i32 as u8,
            32 as i32 as u8,
            145 as i32 as u8,
            29 as i32 as u8,
            97 as i32 as u8,
            33 as i32 as u8,
            77 as i32 as u8,
            40 as i32 as u8,
        ],
    ],
];
#[c2rust::src_loc = "140:28"]
static mut small_energy_icdf: [u8; 3] = [2 as i32 as u8, 1 as i32 as u8, 0 as i32 as u8];
#[c2rust::src_loc = "142:1"]
unsafe fn loss_distortion(
    eBands: *const opus_val16,
    oldEBands: *mut opus_val16,
    start: i32,
    end: i32,
    len: i32,
    C: i32,
) -> opus_val32 {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    let mut dist: opus_val32 = 0 as i32 as opus_val32;
    c = 0 as i32;
    loop {
        i = start;
        while i < end {
            let d: opus_val16 =
                *eBands.offset((i + c * len) as isize) - *oldEBands.offset((i + c * len) as isize);
            dist = dist + d * d;
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    return if (200 as i32 as f32) < dist {
        200 as i32 as f32
    } else {
        dist
    };
}
#[c2rust::src_loc = "156:1"]
unsafe fn quant_coarse_energy_impl(
    m: *const OpusCustomMode,
    start: i32,
    end: i32,
    eBands: *const opus_val16,
    oldEBands: *mut opus_val16,
    budget: i32,
    mut tell: i32,
    prob_model: *const u8,
    error: *mut opus_val16,
    enc: *mut ec_enc,
    C: i32,
    LM: i32,
    intra: i32,
    max_decay: opus_val16,
    lfe: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut badness: i32 = 0 as i32;
    let mut prev: [opus_val32; 2] = [0 as i32 as opus_val32, 0 as i32 as opus_val32];
    let mut coef: opus_val16 = 0.;
    let mut beta: opus_val16 = 0.;
    if tell + 3 as i32 <= budget {
        ec_enc_bit_logp(enc, intra, 3 as i32 as u32);
    }
    if intra != 0 {
        coef = 0 as i32 as opus_val16;
        beta = beta_intra;
    } else {
        beta = beta_coef[LM as usize];
        coef = pred_coef[LM as usize];
    }
    i = start;
    while i < end {
        c = 0 as i32;
        loop {
            let mut bits_left: i32 = 0;
            let mut qi: i32 = 0;
            let mut qi0: i32 = 0;
            let mut q: opus_val32 = 0.;
            let mut x: opus_val16 = 0.;
            let mut f: opus_val32 = 0.;
            let mut tmp: opus_val32 = 0.;
            let mut oldE: opus_val16 = 0.;
            let mut decay_bound: opus_val16 = 0.;
            x = *eBands.offset((i + c * (*m).nbEBands) as isize);
            oldE = if -9.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                -9.0f32
            } else {
                *oldEBands.offset((i + c * (*m).nbEBands) as isize)
            };
            f = x - coef * oldE - prev[c as usize];
            qi = (0.5f32 + f).floor() as i32;
            decay_bound = (if -28.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                -28.0f32
            } else {
                *oldEBands.offset((i + c * (*m).nbEBands) as isize)
            }) - max_decay;
            if qi < 0 as i32 && x < decay_bound {
                qi += (decay_bound - x) as i32;
                if qi > 0 as i32 {
                    qi = 0 as i32;
                }
            }
            qi0 = qi;
            tell = ec_tell(enc);
            bits_left = budget - tell - 3 as i32 * C * (end - i);
            if i != start && bits_left < 30 as i32 {
                if bits_left < 24 as i32 {
                    qi = if (1 as i32) < qi { 1 as i32 } else { qi };
                }
                if bits_left < 16 as i32 {
                    qi = if -(1 as i32) > qi { -(1 as i32) } else { qi };
                }
            }
            if lfe != 0 && i >= 2 as i32 {
                qi = if qi < 0 as i32 { qi } else { 0 as i32 };
            }
            if budget - tell >= 15 as i32 {
                let mut pi: i32 = 0;
                pi = 2 as i32 * (if i < 20 as i32 { i } else { 20 as i32 });
                ec_laplace_encode(
                    enc,
                    &mut qi,
                    ((*prob_model.offset(pi as isize) as i32) << 7 as i32) as u32,
                    (*prob_model.offset((pi + 1 as i32) as isize) as i32) << 6 as i32,
                );
            } else if budget - tell >= 2 as i32 {
                qi = if -(1 as i32) > (if qi < 1 as i32 { qi } else { 1 as i32 }) {
                    -(1 as i32)
                } else if qi < 1 as i32 {
                    qi
                } else {
                    1 as i32
                };
                ec_enc_icdf(
                    enc,
                    2 as i32 * qi ^ -((qi < 0 as i32) as i32),
                    small_energy_icdf.as_ptr(),
                    2 as i32 as u32,
                );
            } else if budget - tell >= 1 as i32 {
                qi = if (0 as i32) < qi { 0 as i32 } else { qi };
                ec_enc_bit_logp(enc, -qi, 1 as i32 as u32);
            } else {
                qi = -(1 as i32);
            }
            *error.offset((i + c * (*m).nbEBands) as isize) = f - qi as f32;
            badness += (qi0 - qi).abs();
            q = qi as opus_val32;
            tmp = coef * oldE + prev[c as usize] + q;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) = tmp;
            prev[c as usize] = prev[c as usize] + q - beta * q;
            c += 1;
            if !(c < C) {
                break;
            }
        }
        i += 1;
    }
    return if lfe != 0 { 0 as i32 } else { badness };
}
#[c2rust::src_loc = "261:1"]
pub unsafe fn quant_coarse_energy(
    m: *const OpusCustomMode,
    start: i32,
    end: i32,
    effEnd: i32,
    eBands: *const opus_val16,
    oldEBands: *mut opus_val16,
    budget: u32,
    error: *mut opus_val16,
    enc: *mut ec_enc,
    C: i32,
    LM: i32,
    nbAvailableBytes: i32,
    force_intra: i32,
    delayedIntra: *mut opus_val32,
    mut two_pass: i32,
    loss_rate: i32,
    lfe: i32,
) {
    let mut intra: i32 = 0;
    let mut max_decay: opus_val16 = 0.;
    let mut enc_start_state: ec_enc = ec_enc {
        buf: 0 as *mut u8,
        storage: 0,
        end_offs: 0,
        end_window: 0,
        nend_bits: 0,
        nbits_total: 0,
        offs: 0,
        rng: 0,
        val: 0,
        ext: 0,
        rem: 0,
        error: 0,
    };
    let mut tell: u32 = 0;
    let mut badness1: i32 = 0 as i32;
    let mut intra_bias: i32 = 0;
    let mut new_distortion: opus_val32 = 0.;
    intra = (force_intra != 0
        || two_pass == 0
            && *delayedIntra > (2 as i32 * C * (end - start)) as f32
            && nbAvailableBytes > (end - start) * C) as i32;
    intra_bias =
        (budget as f32 * *delayedIntra * loss_rate as f32 / (C * 512 as i32) as f32) as i32;
    new_distortion = loss_distortion(eBands, oldEBands, start, effEnd, (*m).nbEBands, C);
    tell = ec_tell(enc) as u32;
    if tell.wrapping_add(3 as i32 as u32) > budget {
        intra = 0 as i32;
        two_pass = intra;
    }
    max_decay = 16.0f32;
    if end - start > 10 as i32 {
        max_decay = if max_decay < 0.125f32 * nbAvailableBytes as f32 {
            max_decay
        } else {
            0.125f32 * nbAvailableBytes as f32
        };
    }
    if lfe != 0 {
        max_decay = 3.0f32;
    }
    enc_start_state = *enc;
    let vla = (C * (*m).nbEBands) as usize;
    let mut oldEBands_intra: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    let vla_0 = (C * (*m).nbEBands) as usize;
    let mut error_intra: Vec<opus_val16> = ::std::vec::from_elem(0., vla_0);
    memcpy(
        oldEBands_intra.as_mut_ptr() as *mut core::ffi::c_void,
        oldEBands as *const core::ffi::c_void,
        ((C * (*m).nbEBands) as u64)
            .wrapping_mul(::core::mem::size_of::<opus_val16>() as u64)
            .wrapping_add(
                (0 as i32 as i64 * oldEBands_intra.as_mut_ptr().offset_from(oldEBands) as i64)
                    as u64,
            ),
    );
    if two_pass != 0 || intra != 0 {
        badness1 = quant_coarse_energy_impl(
            m,
            start,
            end,
            eBands,
            oldEBands_intra.as_mut_ptr(),
            budget as i32,
            tell as i32,
            (e_prob_model[LM as usize][1 as i32 as usize]).as_ptr(),
            error_intra.as_mut_ptr(),
            enc,
            C,
            LM,
            1 as i32,
            max_decay,
            lfe,
        );
    }
    if intra == 0 {
        let mut intra_buf: *mut u8 = 0 as *mut u8;
        let mut enc_intra_state: ec_enc = ec_enc {
            buf: 0 as *mut u8,
            storage: 0,
            end_offs: 0,
            end_window: 0,
            nend_bits: 0,
            nbits_total: 0,
            offs: 0,
            rng: 0,
            val: 0,
            ext: 0,
            rem: 0,
            error: 0,
        };
        let mut tell_intra: i32 = 0;
        let mut nstart_bytes: u32 = 0;
        let mut nintra_bytes: u32 = 0;
        let mut save_bytes: u32 = 0;
        let mut badness2: i32 = 0;
        tell_intra = ec_tell_frac(enc) as i32;
        enc_intra_state = *enc;
        nstart_bytes = ec_range_bytes(&mut enc_start_state);
        nintra_bytes = ec_range_bytes(&mut enc_intra_state);
        intra_buf = (ec_get_buffer(&mut enc_intra_state)).offset(nstart_bytes as isize);
        save_bytes = nintra_bytes.wrapping_sub(nstart_bytes);
        if save_bytes == 0 as i32 as u32 {
            save_bytes = ALLOC_NONE as u32;
        }
        let vla_1 = save_bytes as usize;
        let mut intra_bits: Vec<u8> = ::std::vec::from_elem(0, vla_1);
        memcpy(
            intra_bits.as_mut_ptr() as *mut core::ffi::c_void,
            intra_buf as *const core::ffi::c_void,
            (nintra_bytes.wrapping_sub(nstart_bytes) as u64)
                .wrapping_mul(::core::mem::size_of::<u8>() as u64)
                .wrapping_add(
                    (0 as i32 as i64 * intra_bits.as_mut_ptr().offset_from(intra_buf) as i64)
                        as u64,
                ),
        );
        *enc = enc_start_state;
        badness2 = quant_coarse_energy_impl(
            m,
            start,
            end,
            eBands,
            oldEBands,
            budget as i32,
            tell as i32,
            (e_prob_model[LM as usize][intra as usize]).as_ptr(),
            error,
            enc,
            C,
            LM,
            0 as i32,
            max_decay,
            lfe,
        );
        if two_pass != 0
            && (badness1 < badness2
                || badness1 == badness2 && ec_tell_frac(enc) as i32 + intra_bias > tell_intra)
        {
            *enc = enc_intra_state;
            memcpy(
                intra_buf as *mut core::ffi::c_void,
                intra_bits.as_mut_ptr() as *const core::ffi::c_void,
                (nintra_bytes.wrapping_sub(nstart_bytes) as u64)
                    .wrapping_mul(::core::mem::size_of::<u8>() as u64)
                    .wrapping_add(
                        (0 as i32 as i64 * intra_buf.offset_from(intra_bits.as_mut_ptr()) as i64)
                            as u64,
                    ),
            );
            memcpy(
                oldEBands as *mut core::ffi::c_void,
                oldEBands_intra.as_mut_ptr() as *const core::ffi::c_void,
                ((C * (*m).nbEBands) as u64)
                    .wrapping_mul(::core::mem::size_of::<opus_val16>() as u64)
                    .wrapping_add(
                        (0 as i32 as i64
                            * oldEBands.offset_from(oldEBands_intra.as_mut_ptr()) as i64)
                            as u64,
                    ),
            );
            memcpy(
                error as *mut core::ffi::c_void,
                error_intra.as_mut_ptr() as *const core::ffi::c_void,
                ((C * (*m).nbEBands) as u64)
                    .wrapping_mul(::core::mem::size_of::<opus_val16>() as u64)
                    .wrapping_add(
                        (0 as i32 as i64 * error.offset_from(error_intra.as_mut_ptr()) as i64)
                            as u64,
                    ),
            );
            intra = 1 as i32;
        }
    } else {
        memcpy(
            oldEBands as *mut core::ffi::c_void,
            oldEBands_intra.as_mut_ptr() as *const core::ffi::c_void,
            ((C * (*m).nbEBands) as u64)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as u64)
                .wrapping_add(
                    (0 as i32 as i64 * oldEBands.offset_from(oldEBands_intra.as_mut_ptr()) as i64)
                        as u64,
                ),
        );
        memcpy(
            error as *mut core::ffi::c_void,
            error_intra.as_mut_ptr() as *const core::ffi::c_void,
            ((C * (*m).nbEBands) as u64)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as u64)
                .wrapping_add(
                    (0 as i32 as i64 * error.offset_from(error_intra.as_mut_ptr()) as i64) as u64,
                ),
        );
    }
    if intra != 0 {
        *delayedIntra = new_distortion;
    } else {
        *delayedIntra =
            pred_coef[LM as usize] * pred_coef[LM as usize] * *delayedIntra + new_distortion;
    };
}
#[c2rust::src_loc = "361:1"]
pub unsafe fn quant_fine_energy(
    m: *const OpusCustomMode,
    start: i32,
    end: i32,
    oldEBands: *mut opus_val16,
    error: *mut opus_val16,
    fine_quant: *mut i32,
    enc: *mut ec_enc,
    C: i32,
) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    i = start;
    while i < end {
        let frac: i16 = ((1 as i32) << *fine_quant.offset(i as isize)) as i16;
        if !(*fine_quant.offset(i as isize) <= 0 as i32) {
            c = 0 as i32;
            loop {
                let mut q2: i32 = 0;
                let mut offset: opus_val16 = 0.;
                q2 = ((*error.offset((i + c * (*m).nbEBands) as isize) + 0.5f32)
                    * frac as i32 as f32)
                    .floor() as i32;
                if q2 > frac as i32 - 1 as i32 {
                    q2 = frac as i32 - 1 as i32;
                }
                if q2 < 0 as i32 {
                    q2 = 0 as i32;
                }
                ec_enc_bits(enc, q2 as u32, *fine_quant.offset(i as isize) as u32);
                offset = (q2 as f32 + 0.5f32)
                    * ((1 as i32) << 14 as i32 - *fine_quant.offset(i as isize)) as f32
                    * (1.0f32 / 16384 as i32 as f32)
                    - 0.5f32;
                let ref mut fresh0 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                *fresh0 += offset;
                let ref mut fresh1 = *error.offset((i + c * (*m).nbEBands) as isize);
                *fresh1 -= offset;
                c += 1;
                if !(c < C) {
                    break;
                }
            }
        }
        i += 1;
    }
}
#[c2rust::src_loc = "398:1"]
pub unsafe fn quant_energy_finalise(
    m: *const OpusCustomMode,
    start: i32,
    end: i32,
    oldEBands: *mut opus_val16,
    error: *mut opus_val16,
    fine_quant: *mut i32,
    fine_priority: *mut i32,
    mut bits_left: i32,
    enc: *mut ec_enc,
    C: i32,
) {
    let mut i: i32 = 0;
    let mut prio: i32 = 0;
    let mut c: i32 = 0;
    prio = 0 as i32;
    while prio < 2 as i32 {
        i = start;
        while i < end && bits_left >= C {
            if !(*fine_quant.offset(i as isize) >= MAX_FINE_BITS
                || *fine_priority.offset(i as isize) != prio)
            {
                c = 0 as i32;
                loop {
                    let mut q2: i32 = 0;
                    let mut offset: opus_val16 = 0.;
                    q2 = if *error.offset((i + c * (*m).nbEBands) as isize) < 0 as i32 as f32 {
                        0 as i32
                    } else {
                        1 as i32
                    };
                    ec_enc_bits(enc, q2 as u32, 1 as i32 as u32);
                    offset = (q2 as f32 - 0.5f32)
                        * ((1 as i32) << 14 as i32 - *fine_quant.offset(i as isize) - 1 as i32)
                            as f32
                        * (1.0f32 / 16384 as i32 as f32);
                    let ref mut fresh2 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                    *fresh2 += offset;
                    let ref mut fresh3 = *error.offset((i + c * (*m).nbEBands) as isize);
                    *fresh3 -= offset;
                    bits_left -= 1;
                    c += 1;
                    if !(c < C) {
                        break;
                    }
                }
            }
            i += 1;
        }
        prio += 1;
    }
}
#[c2rust::src_loc = "428:1"]
pub unsafe fn unquant_coarse_energy(
    m: *const OpusCustomMode,
    start: i32,
    end: i32,
    oldEBands: *mut opus_val16,
    intra: i32,
    dec: *mut ec_dec,
    C: i32,
    LM: i32,
) {
    let prob_model: *const u8 = (e_prob_model[LM as usize][intra as usize]).as_ptr();
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut prev: [opus_val32; 2] = [0 as i32 as opus_val32, 0 as i32 as opus_val32];
    let mut coef: opus_val16 = 0.;
    let mut beta: opus_val16 = 0.;
    let mut budget: i32 = 0;
    let mut tell: i32 = 0;
    if intra != 0 {
        coef = 0 as i32 as opus_val16;
        beta = beta_intra;
    } else {
        beta = beta_coef[LM as usize];
        coef = pred_coef[LM as usize];
    }
    budget = ((*dec).storage).wrapping_mul(8 as i32 as u32) as i32;
    i = start;
    while i < end {
        c = 0 as i32;
        loop {
            let mut qi: i32 = 0;
            let mut q: opus_val32 = 0.;
            let mut tmp: opus_val32 = 0.;
            tell = ec_tell(dec);
            if budget - tell >= 15 as i32 {
                let mut pi: i32 = 0;
                pi = 2 as i32 * (if i < 20 as i32 { i } else { 20 as i32 });
                qi = ec_laplace_decode(
                    dec,
                    ((*prob_model.offset(pi as isize) as i32) << 7 as i32) as u32,
                    (*prob_model.offset((pi + 1 as i32) as isize) as i32) << 6 as i32,
                );
            } else if budget - tell >= 2 as i32 {
                qi = ec_dec_icdf(dec, small_energy_icdf.as_ptr(), 2 as i32 as u32);
                qi = qi >> 1 as i32 ^ -(qi & 1 as i32);
            } else if budget - tell >= 1 as i32 {
                qi = -ec_dec_bit_logp(dec, 1 as i32 as u32);
            } else {
                qi = -(1 as i32);
            }
            q = qi as opus_val32;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) =
                if -9.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                    -9.0f32
                } else {
                    *oldEBands.offset((i + c * (*m).nbEBands) as isize)
                };
            tmp = coef * *oldEBands.offset((i + c * (*m).nbEBands) as isize) + prev[c as usize] + q;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) = tmp;
            prev[c as usize] = prev[c as usize] + q - beta * q;
            c += 1;
            if !(c < C) {
                break;
            }
        }
        i += 1;
    }
}
#[c2rust::src_loc = "493:1"]
pub unsafe fn unquant_fine_energy(
    m: *const OpusCustomMode,
    start: i32,
    end: i32,
    oldEBands: *mut opus_val16,
    fine_quant: *mut i32,
    dec: *mut ec_dec,
    C: i32,
) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    i = start;
    while i < end {
        if !(*fine_quant.offset(i as isize) <= 0 as i32) {
            c = 0 as i32;
            loop {
                let mut q2: i32 = 0;
                let mut offset: opus_val16 = 0.;
                q2 = ec_dec_bits(dec, *fine_quant.offset(i as isize) as u32) as i32;
                offset = (q2 as f32 + 0.5f32)
                    * ((1 as i32) << 14 as i32 - *fine_quant.offset(i as isize)) as f32
                    * (1.0f32 / 16384 as i32 as f32)
                    - 0.5f32;
                let ref mut fresh4 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                *fresh4 += offset;
                c += 1;
                if !(c < C) {
                    break;
                }
            }
        }
        i += 1;
    }
}
#[c2rust::src_loc = "516:1"]
pub unsafe fn unquant_energy_finalise(
    m: *const OpusCustomMode,
    start: i32,
    end: i32,
    oldEBands: *mut opus_val16,
    fine_quant: *mut i32,
    fine_priority: *mut i32,
    mut bits_left: i32,
    dec: *mut ec_dec,
    C: i32,
) {
    let mut i: i32 = 0;
    let mut prio: i32 = 0;
    let mut c: i32 = 0;
    prio = 0 as i32;
    while prio < 2 as i32 {
        i = start;
        while i < end && bits_left >= C {
            if !(*fine_quant.offset(i as isize) >= MAX_FINE_BITS
                || *fine_priority.offset(i as isize) != prio)
            {
                c = 0 as i32;
                loop {
                    let mut q2: i32 = 0;
                    let mut offset: opus_val16 = 0.;
                    q2 = ec_dec_bits(dec, 1 as i32 as u32) as i32;
                    offset = (q2 as f32 - 0.5f32)
                        * ((1 as i32) << 14 as i32 - *fine_quant.offset(i as isize) - 1 as i32)
                            as f32
                        * (1.0f32 / 16384 as i32 as f32);
                    let ref mut fresh5 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                    *fresh5 += offset;
                    bits_left -= 1;
                    c += 1;
                    if !(c < C) {
                        break;
                    }
                }
            }
            i += 1;
        }
        prio += 1;
    }
}
#[c2rust::src_loc = "544:1"]
pub unsafe fn amp2Log2(
    m: *const OpusCustomMode,
    effEnd: i32,
    end: i32,
    bandE: *mut celt_ener,
    bandLogE: *mut opus_val16,
    C: i32,
) {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    c = 0 as i32;
    loop {
        i = 0 as i32;
        while i < effEnd {
            *bandLogE.offset((i + c * (*m).nbEBands) as isize) = (std::f32::consts::LOG2_E
                * (*bandE.offset((i + c * (*m).nbEBands) as isize)).ln())
                - eMeans[i as usize];
            i += 1;
        }
        i = effEnd;
        while i < end {
            *bandLogE.offset((c * (*m).nbEBands + i) as isize) = -14.0f32;
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
}
