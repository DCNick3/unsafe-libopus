use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:32"]
pub mod resampler_structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [i32; 6],
        pub sFIR: C2RustUnnamed,
        pub delayBuf: [i16; 48],
        pub resampler_function: libc::c_int,
        pub batchSize: libc::c_int,
        pub invRatio_Q16: i32,
        pub FIR_Order: libc::c_int,
        pub FIR_Fracs: libc::c_int,
        pub Fs_in_kHz: libc::c_int,
        pub Fs_out_kHz: libc::c_int,
        pub inputDelay: libc::c_int,
        pub Coefs: *const i16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:5"]
    pub union C2RustUnnamed {
        pub i32_0: [i32; 36],
        pub i16_0: [i16; 36],
    }
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:9"]
    pub struct silk_NLSF_CB_struct {
        pub nVectors: i16,
        pub order: i16,
        pub quantStepSize_Q16: i16,
        pub invQuantStepSize_Q6: i16,
        pub CB1_NLSF_Q8: *const u8,
        pub CB1_Wght_Q9: *const i16,
        pub CB1_iCDF: *const u8,
        pub pred_Q8: *const u8,
        pub ec_sel: *const u8,
        pub ec_iCDF: *const u8,
        pub ec_Rates_Q5: *const u8,
        pub deltaMin_Q15: *const i16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:9"]
    pub struct SideInfoIndices {
        pub GainsIndices: [i8; 4],
        pub LTPIndex: [i8; 4],
        pub NLSFIndices: [i8; 17],
        pub lagIndex: i16,
        pub contourIndex: i8,
        pub signalType: i8,
        pub quantOffsetType: i8,
        pub NLSFInterpCoef_Q2: i8,
        pub PERIndex: i8,
        pub LTP_scaleIndex: i8,
        pub Seed: i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "232:9"]
    pub struct silk_PLC_struct {
        pub pitchL_Q8: i32,
        pub LTPCoef_Q14: [i16; 5],
        pub prevLPC_Q12: [i16; 16],
        pub last_frame_lost: libc::c_int,
        pub rand_seed: i32,
        pub randScale_Q14: i16,
        pub conc_energy: i32,
        pub conc_energy_shift: libc::c_int,
        pub prevLTP_scale_Q14: i16,
        pub prevGain_Q16: [i32; 2],
        pub fs_kHz: libc::c_int,
        pub nb_subfr: libc::c_int,
        pub subfr_length: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "249:9"]
    pub struct silk_CNG_struct {
        pub CNG_exc_buf_Q14: [i32; 320],
        pub CNG_smth_NLSF_Q15: [i16; 16],
        pub CNG_synth_state: [i32; 16],
        pub CNG_smth_Gain_Q16: i32,
        pub rand_seed: i32,
        pub fs_kHz: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "261:9"]
    pub struct silk_decoder_state {
        pub prev_gain_Q16: i32,
        pub exc_Q14: [i32; 320],
        pub sLPC_Q14_buf: [i32; 16],
        pub outBuf: [i16; 480],
        pub lagPrev: libc::c_int,
        pub LastGainIndex: i8,
        pub fs_kHz: libc::c_int,
        pub fs_API_hz: i32,
        pub nb_subfr: libc::c_int,
        pub frame_length: libc::c_int,
        pub subfr_length: libc::c_int,
        pub ltp_mem_length: libc::c_int,
        pub LPC_order: libc::c_int,
        pub prevNLSF_Q15: [i16; 16],
        pub first_frame_after_reset: libc::c_int,
        pub pitch_lag_low_bits_iCDF: *const u8,
        pub pitch_contour_iCDF: *const u8,
        pub nFramesDecoded: libc::c_int,
        pub nFramesPerPacket: libc::c_int,
        pub ec_prevSignalType: libc::c_int,
        pub ec_prevLagIndex: i16,
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "314:9"]
    pub struct silk_decoder_control {
        pub pitchL: [libc::c_int; 4],
        pub Gains_Q16: [i32; 4],
        pub PredCoef_Q12: [[i16; 16]; 2],
        pub LTPCoef_Q14: [i16; 20],
        pub LTP_scale_Q14: libc::c_int,
    }
    use super::resampler_structs_h::silk_resampler_state_struct;
}

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/PLC.h:32"]
pub mod PLC_h {
    #[c2rust::src_loc = "38:9"]
    pub const RAND_BUF_MASK: libc::c_int = RAND_BUF_SIZE - 1 as libc::c_int;
    #[c2rust::src_loc = "37:9"]
    pub const RAND_BUF_SIZE: libc::c_int = 128 as libc::c_int;
    #[c2rust::src_loc = "34:9"]
    pub const V_PITCH_GAIN_START_MIN_Q14: libc::c_int = 11469 as libc::c_int;
    #[c2rust::src_loc = "35:9"]
    pub const V_PITCH_GAIN_START_MAX_Q14: libc::c_int = 15565 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "70:9"]
    pub const TYPE_NO_VOICE_ACTIVITY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "142:9"]
    pub const MAX_LPC_ORDER: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "90:9"]
    pub const MAX_NB_SUBFR: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "146:9"]
    pub const LTP_ORDER: libc::c_int = 5 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "43:9"]
    pub const silk_int32_MIN: libc::c_uint = 0x80000000 as libc::c_uint;
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:32"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:32"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int =
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/macros.h:32"]
pub mod macros_h {
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn silk_CLZ32(in32: i32) -> i32 {
        return if in32 != 0 {
            32 as libc::c_int - (EC_CLZ0 - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }
    use super::ecintrin_h::EC_CLZ0;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "398:1"]
    pub unsafe extern "C" fn silk_ROR32(a32: i32, rot: libc::c_int) -> i32 {
        let x: u32 = a32 as u32;
        let r: u32 = rot as u32;
        let m: u32 = -rot as u32;
        if rot == 0 as libc::c_int {
            return a32;
        } else if rot < 0 as libc::c_int {
            return (x << m | x >> (32 as libc::c_int as libc::c_uint).wrapping_sub(m)) as i32;
        } else {
            return (x << (32 as libc::c_int as libc::c_uint).wrapping_sub(r) | x >> r) as i32;
        };
    }
    #[inline]
    #[c2rust::src_loc = "546:1"]
    pub unsafe extern "C" fn silk_min_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
        return if a < b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "554:1"]
    pub unsafe extern "C" fn silk_min_32(a: i32, b: i32) -> i32 {
        return if a < b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "564:1"]
    pub unsafe extern "C" fn silk_max_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
        return if a > b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "568:1"]
    pub unsafe extern "C" fn silk_max_16(a: i16, b: i16) -> i16 {
        return (if a as libc::c_int > b as libc::c_int {
            a as libc::c_int
        } else {
            b as libc::c_int
        }) as i16;
    }
    #[inline]
    #[c2rust::src_loc = "572:1"]
    pub unsafe extern "C" fn silk_max_32(a: i32, b: i32) -> i32 {
        return if a > b { a } else { b };
    }
    extern "C" {
        #[c2rust::src_loc = "123:1"]
        pub fn silk_LPC_analysis_filter(
            out: *mut i16,
            in_0: *const i16,
            B: *const i16,
            len: i32,
            d: i32,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "133:1"]
        pub fn silk_bwexpander(ar: *mut i16, d: libc::c_int, chirp_Q16: i32);
        #[c2rust::src_loc = "148:1"]
        pub fn silk_LPC_inverse_pred_gain_c(A_Q12: *const i16, order: libc::c_int) -> i32;
        #[c2rust::src_loc = "193:1"]
        pub fn silk_sum_sqr_shift(
            energy: *mut i32,
            shift: *mut libc::c_int,
            x: *const i16,
            len: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/Inlines.h:32"]
pub mod Inlines_h {
    #[inline]
    #[c2rust::src_loc = "56:1"]
    pub unsafe extern "C" fn silk_CLZ_FRAC(in_0: i32, lz: *mut i32, frac_Q7: *mut i32) {
        let lzeros: i32 = silk_CLZ32(in_0);
        *lz = lzeros;
        *frac_Q7 = silk_ROR32(in_0, 24 as libc::c_int - lzeros) & 0x7f as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "71:1"]
    pub unsafe extern "C" fn silk_SQRT_APPROX(x: i32) -> i32 {
        let mut y: i32 = 0;
        let mut lz: i32 = 0;
        let mut frac_Q7: i32 = 0;
        if x <= 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        silk_CLZ_FRAC(x, &mut lz, &mut frac_Q7);
        if lz & 1 as libc::c_int != 0 {
            y = 32768 as libc::c_int;
        } else {
            y = 46214 as libc::c_int;
        }
        y >>= lz >> 1 as libc::c_int;
        y = (y as libc::c_long
            + (y as libc::c_long
                * (213 as libc::c_int as i16 as i32 * frac_Q7 as i16 as i32) as i16 as i64
                >> 16 as libc::c_int)) as i32;
        return y;
    }
    #[inline]
    #[c2rust::src_loc = "143:1"]
    pub unsafe extern "C" fn silk_INVERSE32_varQ(b32: i32, Qres: libc::c_int) -> i32 {
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: i32 = 0;
        let mut b32_nrm: i32 = 0;
        let mut err_Q32: i32 = 0;
        let mut result: i32 = 0;
        b_headrm = silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 }) - 1 as libc::c_int;
        b32_nrm = ((b32 as u32) << b_headrm) as i32;
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int) / (b32_nrm >> 16 as libc::c_int);
        result = ((b32_inv as u32) << 16 as libc::c_int) as i32;
        err_Q32 = (((((1 as libc::c_int) << 29 as libc::c_int)
            - (b32_nrm as libc::c_long * b32_inv as i16 as i64 >> 16 as libc::c_int) as i32)
            as u32)
            << 3 as libc::c_int) as i32;
        result = (result as libc::c_long
            + (err_Q32 as i64 * b32_inv as libc::c_long >> 16 as libc::c_int))
            as i32;
        lshift = 61 as libc::c_int - b_headrm - Qres;
        if lshift <= 0 as libc::c_int {
            return (((if 0x80000000 as libc::c_uint as i32 >> -lshift
                > 0x7fffffff as libc::c_int >> -lshift
            {
                if result > 0x80000000 as libc::c_uint as i32 >> -lshift {
                    0x80000000 as libc::c_uint as i32 >> -lshift
                } else {
                    if result < 0x7fffffff as libc::c_int >> -lshift {
                        0x7fffffff as libc::c_int >> -lshift
                    } else {
                        result
                    }
                }
            } else {
                if result > 0x7fffffff as libc::c_int >> -lshift {
                    0x7fffffff as libc::c_int >> -lshift
                } else {
                    if result < 0x80000000 as libc::c_uint as i32 >> -lshift {
                        0x80000000 as libc::c_uint as i32 >> -lshift
                    } else {
                        result
                    }
                }
            }) as u32)
                << -lshift) as i32;
        } else if lshift < 32 as libc::c_int {
            return result >> lshift;
        } else {
            return 0 as libc::c_int;
        };
    }
    use super::macros_h::silk_CLZ32;
    use super::SigProc_FIX_h::silk_ROR32;
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
pub use self::define_h::{
    LTP_ORDER, MAX_LPC_ORDER, MAX_NB_SUBFR, TYPE_NO_VOICE_ACTIVITY, TYPE_VOICED,
};
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
pub use self::macros_h::silk_CLZ32;
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
use crate::celt::celt::celt_fatal;

