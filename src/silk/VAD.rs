use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "48:9"]
    pub const silk_uint8_MAX: libc::c_int = 0xff as libc::c_int;
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
}
pub use self::typedef_h::{silk_int32_MAX, silk_uint8_MAX};
use crate::celt::celt::celt_fatal;
use crate::externs::memset;
use crate::silk::ana_filt_bank_1::silk_ana_filt_bank_1;
use crate::silk::define::{
    VAD_INTERNAL_SUBFRAMES, VAD_NEGATIVE_OFFSET_Q5, VAD_NOISE_LEVEL_SMOOTH_COEF_Q16, VAD_N_BANDS,
};
use crate::silk::lin2log::silk_lin2log;
use crate::silk::sigm_Q15::silk_sigm_Q15;
use crate::silk::structs::{silk_VAD_state, silk_encoder_state};
use crate::silk::Inlines::silk_SQRT_APPROX;
use crate::silk::SigProc_FIX::{silk_max_32, silk_max_int, silk_min_int};

#[c2rust::src_loc = "46:1"]
pub unsafe extern "C" fn silk_VAD_Init(mut psSilk_VAD: *mut silk_VAD_state) -> libc::c_int {
    let mut b: libc::c_int = 0;
    let ret: libc::c_int = 0 as libc::c_int;
    memset(
        psSilk_VAD as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<silk_VAD_state>() as libc::c_ulong,
    );
    b = 0 as libc::c_int;
    while b < VAD_N_BANDS {
        (*psSilk_VAD).NoiseLevelBias[b as usize] =
            silk_max_32(50 as libc::c_int / (b + 1 as libc::c_int), 1 as libc::c_int);
        b += 1;
    }
    b = 0 as libc::c_int;
    while b < VAD_N_BANDS {
        (*psSilk_VAD).NL[b as usize] =
            100 as libc::c_int * (*psSilk_VAD).NoiseLevelBias[b as usize];
        (*psSilk_VAD).inv_NL[b as usize] = 0x7fffffff as libc::c_int / (*psSilk_VAD).NL[b as usize];
        b += 1;
    }
    (*psSilk_VAD).counter = 15 as libc::c_int;
    b = 0 as libc::c_int;
    while b < VAD_N_BANDS {
        (*psSilk_VAD).NrgRatioSmth_Q8[b as usize] = 100 as libc::c_int * 256 as libc::c_int;
        b += 1;
    }
    return ret;
}
#[c2rust::src_loc = "77:25"]
static mut tiltWeights: [i32; 4] = [
    30000 as libc::c_int,
    6000 as libc::c_int,
    -(12000 as libc::c_int),
    -(12000 as libc::c_int),
];
#[c2rust::src_loc = "82:1"]
pub unsafe extern "C" fn silk_VAD_GetSA_Q8_c(
    mut psEncC: *mut silk_encoder_state,
    pIn: *const i16,
) -> libc::c_int {
    let mut SA_Q15: libc::c_int = 0;
    let mut pSNR_dB_Q7: libc::c_int = 0;
    let mut input_tilt: libc::c_int = 0;
    let mut decimated_framelength1: libc::c_int = 0;
    let mut decimated_framelength2: libc::c_int = 0;
    let mut decimated_framelength: libc::c_int = 0;
    let mut dec_subframe_length: libc::c_int = 0;
    let mut dec_subframe_offset: libc::c_int = 0;
    let mut SNR_Q7: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut sumSquared: i32 = 0;
    let mut smooth_coef_Q16: i32 = 0;
    let mut HPstateTmp: i16 = 0;
    let mut Xnrg: [i32; 4] = [0; 4];
    let mut NrgToNoiseRatio_Q8: [i32; 4] = [0; 4];
    let mut speech_nrg: i32 = 0;
    let mut x_tmp: i32 = 0;
    let mut X_offset: [libc::c_int; 4] = [0; 4];
    let ret: libc::c_int = 0 as libc::c_int;
    let mut psSilk_VAD: *mut silk_VAD_state = &mut (*psEncC).sVAD;
    if !(5 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int >= (*psEncC).frame_length) {
        celt_fatal(
            b"assertion failed: MAX_FRAME_LENGTH >= psEncC->frame_length\0" as *const u8
                as *const libc::c_char,
            b"silk/VAD.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
        );
    }
    if !((*psEncC).frame_length <= 512 as libc::c_int) {
        celt_fatal(
            b"assertion failed: psEncC->frame_length <= 512\0" as *const u8 as *const libc::c_char,
            b"silk/VAD.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
        );
    }
    if !((*psEncC).frame_length == 8 as libc::c_int * ((*psEncC).frame_length >> 3 as libc::c_int))
    {
        celt_fatal(
            b"assertion failed: psEncC->frame_length == 8 * silk_RSHIFT( psEncC->frame_length, 3 )\0"
                as *const u8 as *const libc::c_char,
            b"silk/VAD.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
        );
    }
    decimated_framelength1 = (*psEncC).frame_length >> 1 as libc::c_int;
    decimated_framelength2 = (*psEncC).frame_length >> 2 as libc::c_int;
    decimated_framelength = (*psEncC).frame_length >> 3 as libc::c_int;
    X_offset[0 as libc::c_int as usize] = 0 as libc::c_int;
    X_offset[1 as libc::c_int as usize] = decimated_framelength + decimated_framelength2;
    X_offset[2 as libc::c_int as usize] =
        X_offset[1 as libc::c_int as usize] + decimated_framelength;
    X_offset[3 as libc::c_int as usize] =
        X_offset[2 as libc::c_int as usize] + decimated_framelength2;
    let vla = (X_offset[3 as libc::c_int as usize] + decimated_framelength1) as usize;
    let mut X: Vec<i16> = ::std::vec::from_elem(0, vla);
    silk_ana_filt_bank_1(
        pIn,
        &mut *((*psSilk_VAD).AnaState)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        X.as_mut_ptr(),
        &mut *X
            .as_mut_ptr()
            .offset(*X_offset.as_mut_ptr().offset(3 as libc::c_int as isize) as isize),
        (*psEncC).frame_length,
    );
    silk_ana_filt_bank_1(
        X.as_mut_ptr(),
        &mut *((*psSilk_VAD).AnaState1)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        X.as_mut_ptr(),
        &mut *X
            .as_mut_ptr()
            .offset(*X_offset.as_mut_ptr().offset(2 as libc::c_int as isize) as isize),
        decimated_framelength1,
    );
    silk_ana_filt_bank_1(
        X.as_mut_ptr(),
        &mut *((*psSilk_VAD).AnaState2)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        X.as_mut_ptr(),
        &mut *X
            .as_mut_ptr()
            .offset(*X_offset.as_mut_ptr().offset(1 as libc::c_int as isize) as isize),
        decimated_framelength2,
    );
    *X.as_mut_ptr()
        .offset((decimated_framelength - 1 as libc::c_int) as isize) =
        (*X.as_mut_ptr()
            .offset((decimated_framelength - 1 as libc::c_int) as isize) as libc::c_int
            >> 1 as libc::c_int) as i16;
    HPstateTmp = *X
        .as_mut_ptr()
        .offset((decimated_framelength - 1 as libc::c_int) as isize);
    i = decimated_framelength - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        *X.as_mut_ptr().offset((i - 1 as libc::c_int) as isize) =
            (*X.as_mut_ptr().offset((i - 1 as libc::c_int) as isize) as libc::c_int
                >> 1 as libc::c_int) as i16;
        let ref mut fresh0 = *X.as_mut_ptr().offset(i as isize);
        *fresh0 = (*fresh0 as libc::c_int
            - *X.as_mut_ptr().offset((i - 1 as libc::c_int) as isize) as libc::c_int)
            as i16;
        i -= 1;
    }
    let ref mut fresh1 = *X.as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh1 = (*fresh1 as libc::c_int - (*psSilk_VAD).HPstate as libc::c_int) as i16;
    (*psSilk_VAD).HPstate = HPstateTmp;
    b = 0 as libc::c_int;
    while b < VAD_N_BANDS {
        decimated_framelength = (*psEncC).frame_length
            >> silk_min_int(4 as libc::c_int - b, 4 as libc::c_int - 1 as libc::c_int);
        dec_subframe_length = decimated_framelength >> 2 as libc::c_int;
        dec_subframe_offset = 0 as libc::c_int;
        Xnrg[b as usize] = (*psSilk_VAD).XnrgSubfr[b as usize];
        s = 0 as libc::c_int;
        while s < VAD_INTERNAL_SUBFRAMES {
            sumSquared = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < dec_subframe_length {
                x_tmp = *X
                    .as_mut_ptr()
                    .offset((X_offset[b as usize] + i + dec_subframe_offset) as isize)
                    as libc::c_int
                    >> 3 as libc::c_int;
                sumSquared = sumSquared + x_tmp as i16 as i32 * x_tmp as i16 as i32;
                i += 1;
            }
            if s < VAD_INTERNAL_SUBFRAMES - 1 as libc::c_int {
                Xnrg[b as usize] = if (Xnrg[b as usize] as u32).wrapping_add(sumSquared as u32)
                    & 0x80000000 as libc::c_uint
                    != 0
                {
                    silk_int32_MAX
                } else {
                    Xnrg[b as usize] + sumSquared
                };
            } else {
                Xnrg[b as usize] = if (Xnrg[b as usize] as u32)
                    .wrapping_add((sumSquared >> 1 as libc::c_int) as u32)
                    & 0x80000000 as libc::c_uint
                    != 0
                {
                    silk_int32_MAX
                } else {
                    Xnrg[b as usize] + (sumSquared >> 1 as libc::c_int)
                };
            }
            dec_subframe_offset += dec_subframe_length;
            s += 1;
        }
        (*psSilk_VAD).XnrgSubfr[b as usize] = sumSquared;
        b += 1;
    }
    silk_VAD_GetNoiseLevels(
        &mut *Xnrg.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut i32 as *const i32,
        psSilk_VAD,
    );
    sumSquared = 0 as libc::c_int;
    input_tilt = 0 as libc::c_int;
    b = 0 as libc::c_int;
    while b < VAD_N_BANDS {
        speech_nrg = Xnrg[b as usize] - (*psSilk_VAD).NL[b as usize];
        if speech_nrg > 0 as libc::c_int {
            if Xnrg[b as usize] as libc::c_uint & 0xff800000 as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                NrgToNoiseRatio_Q8[b as usize] = ((Xnrg[b as usize] as u32) << 8 as libc::c_int)
                    as i32
                    / ((*psSilk_VAD).NL[b as usize] + 1 as libc::c_int);
            } else {
                NrgToNoiseRatio_Q8[b as usize] = Xnrg[b as usize]
                    / (((*psSilk_VAD).NL[b as usize] >> 8 as libc::c_int) + 1 as libc::c_int);
            }
            SNR_Q7 = silk_lin2log(NrgToNoiseRatio_Q8[b as usize])
                - 8 as libc::c_int * 128 as libc::c_int;
            sumSquared = sumSquared + SNR_Q7 as i16 as i32 * SNR_Q7 as i16 as i32;
            if speech_nrg < (1 as libc::c_int) << 20 as libc::c_int {
                SNR_Q7 = (((silk_SQRT_APPROX(speech_nrg) as u32) << 6 as libc::c_int) as i32
                    as libc::c_long
                    * SNR_Q7 as i16 as i64
                    >> 16 as libc::c_int) as i32;
            }
            input_tilt = (input_tilt as libc::c_long
                + (tiltWeights[b as usize] as libc::c_long * SNR_Q7 as i16 as i64
                    >> 16 as libc::c_int)) as i32;
        } else {
            NrgToNoiseRatio_Q8[b as usize] = 256 as libc::c_int;
        }
        b += 1;
    }
    sumSquared = sumSquared / 4 as libc::c_int;
    pSNR_dB_Q7 = (3 as libc::c_int * silk_SQRT_APPROX(sumSquared)) as i16 as libc::c_int;
    SA_Q15 = silk_sigm_Q15(
        (45000 as libc::c_int as libc::c_long * pSNR_dB_Q7 as i16 as i64 >> 16 as libc::c_int)
            as i32
            - VAD_NEGATIVE_OFFSET_Q5,
    );
    (*psEncC).input_tilt_Q15 =
        (((silk_sigm_Q15(input_tilt) - 16384 as libc::c_int) as u32) << 1 as libc::c_int) as i32;
    speech_nrg = 0 as libc::c_int;
    b = 0 as libc::c_int;
    while b < VAD_N_BANDS {
        speech_nrg += (b + 1 as libc::c_int)
            * (Xnrg[b as usize] - (*psSilk_VAD).NL[b as usize] >> 4 as libc::c_int);
        b += 1;
    }
    if (*psEncC).frame_length == 20 as libc::c_int * (*psEncC).fs_kHz {
        speech_nrg = speech_nrg >> 1 as libc::c_int;
    }
    if speech_nrg <= 0 as libc::c_int {
        SA_Q15 = SA_Q15 >> 1 as libc::c_int;
    } else if speech_nrg < 16384 as libc::c_int {
        speech_nrg = ((speech_nrg as u32) << 16 as libc::c_int) as i32;
        speech_nrg = silk_SQRT_APPROX(speech_nrg);
        SA_Q15 = ((32768 as libc::c_int + speech_nrg) as libc::c_long * SA_Q15 as i16 as i64
            >> 16 as libc::c_int) as i32;
    }
    (*psEncC).speech_activity_Q8 = silk_min_int(SA_Q15 >> 7 as libc::c_int, silk_uint8_MAX);
    smooth_coef_Q16 = (4096 as libc::c_int as libc::c_long
        * (SA_Q15 as libc::c_long * SA_Q15 as i16 as i64 >> 16 as libc::c_int) as i32 as i16 as i64
        >> 16 as libc::c_int) as i32;
    if (*psEncC).frame_length == 10 as libc::c_int * (*psEncC).fs_kHz {
        smooth_coef_Q16 >>= 1 as libc::c_int;
    }
    b = 0 as libc::c_int;
    while b < VAD_N_BANDS {
        (*psSilk_VAD).NrgRatioSmth_Q8[b as usize] = ((*psSilk_VAD).NrgRatioSmth_Q8[b as usize]
            as libc::c_long
            + ((NrgToNoiseRatio_Q8[b as usize] - (*psSilk_VAD).NrgRatioSmth_Q8[b as usize])
                as libc::c_long
                * smooth_coef_Q16 as i16 as i64
                >> 16 as libc::c_int)) as i32;
        SNR_Q7 = 3 as libc::c_int
            * (silk_lin2log((*psSilk_VAD).NrgRatioSmth_Q8[b as usize])
                - 8 as libc::c_int * 128 as libc::c_int);
        (*psEncC).input_quality_bands_Q15[b as usize] =
            silk_sigm_Q15(SNR_Q7 - 16 as libc::c_int * 128 as libc::c_int >> 4 as libc::c_int);
        b += 1;
    }
    return ret;
}
#[inline]
#[c2rust::src_loc = "301:1"]
unsafe extern "C" fn silk_VAD_GetNoiseLevels(pX: *const i32, mut psSilk_VAD: *mut silk_VAD_state) {
    let mut k: libc::c_int = 0;
    let mut nl: i32 = 0;
    let mut nrg: i32 = 0;
    let mut inv_nrg: i32 = 0;
    let mut coef: libc::c_int = 0;
    let mut min_coef: libc::c_int = 0;
    if (*psSilk_VAD).counter < 1000 as libc::c_int {
        min_coef = 0x7fff as libc::c_int
            / (((*psSilk_VAD).counter >> 4 as libc::c_int) + 1 as libc::c_int);
        (*psSilk_VAD).counter += 1;
    } else {
        min_coef = 0 as libc::c_int;
    }
    k = 0 as libc::c_int;
    while k < VAD_N_BANDS {
        nl = (*psSilk_VAD).NL[k as usize];
        nrg = if (*pX.offset(k as isize) as u32)
            .wrapping_add((*psSilk_VAD).NoiseLevelBias[k as usize] as u32)
            & 0x80000000 as libc::c_uint
            != 0
        {
            silk_int32_MAX
        } else {
            *pX.offset(k as isize) + (*psSilk_VAD).NoiseLevelBias[k as usize]
        };
        inv_nrg = 0x7fffffff as libc::c_int / nrg;
        if nrg > ((nl as u32) << 3 as libc::c_int) as i32 {
            coef = VAD_NOISE_LEVEL_SMOOTH_COEF_Q16 >> 3 as libc::c_int;
        } else if nrg < nl {
            coef = VAD_NOISE_LEVEL_SMOOTH_COEF_Q16;
        } else {
            coef = ((inv_nrg as i64 * nl as libc::c_long >> 16 as libc::c_int) as i32
                as libc::c_long
                * ((1024 as libc::c_int) << 1 as libc::c_int) as i16 as i64
                >> 16 as libc::c_int) as i32;
        }
        coef = silk_max_int(coef, min_coef);
        (*psSilk_VAD).inv_NL[k as usize] = ((*psSilk_VAD).inv_NL[k as usize] as libc::c_long
            + ((inv_nrg - (*psSilk_VAD).inv_NL[k as usize]) as libc::c_long * coef as i16 as i64
                >> 16 as libc::c_int)) as i32;
        nl = 0x7fffffff as libc::c_int / (*psSilk_VAD).inv_NL[k as usize];
        nl = if nl < 0xffffff as libc::c_int {
            nl
        } else {
            0xffffff as libc::c_int
        };
        (*psSilk_VAD).NL[k as usize] = nl;
        k += 1;
    }
}
