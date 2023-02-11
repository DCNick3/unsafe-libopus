use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:9"]
    pub struct silk_nsq_state {
        pub xq: [i16; 640],
        pub sLTP_shp_Q14: [i32; 640],
        pub sLPC_Q14: [i32; 96],
        pub sAR2_Q14: [i32; 24],
        pub sLF_AR_shp_Q14: i32,
        pub sDiff_shp_Q14: i32,
        pub lagPrev: libc::c_int,
        pub sLTP_buf_idx: libc::c_int,
        pub sLTP_shp_buf_idx: libc::c_int,
        pub rand_seed: i32,
        pub prev_gain_Q16: i32,
        pub rewhite_flag: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:9"]
    pub struct silk_VAD_state {
        pub AnaState: [i32; 2],
        pub AnaState1: [i32; 2],
        pub AnaState2: [i32; 2],
        pub XnrgSubfr: [i32; 4],
        pub NrgRatioSmth_Q8: [i32; 4],
        pub HPstate: i16,
        pub NL: [i32; 4],
        pub inv_NL: [i32; 4],
        pub NoiseLevelBias: [i32; 4],
        pub counter: i32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:9"]
    pub struct silk_LP_state {
        pub In_LP_State: [i32; 2],
        pub transition_frame_no: i32,
        pub mode: libc::c_int,
        pub saved_fs_kHz: i32,
    }
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
    #[c2rust::src_loc = "135:9"]
    pub struct silk_encoder_state {
        pub In_HP_State: [i32; 2],
        pub variable_HP_smth1_Q15: i32,
        pub variable_HP_smth2_Q15: i32,
        pub sLP: silk_LP_state,
        pub sVAD: silk_VAD_state,
        pub sNSQ: silk_nsq_state,
        pub prev_NLSFq_Q15: [i16; 16],
        pub speech_activity_Q8: libc::c_int,
        pub allow_bandwidth_switch: libc::c_int,
        pub LBRRprevLastGainIndex: i8,
        pub prevSignalType: i8,
        pub prevLag: libc::c_int,
        pub pitch_LPC_win_length: libc::c_int,
        pub max_pitch_lag: libc::c_int,
        pub API_fs_Hz: i32,
        pub prev_API_fs_Hz: i32,
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
        pub TargetRate_bps: i32,
        pub PacketSize_ms: libc::c_int,
        pub PacketLoss_perc: libc::c_int,
        pub frameCounter: i32,
        pub Complexity: libc::c_int,
        pub nStatesDelayedDecision: libc::c_int,
        pub useInterpolatedNLSFs: libc::c_int,
        pub shapingLPCOrder: libc::c_int,
        pub predictLPCOrder: libc::c_int,
        pub pitchEstimationComplexity: libc::c_int,
        pub pitchEstimationLPCOrder: libc::c_int,
        pub pitchEstimationThreshold_Q16: i32,
        pub sum_log_gain_Q7: i32,
        pub NLSF_MSVQ_Survivors: libc::c_int,
        pub first_frame_after_reset: libc::c_int,
        pub controlled_since_last_payload: libc::c_int,
        pub warping_Q16: libc::c_int,
        pub useCBR: libc::c_int,
        pub prefillFlag: libc::c_int,
        pub pitch_lag_low_bits_iCDF: *const u8,
        pub pitch_contour_iCDF: *const u8,
        pub psNLSF_CB: *const silk_NLSF_CB_struct,
        pub input_quality_bands_Q15: [libc::c_int; 4],
        pub input_tilt_Q15: libc::c_int,
        pub SNR_dB_Q7: libc::c_int,
        pub VAD_flags: [i8; 3],
        pub LBRR_flag: i8,
        pub LBRR_flags: [libc::c_int; 3],
        pub indices: SideInfoIndices,
        pub pulses: [i8; 320],
        pub arch: libc::c_int,
        pub inputBuf: [i16; 322],
        pub inputBufIx: libc::c_int,
        pub nFramesPerPacket: libc::c_int,
        pub nFramesEncoded: libc::c_int,
        pub nChannelsAPI: libc::c_int,
        pub nChannelsInternal: libc::c_int,
        pub channelNb: libc::c_int,
        pub frames_since_onset: libc::c_int,
        pub ec_prevSignalType: libc::c_int,
        pub ec_prevLagIndex: i16,
        pub resampler_state: silk_resampler_state_struct,
        pub useDTX: libc::c_int,
        pub inDTX: libc::c_int,
        pub noSpeechCounter: libc::c_int,
        pub useInBandFEC: libc::c_int,
        pub LBRR_enabled: libc::c_int,
        pub LBRR_GainIncreases: libc::c_int,
        pub indices_LBRR: [SideInfoIndices; 3],
        pub pulses_LBRR: [[i8; 320]; 3],
    }
    use crate::silk::resampler_structs::silk_resampler_state_struct;
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
    #[c2rust::src_loc = "564:1"]
    pub unsafe extern "C" fn silk_max_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
        return if a > b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "572:1"]
    pub unsafe extern "C" fn silk_max_32(a: i32, b: i32) -> i32 {
        return if a > b { a } else { b };
    }
    extern "C" {
        #[c2rust::src_loc = "154:1"]
        pub fn silk_ana_filt_bank_1(
            in_0: *const i16,
            S: *mut i32,
            outL: *mut i16,
            outH: *mut i16,
            N: i32,
        );
        #[c2rust::src_loc = "176:1"]
        pub fn silk_lin2log(inLin: i32) -> i32;
        #[c2rust::src_loc = "181:1"]
        pub fn silk_sigm_Q15(in_Q5: libc::c_int) -> libc::c_int;
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
    #[c2rust::src_loc = "185:9"]
    pub const VAD_N_BANDS: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "194:9"]
    pub const VAD_NEGATIVE_OFFSET_Q5: libc::c_int = 128 as libc::c_int;
    #[c2rust::src_loc = "190:9"]
    pub const VAD_NOISE_LEVEL_SMOOTH_COEF_Q16: libc::c_int = 1024 as libc::c_int;
    #[c2rust::src_loc = "187:9"]
    pub const VAD_INTERNAL_SUBFRAMES_LOG2: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "188:9"]
    pub const VAD_INTERNAL_SUBFRAMES: libc::c_int =
        (1 as libc::c_int) << VAD_INTERNAL_SUBFRAMES_LOG2;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "48:9"]
    pub const silk_uint8_MAX: libc::c_int = 0xff as libc::c_int;
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
pub use self::define_h::{
    VAD_INTERNAL_SUBFRAMES, VAD_INTERNAL_SUBFRAMES_LOG2, VAD_NEGATIVE_OFFSET_Q5,
    VAD_NOISE_LEVEL_SMOOTH_COEF_Q16, VAD_N_BANDS,
};
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
pub use self::macros_h::silk_CLZ32;
use crate::celt::celt::celt_fatal;

pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    SideInfoIndices,
};
pub use self::typedef_h::{silk_int32_MAX, silk_uint8_MAX};

pub use self::Inlines_h::{silk_CLZ_FRAC, silk_SQRT_APPROX};
pub use self::SigProc_FIX_h::{
    silk_ROR32, silk_ana_filt_bank_1, silk_lin2log, silk_max_32, silk_max_int, silk_min_int,
    silk_sigm_Q15,
};
use crate::externs::memset;
#[no_mangle]
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
#[no_mangle]
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
