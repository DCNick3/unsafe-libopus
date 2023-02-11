use crate::celt::celt::celt_fatal;
use crate::celt::entdec::{ec_dec, ec_dec_icdf};
use crate::silk::define::{
    CODE_CONDITIONALLY, CODE_INDEPENDENTLY, MAX_NB_SUBFR, NLSF_QUANT_MAX_AMPLITUDE, TYPE_VOICED,
};
use crate::silk::structs::silk_decoder_state;
use crate::silk::tables_LTP::{silk_LTP_gain_iCDF_ptrs, silk_LTP_per_index_iCDF};
use crate::silk::tables_gain::{silk_delta_gain_iCDF, silk_gain_iCDF};
use crate::silk::tables_other::{
    silk_LTPscale_iCDF, silk_NLSF_EXT_iCDF, silk_NLSF_interpolation_factor_iCDF,
    silk_type_offset_VAD_iCDF, silk_type_offset_no_VAD_iCDF, silk_uniform4_iCDF,
    silk_uniform8_iCDF,
};
use crate::silk::tables_pitch_lag::{silk_pitch_delta_iCDF, silk_pitch_lag_iCDF};
use crate::silk::NLSF_unpack::silk_NLSF_unpack;
use ::libc;

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
