use ndarray::{aview1, azip, s};

use crate::silk::SigProc_FIX::{silk_RSHIFT_ROUND, silk_SAT16};

/// LPC analysis filter
///
/// NB! State is kept internally and the
/// filter always starts with zero state
/// first d output samples are set to zero
///
/// out  O    Output signal
/// in   I    Input signal
/// B    I    MA prediction coefficients, Q12 [order]
/// len  I    Signal length
/// d    I    Filter order
pub fn silk_LPC_analysis_filter(out: &mut [i16], input: &[i16], B: &[i16]) {
    let len = input.len();
    let d = B.len();

    assert!(d >= 6);
    assert_eq!(d % 2, 0);
    assert!(d <= len);
    assert_eq!(out.len(), len);

    let input = aview1(input);
    let B = aview1(B);

    azip!((out in &mut out[d..], w in input.windows(d + 1)) {
        let in_ptr = w.slice(s![..d;-1]);

        let mut out32_Q12 = 0i32;
        /* Allowing wrap around so that two wraps can cancel each other. The rare
        cases where the result wraps around can only be triggered by invalid streams*/
        azip!((&x in in_ptr, &b in B ) {
            out32_Q12 = out32_Q12.wrapping_add(x as i32 * b as i32);
        });
        /* Subtract prediction */
        out32_Q12 = ((w[d] as i32) << 12).wrapping_sub(out32_Q12);

        /* Scale to Q0 */
        let out32 = silk_RSHIFT_ROUND(out32_Q12, 12);

        /* Saturate output */
        *out = silk_SAT16(out32) as i16;
    });

    /* Set first d output samples to zero */
    out[..d].fill(0);
}
