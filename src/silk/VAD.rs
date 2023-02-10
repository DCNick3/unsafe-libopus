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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    #[c2rust::src_loc = "52:4"]
    pub type opus_uint8 = uint8_t;
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
    use super::stdint_uintn_h::{uint32_t, uint8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:32"]
pub mod resampler_structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [opus_int32; 6],
        pub sFIR: C2RustUnnamed,
        pub delayBuf: [opus_int16; 48],
        pub resampler_function: libc::c_int,
        pub batchSize: libc::c_int,
        pub invRatio_Q16: opus_int32,
        pub FIR_Order: libc::c_int,
        pub FIR_Fracs: libc::c_int,
        pub Fs_in_kHz: libc::c_int,
        pub Fs_out_kHz: libc::c_int,
        pub inputDelay: libc::c_int,
        pub Coefs: *const opus_int16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:5"]
    pub union C2RustUnnamed {
        pub i32_0: [opus_int32; 36],
        pub i16_0: [opus_int16; 36],
    }
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
    use super::opus_types_h::{opus_int16, opus_int32};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:9"]
    pub struct silk_nsq_state {
        pub xq: [opus_int16; 640],
        pub sLTP_shp_Q14: [opus_int32; 640],
        pub sLPC_Q14: [opus_int32; 96],
        pub sAR2_Q14: [opus_int32; 24],
        pub sLF_AR_shp_Q14: opus_int32,
        pub sDiff_shp_Q14: opus_int32,
        pub lagPrev: libc::c_int,
        pub sLTP_buf_idx: libc::c_int,
        pub sLTP_shp_buf_idx: libc::c_int,
        pub rand_seed: opus_int32,
        pub prev_gain_Q16: opus_int32,
        pub rewhite_flag: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:9"]
    pub struct silk_VAD_state {
        pub AnaState: [opus_int32; 2],
        pub AnaState1: [opus_int32; 2],
        pub AnaState2: [opus_int32; 2],
        pub XnrgSubfr: [opus_int32; 4],
        pub NrgRatioSmth_Q8: [opus_int32; 4],
        pub HPstate: opus_int16,
        pub NL: [opus_int32; 4],
        pub inv_NL: [opus_int32; 4],
        pub NoiseLevelBias: [opus_int32; 4],
        pub counter: opus_int32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:9"]
    pub struct silk_LP_state {
        pub In_LP_State: [opus_int32; 2],
        pub transition_frame_no: opus_int32,
        pub mode: libc::c_int,
        pub saved_fs_kHz: opus_int32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:9"]
    pub struct silk_NLSF_CB_struct {
        pub nVectors: opus_int16,
        pub order: opus_int16,
        pub quantStepSize_Q16: opus_int16,
        pub invQuantStepSize_Q6: opus_int16,
        pub CB1_NLSF_Q8: *const opus_uint8,
        pub CB1_Wght_Q9: *const opus_int16,
        pub CB1_iCDF: *const opus_uint8,
        pub pred_Q8: *const opus_uint8,
        pub ec_sel: *const opus_uint8,
        pub ec_iCDF: *const opus_uint8,
        pub ec_Rates_Q5: *const opus_uint8,
        pub deltaMin_Q15: *const opus_int16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:9"]
    pub struct SideInfoIndices {
        pub GainsIndices: [opus_int8; 4],
        pub LTPIndex: [opus_int8; 4],
        pub NLSFIndices: [opus_int8; 17],
        pub lagIndex: opus_int16,
        pub contourIndex: opus_int8,
        pub signalType: opus_int8,
        pub quantOffsetType: opus_int8,
        pub NLSFInterpCoef_Q2: opus_int8,
        pub PERIndex: opus_int8,
        pub LTP_scaleIndex: opus_int8,
        pub Seed: opus_int8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:9"]
    pub struct silk_encoder_state {
        pub In_HP_State: [opus_int32; 2],
        pub variable_HP_smth1_Q15: opus_int32,
        pub variable_HP_smth2_Q15: opus_int32,
        pub sLP: silk_LP_state,
        pub sVAD: silk_VAD_state,
        pub sNSQ: silk_nsq_state,
        pub prev_NLSFq_Q15: [opus_int16; 16],
        pub speech_activity_Q8: libc::c_int,
        pub allow_bandwidth_switch: libc::c_int,
        pub LBRRprevLastGainIndex: opus_int8,
        pub prevSignalType: opus_int8,
        pub prevLag: libc::c_int,
        pub pitch_LPC_win_length: libc::c_int,
        pub max_pitch_lag: libc::c_int,
        pub API_fs_Hz: opus_int32,
        pub prev_API_fs_Hz: opus_int32,
        pub maxInternal_fs_Hz: libc::c_int,
        pub minInternal_fs_Hz: libc::c_int,
        pub desiredInternal_fs_Hz: libc::c_int,
        pub fs_kHz: libc::c_int,
        pub nb_subfr: libc::c_int,
        pub frame_length: libc::c_int,
        pub subfr_length: libc::c_int,
        pub ltp_mem_length: libc::c_int,
        pub la_pitch: libc::c_int,
        pub la_shape: libc::c_int,
        pub shapeWinLength: libc::c_int,
        pub TargetRate_bps: opus_int32,
        pub PacketSize_ms: libc::c_int,
        pub PacketLoss_perc: libc::c_int,
        pub frameCounter: opus_int32,
        pub Complexity: libc::c_int,
        pub nStatesDelayedDecision: libc::c_int,
        pub useInterpolatedNLSFs: libc::c_int,
        pub shapingLPCOrder: libc::c_int,
        pub predictLPCOrder: libc::c_int,
        pub pitchEstimationComplexity: libc::c_int,
        pub pitchEstimationLPCOrder: libc::c_int,
        pub pitchEstimationThreshold_Q16: opus_int32,
        pub sum_log_gain_Q7: opus_int32,
        pub NLSF_MSVQ_Survivors: libc::c_int,
        pub first_frame_after_reset: libc::c_int,
        pub controlled_since_last_payload: libc::c_int,
        pub warping_Q16: libc::c_int,
        pub useCBR: libc::c_int,
        pub prefillFlag: libc::c_int,
        pub pitch_lag_low_bits_iCDF: *const opus_uint8,
        pub pitch_contour_iCDF: *const opus_uint8,
        pub psNLSF_CB: *const silk_NLSF_CB_struct,
        pub input_quality_bands_Q15: [libc::c_int; 4],
        pub input_tilt_Q15: libc::c_int,
        pub SNR_dB_Q7: libc::c_int,
        pub VAD_flags: [opus_int8; 3],
        pub LBRR_flag: opus_int8,
        pub LBRR_flags: [libc::c_int; 3],
        pub indices: SideInfoIndices,
        pub pulses: [opus_int8; 320],
        pub arch: libc::c_int,
        pub inputBuf: [opus_int16; 322],
        pub inputBufIx: libc::c_int,
        pub nFramesPerPacket: libc::c_int,
        pub nFramesEncoded: libc::c_int,
        pub nChannelsAPI: libc::c_int,
        pub nChannelsInternal: libc::c_int,
        pub channelNb: libc::c_int,
        pub frames_since_onset: libc::c_int,
        pub ec_prevSignalType: libc::c_int,
        pub ec_prevLagIndex: opus_int16,
        pub resampler_state: silk_resampler_state_struct,
        pub useDTX: libc::c_int,
        pub inDTX: libc::c_int,
        pub noSpeechCounter: libc::c_int,
        pub useInBandFEC: libc::c_int,
        pub LBRR_enabled: libc::c_int,
        pub LBRR_GainIncreases: libc::c_int,
        pub indices_LBRR: [SideInfoIndices; 3],
        pub pulses_LBRR: [[opus_int8; 320]; 3],
    }
    use super::opus_types_h::{opus_int16, opus_int32, opus_int8, opus_uint8};
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
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/macros.h:32"]
pub mod macros_h {
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn silk_CLZ32(mut in32: opus_int32) -> opus_int32 {
        return if in32 != 0 {
            32 as libc::c_int
                - (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }
    use super::opus_types_h::opus_int32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "398:1"]
    pub unsafe extern "C" fn silk_ROR32(mut a32: opus_int32, mut rot: libc::c_int) -> opus_int32 {
        let mut x: opus_uint32 = a32 as opus_uint32;
        let mut r: opus_uint32 = rot as opus_uint32;
        let mut m: opus_uint32 = -rot as opus_uint32;
        if rot == 0 as libc::c_int {
            return a32;
        } else if rot < 0 as libc::c_int {
            return (x << m | x >> (32 as libc::c_int as libc::c_uint).wrapping_sub(m))
                as opus_int32;
        } else {
            return (x << (32 as libc::c_int as libc::c_uint).wrapping_sub(r) | x >> r)
                as opus_int32;
        };
    }
    #[inline]
    #[c2rust::src_loc = "546:1"]
    pub unsafe extern "C" fn silk_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
        return if a < b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "564:1"]
    pub unsafe extern "C" fn silk_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
        return if a > b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "572:1"]
    pub unsafe extern "C" fn silk_max_32(mut a: opus_int32, mut b: opus_int32) -> opus_int32 {
        return if a > b { a } else { b };
    }
    use super::opus_types_h::{opus_int16, opus_int32, opus_uint32};
    extern "C" {
        #[c2rust::src_loc = "154:1"]
        pub fn silk_ana_filt_bank_1(
            in_0: *const opus_int16,
            S: *mut opus_int32,
            outL: *mut opus_int16,
            outH: *mut opus_int16,
            N: opus_int32,
        );
        #[c2rust::src_loc = "176:1"]
        pub fn silk_lin2log(inLin: opus_int32) -> opus_int32;
        #[c2rust::src_loc = "181:1"]
        pub fn silk_sigm_Q15(in_Q5: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/Inlines.h:32"]
pub mod Inlines_h {
    #[inline]
    #[c2rust::src_loc = "56:1"]
    pub unsafe extern "C" fn silk_CLZ_FRAC(
        mut in_0: opus_int32,
        mut lz: *mut opus_int32,
        mut frac_Q7: *mut opus_int32,
    ) {
        let mut lzeros: opus_int32 = silk_CLZ32(in_0);
        *lz = lzeros;
        *frac_Q7 = silk_ROR32(in_0, 24 as libc::c_int - lzeros) & 0x7f as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "71:1"]
    pub unsafe extern "C" fn silk_SQRT_APPROX(mut x: opus_int32) -> opus_int32 {
        let mut y: opus_int32 = 0;
        let mut lz: opus_int32 = 0;
        let mut frac_Q7: opus_int32 = 0;
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
                * (213 as libc::c_int as opus_int16 as opus_int32
                    * frac_Q7 as opus_int16 as opus_int32) as opus_int16
                    as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        return y;
    }
    use super::macros_h::silk_CLZ32;
    use super::opus_types_h::{opus_int16, opus_int32, opus_int64};
    use super::SigProc_FIX_h::silk_ROR32;
}
use self::arch_h::celt_fatal;
pub use self::macros_h::silk_CLZ32;
pub use self::opus_types_h::{
    opus_int16, opus_int32, opus_int64, opus_int8, opus_uint32, opus_uint8,
};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::string_h::memset;
pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    SideInfoIndices,
};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __int8_t, __uint32_t, __uint8_t};
pub use self::Inlines_h::{silk_CLZ_FRAC, silk_SQRT_APPROX};
pub use self::SigProc_FIX_h::{
    silk_ROR32, silk_ana_filt_bank_1, silk_lin2log, silk_max_32, silk_max_int, silk_min_int,
    silk_sigm_Q15,
};
#[no_mangle]
#[c2rust::src_loc = "46:1"]
pub unsafe extern "C" fn silk_VAD_Init(mut psSilk_VAD: *mut silk_VAD_state) -> libc::c_int {
    let mut b: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    memset(
        psSilk_VAD as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<silk_VAD_state>() as libc::c_ulong,
    );
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        (*psSilk_VAD).NoiseLevelBias[b as usize] =
            silk_max_32(50 as libc::c_int / (b + 1 as libc::c_int), 1 as libc::c_int);
        b += 1;
    }
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        (*psSilk_VAD).NL[b as usize] =
            100 as libc::c_int * (*psSilk_VAD).NoiseLevelBias[b as usize];
        (*psSilk_VAD).inv_NL[b as usize] = 0x7fffffff as libc::c_int / (*psSilk_VAD).NL[b as usize];
        b += 1;
    }
    (*psSilk_VAD).counter = 15 as libc::c_int;
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        (*psSilk_VAD).NrgRatioSmth_Q8[b as usize] = 100 as libc::c_int * 256 as libc::c_int;
        b += 1;
    }
    return ret;
}
#[c2rust::src_loc = "77:25"]
static mut tiltWeights: [opus_int32; 4] = [
    30000 as libc::c_int,
    6000 as libc::c_int,
    -(12000 as libc::c_int),
    -(12000 as libc::c_int),
];
#[no_mangle]
#[c2rust::src_loc = "82:1"]
pub unsafe extern "C" fn silk_VAD_GetSA_Q8_c(
    mut psEncC: *mut silk_encoder_state,
    mut pIn: *const opus_int16,
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
    let mut sumSquared: opus_int32 = 0;
    let mut smooth_coef_Q16: opus_int32 = 0;
    let mut HPstateTmp: opus_int16 = 0;
    let mut Xnrg: [opus_int32; 4] = [0; 4];
    let mut NrgToNoiseRatio_Q8: [opus_int32; 4] = [0; 4];
    let mut speech_nrg: opus_int32 = 0;
    let mut x_tmp: opus_int32 = 0;
    let mut X_offset: [libc::c_int; 4] = [0; 4];
    let mut ret: libc::c_int = 0 as libc::c_int;
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
    let mut X: Vec<opus_int16> = ::std::vec::from_elem(0, vla);
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
            >> 1 as libc::c_int) as opus_int16;
    HPstateTmp = *X
        .as_mut_ptr()
        .offset((decimated_framelength - 1 as libc::c_int) as isize);
    i = decimated_framelength - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        *X.as_mut_ptr().offset((i - 1 as libc::c_int) as isize) =
            (*X.as_mut_ptr().offset((i - 1 as libc::c_int) as isize) as libc::c_int
                >> 1 as libc::c_int) as opus_int16;
        let ref mut fresh0 = *X.as_mut_ptr().offset(i as isize);
        *fresh0 = (*fresh0 as libc::c_int
            - *X.as_mut_ptr().offset((i - 1 as libc::c_int) as isize) as libc::c_int)
            as opus_int16;
        i -= 1;
    }
    let ref mut fresh1 = *X.as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh1 = (*fresh1 as libc::c_int - (*psSilk_VAD).HPstate as libc::c_int) as opus_int16;
    (*psSilk_VAD).HPstate = HPstateTmp;
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        decimated_framelength = (*psEncC).frame_length
            >> silk_min_int(4 as libc::c_int - b, 4 as libc::c_int - 1 as libc::c_int);
        dec_subframe_length = decimated_framelength >> 2 as libc::c_int;
        dec_subframe_offset = 0 as libc::c_int;
        Xnrg[b as usize] = (*psSilk_VAD).XnrgSubfr[b as usize];
        s = 0 as libc::c_int;
        while s < (1 as libc::c_int) << 2 as libc::c_int {
            sumSquared = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < dec_subframe_length {
                x_tmp = *X
                    .as_mut_ptr()
                    .offset((X_offset[b as usize] + i + dec_subframe_offset) as isize)
                    as libc::c_int
                    >> 3 as libc::c_int;
                sumSquared = sumSquared
                    + x_tmp as opus_int16 as opus_int32 * x_tmp as opus_int16 as opus_int32;
                i += 1;
            }
            if s < ((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int {
                Xnrg[b as usize] = if (Xnrg[b as usize] as opus_uint32)
                    .wrapping_add(sumSquared as opus_uint32)
                    & 0x80000000 as libc::c_uint
                    != 0
                {
                    0x7fffffff as libc::c_int
                } else {
                    Xnrg[b as usize] + sumSquared
                };
            } else {
                Xnrg[b as usize] = if (Xnrg[b as usize] as opus_uint32)
                    .wrapping_add((sumSquared >> 1 as libc::c_int) as opus_uint32)
                    & 0x80000000 as libc::c_uint
                    != 0
                {
                    0x7fffffff as libc::c_int
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
        &mut *Xnrg.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut opus_int32
            as *const opus_int32,
        psSilk_VAD,
    );
    sumSquared = 0 as libc::c_int;
    input_tilt = 0 as libc::c_int;
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        speech_nrg = Xnrg[b as usize] - (*psSilk_VAD).NL[b as usize];
        if speech_nrg > 0 as libc::c_int {
            if Xnrg[b as usize] as libc::c_uint & 0xff800000 as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                NrgToNoiseRatio_Q8[b as usize] =
                    ((Xnrg[b as usize] as opus_uint32) << 8 as libc::c_int) as opus_int32
                        / ((*psSilk_VAD).NL[b as usize] + 1 as libc::c_int);
            } else {
                NrgToNoiseRatio_Q8[b as usize] = Xnrg[b as usize]
                    / (((*psSilk_VAD).NL[b as usize] >> 8 as libc::c_int) + 1 as libc::c_int);
            }
            SNR_Q7 = silk_lin2log(NrgToNoiseRatio_Q8[b as usize])
                - 8 as libc::c_int * 128 as libc::c_int;
            sumSquared = sumSquared
                + SNR_Q7 as opus_int16 as opus_int32 * SNR_Q7 as opus_int16 as opus_int32;
            if speech_nrg < (1 as libc::c_int) << 20 as libc::c_int {
                SNR_Q7 = (((silk_SQRT_APPROX(speech_nrg) as opus_uint32) << 6 as libc::c_int)
                    as opus_int32 as libc::c_long
                    * SNR_Q7 as opus_int16 as opus_int64
                    >> 16 as libc::c_int) as opus_int32;
            }
            input_tilt = (input_tilt as libc::c_long
                + (tiltWeights[b as usize] as libc::c_long * SNR_Q7 as opus_int16 as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
        } else {
            NrgToNoiseRatio_Q8[b as usize] = 256 as libc::c_int;
        }
        b += 1;
    }
    sumSquared = sumSquared / 4 as libc::c_int;
    pSNR_dB_Q7 = (3 as libc::c_int * silk_SQRT_APPROX(sumSquared)) as opus_int16 as libc::c_int;
    SA_Q15 = silk_sigm_Q15(
        (45000 as libc::c_int as libc::c_long * pSNR_dB_Q7 as opus_int16 as opus_int64
            >> 16 as libc::c_int) as opus_int32
            - 128 as libc::c_int,
    );
    (*psEncC).input_tilt_Q15 = (((silk_sigm_Q15(input_tilt) - 16384 as libc::c_int) as opus_uint32)
        << 1 as libc::c_int) as opus_int32;
    speech_nrg = 0 as libc::c_int;
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
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
        speech_nrg = ((speech_nrg as opus_uint32) << 16 as libc::c_int) as opus_int32;
        speech_nrg = silk_SQRT_APPROX(speech_nrg);
        SA_Q15 = ((32768 as libc::c_int + speech_nrg) as libc::c_long
            * SA_Q15 as opus_int16 as opus_int64
            >> 16 as libc::c_int) as opus_int32;
    }
    (*psEncC).speech_activity_Q8 = silk_min_int(SA_Q15 >> 7 as libc::c_int, 0xff as libc::c_int);
    smooth_coef_Q16 = (4096 as libc::c_int as libc::c_long
        * (SA_Q15 as libc::c_long * SA_Q15 as opus_int16 as opus_int64 >> 16 as libc::c_int)
            as opus_int32 as opus_int16 as opus_int64
        >> 16 as libc::c_int) as opus_int32;
    if (*psEncC).frame_length == 10 as libc::c_int * (*psEncC).fs_kHz {
        smooth_coef_Q16 >>= 1 as libc::c_int;
    }
    b = 0 as libc::c_int;
    while b < 4 as libc::c_int {
        (*psSilk_VAD).NrgRatioSmth_Q8[b as usize] =
            ((*psSilk_VAD).NrgRatioSmth_Q8[b as usize] as libc::c_long
                + ((NrgToNoiseRatio_Q8[b as usize] - (*psSilk_VAD).NrgRatioSmth_Q8[b as usize])
                    as libc::c_long
                    * smooth_coef_Q16 as opus_int16 as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
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
unsafe extern "C" fn silk_VAD_GetNoiseLevels(
    mut pX: *const opus_int32,
    mut psSilk_VAD: *mut silk_VAD_state,
) {
    let mut k: libc::c_int = 0;
    let mut nl: opus_int32 = 0;
    let mut nrg: opus_int32 = 0;
    let mut inv_nrg: opus_int32 = 0;
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
    while k < 4 as libc::c_int {
        nl = (*psSilk_VAD).NL[k as usize];
        nrg = if (*pX.offset(k as isize) as opus_uint32)
            .wrapping_add((*psSilk_VAD).NoiseLevelBias[k as usize] as opus_uint32)
            & 0x80000000 as libc::c_uint
            != 0
        {
            0x7fffffff as libc::c_int
        } else {
            *pX.offset(k as isize) + (*psSilk_VAD).NoiseLevelBias[k as usize]
        };
        inv_nrg = 0x7fffffff as libc::c_int / nrg;
        if nrg > ((nl as opus_uint32) << 3 as libc::c_int) as opus_int32 {
            coef = 1024 as libc::c_int >> 3 as libc::c_int;
        } else if nrg < nl {
            coef = 1024 as libc::c_int;
        } else {
            coef = ((inv_nrg as opus_int64 * nl as libc::c_long >> 16 as libc::c_int) as opus_int32
                as libc::c_long
                * ((1024 as libc::c_int) << 1 as libc::c_int) as opus_int16 as opus_int64
                >> 16 as libc::c_int) as opus_int32;
        }
        coef = silk_max_int(coef, min_coef);
        (*psSilk_VAD).inv_NL[k as usize] = ((*psSilk_VAD).inv_NL[k as usize] as libc::c_long
            + ((inv_nrg - (*psSilk_VAD).inv_NL[k as usize]) as libc::c_long
                * coef as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
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
