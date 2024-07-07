use crate::silk::macros::silk_CLZ32;
use crate::silk::SigProc_FIX::silk_max_32;

fn silk_sum_sqr_shift_inner(mut nrg: i32, shft: i32, x: &[i16]) -> i32 {
    let len = x.len();

    let mut i = 0;
    while i < len - 1 {
        let nrg_tmp = x[i] as i32 * x[i] as i32;
        let nrg_tmp = nrg_tmp.wrapping_add(x[i + 1] as i32 * x[i + 1] as i32);
        nrg = nrg.wrapping_add(nrg_tmp >> shft);
        i += 2;
    }
    if i < len {
        /* One sample left to process */
        let nrg_tmp = x[i] as i32 * x[i] as i32;
        nrg = nrg.wrapping_add(nrg_tmp >> shft);
    }

    debug_assert!(nrg >= 0);

    nrg
}

/// Compute number of bits to right shift the sum of squares of a vector of int16s to make it fit in an int32
///
/// ```text
/// energy   O   Energy of x, after shifting to the right
/// shift    O   Number of bits right shift applied to energy
/// x        I   Input vector
/// len      I   Length of input vector
/// ```
pub fn silk_sum_sqr_shift(energy: &mut i32, shift: &mut i32, x: &[i16]) {
    let len = x.len();

    /* Do a first run with the maximum shift we could have. */
    let shft = 31 - silk_CLZ32(len as i32);
    /* Let's be conservative with rounding and start with nrg=len. */
    let nrg = silk_sum_sqr_shift_inner(len as i32, shft, x);

    /* Make sure the result will fit in a 32-bit signed integer with two bits
    of headroom. */
    let shft = silk_max_32(0, shft + 3 - silk_CLZ32(nrg));
    let nrg = silk_sum_sqr_shift_inner(0, shft, x);

    /* Output arguments */
    *shift = shft;
    *energy = nrg;
}
