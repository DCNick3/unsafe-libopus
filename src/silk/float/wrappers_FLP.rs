use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:32"]
pub mod resampler_structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [i32; 6],
        pub sFIR: C2RustUnnamed,
        pub delayBuf: [i16; 48],
        pub resampler_function: libc::c_int,
        pub batchSize: libc::c_int,
        pub invRatio_Q16: i32,
        pub FIR_Order: libc::c_int,
        pub FIR_Fracs: libc::c_int,
        pub Fs_in_kHz: libc::c_int,
        pub Fs_out_kHz: libc::c_int,
        pub inputDelay: libc::c_int,
        pub Coefs: *const i16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:5"]
    pub union C2RustUnnamed {
        pub i32_0: [i32; 36],
        pub i16_0: [i16; 36],
    }
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/xmmintrin.h:32"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:9"]
    pub struct silk_nsq_state {
        pub xq: [i16; 640],
        pub sLTP_shp_Q14: [i32; 640],
        pub sLPC_Q14: [i32; 96],
        pub sAR2_Q14: [i32; 24],
        pub sLF_AR_shp_Q14: i32,
        pub sDiff_shp_Q14: i32,
        pub lagPrev: libc::c_int,
        pub sLTP_buf_idx: libc::c_int,
        pub sLTP_shp_buf_idx: libc::c_int,
        pub rand_seed: i32,
        pub prev_gain_Q16: i32,
        pub rewhite_flag: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:9"]
    pub struct silk_VAD_state {
        pub AnaState: [i32; 2],
        pub AnaState1: [i32; 2],
        pub AnaState2: [i32; 2],
        pub XnrgSubfr: [i32; 4],
        pub NrgRatioSmth_Q8: [i32; 4],
        pub HPstate: i16,
        pub NL: [i32; 4],
        pub inv_NL: [i32; 4],
        pub NoiseLevelBias: [i32; 4],
        pub counter: i32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:9"]
    pub struct silk_LP_state {
        pub In_LP_State: [i32; 2],
        pub transition_frame_no: i32,
        pub mode: libc::c_int,
        pub saved_fs_kHz: i32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:9"]
    pub struct silk_NLSF_CB_struct {
        pub nVectors: i16,
        pub order: i16,
        pub quantStepSize_Q16: i16,
        pub invQuantStepSize_Q6: i16,
        pub CB1_NLSF_Q8: *const u8,
        pub CB1_Wght_Q9: *const i16,
        pub CB1_iCDF: *const u8,
        pub pred_Q8: *const u8,
        pub ec_sel: *const u8,
        pub ec_iCDF: *const u8,
        pub ec_Rates_Q5: *const u8,
        pub deltaMin_Q15: *const i16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:9"]
    pub struct SideInfoIndices {
        pub GainsIndices: [i8; 4],
        pub LTPIndex: [i8; 4],
        pub NLSFIndices: [i8; 17],
        pub lagIndex: i16,
        pub contourIndex: i8,
        pub signalType: i8,
        pub quantOffsetType: i8,
        pub NLSFInterpCoef_Q2: i8,
        pub PERIndex: i8,
        pub LTP_scaleIndex: i8,
        pub Seed: i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:9"]
    pub struct silk_encoder_state {
        pub In_HP_State: [i32; 2],
        pub variable_HP_smth1_Q15: i32,
        pub variable_HP_smth2_Q15: i32,
        pub sLP: silk_LP_state,
        pub sVAD: silk_VAD_state,
        pub sNSQ: silk_nsq_state,
        pub prev_NLSFq_Q15: [i16; 16],
        pub speech_activity_Q8: libc::c_int,
        pub allow_bandwidth_switch: libc::c_int,
        pub LBRRprevLastGainIndex: i8,
        pub prevSignalType: i8,
        pub prevLag: libc::c_int,
        pub pitch_LPC_win_length: libc::c_int,
        pub max_pitch_lag: libc::c_int,
        pub API_fs_Hz: i32,
        pub prev_API_fs_Hz: i32,
        pub maxInternal_fs_Hz: libc::c_int,
        pub minInternal_fs_Hz: libc::c_int,
        pub desiredInternal_fs_Hz: libc::c_int,
        pub fs_kHz: libc::c_int,
        pub nb_subfr: libc::c_int,
        pub frame_length: libc::c_int,
        pub subfr_length: libc::c_int,
        pub ltp_mem_length: libc::c_int,
        pub la_pitch: libc::c_int,
        pub la_shape: libc::c_int,
        pub shapeWinLength: libc::c_int,
        pub TargetRate_bps: i32,
        pub PacketSize_ms: libc::c_int,
        pub PacketLoss_perc: libc::c_int,
        pub frameCounter: i32,
        pub Complexity: libc::c_int,
        pub nStatesDelayedDecision: libc::c_int,
        pub useInterpolatedNLSFs: libc::c_int,
        pub shapingLPCOrder: libc::c_int,
        pub predictLPCOrder: libc::c_int,
        pub pitchEstimationComplexity: libc::c_int,
        pub pitchEstimationLPCOrder: libc::c_int,
        pub pitchEstimationThreshold_Q16: i32,
        pub sum_log_gain_Q7: i32,
        pub NLSF_MSVQ_Survivors: libc::c_int,
        pub first_frame_after_reset: libc::c_int,
        pub controlled_since_last_payload: libc::c_int,
        pub warping_Q16: libc::c_int,
        pub useCBR: libc::c_int,
        pub prefillFlag: libc::c_int,
        pub pitch_lag_low_bits_iCDF: *const u8,
        pub pitch_contour_iCDF: *const u8,
        pub psNLSF_CB: *const silk_NLSF_CB_struct,
        pub input_quality_bands_Q15: [libc::c_int; 4],
        pub input_tilt_Q15: libc::c_int,
        pub SNR_dB_Q7: libc::c_int,
        pub VAD_flags: [i8; 3],
        pub LBRR_flag: i8,
        pub LBRR_flags: [libc::c_int; 3],
        pub indices: SideInfoIndices,
        pub pulses: [i8; 320],
        pub arch: libc::c_int,
        pub inputBuf: [i16; 322],
        pub inputBufIx: libc::c_int,
        pub nFramesPerPacket: libc::c_int,
        pub nFramesEncoded: libc::c_int,
        pub nChannelsAPI: libc::c_int,
        pub nChannelsInternal: libc::c_int,
        pub channelNb: libc::c_int,
        pub frames_since_onset: libc::c_int,
        pub ec_prevSignalType: libc::c_int,
        pub ec_prevLagIndex: i16,
        pub resampler_state: silk_resampler_state_struct,
        pub useDTX: libc::c_int,
        pub inDTX: libc::c_int,
        pub noSpeechCounter: libc::c_int,
        pub useInBandFEC: libc::c_int,
        pub LBRR_enabled: libc::c_int,
        pub LBRR_GainIncreases: libc::c_int,
        pub indices_LBRR: [SideInfoIndices; 3],
        pub pulses_LBRR: [[i8; 320]; 3],
    }
    use super::resampler_structs_h::silk_resampler_state_struct;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/structs_FLP.h:32"]
pub mod structs_FLP_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:9"]
    pub struct silk_shape_state_FLP {
        pub LastGainIndex: i8,
        pub HarmShapeGain_smth: libc::c_float,
        pub Tilt_smth: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:9"]
    pub struct silk_encoder_state_FLP {
        pub sCmn: silk_encoder_state,
        pub sShape: silk_shape_state_FLP,
        pub x_buf: [libc::c_float; 720],
        pub LTPCorr: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "64:9"]
    pub struct silk_encoder_control_FLP {
        pub Gains: [libc::c_float; 4],
        pub PredCoef: [[libc::c_float; 16]; 2],
        pub LTPCoef: [libc::c_float; 20],
        pub LTP_scale: libc::c_float,
        pub pitchL: [libc::c_int; 4],
        pub AR: [libc::c_float; 96],
        pub LF_MA_shp: [libc::c_float; 4],
        pub LF_AR_shp: [libc::c_float; 4],
        pub Tilt: [libc::c_float; 4],
        pub HarmShapeGain: [libc::c_float; 4],
        pub Lambda: libc::c_float,
        pub input_quality: libc::c_float,
        pub coding_quality: libc::c_float,
        pub predGain: libc::c_float,
        pub LTPredCodGain: libc::c_float,
        pub ResNrg: [libc::c_float; 4],
        pub GainsUnq_Q16: [i32; 4],
        pub lastGainIndexPrev: i8,
    }
    use super::structs_h::silk_encoder_state;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    extern "C" {
        #[c2rust::src_loc = "279:1"]
        pub fn silk_A2NLSF(NLSF: *mut i16, a_Q16: *mut i32, d: libc::c_int);
        #[c2rust::src_loc = "286:1"]
        pub fn silk_NLSF2A(a_Q12: *mut i16, NLSF: *const i16, d: libc::c_int, arch: libc::c_int);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/float_cast.h:32"]
pub mod float_cast_h {
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn float2int(x: libc::c_float) -> i32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }
    use super::xmmintrin_h::{_mm_cvt_ss2si, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:32"]
pub mod SigProc_FLP_h {
    #[inline]
    #[c2rust::src_loc = "156:1"]
    pub unsafe extern "C" fn silk_float2int(x: libc::c_float) -> i32 {
        return float2int(x);
    }
    use super::float_cast_h::float2int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "84:26"]
        pub static silk_LTPScales_table_Q14: [i16; 3];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use super::structs_h::{silk_encoder_state, silk_nsq_state, SideInfoIndices};
    extern "C" {
        #[c2rust::src_loc = "211:1"]
        pub fn silk_quant_LTP_gains(
            B_Q14: *mut i16,
            cbk_index: *mut i8,
            periodicity_index: *mut i8,
            sum_gain_dB_Q7: *mut i32,
            pred_gain_dB_Q7: *mut libc::c_int,
            XX_Q17: *const i32,
            xX_Q17: *const i32,
            subfr_len: libc::c_int,
            nb_subfr: libc::c_int,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "249:1"]
        pub fn silk_NSQ_c(
            psEncC: *const silk_encoder_state,
            NSQ: *mut silk_nsq_state,
            psIndices: *mut SideInfoIndices,
            x16: *const i16,
            pulses: *mut i8,
            PredCoef_Q12: *const i16,
            LTPCoef_Q14: *const i16,
            AR_Q13: *const i16,
            HarmShapeGain_Q14: *const libc::c_int,
            Tilt_Q14: *const libc::c_int,
            LF_shp_Q14: *const i32,
            Gains_Q16: *const i32,
            pitchL: *const libc::c_int,
            Lambda_Q10: libc::c_int,
            LTP_scale_Q14: libc::c_int,
        );
        #[c2rust::src_loc = "275:1"]
        pub fn silk_NSQ_del_dec_c(
            psEncC: *const silk_encoder_state,
            NSQ: *mut silk_nsq_state,
            psIndices: *mut SideInfoIndices,
            x16: *const i16,
            pulses: *mut i8,
            PredCoef_Q12: *const i16,
            LTPCoef_Q14: *const i16,
            AR_Q13: *const i16,
            HarmShapeGain_Q14: *const libc::c_int,
            Tilt_Q14: *const libc::c_int,
            LF_shp_Q14: *const i32,
            Gains_Q16: *const i32,
            pitchL: *const libc::c_int,
            Lambda_Q10: libc::c_int,
            LTP_scale_Q14: libc::c_int,
        );
        #[c2rust::src_loc = "331:1"]
        pub fn silk_process_NLSFs(
            psEncC: *mut silk_encoder_state,
            PredCoef_Q12: *mut [i16; 16],
            pNLSF_Q15: *mut i16,
            prev_NLSFq_Q15: *const i16,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "146:9"]
    pub const LTP_ORDER: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "155:9"]
    pub const MAX_SHAPE_LPC_ORDER: libc::c_int = 24 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
}
pub use self::define_h::{LTP_ORDER, MAX_SHAPE_LPC_ORDER, TYPE_VOICED};
pub use self::float_cast_h::float2int;
use self::main_h::{silk_NSQ_c, silk_NSQ_del_dec_c, silk_process_NLSFs, silk_quant_LTP_gains};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};

pub use self::structs_FLP_h::{
    silk_encoder_control_FLP, silk_encoder_state_FLP, silk_shape_state_FLP,
};
pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    SideInfoIndices,
};
use self::tables_h::silk_LTPScales_table_Q14;

use self::SigProc_FIX_h::{silk_A2NLSF, silk_NLSF2A};
pub use self::SigProc_FLP_h::silk_float2int;
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn silk_A2NLSF_FLP(
    NLSF_Q15: *mut i16,
    pAR: *const libc::c_float,
    LPC_order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut a_fix_Q16: [i32; 16] = [0; 16];
    i = 0 as libc::c_int;
    while i < LPC_order {
        a_fix_Q16[i as usize] = silk_float2int(*pAR.offset(i as isize) * 65536.0f32);
        i += 1;
    }
    silk_A2NLSF(NLSF_Q15, a_fix_Q16.as_mut_ptr(), LPC_order);
}
#[no_mangle]
#[c2rust::src_loc = "54:1"]
pub unsafe extern "C" fn silk_NLSF2A_FLP(
    pAR: *mut libc::c_float,
    NLSF_Q15: *const i16,
    LPC_order: libc::c_int,
    arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut a_fix_Q12: [i16; 16] = [0; 16];
    silk_NLSF2A(a_fix_Q12.as_mut_ptr(), NLSF_Q15, LPC_order, arch);
    i = 0 as libc::c_int;
    while i < LPC_order {
        *pAR.offset(i as isize) = a_fix_Q12[i as usize] as libc::c_float * (1.0f32 / 4096.0f32);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn silk_process_NLSFs_FLP(
    psEncC: *mut silk_encoder_state,
    PredCoef: *mut [libc::c_float; 16],
    NLSF_Q15: *mut i16,
    prev_NLSF_Q15: *const i16,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut PredCoef_Q12: [[i16; 16]; 2] = [[0; 16]; 2];
    silk_process_NLSFs(psEncC, PredCoef_Q12.as_mut_ptr(), NLSF_Q15, prev_NLSF_Q15);
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*psEncC).predictLPCOrder {
            (*PredCoef.offset(j as isize))[i as usize] =
                PredCoef_Q12[j as usize][i as usize] as libc::c_float * (1.0f32 / 4096.0f32);
            i += 1;
        }
        j += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "96:1"]
pub unsafe extern "C" fn silk_NSQ_wrapper_FLP(
    psEnc: *mut silk_encoder_state_FLP,
    psEncCtrl: *mut silk_encoder_control_FLP,
    psIndices: *mut SideInfoIndices,
    psNSQ: *mut silk_nsq_state,
    pulses: *mut i8,
    x: *const libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x16: [i16; 320] = [0; 320];
    let mut Gains_Q16: [i32; 4] = [0; 4];
    let mut PredCoef_Q12: [[i16; 16]; 2] = [[0; 16]; 2];
    let mut LTPCoef_Q14: [i16; 20] = [0; 20];
    let mut LTP_scale_Q14: libc::c_int = 0;
    let mut AR_Q13: [i16; 96] = [0; 96];
    let mut LF_shp_Q14: [i32; 4] = [0; 4];
    let mut Lambda_Q10: libc::c_int = 0;
    let mut Tilt_Q14: [libc::c_int; 4] = [0; 4];
    let mut HarmShapeGain_Q14: [libc::c_int; 4] = [0; 4];
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.nb_subfr {
        j = 0 as libc::c_int;
        while j < (*psEnc).sCmn.shapingLPCOrder {
            AR_Q13[(i * MAX_SHAPE_LPC_ORDER + j) as usize] =
                silk_float2int((*psEncCtrl).AR[(i * MAX_SHAPE_LPC_ORDER + j) as usize] * 8192.0f32)
                    as i16;
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.nb_subfr {
        LF_shp_Q14[i as usize] = ((silk_float2int((*psEncCtrl).LF_AR_shp[i as usize] * 16384.0f32)
            as u32)
            << 16 as libc::c_int) as i32
            | silk_float2int((*psEncCtrl).LF_MA_shp[i as usize] * 16384.0f32) as u16 as libc::c_int;
        Tilt_Q14[i as usize] = silk_float2int((*psEncCtrl).Tilt[i as usize] * 16384.0f32);
        HarmShapeGain_Q14[i as usize] =
            silk_float2int((*psEncCtrl).HarmShapeGain[i as usize] * 16384.0f32);
        i += 1;
    }
    Lambda_Q10 = silk_float2int((*psEncCtrl).Lambda * 1024.0f32);
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.nb_subfr * LTP_ORDER {
        LTPCoef_Q14[i as usize] =
            silk_float2int((*psEncCtrl).LTPCoef[i as usize] * 16384.0f32) as i16;
        i += 1;
    }
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*psEnc).sCmn.predictLPCOrder {
            PredCoef_Q12[j as usize][i as usize] =
                silk_float2int((*psEncCtrl).PredCoef[j as usize][i as usize] * 4096.0f32) as i16;
            i += 1;
        }
        j += 1;
    }
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.nb_subfr {
        Gains_Q16[i as usize] = silk_float2int((*psEncCtrl).Gains[i as usize] * 65536.0f32);
        i += 1;
    }
    if (*psIndices).signalType as libc::c_int == TYPE_VOICED {
        LTP_scale_Q14 =
            silk_LTPScales_table_Q14[(*psIndices).LTP_scaleIndex as usize] as libc::c_int;
    } else {
        LTP_scale_Q14 = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.frame_length {
        x16[i as usize] = silk_float2int(*x.offset(i as isize)) as i16;
        i += 1;
    }
    if (*psEnc).sCmn.nStatesDelayedDecision > 1 as libc::c_int
        || (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int
    {
        silk_NSQ_del_dec_c(
            &mut (*psEnc).sCmn,
            psNSQ,
            psIndices,
            x16.as_mut_ptr() as *const i16,
            pulses,
            (PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr() as *const i16,
            LTPCoef_Q14.as_mut_ptr() as *const i16,
            AR_Q13.as_mut_ptr() as *const i16,
            HarmShapeGain_Q14.as_mut_ptr() as *const libc::c_int,
            Tilt_Q14.as_mut_ptr() as *const libc::c_int,
            LF_shp_Q14.as_mut_ptr() as *const i32,
            Gains_Q16.as_mut_ptr() as *const i32,
            ((*psEncCtrl).pitchL).as_mut_ptr() as *const libc::c_int,
            Lambda_Q10,
            LTP_scale_Q14,
        );
    } else {
        silk_NSQ_c(
            &mut (*psEnc).sCmn,
            psNSQ,
            psIndices,
            x16.as_mut_ptr() as *const i16,
            pulses,
            (PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr() as *const i16,
            LTPCoef_Q14.as_mut_ptr() as *const i16,
            AR_Q13.as_mut_ptr() as *const i16,
            HarmShapeGain_Q14.as_mut_ptr() as *const libc::c_int,
            Tilt_Q14.as_mut_ptr() as *const libc::c_int,
            LF_shp_Q14.as_mut_ptr() as *const i32,
            Gains_Q16.as_mut_ptr() as *const i32,
            ((*psEncCtrl).pitchL).as_mut_ptr() as *const libc::c_int,
            Lambda_Q10,
            LTP_scale_Q14,
        );
    };
}
#[no_mangle]
#[c2rust::src_loc = "175:1"]
pub unsafe extern "C" fn silk_quant_LTP_gains_FLP(
    B: *mut libc::c_float,
    cbk_index: *mut i8,
    periodicity_index: *mut i8,
    sum_log_gain_Q7: *mut i32,
    pred_gain_dB: *mut libc::c_float,
    XX: *const libc::c_float,
    xX: *const libc::c_float,
    subfr_len: libc::c_int,
    nb_subfr: libc::c_int,
    arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut pred_gain_dB_Q7: libc::c_int = 0;
    let mut B_Q14: [i16; 20] = [0; 20];
    let mut XX_Q17: [i32; 100] = [0; 100];
    let mut xX_Q17: [i32; 20] = [0; 20];
    i = 0 as libc::c_int;
    while i < nb_subfr * LTP_ORDER * LTP_ORDER {
        XX_Q17[i as usize] = silk_float2int(*XX.offset(i as isize) * 131072.0f32);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nb_subfr * LTP_ORDER {
        xX_Q17[i as usize] = silk_float2int(*xX.offset(i as isize) * 131072.0f32);
        i += 1;
    }
    silk_quant_LTP_gains(
        B_Q14.as_mut_ptr(),
        cbk_index,
        periodicity_index,
        sum_log_gain_Q7,
        &mut pred_gain_dB_Q7,
        XX_Q17.as_mut_ptr() as *const i32,
        xX_Q17.as_mut_ptr() as *const i32,
        subfr_len,
        nb_subfr,
        arch,
    );
    i = 0 as libc::c_int;
    while i < nb_subfr * LTP_ORDER {
        *B.offset(i as isize) = B_Q14[i as usize] as libc::c_float * (1.0f32 / 16384.0f32);
        i += 1;
    }
    *pred_gain_dB = pred_gain_dB_Q7 as libc::c_float * (1.0f32 / 128.0f32);
}
