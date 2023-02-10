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
    use super::resampler_structs_h::silk_resampler_state_struct;
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "75:9"]
    pub const CODE_INDEPENDENTLY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "77:9"]
    pub const CODE_CONDITIONALLY: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "90:9"]
    pub const MAX_NB_SUBFR: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "208:9"]
    pub const NLSF_QUANT_MAX_AMPLITUDE: libc::c_int = 4 as libc::c_int;
}
pub use self::define_h::{
    CODE_CONDITIONALLY, CODE_INDEPENDENTLY, MAX_NB_SUBFR, NLSF_QUANT_MAX_AMPLITUDE, TYPE_VOICED,
};
use self::main_h::silk_NLSF_unpack;
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
use crate::celt::celt::celt_fatal;
use crate::celt::entdec::{ec_dec, ec_dec_icdf};

pub use self::structs_h::{
    silk_CNG_struct, silk_NLSF_CB_struct, silk_PLC_struct, silk_decoder_state, SideInfoIndices,
};
use self::tables_h::{
    silk_LTP_gain_iCDF_ptrs, silk_LTP_per_index_iCDF, silk_LTPscale_iCDF, silk_NLSF_EXT_iCDF,
    silk_NLSF_interpolation_factor_iCDF, silk_delta_gain_iCDF, silk_gain_iCDF,
    silk_pitch_delta_iCDF, silk_pitch_lag_iCDF, silk_type_offset_VAD_iCDF,
    silk_type_offset_no_VAD_iCDF, silk_uniform4_iCDF, silk_uniform8_iCDF,
};

