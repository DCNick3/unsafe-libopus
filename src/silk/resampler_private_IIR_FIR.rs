pub mod typedef_h {
    pub const silk_int16_MAX: i32 = i16::MAX as i32;
    pub const silk_int16_MIN: i32 = i16::MIN as i32;
}
use crate::silk::resampler::silk_resampler_state_struct;

pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};

use crate::silk::resampler_private_up2_HQ::{silk_resampler_private_up2_HQ, ResamplerUp2HqState};
use crate::silk::resampler_rom::{silk_resampler_frac_FIR_12, RESAMPLER_ORDER_FIR_12};

#[derive(Default)]
pub struct ResamplerIirFirState {
    up2_HQ: ResamplerUp2HqState,
    fir_state: [i16; RESAMPLER_ORDER_FIR_12],
}

#[inline]
fn silk_resampler_private_IIR_FIR_INTERPOL<'a>(
    mut out: &'a mut [i16],
    buf: &[i16],
    max_index_Q16: i32,
    index_increment_Q16: i32,
) -> &'a mut [i16] {
    let mut res_Q15: i32 = 0;

    /* Interpolate upsampled signal and store in output array */
    let mut index_Q16 = 0;
    while index_Q16 < max_index_Q16 {
        let table_index = ((index_Q16 & 0xffff) as i64 * 12 as i64 >> 16) as usize;
        let buf_ptr = &buf[(index_Q16 >> 16) as usize..][..8];

        res_Q15 = buf_ptr[0] as i32 * silk_resampler_frac_FIR_12[table_index][0] as i32;
        res_Q15 += buf_ptr[1] as i32 * silk_resampler_frac_FIR_12[table_index][1] as i32;
        res_Q15 += buf_ptr[2] as i32 * silk_resampler_frac_FIR_12[table_index][2] as i32;
        res_Q15 += buf_ptr[3] as i32 * silk_resampler_frac_FIR_12[table_index][3] as i32;
        res_Q15 += buf_ptr[4] as i32 * silk_resampler_frac_FIR_12[11 - table_index][3] as i32;
        res_Q15 += buf_ptr[5] as i32 * silk_resampler_frac_FIR_12[11 - table_index][2] as i32;
        res_Q15 += buf_ptr[6] as i32 * silk_resampler_frac_FIR_12[11 - table_index][1] as i32;
        res_Q15 += buf_ptr[7] as i32 * silk_resampler_frac_FIR_12[11 - table_index][0] as i32;

        out[0] = (if (if 15 == 1 {
            (res_Q15 >> 1) + (res_Q15 & 1)
        } else {
            (res_Q15 >> 15 - 1) + 1 >> 1
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 15 == 1 {
            (res_Q15 >> 1) + (res_Q15 & 1)
        } else {
            (res_Q15 >> 15 - 1) + 1 >> 1
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 15 == 1 {
            (res_Q15 >> 1) + (res_Q15 & 1)
        } else {
            (res_Q15 >> 15 - 1) + 1 >> 1
        }) as i16;

        out = &mut out[1..];

        index_Q16 += index_increment_Q16;
    }

    out
}

/* Upsample using a combination of allpass-based 2x upsampling and FIR interpolation */
pub unsafe fn silk_resampler_private_IIR_FIR(
    S: &mut silk_resampler_state_struct,
    mut out: &mut [i16],
    mut in_0: &[i16],
) {
    let mut nSamplesIn: usize = 0;
    let mut max_index_Q16: i32 = 0;
    let mut index_increment_Q16: i32 = 0;
    let vla = (2 * S.batchSize + 8) as usize;
    let mut buf: Vec<i16> = ::std::vec::from_elem(0, vla);

    /* Copy buffered samples to start of buffer */
    buf[0..RESAMPLER_ORDER_FIR_12].copy_from_slice(&S.sFIR.i16_0[..RESAMPLER_ORDER_FIR_12]);

    // let mut in_0 = std::slice::from_raw_parts(in_0, inLen as usize);

    // let out_size = inLen * S.Fs_out_kHz / S.Fs_in_kHz;
    // let mut out = std::slice::from_raw_parts_mut(out, out_size as usize);

    /* Iterate over blocks of frameSizeIn input samples */
    index_increment_Q16 = S.invRatio_Q16;
    loop {
        nSamplesIn = (S.batchSize as usize).min(in_0.len());
        silk_resampler_private_up2_HQ(
            &mut S.sIIR,
            &mut buf[RESAMPLER_ORDER_FIR_12..][..nSamplesIn * 2],
            &in_0[..nSamplesIn],
        );
        max_index_Q16 = ((nSamplesIn as u32) << (16 + 1)) as i32; /* + 1 because 2x upsampling */
        out =
            silk_resampler_private_IIR_FIR_INTERPOL(out, &buf, max_index_Q16, index_increment_Q16);
        in_0 = &in_0[nSamplesIn..];

        if in_0.is_empty() {
            break;
        }

        /* More iterations to do; copy last part of filtered signal to beginning of buffer */
        buf.copy_within(nSamplesIn * 2..nSamplesIn * 2 + RESAMPLER_ORDER_FIR_12, 0);
    }

    /* Copy last part of filtered signal to the state for the next call */
    S.sFIR.i16_0[..RESAMPLER_ORDER_FIR_12]
        .copy_from_slice(&buf[nSamplesIn * 2..][..RESAMPLER_ORDER_FIR_12]);
}
