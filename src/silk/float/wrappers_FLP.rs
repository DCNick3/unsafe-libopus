use crate::silk::A2NLSF::silk_A2NLSF;
use crate::silk::NLSF2A::silk_NLSF2A;
use ::libc;

#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/xmmintrin.h:32"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:32"]
pub mod SigProc_FLP_h {
    #[inline]
    #[c2rust::src_loc = "156:1"]
    pub unsafe extern "C" fn silk_float2int(x: libc::c_float) -> i32 {
        return float2int(x);
    }
    use crate::celt::float_cast::float2int;
}

pub use self::SigProc_FLP_h::silk_float2int;
use crate::silk::define::{LTP_ORDER, MAX_SHAPE_LPC_ORDER, TYPE_VOICED};
use crate::silk::float::structs_FLP::{silk_encoder_control_FLP, silk_encoder_state_FLP};
use crate::silk::process_NLSFs::silk_process_NLSFs;
use crate::silk::quant_LTP_gains::silk_quant_LTP_gains;
use crate::silk::structs::{silk_encoder_state, silk_nsq_state, SideInfoIndices};
use crate::silk::tables_other::silk_LTPScales_table_Q14;
use crate::silk::NSQ_del_dec::silk_NSQ_del_dec_c;
use crate::silk::NSQ::silk_NSQ_c;

