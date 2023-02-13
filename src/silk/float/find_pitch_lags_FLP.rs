#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tuning_parameters.h:34"]
pub mod tuning_parameters_h {
    #[c2rust::src_loc = "44:9"]
    pub const FIND_PITCH_WHITE_NOISE_FRACTION: f32 = 1e-3f32;
    #[c2rust::src_loc = "47:9"]
    pub const FIND_PITCH_BANDWIDTH_EXPANSION: f32 = 0.99f32;
}
pub use self::tuning_parameters_h::{
    FIND_PITCH_BANDWIDTH_EXPANSION, FIND_PITCH_WHITE_NOISE_FRACTION,
};
use crate::celt::celt::celt_fatal;
use crate::externs::{memcpy, memset};
use crate::silk::define::{TYPE_NO_VOICE_ACTIVITY, TYPE_UNVOICED, TYPE_VOICED};
use crate::silk::float::apply_sine_window_FLP::silk_apply_sine_window_FLP;
use crate::silk::float::autocorrelation_FLP::silk_autocorrelation_FLP;
use crate::silk::float::bwexpander_FLP::silk_bwexpander_FLP;
use crate::silk::float::k2a_FLP::silk_k2a_FLP;
use crate::silk::float::pitch_analysis_core_FLP::silk_pitch_analysis_core_FLP;
use crate::silk::float::schur_FLP::silk_schur_FLP;
use crate::silk::float::structs_FLP::{silk_encoder_control_FLP, silk_encoder_state_FLP};
use crate::silk::float::LPC_analysis_filter_FLP::silk_LPC_analysis_filter_FLP;

#[c2rust::src_loc = "36:1"]
pub unsafe fn silk_find_pitch_lags_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut psEncCtrl: *mut silk_encoder_control_FLP,
    res: *mut f32,
    x: *const f32,
    arch: i32,
) {
    let mut buf_len: i32 = 0;
    let mut thrhld: f32 = 0.;
    let mut res_nrg: f32 = 0.;
    let mut x_buf_ptr: *const f32 = 0 as *const f32;
    let mut x_buf: *const f32 = 0 as *const f32;
    let mut auto_corr: [f32; 17] = [0.; 17];
    let mut A: [f32; 16] = [0.; 16];
    let mut refl_coef: [f32; 16] = [0.; 16];
    let mut Wsig: [f32; 384] = [0.; 384];
    let mut Wsig_ptr: *mut f32 = 0 as *mut f32;
    buf_len = (*psEnc).sCmn.la_pitch + (*psEnc).sCmn.frame_length + (*psEnc).sCmn.ltp_mem_length;
    if !(buf_len >= (*psEnc).sCmn.pitch_LPC_win_length) {
        celt_fatal(
            b"assertion failed: buf_len >= psEnc->sCmn.pitch_LPC_win_length\0" as *const u8
                as *const i8,
            b"silk/float/find_pitch_lags_FLP.c\0" as *const u8 as *const i8,
            59 as i32,
        );
    }
    x_buf = x.offset(-((*psEnc).sCmn.ltp_mem_length as isize));
    x_buf_ptr = x_buf
        .offset(buf_len as isize)
        .offset(-((*psEnc).sCmn.pitch_LPC_win_length as isize));
    Wsig_ptr = Wsig.as_mut_ptr();
    silk_apply_sine_window_FLP(Wsig_ptr, x_buf_ptr, 1 as i32, (*psEnc).sCmn.la_pitch);
    Wsig_ptr = Wsig_ptr.offset((*psEnc).sCmn.la_pitch as isize);
    x_buf_ptr = x_buf_ptr.offset((*psEnc).sCmn.la_pitch as isize);
    memcpy(
        Wsig_ptr as *mut core::ffi::c_void,
        x_buf_ptr as *const core::ffi::c_void,
        (((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as i32)) as u64)
            .wrapping_mul(::core::mem::size_of::<f32>() as u64),
    );
    Wsig_ptr = Wsig_ptr.offset(
        ((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as i32)) as isize,
    );
    x_buf_ptr = x_buf_ptr.offset(
        ((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as i32)) as isize,
    );
    silk_apply_sine_window_FLP(Wsig_ptr, x_buf_ptr, 2 as i32, (*psEnc).sCmn.la_pitch);
    silk_autocorrelation_FLP(
        auto_corr.as_mut_ptr(),
        Wsig.as_mut_ptr(),
        (*psEnc).sCmn.pitch_LPC_win_length,
        (*psEnc).sCmn.pitchEstimationLPCOrder + 1 as i32,
    );
    auto_corr[0 as i32 as usize] +=
        auto_corr[0 as i32 as usize] * FIND_PITCH_WHITE_NOISE_FRACTION + 1 as i32 as f32;
    res_nrg = silk_schur_FLP(
        refl_coef.as_mut_ptr(),
        auto_corr.as_mut_ptr() as *const f32,
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    (*psEncCtrl).predGain =
        auto_corr[0 as i32 as usize] / (if res_nrg > 1.0f32 { res_nrg } else { 1.0f32 });
    silk_k2a_FLP(
        A.as_mut_ptr(),
        refl_coef.as_mut_ptr(),
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    silk_bwexpander_FLP(
        A.as_mut_ptr(),
        (*psEnc).sCmn.pitchEstimationLPCOrder,
        FIND_PITCH_BANDWIDTH_EXPANSION,
    );
    silk_LPC_analysis_filter_FLP(
        res,
        A.as_mut_ptr() as *const f32,
        x_buf,
        buf_len,
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    if (*psEnc).sCmn.indices.signalType as i32 != TYPE_NO_VOICE_ACTIVITY
        && (*psEnc).sCmn.first_frame_after_reset == 0 as i32
    {
        thrhld = 0.6f32;
        thrhld -= 0.004f32 * (*psEnc).sCmn.pitchEstimationLPCOrder as f32;
        thrhld -= 0.1f32 * (*psEnc).sCmn.speech_activity_Q8 as f32 * (1.0f32 / 256.0f32);
        thrhld -= 0.15f32 * ((*psEnc).sCmn.prevSignalType as i32 >> 1 as i32) as f32;
        thrhld -= 0.1f32 * (*psEnc).sCmn.input_tilt_Q15 as f32 * (1.0f32 / 32768.0f32);
        if silk_pitch_analysis_core_FLP(
            res as *const f32,
            ((*psEncCtrl).pitchL).as_mut_ptr(),
            &mut (*psEnc).sCmn.indices.lagIndex,
            &mut (*psEnc).sCmn.indices.contourIndex,
            &mut (*psEnc).LTPCorr,
            (*psEnc).sCmn.prevLag,
            (*psEnc).sCmn.pitchEstimationThreshold_Q16 as f32 / 65536.0f32,
            thrhld,
            (*psEnc).sCmn.fs_kHz,
            (*psEnc).sCmn.pitchEstimationComplexity,
            (*psEnc).sCmn.nb_subfr,
            arch,
        ) == 0 as i32
        {
            (*psEnc).sCmn.indices.signalType = TYPE_VOICED as i8;
        } else {
            (*psEnc).sCmn.indices.signalType = TYPE_UNVOICED as i8;
        }
    } else {
        memset(
            ((*psEncCtrl).pitchL).as_mut_ptr() as *mut core::ffi::c_void,
            0 as i32,
            ::core::mem::size_of::<[i32; 4]>() as u64,
        );
        (*psEnc).sCmn.indices.lagIndex = 0 as i32 as i16;
        (*psEnc).sCmn.indices.contourIndex = 0 as i32 as i8;
        (*psEnc).LTPCorr = 0 as i32 as f32;
    };
}
