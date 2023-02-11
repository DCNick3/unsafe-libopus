use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "155:9"]
    pub const MAX_SHAPE_LPC_ORDER: libc::c_int = 24 as libc::c_int;
    #[c2rust::src_loc = "157:9"]
    pub const HARM_SHAPE_FIR_TAPS: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "146:9"]
    pub const LTP_ORDER: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "180:10"]
    pub const NSQ_LPC_BUF_LENGTH: libc::c_int = MAX_LPC_ORDER;
    #[c2rust::src_loc = "142:9"]
    pub const MAX_LPC_ORDER: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "101:26"]
        pub static silk_Quantization_Offsets_Q10: [[i16; 2]; 2];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/NSQ.h:34"]
pub mod NSQ_h {
    #[inline]
    #[c2rust::src_loc = "35:1"]
    pub unsafe extern "C" fn silk_noise_shape_quantizer_short_prediction_c(
        buf32: *const i32,
        coef16: *const i16,
        order: libc::c_int,
    ) -> i32 {
        let mut out: i32 = 0;
        out = order >> 1 as libc::c_int;
        out = (out as libc::c_long
            + (*buf32.offset(0 as libc::c_int as isize) as libc::c_long
                * *coef16.offset(0 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(1 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(1 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(2 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(2 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(3 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(3 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(4 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(4 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(5 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(5 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(6 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(6 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(7 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(7 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(8 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(8 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = (out as libc::c_long
            + (*buf32.offset(-(9 as libc::c_int) as isize) as libc::c_long
                * *coef16.offset(9 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        if order == 16 as libc::c_int {
            out = (out as libc::c_long
                + (*buf32.offset(-(10 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(10 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            out = (out as libc::c_long
                + (*buf32.offset(-(11 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(11 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            out = (out as libc::c_long
                + (*buf32.offset(-(12 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(12 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            out = (out as libc::c_long
                + (*buf32.offset(-(13 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(13 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            out = (out as libc::c_long
                + (*buf32.offset(-(14 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(14 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            out = (out as libc::c_long
                + (*buf32.offset(-(15 as libc::c_int) as isize) as libc::c_long
                    * *coef16.offset(15 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
        }
        return out;
    }
    #[inline]
    #[c2rust::src_loc = "67:1"]
    pub unsafe extern "C" fn silk_NSQ_noise_shape_feedback_loop_c(
        data0: *const i32,
        data1: *mut i32,
        coef: *const i16,
        order: libc::c_int,
    ) -> i32 {
        let mut out: i32 = 0;
        let mut tmp1: i32 = 0;
        let mut tmp2: i32 = 0;
        let mut j: libc::c_int = 0;
        tmp2 = *data0.offset(0 as libc::c_int as isize);
        tmp1 = *data1.offset(0 as libc::c_int as isize);
        *data1.offset(0 as libc::c_int as isize) = tmp2;
        out = order >> 1 as libc::c_int;
        out = (out as libc::c_long
            + (tmp2 as libc::c_long * *coef.offset(0 as libc::c_int as isize) as i64
                >> 16 as libc::c_int)) as i32;
        j = 2 as libc::c_int;
        while j < order {
            tmp2 = *data1.offset((j - 1 as libc::c_int) as isize);
            *data1.offset((j - 1 as libc::c_int) as isize) = tmp1;
            out = (out as libc::c_long
                + (tmp1 as libc::c_long * *coef.offset((j - 1 as libc::c_int) as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            tmp1 = *data1.offset((j + 0 as libc::c_int) as isize);
            *data1.offset((j + 0 as libc::c_int) as isize) = tmp2;
            out = (out as libc::c_long
                + (tmp2 as libc::c_long * *coef.offset(j as isize) as i64 >> 16 as libc::c_int))
                as i32;
            j += 2 as libc::c_int;
        }
        *data1.offset((order - 1 as libc::c_int) as isize) = tmp1;
        out = (out as libc::c_long
            + (tmp1 as libc::c_long * *coef.offset((order - 1 as libc::c_int) as isize) as i64
                >> 16 as libc::c_int)) as i32;
        out = ((out as u32) << 1 as libc::c_int) as i32;
        return out;
    }
}

pub use self::define_h::{
    HARM_SHAPE_FIR_TAPS, LTP_ORDER, MAX_LPC_ORDER, MAX_SHAPE_LPC_ORDER, NSQ_LPC_BUF_LENGTH,
    TYPE_VOICED,
};
use self::tables_h::silk_Quantization_Offsets_Q10;
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
pub use self::NSQ_h::{
    silk_NSQ_noise_shape_feedback_loop_c, silk_noise_shape_quantizer_short_prediction_c,
};
use crate::celt::celt::celt_fatal;
use crate::externs::{memcpy, memmove};
use crate::silk::structs::{silk_encoder_state, silk_nsq_state, SideInfoIndices};
use crate::silk::Inlines::{silk_DIV32_varQ, silk_INVERSE32_varQ};
use crate::silk::LPC_analysis_filter::silk_LPC_analysis_filter;
#[no_mangle]
#[c2rust::src_loc = "76:1"]
pub unsafe extern "C" fn silk_NSQ_c(
    psEncC: *const silk_encoder_state,
    mut NSQ: *mut silk_nsq_state,
    psIndices: *mut SideInfoIndices,
    mut x16: *const i16,
    mut pulses: *mut i8,
    PredCoef_Q12: *const i16,
    LTPCoef_Q14: *const i16,
    AR_Q13: *const i16,
    HarmShapeGain_Q14: *const libc::c_int,
    Tilt_Q14: *const libc::c_int,
    LF_shp_Q14: *const i32,
    Gains_Q16: *const i32,
    pitchL: *const libc::c_int,
    Lambda_Q10: libc::c_int,
    LTP_scale_Q14: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut start_idx: libc::c_int = 0;
    let mut LSF_interpolation_flag: libc::c_int = 0;
    let mut A_Q12: *const i16 = 0 as *const i16;
    let mut B_Q14: *const i16 = 0 as *const i16;
    let mut AR_shp_Q13: *const i16 = 0 as *const i16;
    let mut pxq: *mut i16 = 0 as *mut i16;
    let mut HarmShapeFIRPacked_Q14: i32 = 0;
    let mut offset_Q10: libc::c_int = 0;
    (*NSQ).rand_seed = (*psIndices).Seed as i32;
    lag = (*NSQ).lagPrev;
    offset_Q10 = silk_Quantization_Offsets_Q10
        [((*psIndices).signalType as libc::c_int >> 1 as libc::c_int) as usize]
        [(*psIndices).quantOffsetType as usize] as libc::c_int;
    if (*psIndices).NLSFInterpCoef_Q2 as libc::c_int == 4 as libc::c_int {
        LSF_interpolation_flag = 0 as libc::c_int;
    } else {
        LSF_interpolation_flag = 1 as libc::c_int;
    }
    let vla = ((*psEncC).ltp_mem_length + (*psEncC).frame_length) as usize;
    let mut sLTP_Q15: Vec<i32> = ::std::vec::from_elem(0, vla);
    let vla_0 = ((*psEncC).ltp_mem_length + (*psEncC).frame_length) as usize;
    let mut sLTP: Vec<i16> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = (*psEncC).subfr_length as usize;
    let mut x_sc_Q10: Vec<i32> = ::std::vec::from_elem(0, vla_1);
    (*NSQ).sLTP_shp_buf_idx = (*psEncC).ltp_mem_length;
    (*NSQ).sLTP_buf_idx = (*psEncC).ltp_mem_length;
    pxq = &mut *((*NSQ).xq)
        .as_mut_ptr()
        .offset((*psEncC).ltp_mem_length as isize) as *mut i16;
    k = 0 as libc::c_int;
    while k < (*psEncC).nb_subfr {
        A_Q12 = &*PredCoef_Q12.offset(
            ((k >> 1 as libc::c_int | 1 as libc::c_int - LSF_interpolation_flag) * MAX_LPC_ORDER)
                as isize,
        ) as *const i16;
        B_Q14 = &*LTPCoef_Q14.offset((k * LTP_ORDER) as isize) as *const i16;
        AR_shp_Q13 = &*AR_Q13.offset((k * MAX_SHAPE_LPC_ORDER) as isize) as *const i16;
        HarmShapeFIRPacked_Q14 = *HarmShapeGain_Q14.offset(k as isize) >> 2 as libc::c_int;
        HarmShapeFIRPacked_Q14 |= (((*HarmShapeGain_Q14.offset(k as isize) >> 1 as libc::c_int)
            as u32)
            << 16 as libc::c_int) as i32;
        (*NSQ).rewhite_flag = 0 as libc::c_int;
        if (*psIndices).signalType as libc::c_int == TYPE_VOICED {
            lag = *pitchL.offset(k as isize);
            if k & 3 as libc::c_int - ((LSF_interpolation_flag as u32) << 1 as libc::c_int) as i32
                == 0 as libc::c_int
            {
                start_idx = (*psEncC).ltp_mem_length
                    - lag
                    - (*psEncC).predictLPCOrder
                    - LTP_ORDER / 2 as libc::c_int;
                if !(start_idx > 0 as libc::c_int) {
                    celt_fatal(
                        b"assertion failed: start_idx > 0\0" as *const u8 as *const libc::c_char,
                        b"silk/NSQ.c\0" as *const u8 as *const libc::c_char,
                        146 as libc::c_int,
                    );
                }
                silk_LPC_analysis_filter(
                    &mut *sLTP.as_mut_ptr().offset(start_idx as isize),
                    &mut *((*NSQ).xq)
                        .as_mut_ptr()
                        .offset((start_idx + k * (*psEncC).subfr_length) as isize),
                    A_Q12,
                    (*psEncC).ltp_mem_length - start_idx,
                    (*psEncC).predictLPCOrder,
                    (*psEncC).arch,
                );
                (*NSQ).rewhite_flag = 1 as libc::c_int;
                (*NSQ).sLTP_buf_idx = (*psEncC).ltp_mem_length;
            }
        }
        silk_nsq_scale_states(
            psEncC,
            NSQ,
            x16,
            x_sc_Q10.as_mut_ptr(),
            sLTP.as_mut_ptr() as *const i16,
            sLTP_Q15.as_mut_ptr(),
            k,
            LTP_scale_Q14,
            Gains_Q16,
            pitchL,
            (*psIndices).signalType as libc::c_int,
        );
        silk_noise_shape_quantizer(
            NSQ,
            (*psIndices).signalType as libc::c_int,
            x_sc_Q10.as_mut_ptr() as *const i32,
            pulses,
            pxq,
            sLTP_Q15.as_mut_ptr(),
            A_Q12,
            B_Q14,
            AR_shp_Q13,
            lag,
            HarmShapeFIRPacked_Q14,
            *Tilt_Q14.offset(k as isize),
            *LF_shp_Q14.offset(k as isize),
            *Gains_Q16.offset(k as isize),
            Lambda_Q10,
            offset_Q10,
            (*psEncC).subfr_length,
            (*psEncC).shapingLPCOrder,
            (*psEncC).predictLPCOrder,
            (*psEncC).arch,
        );
        x16 = x16.offset((*psEncC).subfr_length as isize);
        pulses = pulses.offset((*psEncC).subfr_length as isize);
        pxq = pxq.offset((*psEncC).subfr_length as isize);
        k += 1;
    }
    (*NSQ).lagPrev = *pitchL.offset(((*psEncC).nb_subfr - 1 as libc::c_int) as isize);
    memmove(
        ((*NSQ).xq).as_mut_ptr() as *mut libc::c_void,
        &mut *((*NSQ).xq)
            .as_mut_ptr()
            .offset((*psEncC).frame_length as isize) as *mut i16 as *const libc::c_void,
        ((*psEncC).ltp_mem_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    memmove(
        ((*NSQ).sLTP_shp_Q14).as_mut_ptr() as *mut libc::c_void,
        &mut *((*NSQ).sLTP_shp_Q14)
            .as_mut_ptr()
            .offset((*psEncC).frame_length as isize) as *mut i32 as *const libc::c_void,
        ((*psEncC).ltp_mem_length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
}
#[inline]
#[c2rust::src_loc = "181:1"]
unsafe extern "C" fn silk_noise_shape_quantizer(
    mut NSQ: *mut silk_nsq_state,
    signalType: libc::c_int,
    x_sc_Q10: *const i32,
    pulses: *mut i8,
    xq: *mut i16,
    sLTP_Q15: *mut i32,
    a_Q12: *const i16,
    b_Q14: *const i16,
    AR_shp_Q13: *const i16,
    lag: libc::c_int,
    HarmShapeFIRPacked_Q14: i32,
    Tilt_Q14: libc::c_int,
    LF_shp_Q14: i32,
    Gain_Q16: i32,
    Lambda_Q10: libc::c_int,
    offset_Q10: libc::c_int,
    length: libc::c_int,
    shapingLPCOrder: libc::c_int,
    predictLPCOrder: libc::c_int,
    _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut LTP_pred_Q13: i32 = 0;
    let mut LPC_pred_Q10: i32 = 0;
    let mut n_AR_Q12: i32 = 0;
    let mut n_LTP_Q13: i32 = 0;
    let mut n_LF_Q12: i32 = 0;
    let mut r_Q10: i32 = 0;
    let mut rr_Q10: i32 = 0;
    let mut q1_Q0: i32 = 0;
    let mut q1_Q10: i32 = 0;
    let mut q2_Q10: i32 = 0;
    let mut rd1_Q20: i32 = 0;
    let mut rd2_Q20: i32 = 0;
    let mut exc_Q14: i32 = 0;
    let mut LPC_exc_Q14: i32 = 0;
    let mut xq_Q14: i32 = 0;
    let mut Gain_Q10: i32 = 0;
    let mut tmp1: i32 = 0;
    let mut tmp2: i32 = 0;
    let mut sLF_AR_shp_Q14: i32 = 0;
    let mut psLPC_Q14: *mut i32 = 0 as *mut i32;
    let mut shp_lag_ptr: *mut i32 = 0 as *mut i32;
    let mut pred_lag_ptr: *mut i32 = 0 as *mut i32;
    shp_lag_ptr = &mut *((*NSQ).sLTP_shp_Q14)
        .as_mut_ptr()
        .offset(((*NSQ).sLTP_shp_buf_idx - lag + HARM_SHAPE_FIR_TAPS / 2 as libc::c_int) as isize)
        as *mut i32;
    pred_lag_ptr = &mut *sLTP_Q15
        .offset(((*NSQ).sLTP_buf_idx - lag + LTP_ORDER / 2 as libc::c_int) as isize)
        as *mut i32;
    Gain_Q10 = Gain_Q16 >> 6 as libc::c_int;
    psLPC_Q14 = &mut *((*NSQ).sLPC_Q14)
        .as_mut_ptr()
        .offset((NSQ_LPC_BUF_LENGTH - 1 as libc::c_int) as isize) as *mut i32;
    i = 0 as libc::c_int;
    while i < length {
        (*NSQ).rand_seed = (907633515 as libc::c_int as u32)
            .wrapping_add(((*NSQ).rand_seed as u32).wrapping_mul(196314165 as libc::c_int as u32))
            as i32;
        LPC_pred_Q10 =
            silk_noise_shape_quantizer_short_prediction_c(psLPC_Q14, a_Q12, predictLPCOrder);
        if signalType == TYPE_VOICED {
            LTP_pred_Q13 = 2 as libc::c_int;
            LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                + (*pred_lag_ptr.offset(0 as libc::c_int as isize) as libc::c_long
                    * *b_Q14.offset(0 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                + (*pred_lag_ptr.offset(-(1 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(1 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                + (*pred_lag_ptr.offset(-(2 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(2 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                + (*pred_lag_ptr.offset(-(3 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(3 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                + (*pred_lag_ptr.offset(-(4 as libc::c_int) as isize) as libc::c_long
                    * *b_Q14.offset(4 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            pred_lag_ptr = pred_lag_ptr.offset(1);
        } else {
            LTP_pred_Q13 = 0 as libc::c_int;
        }
        if !(shapingLPCOrder & 1 as libc::c_int == 0 as libc::c_int) {
            celt_fatal(
                b"assertion failed: ( shapingLPCOrder & 1 ) == 0\0" as *const u8
                    as *const libc::c_char,
                b"silk/NSQ.c\0" as *const u8 as *const libc::c_char,
                250 as libc::c_int,
            );
        }
        n_AR_Q12 = silk_NSQ_noise_shape_feedback_loop_c(
            &mut (*NSQ).sDiff_shp_Q14,
            ((*NSQ).sAR2_Q14).as_mut_ptr(),
            AR_shp_Q13,
            shapingLPCOrder,
        );
        n_AR_Q12 = (n_AR_Q12 as libc::c_long
            + ((*NSQ).sLF_AR_shp_Q14 as libc::c_long * Tilt_Q14 as i16 as i64 >> 16 as libc::c_int))
            as i32;
        n_LF_Q12 = ((*NSQ).sLTP_shp_Q14[((*NSQ).sLTP_shp_buf_idx - 1 as libc::c_int) as usize]
            as libc::c_long
            * LF_shp_Q14 as i16 as i64
            >> 16 as libc::c_int) as i32;
        n_LF_Q12 = (n_LF_Q12 as libc::c_long
            + ((*NSQ).sLF_AR_shp_Q14 as libc::c_long * (LF_shp_Q14 as i64 >> 16 as libc::c_int)
                >> 16 as libc::c_int)) as i32;
        if !(lag > 0 as libc::c_int || signalType != 2 as libc::c_int) {
            celt_fatal(
                b"assertion failed: lag > 0 || signalType != TYPE_VOICED\0" as *const u8
                    as *const libc::c_char,
                b"silk/NSQ.c\0" as *const u8 as *const libc::c_char,
                258 as libc::c_int,
            );
        }
        tmp1 = ((LPC_pred_Q10 as u32) << 2 as libc::c_int) as i32 - n_AR_Q12;
        tmp1 = tmp1 - n_LF_Q12;
        if lag > 0 as libc::c_int {
            n_LTP_Q13 = ((*shp_lag_ptr.offset(0 as libc::c_int as isize)
                + *shp_lag_ptr.offset(-(2 as libc::c_int) as isize))
                as libc::c_long
                * HarmShapeFIRPacked_Q14 as i16 as i64
                >> 16 as libc::c_int) as i32;
            n_LTP_Q13 = (n_LTP_Q13 as libc::c_long
                + (*shp_lag_ptr.offset(-(1 as libc::c_int) as isize) as libc::c_long
                    * (HarmShapeFIRPacked_Q14 as i64 >> 16 as libc::c_int)
                    >> 16 as libc::c_int)) as i32;
            n_LTP_Q13 = ((n_LTP_Q13 as u32) << 1 as libc::c_int) as i32;
            shp_lag_ptr = shp_lag_ptr.offset(1);
            tmp2 = LTP_pred_Q13 - n_LTP_Q13;
            tmp1 = tmp2 + ((tmp1 as u32) << 1 as libc::c_int) as i32;
            tmp1 = if 3 as libc::c_int == 1 as libc::c_int {
                (tmp1 >> 1 as libc::c_int) + (tmp1 & 1 as libc::c_int)
            } else {
                (tmp1 >> 3 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
            };
        } else {
            tmp1 = if 2 as libc::c_int == 1 as libc::c_int {
                (tmp1 >> 1 as libc::c_int) + (tmp1 & 1 as libc::c_int)
            } else {
                (tmp1 >> 2 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
            };
        }
        r_Q10 = *x_sc_Q10.offset(i as isize) - tmp1;
        if (*NSQ).rand_seed < 0 as libc::c_int {
            r_Q10 = -r_Q10;
        }
        r_Q10 = if -((31 as libc::c_int) << 10 as libc::c_int)
            > (30 as libc::c_int) << 10 as libc::c_int
        {
            if r_Q10 > -((31 as libc::c_int) << 10 as libc::c_int) {
                -((31 as libc::c_int) << 10 as libc::c_int)
            } else if r_Q10 < (30 as libc::c_int) << 10 as libc::c_int {
                (30 as libc::c_int) << 10 as libc::c_int
            } else {
                r_Q10
            }
        } else if r_Q10 > (30 as libc::c_int) << 10 as libc::c_int {
            (30 as libc::c_int) << 10 as libc::c_int
        } else if r_Q10 < -((31 as libc::c_int) << 10 as libc::c_int) {
            -((31 as libc::c_int) << 10 as libc::c_int)
        } else {
            r_Q10
        };
        q1_Q10 = r_Q10 - offset_Q10;
        q1_Q0 = q1_Q10 >> 10 as libc::c_int;
        if Lambda_Q10 > 2048 as libc::c_int {
            let rdo_offset: libc::c_int = Lambda_Q10 / 2 as libc::c_int - 512 as libc::c_int;
            if q1_Q10 > rdo_offset {
                q1_Q0 = q1_Q10 - rdo_offset >> 10 as libc::c_int;
            } else if q1_Q10 < -rdo_offset {
                q1_Q0 = q1_Q10 + rdo_offset >> 10 as libc::c_int;
            } else if q1_Q10 < 0 as libc::c_int {
                q1_Q0 = -(1 as libc::c_int);
            } else {
                q1_Q0 = 0 as libc::c_int;
            }
        }
        if q1_Q0 > 0 as libc::c_int {
            q1_Q10 = ((q1_Q0 as u32) << 10 as libc::c_int) as i32 - 80 as libc::c_int;
            q1_Q10 = q1_Q10 + offset_Q10;
            q2_Q10 = q1_Q10 + 1024 as libc::c_int;
            rd1_Q20 = q1_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
            rd2_Q20 = q2_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
        } else if q1_Q0 == 0 as libc::c_int {
            q1_Q10 = offset_Q10;
            q2_Q10 = q1_Q10 + (1024 as libc::c_int - 80 as libc::c_int);
            rd1_Q20 = q1_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
            rd2_Q20 = q2_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
        } else if q1_Q0 == -(1 as libc::c_int) {
            q2_Q10 = offset_Q10;
            q1_Q10 = q2_Q10 - (1024 as libc::c_int - 80 as libc::c_int);
            rd1_Q20 = -q1_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
            rd2_Q20 = q2_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
        } else {
            q1_Q10 = ((q1_Q0 as u32) << 10 as libc::c_int) as i32 + 80 as libc::c_int;
            q1_Q10 = q1_Q10 + offset_Q10;
            q2_Q10 = q1_Q10 + 1024 as libc::c_int;
            rd1_Q20 = -q1_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
            rd2_Q20 = -q2_Q10 as i16 as i32 * Lambda_Q10 as i16 as i32;
        }
        rr_Q10 = r_Q10 - q1_Q10;
        rd1_Q20 = rd1_Q20 + rr_Q10 as i16 as i32 * rr_Q10 as i16 as i32;
        rr_Q10 = r_Q10 - q2_Q10;
        rd2_Q20 = rd2_Q20 + rr_Q10 as i16 as i32 * rr_Q10 as i16 as i32;
        if rd2_Q20 < rd1_Q20 {
            q1_Q10 = q2_Q10;
        }
        *pulses.offset(i as isize) = (if 10 as libc::c_int == 1 as libc::c_int {
            (q1_Q10 >> 1 as libc::c_int) + (q1_Q10 & 1 as libc::c_int)
        } else {
            (q1_Q10 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i8;
        exc_Q14 = ((q1_Q10 as u32) << 4 as libc::c_int) as i32;
        if (*NSQ).rand_seed < 0 as libc::c_int {
            exc_Q14 = -exc_Q14;
        }
        LPC_exc_Q14 = exc_Q14 + ((LTP_pred_Q13 as u32) << 1 as libc::c_int) as i32;
        xq_Q14 = LPC_exc_Q14 + ((LPC_pred_Q10 as u32) << 4 as libc::c_int) as i32;
        *xq.offset(i as isize) = (if (if 8 as libc::c_int == 1 as libc::c_int {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 8 as libc::c_int == 1 as libc::c_int {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            ((xq_Q14 as i64 * Gain_Q10 as libc::c_long >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as i16;
        psLPC_Q14 = psLPC_Q14.offset(1);
        *psLPC_Q14 = xq_Q14;
        (*NSQ).sDiff_shp_Q14 =
            xq_Q14 - ((*x_sc_Q10.offset(i as isize) as u32) << 4 as libc::c_int) as i32;
        sLF_AR_shp_Q14 = (*NSQ).sDiff_shp_Q14 - ((n_AR_Q12 as u32) << 2 as libc::c_int) as i32;
        (*NSQ).sLF_AR_shp_Q14 = sLF_AR_shp_Q14;
        (*NSQ).sLTP_shp_Q14[(*NSQ).sLTP_shp_buf_idx as usize] =
            sLF_AR_shp_Q14 - ((n_LF_Q12 as u32) << 2 as libc::c_int) as i32;
        *sLTP_Q15.offset((*NSQ).sLTP_buf_idx as isize) =
            ((LPC_exc_Q14 as u32) << 1 as libc::c_int) as i32;
        (*NSQ).sLTP_shp_buf_idx += 1;
        (*NSQ).sLTP_buf_idx += 1;
        (*NSQ).rand_seed =
            ((*NSQ).rand_seed as u32).wrapping_add(*pulses.offset(i as isize) as u32) as i32;
        i += 1;
    }
    memcpy(
        ((*NSQ).sLPC_Q14).as_mut_ptr() as *mut libc::c_void,
        &mut *((*NSQ).sLPC_Q14).as_mut_ptr().offset(length as isize) as *mut i32
            as *const libc::c_void,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
}
#[inline]
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn silk_nsq_scale_states(
    psEncC: *const silk_encoder_state,
    mut NSQ: *mut silk_nsq_state,
    x16: *const i16,
    x_sc_Q10: *mut i32,
    sLTP: *const i16,
    sLTP_Q15: *mut i32,
    subfr: libc::c_int,
    LTP_scale_Q14: libc::c_int,
    Gains_Q16: *const i32,
    pitchL: *const libc::c_int,
    signal_type: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut gain_adj_Q16: i32 = 0;
    let mut inv_gain_Q31: i32 = 0;
    let mut inv_gain_Q26: i32 = 0;
    lag = *pitchL.offset(subfr as isize);
    inv_gain_Q31 = silk_INVERSE32_varQ(
        if *Gains_Q16.offset(subfr as isize) > 1 as libc::c_int {
            *Gains_Q16.offset(subfr as isize)
        } else {
            1 as libc::c_int
        },
        47 as libc::c_int,
    );
    inv_gain_Q26 = if 5 as libc::c_int == 1 as libc::c_int {
        (inv_gain_Q31 >> 1 as libc::c_int) + (inv_gain_Q31 & 1 as libc::c_int)
    } else {
        (inv_gain_Q31 >> 5 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
    };
    i = 0 as libc::c_int;
    while i < (*psEncC).subfr_length {
        *x_sc_Q10.offset(i as isize) = (*x16.offset(i as isize) as i64
            * inv_gain_Q26 as libc::c_long
            >> 16 as libc::c_int) as i32;
        i += 1;
    }
    if (*NSQ).rewhite_flag != 0 {
        if subfr == 0 as libc::c_int {
            inv_gain_Q31 = (((inv_gain_Q31 as libc::c_long * LTP_scale_Q14 as i16 as i64
                >> 16 as libc::c_int) as i32 as u32)
                << 2 as libc::c_int) as i32;
        }
        i = (*NSQ).sLTP_buf_idx - lag - LTP_ORDER / 2 as libc::c_int;
        while i < (*NSQ).sLTP_buf_idx {
            *sLTP_Q15.offset(i as isize) = (inv_gain_Q31 as libc::c_long
                * *sLTP.offset(i as isize) as i64
                >> 16 as libc::c_int) as i32;
            i += 1;
        }
    }
    if *Gains_Q16.offset(subfr as isize) != (*NSQ).prev_gain_Q16 {
        gain_adj_Q16 = silk_DIV32_varQ(
            (*NSQ).prev_gain_Q16,
            *Gains_Q16.offset(subfr as isize),
            16 as libc::c_int,
        );
        i = (*NSQ).sLTP_shp_buf_idx - (*psEncC).ltp_mem_length;
        while i < (*NSQ).sLTP_shp_buf_idx {
            (*NSQ).sLTP_shp_Q14[i as usize] = (gain_adj_Q16 as i64
                * (*NSQ).sLTP_shp_Q14[i as usize] as libc::c_long
                >> 16 as libc::c_int) as i32;
            i += 1;
        }
        if signal_type == TYPE_VOICED && (*NSQ).rewhite_flag == 0 as libc::c_int {
            i = (*NSQ).sLTP_buf_idx - lag - LTP_ORDER / 2 as libc::c_int;
            while i < (*NSQ).sLTP_buf_idx {
                *sLTP_Q15.offset(i as isize) = (gain_adj_Q16 as i64
                    * *sLTP_Q15.offset(i as isize) as libc::c_long
                    >> 16 as libc::c_int) as i32;
                i += 1;
            }
        }
        (*NSQ).sLF_AR_shp_Q14 = (gain_adj_Q16 as i64 * (*NSQ).sLF_AR_shp_Q14 as libc::c_long
            >> 16 as libc::c_int) as i32;
        (*NSQ).sDiff_shp_Q14 = (gain_adj_Q16 as i64 * (*NSQ).sDiff_shp_Q14 as libc::c_long
            >> 16 as libc::c_int) as i32;
        i = 0 as libc::c_int;
        while i < NSQ_LPC_BUF_LENGTH {
            (*NSQ).sLPC_Q14[i as usize] = (gain_adj_Q16 as i64
                * (*NSQ).sLPC_Q14[i as usize] as libc::c_long
                >> 16 as libc::c_int) as i32;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < MAX_SHAPE_LPC_ORDER {
            (*NSQ).sAR2_Q14[i as usize] = (gain_adj_Q16 as i64
                * (*NSQ).sAR2_Q14[i as usize] as libc::c_long
                >> 16 as libc::c_int) as i32;
            i += 1;
        }
        (*NSQ).prev_gain_Q16 = *Gains_Q16.offset(subfr as isize);
    }
}
