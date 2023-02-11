use crate::silk::structs::{silk_encoder_state, stereo_enc_state};

#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "95:9"]
pub struct silk_encoder {
    pub state_Fxx: [silk_encoder_state_FLP; 2],
    pub sStereo: stereo_enc_state,
    pub nBitsUsedLBRR: i32,
    pub nBitsExceeded: i32,
    pub nChannelsAPI: libc::c_int,
    pub nChannelsInternal: libc::c_int,
    pub nPrevChannelsInternal: libc::c_int,
    pub timeSinceSwitchAllowed_ms: libc::c_int,
    pub allowBandwidthSwitch: libc::c_int,
    pub prev_decode_only_middle: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "52:9"]
pub struct silk_encoder_state_FLP {
    pub sCmn: silk_encoder_state,
    pub sShape: silk_shape_state_FLP,
    pub x_buf: [libc::c_float; 720],
    pub LTPCorr: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "43:9"]
pub struct silk_shape_state_FLP {
    pub LastGainIndex: i8,
    pub HarmShapeGain_smth: libc::c_float,
    pub Tilt_smth: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "64:9"]
pub struct silk_encoder_control_FLP {
    pub Gains: [libc::c_float; 4],
    pub PredCoef: [[libc::c_float; 16]; 2],
    pub LTPCoef: [libc::c_float; 20],
    pub LTP_scale: libc::c_float,
    pub pitchL: [libc::c_int; 4],
    pub AR: [libc::c_float; 96],
    pub LF_MA_shp: [libc::c_float; 4],
    pub LF_AR_shp: [libc::c_float; 4],
    pub Tilt: [libc::c_float; 4],
    pub HarmShapeGain: [libc::c_float; 4],
    pub Lambda: libc::c_float,
    pub input_quality: libc::c_float,
    pub coding_quality: libc::c_float,
    pub predGain: libc::c_float,
    pub LTPredCodGain: libc::c_float,
    pub ResNrg: [libc::c_float; 4],
    pub GainsUnq_Q16: [i32; 4],
    pub lastGainIndexPrev: i8,
}
