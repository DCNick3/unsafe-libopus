use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "46:9"]
pub struct silk_EncControlStruct {
    pub nChannelsAPI: i32,
    pub nChannelsInternal: i32,
    pub API_sampleRate: i32,
    pub maxInternalSampleRate: i32,
    pub minInternalSampleRate: i32,
    pub desiredInternalSampleRate: i32,
    pub payloadSize_ms: libc::c_int,
    pub bitRate: i32,
    pub packetLossPercentage: libc::c_int,
    pub complexity: libc::c_int,
    pub useInBandFEC: libc::c_int,
    pub LBRR_coded: libc::c_int,
    pub useDTX: libc::c_int,
    pub useCBR: libc::c_int,
    pub maxBits: libc::c_int,
    pub toMono: libc::c_int,
    pub opusCanSwitch: libc::c_int,
    pub reducedDependency: libc::c_int,
    pub internalSampleRate: i32,
    pub allowBandwidthSwitch: libc::c_int,
    pub inWBmodeWithoutVariableLP: libc::c_int,
    pub stereoWidth_Q14: libc::c_int,
    pub switchReady: libc::c_int,
    pub signalType: libc::c_int,
    pub offset: libc::c_int,
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/errors.h:31"]
pub mod errors_h {
    #[c2rust::src_loc = "46:9"]
    pub const SILK_ENC_INPUT_INVALID_NO_OF_SAMPLES: libc::c_int = -(101 as libc::c_int);
    #[c2rust::src_loc = "39:9"]
    pub const SILK_NO_ERROR: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/main_FLP.h:41"]
pub mod main_FLP_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_encode_do_VAD_Fxx: unsafe extern "C" fn(
        *mut silk_encoder_state_FLP,
        libc::c_int,
    ) -> () = silk_encode_do_VAD_FLP;
    #[c2rust::src_loc = "46:9"]
    pub const silk_encode_frame_Fxx: unsafe extern "C" fn(
        *mut silk_encoder_state_FLP,
        *mut i32,
        *mut ec_enc,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_int = silk_encode_frame_FLP;

    use super::silk_EncControlStruct;
    use crate::celt::entenc::ec_enc;
    use crate::silk::float::structs_FLP::silk_encoder_state_FLP;
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn silk_control_encoder(
            psEnc: *mut silk_encoder_state_FLP,
            encControl: *mut silk_EncControlStruct,
            allow_bw_switch: libc::c_int,
            channelNb: libc::c_int,
            force_fs_kHz: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "53:1"]
        pub fn silk_HP_variable_cutoff(state_Fxx: *mut silk_encoder_state_FLP);
        #[c2rust::src_loc = "58:1"]
        pub fn silk_encode_do_VAD_FLP(psEnc: *mut silk_encoder_state_FLP, activity: libc::c_int);
        #[c2rust::src_loc = "64:1"]
        pub fn silk_encode_frame_FLP(
            psEnc: *mut silk_encoder_state_FLP,
            pnBytesOut: *mut i32,
            psRangeEnc: *mut ec_enc,
            condCoding: libc::c_int,
            maxBits: libc::c_int,
            useCBR: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "74:1"]
        pub fn silk_init_encoder(
            psEnc: *mut silk_encoder_state_FLP,
            arch: libc::c_int,
        ) -> libc::c_int;
    }
}
use self::errors_h::{SILK_ENC_INPUT_INVALID_NO_OF_SAMPLES, SILK_NO_ERROR};
pub use self::main_FLP_h::{
    silk_HP_variable_cutoff, silk_control_encoder, silk_encode_do_VAD_FLP, silk_encode_do_VAD_Fxx,
    silk_encode_frame_FLP, silk_encode_frame_Fxx, silk_init_encoder,
};
use crate::celt::celt::celt_fatal;
use crate::celt::entcode::ec_tell;
use crate::celt::entenc::{ec_enc, ec_enc_icdf, ec_enc_patch_initial_bits};
use crate::externs::{memcpy, memset};
use crate::silk::check_control_input::check_control_input;
use crate::silk::control_SNR::silk_control_SNR;
use crate::silk::define::{
    CODE_CONDITIONALLY, CODE_INDEPENDENTLY, CODE_INDEPENDENTLY_NO_LTP_SCALING,
    ENCODER_NUM_CHANNELS, TYPE_NO_VOICE_ACTIVITY,
};
use crate::silk::encode_indices::silk_encode_indices;
use crate::silk::encode_pulses::silk_encode_pulses;
use crate::silk::float::structs_FLP::{silk_encoder, silk_encoder_state_FLP, silk_shape_state_FLP};
use crate::silk::resampler::silk_resampler;
use crate::silk::resampler_structs::silk_resampler_state_struct;
use crate::silk::stereo_LR_to_MS::silk_stereo_LR_to_MS;
use crate::silk::stereo_encode_pred::{silk_stereo_encode_mid_only, silk_stereo_encode_pred};
use crate::silk::structs::{silk_LP_state, silk_nsq_state};
use crate::silk::tables_other::{silk_LBRR_flags_iCDF_ptr, silk_Quantization_Offsets_Q10};

#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub unsafe extern "C" fn silk_Get_Encoder_Size(encSizeBytes: *mut libc::c_int) -> libc::c_int {
    let ret: libc::c_int = SILK_NO_ERROR;
    *encSizeBytes = ::core::mem::size_of::<silk_encoder>() as libc::c_ulong as libc::c_int;
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn silk_InitEncoder(
    encState: *mut libc::c_void,
    arch: libc::c_int,
    encStatus: *mut silk_EncControlStruct,
) -> libc::c_int {
    let mut psEnc: *mut silk_encoder = 0 as *mut silk_encoder;
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = SILK_NO_ERROR;
    psEnc = encState as *mut silk_encoder;
    memset(
        psEnc as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<silk_encoder>() as libc::c_ulong,
    );
    n = 0 as libc::c_int;
    while n < ENCODER_NUM_CHANNELS {
        ret += silk_init_encoder(
            &mut *((*psEnc).state_Fxx).as_mut_ptr().offset(n as isize),
            arch,
        );
        if ret != 0 {
            if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"silk/enc_API.c\0" as *const u8 as *const libc::c_char,
                    85 as libc::c_int,
                );
            }
        }
        n += 1;
    }
    (*psEnc).nChannelsAPI = 1 as libc::c_int;
    (*psEnc).nChannelsInternal = 1 as libc::c_int;
    ret += silk_QueryEncoder(encState, encStatus);
    if ret != 0 {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/enc_API.c\0" as *const u8 as *const libc::c_char,
                94 as libc::c_int,
            );
        }
    }
    return ret;
}
#[c2rust::src_loc = "103:1"]
unsafe extern "C" fn silk_QueryEncoder(
    encState: *const libc::c_void,
    mut encStatus: *mut silk_EncControlStruct,
) -> libc::c_int {
    let ret: libc::c_int = SILK_NO_ERROR;
    let mut state_Fxx: *mut silk_encoder_state_FLP = 0 as *mut silk_encoder_state_FLP;
    let psEnc: *mut silk_encoder = encState as *mut silk_encoder;
    state_Fxx = ((*psEnc).state_Fxx).as_mut_ptr();
    (*encStatus).nChannelsAPI = (*psEnc).nChannelsAPI;
    (*encStatus).nChannelsInternal = (*psEnc).nChannelsInternal;
    (*encStatus).API_sampleRate = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .API_fs_Hz;
    (*encStatus).maxInternalSampleRate = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .maxInternal_fs_Hz;
    (*encStatus).minInternalSampleRate = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .minInternal_fs_Hz;
    (*encStatus).desiredInternalSampleRate = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .desiredInternal_fs_Hz;
    (*encStatus).payloadSize_ms = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .PacketSize_ms;
    (*encStatus).bitRate = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .TargetRate_bps;
    (*encStatus).packetLossPercentage = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .PacketLoss_perc;
    (*encStatus).complexity = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .Complexity;
    (*encStatus).useInBandFEC = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .useInBandFEC;
    (*encStatus).useDTX = (*state_Fxx.offset(0 as libc::c_int as isize)).sCmn.useDTX;
    (*encStatus).useCBR = (*state_Fxx.offset(0 as libc::c_int as isize)).sCmn.useCBR;
    (*encStatus).internalSampleRate = (*state_Fxx.offset(0 as libc::c_int as isize)).sCmn.fs_kHz
        as i16 as i32
        * 1000 as libc::c_int as i16 as i32;
    (*encStatus).allowBandwidthSwitch = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .allow_bandwidth_switch;
    (*encStatus).inWBmodeWithoutVariableLP =
        ((*state_Fxx.offset(0 as libc::c_int as isize)).sCmn.fs_kHz == 16 as libc::c_int
            && (*state_Fxx.offset(0 as libc::c_int as isize)).sCmn.sLP.mode == 0 as libc::c_int)
            as libc::c_int;
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "140:1"]
pub unsafe extern "C" fn silk_Encode(
    encState: *mut libc::c_void,
    mut encControl: *mut silk_EncControlStruct,
    mut samplesIn: *const i16,
    mut nSamplesIn: libc::c_int,
    psRangeEnc: *mut ec_enc,
    nBytesOut: *mut i32,
    prefillFlag: libc::c_int,
    activity: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nBits: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut tmp_payloadSize_ms: libc::c_int = 0 as libc::c_int;
    let mut tmp_complexity: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut nSamplesToBuffer: libc::c_int = 0;
    let mut nSamplesToBufferMax: libc::c_int = 0;
    let mut nBlocksOf10ms: libc::c_int = 0;
    let mut nSamplesFromInput: libc::c_int = 0 as libc::c_int;
    let mut nSamplesFromInputMax: libc::c_int = 0;
    let mut speech_act_thr_for_switch_Q8: libc::c_int = 0;
    let mut TargetRate_bps: i32 = 0;
    let mut MStargetRates_bps: [i32; 2] = [0; 2];
    let mut channelRate_bps: i32 = 0;
    let mut LBRR_symbol: i32 = 0;
    let mut sum: i32 = 0;
    let mut psEnc: *mut silk_encoder = encState as *mut silk_encoder;
    let mut transition: libc::c_int = 0;
    let mut curr_block: libc::c_int = 0;
    let mut tot_blocks: libc::c_int = 0;
    if (*encControl).reducedDependency != 0 {
        (*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .first_frame_after_reset = 1 as libc::c_int;
        (*psEnc).state_Fxx[1 as libc::c_int as usize]
            .sCmn
            .first_frame_after_reset = 1 as libc::c_int;
    }
    (*psEnc).state_Fxx[1 as libc::c_int as usize]
        .sCmn
        .nFramesEncoded = 0 as libc::c_int;
    (*psEnc).state_Fxx[0 as libc::c_int as usize]
        .sCmn
        .nFramesEncoded = (*psEnc).state_Fxx[1 as libc::c_int as usize]
        .sCmn
        .nFramesEncoded;
    ret = check_control_input(encControl);
    if ret != 0 as libc::c_int {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/enc_API.c\0" as *const u8 as *const libc::c_char,
                170 as libc::c_int,
            );
        }
        return ret;
    }
    (*encControl).switchReady = 0 as libc::c_int;
    if (*encControl).nChannelsInternal > (*psEnc).nChannelsInternal {
        ret += silk_init_encoder(
            &mut *((*psEnc).state_Fxx)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize),
            (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.arch,
        );
        memset(
            ((*psEnc).sStereo.pred_prev_Q13).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[i16; 2]>() as libc::c_ulong,
        );
        memset(
            ((*psEnc).sStereo.sSide).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[i16; 2]>() as libc::c_ulong,
        );
        (*psEnc).sStereo.mid_side_amp_Q0[0 as libc::c_int as usize] = 0 as libc::c_int;
        (*psEnc).sStereo.mid_side_amp_Q0[1 as libc::c_int as usize] = 1 as libc::c_int;
        (*psEnc).sStereo.mid_side_amp_Q0[2 as libc::c_int as usize] = 0 as libc::c_int;
        (*psEnc).sStereo.mid_side_amp_Q0[3 as libc::c_int as usize] = 1 as libc::c_int;
        (*psEnc).sStereo.width_prev_Q14 = 0 as libc::c_int as i16;
        (*psEnc).sStereo.smth_width_Q14 = ((1 as libc::c_int as libc::c_long
            * ((1 as libc::c_int as i64) << 14 as libc::c_int))
            as libc::c_double
            + 0.5f64) as i32 as i16;
        if (*psEnc).nChannelsAPI == 2 as libc::c_int {
            memcpy(
                &mut (*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .sCmn
                .resampler_state as *mut silk_resampler_state_struct
                    as *mut libc::c_void,
                &mut (*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .resampler_state as *mut silk_resampler_state_struct
                    as *const libc::c_void,
                ::core::mem::size_of::<silk_resampler_state_struct>() as libc::c_ulong,
            );
            memcpy(
                &mut (*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .sCmn
                .In_HP_State as *mut [i32; 2] as *mut libc::c_void,
                &mut (*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .In_HP_State as *mut [i32; 2] as *const libc::c_void,
                ::core::mem::size_of::<[i32; 2]>() as libc::c_ulong,
            );
        }
    }
    transition = ((*encControl).payloadSize_ms
        != (*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .PacketSize_ms
        || (*psEnc).nChannelsInternal != (*encControl).nChannelsInternal)
        as libc::c_int;
    (*psEnc).nChannelsAPI = (*encControl).nChannelsAPI;
    (*psEnc).nChannelsInternal = (*encControl).nChannelsInternal;
    nBlocksOf10ms = 100 as libc::c_int * nSamplesIn / (*encControl).API_sampleRate;
    tot_blocks = if nBlocksOf10ms > 1 as libc::c_int {
        nBlocksOf10ms >> 1 as libc::c_int
    } else {
        1 as libc::c_int
    };
    curr_block = 0 as libc::c_int;
    if prefillFlag != 0 {
        let mut save_LP: silk_LP_state = silk_LP_state {
            In_LP_State: [0; 2],
            transition_frame_no: 0,
            mode: 0,
            saved_fs_kHz: 0,
        };
        if nBlocksOf10ms != 1 as libc::c_int {
            if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"silk/enc_API.c\0" as *const u8 as *const libc::c_char,
                    206 as libc::c_int,
                );
            }
            return SILK_ENC_INPUT_INVALID_NO_OF_SAMPLES;
        }
        if prefillFlag == 2 as libc::c_int {
            save_LP = (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.sLP;
            save_LP.saved_fs_kHz = (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz;
        }
        n = 0 as libc::c_int;
        while n < (*encControl).nChannelsInternal {
            ret = silk_init_encoder(
                &mut *((*psEnc).state_Fxx).as_mut_ptr().offset(n as isize),
                (*psEnc).state_Fxx[n as usize].sCmn.arch,
            );
            if prefillFlag == 2 as libc::c_int {
                (*psEnc).state_Fxx[n as usize].sCmn.sLP = save_LP;
            }
            if ret != 0 {
                celt_fatal(
                    b"assertion failed: !ret\0" as *const u8 as *const libc::c_char,
                    b"silk/enc_API.c\0" as *const u8 as *const libc::c_char,
                    222 as libc::c_int,
                );
            }
            n += 1;
        }
        tmp_payloadSize_ms = (*encControl).payloadSize_ms;
        (*encControl).payloadSize_ms = 10 as libc::c_int;
        tmp_complexity = (*encControl).complexity;
        (*encControl).complexity = 0 as libc::c_int;
        n = 0 as libc::c_int;
        while n < (*encControl).nChannelsInternal {
            (*psEnc).state_Fxx[n as usize]
                .sCmn
                .controlled_since_last_payload = 0 as libc::c_int;
            (*psEnc).state_Fxx[n as usize].sCmn.prefillFlag = 1 as libc::c_int;
            n += 1;
        }
    } else {
        if nBlocksOf10ms * (*encControl).API_sampleRate != 100 as libc::c_int * nSamplesIn
            || nSamplesIn < 0 as libc::c_int
        {
            if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"silk/enc_API.c\0" as *const u8 as *const libc::c_char,
                    235 as libc::c_int,
                );
            }
            return SILK_ENC_INPUT_INVALID_NO_OF_SAMPLES;
        }
        if 1000 as libc::c_int * nSamplesIn
            > (*encControl).payloadSize_ms * (*encControl).API_sampleRate
        {
            if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"silk/enc_API.c\0" as *const u8 as *const libc::c_char,
                    241 as libc::c_int,
                );
            }
            return SILK_ENC_INPUT_INVALID_NO_OF_SAMPLES;
        }
    }
    n = 0 as libc::c_int;
    while n < (*encControl).nChannelsInternal {
        let force_fs_kHz: libc::c_int = if n == 1 as libc::c_int {
            (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz
        } else {
            0 as libc::c_int
        };
        ret = silk_control_encoder(
            &mut *((*psEnc).state_Fxx).as_mut_ptr().offset(n as isize),
            encControl,
            (*psEnc).allowBandwidthSwitch,
            n,
            force_fs_kHz,
        );
        if ret != 0 as libc::c_int {
            return ret;
        }
        if (*psEnc).state_Fxx[n as usize].sCmn.first_frame_after_reset != 0 || transition != 0 {
            i = 0 as libc::c_int;
            while i
                < (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesPerPacket
            {
                (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags[i as usize] = 0 as libc::c_int;
                i += 1;
            }
        }
        (*psEnc).state_Fxx[n as usize].sCmn.inDTX = (*psEnc).state_Fxx[n as usize].sCmn.useDTX;
        n += 1;
    }
    if !((*encControl).nChannelsInternal == 1 as libc::c_int
        || (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz
            == (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.fs_kHz)
    {
        celt_fatal(
            b"assertion failed: encControl->nChannelsInternal == 1 || psEnc->state_Fxx[ 0 ].sCmn.fs_kHz == psEnc->state_Fxx[ 1 ].sCmn.fs_kHz\0"
                as *const u8 as *const libc::c_char,
            b"silk/enc_API.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
        );
    }
    nSamplesToBufferMax = 10 as libc::c_int
        * nBlocksOf10ms
        * (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz;
    nSamplesFromInputMax = nSamplesToBufferMax
        * (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.API_fs_Hz
        / ((*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz * 1000 as libc::c_int);
    let vla = nSamplesFromInputMax as usize;
    let mut buf: Vec<i16> = ::std::vec::from_elem(0, vla);
    loop {
        nSamplesToBuffer = (*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .frame_length
            - (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .inputBufIx;
        nSamplesToBuffer = if nSamplesToBuffer < nSamplesToBufferMax {
            nSamplesToBuffer
        } else {
            nSamplesToBufferMax
        };
        nSamplesFromInput = nSamplesToBuffer
            * (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.API_fs_Hz
            / ((*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz * 1000 as libc::c_int);
        if (*encControl).nChannelsAPI == 2 as libc::c_int
            && (*encControl).nChannelsInternal == 2 as libc::c_int
        {
            let id: libc::c_int = (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesEncoded;
            n = 0 as libc::c_int;
            while n < nSamplesFromInput {
                *buf.as_mut_ptr().offset(n as isize) =
                    *samplesIn.offset((2 as libc::c_int * n) as isize);
                n += 1;
            }
            if (*psEnc).nPrevChannelsInternal == 1 as libc::c_int && id == 0 as libc::c_int {
                memcpy(
                    &mut (*((*psEnc).state_Fxx)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                    .sCmn
                    .resampler_state as *mut silk_resampler_state_struct
                        as *mut libc::c_void,
                    &mut (*((*psEnc).state_Fxx)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .sCmn
                    .resampler_state as *mut silk_resampler_state_struct
                        as *const libc::c_void,
                    ::core::mem::size_of::<silk_resampler_state_struct>() as libc::c_ulong,
                );
            }
            ret += silk_resampler(
                &mut (*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .resampler_state,
                &mut *((*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .inputBuf)
                    .as_mut_ptr()
                    .offset(
                        ((*((*psEnc).state_Fxx)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                        .sCmn
                        .inputBufIx
                            + 2 as libc::c_int) as isize,
                    ),
                buf.as_mut_ptr() as *const i16,
                nSamplesFromInput,
            );
            (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .inputBufIx += nSamplesToBuffer;
            nSamplesToBuffer = (*psEnc).state_Fxx[1 as libc::c_int as usize]
                .sCmn
                .frame_length
                - (*psEnc).state_Fxx[1 as libc::c_int as usize]
                    .sCmn
                    .inputBufIx;
            nSamplesToBuffer = if nSamplesToBuffer
                < 10 as libc::c_int
                    * nBlocksOf10ms
                    * (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.fs_kHz
            {
                nSamplesToBuffer
            } else {
                10 as libc::c_int
                    * nBlocksOf10ms
                    * (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.fs_kHz
            };
            n = 0 as libc::c_int;
            while n < nSamplesFromInput {
                *buf.as_mut_ptr().offset(n as isize) =
                    *samplesIn.offset((2 as libc::c_int * n + 1 as libc::c_int) as isize);
                n += 1;
            }
            ret += silk_resampler(
                &mut (*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .sCmn
                .resampler_state,
                &mut *((*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .sCmn
                .inputBuf)
                    .as_mut_ptr()
                    .offset(
                        ((*((*psEnc).state_Fxx)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize))
                        .sCmn
                        .inputBufIx
                            + 2 as libc::c_int) as isize,
                    ),
                buf.as_mut_ptr() as *const i16,
                nSamplesFromInput,
            );
            (*psEnc).state_Fxx[1 as libc::c_int as usize]
                .sCmn
                .inputBufIx += nSamplesToBuffer;
        } else if (*encControl).nChannelsAPI == 2 as libc::c_int
            && (*encControl).nChannelsInternal == 1 as libc::c_int
        {
            n = 0 as libc::c_int;
            while n < nSamplesFromInput {
                sum = *samplesIn.offset((2 as libc::c_int * n) as isize) as libc::c_int
                    + *samplesIn.offset((2 as libc::c_int * n + 1 as libc::c_int) as isize)
                        as libc::c_int;
                *buf.as_mut_ptr().offset(n as isize) = (if 1 as libc::c_int == 1 as libc::c_int {
                    (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
                } else {
                    (sum >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) as i16;
                n += 1;
            }
            ret += silk_resampler(
                &mut (*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .resampler_state,
                &mut *((*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .inputBuf)
                    .as_mut_ptr()
                    .offset(
                        ((*((*psEnc).state_Fxx)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                        .sCmn
                        .inputBufIx
                            + 2 as libc::c_int) as isize,
                    ),
                buf.as_mut_ptr() as *const i16,
                nSamplesFromInput,
            );
            if (*psEnc).nPrevChannelsInternal == 2 as libc::c_int
                && (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesEncoded
                    == 0 as libc::c_int
            {
                ret += silk_resampler(
                    &mut (*((*psEnc).state_Fxx)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                    .sCmn
                    .resampler_state,
                    &mut *((*((*psEnc).state_Fxx)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                    .sCmn
                    .inputBuf)
                        .as_mut_ptr()
                        .offset(
                            ((*((*psEnc).state_Fxx)
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize))
                            .sCmn
                            .inputBufIx
                                + 2 as libc::c_int) as isize,
                        ),
                    buf.as_mut_ptr() as *const i16,
                    nSamplesFromInput,
                );
                n = 0 as libc::c_int;
                while n
                    < (*psEnc).state_Fxx[0 as libc::c_int as usize]
                        .sCmn
                        .frame_length
                {
                    (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.inputBuf[((*psEnc).state_Fxx
                        [0 as libc::c_int as usize]
                        .sCmn
                        .inputBufIx
                        + n
                        + 2 as libc::c_int)
                        as usize] = ((*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.inputBuf
                        [((*psEnc).state_Fxx[0 as libc::c_int as usize]
                            .sCmn
                            .inputBufIx
                            + n
                            + 2 as libc::c_int) as usize]
                        as libc::c_int
                        + (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.inputBuf[((*psEnc)
                            .state_Fxx[1 as libc::c_int as usize]
                            .sCmn
                            .inputBufIx
                            + n
                            + 2 as libc::c_int)
                            as usize] as libc::c_int
                        >> 1 as libc::c_int) as i16;
                    n += 1;
                }
            }
            (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .inputBufIx += nSamplesToBuffer;
        } else {
            if !((*encControl).nChannelsAPI == 1 as libc::c_int
                && (*encControl).nChannelsInternal == 1 as libc::c_int)
            {
                celt_fatal(
                    b"assertion failed: encControl->nChannelsAPI == 1 && encControl->nChannelsInternal == 1\0"
                        as *const u8 as *const libc::c_char,
                    b"silk/enc_API.c\0" as *const u8 as *const libc::c_char,
                    320 as libc::c_int,
                );
            }
            memcpy(
                buf.as_mut_ptr() as *mut libc::c_void,
                samplesIn as *const libc::c_void,
                (nSamplesFromInput as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
            );
            ret += silk_resampler(
                &mut (*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .resampler_state,
                &mut *((*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .inputBuf)
                    .as_mut_ptr()
                    .offset(
                        ((*((*psEnc).state_Fxx)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                        .sCmn
                        .inputBufIx
                            + 2 as libc::c_int) as isize,
                    ),
                buf.as_mut_ptr() as *const i16,
                nSamplesFromInput,
            );
            (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .inputBufIx += nSamplesToBuffer;
        }
        samplesIn = samplesIn.offset((nSamplesFromInput * (*encControl).nChannelsAPI) as isize);
        nSamplesIn -= nSamplesFromInput;
        (*psEnc).allowBandwidthSwitch = 0 as libc::c_int;
        if !((*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .inputBufIx
            >= (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .frame_length)
        {
            break;
        }
        if !((*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .inputBufIx
            == (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .frame_length)
        {
            celt_fatal(
                b"assertion failed: psEnc->state_Fxx[ 0 ].sCmn.inputBufIx == psEnc->state_Fxx[ 0 ].sCmn.frame_length\0"
                    as *const u8 as *const libc::c_char,
                b"silk/enc_API.c\0" as *const u8 as *const libc::c_char,
                336 as libc::c_int,
            );
        }
        if !((*encControl).nChannelsInternal == 1 as libc::c_int
            || (*psEnc).state_Fxx[1 as libc::c_int as usize]
                .sCmn
                .inputBufIx
                == (*psEnc).state_Fxx[1 as libc::c_int as usize]
                    .sCmn
                    .frame_length)
        {
            celt_fatal(
                b"assertion failed: encControl->nChannelsInternal == 1 || psEnc->state_Fxx[ 1 ].sCmn.inputBufIx == psEnc->state_Fxx[ 1 ].sCmn.frame_length\0"
                    as *const u8 as *const libc::c_char,
                b"silk/enc_API.c\0" as *const u8 as *const libc::c_char,
                337 as libc::c_int,
            );
        }
        if (*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .nFramesEncoded
            == 0 as libc::c_int
            && prefillFlag == 0
        {
            let mut iCDF: [u8; 2] = [0 as libc::c_int as u8, 0 as libc::c_int as u8];
            iCDF[0 as libc::c_int as usize] = (256 as libc::c_int
                - (256 as libc::c_int
                    >> ((*psEnc).state_Fxx[0 as libc::c_int as usize]
                        .sCmn
                        .nFramesPerPacket
                        + 1 as libc::c_int)
                        * (*encControl).nChannelsInternal))
                as u8;
            ec_enc_icdf(
                psRangeEnc,
                0 as libc::c_int,
                iCDF.as_mut_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
            n = 0 as libc::c_int;
            while n < (*encControl).nChannelsInternal {
                LBRR_symbol = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < (*psEnc).state_Fxx[n as usize].sCmn.nFramesPerPacket {
                    LBRR_symbol |= (((*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags[i as usize]
                        as u32)
                        << i) as i32;
                    i += 1;
                }
                (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flag = (if LBRR_symbol > 0 as libc::c_int {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as i8;
                if LBRR_symbol != 0
                    && (*psEnc).state_Fxx[n as usize].sCmn.nFramesPerPacket > 1 as libc::c_int
                {
                    ec_enc_icdf(
                        psRangeEnc,
                        LBRR_symbol - 1 as libc::c_int,
                        silk_LBRR_flags_iCDF_ptr[((*psEnc).state_Fxx[n as usize]
                            .sCmn
                            .nFramesPerPacket
                            - 2 as libc::c_int)
                            as usize],
                        8 as libc::c_int as libc::c_uint,
                    );
                }
                n += 1;
            }
            i = 0 as libc::c_int;
            while i
                < (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesPerPacket
            {
                n = 0 as libc::c_int;
                while n < (*encControl).nChannelsInternal {
                    if (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags[i as usize] != 0 {
                        let mut condCoding: libc::c_int = 0;
                        if (*encControl).nChannelsInternal == 2 as libc::c_int
                            && n == 0 as libc::c_int
                        {
                            silk_stereo_encode_pred(
                                psRangeEnc,
                                ((*psEnc).sStereo.predIx[i as usize]).as_mut_ptr(),
                            );
                            if (*psEnc).state_Fxx[1 as libc::c_int as usize]
                                .sCmn
                                .LBRR_flags[i as usize]
                                == 0 as libc::c_int
                            {
                                silk_stereo_encode_mid_only(
                                    psRangeEnc,
                                    (*psEnc).sStereo.mid_only_flags[i as usize],
                                );
                            }
                        }
                        if i > 0 as libc::c_int
                            && (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags
                                [(i - 1 as libc::c_int) as usize]
                                != 0
                        {
                            condCoding = CODE_CONDITIONALLY;
                        } else {
                            condCoding = CODE_INDEPENDENTLY;
                        }
                        silk_encode_indices(
                            &mut (*((*psEnc).state_Fxx).as_mut_ptr().offset(n as isize)).sCmn,
                            psRangeEnc,
                            i,
                            1 as libc::c_int,
                            condCoding,
                        );
                        silk_encode_pulses(
                            psRangeEnc,
                            (*psEnc).state_Fxx[n as usize].sCmn.indices_LBRR[i as usize].signalType
                                as libc::c_int,
                            (*psEnc).state_Fxx[n as usize].sCmn.indices_LBRR[i as usize]
                                .quantOffsetType as libc::c_int,
                            ((*psEnc).state_Fxx[n as usize].sCmn.pulses_LBRR[i as usize])
                                .as_mut_ptr(),
                            (*psEnc).state_Fxx[n as usize].sCmn.frame_length,
                        );
                    }
                    n += 1;
                }
                i += 1;
            }
            n = 0 as libc::c_int;
            while n < (*encControl).nChannelsInternal {
                memset(
                    ((*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags).as_mut_ptr()
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong,
                );
                n += 1;
            }
            (*psEnc).nBitsUsedLBRR = ec_tell(psRangeEnc);
        }
        silk_HP_variable_cutoff(((*psEnc).state_Fxx).as_mut_ptr());
        nBits = (*encControl).bitRate * (*encControl).payloadSize_ms / 1000 as libc::c_int;
        if prefillFlag == 0 {
            nBits -= (*psEnc).nBitsUsedLBRR;
        }
        nBits = nBits
            / (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesPerPacket;
        if (*encControl).payloadSize_ms == 10 as libc::c_int {
            TargetRate_bps = nBits as i16 as i32 * 100 as libc::c_int as i16 as i32;
        } else {
            TargetRate_bps = nBits as i16 as i32 * 50 as libc::c_int as i16 as i32;
        }
        TargetRate_bps -= (*psEnc).nBitsExceeded * 1000 as libc::c_int / 500 as libc::c_int;
        if prefillFlag == 0
            && (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesEncoded
                > 0 as libc::c_int
        {
            let bitsBalance: i32 = ec_tell(psRangeEnc)
                - (*psEnc).nBitsUsedLBRR
                - nBits
                    * (*psEnc).state_Fxx[0 as libc::c_int as usize]
                        .sCmn
                        .nFramesEncoded;
            TargetRate_bps -= bitsBalance * 1000 as libc::c_int / 500 as libc::c_int;
        }
        TargetRate_bps = if (*encControl).bitRate > 5000 as libc::c_int {
            if TargetRate_bps > (*encControl).bitRate {
                (*encControl).bitRate
            } else if TargetRate_bps < 5000 as libc::c_int {
                5000 as libc::c_int
            } else {
                TargetRate_bps
            }
        } else if TargetRate_bps > 5000 as libc::c_int {
            5000 as libc::c_int
        } else if TargetRate_bps < (*encControl).bitRate {
            (*encControl).bitRate
        } else {
            TargetRate_bps
        };
        if (*encControl).nChannelsInternal == 2 as libc::c_int {
            silk_stereo_LR_to_MS(
                &mut (*psEnc).sStereo,
                &mut *((*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .inputBuf)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize),
                &mut *((*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .sCmn
                .inputBuf)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize),
                ((*psEnc).sStereo.predIx[(*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesEncoded as usize])
                    .as_mut_ptr(),
                &mut *((*psEnc).sStereo.mid_only_flags).as_mut_ptr().offset(
                    (*((*psEnc).state_Fxx)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .sCmn
                    .nFramesEncoded as isize,
                ),
                MStargetRates_bps.as_mut_ptr(),
                TargetRate_bps,
                (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .speech_activity_Q8,
                (*encControl).toMono,
                (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz,
                (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .frame_length,
            );
            if (*psEnc).sStereo.mid_only_flags[(*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesEncoded as usize] as libc::c_int
                == 0 as libc::c_int
            {
                if (*psEnc).prev_decode_only_middle == 1 as libc::c_int {
                    memset(
                        &mut (*((*psEnc).state_Fxx)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize))
                        .sShape as *mut silk_shape_state_FLP
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<silk_shape_state_FLP>() as libc::c_ulong,
                    );
                    memset(
                        &mut (*((*psEnc).state_Fxx)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize))
                        .sCmn
                        .sNSQ as *mut silk_nsq_state as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<silk_nsq_state>() as libc::c_ulong,
                    );
                    memset(
                        ((*psEnc).state_Fxx[1 as libc::c_int as usize]
                            .sCmn
                            .prev_NLSFq_Q15)
                            .as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[i16; 16]>() as libc::c_ulong,
                    );
                    memset(
                        &mut (*((*psEnc).state_Fxx)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize))
                        .sCmn
                        .sLP
                        .In_LP_State as *mut [i32; 2] as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[i32; 2]>() as libc::c_ulong,
                    );
                    (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.prevLag = 100 as libc::c_int;
                    (*psEnc).state_Fxx[1 as libc::c_int as usize]
                        .sCmn
                        .sNSQ
                        .lagPrev = 100 as libc::c_int;
                    (*psEnc).state_Fxx[1 as libc::c_int as usize]
                        .sShape
                        .LastGainIndex = 10 as libc::c_int as i8;
                    (*psEnc).state_Fxx[1 as libc::c_int as usize]
                        .sCmn
                        .prevSignalType = TYPE_NO_VOICE_ACTIVITY as i8;
                    (*psEnc).state_Fxx[1 as libc::c_int as usize]
                        .sCmn
                        .sNSQ
                        .prev_gain_Q16 = 65536 as libc::c_int;
                    (*psEnc).state_Fxx[1 as libc::c_int as usize]
                        .sCmn
                        .first_frame_after_reset = 1 as libc::c_int;
                }
                silk_encode_do_VAD_FLP(
                    &mut *((*psEnc).state_Fxx)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize),
                    activity,
                );
            } else {
                (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.VAD_flags[(*psEnc).state_Fxx
                    [0 as libc::c_int as usize]
                    .sCmn
                    .nFramesEncoded
                    as usize] = 0 as libc::c_int as i8;
            }
            if prefillFlag == 0 {
                silk_stereo_encode_pred(
                    psRangeEnc,
                    ((*psEnc).sStereo.predIx[(*psEnc).state_Fxx[0 as libc::c_int as usize]
                        .sCmn
                        .nFramesEncoded as usize])
                        .as_mut_ptr(),
                );
                if (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.VAD_flags[(*psEnc).state_Fxx
                    [0 as libc::c_int as usize]
                    .sCmn
                    .nFramesEncoded
                    as usize] as libc::c_int
                    == 0 as libc::c_int
                {
                    silk_stereo_encode_mid_only(
                        psRangeEnc,
                        (*psEnc).sStereo.mid_only_flags[(*psEnc).state_Fxx
                            [0 as libc::c_int as usize]
                            .sCmn
                            .nFramesEncoded
                            as usize],
                    );
                }
            }
        } else {
            memcpy(
                ((*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.inputBuf).as_mut_ptr()
                    as *mut libc::c_void,
                ((*psEnc).sStereo.sMid).as_mut_ptr() as *const libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
            );
            memcpy(
                ((*psEnc).sStereo.sMid).as_mut_ptr() as *mut libc::c_void,
                &mut *((*((*psEnc).state_Fxx)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .inputBuf)
                    .as_mut_ptr()
                    .offset(
                        (*((*psEnc).state_Fxx)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                        .sCmn
                        .frame_length as isize,
                    ) as *mut i16 as *const libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
            );
        }
        silk_encode_do_VAD_FLP(
            &mut *((*psEnc).state_Fxx)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            activity,
        );
        n = 0 as libc::c_int;
        while n < (*encControl).nChannelsInternal {
            let mut maxBits: libc::c_int = 0;
            let mut useCBR: libc::c_int = 0;
            maxBits = (*encControl).maxBits;
            if tot_blocks == 2 as libc::c_int && curr_block == 0 as libc::c_int {
                maxBits = maxBits * 3 as libc::c_int / 5 as libc::c_int;
            } else if tot_blocks == 3 as libc::c_int {
                if curr_block == 0 as libc::c_int {
                    maxBits = maxBits * 2 as libc::c_int / 5 as libc::c_int;
                } else if curr_block == 1 as libc::c_int {
                    maxBits = maxBits * 3 as libc::c_int / 4 as libc::c_int;
                }
            }
            useCBR = ((*encControl).useCBR != 0 && curr_block == tot_blocks - 1 as libc::c_int)
                as libc::c_int;
            if (*encControl).nChannelsInternal == 1 as libc::c_int {
                channelRate_bps = TargetRate_bps;
            } else {
                channelRate_bps = MStargetRates_bps[n as usize];
                if n == 0 as libc::c_int
                    && MStargetRates_bps[1 as libc::c_int as usize] > 0 as libc::c_int
                {
                    useCBR = 0 as libc::c_int;
                    maxBits -= (*encControl).maxBits / (tot_blocks * 2 as libc::c_int);
                }
            }
            if channelRate_bps > 0 as libc::c_int {
                let mut condCoding_0: libc::c_int = 0;
                silk_control_SNR(
                    &mut (*((*psEnc).state_Fxx).as_mut_ptr().offset(n as isize)).sCmn,
                    channelRate_bps,
                );
                if (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesEncoded
                    - n
                    <= 0 as libc::c_int
                {
                    condCoding_0 = CODE_INDEPENDENTLY;
                } else if n > 0 as libc::c_int && (*psEnc).prev_decode_only_middle != 0 {
                    condCoding_0 = CODE_INDEPENDENTLY_NO_LTP_SCALING;
                } else {
                    condCoding_0 = CODE_CONDITIONALLY;
                }
                ret = silk_encode_frame_FLP(
                    &mut *((*psEnc).state_Fxx).as_mut_ptr().offset(n as isize),
                    nBytesOut,
                    psRangeEnc,
                    condCoding_0,
                    maxBits,
                    useCBR,
                );
                let _ = ret != 0 as libc::c_int;
            }
            (*psEnc).state_Fxx[n as usize]
                .sCmn
                .controlled_since_last_payload = 0 as libc::c_int;
            (*psEnc).state_Fxx[n as usize].sCmn.inputBufIx = 0 as libc::c_int;
            (*psEnc).state_Fxx[n as usize].sCmn.nFramesEncoded += 1;
            n += 1;
        }
        (*psEnc).prev_decode_only_middle =
            (*psEnc).sStereo.mid_only_flags[((*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesEncoded
                - 1 as libc::c_int) as usize] as libc::c_int;
        if *nBytesOut > 0 as libc::c_int
            && (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesEncoded
                == (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesPerPacket
        {
            flags = 0 as libc::c_int;
            n = 0 as libc::c_int;
            while n < (*encControl).nChannelsInternal {
                i = 0 as libc::c_int;
                while i < (*psEnc).state_Fxx[n as usize].sCmn.nFramesPerPacket {
                    flags = ((flags as u32) << 1 as libc::c_int) as i32;
                    flags |=
                        (*psEnc).state_Fxx[n as usize].sCmn.VAD_flags[i as usize] as libc::c_int;
                    i += 1;
                }
                flags = ((flags as u32) << 1 as libc::c_int) as i32;
                flags |= (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flag as libc::c_int;
                n += 1;
            }
            if prefillFlag == 0 {
                ec_enc_patch_initial_bits(
                    psRangeEnc,
                    flags as libc::c_uint,
                    (((*psEnc).state_Fxx[0 as libc::c_int as usize]
                        .sCmn
                        .nFramesPerPacket
                        + 1 as libc::c_int)
                        * (*encControl).nChannelsInternal) as libc::c_uint,
                );
            }
            if (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.inDTX != 0
                && ((*encControl).nChannelsInternal == 1 as libc::c_int
                    || (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.inDTX != 0)
            {
                *nBytesOut = 0 as libc::c_int;
            }
            (*psEnc).nBitsExceeded += *nBytesOut * 8 as libc::c_int;
            (*psEnc).nBitsExceeded -=
                (*encControl).bitRate * (*encControl).payloadSize_ms / 1000 as libc::c_int;
            (*psEnc).nBitsExceeded = if 0 as libc::c_int > 10000 as libc::c_int {
                if (*psEnc).nBitsExceeded > 0 as libc::c_int {
                    0 as libc::c_int
                } else if (*psEnc).nBitsExceeded < 10000 as libc::c_int {
                    10000 as libc::c_int
                } else {
                    (*psEnc).nBitsExceeded
                }
            } else if (*psEnc).nBitsExceeded > 10000 as libc::c_int {
                10000 as libc::c_int
            } else if (*psEnc).nBitsExceeded < 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (*psEnc).nBitsExceeded
            };
            speech_act_thr_for_switch_Q8 = (((0.05f32
                * ((1 as libc::c_int as i64) << 8 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32 as libc::c_long
                + ((((1 as libc::c_int as libc::c_float - 0.05f32)
                    / 5000 as libc::c_int as libc::c_float
                    * ((1 as libc::c_int as i64) << 16 as libc::c_int + 8 as libc::c_int)
                        as libc::c_float) as libc::c_double
                    + 0.5f64) as i32 as libc::c_long
                    * (*psEnc).timeSinceSwitchAllowed_ms as i16 as i64
                    >> 16 as libc::c_int)) as i32;
            if (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .speech_activity_Q8
                < speech_act_thr_for_switch_Q8
            {
                (*psEnc).allowBandwidthSwitch = 1 as libc::c_int;
                (*psEnc).timeSinceSwitchAllowed_ms = 0 as libc::c_int;
            } else {
                (*psEnc).allowBandwidthSwitch = 0 as libc::c_int;
                (*psEnc).timeSinceSwitchAllowed_ms += (*encControl).payloadSize_ms;
            }
        }
        if nSamplesIn == 0 as libc::c_int {
            break;
        }
        curr_block += 1;
    }
    (*psEnc).nPrevChannelsInternal = (*encControl).nChannelsInternal;
    (*encControl).allowBandwidthSwitch = (*psEnc).allowBandwidthSwitch;
    (*encControl).inWBmodeWithoutVariableLP =
        ((*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz == 16 as libc::c_int
            && (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.sLP.mode == 0 as libc::c_int)
            as libc::c_int;
    (*encControl).internalSampleRate = (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz
        as i16 as i32
        * 1000 as libc::c_int as i16 as i32;
    (*encControl).stereoWidth_Q14 = if (*encControl).toMono != 0 {
        0 as libc::c_int
    } else {
        (*psEnc).sStereo.smth_width_Q14 as libc::c_int
    };
    if prefillFlag != 0 {
        (*encControl).payloadSize_ms = tmp_payloadSize_ms;
        (*encControl).complexity = tmp_complexity;
        n = 0 as libc::c_int;
        while n < (*encControl).nChannelsInternal {
            (*psEnc).state_Fxx[n as usize]
                .sCmn
                .controlled_since_last_payload = 0 as libc::c_int;
            (*psEnc).state_Fxx[n as usize].sCmn.prefillFlag = 0 as libc::c_int;
            n += 1;
        }
    }
    (*encControl).signalType = (*psEnc).state_Fxx[0 as libc::c_int as usize]
        .sCmn
        .indices
        .signalType as libc::c_int;
    (*encControl).offset = silk_Quantization_Offsets_Q10[((*psEnc).state_Fxx
        [0 as libc::c_int as usize]
        .sCmn
        .indices
        .signalType as libc::c_int
        >> 1 as libc::c_int) as usize][(*psEnc).state_Fxx[0 as libc::c_int as usize]
        .sCmn
        .indices
        .quantOffsetType as usize] as libc::c_int;
    return ret;
}
