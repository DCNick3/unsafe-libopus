use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    #[c2rust::src_loc = "52:4"]
    pub type opus_uint8 = uint8_t;
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    use super::stdint_intn_h::{int16_t, int32_t, int8_t};
    use super::stdint_uintn_h::{uint32_t, uint8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:32"]
pub mod resampler_structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [opus_int32; 6],
        pub sFIR: C2RustUnnamed,
        pub delayBuf: [opus_int16; 48],
        pub resampler_function: libc::c_int,
        pub batchSize: libc::c_int,
        pub invRatio_Q16: opus_int32,
        pub FIR_Order: libc::c_int,
        pub FIR_Fracs: libc::c_int,
        pub Fs_in_kHz: libc::c_int,
        pub Fs_out_kHz: libc::c_int,
        pub inputDelay: libc::c_int,
        pub Coefs: *const opus_int16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:5"]
    pub union C2RustUnnamed {
        pub i32_0: [opus_int32; 36],
        pub i16_0: [opus_int16; 36],
    }
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
    use super::opus_types_h::{opus_int16, opus_int32};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:32"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = opus_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: opus_uint32,
        pub end_offs: opus_uint32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: opus_uint32,
        pub rng: opus_uint32,
        pub val: opus_uint32,
        pub ext: opus_uint32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "47:1"]
    pub type ec_enc = ec_ctx;
    use super::opus_types_h::opus_uint32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:9"]
    pub struct silk_nsq_state {
        pub xq: [opus_int16; 640],
        pub sLTP_shp_Q14: [opus_int32; 640],
        pub sLPC_Q14: [opus_int32; 96],
        pub sAR2_Q14: [opus_int32; 24],
        pub sLF_AR_shp_Q14: opus_int32,
        pub sDiff_shp_Q14: opus_int32,
        pub lagPrev: libc::c_int,
        pub sLTP_buf_idx: libc::c_int,
        pub sLTP_shp_buf_idx: libc::c_int,
        pub rand_seed: opus_int32,
        pub prev_gain_Q16: opus_int32,
        pub rewhite_flag: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:9"]
    pub struct silk_VAD_state {
        pub AnaState: [opus_int32; 2],
        pub AnaState1: [opus_int32; 2],
        pub AnaState2: [opus_int32; 2],
        pub XnrgSubfr: [opus_int32; 4],
        pub NrgRatioSmth_Q8: [opus_int32; 4],
        pub HPstate: opus_int16,
        pub NL: [opus_int32; 4],
        pub inv_NL: [opus_int32; 4],
        pub NoiseLevelBias: [opus_int32; 4],
        pub counter: opus_int32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:9"]
    pub struct silk_LP_state {
        pub In_LP_State: [opus_int32; 2],
        pub transition_frame_no: opus_int32,
        pub mode: libc::c_int,
        pub saved_fs_kHz: opus_int32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:9"]
    pub struct silk_NLSF_CB_struct {
        pub nVectors: opus_int16,
        pub order: opus_int16,
        pub quantStepSize_Q16: opus_int16,
        pub invQuantStepSize_Q6: opus_int16,
        pub CB1_NLSF_Q8: *const opus_uint8,
        pub CB1_Wght_Q9: *const opus_int16,
        pub CB1_iCDF: *const opus_uint8,
        pub pred_Q8: *const opus_uint8,
        pub ec_sel: *const opus_uint8,
        pub ec_iCDF: *const opus_uint8,
        pub ec_Rates_Q5: *const opus_uint8,
        pub deltaMin_Q15: *const opus_int16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:9"]
    pub struct SideInfoIndices {
        pub GainsIndices: [opus_int8; 4],
        pub LTPIndex: [opus_int8; 4],
        pub NLSFIndices: [opus_int8; 17],
        pub lagIndex: opus_int16,
        pub contourIndex: opus_int8,
        pub signalType: opus_int8,
        pub quantOffsetType: opus_int8,
        pub NLSFInterpCoef_Q2: opus_int8,
        pub PERIndex: opus_int8,
        pub LTP_scaleIndex: opus_int8,
        pub Seed: opus_int8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:9"]
    pub struct silk_encoder_state {
        pub In_HP_State: [opus_int32; 2],
        pub variable_HP_smth1_Q15: opus_int32,
        pub variable_HP_smth2_Q15: opus_int32,
        pub sLP: silk_LP_state,
        pub sVAD: silk_VAD_state,
        pub sNSQ: silk_nsq_state,
        pub prev_NLSFq_Q15: [opus_int16; 16],
        pub speech_activity_Q8: libc::c_int,
        pub allow_bandwidth_switch: libc::c_int,
        pub LBRRprevLastGainIndex: opus_int8,
        pub prevSignalType: opus_int8,
        pub prevLag: libc::c_int,
        pub pitch_LPC_win_length: libc::c_int,
        pub max_pitch_lag: libc::c_int,
        pub API_fs_Hz: opus_int32,
        pub prev_API_fs_Hz: opus_int32,
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
        pub TargetRate_bps: opus_int32,
        pub PacketSize_ms: libc::c_int,
        pub PacketLoss_perc: libc::c_int,
        pub frameCounter: opus_int32,
        pub Complexity: libc::c_int,
        pub nStatesDelayedDecision: libc::c_int,
        pub useInterpolatedNLSFs: libc::c_int,
        pub shapingLPCOrder: libc::c_int,
        pub predictLPCOrder: libc::c_int,
        pub pitchEstimationComplexity: libc::c_int,
        pub pitchEstimationLPCOrder: libc::c_int,
        pub pitchEstimationThreshold_Q16: opus_int32,
        pub sum_log_gain_Q7: opus_int32,
        pub NLSF_MSVQ_Survivors: libc::c_int,
        pub first_frame_after_reset: libc::c_int,
        pub controlled_since_last_payload: libc::c_int,
        pub warping_Q16: libc::c_int,
        pub useCBR: libc::c_int,
        pub prefillFlag: libc::c_int,
        pub pitch_lag_low_bits_iCDF: *const opus_uint8,
        pub pitch_contour_iCDF: *const opus_uint8,
        pub psNLSF_CB: *const silk_NLSF_CB_struct,
        pub input_quality_bands_Q15: [libc::c_int; 4],
        pub input_tilt_Q15: libc::c_int,
        pub SNR_dB_Q7: libc::c_int,
        pub VAD_flags: [opus_int8; 3],
        pub LBRR_flag: opus_int8,
        pub LBRR_flags: [libc::c_int; 3],
        pub indices: SideInfoIndices,
        pub pulses: [opus_int8; 320],
        pub arch: libc::c_int,
        pub inputBuf: [opus_int16; 322],
        pub inputBufIx: libc::c_int,
        pub nFramesPerPacket: libc::c_int,
        pub nFramesEncoded: libc::c_int,
        pub nChannelsAPI: libc::c_int,
        pub nChannelsInternal: libc::c_int,
        pub channelNb: libc::c_int,
        pub frames_since_onset: libc::c_int,
        pub ec_prevSignalType: libc::c_int,
        pub ec_prevLagIndex: opus_int16,
        pub resampler_state: silk_resampler_state_struct,
        pub useDTX: libc::c_int,
        pub inDTX: libc::c_int,
        pub noSpeechCounter: libc::c_int,
        pub useInBandFEC: libc::c_int,
        pub LBRR_enabled: libc::c_int,
        pub LBRR_GainIncreases: libc::c_int,
        pub indices_LBRR: [SideInfoIndices; 3],
        pub pulses_LBRR: [[opus_int8; 320]; 3],
    }
    use super::opus_types_h::{opus_int16, opus_int32, opus_int8, opus_uint8};
    use super::resampler_structs_h::silk_resampler_state_struct;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.h:32"]
pub mod entenc_h {
    use super::entcode_h::ec_enc;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn ec_enc_icdf(
            _this: *mut ec_enc,
            _s: libc::c_int,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    use super::opus_types_h::opus_uint8;
    extern "C" {
        #[c2rust::src_loc = "40:26"]
        pub static silk_gain_iCDF: [[opus_uint8; 8]; 3];
        #[c2rust::src_loc = "41:26"]
        pub static silk_delta_gain_iCDF: [opus_uint8; 41];
        #[c2rust::src_loc = "43:26"]
        pub static silk_pitch_lag_iCDF: [opus_uint8; 32];
        #[c2rust::src_loc = "44:26"]
        pub static silk_pitch_delta_iCDF: [opus_uint8; 21];
        #[c2rust::src_loc = "69:26"]
        pub static silk_uniform4_iCDF: [opus_uint8; 4];
        #[c2rust::src_loc = "72:26"]
        pub static silk_uniform8_iCDF: [opus_uint8; 8];
        #[c2rust::src_loc = "74:26"]
        pub static silk_NLSF_EXT_iCDF: [opus_uint8; 7];
        #[c2rust::src_loc = "76:26"]
        pub static silk_LTP_per_index_iCDF: [opus_uint8; 3];
        #[c2rust::src_loc = "77:34"]
        pub static silk_LTP_gain_iCDF_ptrs: [*const opus_uint8; 3];
        #[c2rust::src_loc = "83:26"]
        pub static silk_LTPscale_iCDF: [opus_uint8; 3];
        #[c2rust::src_loc = "86:26"]
        pub static silk_type_offset_VAD_iCDF: [opus_uint8; 4];
        #[c2rust::src_loc = "87:26"]
        pub static silk_type_offset_no_VAD_iCDF: [opus_uint8; 2];
        #[c2rust::src_loc = "95:26"]
        pub static silk_NLSF_interpolation_factor_iCDF: [opus_uint8; 5];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use super::opus_types_h::{opus_int16, opus_uint8};
    use super::structs_h::silk_NLSF_CB_struct;
    extern "C" {
        #[c2rust::src_loc = "373:1"]
        pub fn silk_NLSF_unpack(
            ec_ix: *mut opus_int16,
            pred_Q8: *mut opus_uint8,
            psNLSF_CB: *const silk_NLSF_CB_struct,
            CB1_index: libc::c_int,
        );
    }
}
pub use self::entcode_h::{ec_ctx, ec_enc, ec_window};
use self::entenc_h::ec_enc_icdf;
use self::main_h::silk_NLSF_unpack;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int8, opus_uint32, opus_uint8};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    SideInfoIndices,
};
use self::tables_h::{
    silk_LTP_gain_iCDF_ptrs, silk_LTP_per_index_iCDF, silk_LTPscale_iCDF, silk_NLSF_EXT_iCDF,
    silk_NLSF_interpolation_factor_iCDF, silk_delta_gain_iCDF, silk_gain_iCDF,
    silk_pitch_delta_iCDF, silk_pitch_lag_iCDF, silk_type_offset_VAD_iCDF,
    silk_type_offset_no_VAD_iCDF, silk_uniform4_iCDF, silk_uniform8_iCDF,
};
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint32_t, __uint8_t};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_encode_indices(
    mut psEncC: *mut silk_encoder_state,
    mut psRangeEnc: *mut ec_enc,
    mut FrameIndex: libc::c_int,
    mut encode_LBRR: libc::c_int,
    mut condCoding: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut typeOffset: libc::c_int = 0;
    let mut encode_absolute_lagIndex: libc::c_int = 0;
    let mut delta_lagIndex: libc::c_int = 0;
    let mut ec_ix: [opus_int16; 16] = [0; 16];
    let mut pred_Q8: [opus_uint8; 16] = [0; 16];
    let mut psIndices: *const SideInfoIndices = 0 as *const SideInfoIndices;
    if encode_LBRR != 0 {
        psIndices = &mut *((*psEncC).indices_LBRR)
            .as_mut_ptr()
            .offset(FrameIndex as isize) as *mut SideInfoIndices;
    } else {
        psIndices = &mut (*psEncC).indices;
    }
    typeOffset = 2 as libc::c_int * (*psIndices).signalType as libc::c_int
        + (*psIndices).quantOffsetType as libc::c_int;
    if encode_LBRR != 0 || typeOffset >= 2 as libc::c_int {
        ec_enc_icdf(
            psRangeEnc,
            typeOffset - 2 as libc::c_int,
            silk_type_offset_VAD_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
    } else {
        ec_enc_icdf(
            psRangeEnc,
            typeOffset,
            silk_type_offset_no_VAD_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
    }
    if condCoding == 2 as libc::c_int {
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).GainsIndices[0 as libc::c_int as usize] as libc::c_int,
            silk_delta_gain_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
    } else {
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).GainsIndices[0 as libc::c_int as usize] as libc::c_int >> 3 as libc::c_int,
            (silk_gain_iCDF[(*psIndices).signalType as usize]).as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).GainsIndices[0 as libc::c_int as usize] as libc::c_int & 7 as libc::c_int,
            silk_uniform8_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
    }
    i = 1 as libc::c_int;
    while i < (*psEncC).nb_subfr {
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).GainsIndices[i as usize] as libc::c_int,
            silk_delta_gain_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
        i += 1;
    }
    ec_enc_icdf(
        psRangeEnc,
        (*psIndices).NLSFIndices[0 as libc::c_int as usize] as libc::c_int,
        &*((*(*psEncC).psNLSF_CB).CB1_iCDF).offset(
            (((*psIndices).signalType as libc::c_int >> 1 as libc::c_int)
                * (*(*psEncC).psNLSF_CB).nVectors as libc::c_int) as isize,
        ),
        8 as libc::c_int as libc::c_uint,
    );
    silk_NLSF_unpack(
        ec_ix.as_mut_ptr(),
        pred_Q8.as_mut_ptr(),
        (*psEncC).psNLSF_CB,
        (*psIndices).NLSFIndices[0 as libc::c_int as usize] as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < (*(*psEncC).psNLSF_CB).order as libc::c_int {
        if (*psIndices).NLSFIndices[(i + 1 as libc::c_int) as usize] as libc::c_int
            >= 4 as libc::c_int
        {
            ec_enc_icdf(
                psRangeEnc,
                2 as libc::c_int * 4 as libc::c_int,
                &*((*(*psEncC).psNLSF_CB).ec_iCDF)
                    .offset(*ec_ix.as_mut_ptr().offset(i as isize) as isize),
                8 as libc::c_int as libc::c_uint,
            );
            ec_enc_icdf(
                psRangeEnc,
                (*psIndices).NLSFIndices[(i + 1 as libc::c_int) as usize] as libc::c_int
                    - 4 as libc::c_int,
                silk_NLSF_EXT_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        } else if (*psIndices).NLSFIndices[(i + 1 as libc::c_int) as usize] as libc::c_int
            <= -(4 as libc::c_int)
        {
            ec_enc_icdf(
                psRangeEnc,
                0 as libc::c_int,
                &*((*(*psEncC).psNLSF_CB).ec_iCDF)
                    .offset(*ec_ix.as_mut_ptr().offset(i as isize) as isize),
                8 as libc::c_int as libc::c_uint,
            );
            ec_enc_icdf(
                psRangeEnc,
                -((*psIndices).NLSFIndices[(i + 1 as libc::c_int) as usize] as libc::c_int)
                    - 4 as libc::c_int,
                silk_NLSF_EXT_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        } else {
            ec_enc_icdf(
                psRangeEnc,
                (*psIndices).NLSFIndices[(i + 1 as libc::c_int) as usize] as libc::c_int
                    + 4 as libc::c_int,
                &*((*(*psEncC).psNLSF_CB).ec_iCDF)
                    .offset(*ec_ix.as_mut_ptr().offset(i as isize) as isize),
                8 as libc::c_int as libc::c_uint,
            );
        }
        i += 1;
    }
    if (*psEncC).nb_subfr == 4 as libc::c_int {
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).NLSFInterpCoef_Q2 as libc::c_int,
            silk_NLSF_interpolation_factor_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
    }
    if (*psIndices).signalType as libc::c_int == 2 as libc::c_int {
        encode_absolute_lagIndex = 1 as libc::c_int;
        if condCoding == 2 as libc::c_int && (*psEncC).ec_prevSignalType == 2 as libc::c_int {
            delta_lagIndex =
                (*psIndices).lagIndex as libc::c_int - (*psEncC).ec_prevLagIndex as libc::c_int;
            if delta_lagIndex < -(8 as libc::c_int) || delta_lagIndex > 11 as libc::c_int {
                delta_lagIndex = 0 as libc::c_int;
            } else {
                delta_lagIndex = delta_lagIndex + 9 as libc::c_int;
                encode_absolute_lagIndex = 0 as libc::c_int;
            }
            ec_enc_icdf(
                psRangeEnc,
                delta_lagIndex,
                silk_pitch_delta_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        }
        if encode_absolute_lagIndex != 0 {
            let mut pitch_high_bits: opus_int32 = 0;
            let mut pitch_low_bits: opus_int32 = 0;
            pitch_high_bits =
                (*psIndices).lagIndex as libc::c_int / ((*psEncC).fs_kHz >> 1 as libc::c_int);
            pitch_low_bits = (*psIndices).lagIndex as libc::c_int
                - pitch_high_bits as opus_int16 as opus_int32
                    * ((*psEncC).fs_kHz >> 1 as libc::c_int) as opus_int16 as opus_int32;
            ec_enc_icdf(
                psRangeEnc,
                pitch_high_bits,
                silk_pitch_lag_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
            ec_enc_icdf(
                psRangeEnc,
                pitch_low_bits,
                (*psEncC).pitch_lag_low_bits_iCDF,
                8 as libc::c_int as libc::c_uint,
            );
        }
        (*psEncC).ec_prevLagIndex = (*psIndices).lagIndex;
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).contourIndex as libc::c_int,
            (*psEncC).pitch_contour_iCDF,
            8 as libc::c_int as libc::c_uint,
        );
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).PERIndex as libc::c_int,
            silk_LTP_per_index_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
        k = 0 as libc::c_int;
        while k < (*psEncC).nb_subfr {
            ec_enc_icdf(
                psRangeEnc,
                (*psIndices).LTPIndex[k as usize] as libc::c_int,
                silk_LTP_gain_iCDF_ptrs[(*psIndices).PERIndex as usize],
                8 as libc::c_int as libc::c_uint,
            );
            k += 1;
        }
        if condCoding == 0 as libc::c_int {
            ec_enc_icdf(
                psRangeEnc,
                (*psIndices).LTP_scaleIndex as libc::c_int,
                silk_LTPscale_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        }
    }
    (*psEncC).ec_prevSignalType = (*psIndices).signalType as libc::c_int;
    ec_enc_icdf(
        psRangeEnc,
        (*psIndices).Seed as libc::c_int,
        silk_uniform4_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
}