#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn silk_A2NLSF_FLP(
    NLSF_Q15: *mut i16,
    pAR: *const libc::c_float,
    LPC_order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut a_fix_Q16: [i32; 16] = [0; 16];
    i = 0 as libc::c_int;
    while i < LPC_order {
        a_fix_Q16[i as usize] = silk_float2int(*pAR.offset(i as isize) * 65536.0f32);
        i += 1;
    }
    silk_A2NLSF(NLSF_Q15, a_fix_Q16.as_mut_ptr(), LPC_order);
}
#[c2rust::src_loc = "54:1"]
pub unsafe extern "C" fn silk_NLSF2A_FLP(
    pAR: *mut libc::c_float,
    NLSF_Q15: *const i16,
    LPC_order: libc::c_int,
    arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut a_fix_Q12: [i16; 16] = [0; 16];
    silk_NLSF2A(a_fix_Q12.as_mut_ptr(), NLSF_Q15, LPC_order, arch);
    i = 0 as libc::c_int;
    while i < LPC_order {
        *pAR.offset(i as isize) = a_fix_Q12[i as usize] as libc::c_float * (1.0f32 / 4096.0f32);
        i += 1;
    }
}
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn silk_process_NLSFs_FLP(
    psEncC: *mut silk_encoder_state,
    PredCoef: *mut [libc::c_float; 16],
    NLSF_Q15: *mut i16,
    prev_NLSF_Q15: *const i16,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut PredCoef_Q12: [[i16; 16]; 2] = [[0; 16]; 2];
    silk_process_NLSFs(psEncC, PredCoef_Q12.as_mut_ptr(), NLSF_Q15, prev_NLSF_Q15);
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*psEncC).predictLPCOrder {
            (*PredCoef.offset(j as isize))[i as usize] =
                PredCoef_Q12[j as usize][i as usize] as libc::c_float * (1.0f32 / 4096.0f32);
            i += 1;
        }
        j += 1;
    }
}
#[c2rust::src_loc = "96:1"]
pub unsafe extern "C" fn silk_NSQ_wrapper_FLP(
    psEnc: *mut silk_encoder_state_FLP,
    psEncCtrl: *mut silk_encoder_control_FLP,
    psIndices: *mut SideInfoIndices,
    psNSQ: *mut silk_nsq_state,
    pulses: *mut i8,
    x: *const libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x16: [i16; 320] = [0; 320];
    let mut Gains_Q16: [i32; 4] = [0; 4];
    let mut PredCoef_Q12: [[i16; 16]; 2] = [[0; 16]; 2];
    let mut LTPCoef_Q14: [i16; 20] = [0; 20];
    let mut LTP_scale_Q14: libc::c_int = 0;
    let mut AR_Q13: [i16; 96] = [0; 96];
    let mut LF_shp_Q14: [i32; 4] = [0; 4];
    let mut Lambda_Q10: libc::c_int = 0;
    let mut Tilt_Q14: [libc::c_int; 4] = [0; 4];
    let mut HarmShapeGain_Q14: [libc::c_int; 4] = [0; 4];
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.nb_subfr {
        j = 0 as libc::c_int;
        while j < (*psEnc).sCmn.shapingLPCOrder {
            AR_Q13[(i * MAX_SHAPE_LPC_ORDER + j) as usize] =
                silk_float2int((*psEncCtrl).AR[(i * MAX_SHAPE_LPC_ORDER + j) as usize] * 8192.0f32)
                    as i16;
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.nb_subfr {
        LF_shp_Q14[i as usize] = ((silk_float2int((*psEncCtrl).LF_AR_shp[i as usize] * 16384.0f32)
            as u32)
            << 16 as libc::c_int) as i32
            | silk_float2int((*psEncCtrl).LF_MA_shp[i as usize] * 16384.0f32) as u16 as libc::c_int;
        Tilt_Q14[i as usize] = silk_float2int((*psEncCtrl).Tilt[i as usize] * 16384.0f32);
        HarmShapeGain_Q14[i as usize] =
            silk_float2int((*psEncCtrl).HarmShapeGain[i as usize] * 16384.0f32);
        i += 1;
    }
    Lambda_Q10 = silk_float2int((*psEncCtrl).Lambda * 1024.0f32);
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.nb_subfr * LTP_ORDER {
        LTPCoef_Q14[i as usize] =
            silk_float2int((*psEncCtrl).LTPCoef[i as usize] * 16384.0f32) as i16;
        i += 1;
    }
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*psEnc).sCmn.predictLPCOrder {
            PredCoef_Q12[j as usize][i as usize] =
                silk_float2int((*psEncCtrl).PredCoef[j as usize][i as usize] * 4096.0f32) as i16;
            i += 1;
        }
        j += 1;
    }
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.nb_subfr {
        Gains_Q16[i as usize] = silk_float2int((*psEncCtrl).Gains[i as usize] * 65536.0f32);
        i += 1;
    }
    if (*psIndices).signalType as libc::c_int == TYPE_VOICED {
        LTP_scale_Q14 =
            silk_LTPScales_table_Q14[(*psIndices).LTP_scaleIndex as usize] as libc::c_int;
    } else {
        LTP_scale_Q14 = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*psEnc).sCmn.frame_length {
        x16[i as usize] = silk_float2int(*x.offset(i as isize)) as i16;
        i += 1;
    }
    if (*psEnc).sCmn.nStatesDelayedDecision > 1 as libc::c_int
        || (*psEnc).sCmn.warping_Q16 > 0 as libc::c_int
    {
        silk_NSQ_del_dec_c(
            &mut (*psEnc).sCmn,
            psNSQ,
            psIndices,
            x16.as_mut_ptr() as *const i16,
            pulses,
            (PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr() as *const i16,
            LTPCoef_Q14.as_mut_ptr() as *const i16,
            AR_Q13.as_mut_ptr() as *const i16,
            HarmShapeGain_Q14.as_mut_ptr() as *const libc::c_int,
            Tilt_Q14.as_mut_ptr() as *const libc::c_int,
            LF_shp_Q14.as_mut_ptr() as *const i32,
            Gains_Q16.as_mut_ptr() as *const i32,
            ((*psEncCtrl).pitchL).as_mut_ptr() as *const libc::c_int,
            Lambda_Q10,
            LTP_scale_Q14,
        );
    } else {
        silk_NSQ_c(
            &mut (*psEnc).sCmn,
            psNSQ,
            psIndices,
            x16.as_mut_ptr() as *const i16,
            pulses,
            (PredCoef_Q12[0 as libc::c_int as usize]).as_mut_ptr() as *const i16,
            LTPCoef_Q14.as_mut_ptr() as *const i16,
            AR_Q13.as_mut_ptr() as *const i16,
            HarmShapeGain_Q14.as_mut_ptr() as *const libc::c_int,
            Tilt_Q14.as_mut_ptr() as *const libc::c_int,
            LF_shp_Q14.as_mut_ptr() as *const i32,
            Gains_Q16.as_mut_ptr() as *const i32,
            ((*psEncCtrl).pitchL).as_mut_ptr() as *const libc::c_int,
            Lambda_Q10,
            LTP_scale_Q14,
        );
    };
}
#[c2rust::src_loc = "175:1"]
pub unsafe extern "C" fn silk_quant_LTP_gains_FLP(
    B: *mut libc::c_float,
    cbk_index: *mut i8,
    periodicity_index: *mut i8,
    sum_log_gain_Q7: *mut i32,
    pred_gain_dB: *mut libc::c_float,
    XX: *const libc::c_float,
    xX: *const libc::c_float,
    subfr_len: libc::c_int,
    nb_subfr: libc::c_int,
    arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut pred_gain_dB_Q7: libc::c_int = 0;
    let mut B_Q14: [i16; 20] = [0; 20];
    let mut XX_Q17: [i32; 100] = [0; 100];
    let mut xX_Q17: [i32; 20] = [0; 20];
    i = 0 as libc::c_int;
    while i < nb_subfr * LTP_ORDER * LTP_ORDER {
        XX_Q17[i as usize] = silk_float2int(*XX.offset(i as isize) * 131072.0f32);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nb_subfr * LTP_ORDER {
        xX_Q17[i as usize] = silk_float2int(*xX.offset(i as isize) * 131072.0f32);
        i += 1;
    }
    silk_quant_LTP_gains(
        B_Q14.as_mut_ptr(),
        cbk_index,
        periodicity_index,
        sum_log_gain_Q7,
        &mut pred_gain_dB_Q7,
        XX_Q17.as_mut_ptr() as *const i32,
        xX_Q17.as_mut_ptr() as *const i32,
        subfr_len,
        nb_subfr,
        arch,
    );
    i = 0 as libc::c_int;
    while i < nb_subfr * LTP_ORDER {
        *B.offset(i as isize) = B_Q14[i as usize] as libc::c_float * (1.0f32 / 16384.0f32);
        i += 1;
    }
    *pred_gain_dB = pred_gain_dB_Q7 as libc::c_float * (1.0f32 / 128.0f32);
}
