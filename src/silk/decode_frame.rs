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
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/control.h:32"]
pub mod control_h {
    #[c2rust::src_loc = "39:9"]
    pub const FLAG_DECODE_NORMAL: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "41:9"]
    pub const FLAG_DECODE_LBRR: libc::c_int = 2 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use super::entcode_h::ec_dec;
    use super::structs_h::{silk_decoder_control, silk_decoder_state};
    extern "C" {
        #[c2rust::src_loc = "417:1"]
        pub fn silk_decode_indices(
            psDec: *mut silk_decoder_state,
            psRangeDec: *mut ec_dec,
            FrameIndex: libc::c_int,
            decode_LBRR: libc::c_int,
            condCoding: libc::c_int,
        );
        #[c2rust::src_loc = "442:1"]
        pub fn silk_decode_pulses(
            psRangeDec: *mut ec_dec,
            pulses: *mut i16,
            signalType: libc::c_int,
            quantOffsetType: libc::c_int,
            frame_length: libc::c_int,
        );
        #[c2rust::src_loc = "426:1"]
        pub fn silk_decode_parameters(
            psDec: *mut silk_decoder_state,
            psDecCtrl: *mut silk_decoder_control,
            condCoding: libc::c_int,
        );
        #[c2rust::src_loc = "433:1"]
        pub fn silk_decode_core(
            psDec: *mut silk_decoder_state,
            psDecCtrl: *mut silk_decoder_control,
            xq: *mut i16,
            pulses: *const i16,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "460:1"]
        pub fn silk_CNG(
            psDec: *mut silk_decoder_state,
            psDecCtrl: *mut silk_decoder_control,
            frame: *mut i16,
            length: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/PLC.h:32"]
pub mod PLC_h {
    use super::structs_h::{silk_decoder_control, silk_decoder_state};
    extern "C" {
        #[c2rust::src_loc = "47:1"]
        pub fn silk_PLC(
            psDec: *mut silk_decoder_state,
            psDecCtrl: *mut silk_decoder_control,
            frame: *mut i16,
            lost: libc::c_int,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "55:1"]
        pub fn silk_PLC_glue_frames(
            psDec: *mut silk_decoder_state,
            frame: *mut i16,
            length: libc::c_int,
        );
    }
}
use self::arch_h::celt_fatal;
pub use self::control_h::{FLAG_DECODE_LBRR, FLAG_DECODE_NORMAL};
pub use self::entcode_h::{ec_ctx, ec_dec, ec_window};
use self::main_h::{
    silk_CNG, silk_decode_core, silk_decode_indices, silk_decode_parameters, silk_decode_pulses,
};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};

pub use self::structs_h::{
    silk_CNG_struct, silk_NLSF_CB_struct, silk_PLC_struct, silk_decoder_control,
    silk_decoder_state, SideInfoIndices,
};

use self::PLC_h::{silk_PLC, silk_PLC_glue_frames};
use crate::externs::{memcpy, memmove};
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_decode_frame(
    mut psDec: *mut silk_decoder_state,
    psRangeDec: *mut ec_dec,
    pOut: *mut i16,
    pN: *mut i32,
    lostFlag: libc::c_int,
    condCoding: libc::c_int,
    arch: libc::c_int,
) -> libc::c_int {
    let mut L: libc::c_int = 0;
    let mut mv_len: libc::c_int = 0;
    let ret: libc::c_int = 0 as libc::c_int;
    L = (*psDec).frame_length;
    let mut psDecCtrl: [silk_decoder_control; 1] = [silk_decoder_control {
        pitchL: [0; 4],
        Gains_Q16: [0; 4],
        PredCoef_Q12: [[0; 16]; 2],
        LTPCoef_Q14: [0; 20],
        LTP_scale_Q14: 0,
    }; 1];
    (*psDecCtrl.as_mut_ptr()).LTP_scale_Q14 = 0 as libc::c_int;
    if !(L > 0 as libc::c_int && L <= 5 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) {
        celt_fatal(
            b"assertion failed: L > 0 && L <= MAX_FRAME_LENGTH\0" as *const u8
                as *const libc::c_char,
            b"silk/decode_frame.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
        );
    }
    if lostFlag == FLAG_DECODE_NORMAL
        || lostFlag == FLAG_DECODE_LBRR
            && (*psDec).LBRR_flags[(*psDec).nFramesDecoded as usize] == 1 as libc::c_int
    {
        let vla = (L + 16 as libc::c_int - 1 as libc::c_int
            & !(16 as libc::c_int - 1 as libc::c_int)) as usize;
        let mut pulses: Vec<i16> = ::std::vec::from_elem(0, vla);
        silk_decode_indices(
            psDec,
            psRangeDec,
            (*psDec).nFramesDecoded,
            lostFlag,
            condCoding,
        );
        silk_decode_pulses(
            psRangeDec,
            pulses.as_mut_ptr(),
            (*psDec).indices.signalType as libc::c_int,
            (*psDec).indices.quantOffsetType as libc::c_int,
            (*psDec).frame_length,
        );
        silk_decode_parameters(psDec, psDecCtrl.as_mut_ptr(), condCoding);
        silk_decode_core(
            psDec,
            psDecCtrl.as_mut_ptr(),
            pOut,
            pulses.as_mut_ptr() as *const i16,
            arch,
        );
        silk_PLC(psDec, psDecCtrl.as_mut_ptr(), pOut, 0 as libc::c_int, arch);
        (*psDec).lossCnt = 0 as libc::c_int;
        (*psDec).prevSignalType = (*psDec).indices.signalType as libc::c_int;
        if !((*psDec).prevSignalType >= 0 as libc::c_int
            && (*psDec).prevSignalType <= 2 as libc::c_int)
        {
            celt_fatal(
                b"assertion failed: psDec->prevSignalType >= 0 && psDec->prevSignalType <= 2\0"
                    as *const u8 as *const libc::c_char,
                b"silk/decode_frame.c\0" as *const u8 as *const libc::c_char,
                94 as libc::c_int,
            );
        }
        (*psDec).first_frame_after_reset = 0 as libc::c_int;
    } else {
        (*psDec).indices.signalType = (*psDec).prevSignalType as i8;
        silk_PLC(psDec, psDecCtrl.as_mut_ptr(), pOut, 1 as libc::c_int, arch);
    }
    if !((*psDec).ltp_mem_length >= (*psDec).frame_length) {
        celt_fatal(
            b"assertion failed: psDec->ltp_mem_length >= psDec->frame_length\0" as *const u8
                as *const libc::c_char,
            b"silk/decode_frame.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
        );
    }
    mv_len = (*psDec).ltp_mem_length - (*psDec).frame_length;
    memmove(
        ((*psDec).outBuf).as_mut_ptr() as *mut libc::c_void,
        &mut *((*psDec).outBuf)
            .as_mut_ptr()
            .offset((*psDec).frame_length as isize) as *mut i16 as *const libc::c_void,
        (mv_len as libc::c_ulong).wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    memcpy(
        &mut *((*psDec).outBuf).as_mut_ptr().offset(mv_len as isize) as *mut i16
            as *mut libc::c_void,
        pOut as *const libc::c_void,
        ((*psDec).frame_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    silk_CNG(psDec, psDecCtrl.as_mut_ptr(), pOut, L);
    silk_PLC_glue_frames(psDec, pOut, L);
    (*psDec).lagPrev =
        (*psDecCtrl.as_mut_ptr()).pitchL[((*psDec).nb_subfr - 1 as libc::c_int) as usize];
    *pN = L;
    return ret;
}
