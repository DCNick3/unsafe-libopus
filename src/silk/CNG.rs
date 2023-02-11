use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
    #[c2rust::src_loc = "43:9"]
    pub const silk_int32_MIN: libc::c_uint = 0x80000000 as libc::c_uint;
}
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN, silk_int32_MAX, silk_int32_MIN};
use crate::celt::celt::celt_fatal;
use crate::externs::{memcpy, memmove, memset};
use crate::silk::define::{CNG_BUF_MASK_MAX, MAX_LPC_ORDER, TYPE_NO_VOICE_ACTIVITY};
use crate::silk::structs::{silk_CNG_struct, silk_decoder_control, silk_decoder_state};
use crate::silk::Inlines::silk_SQRT_APPROX;
use crate::silk::NLSF2A::silk_NLSF2A;
#[inline]
#[c2rust::src_loc = "36:1"]
unsafe fn silk_CNG_exc(
    exc_Q14: *mut i32,
    exc_buf_Q14: *mut i32,
    length: libc::c_int,
    rand_seed: *mut i32,
) {
    let mut seed: i32 = 0;
    let mut i: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut exc_mask: libc::c_int = 0;
    exc_mask = CNG_BUF_MASK_MAX;
    while exc_mask > length {
        exc_mask = exc_mask >> 1 as libc::c_int;
    }
    seed = *rand_seed;
    i = 0 as libc::c_int;
    while i < length {
        seed = (907633515 as libc::c_int as u32)
            .wrapping_add((seed as u32).wrapping_mul(196314165 as libc::c_int as u32))
            as i32;
        idx = seed >> 24 as libc::c_int & exc_mask;
        *exc_Q14.offset(i as isize) = *exc_buf_Q14.offset(idx as isize);
        i += 1;
    }
    *rand_seed = seed;
}
#[c2rust::src_loc = "62:1"]
pub unsafe fn silk_CNG_Reset(mut psDec: *mut silk_decoder_state) {
    let mut i: libc::c_int = 0;
    let mut NLSF_step_Q15: libc::c_int = 0;
    let mut NLSF_acc_Q15: libc::c_int = 0;
    NLSF_step_Q15 = 0x7fff as libc::c_int / ((*psDec).LPC_order + 1 as libc::c_int);
    NLSF_acc_Q15 = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*psDec).LPC_order {
        NLSF_acc_Q15 += NLSF_step_Q15;
        (*psDec).sCNG.CNG_smth_NLSF_Q15[i as usize] = NLSF_acc_Q15 as i16;
        i += 1;
    }
    (*psDec).sCNG.CNG_smth_Gain_Q16 = 0 as libc::c_int;
    (*psDec).sCNG.rand_seed = 3176576 as libc::c_int;
}
#[c2rust::src_loc = "79:1"]
pub unsafe fn silk_CNG(
    psDec: *mut silk_decoder_state,
    psDecCtrl: *mut silk_decoder_control,
    frame: *mut i16,
    length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut subfr: libc::c_int = 0;
    let mut LPC_pred_Q10: i32 = 0;
    let mut max_Gain_Q16: i32 = 0;
    let mut gain_Q16: i32 = 0;
    let mut gain_Q10: i32 = 0;
    let mut A_Q12: [i16; 16] = [0; 16];
    let mut psCNG: *mut silk_CNG_struct = &mut (*psDec).sCNG;
    if (*psDec).fs_kHz != (*psCNG).fs_kHz {
        silk_CNG_Reset(psDec);
        (*psCNG).fs_kHz = (*psDec).fs_kHz;
    }
    if (*psDec).lossCnt == 0 as libc::c_int && (*psDec).prevSignalType == TYPE_NO_VOICE_ACTIVITY {
        i = 0 as libc::c_int;
        while i < (*psDec).LPC_order {
            (*psCNG).CNG_smth_NLSF_Q15[i as usize] =
                ((*psCNG).CNG_smth_NLSF_Q15[i as usize] as libc::c_int
                    + (((*psDec).prevNLSF_Q15[i as usize] as i32
                        - (*psCNG).CNG_smth_NLSF_Q15[i as usize] as i32)
                        as libc::c_long
                        * 16348 as libc::c_int as i16 as i64
                        >> 16 as libc::c_int) as i32) as i16;
            i += 1;
        }
        max_Gain_Q16 = 0 as libc::c_int;
        subfr = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*psDec).nb_subfr {
            if (*psDecCtrl).Gains_Q16[i as usize] > max_Gain_Q16 {
                max_Gain_Q16 = (*psDecCtrl).Gains_Q16[i as usize];
                subfr = i;
            }
            i += 1;
        }
        memmove(
            &mut *((*psCNG).CNG_exc_buf_Q14)
                .as_mut_ptr()
                .offset((*psDec).subfr_length as isize) as *mut i32
                as *mut libc::c_void,
            ((*psCNG).CNG_exc_buf_Q14).as_mut_ptr() as *const libc::c_void,
            ((((*psDec).nb_subfr - 1 as libc::c_int) * (*psDec).subfr_length) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
        memcpy(
            ((*psCNG).CNG_exc_buf_Q14).as_mut_ptr() as *mut libc::c_void,
            &mut *((*psDec).exc_Q14)
                .as_mut_ptr()
                .offset((subfr * (*psDec).subfr_length) as isize) as *mut i32
                as *const libc::c_void,
            ((*psDec).subfr_length as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
        i = 0 as libc::c_int;
        while i < (*psDec).nb_subfr {
            (*psCNG).CNG_smth_Gain_Q16 +=
                (((*psDecCtrl).Gains_Q16[i as usize] - (*psCNG).CNG_smth_Gain_Q16) as libc::c_long
                    * 4634 as libc::c_int as i16 as i64
                    >> 16 as libc::c_int) as i32;
            i += 1;
        }
    }
    if (*psDec).lossCnt != 0 {
        let vla = (length + 16 as libc::c_int) as usize;
        let mut CNG_sig_Q14: Vec<i32> = ::std::vec::from_elem(0, vla);
        gain_Q16 = ((*psDec).sPLC.randScale_Q14 as i64
            * (*psDec).sPLC.prevGain_Q16[1 as libc::c_int as usize] as libc::c_long
            >> 16 as libc::c_int) as i32;
        if gain_Q16 >= (1 as libc::c_int) << 21 as libc::c_int
            || (*psCNG).CNG_smth_Gain_Q16 > (1 as libc::c_int) << 23 as libc::c_int
        {
            gain_Q16 = (gain_Q16 >> 16 as libc::c_int) * (gain_Q16 >> 16 as libc::c_int);
            gain_Q16 = ((*psCNG).CNG_smth_Gain_Q16 >> 16 as libc::c_int)
                * ((*psCNG).CNG_smth_Gain_Q16 >> 16 as libc::c_int)
                - ((gain_Q16 as u32) << 5 as libc::c_int) as i32;
            gain_Q16 = ((silk_SQRT_APPROX(gain_Q16) as u32) << 16 as libc::c_int) as i32;
        } else {
            gain_Q16 = (gain_Q16 as i64 * gain_Q16 as libc::c_long >> 16 as libc::c_int) as i32;
            gain_Q16 = ((*psCNG).CNG_smth_Gain_Q16 as i64
                * (*psCNG).CNG_smth_Gain_Q16 as libc::c_long
                >> 16 as libc::c_int) as i32
                - ((gain_Q16 as u32) << 5 as libc::c_int) as i32;
            gain_Q16 = ((silk_SQRT_APPROX(gain_Q16) as u32) << 8 as libc::c_int) as i32;
        }
        gain_Q10 = gain_Q16 >> 6 as libc::c_int;
        silk_CNG_exc(
            CNG_sig_Q14.as_mut_ptr().offset(MAX_LPC_ORDER as isize),
            ((*psCNG).CNG_exc_buf_Q14).as_mut_ptr(),
            length,
            &mut (*psCNG).rand_seed,
        );
        silk_NLSF2A(
            A_Q12.as_mut_ptr(),
            ((*psCNG).CNG_smth_NLSF_Q15).as_mut_ptr(),
            (*psDec).LPC_order,
            (*psDec).arch,
        );
        memcpy(
            CNG_sig_Q14.as_mut_ptr() as *mut libc::c_void,
            ((*psCNG).CNG_synth_state).as_mut_ptr() as *const libc::c_void,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
        if !((*psDec).LPC_order == 10 as libc::c_int || (*psDec).LPC_order == 16 as libc::c_int) {
            celt_fatal(
                b"assertion failed: psDec->LPC_order == 10 || psDec->LPC_order == 16\0" as *const u8
                    as *const libc::c_char,
                b"silk/CNG.c\0" as *const u8 as *const libc::c_char,
                149 as libc::c_int,
            );
        }
        i = 0 as libc::c_int;
        while i < length {
            LPC_pred_Q10 = (*psDec).LPC_order >> 1 as libc::c_int;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 1 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12[0 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 2 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12[1 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 3 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12[2 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 4 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12[3 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 5 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12[4 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 6 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12[5 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 7 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12[6 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 8 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12[7 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 9 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12[8 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 10 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12[9 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            if (*psDec).LPC_order == 16 as libc::c_int {
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*CNG_sig_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 11 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12[10 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*CNG_sig_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 12 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12[11 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*CNG_sig_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 13 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12[12 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*CNG_sig_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 14 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12[13 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*CNG_sig_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 15 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12[14 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*CNG_sig_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 16 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12[15 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
            }
            *CNG_sig_Q14
                .as_mut_ptr()
                .offset((MAX_LPC_ORDER + i) as isize) = if (*CNG_sig_Q14
                .as_mut_ptr()
                .offset((16 as libc::c_int + i) as isize)
                as u32)
                .wrapping_add(
                    (((if 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                        > 0x7fffffff as libc::c_int >> 4 as libc::c_int
                    {
                        if LPC_pred_Q10 > 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int {
                            0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                        } else {
                            if LPC_pred_Q10 < 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                                0x7fffffff as libc::c_int >> 4 as libc::c_int
                            } else {
                                LPC_pred_Q10
                            }
                        }
                    } else {
                        if LPC_pred_Q10 > 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                            0x7fffffff as libc::c_int >> 4 as libc::c_int
                        } else {
                            if LPC_pred_Q10 < 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                            {
                                0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                            } else {
                                LPC_pred_Q10
                            }
                        }
                    }) as u32)
                        << 4 as libc::c_int) as i32 as u32,
                )
                & 0x80000000 as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                if (*CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i) as isize)
                    & (((if 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                        > 0x7fffffff as libc::c_int >> 4 as libc::c_int
                    {
                        if LPC_pred_Q10 > 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int {
                            0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                        } else {
                            if LPC_pred_Q10 < 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                                0x7fffffff as libc::c_int >> 4 as libc::c_int
                            } else {
                                LPC_pred_Q10
                            }
                        }
                    } else {
                        if LPC_pred_Q10 > 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                            0x7fffffff as libc::c_int >> 4 as libc::c_int
                        } else {
                            if LPC_pred_Q10 < 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                            {
                                0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                            } else {
                                LPC_pred_Q10
                            }
                        }
                    }) as u32)
                        << 4 as libc::c_int) as i32) as libc::c_uint
                    & 0x80000000 as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    silk_int32_MIN as i32
                } else {
                    *CNG_sig_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i) as isize)
                        + (((if 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                            > 0x7fffffff as libc::c_int >> 4 as libc::c_int
                        {
                            if LPC_pred_Q10 > 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                            {
                                0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                            } else {
                                if LPC_pred_Q10 < 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                                    0x7fffffff as libc::c_int >> 4 as libc::c_int
                                } else {
                                    LPC_pred_Q10
                                }
                            }
                        } else {
                            if LPC_pred_Q10 > 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                                0x7fffffff as libc::c_int >> 4 as libc::c_int
                            } else {
                                if LPC_pred_Q10
                                    < 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                                {
                                    0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                                } else {
                                    LPC_pred_Q10
                                }
                            }
                        }) as u32)
                            << 4 as libc::c_int) as i32
                }
            } else if (*CNG_sig_Q14
                .as_mut_ptr()
                .offset((16 as libc::c_int + i) as isize)
                | (((if 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                    > 0x7fffffff as libc::c_int >> 4 as libc::c_int
                {
                    if LPC_pred_Q10 > 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int {
                        0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                    } else {
                        if LPC_pred_Q10 < 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                            0x7fffffff as libc::c_int >> 4 as libc::c_int
                        } else {
                            LPC_pred_Q10
                        }
                    }
                } else {
                    if LPC_pred_Q10 > 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                        0x7fffffff as libc::c_int >> 4 as libc::c_int
                    } else {
                        if LPC_pred_Q10 < 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int {
                            0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                        } else {
                            LPC_pred_Q10
                        }
                    }
                }) as u32)
                    << 4 as libc::c_int) as i32) as libc::c_uint
                & 0x80000000 as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                silk_int32_MAX
            } else {
                *CNG_sig_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i) as isize)
                    + (((if 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                        > 0x7fffffff as libc::c_int >> 4 as libc::c_int
                    {
                        if LPC_pred_Q10 > 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int {
                            0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                        } else {
                            if LPC_pred_Q10 < 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                                0x7fffffff as libc::c_int >> 4 as libc::c_int
                            } else {
                                LPC_pred_Q10
                            }
                        }
                    } else {
                        if LPC_pred_Q10 > 0x7fffffff as libc::c_int >> 4 as libc::c_int {
                            0x7fffffff as libc::c_int >> 4 as libc::c_int
                        } else {
                            if LPC_pred_Q10 < 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                            {
                                0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                            } else {
                                LPC_pred_Q10
                            }
                        }
                    }) as u32)
                        << 4 as libc::c_int) as i32
            };
            *frame.offset(i as isize) = (if *frame.offset(i as isize) as i32
                + (if (if 8 as libc::c_int == 1 as libc::c_int {
                    ((*CNG_sig_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i) as isize) as i64
                        * gain_Q10 as libc::c_long
                        >> 16 as libc::c_int) as i32
                        >> 1 as libc::c_int)
                        + ((*CNG_sig_Q14
                            .as_mut_ptr()
                            .offset((16 as libc::c_int + i) as isize)
                            as i64
                            * gain_Q10 as libc::c_long
                            >> 16 as libc::c_int) as i32
                            & 1 as libc::c_int)
                } else {
                    ((*CNG_sig_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i) as isize) as i64
                        * gain_Q10 as libc::c_long
                        >> 16 as libc::c_int) as i32
                        >> 8 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) > 0x7fff as libc::c_int
                {
                    0x7fff as libc::c_int
                } else {
                    if (if 8 as libc::c_int == 1 as libc::c_int {
                        ((*CNG_sig_Q14
                            .as_mut_ptr()
                            .offset((16 as libc::c_int + i) as isize)
                            as i64
                            * gain_Q10 as libc::c_long
                            >> 16 as libc::c_int) as i32
                            >> 1 as libc::c_int)
                            + ((*CNG_sig_Q14
                                .as_mut_ptr()
                                .offset((16 as libc::c_int + i) as isize)
                                as i64
                                * gain_Q10 as libc::c_long
                                >> 16 as libc::c_int) as i32
                                & 1 as libc::c_int)
                    } else {
                        ((*CNG_sig_Q14
                            .as_mut_ptr()
                            .offset((16 as libc::c_int + i) as isize)
                            as i64
                            * gain_Q10 as libc::c_long
                            >> 16 as libc::c_int) as i32
                            >> 8 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) < 0x8000 as libc::c_int as i16 as libc::c_int
                    {
                        0x8000 as libc::c_int as i16 as libc::c_int
                    } else {
                        if 8 as libc::c_int == 1 as libc::c_int {
                            ((*CNG_sig_Q14
                                .as_mut_ptr()
                                .offset((16 as libc::c_int + i) as isize)
                                as i64
                                * gain_Q10 as libc::c_long
                                >> 16 as libc::c_int) as i32
                                >> 1 as libc::c_int)
                                + ((*CNG_sig_Q14
                                    .as_mut_ptr()
                                    .offset((16 as libc::c_int + i) as isize)
                                    as i64
                                    * gain_Q10 as libc::c_long
                                    >> 16 as libc::c_int) as i32
                                    & 1 as libc::c_int)
                        } else {
                            ((*CNG_sig_Q14
                                .as_mut_ptr()
                                .offset((16 as libc::c_int + i) as isize)
                                as i64
                                * gain_Q10 as libc::c_long
                                >> 16 as libc::c_int) as i32
                                >> 8 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }
                    }
                })
                > silk_int16_MAX
            {
                silk_int16_MAX
            } else if *frame.offset(i as isize) as i32
                + (if (if 8 as libc::c_int == 1 as libc::c_int {
                    ((*CNG_sig_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i) as isize) as i64
                        * gain_Q10 as libc::c_long
                        >> 16 as libc::c_int) as i32
                        >> 1 as libc::c_int)
                        + ((*CNG_sig_Q14
                            .as_mut_ptr()
                            .offset((16 as libc::c_int + i) as isize)
                            as i64
                            * gain_Q10 as libc::c_long
                            >> 16 as libc::c_int) as i32
                            & 1 as libc::c_int)
                } else {
                    ((*CNG_sig_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i) as isize) as i64
                        * gain_Q10 as libc::c_long
                        >> 16 as libc::c_int) as i32
                        >> 8 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) > 0x7fff as libc::c_int
                {
                    0x7fff as libc::c_int
                } else {
                    if (if 8 as libc::c_int == 1 as libc::c_int {
                        ((*CNG_sig_Q14
                            .as_mut_ptr()
                            .offset((16 as libc::c_int + i) as isize)
                            as i64
                            * gain_Q10 as libc::c_long
                            >> 16 as libc::c_int) as i32
                            >> 1 as libc::c_int)
                            + ((*CNG_sig_Q14
                                .as_mut_ptr()
                                .offset((16 as libc::c_int + i) as isize)
                                as i64
                                * gain_Q10 as libc::c_long
                                >> 16 as libc::c_int) as i32
                                & 1 as libc::c_int)
                    } else {
                        ((*CNG_sig_Q14
                            .as_mut_ptr()
                            .offset((16 as libc::c_int + i) as isize)
                            as i64
                            * gain_Q10 as libc::c_long
                            >> 16 as libc::c_int) as i32
                            >> 8 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) < 0x8000 as libc::c_int as i16 as libc::c_int
                    {
                        0x8000 as libc::c_int as i16 as libc::c_int
                    } else {
                        if 8 as libc::c_int == 1 as libc::c_int {
                            ((*CNG_sig_Q14
                                .as_mut_ptr()
                                .offset((16 as libc::c_int + i) as isize)
                                as i64
                                * gain_Q10 as libc::c_long
                                >> 16 as libc::c_int) as i32
                                >> 1 as libc::c_int)
                                + ((*CNG_sig_Q14
                                    .as_mut_ptr()
                                    .offset((16 as libc::c_int + i) as isize)
                                    as i64
                                    * gain_Q10 as libc::c_long
                                    >> 16 as libc::c_int) as i32
                                    & 1 as libc::c_int)
                        } else {
                            ((*CNG_sig_Q14
                                .as_mut_ptr()
                                .offset((16 as libc::c_int + i) as isize)
                                as i64
                                * gain_Q10 as libc::c_long
                                >> 16 as libc::c_int) as i32
                                >> 8 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }
                    }
                })
                < silk_int16_MIN
            {
                silk_int16_MIN
            } else {
                *frame.offset(i as isize) as i32
                    + (if (if 8 as libc::c_int == 1 as libc::c_int {
                        ((*CNG_sig_Q14
                            .as_mut_ptr()
                            .offset((16 as libc::c_int + i) as isize)
                            as i64
                            * gain_Q10 as libc::c_long
                            >> 16 as libc::c_int) as i32
                            >> 1 as libc::c_int)
                            + ((*CNG_sig_Q14
                                .as_mut_ptr()
                                .offset((16 as libc::c_int + i) as isize)
                                as i64
                                * gain_Q10 as libc::c_long
                                >> 16 as libc::c_int) as i32
                                & 1 as libc::c_int)
                    } else {
                        ((*CNG_sig_Q14
                            .as_mut_ptr()
                            .offset((16 as libc::c_int + i) as isize)
                            as i64
                            * gain_Q10 as libc::c_long
                            >> 16 as libc::c_int) as i32
                            >> 8 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int
                            >> 1 as libc::c_int
                    }) > 0x7fff as libc::c_int
                    {
                        0x7fff as libc::c_int
                    } else {
                        if (if 8 as libc::c_int == 1 as libc::c_int {
                            ((*CNG_sig_Q14
                                .as_mut_ptr()
                                .offset((16 as libc::c_int + i) as isize)
                                as i64
                                * gain_Q10 as libc::c_long
                                >> 16 as libc::c_int) as i32
                                >> 1 as libc::c_int)
                                + ((*CNG_sig_Q14
                                    .as_mut_ptr()
                                    .offset((16 as libc::c_int + i) as isize)
                                    as i64
                                    * gain_Q10 as libc::c_long
                                    >> 16 as libc::c_int) as i32
                                    & 1 as libc::c_int)
                        } else {
                            ((*CNG_sig_Q14
                                .as_mut_ptr()
                                .offset((16 as libc::c_int + i) as isize)
                                as i64
                                * gain_Q10 as libc::c_long
                                >> 16 as libc::c_int) as i32
                                >> 8 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int
                                >> 1 as libc::c_int
                        }) < 0x8000 as libc::c_int as i16 as libc::c_int
                        {
                            0x8000 as libc::c_int as i16 as libc::c_int
                        } else {
                            if 8 as libc::c_int == 1 as libc::c_int {
                                ((*CNG_sig_Q14
                                    .as_mut_ptr()
                                    .offset((16 as libc::c_int + i) as isize)
                                    as i64
                                    * gain_Q10 as libc::c_long
                                    >> 16 as libc::c_int) as i32
                                    >> 1 as libc::c_int)
                                    + ((*CNG_sig_Q14
                                        .as_mut_ptr()
                                        .offset((16 as libc::c_int + i) as isize)
                                        as i64
                                        * gain_Q10 as libc::c_long
                                        >> 16 as libc::c_int)
                                        as i32
                                        & 1 as libc::c_int)
                            } else {
                                ((*CNG_sig_Q14
                                    .as_mut_ptr()
                                    .offset((16 as libc::c_int + i) as isize)
                                    as i64
                                    * gain_Q10 as libc::c_long
                                    >> 16 as libc::c_int) as i32
                                    >> 8 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int
                                    >> 1 as libc::c_int
                            }
                        }
                    })
            }) as i16;
            i += 1;
        }
        memcpy(
            ((*psCNG).CNG_synth_state).as_mut_ptr() as *mut libc::c_void,
            &mut *CNG_sig_Q14.as_mut_ptr().offset(length as isize) as *mut i32
                as *const libc::c_void,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
    } else {
        memset(
            ((*psCNG).CNG_synth_state).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ((*psDec).LPC_order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
    };
}
