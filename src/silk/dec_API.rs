use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:31"]
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
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:31"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int8_t, __int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:31"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:31"]
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
    use super::stdint_intn_h::{int8_t, int16_t, int32_t};
    use super::stdint_uintn_h::{uint8_t, uint32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/control.h:31"]
pub mod control_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:9"]
    pub struct silk_DecControlStruct {
        pub nChannelsAPI: opus_int32,
        pub nChannelsInternal: opus_int32,
        pub API_sampleRate: opus_int32,
        pub internalSampleRate: opus_int32,
        pub payloadSize_ms: libc::c_int,
        pub prevPitchLag: libc::c_int,
    }
    #[c2rust::src_loc = "39:9"]
    pub const FLAG_DECODE_NORMAL: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "41:9"]
    pub const FLAG_DECODE_LBRR: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "40:9"]
    pub const FLAG_PACKET_LOST: libc::c_int = 1 as libc::c_int;
    use super::opus_types_h::opus_int32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:31"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = opus_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: opus_uint32,
        pub end_offs: opus_uint32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: opus_uint32,
        pub rng: opus_uint32,
        pub val: opus_uint32,
        pub ext: opus_uint32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
    use super::opus_types_h::opus_uint32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:9"]
    pub struct stereo_dec_state {
        pub pred_prev_Q13: [opus_int16; 2],
        pub sMid: [opus_int16; 2],
        pub sSide: [opus_int16; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "261:9"]
    pub struct silk_decoder_state {
        pub prev_gain_Q16: opus_int32,
        pub exc_Q14: [opus_int32; 320],
        pub sLPC_Q14_buf: [opus_int32; 16],
        pub outBuf: [opus_int16; 480],
        pub lagPrev: libc::c_int,
        pub LastGainIndex: opus_int8,
        pub fs_kHz: libc::c_int,
        pub fs_API_hz: opus_int32,
        pub nb_subfr: libc::c_int,
        pub frame_length: libc::c_int,
        pub subfr_length: libc::c_int,
        pub ltp_mem_length: libc::c_int,
        pub LPC_order: libc::c_int,
        pub prevNLSF_Q15: [opus_int16; 16],
        pub first_frame_after_reset: libc::c_int,
        pub pitch_lag_low_bits_iCDF: *const opus_uint8,
        pub pitch_contour_iCDF: *const opus_uint8,
        pub nFramesDecoded: libc::c_int,
        pub nFramesPerPacket: libc::c_int,
        pub ec_prevSignalType: libc::c_int,
        pub ec_prevLagIndex: opus_int16,
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
    #[c2rust::src_loc = "232:9"]
    pub struct silk_PLC_struct {
        pub pitchL_Q8: opus_int32,
        pub LTPCoef_Q14: [opus_int16; 5],
        pub prevLPC_Q12: [opus_int16; 16],
        pub last_frame_lost: libc::c_int,
        pub rand_seed: opus_int32,
        pub randScale_Q14: opus_int16,
        pub conc_energy: opus_int32,
        pub conc_energy_shift: libc::c_int,
        pub prevLTP_scale_Q14: opus_int16,
        pub prevGain_Q16: [opus_int32; 2],
        pub fs_kHz: libc::c_int,
        pub nb_subfr: libc::c_int,
        pub subfr_length: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "249:9"]
    pub struct silk_CNG_struct {
        pub CNG_exc_buf_Q14: [opus_int32; 320],
        pub CNG_smth_NLSF_Q15: [opus_int16; 16],
        pub CNG_synth_state: [opus_int32; 16],
        pub CNG_smth_Gain_Q16: opus_int32,
        pub rand_seed: opus_int32,
        pub fs_kHz: libc::c_int,
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
    use super::opus_types_h::{opus_int16, opus_int32, opus_int8, opus_uint8};
    use super::resampler_structs_h::silk_resampler_state_struct;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:32"]
pub mod resampler_structs_h {
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
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
    use super::opus_types_h::{opus_int32, opus_int16};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/errors.h:31"]
pub mod errors_h {
    #[c2rust::src_loc = "83:9"]
    pub const SILK_DEC_INVALID_SAMPLING_FREQUENCY: libc::c_int = -(200 as libc::c_int);
    #[c2rust::src_loc = "39:9"]
    pub const SILK_NO_ERROR: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "92:9"]
    pub const SILK_DEC_INVALID_FRAME_SIZE: libc::c_int = -(203 as libc::c_int);
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.h:31"]
pub mod entdec_h {
    use super::entcode_h::ec_dec;
    extern "C" {
        #[c2rust::src_loc = "82:1"]
        pub fn ec_dec_icdf(
            _this: *mut ec_dec,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        ) -> libc::c_int;
        #[c2rust::src_loc = "72:1"]
        pub fn ec_dec_bit_logp(_this: *mut ec_dec, _logp: libc::c_uint) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:31"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "77:9"]
    pub const CODE_CONDITIONALLY: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "76:9"]
    pub const CODE_INDEPENDENTLY_NO_LTP_SCALING: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "75:9"]
    pub const CODE_INDEPENDENTLY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "70:9"]
    pub const TYPE_NO_VOICE_ACTIVITY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "67:9"]
    pub const MAX_API_FS_KHZ: libc::c_int = 48 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const TYPE_VOICED: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "42:9"]
    pub const DECODER_NUM_CHANNELS: libc::c_int = 2 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use super::entcode_h::ec_dec;
    use super::opus_types_h::{opus_int32, opus_int16};
    use super::structs_h::{silk_decoder_state, stereo_dec_state};
    extern "C" {
        #[c2rust::src_loc = "109:1"]
        pub fn silk_stereo_decode_mid_only(
            psRangeDec: *mut ec_dec,
            decode_only_mid: *mut libc::c_int,
        );
        #[c2rust::src_loc = "103:1"]
        pub fn silk_stereo_decode_pred(
            psRangeDec: *mut ec_dec,
            pred_Q13: *mut opus_int32,
        );
        #[c2rust::src_loc = "442:1"]
        pub fn silk_decode_pulses(
            psRangeDec: *mut ec_dec,
            pulses: *mut opus_int16,
            signalType: libc::c_int,
            quantOffsetType: libc::c_int,
            frame_length: libc::c_int,
        );
        #[c2rust::src_loc = "417:1"]
        pub fn silk_decode_indices(
            psDec: *mut silk_decoder_state,
            psRangeDec: *mut ec_dec,
            FrameIndex: libc::c_int,
            decode_LBRR: libc::c_int,
            condCoding: libc::c_int,
        );
        #[c2rust::src_loc = "397:1"]
        pub fn silk_decoder_set_fs(
            psDec: *mut silk_decoder_state,
            fs_kHz: libc::c_int,
            fs_API_Hz: opus_int32,
        ) -> libc::c_int;
        #[c2rust::src_loc = "65:1"]
        pub fn silk_stereo_MS_to_LR(
            state: *mut stereo_dec_state,
            x1: *mut opus_int16,
            x2: *mut opus_int16,
            pred_Q13: *const opus_int32,
            fs_kHz: libc::c_int,
            frame_length: libc::c_int,
        );
        #[c2rust::src_loc = "392:1"]
        pub fn silk_init_decoder(psDec: *mut silk_decoder_state) -> libc::c_int;
        #[c2rust::src_loc = "406:1"]
        pub fn silk_decode_frame(
            psDec: *mut silk_decoder_state,
            psRangeDec: *mut ec_dec,
            pOut: *mut opus_int16,
            pN: *mut opus_int32,
            lostFlag: libc::c_int,
            condCoding: libc::c_int,
            arch: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    use super::opus_types_h::opus_uint8;
    extern "C" {
        #[c2rust::src_loc = "93:34"]
        pub static silk_LBRR_flags_iCDF_ptr: [*const opus_uint8; 2];
    }
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(
            _: *mut libc::c_void,
            _: libc::c_int,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    use super::resampler_structs_h::silk_resampler_state_struct;
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "72:1"]
        pub fn silk_resampler(
            S: *mut silk_resampler_state_struct,
            out: *mut opus_int16,
            in_0: *const opus_int16,
            inLen: opus_int32,
        ) -> libc::c_int;
    }
}
pub use self::types_h::{__int8_t, __uint8_t, __int16_t, __int32_t, __uint32_t};
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::opus_types_h::{opus_int8, opus_uint8, opus_int16, opus_int32, opus_uint32};
pub use self::control_h::{
    silk_DecControlStruct, FLAG_DECODE_NORMAL, FLAG_DECODE_LBRR, FLAG_PACKET_LOST,
};
pub use self::entcode_h::{ec_window, ec_ctx, ec_dec};
pub use self::structs_h::{
    stereo_dec_state, silk_decoder_state, silk_PLC_struct, silk_CNG_struct,
    SideInfoIndices, silk_NLSF_CB_struct,
};
pub use self::resampler_structs_h::{
    silk_resampler_state_struct, _silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::errors_h::{
    SILK_DEC_INVALID_SAMPLING_FREQUENCY, SILK_NO_ERROR, SILK_DEC_INVALID_FRAME_SIZE,
};
use self::entdec_h::{ec_dec_icdf, ec_dec_bit_logp};
use self::arch_h::celt_fatal;
pub use self::define_h::{
    CODE_CONDITIONALLY, CODE_INDEPENDENTLY_NO_LTP_SCALING, CODE_INDEPENDENTLY,
    TYPE_NO_VOICE_ACTIVITY, MAX_API_FS_KHZ, TYPE_VOICED, DECODER_NUM_CHANNELS,
};
use self::main_h::{
    silk_stereo_decode_mid_only, silk_stereo_decode_pred, silk_decode_pulses,
    silk_decode_indices, silk_decoder_set_fs, silk_stereo_MS_to_LR, silk_init_decoder,
    silk_decode_frame,
};
use self::tables_h::silk_LBRR_flags_iCDF_ptr;
use self::string_h::{memcpy, memset};
use self::SigProc_FIX_h::silk_resampler;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "39:9"]
pub struct silk_decoder {
    pub channel_state: [silk_decoder_state; 2],
    pub sStereo: stereo_dec_state,
    pub nChannelsAPI: libc::c_int,
    pub nChannelsInternal: libc::c_int,
    pub prev_decode_only_middle: libc::c_int,
}
#[no_mangle]
#[c2rust::src_loc = "51:1"]
pub unsafe extern "C" fn silk_Get_Decoder_Size(
    decSizeBytes: *mut libc::c_int,
) -> libc::c_int {
    let ret: libc::c_int = SILK_NO_ERROR;
    *decSizeBytes = ::core::mem::size_of::<silk_decoder>() as libc::c_ulong
        as libc::c_int;
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn silk_InitDecoder(
    decState: *mut libc::c_void,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = SILK_NO_ERROR;
    let channel_state: *mut silk_decoder_state = ((*(decState as *mut silk_decoder))
        .channel_state)
        .as_mut_ptr();
    n = 0 as libc::c_int;
    while n < DECODER_NUM_CHANNELS {
        ret = silk_init_decoder(&mut *channel_state.offset(n as isize));
        n += 1;
    }
    memset(
        &mut (*(decState as *mut silk_decoder)).sStereo as *mut stereo_dec_state
            as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<stereo_dec_state>() as libc::c_ulong,
    );
    (*(decState as *mut silk_decoder)).prev_decode_only_middle = 0 as libc::c_int;
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn silk_Decode(
    decState: *mut libc::c_void,
    mut decControl: *mut silk_DecControlStruct,
    lostFlag: libc::c_int,
    newPacketFlag: libc::c_int,
    psRangeDec: *mut ec_dec,
    samplesOut: *mut opus_int16,
    nSamplesOut: *mut opus_int32,
    arch: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut decode_only_middle: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = SILK_NO_ERROR;
    let mut nSamplesOutDec: opus_int32 = 0;
    let mut LBRR_symbol: opus_int32 = 0;
    let mut samplesOut1_tmp: [*mut opus_int16; 2] = [0 as *mut opus_int16; 2];
    let mut MS_pred_Q13: [opus_int32; 2] = [0 as libc::c_int, 0];
    let mut resample_out_ptr: *mut opus_int16 = 0 as *mut opus_int16;
    let mut psDec: *mut silk_decoder = decState as *mut silk_decoder;
    let channel_state: *mut silk_decoder_state = ((*psDec).channel_state)
        .as_mut_ptr();
    let mut has_side: libc::c_int = 0;
    let mut stereo_to_mono: libc::c_int = 0;
    let mut delay_stack_alloc: libc::c_int = 0;
    if !((*decControl).nChannelsInternal == 1 as libc::c_int
        || (*decControl).nChannelsInternal == 2 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: decControl->nChannelsInternal == 1 || decControl->nChannelsInternal == 2\0"
                as *const u8 as *const libc::c_char,
            b"silk/dec_API.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
        );
    }
    if newPacketFlag != 0 {
        n = 0 as libc::c_int;
        while n < (*decControl).nChannelsInternal {
            (*channel_state.offset(n as isize)).nFramesDecoded = 0 as libc::c_int;
            n += 1;
        }
    }
    if (*decControl).nChannelsInternal > (*psDec).nChannelsInternal {
        ret += silk_init_decoder(&mut *channel_state.offset(1 as libc::c_int as isize));
    }
    stereo_to_mono = ((*decControl).nChannelsInternal == 1 as libc::c_int
        && (*psDec).nChannelsInternal == 2 as libc::c_int
        && (*decControl).internalSampleRate
            == 1000 as libc::c_int
                * (*channel_state.offset(0 as libc::c_int as isize)).fs_kHz)
        as libc::c_int;
    if (*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded
        == 0 as libc::c_int
    {
        n = 0 as libc::c_int;
        while n < (*decControl).nChannelsInternal {
            let mut fs_kHz_dec: libc::c_int = 0;
            if (*decControl).payloadSize_ms == 0 as libc::c_int {
                (*channel_state.offset(n as isize)).nFramesPerPacket = 1 as libc::c_int;
                (*channel_state.offset(n as isize)).nb_subfr = 2 as libc::c_int;
            } else if (*decControl).payloadSize_ms == 10 as libc::c_int {
                (*channel_state.offset(n as isize)).nFramesPerPacket = 1 as libc::c_int;
                (*channel_state.offset(n as isize)).nb_subfr = 2 as libc::c_int;
            } else if (*decControl).payloadSize_ms == 20 as libc::c_int {
                (*channel_state.offset(n as isize)).nFramesPerPacket = 1 as libc::c_int;
                (*channel_state.offset(n as isize)).nb_subfr = 4 as libc::c_int;
            } else if (*decControl).payloadSize_ms == 40 as libc::c_int {
                (*channel_state.offset(n as isize)).nFramesPerPacket = 2 as libc::c_int;
                (*channel_state.offset(n as isize)).nb_subfr = 4 as libc::c_int;
            } else if (*decControl).payloadSize_ms == 60 as libc::c_int {
                (*channel_state.offset(n as isize)).nFramesPerPacket = 3 as libc::c_int;
                (*channel_state.offset(n as isize)).nb_subfr = 4 as libc::c_int;
            } else {
                if 0 as libc::c_int == 0 {
                    celt_fatal(
                        b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                        b"silk/dec_API.c\0" as *const u8 as *const libc::c_char,
                        146 as libc::c_int,
                    );
                }
                return SILK_DEC_INVALID_FRAME_SIZE;
            }
            fs_kHz_dec = ((*decControl).internalSampleRate >> 10 as libc::c_int)
                + 1 as libc::c_int;
            if fs_kHz_dec != 8 as libc::c_int && fs_kHz_dec != 12 as libc::c_int
                && fs_kHz_dec != 16 as libc::c_int
            {
                if 0 as libc::c_int == 0 {
                    celt_fatal(
                        b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                        b"silk/dec_API.c\0" as *const u8 as *const libc::c_char,
                        152 as libc::c_int,
                    );
                }
                return SILK_DEC_INVALID_SAMPLING_FREQUENCY;
            }
            ret
                += silk_decoder_set_fs(
                    &mut *channel_state.offset(n as isize),
                    fs_kHz_dec,
                    (*decControl).API_sampleRate,
                );
            n += 1;
        }
    }
    if (*decControl).nChannelsAPI == 2 as libc::c_int
        && (*decControl).nChannelsInternal == 2 as libc::c_int
        && ((*psDec).nChannelsAPI == 1 as libc::c_int
            || (*psDec).nChannelsInternal == 1 as libc::c_int)
    {
        memset(
            ((*psDec).sStereo.pred_prev_Q13).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[opus_int16; 2]>() as libc::c_ulong,
        );
        memset(
            ((*psDec).sStereo.sSide).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[opus_int16; 2]>() as libc::c_ulong,
        );
        memcpy(
            &mut (*channel_state.offset(1 as libc::c_int as isize)).resampler_state
                as *mut silk_resampler_state_struct as *mut libc::c_void,
            &mut (*channel_state.offset(0 as libc::c_int as isize)).resampler_state
                as *mut silk_resampler_state_struct as *const libc::c_void,
            ::core::mem::size_of::<silk_resampler_state_struct>() as libc::c_ulong,
        );
    }
    (*psDec).nChannelsAPI = (*decControl).nChannelsAPI;
    (*psDec).nChannelsInternal = (*decControl).nChannelsInternal;
    if (*decControl).API_sampleRate > MAX_API_FS_KHZ * 1000 as libc::c_int
        || (*decControl).API_sampleRate < 8000 as libc::c_int
    {
        ret = SILK_DEC_INVALID_SAMPLING_FREQUENCY;
        return ret;
    }
    if lostFlag != FLAG_PACKET_LOST
        && (*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded
            == 0 as libc::c_int
    {
        n = 0 as libc::c_int;
        while n < (*decControl).nChannelsInternal {
            i = 0 as libc::c_int;
            while i < (*channel_state.offset(n as isize)).nFramesPerPacket {
                (*channel_state.offset(n as isize))
                    .VAD_flags[i
                    as usize] = ec_dec_bit_logp(
                    psRangeDec,
                    1 as libc::c_int as libc::c_uint,
                );
                i += 1;
            }
            (*channel_state.offset(n as isize))
                .LBRR_flag = ec_dec_bit_logp(
                psRangeDec,
                1 as libc::c_int as libc::c_uint,
            );
            n += 1;
        }
        n = 0 as libc::c_int;
        while n < (*decControl).nChannelsInternal {
            memset(
                ((*channel_state.offset(n as isize)).LBRR_flags).as_mut_ptr()
                    as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong,
            );
            if (*channel_state.offset(n as isize)).LBRR_flag != 0 {
                if (*channel_state.offset(n as isize)).nFramesPerPacket
                    == 1 as libc::c_int
                {
                    (*channel_state.offset(n as isize))
                        .LBRR_flags[0 as libc::c_int as usize] = 1 as libc::c_int;
                } else {
                    LBRR_symbol = ec_dec_icdf(
                        psRangeDec,
                        silk_LBRR_flags_iCDF_ptr[((*channel_state.offset(n as isize))
                            .nFramesPerPacket - 2 as libc::c_int) as usize],
                        8 as libc::c_int as libc::c_uint,
                    ) + 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < (*channel_state.offset(n as isize)).nFramesPerPacket {
                        (*channel_state.offset(n as isize))
                            .LBRR_flags[i
                            as usize] = LBRR_symbol >> i & 1 as libc::c_int;
                        i += 1;
                    }
                }
            }
            n += 1;
        }
        if lostFlag == FLAG_DECODE_NORMAL {
            i = 0 as libc::c_int;
            while i < (*channel_state.offset(0 as libc::c_int as isize)).nFramesPerPacket
            {
                n = 0 as libc::c_int;
                while n < (*decControl).nChannelsInternal {
                    if (*channel_state.offset(n as isize)).LBRR_flags[i as usize] != 0 {
                        let mut pulses: [opus_int16; 320] = [0; 320];
                        let mut condCoding: libc::c_int = 0;
                        if (*decControl).nChannelsInternal == 2 as libc::c_int
                            && n == 0 as libc::c_int
                        {
                            silk_stereo_decode_pred(
                                psRangeDec,
                                MS_pred_Q13.as_mut_ptr(),
                            );
                            if (*channel_state.offset(1 as libc::c_int as isize))
                                .LBRR_flags[i as usize] == 0 as libc::c_int
                            {
                                silk_stereo_decode_mid_only(
                                    psRangeDec,
                                    &mut decode_only_middle,
                                );
                            }
                        }
                        if i > 0 as libc::c_int
                            && (*channel_state.offset(n as isize))
                                .LBRR_flags[(i - 1 as libc::c_int) as usize] != 0
                        {
                            condCoding = CODE_CONDITIONALLY;
                        } else {
                            condCoding = CODE_INDEPENDENTLY;
                        }
                        silk_decode_indices(
                            &mut *channel_state.offset(n as isize),
                            psRangeDec,
                            i,
                            1 as libc::c_int,
                            condCoding,
                        );
                        silk_decode_pulses(
                            psRangeDec,
                            pulses.as_mut_ptr(),
                            (*channel_state.offset(n as isize)).indices.signalType
                                as libc::c_int,
                            (*channel_state.offset(n as isize)).indices.quantOffsetType
                                as libc::c_int,
                            (*channel_state.offset(n as isize)).frame_length,
                        );
                    }
                    n += 1;
                }
                i += 1;
            }
        }
    }
    if (*decControl).nChannelsInternal == 2 as libc::c_int {
        if lostFlag == FLAG_DECODE_NORMAL
            || lostFlag == FLAG_DECODE_LBRR
                && (*channel_state.offset(0 as libc::c_int as isize))
                    .LBRR_flags[(*channel_state.offset(0 as libc::c_int as isize))
                    .nFramesDecoded as usize] == 1 as libc::c_int
        {
            silk_stereo_decode_pred(psRangeDec, MS_pred_Q13.as_mut_ptr());
            if lostFlag == FLAG_DECODE_NORMAL
                && (*channel_state.offset(1 as libc::c_int as isize))
                    .VAD_flags[(*channel_state.offset(0 as libc::c_int as isize))
                    .nFramesDecoded as usize] == 0 as libc::c_int
                || lostFlag == FLAG_DECODE_LBRR
                    && (*channel_state.offset(1 as libc::c_int as isize))
                        .LBRR_flags[(*channel_state.offset(0 as libc::c_int as isize))
                        .nFramesDecoded as usize] == 0 as libc::c_int
            {
                silk_stereo_decode_mid_only(psRangeDec, &mut decode_only_middle);
            } else {
                decode_only_middle = 0 as libc::c_int;
            }
        } else {
            n = 0 as libc::c_int;
            while n < 2 as libc::c_int {
                MS_pred_Q13[n
                    as usize] = (*psDec).sStereo.pred_prev_Q13[n as usize] as opus_int32;
                n += 1;
            }
        }
    }
    if (*decControl).nChannelsInternal == 2 as libc::c_int
        && decode_only_middle == 0 as libc::c_int
        && (*psDec).prev_decode_only_middle == 1 as libc::c_int
    {
        memset(
            ((*psDec).channel_state[1 as libc::c_int as usize].outBuf).as_mut_ptr()
                as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[opus_int16; 480]>() as libc::c_ulong,
        );
        memset(
            ((*psDec).channel_state[1 as libc::c_int as usize].sLPC_Q14_buf).as_mut_ptr()
                as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[opus_int32; 16]>() as libc::c_ulong,
        );
        (*psDec).channel_state[1 as libc::c_int as usize].lagPrev = 100 as libc::c_int;
        (*psDec)
            .channel_state[1 as libc::c_int as usize]
            .LastGainIndex = 10 as libc::c_int as opus_int8;
        (*psDec)
            .channel_state[1 as libc::c_int as usize]
            .prevSignalType = TYPE_NO_VOICE_ACTIVITY;
        (*psDec)
            .channel_state[1 as libc::c_int as usize]
            .first_frame_after_reset = 1 as libc::c_int;
    }
    delay_stack_alloc = ((*decControl).internalSampleRate
        * (*decControl).nChannelsInternal
        < (*decControl).API_sampleRate * (*decControl).nChannelsAPI) as libc::c_int;
    let vla = (if delay_stack_alloc != 0 {
        1 as libc::c_int
    } else {
        (*decControl).nChannelsInternal
            * ((*channel_state.offset(0 as libc::c_int as isize)).frame_length
                + 2 as libc::c_int)
    }) as usize;
    let mut samplesOut1_tmp_storage1: Vec::<opus_int16> = ::std::vec::from_elem(0, vla);
    if delay_stack_alloc != 0 {
        samplesOut1_tmp[0 as libc::c_int as usize] = samplesOut;
        samplesOut1_tmp[1 as libc::c_int
            as usize] = samplesOut
            .offset(
                (*channel_state.offset(0 as libc::c_int as isize)).frame_length as isize,
            )
            .offset(2 as libc::c_int as isize);
    } else {
        samplesOut1_tmp[0 as libc::c_int
            as usize] = samplesOut1_tmp_storage1.as_mut_ptr();
        samplesOut1_tmp[1 as libc::c_int
            as usize] = samplesOut1_tmp_storage1
            .as_mut_ptr()
            .offset(
                (*channel_state.offset(0 as libc::c_int as isize)).frame_length as isize,
            )
            .offset(2 as libc::c_int as isize);
    }
    if lostFlag == FLAG_DECODE_NORMAL {
        has_side = (decode_only_middle == 0) as libc::c_int;
    } else {
        has_side = ((*psDec).prev_decode_only_middle == 0
            || (*decControl).nChannelsInternal == 2 as libc::c_int
                && lostFlag == FLAG_DECODE_LBRR
                && (*channel_state.offset(1 as libc::c_int as isize))
                    .LBRR_flags[(*channel_state.offset(1 as libc::c_int as isize))
                    .nFramesDecoded as usize] == 1 as libc::c_int) as libc::c_int;
    }
    n = 0 as libc::c_int;
    while n < (*decControl).nChannelsInternal {
        if n == 0 as libc::c_int || has_side != 0 {
            let mut FrameIndex: libc::c_int = 0;
            let mut condCoding_0: libc::c_int = 0;
            FrameIndex = (*channel_state.offset(0 as libc::c_int as isize))
                .nFramesDecoded - n;
            if FrameIndex <= 0 as libc::c_int {
                condCoding_0 = CODE_INDEPENDENTLY;
            } else if lostFlag == FLAG_DECODE_LBRR {
                condCoding_0 = if (*channel_state.offset(n as isize))
                    .LBRR_flags[(FrameIndex - 1 as libc::c_int) as usize] != 0
                {
                    CODE_CONDITIONALLY
                } else {
                    CODE_INDEPENDENTLY
                };
            } else if n > 0 as libc::c_int && (*psDec).prev_decode_only_middle != 0 {
                condCoding_0 = CODE_INDEPENDENTLY_NO_LTP_SCALING;
            } else {
                condCoding_0 = CODE_CONDITIONALLY;
            }
            ret
                += silk_decode_frame(
                    &mut *channel_state.offset(n as isize),
                    psRangeDec,
                    &mut *(*samplesOut1_tmp.as_mut_ptr().offset(n as isize))
                        .offset(2 as libc::c_int as isize),
                    &mut nSamplesOutDec,
                    lostFlag,
                    condCoding_0,
                    arch,
                );
        } else {
            memset(
                &mut *(*samplesOut1_tmp.as_mut_ptr().offset(n as isize))
                    .offset(2 as libc::c_int as isize) as *mut opus_int16
                    as *mut libc::c_void,
                0 as libc::c_int,
                (nSamplesOutDec as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
            );
        }
        let ref mut fresh0 = (*channel_state.offset(n as isize)).nFramesDecoded;
        *fresh0 += 1;
        n += 1;
    }
    if (*decControl).nChannelsAPI == 2 as libc::c_int
        && (*decControl).nChannelsInternal == 2 as libc::c_int
    {
        silk_stereo_MS_to_LR(
            &mut (*psDec).sStereo,
            samplesOut1_tmp[0 as libc::c_int as usize],
            samplesOut1_tmp[1 as libc::c_int as usize],
            MS_pred_Q13.as_mut_ptr() as *const opus_int32,
            (*channel_state.offset(0 as libc::c_int as isize)).fs_kHz,
            nSamplesOutDec,
        );
    } else {
        memcpy(
            samplesOut1_tmp[0 as libc::c_int as usize] as *mut libc::c_void,
            ((*psDec).sStereo.sMid).as_mut_ptr() as *const libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
        );
        memcpy(
            ((*psDec).sStereo.sMid).as_mut_ptr() as *mut libc::c_void,
            &mut *(*samplesOut1_tmp.as_mut_ptr().offset(0 as libc::c_int as isize))
                .offset(nSamplesOutDec as isize) as *mut opus_int16
                as *const libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
        );
    }
    *nSamplesOut = nSamplesOutDec * (*decControl).API_sampleRate
        / ((*channel_state.offset(0 as libc::c_int as isize)).fs_kHz as opus_int16
            as opus_int32 * 1000 as libc::c_int as opus_int16 as opus_int32);
    let vla_0 = (if (*decControl).nChannelsAPI == 2 as libc::c_int {
        *nSamplesOut
    } else {
        1 as libc::c_int
    }) as usize;
    let mut samplesOut2_tmp: Vec::<opus_int16> = ::std::vec::from_elem(0, vla_0);
    if (*decControl).nChannelsAPI == 2 as libc::c_int {
        resample_out_ptr = samplesOut2_tmp.as_mut_ptr();
    } else {
        resample_out_ptr = samplesOut;
    }
    let vla_1 = (if delay_stack_alloc != 0 {
        (*decControl).nChannelsInternal
            * ((*channel_state.offset(0 as libc::c_int as isize)).frame_length
                + 2 as libc::c_int)
    } else {
        1 as libc::c_int
    }) as usize;
    let mut samplesOut1_tmp_storage2: Vec::<opus_int16> = ::std::vec::from_elem(
        0,
        vla_1,
    );
    if delay_stack_alloc != 0 {
        memcpy(
            samplesOut1_tmp_storage2.as_mut_ptr() as *mut libc::c_void,
            samplesOut as *const libc::c_void,
            (((*decControl).nChannelsInternal
                * ((*channel_state.offset(0 as libc::c_int as isize)).frame_length
                    + 2 as libc::c_int)) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * samplesOut1_tmp_storage2.as_mut_ptr().offset_from(samplesOut)
                            as libc::c_long) as libc::c_ulong,
                ),
        );
        samplesOut1_tmp[0 as libc::c_int
            as usize] = samplesOut1_tmp_storage2.as_mut_ptr();
        samplesOut1_tmp[1 as libc::c_int
            as usize] = samplesOut1_tmp_storage2
            .as_mut_ptr()
            .offset(
                (*channel_state.offset(0 as libc::c_int as isize)).frame_length as isize,
            )
            .offset(2 as libc::c_int as isize);
    }
    n = 0 as libc::c_int;
    while n
        < (if (*decControl).nChannelsAPI < (*decControl).nChannelsInternal {
            (*decControl).nChannelsAPI
        } else {
            (*decControl).nChannelsInternal
        })
    {
        ret
            += silk_resampler(
                &mut (*channel_state.offset(n as isize)).resampler_state,
                resample_out_ptr,
                &mut *(*samplesOut1_tmp.as_mut_ptr().offset(n as isize))
                    .offset(1 as libc::c_int as isize) as *mut opus_int16
                    as *const opus_int16,
                nSamplesOutDec,
            );
        if (*decControl).nChannelsAPI == 2 as libc::c_int {
            i = 0 as libc::c_int;
            while i < *nSamplesOut {
                *samplesOut
                    .offset(
                        (n + 2 as libc::c_int * i) as isize,
                    ) = *resample_out_ptr.offset(i as isize);
                i += 1;
            }
        }
        n += 1;
    }
    if (*decControl).nChannelsAPI == 2 as libc::c_int
        && (*decControl).nChannelsInternal == 1 as libc::c_int
    {
        if stereo_to_mono != 0 {
            ret
                += silk_resampler(
                    &mut (*channel_state.offset(1 as libc::c_int as isize))
                        .resampler_state,
                    resample_out_ptr,
                    &mut *(*samplesOut1_tmp
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as *mut opus_int16
                        as *const opus_int16,
                    nSamplesOutDec,
                );
            i = 0 as libc::c_int;
            while i < *nSamplesOut {
                *samplesOut
                    .offset(
                        (1 as libc::c_int + 2 as libc::c_int * i) as isize,
                    ) = *resample_out_ptr.offset(i as isize);
                i += 1;
            }
        } else {
            i = 0 as libc::c_int;
            while i < *nSamplesOut {
                *samplesOut
                    .offset(
                        (1 as libc::c_int + 2 as libc::c_int * i) as isize,
                    ) = *samplesOut
                    .offset((0 as libc::c_int + 2 as libc::c_int * i) as isize);
                i += 1;
            }
        }
    }
    if (*channel_state.offset(0 as libc::c_int as isize)).prevSignalType == TYPE_VOICED {
        let mult_tab: [libc::c_int; 3] = [
            6 as libc::c_int,
            4 as libc::c_int,
            3 as libc::c_int,
        ];
        (*decControl)
            .prevPitchLag = (*channel_state.offset(0 as libc::c_int as isize)).lagPrev
            * mult_tab[((*channel_state.offset(0 as libc::c_int as isize)).fs_kHz
                - 8 as libc::c_int >> 2 as libc::c_int) as usize];
    } else {
        (*decControl).prevPitchLag = 0 as libc::c_int;
    }
    if lostFlag == FLAG_PACKET_LOST {
        i = 0 as libc::c_int;
        while i < (*psDec).nChannelsInternal {
            (*psDec)
                .channel_state[i as usize]
                .LastGainIndex = 10 as libc::c_int as opus_int8;
            i += 1;
        }
    } else {
        (*psDec).prev_decode_only_middle = decode_only_middle;
    }
    return ret;
}
