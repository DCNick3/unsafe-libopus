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
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int8_t, __int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
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
    use super::stdint_intn_h::{int8_t, int16_t, int32_t};
    use super::stdint_uintn_h::uint8_t;
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
    use super::opus_types_h::{opus_int32, opus_int16};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
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
    #[c2rust::src_loc = "232:9"]
    pub struct silk_PLC_struct {
        pub pitchL_Q8: opus_int32,
        pub LTPCoef_Q14: [opus_int16; 5],
        pub prevLPC_Q12: [opus_int16; 16],
        pub last_frame_lost: libc::c_int,
        pub rand_seed: opus_int32,
        pub randScale_Q14: opus_int16,
        pub conc_energy: opus_int32,
        pub conc_energy_shift: libc::c_int,
        pub prevLTP_scale_Q14: opus_int16,
        pub prevGain_Q16: [opus_int32; 2],
        pub fs_kHz: libc::c_int,
        pub nb_subfr: libc::c_int,
        pub subfr_length: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "249:9"]
    pub struct silk_CNG_struct {
        pub CNG_exc_buf_Q14: [opus_int32; 320],
        pub CNG_smth_NLSF_Q15: [opus_int16; 16],
        pub CNG_synth_state: [opus_int32; 16],
        pub CNG_smth_Gain_Q16: opus_int32,
        pub rand_seed: opus_int32,
        pub fs_kHz: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "261:9"]
    pub struct silk_decoder_state {
        pub prev_gain_Q16: opus_int32,
        pub exc_Q14: [opus_int32; 320],
        pub sLPC_Q14_buf: [opus_int32; 16],
        pub outBuf: [opus_int16; 480],
        pub lagPrev: libc::c_int,
        pub LastGainIndex: opus_int8,
        pub fs_kHz: libc::c_int,
        pub fs_API_hz: opus_int32,
        pub nb_subfr: libc::c_int,
        pub frame_length: libc::c_int,
        pub subfr_length: libc::c_int,
        pub ltp_mem_length: libc::c_int,
        pub LPC_order: libc::c_int,
        pub prevNLSF_Q15: [opus_int16; 16],
        pub first_frame_after_reset: libc::c_int,
        pub pitch_lag_low_bits_iCDF: *const opus_uint8,
        pub pitch_contour_iCDF: *const opus_uint8,
        pub nFramesDecoded: libc::c_int,
        pub nFramesPerPacket: libc::c_int,
        pub ec_prevSignalType: libc::c_int,
        pub ec_prevLagIndex: opus_int16,
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
    use super::opus_types_h::{opus_int16, opus_uint8, opus_int8, opus_int32};
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
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(
            _: *mut libc::c_void,
            _: libc::c_int,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    use super::resampler_structs_h::silk_resampler_state_struct;
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "62:1"]
        pub fn silk_resampler_init(
            S: *mut silk_resampler_state_struct,
            Fs_Hz_in: opus_int32,
            Fs_Hz_out: opus_int32,
            forEnc: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    use super::opus_types_h::{opus_uint8, opus_int16};
    use super::structs_h::silk_NLSF_CB_struct;
    extern "C" {
        #[c2rust::src_loc = "45:26"]
        pub static silk_pitch_contour_iCDF: [opus_uint8; 34];
        #[c2rust::src_loc = "46:26"]
        pub static silk_pitch_contour_NB_iCDF: [opus_uint8; 11];
        #[c2rust::src_loc = "47:26"]
        pub static silk_pitch_contour_10_ms_iCDF: [opus_uint8; 12];
        #[c2rust::src_loc = "48:26"]
        pub static silk_pitch_contour_10_ms_NB_iCDF: [opus_uint8; 3];
        #[c2rust::src_loc = "69:26"]
        pub static silk_uniform4_iCDF: [opus_uint8; 4];
        #[c2rust::src_loc = "71:26"]
        pub static silk_uniform6_iCDF: [opus_uint8; 6];
        #[c2rust::src_loc = "72:26"]
        pub static silk_uniform8_iCDF: [opus_uint8; 8];
        #[c2rust::src_loc = "97:34"]
        pub static silk_NLSF_CB_WB: silk_NLSF_CB_struct;
        #[c2rust::src_loc = "98:34"]
        pub static silk_NLSF_CB_NB_MB: silk_NLSF_CB_struct;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "70:9"]
    pub const TYPE_NO_VOICE_ACTIVITY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "142:9"]
    pub const MAX_LPC_ORDER: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "143:9"]
    pub const MIN_LPC_ORDER: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "90:9"]
    pub const MAX_NB_SUBFR: libc::c_int = 4 as libc::c_int;
}
pub use self::types_h::{__int8_t, __uint8_t, __int16_t, __int32_t};
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t};
pub use self::stdint_uintn_h::uint8_t;
pub use self::opus_types_h::{opus_int8, opus_uint8, opus_int16, opus_int32};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, C2RustUnnamed, silk_resampler_state_struct,
};
pub use self::structs_h::{
    silk_NLSF_CB_struct, SideInfoIndices, silk_PLC_struct, silk_CNG_struct,
    silk_decoder_state,
};
use self::arch_h::celt_fatal;
use self::string_h::memset;
use self::SigProc_FIX_h::silk_resampler_init;
use self::tables_h::{
    silk_pitch_contour_iCDF, silk_pitch_contour_NB_iCDF, silk_pitch_contour_10_ms_iCDF,
    silk_pitch_contour_10_ms_NB_iCDF, silk_uniform4_iCDF, silk_uniform6_iCDF,
    silk_uniform8_iCDF, silk_NLSF_CB_WB, silk_NLSF_CB_NB_MB,
};
pub use self::define_h::{
    TYPE_NO_VOICE_ACTIVITY, MAX_LPC_ORDER, MIN_LPC_ORDER, MAX_NB_SUBFR,
};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_decoder_set_fs(
    mut psDec: *mut silk_decoder_state,
    mut fs_kHz: libc::c_int,
    mut fs_API_Hz: opus_int32,
) -> libc::c_int {
    let mut frame_length: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if !(fs_kHz == 8 as libc::c_int || fs_kHz == 12 as libc::c_int
        || fs_kHz == 16 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: fs_kHz == 8 || fs_kHz == 12 || fs_kHz == 16\0"
                as *const u8 as *const libc::c_char,
            b"silk/decoder_set_fs.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
        );
    }
    if !((*psDec).nb_subfr == 4 as libc::c_int
        || (*psDec).nb_subfr == 4 as libc::c_int / 2 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: psDec->nb_subfr == MAX_NB_SUBFR || psDec->nb_subfr == MAX_NB_SUBFR/2\0"
                as *const u8 as *const libc::c_char,
            b"silk/decoder_set_fs.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
        );
    }
    (*psDec)
        .subfr_length = 5 as libc::c_int as opus_int16 as opus_int32
        * fs_kHz as opus_int16 as opus_int32;
    frame_length = (*psDec).nb_subfr as opus_int16 as opus_int32
        * (*psDec).subfr_length as opus_int16 as opus_int32;
    if (*psDec).fs_kHz != fs_kHz || (*psDec).fs_API_hz != fs_API_Hz {
        ret
            += silk_resampler_init(
                &mut (*psDec).resampler_state,
                fs_kHz as opus_int16 as opus_int32
                    * 1000 as libc::c_int as opus_int16 as opus_int32,
                fs_API_Hz,
                0 as libc::c_int,
            );
        (*psDec).fs_API_hz = fs_API_Hz;
    }
    if (*psDec).fs_kHz != fs_kHz || frame_length != (*psDec).frame_length {
        if fs_kHz == 8 as libc::c_int {
            if (*psDec).nb_subfr == MAX_NB_SUBFR {
                (*psDec).pitch_contour_iCDF = silk_pitch_contour_NB_iCDF.as_ptr();
            } else {
                (*psDec).pitch_contour_iCDF = silk_pitch_contour_10_ms_NB_iCDF.as_ptr();
            }
        } else if (*psDec).nb_subfr == MAX_NB_SUBFR {
            (*psDec).pitch_contour_iCDF = silk_pitch_contour_iCDF.as_ptr();
        } else {
            (*psDec).pitch_contour_iCDF = silk_pitch_contour_10_ms_iCDF.as_ptr();
        }
        if (*psDec).fs_kHz != fs_kHz {
            (*psDec)
                .ltp_mem_length = 20 as libc::c_int as opus_int16 as opus_int32
                * fs_kHz as opus_int16 as opus_int32;
            if fs_kHz == 8 as libc::c_int || fs_kHz == 12 as libc::c_int {
                (*psDec).LPC_order = MIN_LPC_ORDER;
                (*psDec).psNLSF_CB = &silk_NLSF_CB_NB_MB;
            } else {
                (*psDec).LPC_order = MAX_LPC_ORDER;
                (*psDec).psNLSF_CB = &silk_NLSF_CB_WB;
            }
            if fs_kHz == 16 as libc::c_int {
                (*psDec).pitch_lag_low_bits_iCDF = silk_uniform8_iCDF.as_ptr();
            } else if fs_kHz == 12 as libc::c_int {
                (*psDec).pitch_lag_low_bits_iCDF = silk_uniform6_iCDF.as_ptr();
            } else if fs_kHz == 8 as libc::c_int {
                (*psDec).pitch_lag_low_bits_iCDF = silk_uniform4_iCDF.as_ptr();
            } else if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"silk/decoder_set_fs.c\0" as *const u8 as *const libc::c_char,
                    89 as libc::c_int,
                );
            }
            (*psDec).first_frame_after_reset = 1 as libc::c_int;
            (*psDec).lagPrev = 100 as libc::c_int;
            (*psDec).LastGainIndex = 10 as libc::c_int as opus_int8;
            (*psDec).prevSignalType = TYPE_NO_VOICE_ACTIVITY;
            memset(
                ((*psDec).outBuf).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[opus_int16; 480]>() as libc::c_ulong,
            );
            memset(
                ((*psDec).sLPC_Q14_buf).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[opus_int32; 16]>() as libc::c_ulong,
            );
        }
        (*psDec).fs_kHz = fs_kHz;
        (*psDec).frame_length = frame_length;
    }
    if !((*psDec).frame_length > 0 as libc::c_int
        && (*psDec).frame_length
            <= 5 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: psDec->frame_length > 0 && psDec->frame_length <= MAX_FRAME_LENGTH\0"
                as *const u8 as *const libc::c_char,
            b"silk/decoder_set_fs.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
        );
    }
    return ret;
}
