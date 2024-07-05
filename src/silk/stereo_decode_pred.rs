use crate::celt::entdec::{ec_dec, ec_dec_icdf};
use crate::silk::define::STEREO_QUANT_SUB_STEPS;
use crate::silk::macros::silk_SMULWB;
use crate::silk::tables_other::{
    silk_stereo_only_code_mid_iCDF, silk_stereo_pred_joint_iCDF, silk_stereo_pred_quant_Q13,
    silk_uniform3_iCDF, silk_uniform5_iCDF,
};
use crate::silk::SigProc_FIX::SILK_FIX_CONST;

/// Decode mid/side predictors
///
/// ```text
/// psRangeDec    I/O   Compressor data structure
/// pred_Q13[]    O     Predictors
/// ```
pub fn silk_stereo_decode_pred(psRangeDec: &mut ec_dec, pred_Q13: &mut [i32; 2]) {
    let n = ec_dec_icdf(psRangeDec, &silk_stereo_pred_joint_iCDF, 8) as usize;

    let mut ix: [[usize; 3]; 2] = [[0; 3]; 2];
    ix[0][2] = n / 5;
    ix[1][2] = n - 5 * ix[0][2];

    /* Entropy decoding */
    let mut n = 0;
    while n < 2 {
        ix[n][0] = ec_dec_icdf(psRangeDec, &silk_uniform3_iCDF, 8) as usize;
        ix[n][1] = ec_dec_icdf(psRangeDec, &silk_uniform5_iCDF, 8) as usize;
        n += 1;
    }

    /* Dequantize */
    let mut n = 0;
    while n < 2 {
        ix[n][0] += 3 * ix[n][2];
        let low_Q13 = silk_stereo_pred_quant_Q13[ix[n][0]] as i32;
        let step_Q13 = silk_SMULWB(
            silk_stereo_pred_quant_Q13[ix[n][0] + 1] as i32 - low_Q13,
            SILK_FIX_CONST!(0.5 / STEREO_QUANT_SUB_STEPS as f64, 16),
        );

        pred_Q13[n] = low_Q13 + step_Q13 as i16 as i32 * (2 * ix[n][1] + 1) as i16 as i32;
        n += 1;
    }

    /* Subtract second from first predictor (helps when actually applying these) */
    pred_Q13[0] -= pred_Q13[1];
}

/// Decode mid-only flag
///
/// ```text
/// psRangeDec        I/O   Compressor data structure
/// decode_only_mid   O     Flag that only mid channel has been coded
/// ```
pub fn silk_stereo_decode_mid_only(psRangeDec: &mut ec_dec, decode_only_mid: &mut bool) {
    /* Decode flag that only mid channel is coded */
    *decode_only_mid = ec_dec_icdf(psRangeDec, &silk_stereo_only_code_mid_iCDF, 8) != 0;
}