pub use self::structs_h::{
    silk_CNG_struct, silk_NLSF_CB_struct, silk_PLC_struct, silk_decoder_control,
    silk_decoder_state, SideInfoIndices,
};
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN, silk_int32_MAX, silk_int32_MIN};

pub use self::Inlines_h::{silk_CLZ_FRAC, silk_INVERSE32_varQ, silk_SQRT_APPROX};
pub use self::PLC_h::{
    RAND_BUF_MASK, RAND_BUF_SIZE, V_PITCH_GAIN_START_MAX_Q14, V_PITCH_GAIN_START_MIN_Q14,
};
pub use self::SigProc_FIX_h::{
    silk_LPC_analysis_filter, silk_LPC_inverse_pred_gain_c, silk_ROR32, silk_bwexpander,
    silk_max_16, silk_max_32, silk_max_int, silk_min_32, silk_min_int, silk_sum_sqr_shift,
};
use crate::externs::{memcpy, memset};
#[c2rust::src_loc = "36:9"]
pub const NB_ATT: libc::c_int = 2 as libc::c_int;
#[c2rust::src_loc = "37:25"]
static mut HARM_ATT_Q15: [i16; 2] = [32440 as libc::c_int as i16, 31130 as libc::c_int as i16];
#[c2rust::src_loc = "38:25"]
static mut PLC_RAND_ATTENUATE_V_Q15: [i16; 2] =
    [31130 as libc::c_int as i16, 26214 as libc::c_int as i16];
