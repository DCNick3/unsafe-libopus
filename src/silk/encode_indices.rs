use crate::celt::celt::celt_fatal;
use crate::celt::entenc::{ec_enc, ec_enc_icdf};
use crate::silk::define::{
    CODE_CONDITIONALLY, CODE_INDEPENDENTLY, MAX_NB_SUBFR, NLSF_QUANT_MAX_AMPLITUDE, TYPE_VOICED,
};
use crate::silk::structs::{silk_encoder_state, SideInfoIndices};
use crate::silk::tables_LTP::{silk_LTP_gain_iCDF_ptrs, silk_LTP_per_index_iCDF};
use crate::silk::tables_gain::{silk_delta_gain_iCDF, silk_gain_iCDF};
use crate::silk::tables_other::{
    silk_LTPscale_iCDF, silk_NLSF_EXT_iCDF, silk_NLSF_interpolation_factor_iCDF,
    silk_type_offset_VAD_iCDF, silk_type_offset_no_VAD_iCDF, silk_uniform4_iCDF,
    silk_uniform8_iCDF,
};
use crate::silk::tables_pitch_lag::{silk_pitch_delta_iCDF, silk_pitch_lag_iCDF};
use crate::silk::NLSF_unpack::silk_NLSF_unpack;

pub unsafe fn silk_encode_indices(
    mut psEncC: *mut silk_encoder_state,
    psRangeEnc: *mut ec_enc,
    FrameIndex: i32,
    encode_LBRR: i32,
    condCoding: i32,
) {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut typeOffset: i32 = 0;
    let mut encode_absolute_lagIndex: i32 = 0;
    let mut delta_lagIndex: i32 = 0;
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
    typeOffset = 2 as i32 * (*psIndices).signalType as i32 + (*psIndices).quantOffsetType as i32;
    if !(typeOffset >= 0 as i32 && typeOffset < 6 as i32) {
        celt_fatal(
            b"assertion failed: typeOffset >= 0 && typeOffset < 6\0" as *const u8 as *const i8,
            b"silk/encode_indices.c\0" as *const u8 as *const i8,
            59 as i32,
        );
    }
    if !(encode_LBRR == 0 as i32 || typeOffset >= 2 as i32) {
        celt_fatal(
            b"assertion failed: encode_LBRR == 0 || typeOffset >= 2\0" as *const u8 as *const i8,
            b"silk/encode_indices.c\0" as *const u8 as *const i8,
            60 as i32,
        );
    }
    if encode_LBRR != 0 || typeOffset >= 2 as i32 {
        ec_enc_icdf(
            psRangeEnc,
            typeOffset - 2 as i32,
            silk_type_offset_VAD_iCDF.as_ptr(),
            8 as i32 as u32,
        );
    } else {
        ec_enc_icdf(
            psRangeEnc,
            typeOffset,
            silk_type_offset_no_VAD_iCDF.as_ptr(),
            8 as i32 as u32,
        );
    }
    if condCoding == CODE_CONDITIONALLY {
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).GainsIndices[0 as i32 as usize] as i32,
            silk_delta_gain_iCDF.as_ptr(),
            8 as i32 as u32,
        );
    } else {
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).GainsIndices[0 as i32 as usize] as i32 >> 3 as i32,
            (silk_gain_iCDF[(*psIndices).signalType as usize]).as_ptr(),
            8 as i32 as u32,
        );
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).GainsIndices[0 as i32 as usize] as i32 & 7 as i32,
            silk_uniform8_iCDF.as_ptr(),
            8 as i32 as u32,
        );
    }
    i = 1 as i32;
    while i < (*psEncC).nb_subfr {
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).GainsIndices[i as usize] as i32,
            silk_delta_gain_iCDF.as_ptr(),
            8 as i32 as u32,
        );
        i += 1;
    }
    ec_enc_icdf(
        psRangeEnc,
        (*psIndices).NLSFIndices[0 as i32 as usize] as i32,
        &*((*(*psEncC).psNLSF_CB).CB1_iCDF).offset(
            (((*psIndices).signalType as i32 >> 1 as i32) * (*(*psEncC).psNLSF_CB).nVectors as i32)
                as isize,
        ),
        8 as i32 as u32,
    );
    silk_NLSF_unpack(
        ec_ix.as_mut_ptr(),
        pred_Q8.as_mut_ptr(),
        (*psEncC).psNLSF_CB,
        (*psIndices).NLSFIndices[0 as i32 as usize] as i32,
    );
    if !((*(*psEncC).psNLSF_CB).order as i32 == (*psEncC).predictLPCOrder) {
        celt_fatal(
            b"assertion failed: psEncC->psNLSF_CB->order == psEncC->predictLPCOrder\0" as *const u8
                as *const i8,
            b"silk/encode_indices.c\0" as *const u8 as *const i8,
            93 as i32,
        );
    }
    i = 0 as i32;
    while i < (*(*psEncC).psNLSF_CB).order as i32 {
        if (*psIndices).NLSFIndices[(i + 1 as i32) as usize] as i32 >= NLSF_QUANT_MAX_AMPLITUDE {
            ec_enc_icdf(
                psRangeEnc,
                2 as i32 * NLSF_QUANT_MAX_AMPLITUDE,
                &*((*(*psEncC).psNLSF_CB).ec_iCDF)
                    .offset(*ec_ix.as_mut_ptr().offset(i as isize) as isize),
                8 as i32 as u32,
            );
            ec_enc_icdf(
                psRangeEnc,
                (*psIndices).NLSFIndices[(i + 1 as i32) as usize] as i32 - NLSF_QUANT_MAX_AMPLITUDE,
                silk_NLSF_EXT_iCDF.as_ptr(),
                8 as i32 as u32,
            );
        } else if (*psIndices).NLSFIndices[(i + 1 as i32) as usize] as i32
            <= -NLSF_QUANT_MAX_AMPLITUDE
        {
            ec_enc_icdf(
                psRangeEnc,
                0 as i32,
                &*((*(*psEncC).psNLSF_CB).ec_iCDF)
                    .offset(*ec_ix.as_mut_ptr().offset(i as isize) as isize),
                8 as i32 as u32,
            );
            ec_enc_icdf(
                psRangeEnc,
                -((*psIndices).NLSFIndices[(i + 1 as i32) as usize] as i32)
                    - NLSF_QUANT_MAX_AMPLITUDE,
                silk_NLSF_EXT_iCDF.as_ptr(),
                8 as i32 as u32,
            );
        } else {
            ec_enc_icdf(
                psRangeEnc,
                (*psIndices).NLSFIndices[(i + 1 as i32) as usize] as i32 + NLSF_QUANT_MAX_AMPLITUDE,
                &*((*(*psEncC).psNLSF_CB).ec_iCDF)
                    .offset(*ec_ix.as_mut_ptr().offset(i as isize) as isize),
                8 as i32 as u32,
            );
        }
        i += 1;
    }
    if (*psEncC).nb_subfr == MAX_NB_SUBFR {
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).NLSFInterpCoef_Q2 as i32,
            silk_NLSF_interpolation_factor_iCDF.as_ptr(),
            8 as i32 as u32,
        );
    }
    if (*psIndices).signalType as i32 == TYPE_VOICED {
        encode_absolute_lagIndex = 1 as i32;
        if condCoding == CODE_CONDITIONALLY && (*psEncC).ec_prevSignalType == TYPE_VOICED {
            delta_lagIndex = (*psIndices).lagIndex as i32 - (*psEncC).ec_prevLagIndex as i32;
            if delta_lagIndex < -(8 as i32) || delta_lagIndex > 11 as i32 {
                delta_lagIndex = 0 as i32;
            } else {
                delta_lagIndex = delta_lagIndex + 9 as i32;
                encode_absolute_lagIndex = 0 as i32;
            }
            ec_enc_icdf(
                psRangeEnc,
                delta_lagIndex,
                silk_pitch_delta_iCDF.as_ptr(),
                8 as i32 as u32,
            );
        }
        if encode_absolute_lagIndex != 0 {
            let mut pitch_high_bits: i32 = 0;
            let mut pitch_low_bits: i32 = 0;
            pitch_high_bits = (*psIndices).lagIndex as i32 / ((*psEncC).fs_kHz >> 1 as i32);
            pitch_low_bits = (*psIndices).lagIndex as i32
                - pitch_high_bits as i16 as i32 * ((*psEncC).fs_kHz >> 1 as i32) as i16 as i32;
            ec_enc_icdf(
                psRangeEnc,
                pitch_high_bits,
                silk_pitch_lag_iCDF.as_ptr(),
                8 as i32 as u32,
            );
            ec_enc_icdf(
                psRangeEnc,
                pitch_low_bits,
                (*psEncC).pitch_lag_low_bits_iCDF,
                8 as i32 as u32,
            );
        }
        (*psEncC).ec_prevLagIndex = (*psIndices).lagIndex;
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).contourIndex as i32,
            (*psEncC).pitch_contour_iCDF,
            8 as i32 as u32,
        );
        ec_enc_icdf(
            psRangeEnc,
            (*psIndices).PERIndex as i32,
            silk_LTP_per_index_iCDF.as_ptr(),
            8 as i32 as u32,
        );
        k = 0 as i32;
        while k < (*psEncC).nb_subfr {
            ec_enc_icdf(
                psRangeEnc,
                (*psIndices).LTPIndex[k as usize] as i32,
                silk_LTP_gain_iCDF_ptrs[(*psIndices).PERIndex as usize],
                8 as i32 as u32,
            );
            k += 1;
        }
        if condCoding == CODE_INDEPENDENTLY {
            ec_enc_icdf(
                psRangeEnc,
                (*psIndices).LTP_scaleIndex as i32,
                silk_LTPscale_iCDF.as_ptr(),
                8 as i32 as u32,
            );
        }
    }
    (*psEncC).ec_prevSignalType = (*psIndices).signalType as i32;
    ec_enc_icdf(
        psRangeEnc,
        (*psIndices).Seed as i32,
        silk_uniform4_iCDF.as_ptr(),
        8 as i32 as u32,
    );
}
