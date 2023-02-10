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
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    extern "C" {
        #[c2rust::src_loc = "133:1"]
        pub fn silk_bwexpander(ar: *mut i16, d: libc::c_int, chirp_Q16: i32);
        #[c2rust::src_loc = "254:1"]
        pub fn silk_decode_pitch(
            lagIndex: i16,
            contourIndex: i8,
            pitch_lags: *mut libc::c_int,
            Fs_kHz: libc::c_int,
            nb_subfr: libc::c_int,
        );
        #[c2rust::src_loc = "286:1"]
        pub fn silk_NLSF2A(a_Q12: *mut i16, NLSF: *const i16, d: libc::c_int, arch: libc::c_int);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "79:34"]
        pub static silk_LTP_vq_ptrs_Q7: [*const i8; 3];
        #[c2rust::src_loc = "84:26"]
        pub static silk_LTPScales_table_Q14: [i16; 3];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use super::structs_h::silk_NLSF_CB_struct;
    extern "C" {
        #[c2rust::src_loc = "187:1"]
        pub fn silk_gains_dequant(
            gain_Q16: *mut i32,
            ind: *const i8,
            prev_ind: *mut i8,
            conditional: libc::c_int,
            nb_subfr: libc::c_int,
        );
        #[c2rust::src_loc = "383:1"]
        pub fn silk_NLSF_decode(
            pNLSF_Q15: *mut i16,
            NLSFIndices: *mut i8,
            psNLSF_CB: *const silk_NLSF_CB_struct,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "146:9"]
    pub const LTP_ORDER: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "223:9"]
    pub const BWE_AFTER_LOSS_Q16: libc::c_int = 63570 as libc::c_int;
    #[c2rust::src_loc = "77:9"]
    pub const CODE_CONDITIONALLY: libc::c_int = 2 as libc::c_int;
}
pub use self::define_h::{BWE_AFTER_LOSS_Q16, CODE_CONDITIONALLY, LTP_ORDER, TYPE_VOICED};
use self::main_h::{silk_NLSF_decode, silk_gains_dequant};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::string_h::{memcpy, memset};
pub use self::structs_h::{
    silk_CNG_struct, silk_NLSF_CB_struct, silk_PLC_struct, silk_decoder_control,
    silk_decoder_state, SideInfoIndices,
};
use self::tables_h::{silk_LTPScales_table_Q14, silk_LTP_vq_ptrs_Q7};
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint32_t, __uint8_t};
use self::SigProc_FIX_h::{silk_NLSF2A, silk_bwexpander, silk_decode_pitch};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_decode_parameters(
    mut psDec: *mut silk_decoder_state,
    mut psDecCtrl: *mut silk_decoder_control,
    condCoding: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut Ix: libc::c_int = 0;
    let mut pNLSF_Q15: [i16; 16] = [0; 16];
    let mut pNLSF0_Q15: [i16; 16] = [0; 16];
    let mut cbk_ptr_Q7: *const i8 = 0 as *const i8;
    silk_gains_dequant(
        ((*psDecCtrl).Gains_Q16).as_mut_ptr(),
        ((*psDec).indices.GainsIndices).as_mut_ptr() as *const i8,
        &mut (*psDec).LastGainIndex,
        (condCoding == CODE_CONDITIONALLY) as libc::c_int,
        (*psDec).nb_subfr,
    );
    silk_NLSF_decode(
        pNLSF_Q15.as_mut_ptr(),
        ((*psDec).indices.NLSFIndices).as_mut_ptr(),
        (*psDec).psNLSF_CB,
    );
    silk_NLSF2A(
        ((*psDecCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr(),
        pNLSF_Q15.as_mut_ptr(),
        (*psDec).LPC_order,
        (*psDec).arch,
    );
    if (*psDec).first_frame_after_reset == 1 as libc::c_int {
        (*psDec).indices.NLSFInterpCoef_Q2 = 4 as libc::c_int as i8;
    }
    if ((*psDec).indices.NLSFInterpCoef_Q2 as libc::c_int) < 4 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*psDec).LPC_order {
            pNLSF0_Q15[i as usize] = ((*psDec).prevNLSF_Q15[i as usize] as libc::c_int
                + ((*psDec).indices.NLSFInterpCoef_Q2 as libc::c_int
                    * (pNLSF_Q15[i as usize] as libc::c_int
                        - (*psDec).prevNLSF_Q15[i as usize] as libc::c_int)
                    >> 2 as libc::c_int)) as i16;
            i += 1;
        }
        silk_NLSF2A(
            ((*psDecCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr(),
            pNLSF0_Q15.as_mut_ptr(),
            (*psDec).LPC_order,
            (*psDec).arch,
        );
    } else {
        memcpy(
            ((*psDecCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr()
                as *mut libc::c_void,
            ((*psDecCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr()
                as *const libc::c_void,
            ((*psDec).LPC_order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
        );
    }
    memcpy(
        ((*psDec).prevNLSF_Q15).as_mut_ptr() as *mut libc::c_void,
        pNLSF_Q15.as_mut_ptr() as *const libc::c_void,
        ((*psDec).LPC_order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    if (*psDec).lossCnt != 0 {
        silk_bwexpander(
            ((*psDecCtrl).PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr(),
            (*psDec).LPC_order,
            BWE_AFTER_LOSS_Q16,
        );
        silk_bwexpander(
            ((*psDecCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr(),
            (*psDec).LPC_order,
            BWE_AFTER_LOSS_Q16,
        );
    }
    if (*psDec).indices.signalType as libc::c_int == TYPE_VOICED {
        silk_decode_pitch(
            (*psDec).indices.lagIndex,
            (*psDec).indices.contourIndex,
            ((*psDecCtrl).pitchL).as_mut_ptr(),
            (*psDec).fs_kHz,
            (*psDec).nb_subfr,
        );
        cbk_ptr_Q7 = silk_LTP_vq_ptrs_Q7[(*psDec).indices.PERIndex as usize];
        k = 0 as libc::c_int;
        while k < (*psDec).nb_subfr {
            Ix = (*psDec).indices.LTPIndex[k as usize] as libc::c_int;
            i = 0 as libc::c_int;
            while i < LTP_ORDER {
                (*psDecCtrl).LTPCoef_Q14[(k * LTP_ORDER + i) as usize] =
                    ((*cbk_ptr_Q7.offset((Ix * 5 as libc::c_int + i) as isize) as u32)
                        << 7 as libc::c_int) as i32 as i16;
                i += 1;
            }
            k += 1;
        }
        Ix = (*psDec).indices.LTP_scaleIndex as libc::c_int;
        (*psDecCtrl).LTP_scale_Q14 = silk_LTPScales_table_Q14[Ix as usize] as libc::c_int;
    } else {
        memset(
            ((*psDecCtrl).pitchL).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ((*psDec).nb_subfr as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memset(
            ((*psDecCtrl).LTPCoef_Q14).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ((5 as libc::c_int * (*psDec).nb_subfr) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
        );
        (*psDec).indices.PERIndex = 0 as libc::c_int as i8;
        (*psDecCtrl).LTP_scale_Q14 = 0 as libc::c_int;
    };
}
