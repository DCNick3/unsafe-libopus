use crate::celt::celt::celt_fatal;
use crate::celt::entdec::ec_dec;
use crate::externs::{memcpy, memmove};
use crate::silk::dec_API::{FLAG_DECODE_LBRR, FLAG_DECODE_NORMAL};
use crate::silk::decode_core::silk_decode_core;
use crate::silk::decode_indices::silk_decode_indices;
use crate::silk::decode_parameters::silk_decode_parameters;
use crate::silk::decode_pulses::silk_decode_pulses;
use crate::silk::structs::{silk_decoder_control, silk_decoder_state};
use crate::silk::CNG::silk_CNG;
use crate::silk::PLC::{silk_PLC, silk_PLC_glue_frames};
use ::libc;

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
