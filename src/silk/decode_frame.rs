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
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
    use super::opus_types_h::opus_uint32;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "314:9"]
    pub struct silk_decoder_control {
        pub pitchL: [libc::c_int; 4],
        pub Gains_Q16: [opus_int32; 4],
        pub PredCoef_Q12: [[opus_int16; 16]; 2],
        pub LTPCoef_Q14: [opus_int16; 20],
        pub LTP_scale_Q14: libc::c_int,
    }
    use super::opus_types_h::{opus_int16, opus_int32, opus_int8, opus_uint8};
    use super::resampler_structs_h::silk_resampler_state_struct;
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/PLC.h:32"]
pub mod PLC_h {
    use super::opus_types_h::opus_int16;
    use super::structs_h::{silk_decoder_control, silk_decoder_state};
    extern "C" {
        #[c2rust::src_loc = "47:1"]
        pub fn silk_PLC(
            psDec: *mut silk_decoder_state,
            psDecCtrl: *mut silk_decoder_control,
            frame: *mut opus_int16,
            lost: libc::c_int,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "55:1"]
        pub fn silk_PLC_glue_frames(
            psDec: *mut silk_decoder_state,
            frame: *mut opus_int16,
            length: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use super::entcode_h::ec_dec;
    use super::opus_types_h::opus_int16;
    use super::structs_h::{silk_decoder_control, silk_decoder_state};
    extern "C" {
        #[c2rust::src_loc = "460:1"]
        pub fn silk_CNG(
            psDec: *mut silk_decoder_state,
            psDecCtrl: *mut silk_decoder_control,
            frame: *mut opus_int16,
            length: libc::c_int,
        );
        #[c2rust::src_loc = "433:1"]
        pub fn silk_decode_core(
            psDec: *mut silk_decoder_state,
            psDecCtrl: *mut silk_decoder_control,
            xq: *mut opus_int16,
            pulses: *const opus_int16,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "426:1"]
        pub fn silk_decode_parameters(
            psDec: *mut silk_decoder_state,
            psDecCtrl: *mut silk_decoder_control,
            condCoding: libc::c_int,
        );
        #[c2rust::src_loc = "442:1"]
        pub fn silk_decode_pulses(
            psRangeDec: *mut ec_dec,
            pulses: *mut opus_int16,
            signalType: libc::c_int,
            quantOffsetType: libc::c_int,
            frame_length: libc::c_int,
        );
        #[c2rust::src_loc = "417:1"]
        pub fn silk_decode_indices(
            psDec: *mut silk_decoder_state,
            psRangeDec: *mut ec_dec,
            FrameIndex: libc::c_int,
            decode_LBRR: libc::c_int,
            condCoding: libc::c_int,
        );
    }
}
pub use self::entcode_h::{ec_ctx, ec_dec, ec_window};
use self::main_h::{
    silk_CNG, silk_decode_core, silk_decode_indices, silk_decode_parameters, silk_decode_pulses,
};
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int8, opus_uint32, opus_uint8};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::string_h::{memcpy, memmove};
pub use self::structs_h::{
    silk_CNG_struct, silk_NLSF_CB_struct, silk_PLC_struct, silk_decoder_control,
    silk_decoder_state, SideInfoIndices,
};
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint32_t, __uint8_t};
use self::PLC_h::{silk_PLC, silk_PLC_glue_frames};
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_decode_frame(
    mut psDec: *mut silk_decoder_state,
    mut psRangeDec: *mut ec_dec,
    mut pOut: *mut opus_int16,
    mut pN: *mut opus_int32,
    mut lostFlag: libc::c_int,
    mut condCoding: libc::c_int,
    mut arch: libc::c_int,
) -> libc::c_int {
    let mut L: libc::c_int = 0;
    let mut mv_len: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    L = (*psDec).frame_length;
    let mut psDecCtrl: [silk_decoder_control; 1] = [silk_decoder_control {
        pitchL: [0; 4],
        Gains_Q16: [0; 4],
        PredCoef_Q12: [[0; 16]; 2],
        LTPCoef_Q14: [0; 20],
        LTP_scale_Q14: 0,
    }; 1];
    (*psDecCtrl.as_mut_ptr()).LTP_scale_Q14 = 0 as libc::c_int;
    if lostFlag == 0 as libc::c_int
        || lostFlag == 2 as libc::c_int
            && (*psDec).LBRR_flags[(*psDec).nFramesDecoded as usize] == 1 as libc::c_int
    {
        let vla = (L + 16 as libc::c_int - 1 as libc::c_int
            & !(16 as libc::c_int - 1 as libc::c_int)) as usize;
        let mut pulses: Vec<opus_int16> = ::std::vec::from_elem(0, vla);
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
            pulses.as_mut_ptr() as *const opus_int16,
            arch,
        );
        silk_PLC(psDec, psDecCtrl.as_mut_ptr(), pOut, 0 as libc::c_int, arch);
        (*psDec).lossCnt = 0 as libc::c_int;
        (*psDec).prevSignalType = (*psDec).indices.signalType as libc::c_int;
        (*psDec).first_frame_after_reset = 0 as libc::c_int;
    } else {
        (*psDec).indices.signalType = (*psDec).prevSignalType as opus_int8;
        silk_PLC(psDec, psDecCtrl.as_mut_ptr(), pOut, 1 as libc::c_int, arch);
    }
    mv_len = (*psDec).ltp_mem_length - (*psDec).frame_length;
    memmove(
        ((*psDec).outBuf).as_mut_ptr() as *mut libc::c_void,
        &mut *((*psDec).outBuf)
            .as_mut_ptr()
            .offset((*psDec).frame_length as isize) as *mut opus_int16
            as *const libc::c_void,
        (mv_len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
    memcpy(
        &mut *((*psDec).outBuf).as_mut_ptr().offset(mv_len as isize) as *mut opus_int16
            as *mut libc::c_void,
        pOut as *const libc::c_void,
        ((*psDec).frame_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
    silk_CNG(psDec, psDecCtrl.as_mut_ptr(), pOut, L);
    silk_PLC_glue_frames(psDec, pOut, L);
    (*psDec).lagPrev =
        (*psDecCtrl.as_mut_ptr()).pitchL[((*psDec).nb_subfr - 1 as libc::c_int) as usize];
    *pN = L;
    return ret;
}
