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
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
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
    extern "C" {
        #[c2rust::src_loc = "286:1"]
        pub fn silk_NLSF2A(a_Q12: *mut i16, NLSF: *const i16, d: libc::c_int, arch: libc::c_int);
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
    use super::macros_h::silk_CLZ32;
    use super::SigProc_FIX_h::silk_ROR32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "70:9"]
    pub const TYPE_NO_VOICE_ACTIVITY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "142:9"]
    pub const MAX_LPC_ORDER: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "226:9"]
    pub const CNG_BUF_MASK_MAX: libc::c_int = 255 as libc::c_int;
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
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
use self::arch_h::celt_fatal;
pub use self::define_h::{CNG_BUF_MASK_MAX, MAX_LPC_ORDER, TYPE_NO_VOICE_ACTIVITY};
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
pub use self::macros_h::silk_CLZ32;
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::string_h::{memcpy, memmove, memset};
pub use self::structs_h::{
    silk_CNG_struct, silk_NLSF_CB_struct, silk_PLC_struct, silk_decoder_control,
    silk_decoder_state, SideInfoIndices,
};
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN, silk_int32_MAX, silk_int32_MIN};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __int8_t, __uint32_t, __uint8_t};
pub use self::Inlines_h::{silk_CLZ_FRAC, silk_SQRT_APPROX};
pub use self::SigProc_FIX_h::{silk_NLSF2A, silk_ROR32};
#[inline]
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn silk_CNG_exc(
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
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn silk_CNG_Reset(mut psDec: *mut silk_decoder_state) {
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
#[no_mangle]
#[c2rust::src_loc = "79:1"]
pub unsafe extern "C" fn silk_CNG(
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
