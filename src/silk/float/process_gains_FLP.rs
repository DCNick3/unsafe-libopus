use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/structs_FLP.h:32"]
pub mod structs_FLP_h {
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
    #[c2rust::src_loc = "52:9"]
    pub struct silk_encoder_state_FLP {
        pub sCmn: silk_encoder_state,
        pub sShape: silk_shape_state_FLP,
        pub x_buf: [libc::c_float; 720],
        pub LTPCorr: libc::c_float,
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
    use crate::silk::structs::silk_encoder_state;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:32"]
pub mod SigProc_FLP_h {
    #[inline]
    #[c2rust::src_loc = "150:1"]
    pub unsafe extern "C" fn silk_sigmoid(x: libc::c_float) -> libc::c_float {
        return (1.0f64 / (1.0f64 + (-x as f64).exp())) as libc::c_float;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "101:26"]
        pub static silk_Quantization_Offsets_Q10: [[i16; 2]; 2];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    extern "C" {
        #[c2rust::src_loc = "178:1"]
        pub fn silk_gains_quant(
            ind: *mut i8,
            gain_Q16: *mut i32,
            prev_ind: *mut i8,
            conditional: libc::c_int,
            nb_subfr: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tuning_parameters.h:33"]
pub mod tuning_parameters_h {
    #[c2rust::src_loc = "138:9"]
    pub const LAMBDA_OFFSET: libc::c_float = 1.2f32;
    #[c2rust::src_loc = "139:9"]
    pub const LAMBDA_SPEECH_ACT: libc::c_float = -0.2f32;
    #[c2rust::src_loc = "140:9"]
    pub const LAMBDA_DELAYED_DECISIONS: libc::c_float = -0.05f32;
    #[c2rust::src_loc = "141:9"]
    pub const LAMBDA_INPUT_QUALITY: libc::c_float = -0.1f32;
    #[c2rust::src_loc = "142:9"]
    pub const LAMBDA_CODING_QUALITY: libc::c_float = -0.2f32;
    #[c2rust::src_loc = "143:9"]
    pub const LAMBDA_QUANT_OFFSET: libc::c_float = 0.8f32;
}
use self::main_h::silk_gains_quant;
use crate::silk::define::{CODE_CONDITIONALLY, TYPE_VOICED};

pub use self::structs_FLP_h::{
    silk_encoder_control_FLP, silk_encoder_state_FLP, silk_shape_state_FLP,
};
use self::tables_h::silk_Quantization_Offsets_Q10;
pub use self::tuning_parameters_h::{
    LAMBDA_CODING_QUALITY, LAMBDA_DELAYED_DECISIONS, LAMBDA_INPUT_QUALITY, LAMBDA_OFFSET,
    LAMBDA_QUANT_OFFSET, LAMBDA_SPEECH_ACT,
};

pub use self::SigProc_FLP_h::silk_sigmoid;
use crate::externs::memcpy;
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_process_gains_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut psEncCtrl: *mut silk_encoder_control_FLP,
    condCoding: libc::c_int,
) {
    let psShapeSt: *mut silk_shape_state_FLP = &mut (*psEnc).sShape;
    let mut k: libc::c_int = 0;
    let mut pGains_Q16: [i32; 4] = [0; 4];
    let mut s: libc::c_float = 0.;
    let mut InvMaxSqrVal: libc::c_float = 0.;
    let mut gain: libc::c_float = 0.;
    let mut quant_offset: libc::c_float = 0.;
    if (*psEnc).sCmn.indices.signalType as libc::c_int == TYPE_VOICED {
        s = 1.0f32 - 0.5f32 * silk_sigmoid(0.25f32 * ((*psEncCtrl).LTPredCodGain - 12.0f32));
        k = 0 as libc::c_int;
        while k < (*psEnc).sCmn.nb_subfr {
            (*psEncCtrl).Gains[k as usize] *= s;
            k += 1;
        }
    }
    InvMaxSqrVal = 2.0f32
        .powf(0.33f32 * (21.0f32 - (*psEnc).sCmn.SNR_dB_Q7 as f32 * (1.0 / 128.0)))
        / (*psEnc).sCmn.subfr_length as f32;
    k = 0 as libc::c_int;
    while k < (*psEnc).sCmn.nb_subfr {
        gain = (*psEncCtrl).Gains[k as usize];
        gain = (gain * gain + (*psEncCtrl).ResNrg[k as usize] * InvMaxSqrVal).sqrt();
        (*psEncCtrl).Gains[k as usize] = if gain < 32767.0f32 { gain } else { 32767.0f32 };
        k += 1;
    }
    k = 0 as libc::c_int;
    while k < (*psEnc).sCmn.nb_subfr {
        pGains_Q16[k as usize] = ((*psEncCtrl).Gains[k as usize] * 65536.0f32) as i32;
        k += 1;
    }
    memcpy(
        ((*psEncCtrl).GainsUnq_Q16).as_mut_ptr() as *mut libc::c_void,
        pGains_Q16.as_mut_ptr() as *const libc::c_void,
        ((*psEnc).sCmn.nb_subfr as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
    (*psEncCtrl).lastGainIndexPrev = (*psShapeSt).LastGainIndex;
    silk_gains_quant(
        ((*psEnc).sCmn.indices.GainsIndices).as_mut_ptr(),
        pGains_Q16.as_mut_ptr(),
        &mut (*psShapeSt).LastGainIndex,
        (condCoding == CODE_CONDITIONALLY) as libc::c_int,
        (*psEnc).sCmn.nb_subfr,
    );
    k = 0 as libc::c_int;
    while k < (*psEnc).sCmn.nb_subfr {
        (*psEncCtrl).Gains[k as usize] = pGains_Q16[k as usize] as libc::c_float / 65536.0f32;
        k += 1;
    }
    if (*psEnc).sCmn.indices.signalType as libc::c_int == TYPE_VOICED {
        if (*psEncCtrl).LTPredCodGain
            + (*psEnc).sCmn.input_tilt_Q15 as libc::c_float * (1.0f32 / 32768.0f32)
            > 1.0f32
        {
            (*psEnc).sCmn.indices.quantOffsetType = 0 as libc::c_int as i8;
        } else {
            (*psEnc).sCmn.indices.quantOffsetType = 1 as libc::c_int as i8;
        }
    }
    quant_offset = silk_Quantization_Offsets_Q10
        [((*psEnc).sCmn.indices.signalType as libc::c_int >> 1 as libc::c_int) as usize]
        [(*psEnc).sCmn.indices.quantOffsetType as usize] as libc::c_int
        as libc::c_float
        / 1024.0f32;
    (*psEncCtrl).Lambda = LAMBDA_OFFSET
        + LAMBDA_DELAYED_DECISIONS * (*psEnc).sCmn.nStatesDelayedDecision as libc::c_float
        + LAMBDA_SPEECH_ACT
            * (*psEnc).sCmn.speech_activity_Q8 as libc::c_float
            * (1.0f32 / 256.0f32)
        + LAMBDA_INPUT_QUALITY * (*psEncCtrl).input_quality
        + LAMBDA_CODING_QUALITY * (*psEncCtrl).coding_quality
        + LAMBDA_QUANT_OFFSET * quant_offset;
}
