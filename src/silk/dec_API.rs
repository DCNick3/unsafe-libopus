use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "126:9"]
pub struct silk_DecControlStruct {
    pub nChannelsAPI: i32,
    pub nChannelsInternal: i32,
    pub API_sampleRate: i32,
    pub internalSampleRate: i32,
    pub payloadSize_ms: libc::c_int,
    pub prevPitchLag: libc::c_int,
}
#[c2rust::src_loc = "39:9"]
pub const FLAG_DECODE_NORMAL: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "41:9"]
pub const FLAG_DECODE_LBRR: libc::c_int = 2 as libc::c_int;
#[c2rust::src_loc = "40:9"]
pub const FLAG_PACKET_LOST: libc::c_int = 1 as libc::c_int;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:9"]
    pub struct stereo_dec_state {
        pub pred_prev_Q13: [i16; 2],
        pub sMid: [i16; 2],
        pub sSide: [i16; 2],
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:31"]
pub mod arch_h {}
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
    use super::structs_h::{silk_decoder_state, stereo_dec_state};
    use crate::celt::entdec::ec_dec;
    extern "C" {
        #[c2rust::src_loc = "109:1"]
        pub fn silk_stereo_decode_mid_only(
            psRangeDec: *mut ec_dec,
            decode_only_mid: *mut libc::c_int,
        );
        #[c2rust::src_loc = "103:1"]
        pub fn silk_stereo_decode_pred(psRangeDec: *mut ec_dec, pred_Q13: *mut i32);
        #[c2rust::src_loc = "442:1"]
        pub fn silk_decode_pulses(
            psRangeDec: *mut ec_dec,
            pulses: *mut i16,
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
            fs_API_Hz: i32,
        ) -> libc::c_int;
        #[c2rust::src_loc = "65:1"]
        pub fn silk_stereo_MS_to_LR(
            state: *mut stereo_dec_state,
            x1: *mut i16,
            x2: *mut i16,
            pred_Q13: *const i32,
            fs_kHz: libc::c_int,
            frame_length: libc::c_int,
        );
        #[c2rust::src_loc = "392:1"]
        pub fn silk_init_decoder(psDec: *mut silk_decoder_state) -> libc::c_int;
        #[c2rust::src_loc = "406:1"]
        pub fn silk_decode_frame(
            psDec: *mut silk_decoder_state,
            psRangeDec: *mut ec_dec,
            pOut: *mut i16,
            pN: *mut i32,
            lostFlag: libc::c_int,
            condCoding: libc::c_int,
            arch: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "93:34"]
        pub static silk_LBRR_flags_iCDF_ptr: [*const u8; 2];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    use super::resampler_structs_h::silk_resampler_state_struct;
    extern "C" {
        #[c2rust::src_loc = "72:1"]
        pub fn silk_resampler(
            S: *mut silk_resampler_state_struct,
            out: *mut i16,
            in_0: *const i16,
            inLen: i32,
        ) -> libc::c_int;
    }
}
pub use self::define_h::{
    CODE_CONDITIONALLY, CODE_INDEPENDENTLY, CODE_INDEPENDENTLY_NO_LTP_SCALING,
    DECODER_NUM_CHANNELS, MAX_API_FS_KHZ, TYPE_NO_VOICE_ACTIVITY, TYPE_VOICED,
};
pub use self::errors_h::{
    SILK_DEC_INVALID_FRAME_SIZE, SILK_DEC_INVALID_SAMPLING_FREQUENCY, SILK_NO_ERROR,
};
use self::main_h::{
    silk_decode_frame, silk_decode_indices, silk_decode_pulses, silk_decoder_set_fs,
    silk_init_decoder, silk_stereo_MS_to_LR, silk_stereo_decode_mid_only, silk_stereo_decode_pred,
};
pub use self::resampler_structs_h::{
    silk_resampler_state_struct, C2RustUnnamed, _silk_resampler_state_struct,
};
use crate::celt::celt::celt_fatal;
use crate::celt::entdec::{ec_dec, ec_dec_bit_logp, ec_dec_icdf};

pub use self::structs_h::{
    silk_CNG_struct, silk_NLSF_CB_struct, silk_PLC_struct, silk_decoder_state, stereo_dec_state,
    SideInfoIndices,
};
use self::tables_h::silk_LBRR_flags_iCDF_ptr;

use self::SigProc_FIX_h::silk_resampler;
use crate::externs::{memcpy, memset};
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
pub unsafe extern "C" fn silk_Get_Decoder_Size(decSizeBytes: *mut libc::c_int) -> libc::c_int {
    let ret: libc::c_int = SILK_NO_ERROR;
    *decSizeBytes = ::core::mem::size_of::<silk_decoder>() as libc::c_ulong as libc::c_int;
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn silk_InitDecoder(decState: *mut libc::c_void) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = SILK_NO_ERROR;
    let channel_state: *mut silk_decoder_state =
        ((*(decState as *mut silk_decoder)).channel_state).as_mut_ptr();
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
    samplesOut: *mut i16,
    nSamplesOut: *mut i32,
    arch: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut decode_only_middle: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = SILK_NO_ERROR;
    let mut nSamplesOutDec: i32 = 0;
    let mut LBRR_symbol: i32 = 0;
    let mut samplesOut1_tmp: [*mut i16; 2] = [0 as *mut i16; 2];
    let mut MS_pred_Q13: [i32; 2] = [0 as libc::c_int, 0];
    let mut resample_out_ptr: *mut i16 = 0 as *mut i16;
    let mut psDec: *mut silk_decoder = decState as *mut silk_decoder;
    let channel_state: *mut silk_decoder_state = ((*psDec).channel_state).as_mut_ptr();
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
            == 1000 as libc::c_int * (*channel_state.offset(0 as libc::c_int as isize)).fs_kHz)
        as libc::c_int;
    if (*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded == 0 as libc::c_int {
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
            fs_kHz_dec = ((*decControl).internalSampleRate >> 10 as libc::c_int) + 1 as libc::c_int;
            if fs_kHz_dec != 8 as libc::c_int
                && fs_kHz_dec != 12 as libc::c_int
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
            ret += silk_decoder_set_fs(
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
            ::core::mem::size_of::<[i16; 2]>() as libc::c_ulong,
        );
        memset(
            ((*psDec).sStereo.sSide).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[i16; 2]>() as libc::c_ulong,
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
        && (*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded == 0 as libc::c_int
    {
        n = 0 as libc::c_int;
        while n < (*decControl).nChannelsInternal {
            i = 0 as libc::c_int;
            while i < (*channel_state.offset(n as isize)).nFramesPerPacket {
                (*channel_state.offset(n as isize)).VAD_flags[i as usize] =
                    ec_dec_bit_logp(psRangeDec, 1 as libc::c_int as libc::c_uint);
                i += 1;
            }
            (*channel_state.offset(n as isize)).LBRR_flag =
                ec_dec_bit_logp(psRangeDec, 1 as libc::c_int as libc::c_uint);
            n += 1;
        }
        n = 0 as libc::c_int;
        while n < (*decControl).nChannelsInternal {
            memset(
                ((*channel_state.offset(n as isize)).LBRR_flags).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong,
            );
            if (*channel_state.offset(n as isize)).LBRR_flag != 0 {
                if (*channel_state.offset(n as isize)).nFramesPerPacket == 1 as libc::c_int {
                    (*channel_state.offset(n as isize)).LBRR_flags[0 as libc::c_int as usize] =
                        1 as libc::c_int;
                } else {
                    LBRR_symbol = ec_dec_icdf(
                        psRangeDec,
                        silk_LBRR_flags_iCDF_ptr[((*channel_state.offset(n as isize))
                            .nFramesPerPacket
                            - 2 as libc::c_int)
                            as usize],
                        8 as libc::c_int as libc::c_uint,
                    ) + 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < (*channel_state.offset(n as isize)).nFramesPerPacket {
                        (*channel_state.offset(n as isize)).LBRR_flags[i as usize] =
                            LBRR_symbol >> i & 1 as libc::c_int;
                        i += 1;
                    }
                }
            }
            n += 1;
        }
        if lostFlag == FLAG_DECODE_NORMAL {
            i = 0 as libc::c_int;
            while i < (*channel_state.offset(0 as libc::c_int as isize)).nFramesPerPacket {
                n = 0 as libc::c_int;
                while n < (*decControl).nChannelsInternal {
                    if (*channel_state.offset(n as isize)).LBRR_flags[i as usize] != 0 {
                        let mut pulses: [i16; 320] = [0; 320];
                        let mut condCoding: libc::c_int = 0;
                        if (*decControl).nChannelsInternal == 2 as libc::c_int
                            && n == 0 as libc::c_int
                        {
                            silk_stereo_decode_pred(psRangeDec, MS_pred_Q13.as_mut_ptr());
                            if (*channel_state.offset(1 as libc::c_int as isize)).LBRR_flags
                                [i as usize]
                                == 0 as libc::c_int
                            {
                                silk_stereo_decode_mid_only(psRangeDec, &mut decode_only_middle);
                            }
                        }
                        if i > 0 as libc::c_int
                            && (*channel_state.offset(n as isize)).LBRR_flags
                                [(i - 1 as libc::c_int) as usize]
                                != 0
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
                            (*channel_state.offset(n as isize)).indices.signalType as libc::c_int,
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
                && (*channel_state.offset(0 as libc::c_int as isize)).LBRR_flags
                    [(*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded as usize]
                    == 1 as libc::c_int
        {
            silk_stereo_decode_pred(psRangeDec, MS_pred_Q13.as_mut_ptr());
            if lostFlag == FLAG_DECODE_NORMAL
                && (*channel_state.offset(1 as libc::c_int as isize)).VAD_flags
                    [(*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded as usize]
                    == 0 as libc::c_int
                || lostFlag == FLAG_DECODE_LBRR
                    && (*channel_state.offset(1 as libc::c_int as isize)).LBRR_flags
                        [(*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded as usize]
                        == 0 as libc::c_int
            {
                silk_stereo_decode_mid_only(psRangeDec, &mut decode_only_middle);
            } else {
                decode_only_middle = 0 as libc::c_int;
            }
        } else {
            n = 0 as libc::c_int;
            while n < 2 as libc::c_int {
                MS_pred_Q13[n as usize] = (*psDec).sStereo.pred_prev_Q13[n as usize] as i32;
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
            ::core::mem::size_of::<[i16; 480]>() as libc::c_ulong,
        );
        memset(
            ((*psDec).channel_state[1 as libc::c_int as usize].sLPC_Q14_buf).as_mut_ptr()
                as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[i32; 16]>() as libc::c_ulong,
        );
        (*psDec).channel_state[1 as libc::c_int as usize].lagPrev = 100 as libc::c_int;
        (*psDec).channel_state[1 as libc::c_int as usize].LastGainIndex = 10 as libc::c_int as i8;
        (*psDec).channel_state[1 as libc::c_int as usize].prevSignalType = TYPE_NO_VOICE_ACTIVITY;
        (*psDec).channel_state[1 as libc::c_int as usize].first_frame_after_reset =
            1 as libc::c_int;
    }
    delay_stack_alloc = ((*decControl).internalSampleRate * (*decControl).nChannelsInternal
        < (*decControl).API_sampleRate * (*decControl).nChannelsAPI)
        as libc::c_int;
    let vla = (if delay_stack_alloc != 0 {
        1 as libc::c_int
    } else {
        (*decControl).nChannelsInternal
            * ((*channel_state.offset(0 as libc::c_int as isize)).frame_length + 2 as libc::c_int)
    }) as usize;
    let mut samplesOut1_tmp_storage1: Vec<i16> = ::std::vec::from_elem(0, vla);
    if delay_stack_alloc != 0 {
        samplesOut1_tmp[0 as libc::c_int as usize] = samplesOut;
        samplesOut1_tmp[1 as libc::c_int as usize] = samplesOut
            .offset((*channel_state.offset(0 as libc::c_int as isize)).frame_length as isize)
            .offset(2 as libc::c_int as isize);
    } else {
        samplesOut1_tmp[0 as libc::c_int as usize] = samplesOut1_tmp_storage1.as_mut_ptr();
        samplesOut1_tmp[1 as libc::c_int as usize] = samplesOut1_tmp_storage1
            .as_mut_ptr()
            .offset((*channel_state.offset(0 as libc::c_int as isize)).frame_length as isize)
            .offset(2 as libc::c_int as isize);
    }
    if lostFlag == FLAG_DECODE_NORMAL {
        has_side = (decode_only_middle == 0) as libc::c_int;
    } else {
        has_side = ((*psDec).prev_decode_only_middle == 0
            || (*decControl).nChannelsInternal == 2 as libc::c_int
                && lostFlag == FLAG_DECODE_LBRR
                && (*channel_state.offset(1 as libc::c_int as isize)).LBRR_flags
                    [(*channel_state.offset(1 as libc::c_int as isize)).nFramesDecoded as usize]
                    == 1 as libc::c_int) as libc::c_int;
    }
    n = 0 as libc::c_int;
    while n < (*decControl).nChannelsInternal {
        if n == 0 as libc::c_int || has_side != 0 {
            let mut FrameIndex: libc::c_int = 0;
            let mut condCoding_0: libc::c_int = 0;
            FrameIndex = (*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded - n;
            if FrameIndex <= 0 as libc::c_int {
                condCoding_0 = CODE_INDEPENDENTLY;
            } else if lostFlag == FLAG_DECODE_LBRR {
                condCoding_0 = if (*channel_state.offset(n as isize)).LBRR_flags
                    [(FrameIndex - 1 as libc::c_int) as usize]
                    != 0
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
            ret += silk_decode_frame(
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
                    .offset(2 as libc::c_int as isize) as *mut i16
                    as *mut libc::c_void,
                0 as libc::c_int,
                (nSamplesOutDec as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
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
            MS_pred_Q13.as_mut_ptr() as *const i32,
            (*channel_state.offset(0 as libc::c_int as isize)).fs_kHz,
            nSamplesOutDec,
        );
    } else {
        memcpy(
            samplesOut1_tmp[0 as libc::c_int as usize] as *mut libc::c_void,
            ((*psDec).sStereo.sMid).as_mut_ptr() as *const libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
        );
        memcpy(
            ((*psDec).sStereo.sMid).as_mut_ptr() as *mut libc::c_void,
            &mut *(*samplesOut1_tmp
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
            .offset(nSamplesOutDec as isize) as *mut i16 as *const libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
        );
    }
    *nSamplesOut = nSamplesOutDec * (*decControl).API_sampleRate
        / ((*channel_state.offset(0 as libc::c_int as isize)).fs_kHz as i16 as i32
            * 1000 as libc::c_int as i16 as i32);
    let vla_0 = (if (*decControl).nChannelsAPI == 2 as libc::c_int {
        *nSamplesOut
    } else {
        1 as libc::c_int
    }) as usize;
    let mut samplesOut2_tmp: Vec<i16> = ::std::vec::from_elem(0, vla_0);
    if (*decControl).nChannelsAPI == 2 as libc::c_int {
        resample_out_ptr = samplesOut2_tmp.as_mut_ptr();
    } else {
        resample_out_ptr = samplesOut;
    }
    let vla_1 = (if delay_stack_alloc != 0 {
        (*decControl).nChannelsInternal
            * ((*channel_state.offset(0 as libc::c_int as isize)).frame_length + 2 as libc::c_int)
    } else {
        1 as libc::c_int
    }) as usize;
    let mut samplesOut1_tmp_storage2: Vec<i16> = ::std::vec::from_elem(0, vla_1);
    if delay_stack_alloc != 0 {
        memcpy(
            samplesOut1_tmp_storage2.as_mut_ptr() as *mut libc::c_void,
            samplesOut as *const libc::c_void,
            (((*decControl).nChannelsInternal
                * ((*channel_state.offset(0 as libc::c_int as isize)).frame_length
                    + 2 as libc::c_int)) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * samplesOut1_tmp_storage2
                            .as_mut_ptr()
                            .offset_from(samplesOut) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        samplesOut1_tmp[0 as libc::c_int as usize] = samplesOut1_tmp_storage2.as_mut_ptr();
        samplesOut1_tmp[1 as libc::c_int as usize] = samplesOut1_tmp_storage2
            .as_mut_ptr()
            .offset((*channel_state.offset(0 as libc::c_int as isize)).frame_length as isize)
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
        ret += silk_resampler(
            &mut (*channel_state.offset(n as isize)).resampler_state,
            resample_out_ptr,
            &mut *(*samplesOut1_tmp.as_mut_ptr().offset(n as isize))
                .offset(1 as libc::c_int as isize) as *mut i16 as *const i16,
            nSamplesOutDec,
        );
        if (*decControl).nChannelsAPI == 2 as libc::c_int {
            i = 0 as libc::c_int;
            while i < *nSamplesOut {
                *samplesOut.offset((n + 2 as libc::c_int * i) as isize) =
                    *resample_out_ptr.offset(i as isize);
                i += 1;
            }
        }
        n += 1;
    }
    if (*decControl).nChannelsAPI == 2 as libc::c_int
        && (*decControl).nChannelsInternal == 1 as libc::c_int
    {
        if stereo_to_mono != 0 {
            ret += silk_resampler(
                &mut (*channel_state.offset(1 as libc::c_int as isize)).resampler_state,
                resample_out_ptr,
                &mut *(*samplesOut1_tmp
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .offset(1 as libc::c_int as isize) as *mut i16 as *const i16,
                nSamplesOutDec,
            );
            i = 0 as libc::c_int;
            while i < *nSamplesOut {
                *samplesOut.offset((1 as libc::c_int + 2 as libc::c_int * i) as isize) =
                    *resample_out_ptr.offset(i as isize);
                i += 1;
            }
        } else {
            i = 0 as libc::c_int;
            while i < *nSamplesOut {
                *samplesOut.offset((1 as libc::c_int + 2 as libc::c_int * i) as isize) =
                    *samplesOut.offset((0 as libc::c_int + 2 as libc::c_int * i) as isize);
                i += 1;
            }
        }
    }
    if (*channel_state.offset(0 as libc::c_int as isize)).prevSignalType == TYPE_VOICED {
        let mult_tab: [libc::c_int; 3] = [6 as libc::c_int, 4 as libc::c_int, 3 as libc::c_int];
        (*decControl).prevPitchLag = (*channel_state.offset(0 as libc::c_int as isize)).lagPrev
            * mult_tab[((*channel_state.offset(0 as libc::c_int as isize)).fs_kHz
                - 8 as libc::c_int
                >> 2 as libc::c_int) as usize];
    } else {
        (*decControl).prevPitchLag = 0 as libc::c_int;
    }
    if lostFlag == FLAG_PACKET_LOST {
        i = 0 as libc::c_int;
        while i < (*psDec).nChannelsInternal {
            (*psDec).channel_state[i as usize].LastGainIndex = 10 as libc::c_int as i8;
            i += 1;
        }
    } else {
        (*psDec).prev_decode_only_middle = decode_only_middle;
    }
    return ret;
}
