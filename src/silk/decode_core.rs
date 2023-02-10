use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:32"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
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
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/Inlines.h:32"]
pub mod Inlines_h {
    #[inline]
    #[c2rust::src_loc = "97:1"]
    pub unsafe extern "C" fn silk_DIV32_varQ(a32: i32, b32: i32, Qres: libc::c_int) -> i32 {
        let mut a_headrm: libc::c_int = 0;
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: i32 = 0;
        let mut a32_nrm: i32 = 0;
        let mut b32_nrm: i32 = 0;
        let mut result: i32 = 0;
        a_headrm = silk_CLZ32(if a32 > 0 as libc::c_int { a32 } else { -a32 }) - 1 as libc::c_int;
        a32_nrm = ((a32 as u32) << a_headrm) as i32;
        b_headrm = silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 }) - 1 as libc::c_int;
        b32_nrm = ((b32 as u32) << b_headrm) as i32;
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int) / (b32_nrm >> 16 as libc::c_int);
        result = (a32_nrm as libc::c_long * b32_inv as i16 as i64 >> 16 as libc::c_int) as i32;
        a32_nrm = (a32_nrm as u32).wrapping_sub(
            (((b32_nrm as i64 * result as libc::c_long >> 32 as libc::c_int) as i32 as u32)
                << 3 as libc::c_int) as i32 as u32,
        ) as i32;
        result = (result as libc::c_long
            + (a32_nrm as libc::c_long * b32_inv as i16 as i64 >> 16 as libc::c_int))
            as i32;
        lshift = 29 as libc::c_int + a_headrm - b_headrm - Qres;
        if lshift < 0 as libc::c_int {
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
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "101:26"]
        pub static silk_Quantization_Offsets_Q10: [[i16; 2]; 2];
    }
}
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "142:9"]
    pub const MAX_LPC_ORDER: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "146:9"]
    pub const LTP_ORDER: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "90:9"]
    pub const MAX_NB_SUBFR: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "135:9"]
    pub const QUANT_LEVEL_ADJUST_Q10: libc::c_int = 80 as libc::c_int;
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
use self::arch_h::celt_fatal;
pub use self::define_h::{
    LTP_ORDER, MAX_LPC_ORDER, MAX_NB_SUBFR, QUANT_LEVEL_ADJUST_Q10, TYPE_VOICED,
};
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
pub use self::macros_h::silk_CLZ32;
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
pub use self::structs_h::{
    silk_CNG_struct, silk_NLSF_CB_struct, silk_PLC_struct, silk_decoder_control,
    silk_decoder_state, SideInfoIndices,
};
use self::tables_h::silk_Quantization_Offsets_Q10;
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN, silk_int32_MAX, silk_int32_MIN};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __int8_t, __uint32_t, __uint8_t};
pub use self::Inlines_h::{silk_DIV32_varQ, silk_INVERSE32_varQ};
use self::SigProc_FIX_h::silk_LPC_analysis_filter;
use crate::externs::{memcpy, memset};
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn silk_decode_core(
    mut psDec: *mut silk_decoder_state,
    mut psDecCtrl: *mut silk_decoder_control,
    xq: *mut i16,
    pulses: *const i16,
    arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lag: libc::c_int = 0 as libc::c_int;
    let mut start_idx: libc::c_int = 0;
    let mut sLTP_buf_idx: libc::c_int = 0;
    let mut NLSF_interpolation_flag: libc::c_int = 0;
    let mut signalType: libc::c_int = 0;
    let mut A_Q12: *mut i16 = 0 as *mut i16;
    let mut B_Q14: *mut i16 = 0 as *mut i16;
    let mut pxq: *mut i16 = 0 as *mut i16;
    let mut A_Q12_tmp: [i16; 16] = [0; 16];
    let mut LTP_pred_Q13: i32 = 0;
    let mut LPC_pred_Q10: i32 = 0;
    let mut Gain_Q10: i32 = 0;
    let mut inv_gain_Q31: i32 = 0;
    let mut gain_adj_Q16: i32 = 0;
    let mut rand_seed: i32 = 0;
    let mut offset_Q10: i32 = 0;
    let mut pred_lag_ptr: *mut i32 = 0 as *mut i32;
    let mut pexc_Q14: *mut i32 = 0 as *mut i32;
    let mut pres_Q14: *mut i32 = 0 as *mut i32;
    let vla = (*psDec).ltp_mem_length as usize;
    let mut sLTP: Vec<i16> = ::std::vec::from_elem(0, vla);
    let vla_0 = ((*psDec).ltp_mem_length + (*psDec).frame_length) as usize;
    let mut sLTP_Q15: Vec<i32> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = (*psDec).subfr_length as usize;
    let mut res_Q14: Vec<i32> = ::std::vec::from_elem(0, vla_1);
    let vla_2 = ((*psDec).subfr_length + 16 as libc::c_int) as usize;
    let mut sLPC_Q14: Vec<i32> = ::std::vec::from_elem(0, vla_2);
    offset_Q10 = silk_Quantization_Offsets_Q10
        [((*psDec).indices.signalType as libc::c_int >> 1 as libc::c_int) as usize]
        [(*psDec).indices.quantOffsetType as usize] as i32;
    if ((*psDec).indices.NLSFInterpCoef_Q2 as libc::c_int) < (1 as libc::c_int) << 2 as libc::c_int
    {
        NLSF_interpolation_flag = 1 as libc::c_int;
    } else {
        NLSF_interpolation_flag = 0 as libc::c_int;
    }
    rand_seed = (*psDec).indices.Seed as i32;
    i = 0 as libc::c_int;
    while i < (*psDec).frame_length {
        rand_seed = (907633515 as libc::c_int as u32)
            .wrapping_add((rand_seed as u32).wrapping_mul(196314165 as libc::c_int as u32))
            as i32;
        (*psDec).exc_Q14[i as usize] =
            ((*pulses.offset(i as isize) as i32 as u32) << 14 as libc::c_int) as i32;
        if (*psDec).exc_Q14[i as usize] > 0 as libc::c_int {
            (*psDec).exc_Q14[i as usize] -= QUANT_LEVEL_ADJUST_Q10 << 4 as libc::c_int;
        } else if (*psDec).exc_Q14[i as usize] < 0 as libc::c_int {
            (*psDec).exc_Q14[i as usize] += QUANT_LEVEL_ADJUST_Q10 << 4 as libc::c_int;
        }
        (*psDec).exc_Q14[i as usize] += offset_Q10 << 4 as libc::c_int;
        if rand_seed < 0 as libc::c_int {
            (*psDec).exc_Q14[i as usize] = -(*psDec).exc_Q14[i as usize];
        }
        rand_seed = (rand_seed as u32).wrapping_add(*pulses.offset(i as isize) as u32) as i32;
        i += 1;
    }
    memcpy(
        sLPC_Q14.as_mut_ptr() as *mut libc::c_void,
        ((*psDec).sLPC_Q14_buf).as_mut_ptr() as *const libc::c_void,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
    pexc_Q14 = ((*psDec).exc_Q14).as_mut_ptr();
    pxq = xq;
    sLTP_buf_idx = (*psDec).ltp_mem_length;
    k = 0 as libc::c_int;
    while k < (*psDec).nb_subfr {
        pres_Q14 = res_Q14.as_mut_ptr();
        A_Q12 = ((*psDecCtrl).PredCoef_Q12[(k >> 1 as libc::c_int) as usize]).as_mut_ptr();
        memcpy(
            A_Q12_tmp.as_mut_ptr() as *mut libc::c_void,
            A_Q12 as *const libc::c_void,
            ((*psDec).LPC_order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
        );
        B_Q14 = &mut *((*psDecCtrl).LTPCoef_Q14)
            .as_mut_ptr()
            .offset((k * LTP_ORDER) as isize) as *mut i16;
        signalType = (*psDec).indices.signalType as libc::c_int;
        Gain_Q10 = (*psDecCtrl).Gains_Q16[k as usize] >> 6 as libc::c_int;
        inv_gain_Q31 = silk_INVERSE32_varQ((*psDecCtrl).Gains_Q16[k as usize], 47 as libc::c_int);
        if (*psDecCtrl).Gains_Q16[k as usize] != (*psDec).prev_gain_Q16 {
            gain_adj_Q16 = silk_DIV32_varQ(
                (*psDec).prev_gain_Q16,
                (*psDecCtrl).Gains_Q16[k as usize],
                16 as libc::c_int,
            );
            i = 0 as libc::c_int;
            while i < MAX_LPC_ORDER {
                *sLPC_Q14.as_mut_ptr().offset(i as isize) = (gain_adj_Q16 as i64
                    * *sLPC_Q14.as_mut_ptr().offset(i as isize) as libc::c_long
                    >> 16 as libc::c_int)
                    as i32;
                i += 1;
            }
        } else {
            gain_adj_Q16 = (1 as libc::c_int) << 16 as libc::c_int;
        }
        (*psDec).prev_gain_Q16 = (*psDecCtrl).Gains_Q16[k as usize];
        if (*psDec).lossCnt != 0
            && (*psDec).prevSignalType == TYPE_VOICED
            && (*psDec).indices.signalType as libc::c_int != TYPE_VOICED
            && k < MAX_NB_SUBFR / 2 as libc::c_int
        {
            memset(
                B_Q14 as *mut libc::c_void,
                0 as libc::c_int,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
            );
            *B_Q14.offset((LTP_ORDER / 2 as libc::c_int) as isize) =
                (0.25f64 * ((1 as libc::c_int as i64) << 14 as libc::c_int) as libc::c_double
                    + 0.5f64) as i32 as i16;
            signalType = TYPE_VOICED;
            (*psDecCtrl).pitchL[k as usize] = (*psDec).lagPrev;
        }
        if signalType == TYPE_VOICED {
            lag = (*psDecCtrl).pitchL[k as usize];
            if k == 0 as libc::c_int || k == 2 as libc::c_int && NLSF_interpolation_flag != 0 {
                start_idx = (*psDec).ltp_mem_length
                    - lag
                    - (*psDec).LPC_order
                    - LTP_ORDER / 2 as libc::c_int;
                if !(start_idx > 0 as libc::c_int) {
                    celt_fatal(
                        b"assertion failed: start_idx > 0\0" as *const u8 as *const libc::c_char,
                        b"silk/decode_core.c\0" as *const u8 as *const libc::c_char,
                        144 as libc::c_int,
                    );
                }
                if k == 2 as libc::c_int {
                    memcpy(
                        &mut *((*psDec).outBuf)
                            .as_mut_ptr()
                            .offset((*psDec).ltp_mem_length as isize)
                            as *mut i16 as *mut libc::c_void,
                        xq as *const libc::c_void,
                        ((2 as libc::c_int * (*psDec).subfr_length) as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
                    );
                }
                silk_LPC_analysis_filter(
                    &mut *sLTP.as_mut_ptr().offset(start_idx as isize),
                    &mut *((*psDec).outBuf)
                        .as_mut_ptr()
                        .offset((start_idx + k * (*psDec).subfr_length) as isize),
                    A_Q12,
                    (*psDec).ltp_mem_length - start_idx,
                    (*psDec).LPC_order,
                    arch,
                );
                if k == 0 as libc::c_int {
                    inv_gain_Q31 = (((inv_gain_Q31 as libc::c_long
                        * (*psDecCtrl).LTP_scale_Q14 as i16 as i64
                        >> 16 as libc::c_int) as i32 as u32)
                        << 2 as libc::c_int) as i32;
                }
                i = 0 as libc::c_int;
                while i < lag + LTP_ORDER / 2 as libc::c_int {
                    *sLTP_Q15
                        .as_mut_ptr()
                        .offset((sLTP_buf_idx - i - 1 as libc::c_int) as isize) =
                        (inv_gain_Q31 as libc::c_long
                            * *sLTP
                                .as_mut_ptr()
                                .offset(((*psDec).ltp_mem_length - i - 1 as libc::c_int) as isize)
                                as i64
                            >> 16 as libc::c_int) as i32;
                    i += 1;
                }
            } else if gain_adj_Q16 != (1 as libc::c_int) << 16 as libc::c_int {
                i = 0 as libc::c_int;
                while i < lag + LTP_ORDER / 2 as libc::c_int {
                    *sLTP_Q15
                        .as_mut_ptr()
                        .offset((sLTP_buf_idx - i - 1 as libc::c_int) as isize) =
                        (gain_adj_Q16 as i64
                            * *sLTP_Q15
                                .as_mut_ptr()
                                .offset((sLTP_buf_idx - i - 1 as libc::c_int) as isize)
                                as libc::c_long
                            >> 16 as libc::c_int) as i32;
                    i += 1;
                }
            }
        }
        if signalType == TYPE_VOICED {
            pred_lag_ptr = &mut *sLTP_Q15
                .as_mut_ptr()
                .offset((sLTP_buf_idx - lag + LTP_ORDER / 2 as libc::c_int) as isize)
                as *mut i32;
            i = 0 as libc::c_int;
            while i < (*psDec).subfr_length {
                LTP_pred_Q13 = 2 as libc::c_int;
                LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                    + (*pred_lag_ptr.offset(0 as libc::c_int as isize) as libc::c_long
                        * *B_Q14.offset(0 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                    + (*pred_lag_ptr.offset(-(1 as libc::c_int) as isize) as libc::c_long
                        * *B_Q14.offset(1 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                    + (*pred_lag_ptr.offset(-(2 as libc::c_int) as isize) as libc::c_long
                        * *B_Q14.offset(2 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                    + (*pred_lag_ptr.offset(-(3 as libc::c_int) as isize) as libc::c_long
                        * *B_Q14.offset(3 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                LTP_pred_Q13 = (LTP_pred_Q13 as libc::c_long
                    + (*pred_lag_ptr.offset(-(4 as libc::c_int) as isize) as libc::c_long
                        * *B_Q14.offset(4 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                pred_lag_ptr = pred_lag_ptr.offset(1);
                *pres_Q14.offset(i as isize) = *pexc_Q14.offset(i as isize)
                    + ((LTP_pred_Q13 as u32) << 1 as libc::c_int) as i32;
                *sLTP_Q15.as_mut_ptr().offset(sLTP_buf_idx as isize) =
                    ((*pres_Q14.offset(i as isize) as u32) << 1 as libc::c_int) as i32;
                sLTP_buf_idx += 1;
                i += 1;
            }
        } else {
            pres_Q14 = pexc_Q14;
        }
        i = 0 as libc::c_int;
        while i < (*psDec).subfr_length {
            if !((*psDec).LPC_order == 10 as libc::c_int || (*psDec).LPC_order == 16 as libc::c_int)
            {
                celt_fatal(
                    b"assertion failed: psDec->LPC_order == 10 || psDec->LPC_order == 16\0"
                        as *const u8 as *const libc::c_char,
                    b"silk/decode_core.c\0" as *const u8 as *const libc::c_char,
                    199 as libc::c_int,
                );
            }
            LPC_pred_Q10 = (*psDec).LPC_order >> 1 as libc::c_int;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 1 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12_tmp[0 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 2 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12_tmp[1 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 3 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12_tmp[2 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 4 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12_tmp[3 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 5 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12_tmp[4 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 6 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12_tmp[5 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 7 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12_tmp[6 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 8 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12_tmp[7 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 9 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12_tmp[8 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                + (*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i - 10 as libc::c_int) as isize)
                    as libc::c_long
                    * A_Q12_tmp[9 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            if (*psDec).LPC_order == 16 as libc::c_int {
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*sLPC_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 11 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12_tmp[10 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*sLPC_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 12 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12_tmp[11 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*sLPC_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 13 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12_tmp[12 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*sLPC_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 14 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12_tmp[13 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*sLPC_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 15 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12_tmp[14 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
                LPC_pred_Q10 = (LPC_pred_Q10 as libc::c_long
                    + (*sLPC_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i - 16 as libc::c_int) as isize)
                        as libc::c_long
                        * A_Q12_tmp[15 as libc::c_int as usize] as i64
                        >> 16 as libc::c_int)) as i32;
            }
            *sLPC_Q14.as_mut_ptr().offset((MAX_LPC_ORDER + i) as isize) = if (*pres_Q14
                .offset(i as isize)
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
                if (*pres_Q14.offset(i as isize)
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
                    *pres_Q14.offset(i as isize)
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
            } else if (*pres_Q14.offset(i as isize)
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
                *pres_Q14.offset(i as isize)
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
            *pxq.offset(i as isize) = (if (if 8 as libc::c_int == 1 as libc::c_int {
                ((*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i) as isize) as i64
                    * Gain_Q10 as libc::c_long
                    >> 16 as libc::c_int) as i32
                    >> 1 as libc::c_int)
                    + ((*sLPC_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i) as isize) as i64
                        * Gain_Q10 as libc::c_long
                        >> 16 as libc::c_int) as i32
                        & 1 as libc::c_int)
            } else {
                ((*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i) as isize) as i64
                    * Gain_Q10 as libc::c_long
                    >> 16 as libc::c_int) as i32
                    >> 8 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > silk_int16_MAX
            {
                silk_int16_MAX
            } else if (if 8 as libc::c_int == 1 as libc::c_int {
                ((*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i) as isize) as i64
                    * Gain_Q10 as libc::c_long
                    >> 16 as libc::c_int) as i32
                    >> 1 as libc::c_int)
                    + ((*sLPC_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i) as isize) as i64
                        * Gain_Q10 as libc::c_long
                        >> 16 as libc::c_int) as i32
                        & 1 as libc::c_int)
            } else {
                ((*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i) as isize) as i64
                    * Gain_Q10 as libc::c_long
                    >> 16 as libc::c_int) as i32
                    >> 8 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < silk_int16_MIN
            {
                silk_int16_MIN
            } else if 8 as libc::c_int == 1 as libc::c_int {
                ((*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i) as isize) as i64
                    * Gain_Q10 as libc::c_long
                    >> 16 as libc::c_int) as i32
                    >> 1 as libc::c_int)
                    + ((*sLPC_Q14
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + i) as isize) as i64
                        * Gain_Q10 as libc::c_long
                        >> 16 as libc::c_int) as i32
                        & 1 as libc::c_int)
            } else {
                ((*sLPC_Q14
                    .as_mut_ptr()
                    .offset((16 as libc::c_int + i) as isize) as i64
                    * Gain_Q10 as libc::c_long
                    >> 16 as libc::c_int) as i32
                    >> 8 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as i16;
            i += 1;
        }
        memcpy(
            sLPC_Q14.as_mut_ptr() as *mut libc::c_void,
            &mut *sLPC_Q14.as_mut_ptr().offset((*psDec).subfr_length as isize) as *mut i32
                as *const libc::c_void,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
        pexc_Q14 = pexc_Q14.offset((*psDec).subfr_length as isize);
        pxq = pxq.offset((*psDec).subfr_length as isize);
        k += 1;
    }
    memcpy(
        ((*psDec).sLPC_Q14_buf).as_mut_ptr() as *mut libc::c_void,
        sLPC_Q14.as_mut_ptr() as *const libc::c_void,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
}
