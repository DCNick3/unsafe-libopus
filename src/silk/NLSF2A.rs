use crate::silk::bwexpander_32::silk_bwexpander_32;
use crate::silk::define::{LSF_COS_TAB_SZ_FIX, MAX_LPC_STABILIZE_ITERATIONS};
use crate::silk::table_LSF_cos::silk_LSFCosTab_FIX_Q12;
use crate::silk::LPC_fit::silk_LPC_fit;
use crate::silk::LPC_inv_pred_gain::silk_LPC_inverse_pred_gain_c;
use crate::silk::SigProc_FIX::{silk_RSHIFT_ROUND, silk_RSHIFT_ROUND64, SILK_MAX_ORDER_LPC};
use ndarray::azip;

pub const QA: i32 = 16;

/// helper function for NLSF2A(..)
///
/// ```text
/// out     O   intermediate polynomial, QA [dd+1]
/// cLSF    I   vector of interleaved 2*cos(LSFs), QA [d]
/// dd      I   polynomial order (= 1/2 * filter order)
/// ```
#[inline]
fn silk_NLSF2A_find_poly(out: &mut [i32], cLSF: &[i32]) {
    let d = cLSF.len();
    let dd = d / 2;
    assert_eq!(out.len(), dd + 1);

    out[0] = 1 << QA;
    out[1] = -cLSF[0];

    for k in 1..dd {
        let ftmp = cLSF[2 * k]; /* QA */
        out[k + 1] = out[k - 1] * 2 - silk_RSHIFT_ROUND64(ftmp as i64 * out[k] as i64, QA) as i32;

        for n in (2..=k).rev() {
            out[n] += out[n - 2] - silk_RSHIFT_ROUND64(ftmp as i64 * out[n - 1] as i64, QA) as i32;
        }

        out[1] -= ftmp;
    }
}

/// compute whitening filter coefficients from normalized line spectral frequencies
///
/// ```text
/// a_Q12   O   monic whitening filter coefficients in Q12,  [ d ]
/// NLSF    I   normalized line spectral frequencies in Q15, [ d ]
/// d       I   filter order (should be even)
/// arch    I   Run-time architecture
/// ```
pub fn silk_NLSF2A(a_Q12: &mut [i16], NLSF: &[i16]) {
    let d = a_Q12.len();

    /* This ordering was found to maximize quality. It improves the numerical accuracy of
    silk_NLSF2A_find_poly() compared to "standard" ordering. */
    static ordering16: [u8; 16] = [0, 15, 8, 7, 4, 11, 12, 3, 2, 13, 10, 5, 6, 9, 14, 1];
    static ordering10: [u8; 10] = [0, 9, 6, 3, 4, 5, 8, 1, 2, 7];

    assert!(d == 10 || d == 16);

    /* convert LSFs to 2*cos(LSF), using piecewise linear curve from table */
    let ordering = if d == 16 {
        ordering16.as_slice()
    } else {
        ordering10.as_slice()
    };

    let mut cos_LSF_QA: [i32; SILK_MAX_ORDER_LPC] = [0; 24];
    let cos_LSF_QA = &mut cos_LSF_QA[..d + 1];
    azip!((&ordering in ordering, &NLSF in NLSF) {
        debug_assert!(NLSF >= 0);

        /* f_int on a scale 0-127 (rounded down) */
        let f_int = NLSF as i32 >> (15 - 7);

        /* f_frac, range: 0..255 */
        let f_frac = NLSF as i32 - (f_int << (15 - 7));

        debug_assert!(f_int >= 0);
        debug_assert!(f_int < LSF_COS_TAB_SZ_FIX);

        /* Read start and end value from table */
        let cos_val = silk_LSFCosTab_FIX_Q12[f_int as usize] as i32; /* Q12 */
        let delta = silk_LSFCosTab_FIX_Q12[(f_int + 1) as usize] as i32 - cos_val; /* Q12, with a range of 0..200 */

        cos_LSF_QA[ordering as usize] =
            silk_RSHIFT_ROUND((cos_val << 8) + delta * f_frac, 20 - QA); /* QA */
    });

    let dd = d / 2;

    /* generate even and odd polynomials using convolution */
    let mut P: [i32; SILK_MAX_ORDER_LPC / 2 + 1] = [0; 13];
    let mut Q: [i32; SILK_MAX_ORDER_LPC / 2 + 1] = [0; 13];
    let P = &mut P[..dd + 1];
    let Q = &mut Q[..dd + 1];
    silk_NLSF2A_find_poly(P, &cos_LSF_QA[..d]);
    silk_NLSF2A_find_poly(Q, &cos_LSF_QA[1..][..d]);

    /* convert even and odd polynomials to opus_int32 Q12 filter coefs */
    let mut a32_QA1: [i32; SILK_MAX_ORDER_LPC] = [0; 24];
    let a32_QA1 = &mut a32_QA1[..d];
    for k in 0..dd {
        let Ptmp = P[k + 1] + P[k];
        let Qtmp = Q[k + 1] - Q[k];
        /* the Ptmp and Qtmp values at this stage need to fit in int32 */
        a32_QA1[k] = -Qtmp - Ptmp; /* QA+1 */
        a32_QA1[d - k - 1] = Qtmp - Ptmp; /* QA+1 */
    }

    /* Convert int32 coefficients to Q12 int16 coefs */
    silk_LPC_fit(a_Q12, &mut a32_QA1[..d], 12, QA + 1);

    let mut i = 0;
    while silk_LPC_inverse_pred_gain_c(a_Q12) == 0 && i < MAX_LPC_STABILIZE_ITERATIONS {
        /* Prediction coefficients are (too close to) unstable; apply bandwidth expansion   */
        /* on the unscaled coefficients, convert to Q12 and measure again                   */
        silk_bwexpander_32(a32_QA1, 65536 - (2 << i));

        azip!((a_Q12 in &mut a_Q12[..], &a32_QA1 in &a32_QA1[..]) {
            *a_Q12 = silk_RSHIFT_ROUND(a32_QA1, QA + 1 - 12) as i16; /* QA+1 -> Q12 */
        });

        i += 1;
    }
}
