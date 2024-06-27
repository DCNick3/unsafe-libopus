use crate::silk::SigProc_FIX::silk_RSHIFT_ROUND;

/// Chirp (bandwidth expand) LP AR filter
///
/// `ar`:        I/O  AR filter to be expanded (without leading 1)
/// `d`:         I    number of parameters in the AR filter
/// `chirp_Q16`: I    chirp factor (typically in the range 0 to 1)
pub fn silk_bwexpander(ar: &mut [i16], mut chirp_Q16: i32) {
    let d = ar.len();

    let chirp_minus_one_Q16: i32 = chirp_Q16 - 65536;

    /* NB: Dont use silk_SMULWB, instead of silk_RSHIFT_ROUND( silk_MUL(), 16 ), below.  */
    /* Bias in silk_SMULWB can lead to unstable filters                                */
    for i in 0..d - 1 {
        ar[i] = silk_RSHIFT_ROUND(chirp_Q16 * ar[i] as i32, 16) as i16;
        chirp_Q16 += silk_RSHIFT_ROUND(chirp_Q16 * chirp_minus_one_Q16, 16);
    }

    ar[d - 1] = silk_RSHIFT_ROUND(chirp_Q16 * ar[d - 1] as i32, 16) as i16;
}
