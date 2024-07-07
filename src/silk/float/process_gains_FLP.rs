use crate::celt::mathops::celt_sqrt;
use crate::externs::memcpy;
use crate::silk::define::{CODE_CONDITIONALLY, TYPE_VOICED};
use crate::silk::float::structs_FLP::{
    silk_encoder_control_FLP, silk_encoder_state_FLP, silk_shape_state_FLP,
};
use crate::silk::float::SigProc_FLP::silk_sigmoid;
use crate::silk::gain_quant::silk_gains_quant;
use crate::silk::mathops::silk_exp2;
use crate::silk::tables_other::silk_Quantization_Offsets_Q10;
use crate::silk::tuning_parameters::{
    LAMBDA_CODING_QUALITY, LAMBDA_DELAYED_DECISIONS, LAMBDA_INPUT_QUALITY, LAMBDA_OFFSET,
    LAMBDA_QUANT_OFFSET, LAMBDA_SPEECH_ACT,
};

pub unsafe fn silk_process_gains_FLP(
    psEnc: *mut silk_encoder_state_FLP,
    psEncCtrl: *mut silk_encoder_control_FLP,
    condCoding: i32,
) {
    let psShapeSt: *mut silk_shape_state_FLP = &mut (*psEnc).sShape;
    let mut k: i32 = 0;
    let mut pGains_Q16: [i32; 4] = [0; 4];
    let mut s: f32 = 0.;
    let mut InvMaxSqrVal: f32 = 0.;
    let mut gain: f32 = 0.;
    let mut quant_offset: f32 = 0.;
    if (*psEnc).sCmn.indices.signalType as i32 == TYPE_VOICED {
        s = 1.0f32 - 0.5f32 * silk_sigmoid(0.25f32 * ((*psEncCtrl).LTPredCodGain - 12.0f32));
        k = 0;
        while k < (*psEnc).sCmn.nb_subfr as i32 {
            (*psEncCtrl).Gains[k as usize] *= s;
            k += 1;
        }
    }
    InvMaxSqrVal = silk_exp2(0.33f32 * (21.0f32 - (*psEnc).sCmn.SNR_dB_Q7 as f32 * (1.0 / 128.0)))
        / (*psEnc).sCmn.subfr_length as f32;
    k = 0;
    while k < (*psEnc).sCmn.nb_subfr as i32 {
        gain = (*psEncCtrl).Gains[k as usize];
        gain = celt_sqrt(gain * gain + (*psEncCtrl).ResNrg[k as usize] * InvMaxSqrVal);
        (*psEncCtrl).Gains[k as usize] = if gain < 32767.0f32 { gain } else { 32767.0f32 };
        k += 1;
    }
    k = 0;
    while k < (*psEnc).sCmn.nb_subfr as i32 {
        pGains_Q16[k as usize] = ((*psEncCtrl).Gains[k as usize] * 65536.0f32) as i32;
        k += 1;
    }
    memcpy(
        ((*psEncCtrl).GainsUnq_Q16).as_mut_ptr() as *mut core::ffi::c_void,
        pGains_Q16.as_mut_ptr() as *const core::ffi::c_void,
        ((*psEnc).sCmn.nb_subfr as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    (*psEncCtrl).lastGainIndexPrev = (*psShapeSt).LastGainIndex;
    silk_gains_quant(
        &mut (*psEnc).sCmn.indices.GainsIndices[..(*psEnc).sCmn.nb_subfr as usize],
        &mut pGains_Q16[..(*psEnc).sCmn.nb_subfr as usize],
        &mut (*psShapeSt).LastGainIndex,
        condCoding == CODE_CONDITIONALLY,
    );
    k = 0;
    while k < (*psEnc).sCmn.nb_subfr as i32 {
        (*psEncCtrl).Gains[k as usize] = pGains_Q16[k as usize] as f32 / 65536.0f32;
        k += 1;
    }
    if (*psEnc).sCmn.indices.signalType as i32 == TYPE_VOICED {
        if (*psEncCtrl).LTPredCodGain + (*psEnc).sCmn.input_tilt_Q15 as f32 * (1.0f32 / 32768.0f32)
            > 1.0f32
        {
            (*psEnc).sCmn.indices.quantOffsetType = 0;
        } else {
            (*psEnc).sCmn.indices.quantOffsetType = 1;
        }
    }
    quant_offset = silk_Quantization_Offsets_Q10
        [((*psEnc).sCmn.indices.signalType as i32 >> 1) as usize]
        [(*psEnc).sCmn.indices.quantOffsetType as usize] as i32 as f32
        / 1024.0f32;
    (*psEncCtrl).Lambda = LAMBDA_OFFSET
        + LAMBDA_DELAYED_DECISIONS * (*psEnc).sCmn.nStatesDelayedDecision as f32
        + LAMBDA_SPEECH_ACT * (*psEnc).sCmn.speech_activity_Q8 as f32 * (1.0f32 / 256.0f32)
        + LAMBDA_INPUT_QUALITY * (*psEncCtrl).input_quality
        + LAMBDA_CODING_QUALITY * (*psEncCtrl).coding_quality
        + LAMBDA_QUANT_OFFSET * quant_offset;
}
