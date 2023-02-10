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
    use super::types_h::{__int8_t, __int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
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
    use super::stdint_intn_h::{int8_t, int16_t, int32_t};
    use super::stdint_uintn_h::{uint8_t, uint32_t};
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.h:32"]
pub mod entdec_h {
    use super::entcode_h::ec_dec;
    extern "C" {
        #[c2rust::src_loc = "82:1"]
        pub fn ec_dec_icdf(
            _this: *mut ec_dec,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        ) -> libc::c_int;
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
pub use self::types_h::{__int8_t, __uint8_t, __int16_t, __int32_t, __uint32_t};
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::opus_types_h::{opus_int8, opus_uint8, opus_int16, opus_int32, opus_uint32};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, C2RustUnnamed, silk_resampler_state_struct,
};
pub use self::entcode_h::{ec_window, ec_ctx, ec_dec};
pub use self::structs_h::{
    silk_NLSF_CB_struct, SideInfoIndices, silk_PLC_struct, silk_CNG_struct,
    silk_decoder_state,
};
use self::arch_h::celt_fatal;
use self::entdec_h::ec_dec_icdf;
use self::tables_h::{
    silk_gain_iCDF, silk_delta_gain_iCDF, silk_pitch_lag_iCDF, silk_pitch_delta_iCDF,
    silk_uniform4_iCDF, silk_uniform8_iCDF, silk_NLSF_EXT_iCDF, silk_LTP_per_index_iCDF,
    silk_LTP_gain_iCDF_ptrs, silk_LTPscale_iCDF, silk_type_offset_VAD_iCDF,
    silk_type_offset_no_VAD_iCDF, silk_NLSF_interpolation_factor_iCDF,
};
use self::main_h::silk_NLSF_unpack;
pub use self::define_h::{
    CODE_INDEPENDENTLY, TYPE_VOICED, CODE_CONDITIONALLY, MAX_NB_SUBFR,
    NLSF_QUANT_MAX_AMPLITUDE,
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
    let mut ec_ix: [opus_int16; 16] = [0; 16];
    let mut pred_Q8: [opus_uint8; 16] = [0; 16];
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
    (*psDec).indices.signalType = (Ix >> 1 as libc::c_int) as opus_int8;
    (*psDec).indices.quantOffsetType = (Ix & 1 as libc::c_int) as opus_int8;
    if condCoding == CODE_CONDITIONALLY {
        (*psDec)
            .indices
            .GainsIndices[0 as libc::c_int
            as usize] = ec_dec_icdf(
            psRangeDec,
            silk_delta_gain_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        ) as opus_int8;
    } else {
        (*psDec)
            .indices
            .GainsIndices[0 as libc::c_int
            as usize] = ((ec_dec_icdf(
            psRangeDec,
            (silk_gain_iCDF[(*psDec).indices.signalType as usize]).as_ptr(),
            8 as libc::c_int as libc::c_uint,
        ) as opus_uint32) << 3 as libc::c_int) as opus_int32 as opus_int8;
        (*psDec)
            .indices
            .GainsIndices[0 as libc::c_int
            as usize] = ((*psDec).indices.GainsIndices[0 as libc::c_int as usize]
            as libc::c_int
            + ec_dec_icdf(
                psRangeDec,
                silk_uniform8_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            ) as opus_int8 as libc::c_int) as opus_int8;
    }
    i = 1 as libc::c_int;
    while i < (*psDec).nb_subfr {
        (*psDec)
            .indices
            .GainsIndices[i
            as usize] = ec_dec_icdf(
            psRangeDec,
            silk_delta_gain_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        ) as opus_int8;
        i += 1;
    }
    (*psDec)
        .indices
        .NLSFIndices[0 as libc::c_int
        as usize] = ec_dec_icdf(
        psRangeDec,
        &*((*(*psDec).psNLSF_CB).CB1_iCDF)
            .offset(
                (((*psDec).indices.signalType as libc::c_int >> 1 as libc::c_int)
                    * (*(*psDec).psNLSF_CB).nVectors as libc::c_int) as isize,
            ),
        8 as libc::c_int as libc::c_uint,
    ) as opus_int8;
    silk_NLSF_unpack(
        ec_ix.as_mut_ptr(),
        pred_Q8.as_mut_ptr(),
        (*psDec).psNLSF_CB,
        (*psDec).indices.NLSFIndices[0 as libc::c_int as usize] as libc::c_int,
    );
    if !((*(*psDec).psNLSF_CB).order as libc::c_int == (*psDec).LPC_order) {
        celt_fatal(
            b"assertion failed: psDec->psNLSF_CB->order == psDec->LPC_order\0"
                as *const u8 as *const libc::c_char,
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
            Ix
                -= ec_dec_icdf(
                    psRangeDec,
                    silk_NLSF_EXT_iCDF.as_ptr(),
                    8 as libc::c_int as libc::c_uint,
                );
        } else if Ix == 2 as libc::c_int * NLSF_QUANT_MAX_AMPLITUDE {
            Ix
                += ec_dec_icdf(
                    psRangeDec,
                    silk_NLSF_EXT_iCDF.as_ptr(),
                    8 as libc::c_int as libc::c_uint,
                );
        }
        (*psDec)
            .indices
            .NLSFIndices[(i + 1 as libc::c_int)
            as usize] = (Ix - NLSF_QUANT_MAX_AMPLITUDE) as opus_int8;
        i += 1;
    }
    if (*psDec).nb_subfr == MAX_NB_SUBFR {
        (*psDec)
            .indices
            .NLSFInterpCoef_Q2 = ec_dec_icdf(
            psRangeDec,
            silk_NLSF_interpolation_factor_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        ) as opus_int8;
    } else {
        (*psDec).indices.NLSFInterpCoef_Q2 = 4 as libc::c_int as opus_int8;
    }
    if (*psDec).indices.signalType as libc::c_int == TYPE_VOICED {
        decode_absolute_lagIndex = 1 as libc::c_int;
        if condCoding == CODE_CONDITIONALLY && (*psDec).ec_prevSignalType == TYPE_VOICED
        {
            delta_lagIndex = ec_dec_icdf(
                psRangeDec,
                silk_pitch_delta_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            ) as opus_int16 as libc::c_int;
            if delta_lagIndex > 0 as libc::c_int {
                delta_lagIndex = delta_lagIndex - 9 as libc::c_int;
                (*psDec)
                    .indices
                    .lagIndex = ((*psDec).ec_prevLagIndex as libc::c_int
                    + delta_lagIndex) as opus_int16;
                decode_absolute_lagIndex = 0 as libc::c_int;
            }
        }
        if decode_absolute_lagIndex != 0 {
            (*psDec)
                .indices
                .lagIndex = (ec_dec_icdf(
                psRangeDec,
                silk_pitch_lag_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            ) as opus_int16 as libc::c_int * ((*psDec).fs_kHz >> 1 as libc::c_int))
                as opus_int16;
            (*psDec)
                .indices
                .lagIndex = ((*psDec).indices.lagIndex as libc::c_int
                + ec_dec_icdf(
                    psRangeDec,
                    (*psDec).pitch_lag_low_bits_iCDF,
                    8 as libc::c_int as libc::c_uint,
                ) as opus_int16 as libc::c_int) as opus_int16;
        }
        (*psDec).ec_prevLagIndex = (*psDec).indices.lagIndex;
        (*psDec)
            .indices
            .contourIndex = ec_dec_icdf(
            psRangeDec,
            (*psDec).pitch_contour_iCDF,
            8 as libc::c_int as libc::c_uint,
        ) as opus_int8;
        (*psDec)
            .indices
            .PERIndex = ec_dec_icdf(
            psRangeDec,
            silk_LTP_per_index_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        ) as opus_int8;
        k = 0 as libc::c_int;
        while k < (*psDec).nb_subfr {
            (*psDec)
                .indices
                .LTPIndex[k
                as usize] = ec_dec_icdf(
                psRangeDec,
                silk_LTP_gain_iCDF_ptrs[(*psDec).indices.PERIndex as usize],
                8 as libc::c_int as libc::c_uint,
            ) as opus_int8;
            k += 1;
        }
        if condCoding == CODE_INDEPENDENTLY {
            (*psDec)
                .indices
                .LTP_scaleIndex = ec_dec_icdf(
                psRangeDec,
                silk_LTPscale_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            ) as opus_int8;
        } else {
            (*psDec).indices.LTP_scaleIndex = 0 as libc::c_int as opus_int8;
        }
    }
    (*psDec).ec_prevSignalType = (*psDec).indices.signalType as libc::c_int;
    (*psDec)
        .indices
        .Seed = ec_dec_icdf(
        psRangeDec,
        silk_uniform4_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    ) as opus_int8;
}
