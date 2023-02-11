use ::libc;

use crate::celt::celt::celt_fatal;
use crate::externs::{memcpy, memset};
use crate::silk::resampler_private_IIR_FIR::silk_resampler_private_IIR_FIR;
use crate::silk::resampler_private_down_FIR::silk_resampler_private_down_FIR;
use crate::silk::resampler_private_up2_HQ::silk_resampler_private_up2_HQ_wrapper;
use crate::silk::resampler_rom::{
    silk_Resampler_1_2_COEFS, silk_Resampler_1_3_COEFS, silk_Resampler_1_4_COEFS,
    silk_Resampler_1_6_COEFS, silk_Resampler_2_3_COEFS, silk_Resampler_3_4_COEFS,
    RESAMPLER_DOWN_ORDER_FIR0, RESAMPLER_DOWN_ORDER_FIR1, RESAMPLER_DOWN_ORDER_FIR2,
};
use crate::silk::resampler_structs::silk_resampler_state_struct;

#[c2rust::src_loc = "40:9"]
pub const RESAMPLER_MAX_BATCH_SIZE_MS: libc::c_int = 10 as libc::c_int;

#[c2rust::src_loc = "53:24"]
static mut delay_matrix_enc: [[i8; 3]; 5] = [
    [
        6 as libc::c_int as i8,
        0 as libc::c_int as i8,
        3 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        7 as libc::c_int as i8,
        3 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        1 as libc::c_int as i8,
        10 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        2 as libc::c_int as i8,
        6 as libc::c_int as i8,
    ],
    [
        18 as libc::c_int as i8,
        10 as libc::c_int as i8,
        12 as libc::c_int as i8,
    ],
];
#[c2rust::src_loc = "62:24"]
static mut delay_matrix_dec: [[i8; 5]; 3] = [
    [
        4 as libc::c_int as i8,
        0 as libc::c_int as i8,
        2 as libc::c_int as i8,
        0 as libc::c_int as i8,
        0 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        9 as libc::c_int as i8,
        4 as libc::c_int as i8,
        7 as libc::c_int as i8,
        4 as libc::c_int as i8,
    ],
    [
        0 as libc::c_int as i8,
        3 as libc::c_int as i8,
        12 as libc::c_int as i8,
        7 as libc::c_int as i8,
        7 as libc::c_int as i8,
    ],
];
#[c2rust::src_loc = "72:9"]
pub const USE_silk_resampler_copy: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "73:9"]
pub const USE_silk_resampler_private_up2_HQ_wrapper: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "74:9"]
pub const USE_silk_resampler_private_IIR_FIR: libc::c_int = 2 as libc::c_int;
#[c2rust::src_loc = "75:9"]
pub const USE_silk_resampler_private_down_FIR: libc::c_int = 3 as libc::c_int;
#[c2rust::src_loc = "78:1"]
pub unsafe extern "C" fn silk_resampler_init(
    mut S: *mut silk_resampler_state_struct,
    Fs_Hz_in: i32,
    Fs_Hz_out: i32,
    forEnc: libc::c_int,
) -> libc::c_int {
    let mut up2x: libc::c_int = 0;
    memset(
        S as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<silk_resampler_state_struct>() as libc::c_ulong,
    );
    if forEnc != 0 {
        if Fs_Hz_in != 8000 as libc::c_int
            && Fs_Hz_in != 12000 as libc::c_int
            && Fs_Hz_in != 16000 as libc::c_int
            && Fs_Hz_in != 24000 as libc::c_int
            && Fs_Hz_in != 48000 as libc::c_int
            || Fs_Hz_out != 8000 as libc::c_int
                && Fs_Hz_out != 12000 as libc::c_int
                && Fs_Hz_out != 16000 as libc::c_int
        {
            if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"silk/resampler.c\0" as *const u8 as *const libc::c_char,
                    94 as libc::c_int,
                );
            }
            return -(1 as libc::c_int);
        }
        (*S).inputDelay = delay_matrix_enc[(((Fs_Hz_in >> 12 as libc::c_int)
            - (Fs_Hz_in > 16000 as libc::c_int) as libc::c_int
            >> (Fs_Hz_in > 24000 as libc::c_int) as libc::c_int)
            - 1 as libc::c_int) as usize][(((Fs_Hz_out >> 12 as libc::c_int)
            - (Fs_Hz_out > 16000 as libc::c_int) as libc::c_int
            >> (Fs_Hz_out > 24000 as libc::c_int) as libc::c_int)
            - 1 as libc::c_int) as usize] as libc::c_int;
    } else {
        if Fs_Hz_in != 8000 as libc::c_int
            && Fs_Hz_in != 12000 as libc::c_int
            && Fs_Hz_in != 16000 as libc::c_int
            || Fs_Hz_out != 8000 as libc::c_int
                && Fs_Hz_out != 12000 as libc::c_int
                && Fs_Hz_out != 16000 as libc::c_int
                && Fs_Hz_out != 24000 as libc::c_int
                && Fs_Hz_out != 48000 as libc::c_int
        {
            if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"silk/resampler.c\0" as *const u8 as *const libc::c_char,
                    101 as libc::c_int,
                );
            }
            return -(1 as libc::c_int);
        }
        (*S).inputDelay = delay_matrix_dec[(((Fs_Hz_in >> 12 as libc::c_int)
            - (Fs_Hz_in > 16000 as libc::c_int) as libc::c_int
            >> (Fs_Hz_in > 24000 as libc::c_int) as libc::c_int)
            - 1 as libc::c_int) as usize][(((Fs_Hz_out >> 12 as libc::c_int)
            - (Fs_Hz_out > 16000 as libc::c_int) as libc::c_int
            >> (Fs_Hz_out > 24000 as libc::c_int) as libc::c_int)
            - 1 as libc::c_int) as usize] as libc::c_int;
    }
    (*S).Fs_in_kHz = Fs_Hz_in / 1000 as libc::c_int;
    (*S).Fs_out_kHz = Fs_Hz_out / 1000 as libc::c_int;
    (*S).batchSize = (*S).Fs_in_kHz * RESAMPLER_MAX_BATCH_SIZE_MS;
    up2x = 0 as libc::c_int;
    if Fs_Hz_out > Fs_Hz_in {
        if Fs_Hz_out == Fs_Hz_in * 2 as libc::c_int {
            (*S).resampler_function = USE_silk_resampler_private_up2_HQ_wrapper;
        } else {
            (*S).resampler_function = USE_silk_resampler_private_IIR_FIR;
            up2x = 1 as libc::c_int;
        }
    } else if Fs_Hz_out < Fs_Hz_in {
        (*S).resampler_function = USE_silk_resampler_private_down_FIR;
        if Fs_Hz_out * 4 as libc::c_int == Fs_Hz_in * 3 as libc::c_int {
            (*S).FIR_Fracs = 3 as libc::c_int;
            (*S).FIR_Order = RESAMPLER_DOWN_ORDER_FIR0;
            (*S).Coefs = silk_Resampler_3_4_COEFS.as_ptr();
        } else if Fs_Hz_out * 3 as libc::c_int == Fs_Hz_in * 2 as libc::c_int {
            (*S).FIR_Fracs = 2 as libc::c_int;
            (*S).FIR_Order = RESAMPLER_DOWN_ORDER_FIR0;
            (*S).Coefs = silk_Resampler_2_3_COEFS.as_ptr();
        } else if Fs_Hz_out * 2 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).FIR_Order = RESAMPLER_DOWN_ORDER_FIR1;
            (*S).Coefs = silk_Resampler_1_2_COEFS.as_ptr();
        } else if Fs_Hz_out * 3 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).FIR_Order = RESAMPLER_DOWN_ORDER_FIR2;
            (*S).Coefs = silk_Resampler_1_3_COEFS.as_ptr();
        } else if Fs_Hz_out * 4 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).FIR_Order = RESAMPLER_DOWN_ORDER_FIR2;
            (*S).Coefs = silk_Resampler_1_4_COEFS.as_ptr();
        } else if Fs_Hz_out * 6 as libc::c_int == Fs_Hz_in {
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).FIR_Order = RESAMPLER_DOWN_ORDER_FIR2;
            (*S).Coefs = silk_Resampler_1_6_COEFS.as_ptr();
        } else {
            if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"silk/resampler.c\0" as *const u8 as *const libc::c_char,
                    154 as libc::c_int,
                );
            }
            return -(1 as libc::c_int);
        }
    } else {
        (*S).resampler_function = USE_silk_resampler_copy;
    }
    (*S).invRatio_Q16 = (((((Fs_Hz_in as u32) << 14 as libc::c_int + up2x) as i32 / Fs_Hz_out)
        as u32)
        << 2 as libc::c_int) as i32;
    while (((*S).invRatio_Q16 as i64 * Fs_Hz_out as libc::c_long >> 16 as libc::c_int) as i32)
        < ((Fs_Hz_in as u32) << up2x) as i32
    {
        (*S).invRatio_Q16 += 1;
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "174:1"]
pub unsafe extern "C" fn silk_resampler(
    S: *mut silk_resampler_state_struct,
    out: *mut i16,
    in_0: *const i16,
    inLen: i32,
) -> libc::c_int {
    let mut nSamples: libc::c_int = 0;
    if !(inLen >= (*S).Fs_in_kHz) {
        celt_fatal(
            b"assertion failed: inLen >= S->Fs_in_kHz\0" as *const u8 as *const libc::c_char,
            b"silk/resampler.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
        );
    }
    if !((*S).inputDelay <= (*S).Fs_in_kHz) {
        celt_fatal(
            b"assertion failed: S->inputDelay <= S->Fs_in_kHz\0" as *const u8
                as *const libc::c_char,
            b"silk/resampler.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
        );
    }
    nSamples = (*S).Fs_in_kHz - (*S).inputDelay;
    memcpy(
        &mut *((*S).delayBuf)
            .as_mut_ptr()
            .offset((*S).inputDelay as isize) as *mut i16 as *mut libc::c_void,
        in_0 as *const libc::c_void,
        (nSamples as libc::c_ulong).wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    match (*S).resampler_function {
        USE_silk_resampler_private_up2_HQ_wrapper => {
            silk_resampler_private_up2_HQ_wrapper(
                S as *mut libc::c_void,
                out,
                ((*S).delayBuf).as_mut_ptr(),
                (*S).Fs_in_kHz,
            );
            silk_resampler_private_up2_HQ_wrapper(
                S as *mut libc::c_void,
                &mut *out.offset((*S).Fs_out_kHz as isize),
                &*in_0.offset(nSamples as isize),
                inLen - (*S).Fs_in_kHz,
            );
        }
        USE_silk_resampler_private_IIR_FIR => {
            silk_resampler_private_IIR_FIR(
                S as *mut libc::c_void,
                out,
                ((*S).delayBuf).as_mut_ptr() as *const i16,
                (*S).Fs_in_kHz,
            );
            silk_resampler_private_IIR_FIR(
                S as *mut libc::c_void,
                &mut *out.offset((*S).Fs_out_kHz as isize),
                &*in_0.offset(nSamples as isize),
                inLen - (*S).Fs_in_kHz,
            );
        }
        USE_silk_resampler_private_down_FIR => {
            silk_resampler_private_down_FIR(
                S as *mut libc::c_void,
                out,
                ((*S).delayBuf).as_mut_ptr() as *const i16,
                (*S).Fs_in_kHz,
            );
            silk_resampler_private_down_FIR(
                S as *mut libc::c_void,
                &mut *out.offset((*S).Fs_out_kHz as isize),
                &*in_0.offset(nSamples as isize),
                inLen - (*S).Fs_in_kHz,
            );
        }
        _ => {
            memcpy(
                out as *mut libc::c_void,
                ((*S).delayBuf).as_mut_ptr() as *const libc::c_void,
                ((*S).Fs_in_kHz as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
            );
            memcpy(
                &mut *out.offset((*S).Fs_out_kHz as isize) as *mut i16 as *mut libc::c_void,
                &*in_0.offset(nSamples as isize) as *const i16 as *const libc::c_void,
                ((inLen - (*S).Fs_in_kHz) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
            );
        }
    }
    memcpy(
        ((*S).delayBuf).as_mut_ptr() as *mut libc::c_void,
        &*in_0.offset((inLen - (*S).inputDelay) as isize) as *const i16 as *const libc::c_void,
        ((*S).inputDelay as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    return 0 as libc::c_int;
}
