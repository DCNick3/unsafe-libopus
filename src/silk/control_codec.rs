pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
}
pub mod errors_h {
    pub const SILK_ENC_PACKET_SIZE_NOT_SUPPORTED: i32 = -(103 as i32);
    pub const SILK_NO_ERROR: i32 = 0 as i32;
}
pub mod SigProc_FLP_h {
    #[inline]
    pub unsafe fn silk_float2short_array(out: *mut i16, in_0: *const f32, length: i32) {
        let mut k: i32 = 0;
        k = length - 1 as i32;
        while k >= 0 as i32 {
            *out.offset(k as isize) = (if float2int(*in_0.offset(k as isize)) > silk_int16_MAX {
                silk_int16_MAX
            } else if float2int(*in_0.offset(k as isize)) < silk_int16_MIN {
                silk_int16_MIN
            } else {
                float2int(*in_0.offset(k as isize))
            }) as i16;
            k -= 1;
        }
    }
    #[inline]
    pub unsafe fn silk_short2float_array(out: *mut f32, in_0: *const i16, length: i32) {
        let mut k: i32 = 0;
        k = length - 1 as i32;
        while k >= 0 as i32 {
            *out.offset(k as isize) = *in_0.offset(k as isize) as f32;
            k -= 1;
        }
    }
    use super::typedef_h::{silk_int16_MAX, silk_int16_MIN};
    use crate::celt::float_cast::float2int;
}
pub mod typedef_h {
    pub const silk_int16_MIN: i32 = 0x8000 as i32;
    pub const silk_int16_MAX: i32 = 0x7fff as i32;
}
use self::errors_h::{SILK_ENC_PACKET_SIZE_NOT_SUPPORTED, SILK_NO_ERROR};
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
pub use self::SigProc_FLP_h::{silk_float2short_array, silk_short2float_array};
use crate::celt::celt::celt_fatal;
use crate::externs::memset;
use crate::silk::control_audio_bandwidth::silk_control_audio_bandwidth;
use crate::silk::define::{
    LA_SHAPE_MS, MAX_DEL_DEC_STATES, MAX_LPC_ORDER, MAX_NB_SUBFR, MIN_LPC_ORDER,
    SUB_FRAME_LENGTH_MS, TYPE_NO_VOICE_ACTIVITY,
};
use crate::silk::enc_API::silk_EncControlStruct;
use crate::silk::float::structs_FLP::{silk_encoder_state_FLP, silk_shape_state_FLP};
use crate::silk::pitch_est_tables::{
    SILK_PE_MAX_COMPLEX, SILK_PE_MID_COMPLEX, SILK_PE_MIN_COMPLEX,
};
use crate::silk::resampler::{silk_resampler, silk_resampler_init};
use crate::silk::resampler_structs::silk_resampler_state_struct;
use crate::silk::structs::{silk_encoder_state, silk_nsq_state};
use crate::silk::tables_NLSF_CB_NB_MB::silk_NLSF_CB_NB_MB;
use crate::silk::tables_NLSF_CB_WB::silk_NLSF_CB_WB;
use crate::silk::tables_other::{silk_uniform4_iCDF, silk_uniform6_iCDF, silk_uniform8_iCDF};
use crate::silk::tables_pitch_lag::{
    silk_pitch_contour_10_ms_NB_iCDF, silk_pitch_contour_10_ms_iCDF, silk_pitch_contour_NB_iCDF,
    silk_pitch_contour_iCDF,
};
use crate::silk::SigProc_FIX::{silk_max_int, silk_min_int};

