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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:32"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = u32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: u32,
        pub end_offs: u32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: u32,
        pub rng: u32,
        pub val: u32,
        pub ext: u32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "47:1"]
    pub type ec_enc = ec_ctx;
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:32"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "208:9"]
    pub const NLSF_QUANT_MAX_AMPLITUDE: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "90:9"]
    pub const MAX_NB_SUBFR: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "77:9"]
    pub const CODE_CONDITIONALLY: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "75:9"]
    pub const CODE_INDEPENDENTLY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
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
    extern "C" {
        #[c2rust::src_loc = "40:26"]
        pub static silk_gain_iCDF: [[u8; 8]; 3];
        #[c2rust::src_loc = "41:26"]
        pub static silk_delta_gain_iCDF: [u8; 41];
        #[c2rust::src_loc = "43:26"]
        pub static silk_pitch_lag_iCDF: [u8; 32];
        #[c2rust::src_loc = "44:26"]
        pub static silk_pitch_delta_iCDF: [u8; 21];
        #[c2rust::src_loc = "69:26"]
        pub static silk_uniform4_iCDF: [u8; 4];
        #[c2rust::src_loc = "72:26"]
        pub static silk_uniform8_iCDF: [u8; 8];
        #[c2rust::src_loc = "74:26"]
        pub static silk_NLSF_EXT_iCDF: [u8; 7];
        #[c2rust::src_loc = "76:26"]
        pub static silk_LTP_per_index_iCDF: [u8; 3];
        #[c2rust::src_loc = "77:34"]
        pub static silk_LTP_gain_iCDF_ptrs: [*const u8; 3];
        #[c2rust::src_loc = "83:26"]
        pub static silk_LTPscale_iCDF: [u8; 3];
        #[c2rust::src_loc = "86:26"]
        pub static silk_type_offset_VAD_iCDF: [u8; 4];
        #[c2rust::src_loc = "87:26"]
        pub static silk_type_offset_no_VAD_iCDF: [u8; 2];
        #[c2rust::src_loc = "95:26"]
        pub static silk_NLSF_interpolation_factor_iCDF: [u8; 5];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use super::structs_h::silk_NLSF_CB_struct;
    extern "C" {
        #[c2rust::src_loc = "373:1"]
        pub fn silk_NLSF_unpack(
            ec_ix: *mut i16,
            pred_Q8: *mut u8,
            psNLSF_CB: *const silk_NLSF_CB_struct,
            CB1_index: libc::c_int,
        );
    }
}
use self::arch_h::celt_fatal;
pub use self::define_h::{
    CODE_CONDITIONALLY, CODE_INDEPENDENTLY, MAX_NB_SUBFR, NLSF_QUANT_MAX_AMPLITUDE, TYPE_VOICED,
};
pub use self::entcode_h::{ec_ctx, ec_enc, ec_window};
use self::entenc_h::ec_enc_icdf;
use self::main_h::silk_NLSF_unpack;
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};

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

#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_encode_indices(
    mut psEncC: *mut silk_encoder_state,
    psRangeEnc: *mut ec_enc,
    FrameIndex: libc::c_int,
    encode_LBRR: libc::c_int,
    condCoding: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut typeOffset: libc::c_int = 0;
    let mut encode_absolute_lagIndex: libc::c_int = 0;
    let mut delta_lagIndex: libc::c_int = 0;
    let mut ec_ix: [i16; 16] = [0; 16];
    let mut pred_Q8: [u8; 16] = [0; 16];
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
    if !(typeOffset >= 0 as libc::c_int && typeOffset < 6 as libc::c_int) {
        celt_fatal(
            b"assertion failed: typeOffset >= 0 && typeOffset < 6\0" as *const u8
                as *const libc::c_char,
            b"silk/encode_indices.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
        );
    }
    if !(encode_LBRR == 0 as libc::c_int || typeOffset >= 2 as libc::c_int) {
        celt_fatal(
            b"assertion failed: encode_LBRR == 0 || typeOffset >= 2\0" as *const u8
                as *const libc::c_char,
            b"silk/encode_indices.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
        );
    }
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
    if condCoding == CODE_CONDITIONALLY {
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
    if !((*(*psEncC).psNLSF_CB).order as libc::c_int == (*psEncC).predictLPCOrder) {
        celt_fatal(
            b"assertion failed: psEncC->psNLSF_CB->order == psEncC->predictLPCOrder\0" as *const u8
                as *const libc::c_char,
            b"silk/encode_indices.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < (*(*psEncC).psNLSF_CB).order as libc::c_int {
        if (*psIndices).NLSFIndices[(i + 1 as libc::c_int) as usize] as libc::c_int
            >= NLSF_QUANT_MAX_AMPLITUDE
        {
            ec_enc_icdf(
                psRangeEnc,
                2 as libc::c_int * NLSF_QUANT_MAX_AMPLITUDE,
                &*((*(*psEncC).psNLSF_CB).ec_iCDF)
                    .offset(*ec_ix.as_mut_ptr().offset(i as isize) as isize),
                8 as libc::c_int as libc::c_uint,
            );
            ec_enc_icdf(
                psRangeEnc,
                (*psIndices).NLSFIndices[(i + 1 as libc::c_int) as usize] as libc::c_int
                    - NLSF_QUANT_MAX_AMPLITUDE,
                silk_NLSF_EXT_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        } else if (*psIndices).NLSFIndices[(i + 1 as libc::c_int) as usize] as libc::c_int
            <= -NLSF_QUANT_MAX_AMPLITUDE
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
                    - NLSF_QUANT_MAX_AMPLITUDE,
                silk_NLSF_EXT_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        } else {
            ec_enc_icdf(
                psRangeEnc,
                (*psIndices).NLSFIndices[(i + 1 as libc::c_int) as usize] as libc::c_int
                    + NLSF_QUANT_MAX_AMPLITUDE,
                &*((*(*psEncC).psNLSF_CB).ec_iCDF)
                    .offset(*ec_ix.as_mut_ptr().offset(i as isize) as isize),
                8 as libc::c_int as libc::c_uint,
            );
        }
        i += 1;
    }
    if (*psEncC).nb_subfr == MAX_NB_SUBFR {
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).NLSFInterpCoef_Q2 as libc::c_int,
            silk_NLSF_interpolation_factor_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
    }
    if (*psIndices).signalType as libc::c_int == TYPE_VOICED {
        encode_absolute_lagIndex = 1 as libc::c_int;
        if condCoding == CODE_CONDITIONALLY && (*psEncC).ec_prevSignalType == TYPE_VOICED {
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
            let mut pitch_high_bits: i32 = 0;
            let mut pitch_low_bits: i32 = 0;
            pitch_high_bits =
                (*psIndices).lagIndex as libc::c_int / ((*psEncC).fs_kHz >> 1 as libc::c_int);
            pitch_low_bits = (*psIndices).lagIndex as libc::c_int
                - pitch_high_bits as i16 as i32
                    * ((*psEncC).fs_kHz >> 1 as libc::c_int) as i16 as i32;
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
        if condCoding == CODE_INDEPENDENTLY {
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
