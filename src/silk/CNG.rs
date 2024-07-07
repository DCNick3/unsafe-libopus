use crate::silk::define::{
    CNG_BUF_MASK_MAX, CNG_GAIN_SMTH_Q16, CNG_NLSF_SMTH_Q16, MAX_LPC_ORDER, TYPE_NO_VOICE_ACTIVITY,
};
use crate::silk::macros::{silk_SMLAWB, silk_SMULWB, silk_SMULWW};
use crate::silk::structs::{silk_CNG_struct, silk_decoder_control, silk_decoder_state};
use crate::silk::Inlines::silk_SQRT_APPROX;
use crate::silk::SigProc_FIX::{
    silk_LSHIFT_SAT32, silk_RAND, silk_RSHIFT_ROUND, silk_SAT16, silk_SMULTT,
};
use crate::silk::NLSF2A::silk_NLSF2A;

/// Generates excitation for CNG LPC synthesis
///
/// ```text
/// exc_Q14[]       O     CNG excitation signal Q10
/// exc_buf_Q14[]   I     Random samples buffer Q10
/// length          I     Length
/// rand_seed       I/O   Seed to random index generator
/// ```
#[inline]
fn silk_CNG_exc(exc_Q14: &mut [i32], exc_buf_Q14: &[i32], rand_seed: &mut i32) {
    let mut exc_mask = CNG_BUF_MASK_MAX;
    while exc_mask > exc_Q14.len() as i32 {
        exc_mask >>= 1;
    }

    let mut seed = *rand_seed;
    let mut i = 0;
    while i < exc_Q14.len() {
        seed = silk_RAND(seed);
        let idx = (seed >> 24) & exc_mask;
        debug_assert!(idx >= 0);
        debug_assert!(idx <= CNG_BUF_MASK_MAX);
        exc_Q14[i] = exc_buf_Q14[idx as usize];
        i += 1;
    }
    *rand_seed = seed;
}

pub fn silk_CNG_Reset(psDec: &mut silk_decoder_state) {
    let NLSF_step_Q15 = i16::MAX as i32 / (psDec.LPC_order as i32 + 1);
    let mut NLSF_acc_Q15 = 0;
    for i in 0..psDec.LPC_order {
        NLSF_acc_Q15 += NLSF_step_Q15;
        psDec.sCNG.CNG_smth_NLSF_Q15[i] = NLSF_acc_Q15 as i16;
    }
    psDec.sCNG.CNG_smth_Gain_Q16 = 0;
    psDec.sCNG.rand_seed = 3176576;
}

