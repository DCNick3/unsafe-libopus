use crate::silk::define::MAX_PREDICTION_POWER_GAIN;
use crate::silk::macros::silk_CLZ32;
use crate::silk::Inlines::silk_INVERSE32_varQ;
use crate::silk::SigProc_FIX::{
    silk_RSHIFT_ROUND64, silk_SMMUL, SILK_FIX_CONST, SILK_MAX_ORDER_LPC,
};

const QA: i32 = 24;
const A_LIMIT: i32 = SILK_FIX_CONST!(0.99975, QA);

fn MUL32_FRAC_Q(a32: i32, b32: i32, Q: i32) -> i32 {
    silk_RSHIFT_ROUND64(a32 as i64 * b32 as i64, Q) as i32
}

/// Compute inverse of LPC prediction gain, and test if LPC coefficients are stable (all poles within unit circle)
///
/// ```text
///                              O   Returns inverse prediction gain in energy domain, Q30
/// A_QA[ SILK_MAX_ORDER_LPC ]   I   Prediction coefficients
/// order                        I   Prediction order
/// ```
fn LPC_inverse_pred_gain_QA_c(A_QA: &mut [i32]) -> i32 {
    let order = A_QA.len();

    let mut invGain_Q30 = SILK_FIX_CONST!(1.0, 30);
    let mut k = order - 1;
    while k > 0 {
        /* Check for stability */
        if A_QA[k] > A_LIMIT || A_QA[k] < -A_LIMIT {
            return 0;
        }

        /* Set RC equal to negated AR coef */
        let rc_Q31 = -(A_QA[k] << (31 - QA));

        /* rc_mult1_Q30 range: [ 1 : 2^30 ] */
        let rc_mult1_Q30 = SILK_FIX_CONST!(1, 30) - silk_SMMUL(rc_Q31, rc_Q31);
        debug_assert!(rc_mult1_Q30 > (1 << 15)); /* reduce A_LIMIT if fails */
        debug_assert!(rc_mult1_Q30 <= (1 << 30));

        /* Update inverse gain */
        /* invGain_Q30 range: [ 0 : 2^30 ] */
        invGain_Q30 = silk_SMMUL(invGain_Q30, rc_mult1_Q30) << 2;
        debug_assert!(invGain_Q30 >= 0);
        debug_assert!(invGain_Q30 <= (1 << 30));
        if invGain_Q30 < SILK_FIX_CONST!(1.0 / MAX_PREDICTION_POWER_GAIN, 30) {
            return 0;
        }

        /* rc_mult2 range: [ 2^30 : silk_int32_MAX ] */
        let mult2Q = 32 - silk_CLZ32(rc_mult1_Q30.abs());
        let rc_mult2 = silk_INVERSE32_varQ(rc_mult1_Q30, mult2Q + 30);

        /* Update AR coefficient */
        let mut n = 0;
        while n < (k + 1) / 2 {
            let tmp1 = A_QA[n];
            let tmp2 = A_QA[k - n - 1];
            let tmp64 = silk_RSHIFT_ROUND64(
                tmp1.saturating_sub(MUL32_FRAC_Q(tmp2, rc_Q31, 31)) as i64 * rc_mult2 as i64,
                mult2Q,
            );

            if tmp64 > i32::MAX as i64 || tmp64 < i32::MIN as i64 {
                return 0;
            }
            A_QA[n] = tmp64 as i32;
            let tmp64 = silk_RSHIFT_ROUND64(
                tmp2.saturating_sub(MUL32_FRAC_Q(tmp1, rc_Q31, 31)) as i64 * rc_mult2 as i64,
                mult2Q,
            );

            if tmp64 > i32::MAX as i64 || tmp64 < i32::MIN as i64 {
                return 0;
            }
            A_QA[k - n - 1] = tmp64 as i32;
            n += 1;
        }
        k -= 1;
    }

    /* Check for stability */
    if A_QA[k] > A_LIMIT || A_QA[k] < -A_LIMIT {
        return 0;
    }

    /* Set RC equal to negated AR coef */
    let rc_Q31 = -(A_QA[0] << (31 - QA));

    /* Range: [ 1 : 2^30 ] */
    let rc_mult1_Q30 = SILK_FIX_CONST!(1, 30) - silk_SMMUL(rc_Q31, rc_Q31);

    /* Update inverse gain */
    /* Range: [ 0 : 2^30 ] */
    let invGain_Q30 = silk_SMMUL(invGain_Q30, rc_mult1_Q30) << 2;
    debug_assert!(invGain_Q30 >= 0);
    debug_assert!(invGain_Q30 <= (1 << 30));
    if invGain_Q30 < SILK_FIX_CONST!(1.0 / MAX_PREDICTION_POWER_GAIN, 30) {
        0
    } else {
        invGain_Q30
    }
}

/// Compute inverse of LPC prediction gain, and test if LPC coefficients are stable (all poles within unit circle).
///
/// ```text
///         O   Returns inverse prediction gain in energy domain, Q30
/// A_Q12   I   Prediction coefficients, Q12 [order]
/// order   I   Prediction order
/// ```
pub fn silk_LPC_inverse_pred_gain_c(A_Q12: &[i16]) -> i32 {
    let mut Atmp_QA: [i32; SILK_MAX_ORDER_LPC] = [0; 24];
    let mut DC_resp: i32 = 0;

    let Atmp_QA = &mut Atmp_QA[..A_Q12.len()];

    /* Increase Q domain of the AR coefficients */
    let mut k = 0;
    while k < A_Q12.len() {
        DC_resp += A_Q12[k] as i32;
        Atmp_QA[k] = (A_Q12[k] as i32) << (QA - 12);
        k += 1;
    }
    /* If the DC is unstable, we don't even need to do the full calculations */
    if DC_resp >= 4096 {
        return 0;
    }
    return LPC_inverse_pred_gain_QA_c(Atmp_QA);
}
