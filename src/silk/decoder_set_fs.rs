use crate::celt::celt::celt_fatal;
use crate::externs::memset;
use crate::silk::define::{MAX_LPC_ORDER, MAX_NB_SUBFR, MIN_LPC_ORDER, TYPE_NO_VOICE_ACTIVITY};
use crate::silk::resampler::silk_resampler_init;
use crate::silk::structs::silk_decoder_state;
use crate::silk::tables_NLSF_CB_NB_MB::silk_NLSF_CB_NB_MB;
use crate::silk::tables_NLSF_CB_WB::silk_NLSF_CB_WB;
use crate::silk::tables_other::{silk_uniform4_iCDF, silk_uniform6_iCDF, silk_uniform8_iCDF};
use crate::silk::tables_pitch_lag::{
    silk_pitch_contour_10_ms_NB_iCDF, silk_pitch_contour_10_ms_iCDF, silk_pitch_contour_NB_iCDF,
    silk_pitch_contour_iCDF,
};

#[c2rust::src_loc = "35:1"]
pub unsafe fn silk_decoder_set_fs(
    mut psDec: *mut silk_decoder_state,
    fs_kHz: libc::c_int,
    fs_API_Hz: i32,
) -> libc::c_int {
    let mut frame_length: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if !(fs_kHz == 8 as libc::c_int || fs_kHz == 12 as libc::c_int || fs_kHz == 16 as libc::c_int) {
        celt_fatal(
            b"assertion failed: fs_kHz == 8 || fs_kHz == 12 || fs_kHz == 16\0" as *const u8
                as *const libc::c_char,
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
    (*psDec).subfr_length = 5 as libc::c_int as i16 as i32 * fs_kHz as i16 as i32;
    frame_length = (*psDec).nb_subfr as i16 as i32 * (*psDec).subfr_length as i16 as i32;
    if (*psDec).fs_kHz != fs_kHz || (*psDec).fs_API_hz != fs_API_Hz {
        ret += silk_resampler_init(
            &mut (*psDec).resampler_state,
            fs_kHz as i16 as i32 * 1000 as libc::c_int as i16 as i32,
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
            (*psDec).ltp_mem_length = 20 as libc::c_int as i16 as i32 * fs_kHz as i16 as i32;
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
            (*psDec).LastGainIndex = 10 as libc::c_int as i8;
            (*psDec).prevSignalType = TYPE_NO_VOICE_ACTIVITY;
            memset(
                ((*psDec).outBuf).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[i16; 480]>() as libc::c_ulong,
            );
            memset(
                ((*psDec).sLPC_Q14_buf).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[i32; 16]>() as libc::c_ulong,
            );
        }
        (*psDec).fs_kHz = fs_kHz;
        (*psDec).frame_length = frame_length;
    }
    if !((*psDec).frame_length > 0 as libc::c_int
        && (*psDec).frame_length <= 5 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int)
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
