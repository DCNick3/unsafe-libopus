use crate::silk::macros::silk_SMULWW;
use crate::silk::SigProc_FIX::silk_RSHIFT_ROUND;

/// Chirp (bandwidth expand) LP AR filter
///
/// ```text
/// ar          I/O   AR filter to be expanded (without leading 1)
/// d           I     Length of ar
/// chirp_Q16   I     Chirp factor in Q16
/// ```
pub fn silk_bwexpander_32(ar: &mut [i32], mut chirp_Q16: i32) {
    let d = ar.len();

    let chirp_minus_one_Q16: i32 = chirp_Q16 - 65536;

    for ar in ar.iter_mut().take(d - 1) {
        *ar = silk_SMULWW(chirp_Q16, *ar);
        chirp_Q16 += silk_RSHIFT_ROUND(chirp_Q16 * chirp_minus_one_Q16, 16)
    }

    ar[d - 1] = silk_SMULWW(chirp_Q16, ar[d - 1]);
}
