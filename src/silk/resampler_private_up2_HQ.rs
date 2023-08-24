use crate::silk::resampler_rom::{silk_resampler_up2_hq_0, silk_resampler_up2_hq_1};

pub mod typedef_h {
    pub const silk_int16_MAX: i32 = i16::MAX as i32;
    pub const silk_int16_MIN: i32 = i16::MIN as i32;
}

pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};

#[derive(Default)]
pub struct ResamplerUp2HqState {
    iir_state: [i32; 6],
}

/* Upsample by a factor 2, high quality */
/* Uses 2nd order allpass filters for the 2x upsampling, followed by a      */
/* notch filter just above Nyquist.                                         */
pub fn silk_resampler_private_up2_HQ(S: &mut [i32], out: &mut [i16], in_0: &[i16]) {
    assert_eq!(out.len(), 2 * in_0.len());

    /* Internal variables and state are in Q10 format */
    for k in 0..in_0.len() {
        /* Convert to Q10 */
        let in32 = ((in_0[k] as i32 as u32) << 10) as i32;

        /* First all-pass section for even output sample */
        let Y = in32 - S[0];
        let X = ((Y as i64 * silk_resampler_up2_hq_0[0] as i64) >> 16) as i32;
        let out32_1 = S[0] + X;
        S[0] = in32 + X;

        /* Second all-pass section for even output sample */
        let Y = out32_1 - S[1];
        let X = ((Y as i64 * silk_resampler_up2_hq_0[1] as i64) >> 16) as i32;
        let out32_2 = S[1] + X;
        S[1] = out32_1 + X;

        /* Third all-pass section for even output sample */
        let Y = out32_2 - S[2];
        let X = (Y as i64 + ((Y as i64 * silk_resampler_up2_hq_0[2] as i64) >> 16)) as i32;
        let out32_1 = S[2] + X;
        S[2] = out32_2 + X;

        /* Apply gain in Q15, convert back to int16 and store to output */
        out[2 * k] = (if (if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) as i16;

        /* First all-pass section for odd output sample */
        let Y = in32 - S[3];
        let X = ((Y as i64 * silk_resampler_up2_hq_1[0] as i64) >> 16) as i32;
        let out32_1 = S[3] + X;
        S[3] = in32 + X;

        /* Second all-pass section for odd output sample */
        let Y = out32_1 - S[4];
        let X = ((Y as i64 * silk_resampler_up2_hq_1[1] as i64) >> 16) as i32;
        let out32_2 = S[4] + X;
        S[4] = out32_1 + X;

        /* Third all-pass section for odd output sample */
        let Y = out32_2 - S[5];
        let X = (Y as i64 + ((Y as i64 * silk_resampler_up2_hq_1[2] as i64) >> 16)) as i32;
        let out32_1 = S[5] + X;
        S[5] = out32_2 + X;

        /* Apply gain in Q15, convert back to int16 and store to output */
        out[2 * k + 1] = (if (if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) as i16;
    }
}
