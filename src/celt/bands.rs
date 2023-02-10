use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:35"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[c2rust::src_loc = "183:1"]
    pub type celt_sig = libc::c_float;
    #[c2rust::src_loc = "184:1"]
    pub type celt_norm = libc::c_float;
    #[c2rust::src_loc = "185:1"]
    pub type celt_ener = libc::c_float;
    #[c2rust::src_loc = "203:9"]
    pub const Q15ONE: libc::c_float = 1.0f32;
    #[c2rust::src_loc = "205:9"]
    pub const NORM_SCALING: libc::c_float = 1.0f32;
    #[c2rust::src_loc = "207:9"]
    pub const EPSILON: libc::c_float = 1e-15f32;
}

#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:35"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:35"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int =
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/bands.h:35"]
pub mod bands_h {
    #[c2rust::src_loc = "68:9"]
    pub const SPREAD_NONE: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "69:9"]
    pub const SPREAD_LIGHT: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "70:9"]
    pub const SPREAD_NORMAL: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "71:9"]
    pub const SPREAD_AGGRESSIVE: libc::c_int = 3 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/stack_alloc.h:35"]
pub mod stack_alloc_h {
    #[c2rust::src_loc = "99:9"]
    pub const ALLOC_NONE: libc::c_int = 1 as libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:35"]
pub mod stddef_h {
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/vq.h:37"]
pub mod vq_h {
    use super::arch_h::{celt_norm, opus_val16};
    use crate::celt::entdec::ec_dec;
    use crate::celt::entenc::ec_enc;
    extern "C" {
        #[c2rust::src_loc = "62:1"]
        pub fn alg_quant(
            X: *mut celt_norm,
            N: libc::c_int,
            K: libc::c_int,
            spread: libc::c_int,
            B: libc::c_int,
            enc: *mut ec_enc,
            gain: opus_val16,
            resynth: libc::c_int,
            arch: libc::c_int,
        ) -> libc::c_uint;
        #[c2rust::src_loc = "72:1"]
        pub fn alg_unquant(
            X: *mut celt_norm,
            N: libc::c_int,
            K: libc::c_int,
            spread: libc::c_int,
            B: libc::c_int,
            dec: *mut ec_dec,
            gain: opus_val16,
        ) -> libc::c_uint;
        #[c2rust::src_loc = "75:1"]
        pub fn renormalise_vector(
            X: *mut celt_norm,
            N: libc::c_int,
            gain: opus_val16,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "77:1"]
        pub fn stereo_itheta(
            X: *const celt_norm,
            Y: *const celt_norm,
            stereo: libc::c_int,
            N: libc::c_int,
            arch: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mathops.h:41"]
pub mod mathops_h {
    extern "C" {
        #[c2rust::src_loc = "46:1"]
        pub fn isqrt32(_val: u32) -> libc::c_uint;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/quant_bands.h:43"]
pub mod quant_bands_h {
    use super::arch_h::opus_val16;
    extern "C" {
        #[c2rust::src_loc = "41:25"]
        pub static eMeans: [opus_val16; 25];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/pitch.h:44"]
pub mod pitch_h {
    #[inline]
    #[c2rust::src_loc = "137:1"]
    pub unsafe extern "C" fn dual_inner_prod_c(
        x: *const opus_val16,
        y01: *const opus_val16,
        y02: *const opus_val16,
        N: libc::c_int,
        xy1: *mut opus_val32,
        xy2: *mut opus_val32,
    ) {
        let mut i: libc::c_int = 0;
        let mut xy01: opus_val32 = 0 as libc::c_int as opus_val32;
        let mut xy02: opus_val32 = 0 as libc::c_int as opus_val32;
        i = 0 as libc::c_int;
        while i < N {
            xy01 = xy01 + *x.offset(i as isize) * *y01.offset(i as isize);
            xy02 = xy02 + *x.offset(i as isize) * *y02.offset(i as isize);
            i += 1;
        }
        *xy1 = xy01;
        *xy2 = xy02;
    }
    #[inline]
    #[c2rust::src_loc = "159:1"]
    pub unsafe extern "C" fn celt_inner_prod_c(
        x: *const opus_val16,
        y: *const opus_val16,
        N: libc::c_int,
    ) -> opus_val32 {
        let mut i: libc::c_int = 0;
        let mut xy: opus_val32 = 0 as libc::c_int as opus_val32;
        i = 0 as libc::c_int;
        while i < N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
            i += 1;
        }
        return xy;
    }
    use super::arch_h::{opus_val16, opus_val32};
}
pub use self::arch_h::{
    celt_ener, celt_norm, celt_sig, opus_val16, opus_val32, EPSILON, NORM_SCALING, Q15ONE,
};
pub use self::bands_h::{SPREAD_AGGRESSIVE, SPREAD_LIGHT, SPREAD_NONE, SPREAD_NORMAL};
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
use self::mathops_h::isqrt32;
pub use self::pitch_h::{celt_inner_prod_c, dual_inner_prod_c};
use self::quant_bands_h::eMeans;
pub use self::stack_alloc_h::ALLOC_NONE;
pub use self::stddef_h::NULL;
use crate::celt::celt::celt_fatal;
use crate::celt::entcode::{celt_sudiv, celt_udiv, ec_ctx, ec_tell_frac, BITRES};
use crate::celt::entdec::{ec_dec_bit_logp, ec_dec_bits, ec_dec_uint, ec_dec_update, ec_decode};
use crate::celt::entenc::{ec_enc_bit_logp, ec_enc_bits, ec_enc_uint, ec_encode};
use crate::celt::modes::OpusCustomMode;
use crate::celt::rate::{
    bits2pulses, get_pulses, pulses2bits, QTHETA_OFFSET, QTHETA_OFFSET_TWOPHASE,
};

use self::vq_h::{alg_quant, alg_unquant, renormalise_vector, stereo_itheta};
use crate::externs::{memcpy, memset};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "673:8"]
pub struct band_ctx {
    pub encode: libc::c_int,
    pub resynth: libc::c_int,
    pub m: *const OpusCustomMode,
    pub i: libc::c_int,
    pub intensity: libc::c_int,
    pub spread: libc::c_int,
    pub tf_change: libc::c_int,
    pub ec: *mut ec_ctx,
    pub remaining_bits: i32,
    pub bandE: *const celt_ener,
    pub seed: u32,
    pub arch: libc::c_int,
    pub theta_round: libc::c_int,
    pub disable_inv: libc::c_int,
    pub avoid_split_noise: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "691:8"]
pub struct split_ctx {
    pub inv: libc::c_int,
    pub imid: libc::c_int,
    pub iside: libc::c_int,
    pub delta: libc::c_int,
    pub itheta: libc::c_int,
    pub qalloc: libc::c_int,
}
#[no_mangle]
#[c2rust::src_loc = "46:1"]
pub unsafe extern "C" fn hysteresis_decision(
    val: opus_val16,
    thresholds: *const opus_val16,
    hysteresis: *const opus_val16,
    N: libc::c_int,
    prev: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    while i < N {
        if val < *thresholds.offset(i as isize) {
            break;
        }
        i += 1;
    }
    if i > prev && val < *thresholds.offset(prev as isize) + *hysteresis.offset(prev as isize) {
        i = prev;
    }
    if i < prev
        && val
            > *thresholds.offset((prev - 1 as libc::c_int) as isize)
                - *hysteresis.offset((prev - 1 as libc::c_int) as isize)
    {
        i = prev;
    }
    return i;
}
#[no_mangle]
#[c2rust::src_loc = "61:1"]
pub unsafe extern "C" fn celt_lcg_rand(seed: u32) -> u32 {
    return (1664525 as libc::c_int as libc::c_uint)
        .wrapping_mul(seed)
        .wrapping_add(1013904223 as libc::c_int as libc::c_uint);
}
#[no_mangle]
#[c2rust::src_loc = "68:1"]
pub unsafe extern "C" fn bitexact_cos(x: i16) -> i16 {
    let mut tmp: i32 = 0;
    let mut x2: i16 = 0;
    tmp = 4096 as libc::c_int + x as i32 * x as libc::c_int >> 13 as libc::c_int;
    x2 = tmp as i16;
    x2 = (32767 as libc::c_int - x2 as libc::c_int
        + (16384 as libc::c_int
            + x2 as i32
                * (-(7651 as libc::c_int)
                    + (16384 as libc::c_int
                        + x2 as i32
                            * (8277 as libc::c_int
                                + (16384 as libc::c_int
                                    + -(626 as libc::c_int) as i16 as i32 * x2 as libc::c_int
                                    >> 15 as libc::c_int)) as i16
                                as libc::c_int
                        >> 15 as libc::c_int)) as i16 as libc::c_int
            >> 15 as libc::c_int)) as i16;
    return (1 as libc::c_int + x2 as libc::c_int) as i16;
}
#[no_mangle]
#[c2rust::src_loc = "80:1"]
pub unsafe extern "C" fn bitexact_log2tan(
    mut isin: libc::c_int,
    mut icos: libc::c_int,
) -> libc::c_int {
    let mut lc: libc::c_int = 0;
    let mut ls: libc::c_int = 0;
    lc = EC_CLZ0 - (icos as libc::c_uint).leading_zeros() as i32;
    ls = EC_CLZ0 - (isin as libc::c_uint).leading_zeros() as i32;
    icos <<= 15 as libc::c_int - lc;
    isin <<= 15 as libc::c_int - ls;
    return (ls - lc) * ((1 as libc::c_int) << 11 as libc::c_int)
        + (16384 as libc::c_int
            + isin as i16 as i32
                * ((16384 as libc::c_int
                    + isin as i16 as i32 * -(2597 as libc::c_int) as i16 as libc::c_int
                    >> 15 as libc::c_int)
                    + 7932 as libc::c_int) as i16 as libc::c_int
            >> 15 as libc::c_int)
        - (16384 as libc::c_int
            + icos as i16 as i32
                * ((16384 as libc::c_int
                    + icos as i16 as i32 * -(2597 as libc::c_int) as i16 as libc::c_int
                    >> 15 as libc::c_int)
                    + 7932 as libc::c_int) as i16 as libc::c_int
            >> 15 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "159:1"]
pub unsafe extern "C" fn compute_band_energies(
    m: *const OpusCustomMode,
    X: *const celt_sig,
    bandE: *mut celt_ener,
    end: libc::c_int,
    C: libc::c_int,
    LM: libc::c_int,
    _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let eBands: *const i16 = (*m).eBands;
    N = (*m).shortMdctSize << LM;
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < end {
            let mut sum: opus_val32 = 0.;
            sum = 1e-27f32
                + celt_inner_prod_c(
                    &*X.offset(
                        (c * N + ((*eBands.offset(i as isize) as libc::c_int) << LM)) as isize,
                    ),
                    &*X.offset(
                        (c * N + ((*eBands.offset(i as isize) as libc::c_int) << LM)) as isize,
                    ),
                    (*eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        - *eBands.offset(i as isize) as libc::c_int)
                        << LM,
                );
            *bandE.offset((i + c * (*m).nbEBands) as isize) = sum.sqrt();
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "177:1"]
pub unsafe extern "C" fn normalise_bands(
    m: *const OpusCustomMode,
    freq: *const celt_sig,
    X: *mut celt_norm,
    bandE: *const celt_ener,
    end: libc::c_int,
    C: libc::c_int,
    M: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let eBands: *const i16 = (*m).eBands;
    N = M * (*m).shortMdctSize;
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < end {
            let mut j: libc::c_int = 0;
            let g: opus_val16 =
                1.0f32 / (1e-27f32 + *bandE.offset((i + c * (*m).nbEBands) as isize));
            j = M * *eBands.offset(i as isize) as libc::c_int;
            while j < M * *eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int {
                *X.offset((j + c * N) as isize) = *freq.offset((j + c * N) as isize) * g;
                j += 1;
            }
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "196:1"]
pub unsafe extern "C" fn denormalise_bands(
    m: *const OpusCustomMode,
    X: *const celt_norm,
    freq: *mut celt_sig,
    bandLogE: *const opus_val16,
    mut start: libc::c_int,
    mut end: libc::c_int,
    M: libc::c_int,
    downsample: libc::c_int,
    silence: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut bound: libc::c_int = 0;
    let mut f: *mut celt_sig = 0 as *mut celt_sig;
    let mut x: *const celt_norm = 0 as *const celt_norm;
    let eBands: *const i16 = (*m).eBands;
    N = M * (*m).shortMdctSize;
    bound = M * *eBands.offset(end as isize) as libc::c_int;
    if downsample != 1 as libc::c_int {
        bound = if bound < N / downsample {
            bound
        } else {
            N / downsample
        };
    }
    if silence != 0 {
        bound = 0 as libc::c_int;
        end = 0 as libc::c_int;
        start = end;
    }
    f = freq;
    x = X.offset((M * *eBands.offset(start as isize) as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i < M * *eBands.offset(start as isize) as libc::c_int {
        let fresh0 = f;
        f = f.offset(1);
        *fresh0 = 0 as libc::c_int as celt_sig;
        i += 1;
    }
    i = start;
    while i < end {
        let mut j: libc::c_int = 0;
        let mut band_end: libc::c_int = 0;
        let mut g: opus_val16 = 0.;
        let mut lg: opus_val16 = 0.;
        j = M * *eBands.offset(i as isize) as libc::c_int;
        band_end = M * *eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int;
        lg = *bandLogE.offset(i as isize) + eMeans[i as usize];
        g = (std::f32::consts::LN_2 * (if 32.0 < lg { 32.0f32 } else { lg })).exp()
            as libc::c_float;
        loop {
            let fresh1 = x;
            x = x.offset(1);
            let fresh2 = f;
            f = f.offset(1);
            *fresh2 = *fresh1 * g;
            j += 1;
            if !(j < band_end) {
                break;
            }
        }
        i += 1;
    }
    if !(start <= end) {
        celt_fatal(
            b"assertion failed: start <= end\0" as *const u8 as *const libc::c_char,
            b"celt/bands.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int,
        );
    }
    memset(
        &mut *freq.offset(bound as isize) as *mut celt_sig as *mut libc::c_void,
        0 as libc::c_int,
        ((N - bound) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<celt_sig>() as libc::c_ulong),
    );
}
#[no_mangle]
#[c2rust::src_loc = "268:1"]
pub unsafe extern "C" fn anti_collapse(
    m: *const OpusCustomMode,
    X_: *mut celt_norm,
    collapse_masks: *mut libc::c_uchar,
    LM: libc::c_int,
    C: libc::c_int,
    size: libc::c_int,
    start: libc::c_int,
    end: libc::c_int,
    logE: *const opus_val16,
    prev1logE: *const opus_val16,
    prev2logE: *const opus_val16,
    pulses: *const libc::c_int,
    mut seed: u32,
    arch: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = start;
    while i < end {
        let mut N0: libc::c_int = 0;
        let mut thresh: opus_val16 = 0.;
        let mut sqrt_1: opus_val16 = 0.;
        let mut depth: libc::c_int = 0;
        N0 = *((*m).eBands).offset((i + 1 as libc::c_int) as isize) as libc::c_int
            - *((*m).eBands).offset(i as isize) as libc::c_int;
        depth = (celt_udiv(
            (1 as libc::c_int + *pulses.offset(i as isize)) as u32,
            (*((*m).eBands).offset((i + 1 as libc::c_int) as isize) as libc::c_int
                - *((*m).eBands).offset(i as isize) as libc::c_int) as u32,
        ) >> LM) as libc::c_int;
        thresh = 0.5f32 * (std::f32::consts::LN_2 * (-0.125f32 * depth as libc::c_float)).exp();
        sqrt_1 = 1.0f32 / ((N0 << LM) as f32).sqrt();
        c = 0 as libc::c_int;
        loop {
            let mut X: *mut celt_norm = 0 as *mut celt_norm;
            let mut prev1: opus_val16 = 0.;
            let mut prev2: opus_val16 = 0.;
            let mut Ediff: opus_val32 = 0.;
            let mut r: opus_val16 = 0.;
            let mut renormalize: libc::c_int = 0 as libc::c_int;
            prev1 = *prev1logE.offset((c * (*m).nbEBands + i) as isize);
            prev2 = *prev2logE.offset((c * (*m).nbEBands + i) as isize);
            if C == 1 as libc::c_int {
                prev1 = if prev1 > *prev1logE.offset(((*m).nbEBands + i) as isize) {
                    prev1
                } else {
                    *prev1logE.offset(((*m).nbEBands + i) as isize)
                };
                prev2 = if prev2 > *prev2logE.offset(((*m).nbEBands + i) as isize) {
                    prev2
                } else {
                    *prev2logE.offset(((*m).nbEBands + i) as isize)
                };
            }
            Ediff = *logE.offset((c * (*m).nbEBands + i) as isize)
                - (if prev1 < prev2 { prev1 } else { prev2 });
            Ediff = if 0 as libc::c_int as libc::c_float > Ediff {
                0 as libc::c_int as libc::c_float
            } else {
                Ediff
            };
            r = 2.0f32 * (std::f32::consts::LN_2 * -Ediff).exp();
            if LM == 3 as libc::c_int {
                r *= 1.41421356f32;
            }
            r = if thresh < r { thresh } else { r };
            r = r * sqrt_1;
            X = X_
                .offset((c * size) as isize)
                .offset(((*((*m).eBands).offset(i as isize) as libc::c_int) << LM) as isize);
            k = 0 as libc::c_int;
            while k < (1 as libc::c_int) << LM {
                if *collapse_masks.offset((i * C + c) as isize) as libc::c_int
                    & (1 as libc::c_int) << k
                    == 0
                {
                    j = 0 as libc::c_int;
                    while j < N0 {
                        seed = celt_lcg_rand(seed);
                        *X.offset(((j << LM) + k) as isize) =
                            if seed & 0x8000 as libc::c_int as libc::c_uint != 0 {
                                r
                            } else {
                                -r
                            };
                        j += 1;
                    }
                    renormalize = 1 as libc::c_int;
                }
                k += 1;
            }
            if renormalize != 0 {
                renormalise_vector(X, N0 << LM, Q15ONE, arch);
            }
            c += 1;
            if !(c < C) {
                break;
            }
        }
        i += 1;
    }
}
#[c2rust::src_loc = "371:1"]
unsafe extern "C" fn compute_channel_weights(
    mut Ex: celt_ener,
    mut Ey: celt_ener,
    w: *mut opus_val16,
) {
    let mut minE: celt_ener = 0.;
    minE = if Ex < Ey { Ex } else { Ey };
    Ex = Ex + minE / 3 as libc::c_int as libc::c_float;
    Ey = Ey + minE / 3 as libc::c_int as libc::c_float;
    *w.offset(0 as libc::c_int as isize) = Ex;
    *w.offset(1 as libc::c_int as isize) = Ey;
}
#[c2rust::src_loc = "388:1"]
unsafe extern "C" fn intensity_stereo(
    m: *const OpusCustomMode,
    X: *mut celt_norm,
    Y: *const celt_norm,
    bandE: *const celt_ener,
    bandID: libc::c_int,
    N: libc::c_int,
) {
    let i: libc::c_int = bandID;
    let mut j: libc::c_int = 0;
    let mut a1: opus_val16 = 0.;
    let mut a2: opus_val16 = 0.;
    let mut left: opus_val16 = 0.;
    let mut right: opus_val16 = 0.;
    let mut norm: opus_val16 = 0.;
    left = *bandE.offset(i as isize);
    right = *bandE.offset((i + (*m).nbEBands) as isize);
    norm = EPSILON + (1e-15f32 + left * left + right * right).sqrt();
    a1 = left / norm;
    a2 = right / norm;
    j = 0 as libc::c_int;
    while j < N {
        let mut r: celt_norm = 0.;
        let mut l: celt_norm = 0.;
        l = *X.offset(j as isize);
        r = *Y.offset(j as isize);
        *X.offset(j as isize) = a1 * l + a2 * r;
        j += 1;
    }
}
#[c2rust::src_loc = "413:1"]
unsafe extern "C" fn stereo_split(X: *mut celt_norm, Y: *mut celt_norm, N: libc::c_int) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < N {
        let mut r: opus_val32 = 0.;
        let mut l: opus_val32 = 0.;
        l = 0.70710678f32 * *X.offset(j as isize);
        r = 0.70710678f32 * *Y.offset(j as isize);
        *X.offset(j as isize) = l + r;
        *Y.offset(j as isize) = r - l;
        j += 1;
    }
}
#[c2rust::src_loc = "426:1"]
unsafe extern "C" fn stereo_merge(
    X: *mut celt_norm,
    Y: *mut celt_norm,
    mid: opus_val16,
    N: libc::c_int,
    _arch: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut xp: opus_val32 = 0 as libc::c_int as opus_val32;
    let mut side: opus_val32 = 0 as libc::c_int as opus_val32;
    let mut El: opus_val32 = 0.;
    let mut Er: opus_val32 = 0.;
    let mut mid2: opus_val16 = 0.;
    let mut t: opus_val32 = 0.;
    let mut lgain: opus_val32 = 0.;
    let mut rgain: opus_val32 = 0.;
    dual_inner_prod_c(Y, X, Y, N, &mut xp, &mut side);
    xp = mid * xp;
    mid2 = mid;
    El = mid2 * mid2 + side - 2 as libc::c_int as libc::c_float * xp;
    Er = mid2 * mid2 + side + 2 as libc::c_int as libc::c_float * xp;
    if Er < 6e-4f32 || El < 6e-4f32 {
        memcpy(
            Y as *mut libc::c_void,
            X as *const libc::c_void,
            (N as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long * Y.offset_from(X) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        return;
    }
    t = El;
    lgain = 1.0f32 / t.sqrt();
    t = Er;
    rgain = 1.0f32 / t.sqrt();
    j = 0 as libc::c_int;
    while j < N {
        let mut r: celt_norm = 0.;
        let mut l: celt_norm = 0.;
        l = mid * *X.offset(j as isize);
        r = *Y.offset(j as isize);
        *X.offset(j as isize) = lgain * (l - r);
        *Y.offset(j as isize) = rgain * (l + r);
        j += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "479:1"]
pub unsafe extern "C" fn spreading_decision(
    m: *const OpusCustomMode,
    X: *const celt_norm,
    average: *mut libc::c_int,
    last_decision: libc::c_int,
    hf_average: *mut libc::c_int,
    tapset_decision: *mut libc::c_int,
    update_hf: libc::c_int,
    end: libc::c_int,
    C: libc::c_int,
    M: libc::c_int,
    spread_weight: *const libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut N0: libc::c_int = 0;
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut nbBands: libc::c_int = 0 as libc::c_int;
    let eBands: *const i16 = (*m).eBands;
    let mut decision: libc::c_int = 0;
    let mut hf_sum: libc::c_int = 0 as libc::c_int;
    if !(end > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: end>0\0" as *const u8 as *const libc::c_char,
            b"celt/bands.c\0" as *const u8 as *const libc::c_char,
            489 as libc::c_int,
        );
    }
    N0 = M * (*m).shortMdctSize;
    if M * (*eBands.offset(end as isize) as libc::c_int
        - *eBands.offset((end - 1 as libc::c_int) as isize) as libc::c_int)
        <= 8 as libc::c_int
    {
        return SPREAD_NONE;
    }
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < end {
            let mut j: libc::c_int = 0;
            let mut N: libc::c_int = 0;
            let mut tmp: libc::c_int = 0 as libc::c_int;
            let mut tcount: [libc::c_int; 3] =
                [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
            let x: *const celt_norm = X
                .offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                .offset((c * N0) as isize);
            N = M
                * (*eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *eBands.offset(i as isize) as libc::c_int);
            if !(N <= 8 as libc::c_int) {
                j = 0 as libc::c_int;
                while j < N {
                    let mut x2N: opus_val32 = 0.;
                    x2N = *x.offset(j as isize) * *x.offset(j as isize) * N as opus_val32;
                    if x2N < 0.25f32 {
                        tcount[0 as libc::c_int as usize] += 1;
                    }
                    if x2N < 0.0625f32 {
                        tcount[1 as libc::c_int as usize] += 1;
                    }
                    if x2N < 0.015625f32 {
                        tcount[2 as libc::c_int as usize] += 1;
                    }
                    j += 1;
                }
                if i > (*m).nbEBands - 4 as libc::c_int {
                    hf_sum = (hf_sum as libc::c_uint).wrapping_add(celt_udiv(
                        (32 as libc::c_int
                            * (tcount[1 as libc::c_int as usize]
                                + tcount[0 as libc::c_int as usize]))
                            as u32,
                        N as u32,
                    )) as libc::c_int as libc::c_int;
                }
                tmp = (2 as libc::c_int * tcount[2 as libc::c_int as usize] >= N) as libc::c_int
                    + (2 as libc::c_int * tcount[1 as libc::c_int as usize] >= N) as libc::c_int
                    + (2 as libc::c_int * tcount[0 as libc::c_int as usize] >= N) as libc::c_int;
                sum += tmp * *spread_weight.offset(i as isize);
                nbBands += *spread_weight.offset(i as isize);
            }
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    if update_hf != 0 {
        if hf_sum != 0 {
            hf_sum = celt_udiv(
                hf_sum as u32,
                (C * (4 as libc::c_int - (*m).nbEBands + end)) as u32,
            ) as libc::c_int;
        }
        *hf_average = *hf_average + hf_sum >> 1 as libc::c_int;
        hf_sum = *hf_average;
        if *tapset_decision == 2 as libc::c_int {
            hf_sum += 4 as libc::c_int;
        } else if *tapset_decision == 0 as libc::c_int {
            hf_sum -= 4 as libc::c_int;
        }
        if hf_sum > 22 as libc::c_int {
            *tapset_decision = 2 as libc::c_int;
        } else if hf_sum > 18 as libc::c_int {
            *tapset_decision = 1 as libc::c_int;
        } else {
            *tapset_decision = 0 as libc::c_int;
        }
    }
    if !(nbBands > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: nbBands>0\0" as *const u8 as *const libc::c_char,
            b"celt/bands.c\0" as *const u8 as *const libc::c_char,
            545 as libc::c_int,
        );
    }
    if !(sum >= 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: sum>=0\0" as *const u8 as *const libc::c_char,
            b"celt/bands.c\0" as *const u8 as *const libc::c_char,
            546 as libc::c_int,
        );
    }
    sum = celt_udiv((sum << 8 as libc::c_int) as u32, nbBands as u32) as libc::c_int;
    sum = sum + *average >> 1 as libc::c_int;
    *average = sum;
    sum = 3 as libc::c_int * sum
        + ((3 as libc::c_int - last_decision << 7 as libc::c_int) + 64 as libc::c_int)
        + 2 as libc::c_int
        >> 2 as libc::c_int;
    if sum < 80 as libc::c_int {
        decision = SPREAD_AGGRESSIVE;
    } else if sum < 256 as libc::c_int {
        decision = SPREAD_NORMAL;
    } else if sum < 384 as libc::c_int {
        decision = SPREAD_LIGHT;
    } else {
        decision = SPREAD_NONE;
    }
    return decision;
}
#[c2rust::src_loc = "576:18"]
static mut ordery_table: [libc::c_int; 30] = [
    1 as libc::c_int,
    0 as libc::c_int,
    3 as libc::c_int,
    0 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    7 as libc::c_int,
    0 as libc::c_int,
    4 as libc::c_int,
    3 as libc::c_int,
    6 as libc::c_int,
    1 as libc::c_int,
    5 as libc::c_int,
    2 as libc::c_int,
    15 as libc::c_int,
    0 as libc::c_int,
    8 as libc::c_int,
    7 as libc::c_int,
    12 as libc::c_int,
    3 as libc::c_int,
    11 as libc::c_int,
    4 as libc::c_int,
    14 as libc::c_int,
    1 as libc::c_int,
    9 as libc::c_int,
    6 as libc::c_int,
    13 as libc::c_int,
    2 as libc::c_int,
    10 as libc::c_int,
    5 as libc::c_int,
];
#[c2rust::src_loc = "583:1"]
unsafe extern "C" fn deinterleave_hadamard(
    X: *mut celt_norm,
    N0: libc::c_int,
    stride: libc::c_int,
    hadamard: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    N = N0 * stride;
    let vla = N as usize;
    let mut tmp: Vec<celt_norm> = ::std::vec::from_elem(0., vla);
    if !(stride > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: stride>0\0" as *const u8 as *const libc::c_char,
            b"celt/bands.c\0" as *const u8 as *const libc::c_char,
            591 as libc::c_int,
        );
    }
    if hadamard != 0 {
        let ordery: *const libc::c_int = ordery_table
            .as_ptr()
            .offset(stride as isize)
            .offset(-(2 as libc::c_int as isize));
        i = 0 as libc::c_int;
        while i < stride {
            j = 0 as libc::c_int;
            while j < N0 {
                *tmp.as_mut_ptr()
                    .offset((*ordery.offset(i as isize) * N0 + j) as isize) =
                    *X.offset((j * stride + i) as isize);
                j += 1;
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < stride {
            j = 0 as libc::c_int;
            while j < N0 {
                *tmp.as_mut_ptr().offset((i * N0 + j) as isize) =
                    *X.offset((j * stride + i) as isize);
                j += 1;
            }
            i += 1;
        }
    }
    memcpy(
        X as *mut libc::c_void,
        tmp.as_mut_ptr() as *const libc::c_void,
        (N as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long * X.offset_from(tmp.as_mut_ptr()) as libc::c_long)
                    as libc::c_ulong,
            ),
    );
}
#[c2rust::src_loc = "609:1"]
unsafe extern "C" fn interleave_hadamard(
    X: *mut celt_norm,
    N0: libc::c_int,
    stride: libc::c_int,
    hadamard: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    N = N0 * stride;
    let vla = N as usize;
    let mut tmp: Vec<celt_norm> = ::std::vec::from_elem(0., vla);
    if hadamard != 0 {
        let ordery: *const libc::c_int = ordery_table
            .as_ptr()
            .offset(stride as isize)
            .offset(-(2 as libc::c_int as isize));
        i = 0 as libc::c_int;
        while i < stride {
            j = 0 as libc::c_int;
            while j < N0 {
                *tmp.as_mut_ptr().offset((j * stride + i) as isize) =
                    *X.offset((*ordery.offset(i as isize) * N0 + j) as isize);
                j += 1;
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < stride {
            j = 0 as libc::c_int;
            while j < N0 {
                *tmp.as_mut_ptr().offset((j * stride + i) as isize) =
                    *X.offset((i * N0 + j) as isize);
                j += 1;
            }
            i += 1;
        }
    }
    memcpy(
        X as *mut libc::c_void,
        tmp.as_mut_ptr() as *const libc::c_void,
        (N as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long * X.offset_from(tmp.as_mut_ptr()) as libc::c_long)
                    as libc::c_ulong,
            ),
    );
}
#[no_mangle]
#[c2rust::src_loc = "632:1"]
pub unsafe extern "C" fn haar1(X: *mut celt_norm, mut N0: libc::c_int, stride: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    N0 >>= 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < stride {
        j = 0 as libc::c_int;
        while j < N0 {
            let mut tmp1: opus_val32 = 0.;
            let mut tmp2: opus_val32 = 0.;
            tmp1 = 0.70710678f32 * *X.offset((stride * 2 as libc::c_int * j + i) as isize);
            tmp2 = 0.70710678f32
                * *X.offset((stride * (2 as libc::c_int * j + 1 as libc::c_int) + i) as isize);
            *X.offset((stride * 2 as libc::c_int * j + i) as isize) = tmp1 + tmp2;
            *X.offset((stride * (2 as libc::c_int * j + 1 as libc::c_int) + i) as isize) =
                tmp1 - tmp2;
            j += 1;
        }
        i += 1;
    }
}
#[c2rust::src_loc = "647:1"]
unsafe extern "C" fn compute_qn(
    N: libc::c_int,
    b: libc::c_int,
    offset: libc::c_int,
    pulse_cap: libc::c_int,
    stereo: libc::c_int,
) -> libc::c_int {
    static mut exp2_table8: [i16; 8] = [
        16384 as libc::c_int as i16,
        17866 as libc::c_int as i16,
        19483 as libc::c_int as i16,
        21247 as libc::c_int as i16,
        23170 as libc::c_int as i16,
        25267 as libc::c_int as i16,
        27554 as libc::c_int as i16,
        30048 as libc::c_int as i16,
    ];
    let mut qn: libc::c_int = 0;
    let mut qb: libc::c_int = 0;
    let mut N2: libc::c_int = 2 as libc::c_int * N - 1 as libc::c_int;
    if stereo != 0 && N == 2 as libc::c_int {
        N2 -= 1;
    }
    qb = celt_sudiv(b + N2 * offset, N2);
    qb = if b - pulse_cap - ((4 as libc::c_int) << 3 as libc::c_int) < qb {
        b - pulse_cap - ((4 as libc::c_int) << 3 as libc::c_int)
    } else {
        qb
    };
    qb = if ((8 as libc::c_int) << 3 as libc::c_int) < qb {
        (8 as libc::c_int) << 3 as libc::c_int
    } else {
        qb
    };
    if qb < (1 as libc::c_int) << BITRES >> 1 as libc::c_int {
        qn = 1 as libc::c_int;
    } else {
        qn = exp2_table8[(qb & 0x7 as libc::c_int) as usize] as libc::c_int
            >> 14 as libc::c_int - (qb >> BITRES);
        qn = (qn + 1 as libc::c_int >> 1 as libc::c_int) << 1 as libc::c_int;
    }
    if !(qn <= 256 as libc::c_int) {
        celt_fatal(
            b"assertion failed: qn <= 256\0" as *const u8 as *const libc::c_char,
            b"celt/bands.c\0" as *const u8 as *const libc::c_char,
            669 as libc::c_int,
        );
    }
    return qn;
}
#[c2rust::src_loc = "700:1"]
unsafe extern "C" fn compute_theta(
    ctx: *mut band_ctx,
    mut sctx: *mut split_ctx,
    X: *mut celt_norm,
    Y: *mut celt_norm,
    N: libc::c_int,
    b: *mut libc::c_int,
    B: libc::c_int,
    B0: libc::c_int,
    LM: libc::c_int,
    stereo: libc::c_int,
    fill: *mut libc::c_int,
) {
    let mut qn: libc::c_int = 0;
    let mut itheta: libc::c_int = 0 as libc::c_int;
    let mut delta: libc::c_int = 0;
    let mut imid: libc::c_int = 0;
    let mut iside: libc::c_int = 0;
    let mut qalloc: libc::c_int = 0;
    let mut pulse_cap: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut tell: i32 = 0;
    let mut inv: libc::c_int = 0 as libc::c_int;
    let mut encode: libc::c_int = 0;
    let mut m: *const OpusCustomMode = 0 as *const OpusCustomMode;
    let mut i: libc::c_int = 0;
    let mut intensity: libc::c_int = 0;
    let mut ec: *mut ec_ctx = 0 as *mut ec_ctx;
    let mut bandE: *const celt_ener = 0 as *const celt_ener;
    encode = (*ctx).encode;
    m = (*ctx).m;
    i = (*ctx).i;
    intensity = (*ctx).intensity;
    ec = (*ctx).ec;
    bandE = (*ctx).bandE;
    pulse_cap =
        *((*m).logN).offset(i as isize) as libc::c_int + LM * ((1 as libc::c_int) << BITRES);
    offset = (pulse_cap >> 1 as libc::c_int)
        - (if stereo != 0 && N == 2 as libc::c_int {
            QTHETA_OFFSET_TWOPHASE
        } else {
            QTHETA_OFFSET
        });
    qn = compute_qn(N, *b, offset, pulse_cap, stereo);
    if stereo != 0 && i >= intensity {
        qn = 1 as libc::c_int;
    }
    if encode != 0 {
        itheta = stereo_itheta(X, Y, stereo, N, (*ctx).arch);
    }
    tell = ec_tell_frac(ec) as i32;
    if qn != 1 as libc::c_int {
        if encode != 0 {
            if stereo == 0 || (*ctx).theta_round == 0 as libc::c_int {
                itheta = itheta * qn + 8192 as libc::c_int >> 14 as libc::c_int;
                if stereo == 0
                    && (*ctx).avoid_split_noise != 0
                    && itheta > 0 as libc::c_int
                    && itheta < qn
                {
                    let unquantized: libc::c_int =
                        celt_udiv((itheta * 16384 as libc::c_int) as u32, qn as u32) as libc::c_int;
                    imid = bitexact_cos(unquantized as i16) as libc::c_int;
                    iside =
                        bitexact_cos((16384 as libc::c_int - unquantized) as i16) as libc::c_int;
                    delta = 16384 as libc::c_int
                        + ((N - 1 as libc::c_int) << 7 as libc::c_int) as i16 as i32
                            * bitexact_log2tan(iside, imid) as i16 as libc::c_int
                        >> 15 as libc::c_int;
                    if delta > *b {
                        itheta = qn;
                    } else if delta < -*b {
                        itheta = 0 as libc::c_int;
                    }
                }
            } else {
                let mut down: libc::c_int = 0;
                let bias: libc::c_int = if itheta > 8192 as libc::c_int {
                    32767 as libc::c_int / qn
                } else {
                    -(32767 as libc::c_int) / qn
                };
                down = if (qn - 1 as libc::c_int)
                    < (if 0 as libc::c_int > itheta * qn + bias >> 14 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        itheta * qn + bias >> 14 as libc::c_int
                    }) {
                    qn - 1 as libc::c_int
                } else if 0 as libc::c_int > itheta * qn + bias >> 14 as libc::c_int {
                    0 as libc::c_int
                } else {
                    itheta * qn + bias >> 14 as libc::c_int
                };
                if (*ctx).theta_round < 0 as libc::c_int {
                    itheta = down;
                } else {
                    itheta = down + 1 as libc::c_int;
                }
            }
        }
        if stereo != 0 && N > 2 as libc::c_int {
            let p0: libc::c_int = 3 as libc::c_int;
            let mut x: libc::c_int = itheta;
            let x0: libc::c_int = qn / 2 as libc::c_int;
            let ft: libc::c_int = p0 * (x0 + 1 as libc::c_int) + x0;
            if encode != 0 {
                ec_encode(
                    ec,
                    (if x <= x0 {
                        p0 * x
                    } else {
                        x - 1 as libc::c_int - x0 + (x0 + 1 as libc::c_int) * p0
                    }) as libc::c_uint,
                    (if x <= x0 {
                        p0 * (x + 1 as libc::c_int)
                    } else {
                        x - x0 + (x0 + 1 as libc::c_int) * p0
                    }) as libc::c_uint,
                    ft as libc::c_uint,
                );
            } else {
                let mut fs: libc::c_int = 0;
                fs = ec_decode(ec, ft as libc::c_uint) as libc::c_int;
                if fs < (x0 + 1 as libc::c_int) * p0 {
                    x = fs / p0;
                } else {
                    x = x0 + 1 as libc::c_int + (fs - (x0 + 1 as libc::c_int) * p0);
                }
                ec_dec_update(
                    ec,
                    (if x <= x0 {
                        p0 * x
                    } else {
                        x - 1 as libc::c_int - x0 + (x0 + 1 as libc::c_int) * p0
                    }) as libc::c_uint,
                    (if x <= x0 {
                        p0 * (x + 1 as libc::c_int)
                    } else {
                        x - x0 + (x0 + 1 as libc::c_int) * p0
                    }) as libc::c_uint,
                    ft as libc::c_uint,
                );
                itheta = x;
            }
        } else if B0 > 1 as libc::c_int || stereo != 0 {
            if encode != 0 {
                ec_enc_uint(ec, itheta as u32, (qn + 1 as libc::c_int) as u32);
            } else {
                itheta = ec_dec_uint(ec, (qn + 1 as libc::c_int) as u32) as libc::c_int;
            }
        } else {
            let mut fs_0: libc::c_int = 1 as libc::c_int;
            let mut ft_0: libc::c_int = 0;
            ft_0 = ((qn >> 1 as libc::c_int) + 1 as libc::c_int)
                * ((qn >> 1 as libc::c_int) + 1 as libc::c_int);
            if encode != 0 {
                let mut fl: libc::c_int = 0;
                fs_0 = if itheta <= qn >> 1 as libc::c_int {
                    itheta + 1 as libc::c_int
                } else {
                    qn + 1 as libc::c_int - itheta
                };
                fl = if itheta <= qn >> 1 as libc::c_int {
                    itheta * (itheta + 1 as libc::c_int) >> 1 as libc::c_int
                } else {
                    ft_0 - ((qn + 1 as libc::c_int - itheta) * (qn + 2 as libc::c_int - itheta)
                        >> 1 as libc::c_int)
                };
                ec_encode(
                    ec,
                    fl as libc::c_uint,
                    (fl + fs_0) as libc::c_uint,
                    ft_0 as libc::c_uint,
                );
            } else {
                let mut fl_0: libc::c_int = 0 as libc::c_int;
                let mut fm: libc::c_int = 0;
                fm = ec_decode(ec, ft_0 as libc::c_uint) as libc::c_int;
                if fm
                    < (qn >> 1 as libc::c_int) * ((qn >> 1 as libc::c_int) + 1 as libc::c_int)
                        >> 1 as libc::c_int
                {
                    itheta = ((isqrt32(
                        (8 as libc::c_int as libc::c_uint)
                            .wrapping_mul(fm as u32)
                            .wrapping_add(1 as libc::c_int as libc::c_uint),
                    ))
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        >> 1 as libc::c_int) as libc::c_int;
                    fs_0 = itheta + 1 as libc::c_int;
                    fl_0 = itheta * (itheta + 1 as libc::c_int) >> 1 as libc::c_int;
                } else {
                    itheta = (((2 as libc::c_int * (qn + 1 as libc::c_int)) as libc::c_uint)
                        .wrapping_sub(isqrt32(
                            (8 as libc::c_int as libc::c_uint)
                                .wrapping_mul((ft_0 - fm - 1 as libc::c_int) as u32)
                                .wrapping_add(1 as libc::c_int as libc::c_uint),
                        ))
                        >> 1 as libc::c_int) as libc::c_int;
                    fs_0 = qn + 1 as libc::c_int - itheta;
                    fl_0 = ft_0
                        - ((qn + 1 as libc::c_int - itheta) * (qn + 2 as libc::c_int - itheta)
                            >> 1 as libc::c_int);
                }
                ec_dec_update(
                    ec,
                    fl_0 as libc::c_uint,
                    (fl_0 + fs_0) as libc::c_uint,
                    ft_0 as libc::c_uint,
                );
            }
        }
        if !(itheta >= 0 as libc::c_int) {
            celt_fatal(
                b"assertion failed: itheta>=0\0" as *const u8 as *const libc::c_char,
                b"celt/bands.c\0" as *const u8 as *const libc::c_char,
                838 as libc::c_int,
            );
        }
        itheta = celt_udiv((itheta * 16384 as libc::c_int) as u32, qn as u32) as libc::c_int;
        if encode != 0 && stereo != 0 {
            if itheta == 0 as libc::c_int {
                intensity_stereo(m, X, Y, bandE, i, N);
            } else {
                stereo_split(X, Y, N);
            }
        }
    } else if stereo != 0 {
        if encode != 0 {
            inv = (itheta > 8192 as libc::c_int && (*ctx).disable_inv == 0) as libc::c_int;
            if inv != 0 {
                let mut j: libc::c_int = 0;
                j = 0 as libc::c_int;
                while j < N {
                    *Y.offset(j as isize) = -*Y.offset(j as isize);
                    j += 1;
                }
            }
            intensity_stereo(m, X, Y, bandE, i, N);
        }
        if *b > (2 as libc::c_int) << BITRES && (*ctx).remaining_bits > (2 as libc::c_int) << BITRES
        {
            if encode != 0 {
                ec_enc_bit_logp(ec, inv, 2 as libc::c_int as libc::c_uint);
            } else {
                inv = ec_dec_bit_logp(ec, 2 as libc::c_int as libc::c_uint);
            }
        } else {
            inv = 0 as libc::c_int;
        }
        if (*ctx).disable_inv != 0 {
            inv = 0 as libc::c_int;
        }
        itheta = 0 as libc::c_int;
    }
    qalloc = (ec_tell_frac(ec)).wrapping_sub(tell as libc::c_uint) as libc::c_int;
    *b -= qalloc;
    if itheta == 0 as libc::c_int {
        imid = 32767 as libc::c_int;
        iside = 0 as libc::c_int;
        *fill &= ((1 as libc::c_int) << B) - 1 as libc::c_int;
        delta = -(16384 as libc::c_int);
    } else if itheta == 16384 as libc::c_int {
        imid = 0 as libc::c_int;
        iside = 32767 as libc::c_int;
        *fill &= (((1 as libc::c_int) << B) - 1 as libc::c_int) << B;
        delta = 16384 as libc::c_int;
    } else {
        imid = bitexact_cos(itheta as i16) as libc::c_int;
        iside = bitexact_cos((16384 as libc::c_int - itheta) as i16) as libc::c_int;
        delta = 16384 as libc::c_int
            + ((N - 1 as libc::c_int) << 7 as libc::c_int) as i16 as i32
                * bitexact_log2tan(iside, imid) as i16 as libc::c_int
            >> 15 as libc::c_int;
    }
    (*sctx).inv = inv;
    (*sctx).imid = imid;
    (*sctx).iside = iside;
    (*sctx).delta = delta;
    (*sctx).itheta = itheta;
    (*sctx).qalloc = qalloc;
}
#[c2rust::src_loc = "904:1"]
unsafe extern "C" fn quant_band_n1(
    mut ctx: *mut band_ctx,
    X: *mut celt_norm,
    Y: *mut celt_norm,
    mut b: libc::c_int,
    lowband_out: *mut celt_norm,
) -> libc::c_uint {
    let mut c: libc::c_int = 0;
    let mut stereo: libc::c_int = 0;
    let mut x: *mut celt_norm = X;
    let mut encode: libc::c_int = 0;
    let mut ec: *mut ec_ctx = 0 as *mut ec_ctx;
    encode = (*ctx).encode;
    ec = (*ctx).ec;
    stereo = (Y != NULL as *mut celt_norm) as libc::c_int;
    c = 0 as libc::c_int;
    loop {
        let mut sign: libc::c_int = 0 as libc::c_int;
        if (*ctx).remaining_bits >= (1 as libc::c_int) << BITRES {
            if encode != 0 {
                sign = (*x.offset(0 as libc::c_int as isize) < 0 as libc::c_int as libc::c_float)
                    as libc::c_int;
                ec_enc_bits(ec, sign as u32, 1 as libc::c_int as libc::c_uint);
            } else {
                sign = ec_dec_bits(ec, 1 as libc::c_int as libc::c_uint) as libc::c_int;
            }
            (*ctx).remaining_bits -= (1 as libc::c_int) << BITRES;
            b -= (1 as libc::c_int) << BITRES;
        }
        if (*ctx).resynth != 0 {
            *x.offset(0 as libc::c_int as isize) = if sign != 0 {
                -NORM_SCALING
            } else {
                NORM_SCALING
            };
        }
        x = Y;
        c += 1;
        if !(c < 1 as libc::c_int + stereo) {
            break;
        }
    }
    if !lowband_out.is_null() {
        *lowband_out.offset(0 as libc::c_int as isize) = *X.offset(0 as libc::c_int as isize);
    }
    return 1 as libc::c_int as libc::c_uint;
}
#[c2rust::src_loc = "944:1"]
unsafe extern "C" fn quant_partition(
    mut ctx: *mut band_ctx,
    X: *mut celt_norm,
    mut N: libc::c_int,
    mut b: libc::c_int,
    mut B: libc::c_int,
    lowband: *mut celt_norm,
    mut LM: libc::c_int,
    gain: opus_val16,
    mut fill: libc::c_int,
) -> libc::c_uint {
    let mut cache: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut q: libc::c_int = 0;
    let mut curr_bits: libc::c_int = 0;
    let mut imid: libc::c_int = 0 as libc::c_int;
    let mut iside: libc::c_int = 0 as libc::c_int;
    let B0: libc::c_int = B;
    let mut mid: opus_val16 = 0 as libc::c_int as opus_val16;
    let mut side: opus_val16 = 0 as libc::c_int as opus_val16;
    let mut cm: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut Y: *mut celt_norm = NULL as *mut celt_norm;
    let mut encode: libc::c_int = 0;
    let mut m: *const OpusCustomMode = 0 as *const OpusCustomMode;
    let mut i: libc::c_int = 0;
    let mut spread: libc::c_int = 0;
    let mut ec: *mut ec_ctx = 0 as *mut ec_ctx;
    encode = (*ctx).encode;
    m = (*ctx).m;
    i = (*ctx).i;
    spread = (*ctx).spread;
    ec = (*ctx).ec;
    cache = ((*m).cache.bits).offset(
        *((*m).cache.index).offset(((LM + 1 as libc::c_int) * (*m).nbEBands + i) as isize)
            as libc::c_int as isize,
    );
    if LM != -(1 as libc::c_int)
        && b > *cache.offset(*cache.offset(0 as libc::c_int as isize) as isize) as libc::c_int
            + 12 as libc::c_int
        && N > 2 as libc::c_int
    {
        let mut mbits: libc::c_int = 0;
        let mut sbits: libc::c_int = 0;
        let mut delta: libc::c_int = 0;
        let mut itheta: libc::c_int = 0;
        let mut qalloc: libc::c_int = 0;
        let mut sctx: split_ctx = split_ctx {
            inv: 0,
            imid: 0,
            iside: 0,
            delta: 0,
            itheta: 0,
            qalloc: 0,
        };
        let mut next_lowband2: *mut celt_norm = NULL as *mut celt_norm;
        let mut rebalance: i32 = 0;
        N >>= 1 as libc::c_int;
        Y = X.offset(N as isize);
        LM -= 1 as libc::c_int;
        if B == 1 as libc::c_int {
            fill = fill & 1 as libc::c_int | fill << 1 as libc::c_int;
        }
        B = B + 1 as libc::c_int >> 1 as libc::c_int;
        compute_theta(
            ctx,
            &mut sctx,
            X,
            Y,
            N,
            &mut b,
            B,
            B0,
            LM,
            0 as libc::c_int,
            &mut fill,
        );
        imid = sctx.imid;
        iside = sctx.iside;
        delta = sctx.delta;
        itheta = sctx.itheta;
        qalloc = sctx.qalloc;
        mid = 1.0f32 / 32768 as libc::c_int as libc::c_float * imid as libc::c_float;
        side = 1.0f32 / 32768 as libc::c_int as libc::c_float * iside as libc::c_float;
        if B0 > 1 as libc::c_int && itheta & 0x3fff as libc::c_int != 0 {
            if itheta > 8192 as libc::c_int {
                delta -= delta >> 4 as libc::c_int - LM;
            } else {
                delta = if (0 as libc::c_int)
                    < delta + (N << 3 as libc::c_int >> 5 as libc::c_int - LM)
                {
                    0 as libc::c_int
                } else {
                    delta + (N << 3 as libc::c_int >> 5 as libc::c_int - LM)
                };
            }
        }
        mbits = if 0 as libc::c_int
            > (if b < (b - delta) / 2 as libc::c_int {
                b
            } else {
                (b - delta) / 2 as libc::c_int
            }) {
            0 as libc::c_int
        } else if b < (b - delta) / 2 as libc::c_int {
            b
        } else {
            (b - delta) / 2 as libc::c_int
        };
        sbits = b - mbits;
        (*ctx).remaining_bits -= qalloc;
        if !lowband.is_null() {
            next_lowband2 = lowband.offset(N as isize);
        }
        rebalance = (*ctx).remaining_bits;
        if mbits >= sbits {
            cm = quant_partition(ctx, X, N, mbits, B, lowband, LM, gain * mid, fill);
            rebalance = mbits - (rebalance - (*ctx).remaining_bits);
            if rebalance > (3 as libc::c_int) << BITRES && itheta != 0 as libc::c_int {
                sbits += rebalance - ((3 as libc::c_int) << BITRES);
            }
            cm |= quant_partition(
                ctx,
                Y,
                N,
                sbits,
                B,
                next_lowband2,
                LM,
                gain * side,
                fill >> B,
            ) << (B0 >> 1 as libc::c_int);
        } else {
            cm = quant_partition(
                ctx,
                Y,
                N,
                sbits,
                B,
                next_lowband2,
                LM,
                gain * side,
                fill >> B,
            ) << (B0 >> 1 as libc::c_int);
            rebalance = sbits - (rebalance - (*ctx).remaining_bits);
            if rebalance > (3 as libc::c_int) << BITRES && itheta != 16384 as libc::c_int {
                mbits += rebalance - ((3 as libc::c_int) << BITRES);
            }
            cm |= quant_partition(ctx, X, N, mbits, B, lowband, LM, gain * mid, fill);
        }
    } else {
        q = bits2pulses(m, i, LM, b);
        curr_bits = pulses2bits(m, i, LM, q);
        (*ctx).remaining_bits -= curr_bits;
        while (*ctx).remaining_bits < 0 as libc::c_int && q > 0 as libc::c_int {
            (*ctx).remaining_bits += curr_bits;
            q -= 1;
            curr_bits = pulses2bits(m, i, LM, q);
            (*ctx).remaining_bits -= curr_bits;
        }
        if q != 0 as libc::c_int {
            let K: libc::c_int = get_pulses(q);
            if encode != 0 {
                cm = alg_quant(X, N, K, spread, B, ec, gain, (*ctx).resynth, (*ctx).arch);
            } else {
                cm = alg_unquant(X, N, K, spread, B, ec, gain);
            }
        } else {
            let mut j: libc::c_int = 0;
            if (*ctx).resynth != 0 {
                let mut cm_mask: libc::c_uint = 0;
                cm_mask = (((1 as libc::c_ulong) << B) as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint);
                fill = (fill as libc::c_uint & cm_mask) as libc::c_int;
                if fill == 0 {
                    memset(
                        X as *mut libc::c_void,
                        0 as libc::c_int,
                        (N as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong),
                    );
                } else {
                    if lowband.is_null() {
                        j = 0 as libc::c_int;
                        while j < N {
                            (*ctx).seed = celt_lcg_rand((*ctx).seed);
                            *X.offset(j as isize) =
                                ((*ctx).seed as i32 >> 20 as libc::c_int) as celt_norm;
                            j += 1;
                        }
                        cm = cm_mask;
                    } else {
                        j = 0 as libc::c_int;
                        while j < N {
                            let mut tmp: opus_val16 = 0.;
                            (*ctx).seed = celt_lcg_rand((*ctx).seed);
                            tmp = 1.0f32 / 256 as libc::c_int as libc::c_float;
                            tmp = if (*ctx).seed & 0x8000 as libc::c_int as libc::c_uint != 0 {
                                tmp
                            } else {
                                -tmp
                            };
                            *X.offset(j as isize) = *lowband.offset(j as isize) + tmp;
                            j += 1;
                        }
                        cm = fill as libc::c_uint;
                    }
                    renormalise_vector(X, N, gain, (*ctx).arch);
                }
            }
        }
    }
    return cm;
}
#[c2rust::src_loc = "1110:1"]
unsafe extern "C" fn quant_band(
    ctx: *mut band_ctx,
    X: *mut celt_norm,
    N: libc::c_int,
    b: libc::c_int,
    mut B: libc::c_int,
    mut lowband: *mut celt_norm,
    LM: libc::c_int,
    lowband_out: *mut celt_norm,
    gain: opus_val16,
    lowband_scratch: *mut celt_norm,
    mut fill: libc::c_int,
) -> libc::c_uint {
    let N0: libc::c_int = N;
    let mut N_B: libc::c_int = N;
    let mut N_B0: libc::c_int = 0;
    let mut B0: libc::c_int = B;
    let mut time_divide: libc::c_int = 0 as libc::c_int;
    let mut recombine: libc::c_int = 0 as libc::c_int;
    let mut longBlocks: libc::c_int = 0;
    let mut cm: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut k: libc::c_int = 0;
    let mut encode: libc::c_int = 0;
    let mut tf_change: libc::c_int = 0;
    encode = (*ctx).encode;
    tf_change = (*ctx).tf_change;
    longBlocks = (B0 == 1 as libc::c_int) as libc::c_int;
    N_B = celt_udiv(N_B as u32, B as u32) as libc::c_int;
    if N == 1 as libc::c_int {
        return quant_band_n1(ctx, X, NULL as *mut celt_norm, b, lowband_out);
    }
    if tf_change > 0 as libc::c_int {
        recombine = tf_change;
    }
    if !lowband_scratch.is_null()
        && !lowband.is_null()
        && (recombine != 0
            || N_B & 1 as libc::c_int == 0 as libc::c_int && tf_change < 0 as libc::c_int
            || B0 > 1 as libc::c_int)
    {
        memcpy(
            lowband_scratch as *mut libc::c_void,
            lowband as *const libc::c_void,
            (N as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * lowband_scratch.offset_from(lowband) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        lowband = lowband_scratch;
    }
    k = 0 as libc::c_int;
    while k < recombine {
        static mut bit_interleave_table: [libc::c_uchar; 16] = [
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
        ];
        if encode != 0 {
            haar1(X, N >> k, (1 as libc::c_int) << k);
        }
        if !lowband.is_null() {
            haar1(lowband, N >> k, (1 as libc::c_int) << k);
        }
        fill = bit_interleave_table[(fill & 0xf as libc::c_int) as usize] as libc::c_int
            | (bit_interleave_table[(fill >> 4 as libc::c_int) as usize] as libc::c_int)
                << 2 as libc::c_int;
        k += 1;
    }
    B >>= recombine;
    N_B <<= recombine;
    while N_B & 1 as libc::c_int == 0 as libc::c_int && tf_change < 0 as libc::c_int {
        if encode != 0 {
            haar1(X, N_B, B);
        }
        if !lowband.is_null() {
            haar1(lowband, N_B, B);
        }
        fill |= fill << B;
        B <<= 1 as libc::c_int;
        N_B >>= 1 as libc::c_int;
        time_divide += 1;
        tf_change += 1;
    }
    B0 = B;
    N_B0 = N_B;
    if B0 > 1 as libc::c_int {
        if encode != 0 {
            deinterleave_hadamard(X, N_B >> recombine, B0 << recombine, longBlocks);
        }
        if !lowband.is_null() {
            deinterleave_hadamard(lowband, N_B >> recombine, B0 << recombine, longBlocks);
        }
    }
    cm = quant_partition(ctx, X, N, b, B, lowband, LM, gain, fill);
    if (*ctx).resynth != 0 {
        if B0 > 1 as libc::c_int {
            interleave_hadamard(X, N_B >> recombine, B0 << recombine, longBlocks);
        }
        N_B = N_B0;
        B = B0;
        k = 0 as libc::c_int;
        while k < time_divide {
            B >>= 1 as libc::c_int;
            N_B <<= 1 as libc::c_int;
            cm |= cm >> B;
            haar1(X, N_B, B);
            k += 1;
        }
        k = 0 as libc::c_int;
        while k < recombine {
            static mut bit_deinterleave_table: [libc::c_uchar; 16] = [
                0 as libc::c_int as libc::c_uchar,
                0x3 as libc::c_int as libc::c_uchar,
                0xc as libc::c_int as libc::c_uchar,
                0xf as libc::c_int as libc::c_uchar,
                0x30 as libc::c_int as libc::c_uchar,
                0x33 as libc::c_int as libc::c_uchar,
                0x3c as libc::c_int as libc::c_uchar,
                0x3f as libc::c_int as libc::c_uchar,
                0xc0 as libc::c_int as libc::c_uchar,
                0xc3 as libc::c_int as libc::c_uchar,
                0xcc as libc::c_int as libc::c_uchar,
                0xcf as libc::c_int as libc::c_uchar,
                0xf0 as libc::c_int as libc::c_uchar,
                0xf3 as libc::c_int as libc::c_uchar,
                0xfc as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
            ];
            cm = bit_deinterleave_table[cm as usize] as libc::c_uint;
            haar1(X, N0 >> k, (1 as libc::c_int) << k);
            k += 1;
        }
        B <<= recombine;
        if !lowband_out.is_null() {
            let mut j: libc::c_int = 0;
            let mut n: opus_val16 = 0.;
            n = (N0 as f32).sqrt();
            j = 0 as libc::c_int;
            while j < N0 {
                *lowband_out.offset(j as isize) = n * *X.offset(j as isize);
                j += 1;
            }
        }
        cm &= (((1 as libc::c_int) << B) - 1 as libc::c_int) as libc::c_uint;
    }
    return cm;
}
#[c2rust::src_loc = "1236:1"]
unsafe extern "C" fn quant_band_stereo(
    mut ctx: *mut band_ctx,
    X: *mut celt_norm,
    Y: *mut celt_norm,
    N: libc::c_int,
    mut b: libc::c_int,
    B: libc::c_int,
    lowband: *mut celt_norm,
    LM: libc::c_int,
    lowband_out: *mut celt_norm,
    lowband_scratch: *mut celt_norm,
    mut fill: libc::c_int,
) -> libc::c_uint {
    let mut imid: libc::c_int = 0 as libc::c_int;
    let mut iside: libc::c_int = 0 as libc::c_int;
    let mut inv: libc::c_int = 0 as libc::c_int;
    let mut mid: opus_val16 = 0 as libc::c_int as opus_val16;
    let mut side: opus_val16 = 0 as libc::c_int as opus_val16;
    let mut cm: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut mbits: libc::c_int = 0;
    let mut sbits: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut itheta: libc::c_int = 0;
    let mut qalloc: libc::c_int = 0;
    let mut sctx: split_ctx = split_ctx {
        inv: 0,
        imid: 0,
        iside: 0,
        delta: 0,
        itheta: 0,
        qalloc: 0,
    };
    let mut orig_fill: libc::c_int = 0;
    let mut encode: libc::c_int = 0;
    let mut ec: *mut ec_ctx = 0 as *mut ec_ctx;
    encode = (*ctx).encode;
    ec = (*ctx).ec;
    if N == 1 as libc::c_int {
        return quant_band_n1(ctx, X, Y, b, lowband_out);
    }
    orig_fill = fill;
    compute_theta(
        ctx,
        &mut sctx,
        X,
        Y,
        N,
        &mut b,
        B,
        B,
        LM,
        1 as libc::c_int,
        &mut fill,
    );
    inv = sctx.inv;
    imid = sctx.imid;
    iside = sctx.iside;
    delta = sctx.delta;
    itheta = sctx.itheta;
    qalloc = sctx.qalloc;
    mid = 1.0f32 / 32768 as libc::c_int as libc::c_float * imid as libc::c_float;
    side = 1.0f32 / 32768 as libc::c_int as libc::c_float * iside as libc::c_float;
    if N == 2 as libc::c_int {
        let mut c: libc::c_int = 0;
        let mut sign: libc::c_int = 0 as libc::c_int;
        let mut x2: *mut celt_norm = 0 as *mut celt_norm;
        let mut y2: *mut celt_norm = 0 as *mut celt_norm;
        mbits = b;
        sbits = 0 as libc::c_int;
        if itheta != 0 as libc::c_int && itheta != 16384 as libc::c_int {
            sbits = (1 as libc::c_int) << BITRES;
        }
        mbits -= sbits;
        c = (itheta > 8192 as libc::c_int) as libc::c_int;
        (*ctx).remaining_bits -= qalloc + sbits;
        x2 = if c != 0 { Y } else { X };
        y2 = if c != 0 { X } else { Y };
        if sbits != 0 {
            if encode != 0 {
                sign = (*x2.offset(0 as libc::c_int as isize)
                    * *y2.offset(1 as libc::c_int as isize)
                    - *x2.offset(1 as libc::c_int as isize) * *y2.offset(0 as libc::c_int as isize)
                    < 0 as libc::c_int as libc::c_float) as libc::c_int;
                ec_enc_bits(ec, sign as u32, 1 as libc::c_int as libc::c_uint);
            } else {
                sign = ec_dec_bits(ec, 1 as libc::c_int as libc::c_uint) as libc::c_int;
            }
        }
        sign = 1 as libc::c_int - 2 as libc::c_int * sign;
        cm = quant_band(
            ctx,
            x2,
            N,
            mbits,
            B,
            lowband,
            LM,
            lowband_out,
            Q15ONE,
            lowband_scratch,
            orig_fill,
        );
        *y2.offset(0 as libc::c_int as isize) =
            -sign as libc::c_float * *x2.offset(1 as libc::c_int as isize);
        *y2.offset(1 as libc::c_int as isize) =
            sign as libc::c_float * *x2.offset(0 as libc::c_int as isize);
        if (*ctx).resynth != 0 {
            let mut tmp: celt_norm = 0.;
            *X.offset(0 as libc::c_int as isize) = mid * *X.offset(0 as libc::c_int as isize);
            *X.offset(1 as libc::c_int as isize) = mid * *X.offset(1 as libc::c_int as isize);
            *Y.offset(0 as libc::c_int as isize) = side * *Y.offset(0 as libc::c_int as isize);
            *Y.offset(1 as libc::c_int as isize) = side * *Y.offset(1 as libc::c_int as isize);
            tmp = *X.offset(0 as libc::c_int as isize);
            *X.offset(0 as libc::c_int as isize) = tmp - *Y.offset(0 as libc::c_int as isize);
            *Y.offset(0 as libc::c_int as isize) = tmp + *Y.offset(0 as libc::c_int as isize);
            tmp = *X.offset(1 as libc::c_int as isize);
            *X.offset(1 as libc::c_int as isize) = tmp - *Y.offset(1 as libc::c_int as isize);
            *Y.offset(1 as libc::c_int as isize) = tmp + *Y.offset(1 as libc::c_int as isize);
        }
    } else {
        let mut rebalance: i32 = 0;
        mbits = if 0 as libc::c_int
            > (if b < (b - delta) / 2 as libc::c_int {
                b
            } else {
                (b - delta) / 2 as libc::c_int
            }) {
            0 as libc::c_int
        } else if b < (b - delta) / 2 as libc::c_int {
            b
        } else {
            (b - delta) / 2 as libc::c_int
        };
        sbits = b - mbits;
        (*ctx).remaining_bits -= qalloc;
        rebalance = (*ctx).remaining_bits;
        if mbits >= sbits {
            cm = quant_band(
                ctx,
                X,
                N,
                mbits,
                B,
                lowband,
                LM,
                lowband_out,
                Q15ONE,
                lowband_scratch,
                fill,
            );
            rebalance = mbits - (rebalance - (*ctx).remaining_bits);
            if rebalance > (3 as libc::c_int) << BITRES && itheta != 0 as libc::c_int {
                sbits += rebalance - ((3 as libc::c_int) << BITRES);
            }
            cm |= quant_band(
                ctx,
                Y,
                N,
                sbits,
                B,
                NULL as *mut celt_norm,
                LM,
                NULL as *mut celt_norm,
                side,
                NULL as *mut celt_norm,
                fill >> B,
            );
        } else {
            cm = quant_band(
                ctx,
                Y,
                N,
                sbits,
                B,
                NULL as *mut celt_norm,
                LM,
                NULL as *mut celt_norm,
                side,
                NULL as *mut celt_norm,
                fill >> B,
            );
            rebalance = sbits - (rebalance - (*ctx).remaining_bits);
            if rebalance > (3 as libc::c_int) << BITRES && itheta != 16384 as libc::c_int {
                mbits += rebalance - ((3 as libc::c_int) << BITRES);
            }
            cm |= quant_band(
                ctx,
                X,
                N,
                mbits,
                B,
                lowband,
                LM,
                lowband_out,
                Q15ONE,
                lowband_scratch,
                fill,
            );
        }
    }
    if (*ctx).resynth != 0 {
        if N != 2 as libc::c_int {
            stereo_merge(X, Y, mid, N, (*ctx).arch);
        }
        if inv != 0 {
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while j < N {
                *Y.offset(j as isize) = -*Y.offset(j as isize);
                j += 1;
            }
        }
    }
    return cm;
}
#[c2rust::src_loc = "1384:1"]
unsafe extern "C" fn special_hybrid_folding(
    m: *const OpusCustomMode,
    norm: *mut celt_norm,
    norm2: *mut celt_norm,
    start: libc::c_int,
    M: libc::c_int,
    dual_stereo: libc::c_int,
) {
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let eBands: *const i16 = (*m).eBands;
    n1 = M
        * (*eBands.offset((start + 1 as libc::c_int) as isize) as libc::c_int
            - *eBands.offset(start as isize) as libc::c_int);
    n2 = M
        * (*eBands.offset((start + 2 as libc::c_int) as isize) as libc::c_int
            - *eBands.offset((start + 1 as libc::c_int) as isize) as libc::c_int);
    memcpy(
        &mut *norm.offset(n1 as isize) as *mut celt_norm as *mut libc::c_void,
        &mut *norm.offset((2 as libc::c_int * n1 - n2) as isize) as *mut celt_norm
            as *const libc::c_void,
        ((n2 - n1) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * (&mut *norm.offset(n1 as isize) as *mut celt_norm)
                        .offset_from(&mut *norm.offset((2 as libc::c_int * n1 - n2) as isize))
                        as libc::c_long) as libc::c_ulong,
            ),
    );
    if dual_stereo != 0 {
        memcpy(
            &mut *norm2.offset(n1 as isize) as *mut celt_norm as *mut libc::c_void,
            &mut *norm2.offset((2 as libc::c_int * n1 - n2) as isize) as *mut celt_norm
                as *const libc::c_void,
            ((n2 - n1) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (&mut *norm2.offset(n1 as isize) as *mut celt_norm)
                            .offset_from(&mut *norm2.offset((2 as libc::c_int * n1 - n2) as isize))
                            as libc::c_long) as libc::c_ulong,
                ),
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "1397:1"]
pub unsafe extern "C" fn quant_all_bands(
    encode: libc::c_int,
    m: *const OpusCustomMode,
    start: libc::c_int,
    end: libc::c_int,
    X_: *mut celt_norm,
    Y_: *mut celt_norm,
    collapse_masks: *mut libc::c_uchar,
    bandE: *const celt_ener,
    pulses: *mut libc::c_int,
    shortBlocks: libc::c_int,
    spread: libc::c_int,
    mut dual_stereo: libc::c_int,
    intensity: libc::c_int,
    tf_res: *mut libc::c_int,
    total_bits: i32,
    mut balance: i32,
    ec: *mut ec_ctx,
    LM: libc::c_int,
    codedBands: libc::c_int,
    seed: *mut u32,
    complexity: libc::c_int,
    arch: libc::c_int,
    disable_inv: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut remaining_bits: i32 = 0;
    let eBands: *const i16 = (*m).eBands;
    let mut norm: *mut celt_norm = 0 as *mut celt_norm;
    let mut norm2: *mut celt_norm = 0 as *mut celt_norm;
    let mut resynth_alloc: libc::c_int = 0;
    let mut lowband_scratch: *mut celt_norm = 0 as *mut celt_norm;
    let mut B: libc::c_int = 0;
    let mut M: libc::c_int = 0;
    let mut lowband_offset: libc::c_int = 0;
    let mut update_lowband: libc::c_int = 1 as libc::c_int;
    let C: libc::c_int = if !Y_.is_null() {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let mut norm_offset: libc::c_int = 0;
    let theta_rdo: libc::c_int =
        (encode != 0 && !Y_.is_null() && dual_stereo == 0 && complexity >= 8 as libc::c_int)
            as libc::c_int;
    let resynth: libc::c_int = (encode == 0 || theta_rdo != 0) as libc::c_int;
    let mut ctx: band_ctx = band_ctx {
        encode: 0,
        resynth: 0,
        m: 0 as *const OpusCustomMode,
        i: 0,
        intensity: 0,
        spread: 0,
        tf_change: 0,
        ec: 0 as *mut ec_ctx,
        remaining_bits: 0,
        bandE: 0 as *const celt_ener,
        seed: 0,
        arch: 0,
        theta_round: 0,
        disable_inv: 0,
        avoid_split_noise: 0,
    };
    M = (1 as libc::c_int) << LM;
    B = if shortBlocks != 0 {
        M
    } else {
        1 as libc::c_int
    };
    norm_offset = M * *eBands.offset(start as isize) as libc::c_int;
    let vla = (C
        * (M * *eBands.offset(((*m).nbEBands - 1 as libc::c_int) as isize) as libc::c_int
            - norm_offset)) as usize;
    let mut _norm: Vec<celt_norm> = ::std::vec::from_elem(0., vla);
    norm = _norm.as_mut_ptr();
    norm2 = norm
        .offset(
            (M * *eBands.offset(((*m).nbEBands - 1 as libc::c_int) as isize) as libc::c_int)
                as isize,
        )
        .offset(-(norm_offset as isize));
    if encode != 0 && resynth != 0 {
        resynth_alloc = M
            * (*eBands.offset((*m).nbEBands as isize) as libc::c_int
                - *eBands.offset(((*m).nbEBands - 1 as libc::c_int) as isize) as libc::c_int);
    } else {
        resynth_alloc = ALLOC_NONE;
    }
    let vla_0 = resynth_alloc as usize;
    let mut _lowband_scratch: Vec<celt_norm> = ::std::vec::from_elem(0., vla_0);
    if encode != 0 && resynth != 0 {
        lowband_scratch = _lowband_scratch.as_mut_ptr();
    } else {
        lowband_scratch = X_.offset(
            (M * *eBands.offset(((*m).nbEBands - 1 as libc::c_int) as isize) as libc::c_int)
                as isize,
        );
    }
    let vla_1 = resynth_alloc as usize;
    let mut X_save: Vec<celt_norm> = ::std::vec::from_elem(0., vla_1);
    let vla_2 = resynth_alloc as usize;
    let mut Y_save: Vec<celt_norm> = ::std::vec::from_elem(0., vla_2);
    let vla_3 = resynth_alloc as usize;
    let mut X_save2: Vec<celt_norm> = ::std::vec::from_elem(0., vla_3);
    let vla_4 = resynth_alloc as usize;
    let mut Y_save2: Vec<celt_norm> = ::std::vec::from_elem(0., vla_4);
    let vla_5 = resynth_alloc as usize;
    let mut norm_save2: Vec<celt_norm> = ::std::vec::from_elem(0., vla_5);
    lowband_offset = 0 as libc::c_int;
    ctx.bandE = bandE;
    ctx.ec = ec;
    ctx.encode = encode;
    ctx.intensity = intensity;
    ctx.m = m;
    ctx.seed = *seed;
    ctx.spread = spread;
    ctx.arch = arch;
    ctx.disable_inv = disable_inv;
    ctx.resynth = resynth;
    ctx.theta_round = 0 as libc::c_int;
    ctx.avoid_split_noise = (B > 1 as libc::c_int) as libc::c_int;
    i = start;
    while i < end {
        let mut tell: i32 = 0;
        let mut b: libc::c_int = 0;
        let mut N: libc::c_int = 0;
        let mut curr_balance: i32 = 0;
        let mut effective_lowband: libc::c_int = -(1 as libc::c_int);
        let mut X: *mut celt_norm = 0 as *mut celt_norm;
        let mut Y: *mut celt_norm = 0 as *mut celt_norm;
        let mut tf_change: libc::c_int = 0 as libc::c_int;
        let mut x_cm: libc::c_uint = 0;
        let mut y_cm: libc::c_uint = 0;
        let mut last: libc::c_int = 0;
        ctx.i = i;
        last = (i == end - 1 as libc::c_int) as libc::c_int;
        X = X_.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize);
        if !Y_.is_null() {
            Y = Y_.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize);
        } else {
            Y = NULL as *mut celt_norm;
        }
        N = M * *eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            - M * *eBands.offset(i as isize) as libc::c_int;
        if !(N > 0 as libc::c_int) {
            celt_fatal(
                b"assertion failed: N > 0\0" as *const u8 as *const libc::c_char,
                b"celt/bands.c\0" as *const u8 as *const libc::c_char,
                1495 as libc::c_int,
            );
        }
        tell = ec_tell_frac(ec) as i32;
        if i != start {
            balance -= tell;
        }
        remaining_bits = total_bits - tell - 1 as libc::c_int;
        ctx.remaining_bits = remaining_bits;
        if i <= codedBands - 1 as libc::c_int {
            curr_balance = celt_sudiv(
                balance,
                if (3 as libc::c_int) < codedBands - i {
                    3 as libc::c_int
                } else {
                    codedBands - i
                },
            );
            b = if 0 as libc::c_int
                > (if (16383 as libc::c_int)
                    < (if (remaining_bits + 1 as libc::c_int)
                        < *pulses.offset(i as isize) + curr_balance
                    {
                        remaining_bits + 1 as libc::c_int
                    } else {
                        *pulses.offset(i as isize) + curr_balance
                    })
                {
                    16383 as libc::c_int
                } else {
                    if (remaining_bits + 1 as libc::c_int)
                        < *pulses.offset(i as isize) + curr_balance
                    {
                        remaining_bits + 1 as libc::c_int
                    } else {
                        *pulses.offset(i as isize) + curr_balance
                    }
                }) {
                0 as libc::c_int
            } else if (16383 as libc::c_int)
                < (if (remaining_bits + 1 as libc::c_int)
                    < *pulses.offset(i as isize) + curr_balance
                {
                    remaining_bits + 1 as libc::c_int
                } else {
                    *pulses.offset(i as isize) + curr_balance
                })
            {
                16383 as libc::c_int
            } else if (remaining_bits + 1 as libc::c_int)
                < *pulses.offset(i as isize) + curr_balance
            {
                remaining_bits + 1 as libc::c_int
            } else {
                *pulses.offset(i as isize) + curr_balance
            };
        } else {
            b = 0 as libc::c_int;
        }
        if resynth != 0
            && (M * *eBands.offset(i as isize) as libc::c_int - N
                >= M * *eBands.offset(start as isize) as libc::c_int
                || i == start + 1 as libc::c_int)
            && (update_lowband != 0 || lowband_offset == 0 as libc::c_int)
        {
            lowband_offset = i;
        }
        if i == start + 1 as libc::c_int {
            special_hybrid_folding(m, norm, norm2, start, M, dual_stereo);
        }
        tf_change = *tf_res.offset(i as isize);
        ctx.tf_change = tf_change;
        if i >= (*m).effEBands {
            X = norm;
            if !Y_.is_null() {
                Y = norm;
            }
            lowband_scratch = NULL as *mut celt_norm;
        }
        if last != 0 && theta_rdo == 0 {
            lowband_scratch = NULL as *mut celt_norm;
        }
        if lowband_offset != 0 as libc::c_int
            && (spread != SPREAD_AGGRESSIVE || B > 1 as libc::c_int || tf_change < 0 as libc::c_int)
        {
            let mut fold_start: libc::c_int = 0;
            let mut fold_end: libc::c_int = 0;
            let mut fold_i: libc::c_int = 0;
            effective_lowband = if 0 as libc::c_int
                > M * *eBands.offset(lowband_offset as isize) as libc::c_int - norm_offset - N
            {
                0 as libc::c_int
            } else {
                M * *eBands.offset(lowband_offset as isize) as libc::c_int - norm_offset - N
            };
            fold_start = lowband_offset;
            loop {
                fold_start -= 1;
                if !(M * *eBands.offset(fold_start as isize) as libc::c_int
                    > effective_lowband + norm_offset)
                {
                    break;
                }
            }
            fold_end = lowband_offset - 1 as libc::c_int;
            loop {
                fold_end += 1;
                if !(fold_end < i
                    && (M * *eBands.offset(fold_end as isize) as libc::c_int)
                        < effective_lowband + norm_offset + N)
                {
                    break;
                }
            }
            y_cm = 0 as libc::c_int as libc::c_uint;
            x_cm = y_cm;
            fold_i = fold_start;
            loop {
                x_cm |= *collapse_masks.offset((fold_i * C + 0 as libc::c_int) as isize)
                    as libc::c_uint;
                y_cm |= *collapse_masks.offset((fold_i * C + C - 1 as libc::c_int) as isize)
                    as libc::c_uint;
                fold_i += 1;
                if !(fold_i < fold_end) {
                    break;
                }
            }
        } else {
            y_cm = (((1 as libc::c_int) << B) - 1 as libc::c_int) as libc::c_uint;
            x_cm = y_cm;
        }
        if dual_stereo != 0 && i == intensity {
            let mut j: libc::c_int = 0;
            dual_stereo = 0 as libc::c_int;
            if resynth != 0 {
                j = 0 as libc::c_int;
                while j < M * *eBands.offset(i as isize) as libc::c_int - norm_offset {
                    *norm.offset(j as isize) =
                        0.5f32 * (*norm.offset(j as isize) + *norm2.offset(j as isize));
                    j += 1;
                }
            }
        }
        if dual_stereo != 0 {
            x_cm = quant_band(
                &mut ctx,
                X,
                N,
                b / 2 as libc::c_int,
                B,
                if effective_lowband != -(1 as libc::c_int) {
                    norm.offset(effective_lowband as isize)
                } else {
                    NULL as *mut celt_norm
                },
                LM,
                if last != 0 {
                    NULL as *mut celt_norm
                } else {
                    norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                        .offset(-(norm_offset as isize))
                },
                Q15ONE,
                lowband_scratch,
                x_cm as libc::c_int,
            );
            y_cm = quant_band(
                &mut ctx,
                Y,
                N,
                b / 2 as libc::c_int,
                B,
                if effective_lowband != -(1 as libc::c_int) {
                    norm2.offset(effective_lowband as isize)
                } else {
                    NULL as *mut celt_norm
                },
                LM,
                if last != 0 {
                    NULL as *mut celt_norm
                } else {
                    norm2
                        .offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                        .offset(-(norm_offset as isize))
                },
                Q15ONE,
                lowband_scratch,
                y_cm as libc::c_int,
            );
        } else {
            if !Y.is_null() {
                if theta_rdo != 0 && i < intensity {
                    let mut ec_save: ec_ctx = ec_ctx {
                        buf: 0 as *mut libc::c_uchar,
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
                    let mut ec_save2: ec_ctx = ec_ctx {
                        buf: 0 as *mut libc::c_uchar,
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
                    let mut ctx_save: band_ctx = band_ctx {
                        encode: 0,
                        resynth: 0,
                        m: 0 as *const OpusCustomMode,
                        i: 0,
                        intensity: 0,
                        spread: 0,
                        tf_change: 0,
                        ec: 0 as *mut ec_ctx,
                        remaining_bits: 0,
                        bandE: 0 as *const celt_ener,
                        seed: 0,
                        arch: 0,
                        theta_round: 0,
                        disable_inv: 0,
                        avoid_split_noise: 0,
                    };
                    let mut ctx_save2: band_ctx = band_ctx {
                        encode: 0,
                        resynth: 0,
                        m: 0 as *const OpusCustomMode,
                        i: 0,
                        intensity: 0,
                        spread: 0,
                        tf_change: 0,
                        ec: 0 as *mut ec_ctx,
                        remaining_bits: 0,
                        bandE: 0 as *const celt_ener,
                        seed: 0,
                        arch: 0,
                        theta_round: 0,
                        disable_inv: 0,
                        avoid_split_noise: 0,
                    };
                    let mut dist0: opus_val32 = 0.;
                    let mut dist1: opus_val32 = 0.;
                    let mut cm: libc::c_uint = 0;
                    let mut cm2: libc::c_uint = 0;
                    let mut nstart_bytes: libc::c_int = 0;
                    let mut nend_bytes: libc::c_int = 0;
                    let mut save_bytes: libc::c_int = 0;
                    let mut bytes_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut bytes_save: [libc::c_uchar; 1275] = [0; 1275];
                    let mut w: [opus_val16; 2] = [0.; 2];
                    compute_channel_weights(
                        *bandE.offset(i as isize),
                        *bandE.offset((i + (*m).nbEBands) as isize),
                        w.as_mut_ptr(),
                    );
                    cm = x_cm | y_cm;
                    ec_save = *ec;
                    ctx_save = ctx;
                    memcpy(
                        X_save.as_mut_ptr() as *mut libc::c_void,
                        X as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * X_save.as_mut_ptr().offset_from(X) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    memcpy(
                        Y_save.as_mut_ptr() as *mut libc::c_void,
                        Y as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * Y_save.as_mut_ptr().offset_from(Y) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    ctx.theta_round = -(1 as libc::c_int);
                    x_cm = quant_band_stereo(
                        &mut ctx,
                        X,
                        Y,
                        N,
                        b,
                        B,
                        if effective_lowband != -(1 as libc::c_int) {
                            norm.offset(effective_lowband as isize)
                        } else {
                            NULL as *mut celt_norm
                        },
                        LM,
                        if last != 0 {
                            NULL as *mut celt_norm
                        } else {
                            norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                                .offset(-(norm_offset as isize))
                        },
                        lowband_scratch,
                        cm as libc::c_int,
                    );
                    dist0 = w[0 as libc::c_int as usize]
                        * celt_inner_prod_c(X_save.as_mut_ptr(), X, N)
                        + w[1 as libc::c_int as usize]
                            * celt_inner_prod_c(Y_save.as_mut_ptr(), Y, N);
                    cm2 = x_cm;
                    ec_save2 = *ec;
                    ctx_save2 = ctx;
                    memcpy(
                        X_save2.as_mut_ptr() as *mut libc::c_void,
                        X as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * X_save2.as_mut_ptr().offset_from(X) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    memcpy(
                        Y_save2.as_mut_ptr() as *mut libc::c_void,
                        Y as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * Y_save2.as_mut_ptr().offset_from(Y) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    if last == 0 {
                        memcpy(
                            norm_save2.as_mut_ptr() as *mut libc::c_void,
                            norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                                .offset(-(norm_offset as isize))
                                as *const libc::c_void,
                            (N as libc::c_ulong)
                                .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                                .wrapping_add(
                                    (0 as libc::c_int as libc::c_long
                                        * norm_save2.as_mut_ptr().offset_from(
                                            norm.offset(
                                                (M * *eBands.offset(i as isize) as libc::c_int)
                                                    as isize,
                                            )
                                            .offset(-(norm_offset as isize)),
                                        ) as libc::c_long)
                                        as libc::c_ulong,
                                ),
                        );
                    }
                    nstart_bytes = ec_save.offs as libc::c_int;
                    nend_bytes = ec_save.storage as libc::c_int;
                    bytes_buf = (ec_save.buf).offset(nstart_bytes as isize);
                    save_bytes = nend_bytes - nstart_bytes;
                    memcpy(
                        bytes_save.as_mut_ptr() as *mut libc::c_void,
                        bytes_buf as *const libc::c_void,
                        (save_bytes as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * bytes_save.as_mut_ptr().offset_from(bytes_buf)
                                        as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    *ec = ec_save;
                    ctx = ctx_save;
                    memcpy(
                        X as *mut libc::c_void,
                        X_save.as_mut_ptr() as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * X.offset_from(X_save.as_mut_ptr()) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    memcpy(
                        Y as *mut libc::c_void,
                        Y_save.as_mut_ptr() as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * Y.offset_from(Y_save.as_mut_ptr()) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    if i == start + 1 as libc::c_int {
                        special_hybrid_folding(m, norm, norm2, start, M, dual_stereo);
                    }
                    ctx.theta_round = 1 as libc::c_int;
                    x_cm = quant_band_stereo(
                        &mut ctx,
                        X,
                        Y,
                        N,
                        b,
                        B,
                        if effective_lowband != -(1 as libc::c_int) {
                            norm.offset(effective_lowband as isize)
                        } else {
                            NULL as *mut celt_norm
                        },
                        LM,
                        if last != 0 {
                            NULL as *mut celt_norm
                        } else {
                            norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                                .offset(-(norm_offset as isize))
                        },
                        lowband_scratch,
                        cm as libc::c_int,
                    );
                    dist1 = w[0 as libc::c_int as usize]
                        * celt_inner_prod_c(X_save.as_mut_ptr(), X, N)
                        + w[1 as libc::c_int as usize]
                            * celt_inner_prod_c(Y_save.as_mut_ptr(), Y, N);
                    if dist0 >= dist1 {
                        x_cm = cm2;
                        *ec = ec_save2;
                        ctx = ctx_save2;
                        memcpy(
                            X as *mut libc::c_void,
                            X_save2.as_mut_ptr() as *const libc::c_void,
                            (N as libc::c_ulong)
                                .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                                .wrapping_add(
                                    (0 as libc::c_int as libc::c_long
                                        * X.offset_from(X_save2.as_mut_ptr()) as libc::c_long)
                                        as libc::c_ulong,
                                ),
                        );
                        memcpy(
                            Y as *mut libc::c_void,
                            Y_save2.as_mut_ptr() as *const libc::c_void,
                            (N as libc::c_ulong)
                                .wrapping_mul(::core::mem::size_of::<celt_norm>() as libc::c_ulong)
                                .wrapping_add(
                                    (0 as libc::c_int as libc::c_long
                                        * Y.offset_from(Y_save2.as_mut_ptr()) as libc::c_long)
                                        as libc::c_ulong,
                                ),
                        );
                        if last == 0 {
                            memcpy(
                                norm.offset(
                                    (M * *eBands.offset(i as isize) as libc::c_int) as isize,
                                )
                                .offset(-(norm_offset as isize))
                                    as *mut libc::c_void,
                                norm_save2.as_mut_ptr() as *const libc::c_void,
                                (N as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<celt_norm>() as libc::c_ulong
                                    )
                                    .wrapping_add(
                                        (0 as libc::c_int as libc::c_long
                                            * norm
                                                .offset(
                                                    (M * *eBands.offset(i as isize) as libc::c_int)
                                                        as isize,
                                                )
                                                .offset(-(norm_offset as isize))
                                                .offset_from(norm_save2.as_mut_ptr())
                                                as libc::c_long)
                                            as libc::c_ulong,
                                    ),
                            );
                        }
                        memcpy(
                            bytes_buf as *mut libc::c_void,
                            bytes_save.as_mut_ptr() as *const libc::c_void,
                            (save_bytes as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                )
                                .wrapping_add(
                                    (0 as libc::c_int as libc::c_long
                                        * bytes_buf.offset_from(bytes_save.as_mut_ptr())
                                            as libc::c_long)
                                        as libc::c_ulong,
                                ),
                        );
                    }
                } else {
                    ctx.theta_round = 0 as libc::c_int;
                    x_cm = quant_band_stereo(
                        &mut ctx,
                        X,
                        Y,
                        N,
                        b,
                        B,
                        if effective_lowband != -(1 as libc::c_int) {
                            norm.offset(effective_lowband as isize)
                        } else {
                            NULL as *mut celt_norm
                        },
                        LM,
                        if last != 0 {
                            NULL as *mut celt_norm
                        } else {
                            norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                                .offset(-(norm_offset as isize))
                        },
                        lowband_scratch,
                        (x_cm | y_cm) as libc::c_int,
                    );
                }
            } else {
                x_cm = quant_band(
                    &mut ctx,
                    X,
                    N,
                    b,
                    B,
                    if effective_lowband != -(1 as libc::c_int) {
                        norm.offset(effective_lowband as isize)
                    } else {
                        NULL as *mut celt_norm
                    },
                    LM,
                    if last != 0 {
                        NULL as *mut celt_norm
                    } else {
                        norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                            .offset(-(norm_offset as isize))
                    },
                    Q15ONE,
                    lowband_scratch,
                    (x_cm | y_cm) as libc::c_int,
                );
            }
            y_cm = x_cm;
        }
        *collapse_masks.offset((i * C + 0 as libc::c_int) as isize) = x_cm as libc::c_uchar;
        *collapse_masks.offset((i * C + C - 1 as libc::c_int) as isize) = y_cm as libc::c_uchar;
        balance += *pulses.offset(i as isize) + tell;
        update_lowband = (b > N << BITRES) as libc::c_int;
        ctx.avoid_split_noise = 0 as libc::c_int;
        i += 1;
    }
    *seed = ctx.seed;
}
