use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:33"]
pub mod SigProc_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "45:1"]
        pub fn silk_bwexpander_FLP(ar: *mut libc::c_float, d: libc::c_int, chirp: libc::c_float);
        #[c2rust::src_loc = "59:1"]
        pub fn silk_schur_FLP(
            refl_coef: *mut libc::c_float,
            auto_corr: *const libc::c_float,
            order: libc::c_int,
        ) -> libc::c_float;
        #[c2rust::src_loc = "65:1"]
        pub fn silk_k2a_FLP(A: *mut libc::c_float, rc: *const libc::c_float, order: i32);
        #[c2rust::src_loc = "72:1"]
        pub fn silk_autocorrelation_FLP(
            results: *mut libc::c_float,
            inputData: *const libc::c_float,
            inputDataSize: libc::c_int,
            correlationCount: libc::c_int,
        );
        #[c2rust::src_loc = "79:1"]
        pub fn silk_pitch_analysis_core_FLP(
            frame: *const libc::c_float,
            pitch_out: *mut libc::c_int,
            lagIndex: *mut i16,
            contourIndex: *mut i8,
            LTPCorr: *mut libc::c_float,
            prevLag: libc::c_int,
            search_thres1: libc::c_float,
            search_thres2: libc::c_float,
            Fs_kHz: libc::c_int,
            complexity: libc::c_int,
            nb_subfr: libc::c_int,
            arch: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tuning_parameters.h:34"]
pub mod tuning_parameters_h {
    #[c2rust::src_loc = "44:9"]
    pub const FIND_PITCH_WHITE_NOISE_FRACTION: libc::c_float = 1e-3f32;
    #[c2rust::src_loc = "47:9"]
    pub const FIND_PITCH_BANDWIDTH_EXPANSION: libc::c_float = 0.99f32;
}
pub use self::tuning_parameters_h::{
    FIND_PITCH_BANDWIDTH_EXPANSION, FIND_PITCH_WHITE_NOISE_FRACTION,
};
use self::SigProc_FLP_h::{
    silk_autocorrelation_FLP, silk_bwexpander_FLP, silk_k2a_FLP, silk_pitch_analysis_core_FLP,
    silk_schur_FLP,
};
use crate::celt::celt::celt_fatal;
use crate::externs::{memcpy, memset};
use crate::silk::define::{TYPE_NO_VOICE_ACTIVITY, TYPE_UNVOICED, TYPE_VOICED};
use crate::silk::float::apply_sine_window_FLP::silk_apply_sine_window_FLP;
use crate::silk::float::structs_FLP::{silk_encoder_control_FLP, silk_encoder_state_FLP};
use crate::silk::float::LPC_analysis_filter_FLP::silk_LPC_analysis_filter_FLP;

#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_find_pitch_lags_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut psEncCtrl: *mut silk_encoder_control_FLP,
    res: *mut libc::c_float,
    x: *const libc::c_float,
    arch: libc::c_int,
) {
    let mut buf_len: libc::c_int = 0;
    let mut thrhld: libc::c_float = 0.;
    let mut res_nrg: libc::c_float = 0.;
    let mut x_buf_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut x_buf: *const libc::c_float = 0 as *const libc::c_float;
    let mut auto_corr: [libc::c_float; 17] = [0.; 17];
    let mut A: [libc::c_float; 16] = [0.; 16];
    let mut refl_coef: [libc::c_float; 16] = [0.; 16];
    let mut Wsig: [libc::c_float; 384] = [0.; 384];
    let mut Wsig_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    buf_len = (*psEnc).sCmn.la_pitch + (*psEnc).sCmn.frame_length + (*psEnc).sCmn.ltp_mem_length;
    if !(buf_len >= (*psEnc).sCmn.pitch_LPC_win_length) {
        celt_fatal(
            b"assertion failed: buf_len >= psEnc->sCmn.pitch_LPC_win_length\0" as *const u8
                as *const libc::c_char,
            b"silk/float/find_pitch_lags_FLP.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
        );
    }
    x_buf = x.offset(-((*psEnc).sCmn.ltp_mem_length as isize));
    x_buf_ptr = x_buf
        .offset(buf_len as isize)
        .offset(-((*psEnc).sCmn.pitch_LPC_win_length as isize));
    Wsig_ptr = Wsig.as_mut_ptr();
    silk_apply_sine_window_FLP(
        Wsig_ptr,
        x_buf_ptr,
        1 as libc::c_int,
        (*psEnc).sCmn.la_pitch,
    );
    Wsig_ptr = Wsig_ptr.offset((*psEnc).sCmn.la_pitch as isize);
    x_buf_ptr = x_buf_ptr.offset((*psEnc).sCmn.la_pitch as isize);
    memcpy(
        Wsig_ptr as *mut libc::c_void,
        x_buf_ptr as *const libc::c_void,
        (((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    Wsig_ptr = Wsig_ptr.offset(
        ((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as libc::c_int))
            as isize,
    );
    x_buf_ptr = x_buf_ptr.offset(
        ((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as libc::c_int))
            as isize,
    );
    silk_apply_sine_window_FLP(
        Wsig_ptr,
        x_buf_ptr,
        2 as libc::c_int,
        (*psEnc).sCmn.la_pitch,
    );
    silk_autocorrelation_FLP(
        auto_corr.as_mut_ptr(),
        Wsig.as_mut_ptr(),
        (*psEnc).sCmn.pitch_LPC_win_length,
        (*psEnc).sCmn.pitchEstimationLPCOrder + 1 as libc::c_int,
    );
    auto_corr[0 as libc::c_int as usize] += auto_corr[0 as libc::c_int as usize]
        * FIND_PITCH_WHITE_NOISE_FRACTION
        + 1 as libc::c_int as libc::c_float;
    res_nrg = silk_schur_FLP(
        refl_coef.as_mut_ptr(),
        auto_corr.as_mut_ptr() as *const libc::c_float,
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    (*psEncCtrl).predGain =
        auto_corr[0 as libc::c_int as usize] / (if res_nrg > 1.0f32 { res_nrg } else { 1.0f32 });
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
        A.as_mut_ptr() as *const libc::c_float,
        x_buf,
        buf_len,
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    if (*psEnc).sCmn.indices.signalType as libc::c_int != TYPE_NO_VOICE_ACTIVITY
        && (*psEnc).sCmn.first_frame_after_reset == 0 as libc::c_int
    {
        thrhld = 0.6f32;
        thrhld -= 0.004f32 * (*psEnc).sCmn.pitchEstimationLPCOrder as libc::c_float;
        thrhld -= 0.1f32 * (*psEnc).sCmn.speech_activity_Q8 as libc::c_float * (1.0f32 / 256.0f32);
        thrhld -= 0.15f32
            * ((*psEnc).sCmn.prevSignalType as libc::c_int >> 1 as libc::c_int) as libc::c_float;
        thrhld -= 0.1f32 * (*psEnc).sCmn.input_tilt_Q15 as libc::c_float * (1.0f32 / 32768.0f32);
        if silk_pitch_analysis_core_FLP(
            res as *const libc::c_float,
            ((*psEncCtrl).pitchL).as_mut_ptr(),
            &mut (*psEnc).sCmn.indices.lagIndex,
            &mut (*psEnc).sCmn.indices.contourIndex,
            &mut (*psEnc).LTPCorr,
            (*psEnc).sCmn.prevLag,
            (*psEnc).sCmn.pitchEstimationThreshold_Q16 as libc::c_float / 65536.0f32,
            thrhld,
            (*psEnc).sCmn.fs_kHz,
            (*psEnc).sCmn.pitchEstimationComplexity,
            (*psEnc).sCmn.nb_subfr,
            arch,
        ) == 0 as libc::c_int
        {
            (*psEnc).sCmn.indices.signalType = TYPE_VOICED as i8;
        } else {
            (*psEnc).sCmn.indices.signalType = TYPE_UNVOICED as i8;
        }
    } else {
        memset(
            ((*psEncCtrl).pitchL).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong,
        );
        (*psEnc).sCmn.indices.lagIndex = 0 as libc::c_int as i16;
        (*psEnc).sCmn.indices.contourIndex = 0 as libc::c_int as i8;
        (*psEnc).LTPCorr = 0 as libc::c_int as libc::c_float;
    };
}
