use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::int32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/control.h:32"]
pub mod control_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:9"]
    pub struct silk_EncControlStruct {
        pub nChannelsAPI: opus_int32,
        pub nChannelsInternal: opus_int32,
        pub API_sampleRate: opus_int32,
        pub maxInternalSampleRate: opus_int32,
        pub minInternalSampleRate: opus_int32,
        pub desiredInternalSampleRate: opus_int32,
        pub payloadSize_ms: libc::c_int,
        pub bitRate: opus_int32,
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
        pub internalSampleRate: opus_int32,
        pub allowBandwidthSwitch: libc::c_int,
        pub inWBmodeWithoutVariableLP: libc::c_int,
        pub stereoWidth_Q14: libc::c_int,
        pub switchReady: libc::c_int,
        pub signalType: libc::c_int,
        pub offset: libc::c_int,
    }
    use super::opus_types_h::opus_int32;
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/errors.h:32"]
pub mod errors_h {
    #[c2rust::src_loc = "39:9"]
    pub const SILK_NO_ERROR: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "61:9"]
    pub const SILK_ENC_INVALID_COMPLEXITY_SETTING: libc::c_int = -(106 as libc::c_int);
    #[c2rust::src_loc = "76:9"]
    pub const SILK_ENC_INVALID_NUMBER_OF_CHANNELS_ERROR: libc::c_int = -(111
        as libc::c_int);
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "40:9"]
    pub const ENCODER_NUM_CHANNELS: libc::c_int = 2 as libc::c_int;
}
pub use self::types_h::__int32_t;
pub use self::stdint_intn_h::int32_t;
pub use self::opus_types_h::opus_int32;
pub use self::control_h::silk_EncControlStruct;
use self::arch_h::celt_fatal;
pub use self::errors_h::{
    SILK_NO_ERROR, SILK_ENC_INVALID_COMPLEXITY_SETTING,
    SILK_ENC_INVALID_NUMBER_OF_CHANNELS_ERROR, SILK_ENC_INVALID_INBAND_FEC_SETTING,
    SILK_ENC_INVALID_CBR_SETTING, SILK_ENC_INVALID_DTX_SETTING,
    SILK_ENC_INVALID_LOSS_RATE, SILK_ENC_PACKET_SIZE_NOT_SUPPORTED,
    SILK_ENC_FS_NOT_SUPPORTED,
};
pub use self::define_h::ENCODER_NUM_CHANNELS;
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn check_control_input(
    encControl: *mut silk_EncControlStruct,
) -> libc::c_int {
    if encControl.is_null() {
        celt_fatal(
            b"assertion failed: encControl != NULL\0" as *const u8
                as *const libc::c_char,
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
    if (*encControl).useDTX < 0 as libc::c_int || (*encControl).useDTX > 1 as libc::c_int
    {
        if 0 as libc::c_int == 0 {
            celt_fatal(
                b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                b"silk/check_control_input.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int,
            );
        }
        return SILK_ENC_INVALID_DTX_SETTING;
    }
    if (*encControl).useCBR < 0 as libc::c_int || (*encControl).useCBR > 1 as libc::c_int
    {
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
    if (*encControl).complexity < 0 as libc::c_int
        || (*encControl).complexity > 10 as libc::c_int
    {
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
