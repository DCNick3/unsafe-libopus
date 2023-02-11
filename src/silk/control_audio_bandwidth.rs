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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "90:9"]
    pub const MAX_NB_SUBFR: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "94:9"]
    pub const SUB_FRAME_LENGTH_MS: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "96:9"]
    pub const MAX_FRAME_LENGTH_MS: libc::c_int = SUB_FRAME_LENGTH_MS * MAX_NB_SUBFR;
    #[c2rust::src_loc = "215:9"]
    pub const TRANSITION_TIME_MS: libc::c_int = 5120 as libc::c_int;
    #[c2rust::src_loc = "219:9"]
    pub const TRANSITION_FRAMES: libc::c_int = TRANSITION_TIME_MS / MAX_FRAME_LENGTH_MS;
}
pub use self::define_h::{
    MAX_FRAME_LENGTH_MS, MAX_NB_SUBFR, SUB_FRAME_LENGTH_MS, TRANSITION_FRAMES, TRANSITION_TIME_MS,
};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
use crate::silk::enc_API::silk_EncControlStruct;

pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    SideInfoIndices,
};

use crate::externs::memset;
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_control_audio_bandwidth(
    mut psEncC: *mut silk_encoder_state,
    mut encControl: *mut silk_EncControlStruct,
) -> libc::c_int {
    let mut fs_kHz: libc::c_int = 0;
    let mut orig_kHz: libc::c_int = 0;
    let mut fs_Hz: i32 = 0;
    orig_kHz = (*psEncC).fs_kHz;
    if orig_kHz == 0 as libc::c_int {
        orig_kHz = (*psEncC).sLP.saved_fs_kHz;
    }
    fs_kHz = orig_kHz;
    fs_Hz = fs_kHz as i16 as i32 * 1000 as libc::c_int as i16 as i32;
    if fs_Hz == 0 as libc::c_int {
        fs_Hz = if (*psEncC).desiredInternal_fs_Hz < (*psEncC).API_fs_Hz {
            (*psEncC).desiredInternal_fs_Hz
        } else {
            (*psEncC).API_fs_Hz
        };
        fs_kHz = fs_Hz / 1000 as libc::c_int;
    } else if fs_Hz > (*psEncC).API_fs_Hz
        || fs_Hz > (*psEncC).maxInternal_fs_Hz
        || fs_Hz < (*psEncC).minInternal_fs_Hz
    {
        fs_Hz = (*psEncC).API_fs_Hz;
        fs_Hz = if fs_Hz < (*psEncC).maxInternal_fs_Hz {
            fs_Hz
        } else {
            (*psEncC).maxInternal_fs_Hz
        };
        fs_Hz = if fs_Hz > (*psEncC).minInternal_fs_Hz {
            fs_Hz
        } else {
            (*psEncC).minInternal_fs_Hz
        };
        fs_kHz = fs_Hz / 1000 as libc::c_int;
    } else {
        if (*psEncC).sLP.transition_frame_no >= TRANSITION_FRAMES {
            (*psEncC).sLP.mode = 0 as libc::c_int;
        }
        if (*psEncC).allow_bandwidth_switch != 0 || (*encControl).opusCanSwitch != 0 {
            if orig_kHz as i16 as i32 * 1000 as libc::c_int as i16 as i32
                > (*psEncC).desiredInternal_fs_Hz
            {
                if (*psEncC).sLP.mode == 0 as libc::c_int {
                    (*psEncC).sLP.transition_frame_no = TRANSITION_FRAMES;
                    memset(
                        ((*psEncC).sLP.In_LP_State).as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[i32; 2]>() as libc::c_ulong,
                    );
                }
                if (*encControl).opusCanSwitch != 0 {
                    (*psEncC).sLP.mode = 0 as libc::c_int;
                    fs_kHz = if orig_kHz == 16 as libc::c_int {
                        12 as libc::c_int
                    } else {
                        8 as libc::c_int
                    };
                } else if (*psEncC).sLP.transition_frame_no <= 0 as libc::c_int {
                    (*encControl).switchReady = 1 as libc::c_int;
                    (*encControl).maxBits -= (*encControl).maxBits * 5 as libc::c_int
                        / ((*encControl).payloadSize_ms + 5 as libc::c_int);
                } else {
                    (*psEncC).sLP.mode = -(2 as libc::c_int);
                }
            } else if (orig_kHz as i16 as i32 * 1000 as libc::c_int as i16 as i32)
                < (*psEncC).desiredInternal_fs_Hz
            {
                if (*encControl).opusCanSwitch != 0 {
                    fs_kHz = if orig_kHz == 8 as libc::c_int {
                        12 as libc::c_int
                    } else {
                        16 as libc::c_int
                    };
                    (*psEncC).sLP.transition_frame_no = 0 as libc::c_int;
                    memset(
                        ((*psEncC).sLP.In_LP_State).as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[i32; 2]>() as libc::c_ulong,
                    );
                    (*psEncC).sLP.mode = 1 as libc::c_int;
                } else if (*psEncC).sLP.mode == 0 as libc::c_int {
                    (*encControl).switchReady = 1 as libc::c_int;
                    (*encControl).maxBits -= (*encControl).maxBits * 5 as libc::c_int
                        / ((*encControl).payloadSize_ms + 5 as libc::c_int);
                } else {
                    (*psEncC).sLP.mode = 1 as libc::c_int;
                }
            } else if (*psEncC).sLP.mode < 0 as libc::c_int {
                (*psEncC).sLP.mode = 1 as libc::c_int;
            }
        }
    }
    return fs_kHz;
}