#[c2rust::src_loc = "39:25"]
static mut PLC_RAND_ATTENUATE_UV_Q15: [i16; 2] =
    [32440 as libc::c_int as i16, 29491 as libc::c_int as i16];
#[no_mangle]
#[c2rust::src_loc = "54:1"]
pub unsafe extern "C" fn silk_PLC_Reset(mut psDec: *mut silk_decoder_state) {
    (*psDec).sPLC.pitchL_Q8 =
        (((*psDec).frame_length as u32) << 8 as libc::c_int - 1 as libc::c_int) as i32;
    (*psDec).sPLC.prevGain_Q16[0 as libc::c_int as usize] = ((1 as libc::c_int as libc::c_long
        * ((1 as libc::c_int as i64) << 16 as libc::c_int))
        as libc::c_double
        + 0.5f64) as i32;
    (*psDec).sPLC.prevGain_Q16[1 as libc::c_int as usize] = ((1 as libc::c_int as libc::c_long
        * ((1 as libc::c_int as i64) << 16 as libc::c_int))
        as libc::c_double
        + 0.5f64) as i32;
    (*psDec).sPLC.subfr_length = 20 as libc::c_int;
    (*psDec).sPLC.nb_subfr = 2 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "65:1"]
pub unsafe extern "C" fn silk_PLC(
    mut psDec: *mut silk_decoder_state,
    psDecCtrl: *mut silk_decoder_control,
    frame: *mut i16,
    lost: libc::c_int,
    arch: libc::c_int,
) {
    if (*psDec).fs_kHz != (*psDec).sPLC.fs_kHz {
        silk_PLC_Reset(psDec);
        (*psDec).sPLC.fs_kHz = (*psDec).fs_kHz;
    }
    if lost != 0 {
        silk_PLC_conceal(psDec, psDecCtrl, frame, arch);
        (*psDec).lossCnt += 1;
    } else {
        silk_PLC_update(psDec, psDecCtrl);
    };
}
#[inline]
#[c2rust::src_loc = "97:1"]
unsafe extern "C" fn silk_PLC_update(
    mut psDec: *mut silk_decoder_state,
    psDecCtrl: *mut silk_decoder_control,
) {
    let mut LTP_Gain_Q14: i32 = 0;
    let mut temp_LTP_Gain_Q14: i32 = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut psPLC: *mut silk_PLC_struct = 0 as *mut silk_PLC_struct;
    psPLC = &mut (*psDec).sPLC;
    (*psDec).prevSignalType = (*psDec).indices.signalType as libc::c_int;
    LTP_Gain_Q14 = 0 as libc::c_int;
    if (*psDec).indices.signalType as libc::c_int == TYPE_VOICED {
        j = 0 as libc::c_int;
        while j * (*psDec).subfr_length
            < (*psDecCtrl).pitchL[((*psDec).nb_subfr - 1 as libc::c_int) as usize]
        {
            if j == (*psDec).nb_subfr {
                break;
            }
            temp_LTP_Gain_Q14 = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < LTP_ORDER {
                temp_LTP_Gain_Q14 += (*psDecCtrl).LTPCoef_Q14
                    [(((*psDec).nb_subfr - 1 as libc::c_int - j) * LTP_ORDER + i) as usize]
                    as libc::c_int;
                i += 1;
            }
            if temp_LTP_Gain_Q14 > LTP_Gain_Q14 {
                LTP_Gain_Q14 = temp_LTP_Gain_Q14;
                memcpy(
                    ((*psPLC).LTPCoef_Q14).as_mut_ptr() as *mut libc::c_void,
                    &mut *((*psDecCtrl).LTPCoef_Q14).as_mut_ptr().offset(
                        (((*psDec).nb_subfr - 1 as libc::c_int - j) as i16 as i32
                            * 5 as libc::c_int as i16 as i32) as isize,
                    ) as *mut i16 as *const libc::c_void,
                    (5 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
                );
                (*psPLC).pitchL_Q8 = (((*psDecCtrl).pitchL
                    [((*psDec).nb_subfr - 1 as libc::c_int - j) as usize]
                    as u32)
                    << 8 as libc::c_int) as i32;
            }
            j += 1;
        }
        memset(
            ((*psPLC).LTPCoef_Q14).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
        );
        (*psPLC).LTPCoef_Q14[(LTP_ORDER / 2 as libc::c_int) as usize] = LTP_Gain_Q14 as i16;
        if LTP_Gain_Q14 < V_PITCH_GAIN_START_MIN_Q14 {
            let mut scale_Q10: libc::c_int = 0;
            let mut tmp: i32 = 0;
            tmp = ((11469 as libc::c_int as u32) << 10 as libc::c_int) as i32;
            scale_Q10 = tmp
                / (if LTP_Gain_Q14 > 1 as libc::c_int {
                    LTP_Gain_Q14
                } else {
                    1 as libc::c_int
                });
            i = 0 as libc::c_int;
            while i < LTP_ORDER {
                (*psPLC).LTPCoef_Q14[i as usize] = ((*psPLC).LTPCoef_Q14[i as usize] as i32
                    * scale_Q10 as i16 as i32
                    >> 10 as libc::c_int) as i16;
                i += 1;
            }
        } else if LTP_Gain_Q14 > V_PITCH_GAIN_START_MAX_Q14 {
            let mut scale_Q14: libc::c_int = 0;
            let mut tmp_0: i32 = 0;
            tmp_0 = ((15565 as libc::c_int as u32) << 14 as libc::c_int) as i32;
            scale_Q14 = tmp_0
                / (if LTP_Gain_Q14 > 1 as libc::c_int {
                    LTP_Gain_Q14
                } else {
                    1 as libc::c_int
                });
            i = 0 as libc::c_int;
            while i < LTP_ORDER {
                (*psPLC).LTPCoef_Q14[i as usize] = ((*psPLC).LTPCoef_Q14[i as usize] as i32
                    * scale_Q14 as i16 as i32
                    >> 14 as libc::c_int) as i16;
                i += 1;
            }
        }
    } else {
        (*psPLC).pitchL_Q8 = ((((*psDec).fs_kHz as i16 as i32 * 18 as libc::c_int as i16 as i32)
            as u32)
            << 8 as libc::c_int) as i32;
        memset(
            ((*psPLC).LTPCoef_Q14).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
        );
    }
    memcpy(
        ((*psPLC).prevLPC_Q12).as_mut_ptr() as *mut libc::c_void,
        ((*psDecCtrl).PredCoef_Q12[1 as libc::c_int as usize]).as_mut_ptr() as *const libc::c_void,
        ((*psDec).LPC_order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    (*psPLC).prevLTP_scale_Q14 = (*psDecCtrl).LTP_scale_Q14 as i16;
    memcpy(
        ((*psPLC).prevGain_Q16).as_mut_ptr() as *mut libc::c_void,
        &mut *((*psDecCtrl).Gains_Q16)
            .as_mut_ptr()
            .offset(((*psDec).nb_subfr - 2 as libc::c_int) as isize) as *mut i32
            as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
    (*psPLC).subfr_length = (*psDec).subfr_length;
    (*psPLC).nb_subfr = (*psDec).nb_subfr;
}
#[inline]
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn silk_PLC_energy(
    energy1: *mut i32,
    shift1: *mut libc::c_int,
    energy2: *mut i32,
    shift2: *mut libc::c_int,
    exc_Q14: *const i32,
    prevGain_Q10: *const i32,
    subfr_length: libc::c_int,
    nb_subfr: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut exc_buf_ptr: *mut i16 = 0 as *mut i16;
    let vla = (2 as libc::c_int * subfr_length) as usize;
    let mut exc_buf: Vec<i16> = ::std::vec::from_elem(0, vla);
    exc_buf_ptr = exc_buf.as_mut_ptr();
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < subfr_length {
            *exc_buf_ptr.offset(i as isize) = (if (*exc_Q14
                .offset((i + (k + nb_subfr - 2 as libc::c_int) * subfr_length) as isize)
                as i64
                * *prevGain_Q10.offset(k as isize) as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int
                > silk_int16_MAX
            {
                silk_int16_MAX
            } else if ((*exc_Q14
                .offset((i + (k + nb_subfr - 2 as libc::c_int) * subfr_length) as isize)
                as i64
                * *prevGain_Q10.offset(k as isize) as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int)
                < silk_int16_MIN
            {
                silk_int16_MIN
            } else {
                (*exc_Q14.offset((i + (k + nb_subfr - 2 as libc::c_int) * subfr_length) as isize)
                    as i64
                    * *prevGain_Q10.offset(k as isize) as libc::c_long
                    >> 16 as libc::c_int) as i32
                    >> 8 as libc::c_int
            }) as i16;
            i += 1;
        }
        exc_buf_ptr = exc_buf_ptr.offset(subfr_length as isize);
        k += 1;
    }
    silk_sum_sqr_shift(energy1, shift1, exc_buf.as_mut_ptr(), subfr_length);
    silk_sum_sqr_shift(
        energy2,
        shift2,
        &mut *exc_buf.as_mut_ptr().offset(subfr_length as isize),
        subfr_length,
    );
}
#[inline]
#[c2rust::src_loc = "194:1"]
unsafe extern "C" fn silk_PLC_conceal(
    psDec: *mut silk_decoder_state,
    mut psDecCtrl: *mut silk_decoder_control,
    frame: *mut i16,
    arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut sLTP_buf_idx: libc::c_int = 0;
    let mut shift1: libc::c_int = 0;
    let mut shift2: libc::c_int = 0;
    let mut rand_seed: i32 = 0;
    let mut harm_Gain_Q15: i32 = 0;
    let mut rand_Gain_Q15: i32 = 0;
    let mut inv_gain_Q30: i32 = 0;
    let mut energy1: i32 = 0;
    let mut energy2: i32 = 0;
    let mut rand_ptr: *mut i32 = 0 as *mut i32;
    let mut pred_lag_ptr: *mut i32 = 0 as *mut i32;
    let mut LPC_pred_Q10: i32 = 0;
    let mut LTP_pred_Q12: i32 = 0;
    let mut rand_scale_Q14: i16 = 0;
    let mut B_Q14: *mut i16 = 0 as *mut i16;
    let mut sLPC_Q14_ptr: *mut i32 = 0 as *mut i32;
    let mut A_Q12: [i16; 16] = [0; 16];
    let mut psPLC: *mut silk_PLC_struct = &mut (*psDec).sPLC;
    let mut prevGain_Q10: [i32; 2] = [0; 2];
    let vla = ((*psDec).ltp_mem_length + (*psDec).frame_length) as usize;
    let mut sLTP_Q14: Vec<i32> = ::std::vec::from_elem(0, vla);
    let vla_0 = (*psDec).ltp_mem_length as usize;
    let mut sLTP: Vec<i16> = ::std::vec::from_elem(0, vla_0);
    prevGain_Q10[0 as libc::c_int as usize] =
        (*psPLC).prevGain_Q16[0 as libc::c_int as usize] >> 6 as libc::c_int;
    prevGain_Q10[1 as libc::c_int as usize] =
        (*psPLC).prevGain_Q16[1 as libc::c_int as usize] >> 6 as libc::c_int;
    if (*psDec).first_frame_after_reset != 0 {
        memset(
            ((*psPLC).prevLPC_Q12).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[i16; 16]>() as libc::c_ulong,
        );
    }
    silk_PLC_energy(
        &mut energy1,
        &mut shift1,
        &mut energy2,
        &mut shift2,
        ((*psDec).exc_Q14).as_mut_ptr(),
        prevGain_Q10.as_mut_ptr(),
        (*psDec).subfr_length,
        (*psDec).nb_subfr,
    );
    if energy1 >> shift2 < energy2 >> shift1 {
        rand_ptr = &mut *((*psDec).exc_Q14).as_mut_ptr().offset((silk_max_int
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int)(
            0 as libc::c_int,
            ((*psPLC).nb_subfr - 1 as libc::c_int) * (*psPLC).subfr_length - RAND_BUF_SIZE,
        ) as isize) as *mut i32;
    } else {
        rand_ptr = &mut *((*psDec).exc_Q14).as_mut_ptr().offset((silk_max_int
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int)(
            0 as libc::c_int,
            (*psPLC).nb_subfr * (*psPLC).subfr_length - RAND_BUF_SIZE,
        ) as isize) as *mut i32;
    }
    B_Q14 = ((*psPLC).LTPCoef_Q14).as_mut_ptr();
    rand_scale_Q14 = (*psPLC).randScale_Q14;
    harm_Gain_Q15 =
        HARM_ATT_Q15[silk_min_int(NB_ATT - 1 as libc::c_int, (*psDec).lossCnt) as usize] as i32;
    if (*psDec).prevSignalType == TYPE_VOICED {
        rand_Gain_Q15 = PLC_RAND_ATTENUATE_V_Q15
            [silk_min_int(NB_ATT - 1 as libc::c_int, (*psDec).lossCnt) as usize]
            as i32;
    } else {
        rand_Gain_Q15 = PLC_RAND_ATTENUATE_UV_Q15
            [silk_min_int(NB_ATT - 1 as libc::c_int, (*psDec).lossCnt) as usize]
            as i32;
    }
    silk_bwexpander(
        ((*psPLC).prevLPC_Q12).as_mut_ptr(),
        (*psDec).LPC_order,
        (0.99f64 * ((1 as libc::c_int as i64) << 16 as libc::c_int) as libc::c_double + 0.5f64)
            as i32,
    );
    memcpy(
        A_Q12.as_mut_ptr() as *mut libc::c_void,
        ((*psPLC).prevLPC_Q12).as_mut_ptr() as *const libc::c_void,
        ((*psDec).LPC_order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    if (*psDec).lossCnt == 0 as libc::c_int {
        rand_scale_Q14 = ((1 as libc::c_int) << 14 as libc::c_int) as i16;
        if (*psDec).prevSignalType == TYPE_VOICED {
            i = 0 as libc::c_int;
            while i < LTP_ORDER {
                rand_scale_Q14 = (rand_scale_Q14 as libc::c_int
                    - *B_Q14.offset(i as isize) as libc::c_int)
                    as i16;
                i += 1;
            }
            rand_scale_Q14 = silk_max_16(3277 as libc::c_int as i16, rand_scale_Q14);
            rand_scale_Q14 = (rand_scale_Q14 as i32 * (*psPLC).prevLTP_scale_Q14 as i32
                >> 14 as libc::c_int) as i16;
        } else {
            let mut invGain_Q30: i32 = 0;
            let mut down_scale_Q30: i32 = 0;
            invGain_Q30 = silk_LPC_inverse_pred_gain_c(
                ((*psPLC).prevLPC_Q12).as_mut_ptr(),
                (*psDec).LPC_order,
            );
            down_scale_Q30 = silk_min_32(
                (1 as libc::c_int) << 30 as libc::c_int >> 3 as libc::c_int,
                invGain_Q30,
            );
            down_scale_Q30 = silk_max_32(
                (1 as libc::c_int) << 30 as libc::c_int >> 8 as libc::c_int,
                down_scale_Q30,
            );
            down_scale_Q30 = ((down_scale_Q30 as u32) << 3 as libc::c_int) as i32;
            rand_Gain_Q15 = (down_scale_Q30 as libc::c_long * rand_Gain_Q15 as i16 as i64
                >> 16 as libc::c_int) as i32
                >> 14 as libc::c_int;
        }
    }
    rand_seed = (*psPLC).rand_seed;
    lag = if 8 as libc::c_int == 1 as libc::c_int {
        ((*psPLC).pitchL_Q8 >> 1 as libc::c_int) + ((*psPLC).pitchL_Q8 & 1 as libc::c_int)
    } else {
        ((*psPLC).pitchL_Q8 >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
            >> 1 as libc::c_int
    };
    sLTP_buf_idx = (*psDec).ltp_mem_length;
    idx = (*psDec).ltp_mem_length - lag - (*psDec).LPC_order - LTP_ORDER / 2 as libc::c_int;
    if !(idx > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: idx > 0\0" as *const u8 as *const libc::c_char,
            b"silk/PLC.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int,
        );
    }
    silk_LPC_analysis_filter(
        &mut *sLTP.as_mut_ptr().offset(idx as isize),
        &mut *((*psDec).outBuf).as_mut_ptr().offset(idx as isize),
        A_Q12.as_mut_ptr(),
        (*psDec).ltp_mem_length - idx,
        (*psDec).LPC_order,
        arch,
    );
    inv_gain_Q30 = silk_INVERSE32_varQ(
        (*psPLC).prevGain_Q16[1 as libc::c_int as usize],
        46 as libc::c_int,
    );
    inv_gain_Q30 = if inv_gain_Q30 < 0x7fffffff as libc::c_int >> 1 as libc::c_int {
        inv_gain_Q30
    } else {
        0x7fffffff as libc::c_int >> 1 as libc::c_int
    };
    i = idx + (*psDec).LPC_order;
    while i < (*psDec).ltp_mem_length {
        *sLTP_Q14.as_mut_ptr().offset(i as isize) = (inv_gain_Q30 as libc::c_long
            * *sLTP.as_mut_ptr().offset(i as isize) as i64
            >> 16 as libc::c_int) as i32;
        i += 1;
    }
    k = 0 as libc::c_int;
    while k < (*psDec).nb_subfr {
        pred_lag_ptr = &mut *sLTP_Q14
            .as_mut_ptr()
            .offset((sLTP_buf_idx - lag + LTP_ORDER / 2 as libc::c_int) as isize)
            as *mut i32;
        i = 0 as libc::c_int;
        while i < (*psDec).subfr_length {
            LTP_pred_Q12 = 2 as libc::c_int;
            LTP_pred_Q12 = (LTP_pred_Q12 as libc::c_long
                + (*pred_lag_ptr.offset(0 as libc::c_int as isize) as libc::c_long
                    * *B_Q14.offset(0 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q12 = (LTP_pred_Q12 as libc::c_long
                + (*pred_lag_ptr.offset(-(1 as libc::c_int) as isize) as libc::c_long
                    * *B_Q14.offset(1 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q12 = (LTP_pred_Q12 as libc::c_long
                + (*pred_lag_ptr.offset(-(2 as libc::c_int) as isize) as libc::c_long
                    * *B_Q14.offset(2 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q12 = (LTP_pred_Q12 as libc::c_long
                + (*pred_lag_ptr.offset(-(3 as libc::c_int) as isize) as libc::c_long
                    * *B_Q14.offset(3 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            LTP_pred_Q12 = (LTP_pred_Q12 as libc::c_long
                + (*pred_lag_ptr.offset(-(4 as libc::c_int) as isize) as libc::c_long
                    * *B_Q14.offset(4 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int)) as i32;
            pred_lag_ptr = pred_lag_ptr.offset(1);
            rand_seed = (907633515 as libc::c_int as u32)
                .wrapping_add((rand_seed as u32).wrapping_mul(196314165 as libc::c_int as u32))
                as i32;
            idx = rand_seed >> 25 as libc::c_int & RAND_BUF_MASK;
            *sLTP_Q14.as_mut_ptr().offset(sLTP_buf_idx as isize) =
                (((LTP_pred_Q12 as libc::c_long
                    + (*rand_ptr.offset(idx as isize) as libc::c_long * rand_scale_Q14 as i64
                        >> 16 as libc::c_int)) as i32 as u32)
                    << 2 as libc::c_int) as i32;
            sLTP_buf_idx += 1;
            i += 1;
        }
        j = 0 as libc::c_int;
        while j < LTP_ORDER {
            *B_Q14.offset(j as isize) = (harm_Gain_Q15 as i16 as i32
                * *B_Q14.offset(j as isize) as i32
                >> 15 as libc::c_int) as i16;
            j += 1;
        }
        if (*psDec).indices.signalType as libc::c_int != TYPE_NO_VOICE_ACTIVITY {
            rand_scale_Q14 =
                (rand_scale_Q14 as i32 * rand_Gain_Q15 as i16 as i32 >> 15 as libc::c_int) as i16;
        }
        (*psPLC).pitchL_Q8 = ((*psPLC).pitchL_Q8 as libc::c_long
            + ((*psPLC).pitchL_Q8 as libc::c_long * 655 as libc::c_int as i16 as i64
                >> 16 as libc::c_int)) as i32;
        (*psPLC).pitchL_Q8 = silk_min_32(
            (*psPLC).pitchL_Q8,
            (((18 as libc::c_int as i16 as i32 * (*psDec).fs_kHz as i16 as i32) as u32)
                << 8 as libc::c_int) as i32,
        );
        lag = if 8 as libc::c_int == 1 as libc::c_int {
            ((*psPLC).pitchL_Q8 >> 1 as libc::c_int) + ((*psPLC).pitchL_Q8 & 1 as libc::c_int)
        } else {
            ((*psPLC).pitchL_Q8 >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        k += 1;
    }
    sLPC_Q14_ptr = &mut *sLTP_Q14
        .as_mut_ptr()
        .offset(((*psDec).ltp_mem_length - MAX_LPC_ORDER) as isize) as *mut i32;
    memcpy(
        sLPC_Q14_ptr as *mut libc::c_void,
        ((*psDec).sLPC_Q14_buf).as_mut_ptr() as *const libc::c_void,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
    if !((*psDec).LPC_order >= 10 as libc::c_int) {
        celt_fatal(
            b"assertion failed: psDec->LPC_order >= 10\0" as *const u8 as *const libc::c_char,
            b"silk/PLC.c\0" as *const u8 as *const libc::c_char,
            350 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < (*psDec).frame_length {
        LPC_pred_Q10 = (*psDec).LPC_order >> 1 as libc::c_int;
        LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
            + (*sLPC_Q14_ptr.offset((16 as libc::c_int + i - 1 as libc::c_int) as isize)
                as libc::c_long
                * A_Q12[0 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
            + (*sLPC_Q14_ptr.offset((16 as libc::c_int + i - 2 as libc::c_int) as isize)
                as libc::c_long
                * A_Q12[1 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
            + (*sLPC_Q14_ptr.offset((16 as libc::c_int + i - 3 as libc::c_int) as isize)
                as libc::c_long
                * A_Q12[2 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
            + (*sLPC_Q14_ptr.offset((16 as libc::c_int + i - 4 as libc::c_int) as isize)
                as libc::c_long
                * A_Q12[3 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
            + (*sLPC_Q14_ptr.offset((16 as libc::c_int + i - 5 as libc::c_int) as isize)
                as libc::c_long
                * A_Q12[4 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
            + (*sLPC_Q14_ptr.offset((16 as libc::c_int + i - 6 as libc::c_int) as isize)
                as libc::c_long
                * A_Q12[5 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
            + (*sLPC_Q14_ptr.offset((16 as libc::c_int + i - 7 as libc::c_int) as isize)
                as libc::c_long
                * A_Q12[6 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
            + (*sLPC_Q14_ptr.offset((16 as libc::c_int + i - 8 as libc::c_int) as isize)
                as libc::c_long
                * A_Q12[7 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
            + (*sLPC_Q14_ptr.offset((16 as libc::c_int + i - 9 as libc::c_int) as isize)
                as libc::c_long
                * A_Q12[8 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
            + (*sLPC_Q14_ptr.offset((16 as libc::c_int + i - 10 as libc::c_int) as isize)
                as libc::c_long
                * A_Q12[9 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        j = 10 as libc::c_int;
        while j < (*psDec).LPC_order {
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*sLPC_Q14_ptr.offset((16 as libc::c_int + i - j - 1 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12[j as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            j += 1;
        }
        *sLPC_Q14_ptr.offset((MAX_LPC_ORDER + i) as isize) = if (*sLPC_Q14_ptr
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
                        if LPC_pred_Q10 < 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int {
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
            if (*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize)
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
                        if LPC_pred_Q10 < 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int {
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
                *sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize)
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
            }
        } else if (*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize)
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
            *sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize)
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
                        if LPC_pred_Q10 < 0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int {
                            0x80000000 as libc::c_uint as i32 >> 4 as libc::c_int
                        } else {
                            LPC_pred_Q10
                        }
                    }
                }) as u32)
                    << 4 as libc::c_int) as i32
        };
        *frame.offset(i as isize) = (if (if (if 8 as libc::c_int == 1 as libc::c_int {
            ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                    * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                    >> 16 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else {
            if (if 8 as libc::c_int == 1 as libc::c_int {
                ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                    * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                    >> 16 as libc::c_int) as i32
                    >> 1 as libc::c_int)
                    + ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                        * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                        >> 16 as libc::c_int) as i32
                        & 1 as libc::c_int)
            } else {
                ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                    * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                    >> 16 as libc::c_int) as i32
                    >> 8 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < 0x8000 as libc::c_int as i16 as libc::c_int
            {
                0x8000 as libc::c_int as i16 as libc::c_int
            } else {
                if 8 as libc::c_int == 1 as libc::c_int {
                    ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                        * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                        >> 16 as libc::c_int) as i32
                        >> 1 as libc::c_int)
                        + ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                            * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                            >> 16 as libc::c_int) as i32
                            & 1 as libc::c_int)
                } else {
                    ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                        * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                        >> 16 as libc::c_int) as i32
                        >> 8 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int
                        >> 1 as libc::c_int
                }
            }
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if (if 8 as libc::c_int == 1 as libc::c_int {
            ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                    * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                    >> 16 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else {
            if (if 8 as libc::c_int == 1 as libc::c_int {
                ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                    * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                    >> 16 as libc::c_int) as i32
                    >> 1 as libc::c_int)
                    + ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                        * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                        >> 16 as libc::c_int) as i32
                        & 1 as libc::c_int)
            } else {
                ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                    * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                    >> 16 as libc::c_int) as i32
                    >> 8 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < 0x8000 as libc::c_int as i16 as libc::c_int
            {
                0x8000 as libc::c_int as i16 as libc::c_int
            } else {
                if 8 as libc::c_int == 1 as libc::c_int {
                    ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                        * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                        >> 16 as libc::c_int) as i32
                        >> 1 as libc::c_int)
                        + ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                            * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                            >> 16 as libc::c_int) as i32
                            & 1 as libc::c_int)
                } else {
                    ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                        * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                        >> 16 as libc::c_int) as i32
                        >> 8 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int
                        >> 1 as libc::c_int
                }
            }
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                    * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                    >> 16 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                    * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                    >> 16 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as i16 as libc::c_int
        {
            0x8000 as libc::c_int as i16 as libc::c_int
        } else if 8 as libc::c_int == 1 as libc::c_int {
            ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 1 as libc::c_int)
                + ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                    * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                    >> 16 as libc::c_int) as i32
                    & 1 as libc::c_int)
        } else {
            ((*sLPC_Q14_ptr.offset((16 as libc::c_int + i) as isize) as i64
                * prevGain_Q10[1 as libc::c_int as usize] as libc::c_long
                >> 16 as libc::c_int) as i32
                >> 8 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as i16;
        i += 1;
    }
    memcpy(
        ((*psDec).sLPC_Q14_buf).as_mut_ptr() as *mut libc::c_void,
        &mut *sLPC_Q14_ptr.offset((*psDec).frame_length as isize) as *mut i32
            as *const libc::c_void,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
    (*psPLC).rand_seed = rand_seed;
    (*psPLC).randScale_Q14 = rand_scale_Q14;
    i = 0 as libc::c_int;
    while i < MAX_NB_SUBFR {
        (*psDecCtrl).pitchL[i as usize] = lag;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "392:1"]
pub unsafe extern "C" fn silk_PLC_glue_frames(
    psDec: *mut silk_decoder_state,
    frame: *mut i16,
    length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut energy_shift: libc::c_int = 0;
    let mut energy: i32 = 0;
    let mut psPLC: *mut silk_PLC_struct = 0 as *mut silk_PLC_struct;
    psPLC = &mut (*psDec).sPLC;
    if (*psDec).lossCnt != 0 {
        silk_sum_sqr_shift(
            &mut (*psPLC).conc_energy,
            &mut (*psPLC).conc_energy_shift,
            frame as *const i16,
            length,
        );
        (*psPLC).last_frame_lost = 1 as libc::c_int;
    } else {
        if (*psDec).sPLC.last_frame_lost != 0 {
            silk_sum_sqr_shift(&mut energy, &mut energy_shift, frame as *const i16, length);
            if energy_shift > (*psPLC).conc_energy_shift {
                (*psPLC).conc_energy =
                    (*psPLC).conc_energy >> energy_shift - (*psPLC).conc_energy_shift;
            } else if energy_shift < (*psPLC).conc_energy_shift {
                energy = energy >> (*psPLC).conc_energy_shift - energy_shift;
            }
            if energy > (*psPLC).conc_energy {
                let mut frac_Q24: i32 = 0;
                let mut LZ: i32 = 0;
                let mut gain_Q16: i32 = 0;
                let mut slope_Q16: i32 = 0;
                LZ = silk_CLZ32((*psPLC).conc_energy);
                LZ = LZ - 1 as libc::c_int;
                (*psPLC).conc_energy = (((*psPLC).conc_energy as u32) << LZ) as i32;
                energy = energy >> silk_max_32(24 as libc::c_int - LZ, 0 as libc::c_int);
                frac_Q24 = (*psPLC).conc_energy
                    / (if energy > 1 as libc::c_int {
                        energy
                    } else {
                        1 as libc::c_int
                    });
                gain_Q16 = ((silk_SQRT_APPROX(frac_Q24) as u32) << 4 as libc::c_int) as i32;
                slope_Q16 = (((1 as libc::c_int) << 16 as libc::c_int) - gain_Q16) / length;
                slope_Q16 = ((slope_Q16 as u32) << 2 as libc::c_int) as i32;
                i = 0 as libc::c_int;
                while i < length {
                    *frame.offset(i as isize) =
                        (gain_Q16 as libc::c_long * *frame.offset(i as isize) as i64
                            >> 16 as libc::c_int) as i32 as i16;
                    gain_Q16 += slope_Q16;
                    if gain_Q16 > (1 as libc::c_int) << 16 as libc::c_int {
                        break;
                    }
                    i += 1;
                }
            }
        }
        (*psPLC).last_frame_lost = 0 as libc::c_int;
    };
}
