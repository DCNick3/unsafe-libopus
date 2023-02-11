use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:33"]
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
    use crate::silk::resampler_structs::silk_resampler_state_struct;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/float.h:32"]
pub mod float_h {
    #[c2rust::src_loc = "121:9"]
    pub const FLT_MAX: libc::c_float = __FLT_MAX__;
    use super::internal::__FLT_MAX__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "37:10"]
    pub const silk_float_MAX: libc::c_float = FLT_MAX;
    use super::float_h::FLT_MAX;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "90:9"]
    pub const MAX_NB_SUBFR: libc::c_int = 4 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
pub mod arch_h {}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:33"]
pub mod SigProc_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "102:1"]
        pub fn silk_burg_modified_FLP(
            A: *mut libc::c_float,
            x: *const libc::c_float,
            minInvGain: libc::c_float,
            subfr_length: libc::c_int,
            nb_subfr: libc::c_int,
            D: libc::c_int,
        ) -> libc::c_float;
        #[c2rust::src_loc = "134:1"]
        pub fn silk_energy_FLP(data: *const libc::c_float, dataSize: libc::c_int)
            -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:33"]
pub mod main_h {
    extern "C" {
        #[c2rust::src_loc = "202:1"]
        pub fn silk_interpolate(
            xi: *mut i16,
            x0: *const i16,
            x1: *const i16,
            ifact_Q2: libc::c_int,
            d: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/main_FLP.h:33"]
pub mod main_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "250:1"]
        pub fn silk_A2NLSF_FLP(
            NLSF_Q15: *mut i16,
            pAR: *const libc::c_float,
            LPC_order: libc::c_int,
        );
        #[c2rust::src_loc = "178:1"]
        pub fn silk_LPC_analysis_filter_FLP(
            r_LPC: *mut libc::c_float,
            PredCoef: *const libc::c_float,
            s: *const libc::c_float,
            length: libc::c_int,
            Order: libc::c_int,
        );
        #[c2rust::src_loc = "257:1"]
        pub fn silk_NLSF2A_FLP(
            pAR: *mut libc::c_float,
            NLSF_Q15: *const i16,
            LPC_order: libc::c_int,
            arch: libc::c_int,
        );
    }
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "133:9"]
    pub const __FLT_MAX__: libc::c_float = 3.40282347e+38f32;
}
pub use self::define_h::MAX_NB_SUBFR;
pub use self::float_h::FLT_MAX;
pub use self::internal::__FLT_MAX__;
use self::main_FLP_h::{silk_A2NLSF_FLP, silk_LPC_analysis_filter_FLP, silk_NLSF2A_FLP};
use self::main_h::silk_interpolate;
use crate::celt::celt::celt_fatal;

pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    SideInfoIndices,
};
pub use self::typedef_h::silk_float_MAX;

use self::SigProc_FLP_h::{silk_burg_modified_FLP, silk_energy_FLP};
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn silk_find_LPC_FLP(
    mut psEncC: *mut silk_encoder_state,
    NLSF_Q15: *mut i16,
    x: *const libc::c_float,
    minInvGain: libc::c_float,
) {
    let mut k: libc::c_int = 0;
    let mut subfr_length: libc::c_int = 0;
    let mut a: [libc::c_float; 16] = [0.; 16];
    let mut res_nrg: libc::c_float = 0.;
    let mut res_nrg_2nd: libc::c_float = 0.;
    let mut res_nrg_interp: libc::c_float = 0.;
    let mut NLSF0_Q15: [i16; 16] = [0; 16];
    let mut a_tmp: [libc::c_float; 16] = [0.; 16];
    let mut LPC_res: [libc::c_float; 384] = [0.; 384];
    subfr_length = (*psEncC).subfr_length + (*psEncC).predictLPCOrder;
    (*psEncC).indices.NLSFInterpCoef_Q2 = 4 as libc::c_int as i8;
    res_nrg = silk_burg_modified_FLP(
        a.as_mut_ptr(),
        x,
        minInvGain,
        subfr_length,
        (*psEncC).nb_subfr,
        (*psEncC).predictLPCOrder,
    );
    if (*psEncC).useInterpolatedNLSFs != 0
        && (*psEncC).first_frame_after_reset == 0
        && (*psEncC).nb_subfr == MAX_NB_SUBFR
    {
        res_nrg -= silk_burg_modified_FLP(
            a_tmp.as_mut_ptr(),
            x.offset((MAX_NB_SUBFR / 2 as libc::c_int * subfr_length) as isize),
            minInvGain,
            subfr_length,
            MAX_NB_SUBFR / 2 as libc::c_int,
            (*psEncC).predictLPCOrder,
        );
        silk_A2NLSF_FLP(NLSF_Q15, a_tmp.as_mut_ptr(), (*psEncC).predictLPCOrder);
        res_nrg_2nd = silk_float_MAX;
        k = 3 as libc::c_int;
        while k >= 0 as libc::c_int {
            silk_interpolate(
                NLSF0_Q15.as_mut_ptr(),
                ((*psEncC).prev_NLSFq_Q15).as_mut_ptr() as *const i16,
                NLSF_Q15 as *const i16,
                k,
                (*psEncC).predictLPCOrder,
            );
            silk_NLSF2A_FLP(
                a_tmp.as_mut_ptr(),
                NLSF0_Q15.as_mut_ptr(),
                (*psEncC).predictLPCOrder,
                (*psEncC).arch,
            );
            silk_LPC_analysis_filter_FLP(
                LPC_res.as_mut_ptr(),
                a_tmp.as_mut_ptr() as *const libc::c_float,
                x,
                2 as libc::c_int * subfr_length,
                (*psEncC).predictLPCOrder,
            );
            res_nrg_interp = (silk_energy_FLP(
                LPC_res
                    .as_mut_ptr()
                    .offset((*psEncC).predictLPCOrder as isize),
                subfr_length - (*psEncC).predictLPCOrder,
            ) + silk_energy_FLP(
                LPC_res
                    .as_mut_ptr()
                    .offset((*psEncC).predictLPCOrder as isize)
                    .offset(subfr_length as isize),
                subfr_length - (*psEncC).predictLPCOrder,
            )) as libc::c_float;
            if res_nrg_interp < res_nrg {
                res_nrg = res_nrg_interp;
                (*psEncC).indices.NLSFInterpCoef_Q2 = k as i8;
            } else if res_nrg_interp > res_nrg_2nd {
                break;
            }
            res_nrg_2nd = res_nrg_interp;
            k -= 1;
        }
    }
    if (*psEncC).indices.NLSFInterpCoef_Q2 as libc::c_int == 4 as libc::c_int {
        silk_A2NLSF_FLP(NLSF_Q15, a.as_mut_ptr(), (*psEncC).predictLPCOrder);
    }
    if !((*psEncC).indices.NLSFInterpCoef_Q2 as libc::c_int == 4 as libc::c_int
        || (*psEncC).useInterpolatedNLSFs != 0
            && (*psEncC).first_frame_after_reset == 0
            && (*psEncC).nb_subfr == 4 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: psEncC->indices.NLSFInterpCoef_Q2 == 4 || ( psEncC->useInterpolatedNLSFs && !psEncC->first_frame_after_reset && psEncC->nb_subfr == MAX_NB_SUBFR )\0"
                as *const u8 as *const libc::c_char,
            b"silk/float/find_LPC_FLP.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
        );
    }
}