/// Updates CNG estimate, and applies the CNG when packet was lost
///
/// ```text
/// psDec         I/O   Decoder state
/// psDecCtrl     I/O   Decoder control
/// frame[]       I/O   Signal
/// length        I     Length of residual
/// ```
pub fn silk_CNG(
    psDec: &mut silk_decoder_state,
    psDecCtrl: &mut silk_decoder_control,
    frame: &mut [i16],
) {
    if psDec.fs_kHz != psDec.sCNG.fs_kHz {
        /* Reset state */
        silk_CNG_Reset(psDec);

        psDec.sCNG.fs_kHz = psDec.fs_kHz;
    }
    let psCNG: &mut silk_CNG_struct = &mut psDec.sCNG;
    if psDec.lossCnt == 0 && psDec.prevSignalType == TYPE_NO_VOICE_ACTIVITY {
        /* Update CNG parameters */

        /* Smoothing of LSF's  */
        for i in 0..psDec.LPC_order {
            psCNG.CNG_smth_NLSF_Q15[i] += silk_SMULWB(
                psDec.prevNLSF_Q15[i] as i32 - psCNG.CNG_smth_NLSF_Q15[i] as i32,
                CNG_NLSF_SMTH_Q16,
            ) as i16;
        }
        /* Find the subframe with the highest gain */
        let mut max_Gain_Q16 = 0;
        let mut subfr = 0;

        for i in 0..psDec.nb_subfr {
            if psDecCtrl.Gains_Q16[i] > max_Gain_Q16 {
                max_Gain_Q16 = psDecCtrl.Gains_Q16[i];
                subfr = i;
            }
        }
        /* Update CNG excitation buffer with excitation from this subframe */
        psCNG.CNG_exc_buf_Q14.copy_within(
            0..(psDec.nb_subfr - 1) * psDec.subfr_length,
            psDec.subfr_length,
        );
        psCNG.CNG_exc_buf_Q14[..psDec.subfr_length]
            .copy_from_slice(&psDec.exc_Q14[subfr * psDec.subfr_length..][..psDec.subfr_length]);

        /* Smooth gains */
        for i in 0..psDec.nb_subfr {
            psCNG.CNG_smth_Gain_Q16 += silk_SMULWB(
                psDecCtrl.Gains_Q16[i] - psCNG.CNG_smth_Gain_Q16,
                CNG_GAIN_SMTH_Q16,
            );
        }
    }

    /* Add CNG when packet is lost or during DTX */
    if psDec.lossCnt != 0 {
        let vla = frame.len() + MAX_LPC_ORDER;
        let mut CNG_sig_Q14: Vec<i32> = ::std::vec::from_elem(0, vla);

        /* Generate CNG excitation */
        let mut gain_Q16 = silk_SMULWW(psDec.sPLC.randScale_Q14 as i32, psDec.sPLC.prevGain_Q16[1]);
        if gain_Q16 >= (1 << 21) || psCNG.CNG_smth_Gain_Q16 > (1 << 23) {
            gain_Q16 = silk_SMULTT(gain_Q16, gain_Q16);
            gain_Q16 =
                silk_SMULTT(psCNG.CNG_smth_Gain_Q16, psCNG.CNG_smth_Gain_Q16) - (gain_Q16 << 5);
            gain_Q16 = silk_SQRT_APPROX(gain_Q16) << 16;
        } else {
            gain_Q16 = silk_SMULWW(gain_Q16, gain_Q16);
            gain_Q16 =
                silk_SMULWW(psCNG.CNG_smth_Gain_Q16, psCNG.CNG_smth_Gain_Q16) - (gain_Q16 << 5);
            gain_Q16 = silk_SQRT_APPROX(gain_Q16) << 8;
        }
        let gain_Q10 = gain_Q16 >> 6;
        silk_CNG_exc(
            &mut CNG_sig_Q14[MAX_LPC_ORDER..],
            &psCNG.CNG_exc_buf_Q14[..frame.len()],
            &mut psCNG.rand_seed,
        );

        let mut A_Q12: [i16; MAX_LPC_ORDER] = [0; 16];

        /* Convert CNG NLSF to filter representation */
        silk_NLSF2A(
            &mut A_Q12[..psDec.LPC_order],
            &psCNG.CNG_smth_NLSF_Q15[..psDec.LPC_order],
        );

        /* Generate CNG signal, by synthesis filtering */
        CNG_sig_Q14[..MAX_LPC_ORDER].copy_from_slice(&psCNG.CNG_synth_state);
        assert!(psDec.LPC_order == 10 || psDec.LPC_order == 16);
        for i in 0..frame.len() {
            /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
            let mut LPC_pred_Q10 = psDec.LPC_order as i32 >> 1;
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                CNG_sig_Q14[MAX_LPC_ORDER + i - 1],
                A_Q12[0] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                CNG_sig_Q14[MAX_LPC_ORDER + i - 2],
                A_Q12[1] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                CNG_sig_Q14[MAX_LPC_ORDER + i - 3],
                A_Q12[2] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                CNG_sig_Q14[MAX_LPC_ORDER + i - 4],
                A_Q12[3] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                CNG_sig_Q14[MAX_LPC_ORDER + i - 5],
                A_Q12[4] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                CNG_sig_Q14[MAX_LPC_ORDER + i - 6],
                A_Q12[5] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                CNG_sig_Q14[MAX_LPC_ORDER + i - 7],
                A_Q12[6] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                CNG_sig_Q14[MAX_LPC_ORDER + i - 8],
                A_Q12[7] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                CNG_sig_Q14[MAX_LPC_ORDER + i - 9],
                A_Q12[8] as i32,
            );
            LPC_pred_Q10 = silk_SMLAWB(
                LPC_pred_Q10,
                CNG_sig_Q14[MAX_LPC_ORDER + i - 10],
                A_Q12[9] as i32,
            );
            if psDec.LPC_order == 16 {
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    CNG_sig_Q14[MAX_LPC_ORDER + i - 11],
                    A_Q12[10] as i32,
                );
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    CNG_sig_Q14[MAX_LPC_ORDER + i - 12],
                    A_Q12[11] as i32,
                );
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    CNG_sig_Q14[MAX_LPC_ORDER + i - 13],
                    A_Q12[12] as i32,
                );
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    CNG_sig_Q14[MAX_LPC_ORDER + i - 14],
                    A_Q12[13] as i32,
                );
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    CNG_sig_Q14[MAX_LPC_ORDER + i - 15],
                    A_Q12[14] as i32,
                );
                LPC_pred_Q10 = silk_SMLAWB(
                    LPC_pred_Q10,
                    CNG_sig_Q14[MAX_LPC_ORDER + i - 16],
                    A_Q12[15] as i32,
                );
            }

            /* Update states */
            CNG_sig_Q14[MAX_LPC_ORDER + i] =
                CNG_sig_Q14[MAX_LPC_ORDER + i].saturating_add(silk_LSHIFT_SAT32(LPC_pred_Q10, 4));

            /* Scale with Gain and add to input signal */
            frame[i] = frame[i].saturating_add(silk_SAT16(silk_RSHIFT_ROUND(
                silk_SMULWW(CNG_sig_Q14[MAX_LPC_ORDER + i], gain_Q10),
                8,
            )) as i16);
        }
        psCNG
            .CNG_synth_state
            .copy_from_slice(&CNG_sig_Q14[frame.len()..][..MAX_LPC_ORDER]);
    } else {
        psCNG.CNG_synth_state[..psDec.LPC_order].fill(0);
    };
}
