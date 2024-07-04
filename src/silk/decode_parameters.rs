use crate::silk::bwexpander::silk_bwexpander;
use crate::silk::decode_pitch::silk_decode_pitch;
use crate::silk::define::{BWE_AFTER_LOSS_Q16, CODE_CONDITIONALLY, LTP_ORDER, TYPE_VOICED};
use crate::silk::gain_quant::silk_gains_dequant;
use crate::silk::structs::{silk_decoder_control, silk_decoder_state};
use crate::silk::tables_LTP::silk_LTP_vq_ptrs_Q7;
use crate::silk::tables_other::silk_LTPScales_table_Q14;
use crate::silk::NLSF_decode::silk_NLSF_decode;
use crate::silk::NLSF2A::silk_NLSF2A;

/// Decode parameters from payload
///
/// ```text
/// psDec        I/O   State
/// psDecCtrl    I/O   Decoder control
/// condCoding   I     The type of conditional coding to use
/// ```
pub fn silk_decode_parameters(
    psDec: &mut silk_decoder_state,
    psDecCtrl: &mut silk_decoder_control,
    condCoding: i32,
) {
    let [PredCoef_Q12_0, PredCoef_Q12_1] = &mut psDecCtrl.PredCoef_Q12;
    let PredCoef_Q12_0 = &mut PredCoef_Q12_0[..psDec.LPC_order as usize];
    let PredCoef_Q12_1 = &mut PredCoef_Q12_1[..psDec.LPC_order as usize];

    let Gains_Q16 = &mut psDecCtrl.Gains_Q16[..psDec.nb_subfr as usize];
    let GainsIndices = &psDec.indices.GainsIndices[..psDec.nb_subfr as usize];

    let NLSFIndices = &psDec.indices.NLSFIndices[..psDec.psNLSF_CB.order as usize + 1];

    let prevNLSF_Q15 = &mut psDec.prevNLSF_Q15[..psDec.LPC_order as usize];

    let pitchL = &mut psDecCtrl.pitchL[..psDec.nb_subfr as usize];

    let LTPCoef_Q14 = &mut psDecCtrl.LTPCoef_Q14[..psDec.nb_subfr as usize * LTP_ORDER as usize];

    /* Dequant Gains */
    silk_gains_dequant(
        Gains_Q16,
        GainsIndices,
        &mut psDec.LastGainIndex,
        condCoding == CODE_CONDITIONALLY,
    );

    /****************/
    /* Decode NLSFs */
    /****************/
    let mut pNLSF_Q15: [i16; 16] = [0; 16];
    let pNLSF_Q15 = &mut pNLSF_Q15[..psDec.LPC_order as usize];
    silk_NLSF_decode(pNLSF_Q15, NLSFIndices, psDec.psNLSF_CB);

    /* Convert NLSF parameters to AR prediction filter coefficients */
    silk_NLSF2A(PredCoef_Q12_1, pNLSF_Q15);

    /* If just reset, e.g., because internal Fs changed, do not allow interpolation */
    /* improves the case of packet loss in the first frame after a switch           */
    if psDec.first_frame_after_reset == 1 {
        psDec.indices.NLSFInterpCoef_Q2 = 4;
    }
    if (psDec.indices.NLSFInterpCoef_Q2 as i32) < 4 {
        /* Calculation of the interpolated NLSF0 vector from the interpolation factor, */
        /* the previous NLSF1, and the current NLSF1                                   */
        let mut pNLSF0_Q15: [i16; 16] = [0; 16];
        let pNLSF0_Q15 = &mut pNLSF0_Q15[..psDec.LPC_order as usize];

        for i in 0..psDec.LPC_order as usize {
            pNLSF0_Q15[i] = (prevNLSF_Q15[i] as i32
                + ((psDec.indices.NLSFInterpCoef_Q2 as i32
                    * (pNLSF_Q15[i] as i32 - prevNLSF_Q15[i] as i32))
                    >> 2)) as i16;
        }

        /* Convert NLSF parameters to AR prediction filter coefficients */
        silk_NLSF2A(PredCoef_Q12_0, pNLSF0_Q15);
    } else {
        /* Copy LPC coefficients for first half from second half */
        PredCoef_Q12_0.copy_from_slice(PredCoef_Q12_1);
    }

    prevNLSF_Q15[..psDec.LPC_order as usize]
        .copy_from_slice(&pNLSF_Q15[..psDec.LPC_order as usize]);

    /* After a packet loss do BWE of LPC coefs */
    if psDec.lossCnt != 0 {
        silk_bwexpander(PredCoef_Q12_0, BWE_AFTER_LOSS_Q16);
        silk_bwexpander(PredCoef_Q12_1, BWE_AFTER_LOSS_Q16);
    }

    if psDec.indices.signalType as i32 == TYPE_VOICED {
        /*********************/
        /* Decode pitch lags */
        /*********************/

        /* Decode pitch values */
        silk_decode_pitch(
            psDec.indices.lagIndex,
            psDec.indices.contourIndex,
            pitchL,
            psDec.fs_kHz,
        );

        /* Decode Codebook Index */
        let cbk_ptr_Q7 = silk_LTP_vq_ptrs_Q7[psDec.indices.PERIndex as usize];

        for k in 0..psDec.nb_subfr as usize {
            let Ix = psDec.indices.LTPIndex[k] as usize;
            for i in 0..LTP_ORDER as usize {
                // ugh, I tried making it into a 2D array, but stuff broke
                // no idea why
                // LTPCoef_Q14[k * LTP_ORDER as usize + i] = (cbk_ptr_Q7[Ix][i] as i16) << 7;
                LTPCoef_Q14[k * LTP_ORDER as usize + i] = (cbk_ptr_Q7[Ix][i] as i16) << 7;
            }
        }

        /**********************/
        /* Decode LTP scaling */
        /**********************/
        let Ix = psDec.indices.LTP_scaleIndex as usize;
        psDecCtrl.LTP_scale_Q14 = silk_LTPScales_table_Q14[Ix] as i32;
    } else {
        pitchL.fill(0);
        LTPCoef_Q14.fill(0);

        psDec.indices.PERIndex = 0;
        psDecCtrl.LTP_scale_Q14 = 0;
    };
}
