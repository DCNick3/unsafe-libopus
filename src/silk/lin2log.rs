use crate::silk::Inlines::silk_CLZ_FRAC;

// Approximation of 128 * log2() (very close inverse of silk_log2lin())
// Convert input to a log scale
pub fn silk_lin2log(inLin: i32) -> i32 {
    let mut lz: i32 = 0;
    let mut frac_Q7: i32 = 0;
    silk_CLZ_FRAC(inLin, &mut lz, &mut frac_Q7);
    /* Piece-wise parabolic approximation */
    (frac_Q7 as i64 + ((frac_Q7 * (128 - frac_Q7)) as i64 * 179 as i64 >> 16)) as i32
        + (((31 - lz) as u32) << 7) as i32
}
