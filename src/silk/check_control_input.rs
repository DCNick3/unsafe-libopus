#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/errors.h:32"]
pub mod errors_h {
    #[c2rust::src_loc = "39:9"]
    pub const SILK_NO_ERROR: i32 = 0 as i32;
    #[c2rust::src_loc = "61:9"]
    pub const SILK_ENC_INVALID_COMPLEXITY_SETTING: i32 = -(106 as i32);
    #[c2rust::src_loc = "76:9"]
    pub const SILK_ENC_INVALID_NUMBER_OF_CHANNELS_ERROR: i32 = -(111 as i32);
    #[c2rust::src_loc = "64:9"]
    pub const SILK_ENC_INVALID_INBAND_FEC_SETTING: i32 = -(107 as i32);
    #[c2rust::src_loc = "70:9"]
    pub const SILK_ENC_INVALID_CBR_SETTING: i32 = -(109 as i32);
    #[c2rust::src_loc = "67:9"]
    pub const SILK_ENC_INVALID_DTX_SETTING: i32 = -(108 as i32);
    #[c2rust::src_loc = "58:9"]
    pub const SILK_ENC_INVALID_LOSS_RATE: i32 = -(105 as i32);
    #[c2rust::src_loc = "52:9"]
    pub const SILK_ENC_PACKET_SIZE_NOT_SUPPORTED: i32 = -(103 as i32);
    #[c2rust::src_loc = "49:9"]
    pub const SILK_ENC_FS_NOT_SUPPORTED: i32 = -(102 as i32);
}

pub use self::errors_h::{
    SILK_ENC_FS_NOT_SUPPORTED, SILK_ENC_INVALID_CBR_SETTING, SILK_ENC_INVALID_COMPLEXITY_SETTING,
    SILK_ENC_INVALID_DTX_SETTING, SILK_ENC_INVALID_INBAND_FEC_SETTING, SILK_ENC_INVALID_LOSS_RATE,
    SILK_ENC_INVALID_NUMBER_OF_CHANNELS_ERROR, SILK_ENC_PACKET_SIZE_NOT_SUPPORTED, SILK_NO_ERROR,
};
use crate::celt::celt::celt_fatal;
use crate::silk::define::ENCODER_NUM_CHANNELS;
use crate::silk::enc_API::silk_EncControlStruct;

#[c2rust::src_loc = "37:1"]
pub unsafe fn check_control_input(encControl: *mut silk_EncControlStruct) -> i32 {
    if encControl.is_null() {
        celt_fatal(
            b"assertion failed: encControl != NULL\0" as *const u8 as *const i8,
            b"silk/check_control_input.c\0" as *const u8 as *const i8,
            41 as i32,
        );
    }
    if (*encControl).API_sampleRate != 8000 as i32
        && (*encControl).API_sampleRate != 12000 as i32
        && (*encControl).API_sampleRate != 16000 as i32
        && (*encControl).API_sampleRate != 24000 as i32
        && (*encControl).API_sampleRate != 32000 as i32
        && (*encControl).API_sampleRate != 44100 as i32
        && (*encControl).API_sampleRate != 48000 as i32
        || (*encControl).desiredInternalSampleRate != 8000 as i32
            && (*encControl).desiredInternalSampleRate != 12000 as i32
            && (*encControl).desiredInternalSampleRate != 16000 as i32
        || (*encControl).maxInternalSampleRate != 8000 as i32
            && (*encControl).maxInternalSampleRate != 12000 as i32
            && (*encControl).maxInternalSampleRate != 16000 as i32
        || (*encControl).minInternalSampleRate != 8000 as i32
            && (*encControl).minInternalSampleRate != 12000 as i32
            && (*encControl).minInternalSampleRate != 16000 as i32
        || (*encControl).minInternalSampleRate > (*encControl).desiredInternalSampleRate
        || (*encControl).maxInternalSampleRate < (*encControl).desiredInternalSampleRate
        || (*encControl).minInternalSampleRate > (*encControl).maxInternalSampleRate
    {
        if 0 as i32 == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const i8,
                b"silk/check_control_input.c\0" as *const u8 as *const i8,
                62 as i32,
            );
        }
        return SILK_ENC_FS_NOT_SUPPORTED;
    }
    if (*encControl).payloadSize_ms != 10 as i32
        && (*encControl).payloadSize_ms != 20 as i32
        && (*encControl).payloadSize_ms != 40 as i32
        && (*encControl).payloadSize_ms != 60 as i32
    {
        if 0 as i32 == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const i8,
                b"silk/check_control_input.c\0" as *const u8 as *const i8,
                69 as i32,
            );
        }
        return SILK_ENC_PACKET_SIZE_NOT_SUPPORTED;
    }
    if (*encControl).packetLossPercentage < 0 as i32
        || (*encControl).packetLossPercentage > 100 as i32
    {
        if 0 as i32 == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const i8,
                b"silk/check_control_input.c\0" as *const u8 as *const i8,
                73 as i32,
            );
        }
        return SILK_ENC_INVALID_LOSS_RATE;
    }
    if (*encControl).useDTX < 0 as i32 || (*encControl).useDTX > 1 as i32 {
        if 0 as i32 == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const i8,
                b"silk/check_control_input.c\0" as *const u8 as *const i8,
                77 as i32,
            );
        }
        return SILK_ENC_INVALID_DTX_SETTING;
    }
    if (*encControl).useCBR < 0 as i32 || (*encControl).useCBR > 1 as i32 {
        if 0 as i32 == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const i8,
                b"silk/check_control_input.c\0" as *const u8 as *const i8,
                81 as i32,
            );
        }
        return SILK_ENC_INVALID_CBR_SETTING;
    }
    if (*encControl).useInBandFEC < 0 as i32 || (*encControl).useInBandFEC > 1 as i32 {
        if 0 as i32 == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const i8,
                b"silk/check_control_input.c\0" as *const u8 as *const i8,
                85 as i32,
            );
        }
        return SILK_ENC_INVALID_INBAND_FEC_SETTING;
    }
    if (*encControl).nChannelsAPI < 1 as i32 || (*encControl).nChannelsAPI > ENCODER_NUM_CHANNELS {
        if 0 as i32 == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const i8,
                b"silk/check_control_input.c\0" as *const u8 as *const i8,
                89 as i32,
            );
        }
        return SILK_ENC_INVALID_NUMBER_OF_CHANNELS_ERROR;
    }
    if (*encControl).nChannelsInternal < 1 as i32
        || (*encControl).nChannelsInternal > ENCODER_NUM_CHANNELS
    {
        if 0 as i32 == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const i8,
                b"silk/check_control_input.c\0" as *const u8 as *const i8,
                93 as i32,
            );
        }
        return SILK_ENC_INVALID_NUMBER_OF_CHANNELS_ERROR;
    }
    if (*encControl).nChannelsInternal > (*encControl).nChannelsAPI {
        if 0 as i32 == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const i8,
                b"silk/check_control_input.c\0" as *const u8 as *const i8,
                97 as i32,
            );
        }
        return SILK_ENC_INVALID_NUMBER_OF_CHANNELS_ERROR;
    }
    if (*encControl).complexity < 0 as i32 || (*encControl).complexity > 10 as i32 {
        if 0 as i32 == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const i8,
                b"silk/check_control_input.c\0" as *const u8 as *const i8,
                101 as i32,
            );
        }
        return SILK_ENC_INVALID_COMPLEXITY_SETTING;
    }
    return SILK_NO_ERROR;
}
