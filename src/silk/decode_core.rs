use crate::silk::define::{
    LTP_ORDER, MAX_LPC_ORDER, MAX_NB_SUBFR, QUANT_LEVEL_ADJUST_Q10, TYPE_VOICED,
};
use crate::silk::macros::{silk_SMLAWB, silk_SMULWB, silk_SMULWW};
use crate::silk::structs::{silk_decoder_control, silk_decoder_state};
use crate::silk::tables_other::silk_Quantization_Offsets_Q10;
use crate::silk::Inlines::{silk_DIV32_varQ, silk_INVERSE32_varQ};
use crate::silk::LPC_analysis_filter::silk_LPC_analysis_filter;
use crate::silk::SigProc_FIX::{
    silk_LSHIFT_SAT32, silk_RAND, silk_RSHIFT_ROUND, silk_SAT16, SILK_FIX_CONST,
};

/// Core decoder. Performs inverse NSQ operation LTP + LPC
///
/// ```text
/// psDec                        I/O   Decoder state
/// psDecCtrl                    I     Decoder control
/// xq[]                         O     Decoded speech
/// pulses[ MAX_FRAME_LENGTH ]   I     Pulse signal
/// arch                         I     Run-time architecture
/// ```
pub fn silk_decode_core(
    psDec: &mut silk_decoder_state,
    psDecCtrl: &mut silk_decoder_control,
    xq: &mut [i16],
    pulses: &[i16],
) {
    let mut sLTP: Vec<i16> = ::std::vec::from_elem(0, psDec.ltp_mem_length);
    let mut sLTP_Q15: Vec<i32> =
        ::std::vec::from_elem(0, psDec.ltp_mem_length + psDec.frame_length);
    let mut res_Q14: Vec<i32> = ::std::vec::from_elem(0, psDec.subfr_length);
    let mut sLPC_Q14: Vec<i32> = ::std::vec::from_elem(0, psDec.subfr_length + MAX_LPC_ORDER);

    let offset_Q10 = silk_Quantization_Offsets_Q10[(psDec.indices.signalType as i32 >> 1) as usize]
        [psDec.indices.quantOffsetType as usize] as i32;

    let NLSF_interpolation_flag = if (psDec.indices.NLSFInterpCoef_Q2 as i32) < (1) << 2 {
        1
    } else {
        0
    };

    /* Decode excitation */
    let mut rand_seed = psDec.indices.Seed as i32;
    let mut i = 0;
    while i < psDec.frame_length {
        rand_seed = silk_RAND(rand_seed);
        psDec.exc_Q14[i] = (pulses[i] as i32) << 14;
        if psDec.exc_Q14[i] > 0 {
            psDec.exc_Q14[i] -= QUANT_LEVEL_ADJUST_Q10 << 4;
        } else if psDec.exc_Q14[i] < 0 {
            psDec.exc_Q14[i] += QUANT_LEVEL_ADJUST_Q10 << 4;
        }
        psDec.exc_Q14[i] += offset_Q10 << 4;
        if rand_seed < 0 {
            psDec.exc_Q14[i] = -psDec.exc_Q14[i];
        }
        rand_seed = rand_seed.wrapping_add(pulses[i] as i32);
        i += 1;
    }

    /* Copy LPC state */
    sLPC_Q14[..MAX_LPC_ORDER].copy_from_slice(&psDec.sLPC_Q14_buf);

    let mut pexc_Q14 = psDec.exc_Q14.as_mut_slice();
    // let mut pxq = xq;
    let mut sLTP_buf_idx = psDec.ltp_mem_length;
    /* Loop over subframes */
    let mut k = 0;
    while k < psDec.nb_subfr {
        let mut pres_Q14 = res_Q14.as_mut_slice();
        let A_Q12 = &psDecCtrl.PredCoef_Q12[(k >> 1)][..psDec.LPC_order];

        let mut A_Q12_tmp: [i16; MAX_LPC_ORDER] = [0; 16];
        let A_Q12_tmp = &mut A_Q12_tmp[..psDec.LPC_order];

        /* Preload LPC coeficients to array on stack. Gives small performance gain */
        A_Q12_tmp.copy_from_slice(A_Q12);
        let B_Q14 = &mut psDecCtrl.LTPCoef_Q14[k * LTP_ORDER..];
        let mut signalType = psDec.indices.signalType as i32;

        let Gain_Q10 = psDecCtrl.Gains_Q16[k] >> 6;
        let mut inv_gain_Q31 = silk_INVERSE32_varQ(psDecCtrl.Gains_Q16[k], 47);

        /* Calculate gain adjustment factor */
        let gain_adj_Q16 = if psDecCtrl.Gains_Q16[k] != psDec.prev_gain_Q16 {
            let gain_adj_Q16 = silk_DIV32_varQ(psDec.prev_gain_Q16, psDecCtrl.Gains_Q16[k], 16);

            /* Scale short term state */
            for i in 0..MAX_LPC_ORDER {
                sLPC_Q14[i] = silk_SMULWW(gain_adj_Q16, sLPC_Q14[i]);
            }

            gain_adj_Q16
        } else {
            1 << 16
        };

        /* Save inv_gain */
        debug_assert!(inv_gain_Q31 != 0);
        psDec.prev_gain_Q16 = psDecCtrl.Gains_Q16[k];

        /* Avoid abrupt transition from voiced PLC to unvoiced normal decoding */
        if psDec.lossCnt != 0
            && psDec.prevSignalType == TYPE_VOICED
            && psDec.indices.signalType as i32 != TYPE_VOICED
            && k < MAX_NB_SUBFR / 2
        {
            B_Q14[..LTP_ORDER].fill(0);
            B_Q14[LTP_ORDER / 2] = SILK_FIX_CONST!(0.25, 14) as i16;

            signalType = TYPE_VOICED;
            psDecCtrl.pitchL[k] = psDec.lagPrev;
        }

        let mut lag = 0;
        if signalType == TYPE_VOICED {
            /* Voiced */
            lag = psDecCtrl.pitchL[k] as usize;

            /* Re-whitening */
            if k == 0 || k == 2 && NLSF_interpolation_flag != 0 {
                /* Rewhiten with new A coefs */
                let start_idx = psDec.ltp_mem_length - lag - psDec.LPC_order - LTP_ORDER / 2;
                assert!(start_idx > 0);

                if k == 2 {
                    psDec.outBuf[psDec.ltp_mem_length..][..2 * psDec.subfr_length]
                        .copy_from_slice(&xq[..2 * psDec.subfr_length]);
                }

                silk_LPC_analysis_filter(
                    &mut sLTP[start_idx..psDec.ltp_mem_length],
                    &psDec.outBuf[start_idx + k * psDec.subfr_length..]
                        [..(psDec.ltp_mem_length - start_idx)],
                    A_Q12,
                );

                /* After rewhitening the LTP state is unscaled */
                if k == 0 {
                    /* Do LTP downscaling to reduce inter-packet dependency */
                    inv_gain_Q31 = silk_SMULWB(inv_gain_Q31, psDecCtrl.LTP_scale_Q14) << 2;
                }
                let mut i = 0;
                while i < lag + LTP_ORDER / 2 {
                    sLTP_Q15[sLTP_buf_idx - i - 1] =
                        silk_SMULWB(inv_gain_Q31, sLTP[psDec.ltp_mem_length - i - 1] as i32);
                    i += 1;
                }
                /* Update LTP state when Gain changes */
            } else if gain_adj_Q16 != (1) << 16 {
                let mut i = 0;
                while i < lag + LTP_ORDER / 2 {
                    sLTP_Q15[sLTP_buf_idx - i - 1] =
                        silk_SMULWW(gain_adj_Q16, sLTP_Q15[sLTP_buf_idx - i - 1]);
                    i += 1;
                }
            }
        }

        /* Long-term prediction */
        if signalType == TYPE_VOICED {
            /* Set up pointer */
            let mut pred_lag_ptr = sLTP_buf_idx - lag + LTP_ORDER / 2;
            let mut i = 0;
            while i < psDec.subfr_length {
                /* Unrolled loop */
                /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
                let mut LTP_pred_Q13 = 2;
                LTP_pred_Q13 =
                    silk_SMLAWB(LTP_pred_Q13, sLTP_Q15[pred_lag_ptr + 0], B_Q14[0] as i32);
                LTP_pred_Q13 =
                    silk_SMLAWB(LTP_pred_Q13, sLTP_Q15[pred_lag_ptr - 1], B_Q14[1] as i32);
                LTP_pred_Q13 =
                    silk_SMLAWB(LTP_pred_Q13, sLTP_Q15[pred_lag_ptr - 2], B_Q14[2] as i32);
                LTP_pred_Q13 =
                    silk_SMLAWB(LTP_pred_Q13, sLTP_Q15[pred_lag_ptr - 3], B_Q14[3] as i32);
                LTP_pred_Q13 =
                    silk_SMLAWB(LTP_pred_Q13, sLTP_Q15[pred_lag_ptr - 4], B_Q14[4] as i32);
                pred_lag_ptr += 1;

                /* Generate LPC excitation */
                pres_Q14[i] = pexc_Q14[i] + (LTP_pred_Q13 << 1);

                /* Update states */
                sLTP_Q15[sLTP_buf_idx] = pres_Q14[i] << 1;
                sLTP_buf_idx += 1;
                i += 1;
            }
        } else {
            pres_Q14 = pexc_Q14;
        }

        let mut i = 0;
        while i < psDec.subfr_length {
            /* Short-term prediction */
            assert!(psDec.LPC_order == 10 || psDec.LPC_order == 16);
            /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
            let mut LPC_pred_Q10 = psDec.LPC_order as i32 / 2;
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                sLPC_Q14[MAX_LPC_ORDER + i - 1],
                A_Q12_tmp[0] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                sLPC_Q14[MAX_LPC_ORDER + i - 2],
                A_Q12_tmp[1] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                sLPC_Q14[MAX_LPC_ORDER + i - 3],
                A_Q12_tmp[2] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                sLPC_Q14[MAX_LPC_ORDER + i - 4],
                A_Q12_tmp[3] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                sLPC_Q14[MAX_LPC_ORDER + i - 5],
                A_Q12_tmp[4] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                sLPC_Q14[MAX_LPC_ORDER + i - 6],
                A_Q12_tmp[5] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                sLPC_Q14[MAX_LPC_ORDER + i - 7],
                A_Q12_tmp[6] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                sLPC_Q14[MAX_LPC_ORDER + i - 8],
                A_Q12_tmp[7] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                sLPC_Q14[MAX_LPC_ORDER + i - 9],
                A_Q12_tmp[8] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                sLPC_Q14[MAX_LPC_ORDER + i - 10],
                A_Q12_tmp[9] as i32,
            );
            if psDec.LPC_order == 16 {
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    sLPC_Q14[MAX_LPC_ORDER + i - 11],
                    A_Q12_tmp[10] as i32,
                );
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    sLPC_Q14[MAX_LPC_ORDER + i - 12],
                    A_Q12_tmp[11] as i32,
                );
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    sLPC_Q14[MAX_LPC_ORDER + i - 13],
                    A_Q12_tmp[12] as i32,
                );
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    sLPC_Q14[MAX_LPC_ORDER + i - 14],
                    A_Q12_tmp[13] as i32,
                );
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    sLPC_Q14[MAX_LPC_ORDER + i - 15],
                    A_Q12_tmp[14] as i32,
                );
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    sLPC_Q14[MAX_LPC_ORDER + i - 16],
                    A_Q12_tmp[15] as i32,
                );
            }

            /* Add prediction to LPC excitation */
            sLPC_Q14[MAX_LPC_ORDER + i] =
                pres_Q14[i].saturating_add(silk_LSHIFT_SAT32(LPC_pred_Q10, 4));

            /* Scale with gain */

            xq[k * psDec.subfr_length + i] = silk_SAT16(silk_RSHIFT_ROUND(
                silk_SMULWW(sLPC_Q14[MAX_LPC_ORDER + i], Gain_Q10),
                8,
            )) as i16;

            i += 1;
        }

        /* Update LPC filter state */
        sLPC_Q14.copy_within(psDec.subfr_length.., 0);
        pexc_Q14 = &mut pexc_Q14[psDec.subfr_length..];
        // pxq = &mut pxq[psDec.subfr_length..];
        k += 1;
    }

    /* Save LPC state */
    psDec
        .sLPC_Q14_buf
        .copy_from_slice(&sLPC_Q14[..MAX_LPC_ORDER]);
}
