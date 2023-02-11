use crate::silk::resampler_structs::silk_resampler_state_struct;

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
#[c2rust::src_loc = "232:9"]
pub struct silk_PLC_struct {
    pub pitchL_Q8: i32,
    pub LTPCoef_Q14: [i16; 5],
    pub prevLPC_Q12: [i16; 16],
    pub last_frame_lost: libc::c_int,
    pub rand_seed: i32,
    pub randScale_Q14: i16,
    pub conc_energy: i32,
    pub conc_energy_shift: libc::c_int,
    pub prevLTP_scale_Q14: i16,
    pub prevGain_Q16: [i32; 2],
    pub fs_kHz: libc::c_int,
    pub nb_subfr: libc::c_int,
    pub subfr_length: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "249:9"]
pub struct silk_CNG_struct {
    pub CNG_exc_buf_Q14: [i32; 320],
    pub CNG_smth_NLSF_Q15: [i16; 16],
    pub CNG_synth_state: [i32; 16],
    pub CNG_smth_Gain_Q16: i32,
    pub rand_seed: i32,
    pub fs_kHz: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "261:9"]
pub struct silk_decoder_state {
    pub prev_gain_Q16: i32,
    pub exc_Q14: [i32; 320],
    pub sLPC_Q14_buf: [i32; 16],
    pub outBuf: [i16; 480],
    pub lagPrev: libc::c_int,
    pub LastGainIndex: i8,
    pub fs_kHz: libc::c_int,
    pub fs_API_hz: i32,
    pub nb_subfr: libc::c_int,
    pub frame_length: libc::c_int,
    pub subfr_length: libc::c_int,
    pub ltp_mem_length: libc::c_int,
    pub LPC_order: libc::c_int,
    pub prevNLSF_Q15: [i16; 16],
    pub first_frame_after_reset: libc::c_int,
    pub pitch_lag_low_bits_iCDF: *const u8,
    pub pitch_contour_iCDF: *const u8,
    pub nFramesDecoded: libc::c_int,
    pub nFramesPerPacket: libc::c_int,
    pub ec_prevSignalType: libc::c_int,
    pub ec_prevLagIndex: i16,
    pub VAD_flags: [libc::c_int; 3],
    pub LBRR_flag: libc::c_int,
    pub LBRR_flags: [libc::c_int; 3],
    pub resampler_state: silk_resampler_state_struct,
    pub psNLSF_CB: *const silk_NLSF_CB_struct,
    pub indices: SideInfoIndices,
    pub sCNG: silk_CNG_struct,
    pub lossCnt: libc::c_int,
    pub prevSignalType: libc::c_int,
    pub arch: libc::c_int,
    pub sPLC: silk_PLC_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "314:9"]
pub struct silk_decoder_control {
    pub pitchL: [libc::c_int; 4],
    pub Gains_Q16: [i32; 4],
    pub PredCoef_Q12: [[i16; 16]; 2],
    pub LTPCoef_Q14: [i16; 20],
    pub LTP_scale_Q14: libc::c_int,
}