#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_decode_indices(
    mut psDec: *mut silk_decoder_state,
    psRangeDec: *mut ec_dec,
    FrameIndex: libc::c_int,
    decode_LBRR: libc::c_int,
    condCoding: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut Ix: libc::c_int = 0;
    let mut decode_absolute_lagIndex: libc::c_int = 0;
    let mut delta_lagIndex: libc::c_int = 0;
    let mut ec_ix: [i16; 16] = [0; 16];
    let mut pred_Q8: [u8; 16] = [0; 16];
    if decode_LBRR != 0 || (*psDec).VAD_flags[FrameIndex as usize] != 0 {
        Ix = ec_dec_icdf(
            psRangeDec,
            silk_type_offset_VAD_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        ) + 2 as libc::c_int;
    } else {
        Ix = ec_dec_icdf(
            psRangeDec,
            silk_type_offset_no_VAD_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
    }
    (*psDec).indices.signalType = (Ix >> 1 as libc::c_int) as i8;
    (*psDec).indices.quantOffsetType = (Ix & 1 as libc::c_int) as i8;
    if condCoding == CODE_CONDITIONALLY {
        (*psDec).indices.GainsIndices[0 as libc::c_int as usize] = ec_dec_icdf(
            psRangeDec,
            silk_delta_gain_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        ) as i8;
    } else {
        (*psDec).indices.GainsIndices[0 as libc::c_int as usize] = ((ec_dec_icdf(
            psRangeDec,
            (silk_gain_iCDF[(*psDec).indices.signalType as usize]).as_ptr(),
            8 as libc::c_int as libc::c_uint,
        ) as u32)
            << 3 as libc::c_int)
            as i32 as i8;
        (*psDec).indices.GainsIndices[0 as libc::c_int as usize] =
            ((*psDec).indices.GainsIndices[0 as libc::c_int as usize] as libc::c_int
                + ec_dec_icdf(
                    psRangeDec,
                    silk_uniform8_iCDF.as_ptr(),
                    8 as libc::c_int as libc::c_uint,
                ) as i8 as libc::c_int) as i8;
    }
    i = 1 as libc::c_int;
    while i < (*psDec).nb_subfr {
        (*psDec).indices.GainsIndices[i as usize] = ec_dec_icdf(
            psRangeDec,
            silk_delta_gain_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        ) as i8;
        i += 1;
    }
    (*psDec).indices.NLSFIndices[0 as libc::c_int as usize] = ec_dec_icdf(
        psRangeDec,
        &*((*(*psDec).psNLSF_CB).CB1_iCDF).offset(
            (((*psDec).indices.signalType as libc::c_int >> 1 as libc::c_int)
                * (*(*psDec).psNLSF_CB).nVectors as libc::c_int) as isize,
        ),
        8 as libc::c_int as libc::c_uint,
    ) as i8;
    silk_NLSF_unpack(
        ec_ix.as_mut_ptr(),
        pred_Q8.as_mut_ptr(),
        (*psDec).psNLSF_CB,
        (*psDec).indices.NLSFIndices[0 as libc::c_int as usize] as libc::c_int,
    );
    if !((*(*psDec).psNLSF_CB).order as libc::c_int == (*psDec).LPC_order) {
        celt_fatal(
            b"assertion failed: psDec->psNLSF_CB->order == psDec->LPC_order\0" as *const u8
                as *const libc::c_char,
            b"silk/decode_indices.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < (*(*psDec).psNLSF_CB).order as libc::c_int {
        Ix = ec_dec_icdf(
            psRangeDec,
            &*((*(*psDec).psNLSF_CB).ec_iCDF)
                .offset(*ec_ix.as_mut_ptr().offset(i as isize) as isize),
            8 as libc::c_int as libc::c_uint,
        );
        if Ix == 0 as libc::c_int {
            Ix -= ec_dec_icdf(
                psRangeDec,
                silk_NLSF_EXT_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        } else if Ix == 2 as libc::c_int * NLSF_QUANT_MAX_AMPLITUDE {
            Ix += ec_dec_icdf(
                psRangeDec,
                silk_NLSF_EXT_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        }
        (*psDec).indices.NLSFIndices[(i + 1 as libc::c_int) as usize] =
            (Ix - NLSF_QUANT_MAX_AMPLITUDE) as i8;
        i += 1;
    }
    if (*psDec).nb_subfr == MAX_NB_SUBFR {
        (*psDec).indices.NLSFInterpCoef_Q2 = ec_dec_icdf(
            psRangeDec,
            silk_NLSF_interpolation_factor_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        ) as i8;
    } else {
        (*psDec).indices.NLSFInterpCoef_Q2 = 4 as libc::c_int as i8;
    }
    if (*psDec).indices.signalType as libc::c_int == TYPE_VOICED {
        decode_absolute_lagIndex = 1 as libc::c_int;
        if condCoding == CODE_CONDITIONALLY && (*psDec).ec_prevSignalType == TYPE_VOICED {
            delta_lagIndex = ec_dec_icdf(
                psRangeDec,
                silk_pitch_delta_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            ) as i16 as libc::c_int;
            if delta_lagIndex > 0 as libc::c_int {
                delta_lagIndex = delta_lagIndex - 9 as libc::c_int;
                (*psDec).indices.lagIndex =
                    ((*psDec).ec_prevLagIndex as libc::c_int + delta_lagIndex) as i16;
                decode_absolute_lagIndex = 0 as libc::c_int;
            }
        }
        if decode_absolute_lagIndex != 0 {
            (*psDec).indices.lagIndex = (ec_dec_icdf(
                psRangeDec,
                silk_pitch_lag_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            ) as i16 as libc::c_int
                * ((*psDec).fs_kHz >> 1 as libc::c_int))
                as i16;
            (*psDec).indices.lagIndex = ((*psDec).indices.lagIndex as libc::c_int
                + ec_dec_icdf(
                    psRangeDec,
                    (*psDec).pitch_lag_low_bits_iCDF,
                    8 as libc::c_int as libc::c_uint,
                ) as i16 as libc::c_int) as i16;
        }
        (*psDec).ec_prevLagIndex = (*psDec).indices.lagIndex;
        (*psDec).indices.contourIndex = ec_dec_icdf(
            psRangeDec,
            (*psDec).pitch_contour_iCDF,
            8 as libc::c_int as libc::c_uint,
        ) as i8;
        (*psDec).indices.PERIndex = ec_dec_icdf(
            psRangeDec,
            silk_LTP_per_index_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        ) as i8;
        k = 0 as libc::c_int;
        while k < (*psDec).nb_subfr {
            (*psDec).indices.LTPIndex[k as usize] = ec_dec_icdf(
                psRangeDec,
                silk_LTP_gain_iCDF_ptrs[(*psDec).indices.PERIndex as usize],
                8 as libc::c_int as libc::c_uint,
            ) as i8;
            k += 1;
        }
        if condCoding == CODE_INDEPENDENTLY {
            (*psDec).indices.LTP_scaleIndex = ec_dec_icdf(
                psRangeDec,
                silk_LTPscale_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            ) as i8;
        } else {
            (*psDec).indices.LTP_scaleIndex = 0 as libc::c_int as i8;
        }
    }
    (*psDec).ec_prevSignalType = (*psDec).indices.signalType as libc::c_int;
    (*psDec).indices.Seed = ec_dec_icdf(
        psRangeDec,
        silk_uniform4_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    ) as i8;
}
