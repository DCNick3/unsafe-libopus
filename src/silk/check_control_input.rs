#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/errors.h:32"]
pub mod errors_h {
    #[c2rust::src_loc = "39:9"]
    pub const SILK_NO_ERROR: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "61:9"]
    pub const SILK_ENC_INVALID_COMPLEXITY_SETTING: libc::c_int = -(106 as libc::c_int);
    #[c2rust::src_loc = "76:9"]
    pub const SILK_ENC_INVALID_NUMBER_OF_CHANNELS_ERROR: libc::c_int = -(111 as libc::c_int);
    #[c2rust::src_loc = "64:9"]
    pub const SILK_ENC_INVALID_INBAND_FEC_SETTING: libc::c_int = -(107 as libc::c_int);
    #[c2rust::src_loc = "70:9"]
    pub const SILK_ENC_INVALID_CBR_SETTING: libc::c_int = -(109 as libc::c_int);
    #[c2rust::src_loc = "67:9"]
    pub const SILK_ENC_INVALID_DTX_SETTING: libc::c_int = -(108 as libc::c_int);
    #[c2rust::src_loc = "58:9"]
    pub const SILK_ENC_INVALID_LOSS_RATE: libc::c_int = -(105 as libc::c_int);
    #[c2rust::src_loc = "52:9"]
    pub const SILK_ENC_PACKET_SIZE_NOT_SUPPORTED: libc::c_int = -(103 as libc::c_int);
    #[c2rust::src_loc = "49:9"]
    pub const SILK_ENC_FS_NOT_SUPPORTED: libc::c_int = -(102 as libc::c_int);
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
pub unsafe fn check_control_input(encControl: *mut silk_EncControlStruct) -> libc::c_int {
    if encControl.is_null() {
        celt_fatal(
            b"assertion failed: encControl != NULL\0" as *const u8 as *const libc::c_char,
            b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
        );
    }
    if (*encControl).API_sampleRate != 8000 as libc::c_int
        && (*encControl).API_sampleRate != 12000 as libc::c_int
        && (*encControl).API_sampleRate != 16000 as libc::c_int
        && (*encControl).API_sampleRate != 24000 as libc::c_int
        && (*encControl).API_sampleRate != 32000 as libc::c_int
        && (*encControl).API_sampleRate != 44100 as libc::c_int
        && (*encControl).API_sampleRate != 48000 as libc::c_int
        || (*encControl).desiredInternalSampleRate != 8000 as libc::c_int
            && (*encControl).desiredInternalSampleRate != 12000 as libc::c_int
            && (*encControl).desiredInternalSampleRate != 16000 as libc::c_int
        || (*encControl).maxInternalSampleRate != 8000 as libc::c_int
            && (*encControl).maxInternalSampleRate != 12000 as libc::c_int
            && (*encControl).maxInternalSampleRate != 16000 as libc::c_int
        || (*encControl).minInternalSampleRate != 8000 as libc::c_int
            && (*encControl).minInternalSampleRate != 12000 as libc::c_int
            && (*encControl).minInternalSampleRate != 16000 as libc::c_int
        || (*encControl).minInternalSampleRate > (*encControl).desiredInternalSampleRate
        || (*encControl).maxInternalSampleRate < (*encControl).desiredInternalSampleRate
        || (*encControl).minInternalSampleRate > (*encControl).maxInternalSampleRate
    {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int,
            );
        }
        return SILK_ENC_FS_NOT_SUPPORTED;
    }
    if (*encControl).payloadSize_ms != 10 as libc::c_int
        && (*encControl).payloadSize_ms != 20 as libc::c_int
        && (*encControl).payloadSize_ms != 40 as libc::c_int
        && (*encControl).payloadSize_ms != 60 as libc::c_int
    {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
                69 as libc::c_int,
            );
        }
        return SILK_ENC_PACKET_SIZE_NOT_SUPPORTED;
    }
    if (*encControl).packetLossPercentage < 0 as libc::c_int
        || (*encControl).packetLossPercentage > 100 as libc::c_int
    {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
                73 as libc::c_int,
            );
        }
        return SILK_ENC_INVALID_LOSS_RATE;
    }
    if (*encControl).useDTX < 0 as libc::c_int || (*encControl).useDTX > 1 as libc::c_int {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int,
            );
        }
        return SILK_ENC_INVALID_DTX_SETTING;
    }
    if (*encControl).useCBR < 0 as libc::c_int || (*encControl).useCBR > 1 as libc::c_int {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
                81 as libc::c_int,
            );
        }
        return SILK_ENC_INVALID_CBR_SETTING;
    }
    if (*encControl).useInBandFEC < 0 as libc::c_int
        || (*encControl).useInBandFEC > 1 as libc::c_int
    {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
            );
        }
        return SILK_ENC_INVALID_INBAND_FEC_SETTING;
    }
    if (*encControl).nChannelsAPI < 1 as libc::c_int
        || (*encControl).nChannelsAPI > ENCODER_NUM_CHANNELS
    {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
                89 as libc::c_int,
            );
        }
        return SILK_ENC_INVALID_NUMBER_OF_CHANNELS_ERROR;
    }
    if (*encControl).nChannelsInternal < 1 as libc::c_int
        || (*encControl).nChannelsInternal > ENCODER_NUM_CHANNELS
    {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int,
            );
        }
        return SILK_ENC_INVALID_NUMBER_OF_CHANNELS_ERROR;
    }
    if (*encControl).nChannelsInternal > (*encControl).nChannelsAPI {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
                97 as libc::c_int,
            );
        }
        return SILK_ENC_INVALID_NUMBER_OF_CHANNELS_ERROR;
    }
    if (*encControl).complexity < 0 as libc::c_int || (*encControl).complexity > 10 as libc::c_int {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
                101 as libc::c_int,
            );
        }
        return SILK_ENC_INVALID_COMPLEXITY_SETTING;
    }
    return SILK_NO_ERROR;
}