pub unsafe fn silk_control_encoder(
    mut psEnc: *mut silk_encoder_state_FLP,
    encControl: *mut silk_EncControlStruct,
    allow_bw_switch: i32,
    channelNb: i32,
    force_fs_kHz: i32,
) -> i32 {
    let mut fs_kHz: i32 = 0;
    let mut ret: i32 = 0 as i32;
    (*psEnc).sCmn.useDTX = (*encControl).useDTX;
    (*psEnc).sCmn.useCBR = (*encControl).useCBR;
    (*psEnc).sCmn.API_fs_Hz = (*encControl).API_sampleRate;
    (*psEnc).sCmn.maxInternal_fs_Hz = (*encControl).maxInternalSampleRate;
    (*psEnc).sCmn.minInternal_fs_Hz = (*encControl).minInternalSampleRate;
    (*psEnc).sCmn.desiredInternal_fs_Hz = (*encControl).desiredInternalSampleRate;
    (*psEnc).sCmn.useInBandFEC = (*encControl).useInBandFEC;
    (*psEnc).sCmn.nChannelsAPI = (*encControl).nChannelsAPI;
    (*psEnc).sCmn.nChannelsInternal = (*encControl).nChannelsInternal;
    (*psEnc).sCmn.allow_bandwidth_switch = allow_bw_switch;
    (*psEnc).sCmn.channelNb = channelNb;
    if (*psEnc).sCmn.controlled_since_last_payload != 0 as i32
        && (*psEnc).sCmn.prefillFlag == 0 as i32
    {
        if (*psEnc).sCmn.API_fs_Hz != (*psEnc).sCmn.prev_API_fs_Hz
            && (*psEnc).sCmn.fs_kHz > 0 as i32
        {
            ret += silk_setup_resamplers(psEnc, (*psEnc).sCmn.fs_kHz);
        }
        return ret;
    }
    fs_kHz = silk_control_audio_bandwidth(&mut (*psEnc).sCmn, encControl);
    if force_fs_kHz != 0 {
        fs_kHz = force_fs_kHz;
    }
    ret += silk_setup_resamplers(psEnc, fs_kHz);
    ret += silk_setup_fs(psEnc, fs_kHz, (*encControl).payloadSize_ms);
    ret += silk_setup_complexity(&mut (*psEnc).sCmn, (*encControl).complexity);
    (*psEnc).sCmn.PacketLoss_perc = (*encControl).packetLossPercentage;
    ret += silk_setup_LBRR(&mut (*psEnc).sCmn, encControl);
    (*psEnc).sCmn.controlled_since_last_payload = 1 as i32;
    return ret;
}
unsafe fn silk_setup_resamplers(mut psEnc: *mut silk_encoder_state_FLP, fs_kHz: i32) -> i32 {
    let mut ret: i32 = SILK_NO_ERROR;
    if (*psEnc).sCmn.fs_kHz != fs_kHz || (*psEnc).sCmn.prev_API_fs_Hz != (*psEnc).sCmn.API_fs_Hz {
        if (*psEnc).sCmn.fs_kHz == 0 as i32 {
            ret += silk_resampler_init(
                &mut (*psEnc).sCmn.resampler_state,
                (*psEnc).sCmn.API_fs_Hz,
                fs_kHz * 1000 as i32,
                1 as i32,
            );
        } else {
            let mut new_buf_samples: i32 = 0;
            let mut api_buf_samples: i32 = 0;
            let mut old_buf_samples: i32 = 0;
            let mut buf_length_ms: i32 = 0;
            buf_length_ms =
                ((((*psEnc).sCmn.nb_subfr * 5 as i32) as u32) << 1 as i32) as i32 + LA_SHAPE_MS;
            old_buf_samples = buf_length_ms * (*psEnc).sCmn.fs_kHz;
            new_buf_samples = buf_length_ms * fs_kHz;
            let vla = (if old_buf_samples > new_buf_samples {
                old_buf_samples
            } else {
                new_buf_samples
            }) as usize;
            let mut x_bufFIX: Vec<i16> = ::std::vec::from_elem(0, vla);
            silk_float2short_array(
                x_bufFIX.as_mut_ptr(),
                ((*psEnc).x_buf).as_mut_ptr(),
                old_buf_samples,
            );
            let mut temp_resampler_state: [silk_resampler_state_struct; 1] =
                [silk_resampler_state_struct {
                    sIIR: [0; 6],
                    sFIR: crate::silk::resampler_structs::sFIR_union { i32_0: [0; 36] },
                    delayBuf: [0; 48],
                    resampler_function: 0,
                    batchSize: 0,
                    invRatio_Q16: 0,
                    FIR_Order: 0,
                    FIR_Fracs: 0,
                    Fs_in_kHz: 0,
                    Fs_out_kHz: 0,
                    inputDelay: 0,
                    Coefs: 0 as *const i16,
                }; 1];
            ret += silk_resampler_init(
                temp_resampler_state.as_mut_ptr(),
                (*psEnc).sCmn.fs_kHz as i16 as i32 * 1000 as i32 as i16 as i32,
                (*psEnc).sCmn.API_fs_Hz,
                0 as i32,
            );
            api_buf_samples = buf_length_ms * ((*psEnc).sCmn.API_fs_Hz / 1000 as i32);
            let vla_0 = api_buf_samples as usize;
            let mut x_buf_API_fs_Hz: Vec<i16> = ::std::vec::from_elem(0, vla_0);
            ret += silk_resampler(
                temp_resampler_state.as_mut_ptr(),
                x_buf_API_fs_Hz.as_mut_ptr(),
                x_bufFIX.as_mut_ptr() as *const i16,
                old_buf_samples,
            );
            ret += silk_resampler_init(
                &mut (*psEnc).sCmn.resampler_state,
                (*psEnc).sCmn.API_fs_Hz,
                fs_kHz as i16 as i32 * 1000 as i32 as i16 as i32,
                1 as i32,
            );
            ret += silk_resampler(
                &mut (*psEnc).sCmn.resampler_state,
                x_bufFIX.as_mut_ptr(),
                x_buf_API_fs_Hz.as_mut_ptr() as *const i16,
                api_buf_samples,
            );
            silk_short2float_array(
                ((*psEnc).x_buf).as_mut_ptr(),
                x_bufFIX.as_mut_ptr(),
                new_buf_samples,
            );
        }
    }
    (*psEnc).sCmn.prev_API_fs_Hz = (*psEnc).sCmn.API_fs_Hz;
    return ret;
}
unsafe fn silk_setup_fs(
    mut psEnc: *mut silk_encoder_state_FLP,
    fs_kHz: i32,
    PacketSize_ms: i32,
) -> i32 {
    let mut ret: i32 = SILK_NO_ERROR;
    if PacketSize_ms != (*psEnc).sCmn.PacketSize_ms {
        if PacketSize_ms != 10 as i32
            && PacketSize_ms != 20 as i32
            && PacketSize_ms != 40 as i32
            && PacketSize_ms != 60 as i32
        {
            ret = SILK_ENC_PACKET_SIZE_NOT_SUPPORTED;
        }
        if PacketSize_ms <= 10 as i32 {
            (*psEnc).sCmn.nFramesPerPacket = 1 as i32;
            (*psEnc).sCmn.nb_subfr = if PacketSize_ms == 10 as i32 {
                2 as i32
            } else {
                1 as i32
            };
            (*psEnc).sCmn.frame_length = PacketSize_ms as i16 as i32 * fs_kHz as i16 as i32;
            (*psEnc).sCmn.pitch_LPC_win_length =
                (10 as i32 + ((2 as i32) << 1 as i32)) as i16 as i32 * fs_kHz as i16 as i32;
            if (*psEnc).sCmn.fs_kHz == 8 as i32 {
                (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_10_ms_NB_iCDF.as_ptr();
            } else {
                (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_10_ms_iCDF.as_ptr();
            }
        } else {
            (*psEnc).sCmn.nFramesPerPacket = PacketSize_ms / (5 as i32 * 4 as i32);
            (*psEnc).sCmn.nb_subfr = MAX_NB_SUBFR;
            (*psEnc).sCmn.frame_length = 20 as i32 as i16 as i32 * fs_kHz as i16 as i32;
            (*psEnc).sCmn.pitch_LPC_win_length =
                (20 as i32 + ((2 as i32) << 1 as i32)) as i16 as i32 * fs_kHz as i16 as i32;
            if (*psEnc).sCmn.fs_kHz == 8 as i32 {
                (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_NB_iCDF.as_ptr();
            } else {
                (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_iCDF.as_ptr();
            }
        }
        (*psEnc).sCmn.PacketSize_ms = PacketSize_ms;
        (*psEnc).sCmn.TargetRate_bps = 0 as i32;
    }
    if !(fs_kHz == 8 as i32 || fs_kHz == 12 as i32 || fs_kHz == 16 as i32) {
        celt_fatal(
            b"assertion failed: fs_kHz == 8 || fs_kHz == 12 || fs_kHz == 16\0" as *const u8
                as *const i8,
            b"silk/control_codec.c\0" as *const u8 as *const i8,
            241 as i32,
        );
    }
    if !((*psEnc).sCmn.nb_subfr == 2 as i32 || (*psEnc).sCmn.nb_subfr == 4 as i32) {
        celt_fatal(
            b"assertion failed: psEnc->sCmn.nb_subfr == 2 || psEnc->sCmn.nb_subfr == 4\0"
                as *const u8 as *const i8,
            b"silk/control_codec.c\0" as *const u8 as *const i8,
            242 as i32,
        );
    }
    if (*psEnc).sCmn.fs_kHz != fs_kHz {
        memset(
            &mut (*psEnc).sShape as *mut silk_shape_state_FLP as *mut core::ffi::c_void,
            0 as i32,
            ::core::mem::size_of::<silk_shape_state_FLP>() as u64,
        );
        memset(
            &mut (*psEnc).sCmn.sNSQ as *mut silk_nsq_state as *mut core::ffi::c_void,
            0 as i32,
            ::core::mem::size_of::<silk_nsq_state>() as u64,
        );
        memset(
            ((*psEnc).sCmn.prev_NLSFq_Q15).as_mut_ptr() as *mut core::ffi::c_void,
            0 as i32,
            ::core::mem::size_of::<[i16; 16]>() as u64,
        );
        memset(
            &mut (*psEnc).sCmn.sLP.In_LP_State as *mut [i32; 2] as *mut core::ffi::c_void,
            0 as i32,
            ::core::mem::size_of::<[i32; 2]>() as u64,
        );
        (*psEnc).sCmn.inputBufIx = 0 as i32;
        (*psEnc).sCmn.nFramesEncoded = 0 as i32;
        (*psEnc).sCmn.TargetRate_bps = 0 as i32;
        (*psEnc).sCmn.prevLag = 100 as i32;
        (*psEnc).sCmn.first_frame_after_reset = 1 as i32;
        (*psEnc).sShape.LastGainIndex = 10 as i32 as i8;
        (*psEnc).sCmn.sNSQ.lagPrev = 100 as i32;
        (*psEnc).sCmn.sNSQ.prev_gain_Q16 = 65536 as i32;
        (*psEnc).sCmn.prevSignalType = TYPE_NO_VOICE_ACTIVITY as i8;
        (*psEnc).sCmn.fs_kHz = fs_kHz;
        if (*psEnc).sCmn.fs_kHz == 8 as i32 {
            if (*psEnc).sCmn.nb_subfr == MAX_NB_SUBFR {
                (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_NB_iCDF.as_ptr();
            } else {
                (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_10_ms_NB_iCDF.as_ptr();
            }
        } else if (*psEnc).sCmn.nb_subfr == MAX_NB_SUBFR {
            (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_iCDF.as_ptr();
        } else {
            (*psEnc).sCmn.pitch_contour_iCDF = silk_pitch_contour_10_ms_iCDF.as_ptr();
        }
        if (*psEnc).sCmn.fs_kHz == 8 as i32 || (*psEnc).sCmn.fs_kHz == 12 as i32 {
            (*psEnc).sCmn.predictLPCOrder = MIN_LPC_ORDER;
            (*psEnc).sCmn.psNLSF_CB = &silk_NLSF_CB_NB_MB;
        } else {
            (*psEnc).sCmn.predictLPCOrder = MAX_LPC_ORDER;
            (*psEnc).sCmn.psNLSF_CB = &silk_NLSF_CB_WB;
        }
        (*psEnc).sCmn.subfr_length = SUB_FRAME_LENGTH_MS * fs_kHz;
        (*psEnc).sCmn.frame_length =
            (*psEnc).sCmn.subfr_length as i16 as i32 * (*psEnc).sCmn.nb_subfr as i16 as i32;
        (*psEnc).sCmn.ltp_mem_length = 20 as i32 as i16 as i32 * fs_kHz as i16 as i32;
        (*psEnc).sCmn.la_pitch = 2 as i32 as i16 as i32 * fs_kHz as i16 as i32;
        (*psEnc).sCmn.max_pitch_lag = 18 as i32 as i16 as i32 * fs_kHz as i16 as i32;
        if (*psEnc).sCmn.nb_subfr == MAX_NB_SUBFR {
            (*psEnc).sCmn.pitch_LPC_win_length =
                (20 as i32 + ((2 as i32) << 1 as i32)) as i16 as i32 * fs_kHz as i16 as i32;
        } else {
            (*psEnc).sCmn.pitch_LPC_win_length =
                (10 as i32 + ((2 as i32) << 1 as i32)) as i16 as i32 * fs_kHz as i16 as i32;
        }
        if (*psEnc).sCmn.fs_kHz == 16 as i32 {
            (*psEnc).sCmn.pitch_lag_low_bits_iCDF = silk_uniform8_iCDF.as_ptr();
        } else if (*psEnc).sCmn.fs_kHz == 12 as i32 {
            (*psEnc).sCmn.pitch_lag_low_bits_iCDF = silk_uniform6_iCDF.as_ptr();
        } else {
            (*psEnc).sCmn.pitch_lag_low_bits_iCDF = silk_uniform4_iCDF.as_ptr();
        }
    }
    if !((*psEnc).sCmn.subfr_length * (*psEnc).sCmn.nb_subfr == (*psEnc).sCmn.frame_length) {
        celt_fatal(
            b"assertion failed: ( psEnc->sCmn.subfr_length * psEnc->sCmn.nb_subfr ) == psEnc->sCmn.frame_length\0"
                as *const u8 as *const i8,
            b"silk/control_codec.c\0" as *const u8 as *const i8,
            302 as i32,
        );
    }
    return ret;
}
unsafe fn silk_setup_complexity(mut psEncC: *mut silk_encoder_state, Complexity: i32) -> i32 {
    let ret: i32 = 0 as i32;
    if !(Complexity >= 0 as i32 && Complexity <= 10 as i32) {
        celt_fatal(
            b"assertion failed: Complexity >= 0 && Complexity <= 10\0" as *const u8 as *const i8,
            b"silk/control_codec.c\0" as *const u8 as *const i8,
            315 as i32,
        );
    }
    if Complexity < 1 as i32 {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MIN_COMPLEX;
        (*psEncC).pitchEstimationThreshold_Q16 =
            (0.8f64 * ((1 as i32 as i64) << 16 as i32) as f64 + 0.5f64) as i32;
        (*psEncC).pitchEstimationLPCOrder = 6 as i32;
        (*psEncC).shapingLPCOrder = 12 as i32;
        (*psEncC).la_shape = 3 as i32 * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 1 as i32;
        (*psEncC).useInterpolatedNLSFs = 0 as i32;
        (*psEncC).NLSF_MSVQ_Survivors = 2 as i32;
        (*psEncC).warping_Q16 = 0 as i32;
    } else if Complexity < 2 as i32 {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MID_COMPLEX;
        (*psEncC).pitchEstimationThreshold_Q16 =
            (0.76f64 * ((1 as i32 as i64) << 16 as i32) as f64 + 0.5f64) as i32;
        (*psEncC).pitchEstimationLPCOrder = 8 as i32;
        (*psEncC).shapingLPCOrder = 14 as i32;
        (*psEncC).la_shape = 5 as i32 * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 1 as i32;
        (*psEncC).useInterpolatedNLSFs = 0 as i32;
        (*psEncC).NLSF_MSVQ_Survivors = 3 as i32;
        (*psEncC).warping_Q16 = 0 as i32;
    } else if Complexity < 3 as i32 {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MIN_COMPLEX;
        (*psEncC).pitchEstimationThreshold_Q16 =
            (0.8f64 * ((1 as i32 as i64) << 16 as i32) as f64 + 0.5f64) as i32;
        (*psEncC).pitchEstimationLPCOrder = 6 as i32;
        (*psEncC).shapingLPCOrder = 12 as i32;
        (*psEncC).la_shape = 3 as i32 * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 2 as i32;
        (*psEncC).useInterpolatedNLSFs = 0 as i32;
        (*psEncC).NLSF_MSVQ_Survivors = 2 as i32;
        (*psEncC).warping_Q16 = 0 as i32;
    } else if Complexity < 4 as i32 {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MID_COMPLEX;
        (*psEncC).pitchEstimationThreshold_Q16 =
            (0.76f64 * ((1 as i32 as i64) << 16 as i32) as f64 + 0.5f64) as i32;
        (*psEncC).pitchEstimationLPCOrder = 8 as i32;
        (*psEncC).shapingLPCOrder = 14 as i32;
        (*psEncC).la_shape = 5 as i32 * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 2 as i32;
        (*psEncC).useInterpolatedNLSFs = 0 as i32;
        (*psEncC).NLSF_MSVQ_Survivors = 4 as i32;
        (*psEncC).warping_Q16 = 0 as i32;
    } else if Complexity < 6 as i32 {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MID_COMPLEX;
        (*psEncC).pitchEstimationThreshold_Q16 =
            (0.74f64 * ((1 as i32 as i64) << 16 as i32) as f64 + 0.5f64) as i32;
        (*psEncC).pitchEstimationLPCOrder = 10 as i32;
        (*psEncC).shapingLPCOrder = 16 as i32;
        (*psEncC).la_shape = 5 as i32 * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 2 as i32;
        (*psEncC).useInterpolatedNLSFs = 1 as i32;
        (*psEncC).NLSF_MSVQ_Survivors = 6 as i32;
        (*psEncC).warping_Q16 = (*psEncC).fs_kHz
            * ((0.015f32 * ((1 as i32 as i64) << 16 as i32) as f32) as f64 + 0.5f64) as i32;
    } else if Complexity < 8 as i32 {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MID_COMPLEX;
        (*psEncC).pitchEstimationThreshold_Q16 =
            (0.72f64 * ((1 as i32 as i64) << 16 as i32) as f64 + 0.5f64) as i32;
        (*psEncC).pitchEstimationLPCOrder = 12 as i32;
        (*psEncC).shapingLPCOrder = 20 as i32;
        (*psEncC).la_shape = 5 as i32 * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 3 as i32;
        (*psEncC).useInterpolatedNLSFs = 1 as i32;
        (*psEncC).NLSF_MSVQ_Survivors = 8 as i32;
        (*psEncC).warping_Q16 = (*psEncC).fs_kHz
            * ((0.015f32 * ((1 as i32 as i64) << 16 as i32) as f32) as f64 + 0.5f64) as i32;
    } else {
        (*psEncC).pitchEstimationComplexity = SILK_PE_MAX_COMPLEX;
        (*psEncC).pitchEstimationThreshold_Q16 =
            (0.7f64 * ((1 as i32 as i64) << 16 as i32) as f64 + 0.5f64) as i32;
        (*psEncC).pitchEstimationLPCOrder = 16 as i32;
        (*psEncC).shapingLPCOrder = 24 as i32;
        (*psEncC).la_shape = 5 as i32 * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = MAX_DEL_DEC_STATES;
        (*psEncC).useInterpolatedNLSFs = 1 as i32;
        (*psEncC).NLSF_MSVQ_Survivors = 16 as i32;
        (*psEncC).warping_Q16 = (*psEncC).fs_kHz
            * ((0.015f32 * ((1 as i32 as i64) << 16 as i32) as f32) as f64 + 0.5f64) as i32;
    }
    (*psEncC).pitchEstimationLPCOrder =
        silk_min_int((*psEncC).pitchEstimationLPCOrder, (*psEncC).predictLPCOrder);
    (*psEncC).shapeWinLength =
        SUB_FRAME_LENGTH_MS * (*psEncC).fs_kHz + 2 as i32 * (*psEncC).la_shape;
    (*psEncC).Complexity = Complexity;
    if !((*psEncC).pitchEstimationLPCOrder <= 16 as i32) {
        celt_fatal(
            b"assertion failed: psEncC->pitchEstimationLPCOrder <= MAX_FIND_PITCH_LPC_ORDER\0"
                as *const u8 as *const i8,
            b"silk/control_codec.c\0" as *const u8 as *const i8,
            393 as i32,
        );
    }
    if !((*psEncC).shapingLPCOrder <= 24 as i32) {
        celt_fatal(
            b"assertion failed: psEncC->shapingLPCOrder <= MAX_SHAPE_LPC_ORDER\0" as *const u8
                as *const i8,
            b"silk/control_codec.c\0" as *const u8 as *const i8,
            394 as i32,
        );
    }
    if !((*psEncC).nStatesDelayedDecision <= 4 as i32) {
        celt_fatal(
            b"assertion failed: psEncC->nStatesDelayedDecision <= MAX_DEL_DEC_STATES\0" as *const u8
                as *const i8,
            b"silk/control_codec.c\0" as *const u8 as *const i8,
            395 as i32,
        );
    }
    if !((*psEncC).warping_Q16 <= 32767 as i32) {
        celt_fatal(
            b"assertion failed: psEncC->warping_Q16 <= 32767\0" as *const u8 as *const i8,
            b"silk/control_codec.c\0" as *const u8 as *const i8,
            396 as i32,
        );
    }
    if !((*psEncC).la_shape <= 5 as i32 * 16 as i32) {
        celt_fatal(
            b"assertion failed: psEncC->la_shape <= LA_SHAPE_MAX\0" as *const u8 as *const i8,
            b"silk/control_codec.c\0" as *const u8 as *const i8,
            397 as i32,
        );
    }
    if !((*psEncC).shapeWinLength <= 15 as i32 * 16 as i32) {
        celt_fatal(
            b"assertion failed: psEncC->shapeWinLength <= SHAPE_LPC_WIN_MAX\0" as *const u8
                as *const i8,
            b"silk/control_codec.c\0" as *const u8 as *const i8,
            398 as i32,
        );
    }
    return ret;
}
#[inline]
unsafe fn silk_setup_LBRR(
    mut psEncC: *mut silk_encoder_state,
    encControl: *const silk_EncControlStruct,
) -> i32 {
    let mut LBRR_in_previous_packet: i32 = 0;
    let ret: i32 = SILK_NO_ERROR;
    LBRR_in_previous_packet = (*psEncC).LBRR_enabled;
    (*psEncC).LBRR_enabled = (*encControl).LBRR_coded;
    if (*psEncC).LBRR_enabled != 0 {
        if LBRR_in_previous_packet == 0 as i32 {
            (*psEncC).LBRR_GainIncreases = 7 as i32;
        } else {
            (*psEncC).LBRR_GainIncreases = silk_max_int(
                7 as i32
                    - ((*psEncC).PacketLoss_perc as i64
                        * (0.4f64 * ((1 as i32 as i64) << 16 as i32) as f64 + 0.5f64) as i32 as i16
                            as i64
                        >> 16 as i32) as i32,
                2 as i32,
            );
        }
    }
    return ret;
}
