use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:32"]
pub mod SigProc_FLP_h {
    #[inline]
    #[c2rust::src_loc = "150:1"]
    pub unsafe extern "C" fn silk_sigmoid(x: libc::c_float) -> libc::c_float {
        return (1.0f64 / (1.0f64 + (-x as f64).exp())) as libc::c_float;
    }
    #[inline]
    #[c2rust::src_loc = "188:1"]
    pub unsafe extern "C" fn silk_log2(x: libc::c_double) -> libc::c_float {
        return (3.32192809488736f64 * x.log10()) as libc::c_float;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tuning_parameters.h:33"]
pub mod tuning_parameters_h {
    #[c2rust::src_loc = "44:9"]
    pub const FIND_PITCH_WHITE_NOISE_FRACTION: libc::c_float = 1e-3f32;
    #[c2rust::src_loc = "90:9"]
    pub const BG_SNR_DECR_dB: libc::c_float = 2.0f32;
    #[c2rust::src_loc = "93:9"]
    pub const HARM_SNR_INCR_dB: libc::c_float = 2.0f32;
    #[c2rust::src_loc = "99:9"]
    pub const ENERGY_VARIATION_THRESHOLD_QNT_OFFSET: libc::c_float = 0.6f32;
    #[c2rust::src_loc = "105:9"]
    pub const SHAPE_WHITE_NOISE_FRACTION: libc::c_float = 3e-5f32;
    #[c2rust::src_loc = "108:9"]
    pub const BANDWIDTH_EXPANSION: libc::c_float = 0.94f32;
    #[c2rust::src_loc = "111:9"]
    pub const HARMONIC_SHAPING: libc::c_float = 0.3f32;
    #[c2rust::src_loc = "114:9"]
    pub const HIGH_RATE_OR_LOW_QUALITY_HARMONIC_SHAPING: libc::c_float = 0.2f32;
    #[c2rust::src_loc = "117:9"]
    pub const HP_NOISE_COEF: libc::c_float = 0.25f32;
    #[c2rust::src_loc = "120:9"]
    pub const HARM_HP_NOISE_COEF: libc::c_float = 0.35f32;
    #[c2rust::src_loc = "129:9"]
    pub const LOW_FREQ_SHAPING: libc::c_float = 4.0f32;
    #[c2rust::src_loc = "132:9"]
    pub const LOW_QUALITY_LOW_FREQ_SHAPING_DECR: libc::c_float = 0.5f32;
    #[c2rust::src_loc = "135:9"]
    pub const SUBFR_SMTH_COEF: libc::c_float = 0.4f32;
}
pub use self::tuning_parameters_h::{
    BG_SNR_DECR_dB, HARM_SNR_INCR_dB, BANDWIDTH_EXPANSION, ENERGY_VARIATION_THRESHOLD_QNT_OFFSET,
    FIND_PITCH_WHITE_NOISE_FRACTION, HARMONIC_SHAPING, HARM_HP_NOISE_COEF,
    HIGH_RATE_OR_LOW_QUALITY_HARMONIC_SHAPING, HP_NOISE_COEF, LOW_FREQ_SHAPING,
    LOW_QUALITY_LOW_FREQ_SHAPING_DECR, SHAPE_WHITE_NOISE_FRACTION, SUBFR_SMTH_COEF,
};
use crate::silk::define::{MAX_SHAPE_LPC_ORDER, MIN_QGAIN_DB, TYPE_VOICED, USE_HARM_SHAPING};
use crate::silk::float::structs_FLP::{
    silk_encoder_control_FLP, silk_encoder_state_FLP, silk_shape_state_FLP,
};

pub use self::SigProc_FLP_h::{silk_log2, silk_sigmoid};
use crate::externs::memcpy;
use crate::silk::float::apply_sine_window_FLP::silk_apply_sine_window_FLP;
use crate::silk::float::autocorrelation_FLP::silk_autocorrelation_FLP;
use crate::silk::float::bwexpander_FLP::silk_bwexpander_FLP;
use crate::silk::float::energy_FLP::silk_energy_FLP;
use crate::silk::float::k2a_FLP::silk_k2a_FLP;
use crate::silk::float::schur_FLP::silk_schur_FLP;
use crate::silk::float::warped_autocorrelation_FLP::silk_warped_autocorrelation_FLP;

#[inline]
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn warped_gain(
    coefs: *const libc::c_float,
    mut lambda: libc::c_float,
    order: libc::c_int,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut gain: libc::c_float = 0.;
    lambda = -lambda;
    gain = *coefs.offset((order - 1 as libc::c_int) as isize);
    i = order - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        gain = lambda * gain + *coefs.offset(i as isize);
        i -= 1;
    }
    return 1.0f32 / (1.0f32 - lambda * gain);
}
#[inline]
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn warped_true2monic_coefs(
    coefs: *mut libc::c_float,
    lambda: libc::c_float,
    limit: libc::c_float,
    order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut ind: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_float = 0.;
    let mut maxabs: libc::c_float = 0.;
    let mut chirp: libc::c_float = 0.;
    let mut gain: libc::c_float = 0.;
    i = order - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        *coefs.offset((i - 1 as libc::c_int) as isize) -= lambda * *coefs.offset(i as isize);
        i -= 1;
    }
    gain =
        (1.0f32 - lambda * lambda) / (1.0f32 + lambda * *coefs.offset(0 as libc::c_int as isize));
    i = 0 as libc::c_int;
    while i < order {
        *coefs.offset(i as isize) *= gain;
        i += 1;
    }
    iter = 0 as libc::c_int;
    while iter < 10 as libc::c_int {
        maxabs = -1.0f32;
        i = 0 as libc::c_int;
        while i < order {
            tmp = (*coefs.offset(i as isize)).abs();
            if tmp > maxabs {
                maxabs = tmp;
                ind = i;
            }
            i += 1;
        }
        if maxabs <= limit {
            return;
        }
        i = 1 as libc::c_int;
        while i < order {
            *coefs.offset((i - 1 as libc::c_int) as isize) += lambda * *coefs.offset(i as isize);
            i += 1;
        }
        gain = 1.0f32 / gain;
        i = 0 as libc::c_int;
        while i < order {
            *coefs.offset(i as isize) *= gain;
            i += 1;
        }
        chirp = 0.99f32
            - (0.8f32 + 0.1f32 * iter as libc::c_float) * (maxabs - limit)
                / (maxabs * (ind + 1 as libc::c_int) as libc::c_float);
        silk_bwexpander_FLP(coefs, order, chirp);
        i = order - 1 as libc::c_int;
        while i > 0 as libc::c_int {
            *coefs.offset((i - 1 as libc::c_int) as isize) -= lambda * *coefs.offset(i as isize);
            i -= 1;
        }
        gain = (1.0f32 - lambda * lambda)
            / (1.0f32 + lambda * *coefs.offset(0 as libc::c_int as isize));
        i = 0 as libc::c_int;
        while i < order {
            *coefs.offset(i as isize) *= gain;
            i += 1;
        }
        iter += 1;
    }
}
#[inline]
#[c2rust::src_loc = "116:1"]
unsafe extern "C" fn limit_coefs(
    coefs: *mut libc::c_float,
    limit: libc::c_float,
    order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut ind: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_float = 0.;
    let mut maxabs: libc::c_float = 0.;
    let mut chirp: libc::c_float = 0.;
    iter = 0 as libc::c_int;
    while iter < 10 as libc::c_int {
        maxabs = -1.0f32;
        i = 0 as libc::c_int;
        while i < order {
            tmp = (*coefs.offset(i as isize)).abs();
            if tmp > maxabs {
                maxabs = tmp;
                ind = i;
            }
            i += 1;
        }
        if maxabs <= limit {
            return;
        }
        chirp = 0.99f32
            - (0.8f32 + 0.1f32 * iter as libc::c_float) * (maxabs - limit)
                / (maxabs * (ind + 1 as libc::c_int) as libc::c_float);
        silk_bwexpander_FLP(coefs, order, chirp);
        iter += 1;
    }
}
#[c2rust::src_loc = "147:1"]
pub unsafe extern "C" fn silk_noise_shape_analysis_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut psEncCtrl: *mut silk_encoder_control_FLP,
    pitch_res: *const libc::c_float,
    x: *const libc::c_float,
) {
    let mut psShapeSt: *mut silk_shape_state_FLP = &mut (*psEnc).sShape;
    let mut k: libc::c_int = 0;
    let mut nSamples: libc::c_int = 0;
    let mut nSegs: libc::c_int = 0;
    let mut SNR_adj_dB: libc::c_float = 0.;
    let mut HarmShapeGain: libc::c_float = 0.;
    let mut Tilt: libc::c_float = 0.;
    let mut nrg: libc::c_float = 0.;
    let mut log_energy: libc::c_float = 0.;
    let mut log_energy_prev: libc::c_float = 0.;
    let mut energy_variation: libc::c_float = 0.;
    let mut BWExp: libc::c_float = 0.;
    let mut gain_mult: libc::c_float = 0.;
    let mut gain_add: libc::c_float = 0.;
    let mut strength: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    let mut warping: libc::c_float = 0.;
    let mut x_windowed: [libc::c_float; 240] = [0.; 240];
    let mut auto_corr: [libc::c_float; 25] = [0.; 25];
    let mut rc: [libc::c_float; 25] = [0.; 25];
    let mut x_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut pitch_res_ptr: *const libc::c_float = 0 as *const libc::c_float;
    x_ptr = x.offset(-((*psEnc).sCmn.la_shape as isize));
    SNR_adj_dB =
        (*psEnc).sCmn.SNR_dB_Q7 as libc::c_float * (1 as libc::c_int as libc::c_float / 128.0f32);
    (*psEncCtrl).input_quality = 0.5f32
        * ((*psEnc).sCmn.input_quality_bands_Q15[0 as libc::c_int as usize]
            + (*psEnc).sCmn.input_quality_bands_Q15[1 as libc::c_int as usize])
            as libc::c_float
        * (1.0f32 / 32768.0f32);
    (*psEncCtrl).coding_quality = silk_sigmoid(0.25f32 * (SNR_adj_dB - 20.0f32));
    if (*psEnc).sCmn.useCBR == 0 as libc::c_int {
        b = 1.0f32 - (*psEnc).sCmn.speech_activity_Q8 as libc::c_float * (1.0f32 / 256.0f32);
        SNR_adj_dB -= BG_SNR_DECR_dB
            * (*psEncCtrl).coding_quality
            * (0.5f32 + 0.5f32 * (*psEncCtrl).input_quality)
            * b
            * b;
    }
    if (*psEnc).sCmn.indices.signalType as libc::c_int == TYPE_VOICED {
        SNR_adj_dB += HARM_SNR_INCR_dB * (*psEnc).LTPCorr;
    } else {
        SNR_adj_dB += (-0.4f32
            * (*psEnc).sCmn.SNR_dB_Q7 as libc::c_float
            * (1 as libc::c_int as libc::c_float / 128.0f32)
            + 6.0f32)
            * (1.0f32 - (*psEncCtrl).input_quality);
    }
    if (*psEnc).sCmn.indices.signalType as libc::c_int == TYPE_VOICED {
        (*psEnc).sCmn.indices.quantOffsetType = 0 as libc::c_int as i8;
    } else {
        nSamples = 2 as libc::c_int * (*psEnc).sCmn.fs_kHz;
        energy_variation = 0.0f32;
        log_energy_prev = 0.0f32;
        pitch_res_ptr = pitch_res;
        nSegs = 5 as libc::c_int as i16 as i32 * (*psEnc).sCmn.nb_subfr as i16 as i32
            / 2 as libc::c_int;
        k = 0 as libc::c_int;
        while k < nSegs {
            nrg = nSamples as libc::c_float
                + silk_energy_FLP(pitch_res_ptr, nSamples) as libc::c_float;
            log_energy = silk_log2(nrg as libc::c_double);
            if k > 0 as libc::c_int {
                energy_variation += (log_energy - log_energy_prev).abs();
            }
            log_energy_prev = log_energy;
            pitch_res_ptr = pitch_res_ptr.offset(nSamples as isize);
            k += 1;
        }
        if energy_variation
            > ENERGY_VARIATION_THRESHOLD_QNT_OFFSET * (nSegs - 1 as libc::c_int) as libc::c_float
        {
            (*psEnc).sCmn.indices.quantOffsetType = 0 as libc::c_int as i8;
        } else {
            (*psEnc).sCmn.indices.quantOffsetType = 1 as libc::c_int as i8;
        }
    }
    strength = FIND_PITCH_WHITE_NOISE_FRACTION * (*psEncCtrl).predGain;
    BWExp = BANDWIDTH_EXPANSION / (1.0f32 + strength * strength);
    warping = (*psEnc).sCmn.warping_Q16 as libc::c_float / 65536.0f32
        + 0.01f32 * (*psEncCtrl).coding_quality;
    k = 0 as libc::c_int;
    while k < (*psEnc).sCmn.nb_subfr {
        let mut shift: libc::c_int = 0;
        let mut slope_part: libc::c_int = 0;
        let mut flat_part: libc::c_int = 0;
        flat_part = (*psEnc).sCmn.fs_kHz * 3 as libc::c_int;
        slope_part = ((*psEnc).sCmn.shapeWinLength - flat_part) / 2 as libc::c_int;
        silk_apply_sine_window_FLP(x_windowed.as_mut_ptr(), x_ptr, 1 as libc::c_int, slope_part);
        shift = slope_part;
        memcpy(
            x_windowed.as_mut_ptr().offset(shift as isize) as *mut libc::c_void,
            x_ptr.offset(shift as isize) as *const libc::c_void,
            (flat_part as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
        shift += flat_part;
        silk_apply_sine_window_FLP(
            x_windowed.as_mut_ptr().offset(shift as isize),
            x_ptr.offset(shift as isize),
            2 as libc::c_int,
            slope_part,
        );
        x_ptr = x_ptr.offset((*psEnc).sCmn.subfr_length as isize);
        if (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int {
            silk_warped_autocorrelation_FLP(
                auto_corr.as_mut_ptr(),
                x_windowed.as_mut_ptr(),
                warping,
                (*psEnc).sCmn.shapeWinLength,
                (*psEnc).sCmn.shapingLPCOrder,
            );
        } else {
            silk_autocorrelation_FLP(
                auto_corr.as_mut_ptr(),
                x_windowed.as_mut_ptr(),
                (*psEnc).sCmn.shapeWinLength,
                (*psEnc).sCmn.shapingLPCOrder + 1 as libc::c_int,
            );
        }
        auto_corr[0 as libc::c_int as usize] +=
            auto_corr[0 as libc::c_int as usize] * SHAPE_WHITE_NOISE_FRACTION + 1.0f32;
        nrg = silk_schur_FLP(
            rc.as_mut_ptr(),
            auto_corr.as_mut_ptr() as *const libc::c_float,
            (*psEnc).sCmn.shapingLPCOrder,
        );
        silk_k2a_FLP(
            &mut *((*psEncCtrl).AR)
                .as_mut_ptr()
                .offset((k * MAX_SHAPE_LPC_ORDER) as isize),
            rc.as_mut_ptr(),
            (*psEnc).sCmn.shapingLPCOrder,
        );
        (*psEncCtrl).Gains[k as usize] = (nrg).sqrt();
        if (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int {
            (*psEncCtrl).Gains[k as usize] *= warped_gain(
                &mut *((*psEncCtrl).AR)
                    .as_mut_ptr()
                    .offset((k * MAX_SHAPE_LPC_ORDER) as isize),
                warping,
                (*psEnc).sCmn.shapingLPCOrder,
            );
        }
        silk_bwexpander_FLP(
            &mut *((*psEncCtrl).AR)
                .as_mut_ptr()
                .offset((k * MAX_SHAPE_LPC_ORDER) as isize),
            (*psEnc).sCmn.shapingLPCOrder,
            BWExp,
        );
        if (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int {
            warped_true2monic_coefs(
                &mut *((*psEncCtrl).AR)
                    .as_mut_ptr()
                    .offset((k * MAX_SHAPE_LPC_ORDER) as isize),
                warping,
                3.999f32,
                (*psEnc).sCmn.shapingLPCOrder,
            );
        } else {
            limit_coefs(
                &mut *((*psEncCtrl).AR)
                    .as_mut_ptr()
                    .offset((k * MAX_SHAPE_LPC_ORDER) as isize),
                3.999f32,
                (*psEnc).sCmn.shapingLPCOrder,
            );
        }
        k += 1;
    }
    gain_mult = 2.0f32.powf(-0.16f32 * SNR_adj_dB);
    gain_add = 2.0f32.powf(0.16f32 * MIN_QGAIN_DB as f32);
    k = 0 as libc::c_int;
    while k < (*psEnc).sCmn.nb_subfr {
        (*psEncCtrl).Gains[k as usize] *= gain_mult;
        (*psEncCtrl).Gains[k as usize] += gain_add;
        k += 1;
    }
    strength = LOW_FREQ_SHAPING
        * (1.0f32
            + LOW_QUALITY_LOW_FREQ_SHAPING_DECR
                * ((*psEnc).sCmn.input_quality_bands_Q15[0 as libc::c_int as usize]
                    as libc::c_float
                    * (1.0f32 / 32768.0f32)
                    - 1.0f32));
    strength *= (*psEnc).sCmn.speech_activity_Q8 as libc::c_float * (1.0f32 / 256.0f32);
    if (*psEnc).sCmn.indices.signalType as libc::c_int == TYPE_VOICED {
        k = 0 as libc::c_int;
        while k < (*psEnc).sCmn.nb_subfr {
            b = 0.2f32 / (*psEnc).sCmn.fs_kHz as libc::c_float
                + 3.0f32 / (*psEncCtrl).pitchL[k as usize] as libc::c_float;
            (*psEncCtrl).LF_MA_shp[k as usize] = -1.0f32 + b;
            (*psEncCtrl).LF_AR_shp[k as usize] = 1.0f32 - b - b * strength;
            k += 1;
        }
        Tilt = -HP_NOISE_COEF
            - (1 as libc::c_int as libc::c_float - HP_NOISE_COEF)
                * HARM_HP_NOISE_COEF
                * (*psEnc).sCmn.speech_activity_Q8 as libc::c_float
                * (1.0f32 / 256.0f32);
    } else {
        b = 1.3f32 / (*psEnc).sCmn.fs_kHz as libc::c_float;
        (*psEncCtrl).LF_MA_shp[0 as libc::c_int as usize] = -1.0f32 + b;
        (*psEncCtrl).LF_AR_shp[0 as libc::c_int as usize] = 1.0f32 - b - b * strength * 0.6f32;
        k = 1 as libc::c_int;
        while k < (*psEnc).sCmn.nb_subfr {
            (*psEncCtrl).LF_MA_shp[k as usize] = (*psEncCtrl).LF_MA_shp[0 as libc::c_int as usize];
            (*psEncCtrl).LF_AR_shp[k as usize] = (*psEncCtrl).LF_AR_shp[0 as libc::c_int as usize];
            k += 1;
        }
        Tilt = -HP_NOISE_COEF;
    }
    if USE_HARM_SHAPING != 0 && (*psEnc).sCmn.indices.signalType as libc::c_int == TYPE_VOICED {
        HarmShapeGain = HARMONIC_SHAPING;
        HarmShapeGain += HIGH_RATE_OR_LOW_QUALITY_HARMONIC_SHAPING
            * (1.0f32 - (1.0f32 - (*psEncCtrl).coding_quality) * (*psEncCtrl).input_quality);
        HarmShapeGain *= ((*psEnc).LTPCorr).sqrt();
    } else {
        HarmShapeGain = 0.0f32;
    }
    k = 0 as libc::c_int;
    while k < (*psEnc).sCmn.nb_subfr {
        (*psShapeSt).HarmShapeGain_smth +=
            SUBFR_SMTH_COEF * (HarmShapeGain - (*psShapeSt).HarmShapeGain_smth);
        (*psEncCtrl).HarmShapeGain[k as usize] = (*psShapeSt).HarmShapeGain_smth;
        (*psShapeSt).Tilt_smth += SUBFR_SMTH_COEF * (Tilt - (*psShapeSt).Tilt_smth);
        (*psEncCtrl).Tilt[k as usize] = (*psShapeSt).Tilt_smth;
        k += 1;
    }
}
