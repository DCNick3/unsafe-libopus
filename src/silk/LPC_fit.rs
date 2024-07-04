use crate::silk::bwexpander_32::silk_bwexpander_32;
use crate::silk::SigProc_FIX::{silk_RSHIFT_ROUND, silk_SAT16, SILK_FIX_CONST};
use ndarray::azip;

/// Convert int32 coefficients to int16 coefs and make sure there's no wrap-around
///
/// ```text
/// a_QOUT   O     Output signal
/// a_QIN    I/O   Input signal
/// QOUT     I     Input Q domain
/// QIN      I     Input Q domain
/// d        I     Filter order
/// ```
pub fn silk_LPC_fit(a_QOUT: &mut [i16], a_QIN: &mut [i32], QOUT: i32, QIN: i32) {
    let d = a_QOUT.len();
    assert_eq!(a_QIN.len(), d);

    /* Limit the maximum absolute value of the prediction coefficients, so that they'll fit in int16 */
    let mut i = 0;
    while i < 10 {
        /* Find maximum absolute value and its index */
        let mut maxabs = 0;
        let mut idx = 0;
        let mut k = 0;
        while k < d {
            let absval = a_QIN[k].abs();
            if absval > maxabs {
                maxabs = absval;
                idx = k;
            }
            k += 1;
        }
        maxabs = silk_RSHIFT_ROUND(maxabs, QIN - QOUT);

        if maxabs > i16::MAX as i32 {
            /* Reduce magnitude of prediction coefficients */
            maxabs = std::cmp::min(maxabs, 163838); /* ( silk_int32_MAX >> 14 ) + silk_int16_MAX = 163838 */
            let chirp_Q16 = SILK_FIX_CONST!(0.999f64, 16)
                - ((maxabs - i16::MAX as i32) << 14) / ((maxabs * (idx as i32 + 1)) >> 2);
            silk_bwexpander_32(a_QIN, chirp_Q16);
        } else {
            break;
        }

        i += 1;
    }

    if i == 10 {
        /* Reached the last iteration, clip the coefficients */
        azip!((out in a_QOUT, input in a_QIN) {
            *out = silk_SAT16(silk_RSHIFT_ROUND(*input, QIN - QOUT)) as i16;
            *input = (*out as i32) << (QIN - QOUT);
        });
    } else {
        azip!((out in a_QOUT, &mut input in a_QIN) {
            *out = silk_RSHIFT_ROUND(input, QIN - QOUT) as i16;
        });
    };
}
