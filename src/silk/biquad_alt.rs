use crate::silk::macros::{silk_SMLAWB, silk_SMULWB};
use crate::silk::SigProc_FIX::{silk_RSHIFT_ROUND, silk_SAT16};
use ndarray::azip;

/// Second order ARMA filter, alternative implementation
///
/// Slower than biquad() but uses more precise coefficients.
/// Can handle (slowly) varying coefficients.
///
/// ```text
/// B_Q28  I     MA coefficients [3]
/// A_Q28  I     AR coefficients [2]
/// S      I/O   State vector [2]
/// in     I/O   input/output signal, length must be even
/// ```
pub fn silk_biquad_alt_stride1(
    B_Q28: &[i32; 3],
    A_Q28: &[i32; 2],
    S: &mut [i32; 2],
    signal: &mut [i16],
) {
    /* DIRECT FORM II TRANSPOSED (uses 2 element state vector) */

    /* Negate A_Q28 values and split in two parts */
    let A0_L_Q28 = -A_Q28[0] & 0x3fff;
    let A0_U_Q28 = -A_Q28[0] >> 14;
    let A1_L_Q28 = -A_Q28[1] & 0x3fff;
    let A1_U_Q28 = -A_Q28[1] >> 14;

    assert_eq!(signal.len() % 2, 0);

    azip!(
        (signal in signal) {
            let inval = *signal as i32;

            /* S[ 0 ], S[ 1 ]: Q12 */
            let out32_Q14 = silk_SMLAWB(S[0], B_Q28[0], inval) << 2;

            S[0] = S[1] + silk_RSHIFT_ROUND(silk_SMULWB(out32_Q14, A0_L_Q28), 14);
            S[0] = silk_SMLAWB(S[0], out32_Q14, A0_U_Q28);
            S[0] = silk_SMLAWB(S[0], B_Q28[1], inval);

            S[1] = silk_RSHIFT_ROUND(silk_SMULWB(out32_Q14, A1_L_Q28), 14);
            S[1] = silk_SMLAWB(S[1], out32_Q14, A1_U_Q28);
            S[1] = silk_SMLAWB(S[1], B_Q28[2], inval);

            /* Scale back to Q0 and saturate */
            *signal = silk_SAT16((out32_Q14 + (1 << 14) - 1) >> 14) as i16;
        }
    );
}
