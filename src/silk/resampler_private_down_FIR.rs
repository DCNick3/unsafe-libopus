pub mod typedef_h {
    pub const silk_int16_MAX: i32 = i16::MAX as i32;
    pub const silk_int16_MIN: i32 = i16::MIN as i32;
}
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
use crate::silk::resampler::{
    silk_resampler_state_struct, SILK_RESAMPLER_MAX_FIR_ORDER, SILK_RESAMPLER_MAX_IIR_ORDER,
};
use crate::silk::resampler_private_AR2::silk_resampler_private_AR2;
use crate::silk::resampler_rom::{
    RESAMPLER_DOWN_ORDER_FIR0, RESAMPLER_DOWN_ORDER_FIR1, RESAMPLER_DOWN_ORDER_FIR2,
};

pub struct ResamplerDownFirParams {
    fir_order: i32,
    fir_fracs: i32,
    coefs: &'static [i16],
}

pub struct ResamplerDownFirState {
    iir_state: [i32; SILK_RESAMPLER_MAX_IIR_ORDER],
    fir_state: [i32; SILK_RESAMPLER_MAX_FIR_ORDER],
}

impl Default for ResamplerDownFirState {
    fn default() -> Self {
        ResamplerDownFirState {
            iir_state: [0; SILK_RESAMPLER_MAX_IIR_ORDER],
            fir_state: [0; SILK_RESAMPLER_MAX_FIR_ORDER],
        }
    }
}

#[inline]
fn silk_resampler_private_down_FIR_INTERPOL<'a>(
    mut out: &'a mut [i16],
    buf: &[i32],
    FIR_Coefs: &[i16],
    FIR_Order: i32,
    FIR_Fracs: i32,
    max_index_Q16: i32,
    index_increment_Q16: i32,
) -> &'a mut [i16] {
    match FIR_Order {
        RESAMPLER_DOWN_ORDER_FIR0 => {
            let mut index_Q16 = 0;
            while index_Q16 < max_index_Q16 {
                /* Integer part gives pointer to buffered input */
                let buf_ptr = &buf[(index_Q16 >> 16) as usize..];

                /* Fractional part gives interpolation coefficients */
                let interpol_ind =
                    (((index_Q16 & 0xffff) as i64 * FIR_Fracs as i16 as i64) >> 16) as usize;

                /* Inner product */
                let interpol_ptr =
                    &FIR_Coefs[(RESAMPLER_DOWN_ORDER_FIR0 as usize / 2 * interpol_ind)..];
                let mut res_Q6 = ((buf_ptr[0] as i64 * interpol_ptr[0] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[1] as i64 * interpol_ptr[1] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[2] as i64 * interpol_ptr[2] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[3] as i64 * interpol_ptr[3] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[4] as i64 * interpol_ptr[4] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[5] as i64 * interpol_ptr[5] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[6] as i64 * interpol_ptr[6] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[7] as i64 * interpol_ptr[7] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[8] as i64 * interpol_ptr[8] as i64) >> 16) as i32;

                let interpol_ptr = &FIR_Coefs[(RESAMPLER_DOWN_ORDER_FIR0 as usize / 2
                    * (FIR_Fracs as usize - 1 - interpol_ind))..];
                res_Q6 += ((buf_ptr[17] as i64 * interpol_ptr[0] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[16] as i64 * interpol_ptr[1] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[15] as i64 * interpol_ptr[2] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[14] as i64 * interpol_ptr[3] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[13] as i64 * interpol_ptr[4] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[12] as i64 * interpol_ptr[5] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[11] as i64 * interpol_ptr[6] as i64) >> 16) as i32;
                res_Q6 += ((buf_ptr[10] as i64 * interpol_ptr[7] as i64) >> 16) as i32;
                res_Q6 += (buf_ptr[9] as i64 * interpol_ptr[8] as i64 >> 16) as i32;

                /* Scale down, saturate and store in output array */
                out[0] = (if (if 6 == 1 {
                    (res_Q6 >> 1) + (res_Q6 & 1)
                } else {
                    (res_Q6 >> 6 - 1) + 1 >> 1
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 == 1 {
                    (res_Q6 >> 1) + (res_Q6 & 1)
                } else {
                    (res_Q6 >> 6 - 1) + 1 >> 1
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 == 1 {
                    (res_Q6 >> 1) + (res_Q6 & 1)
                } else {
                    (res_Q6 >> 6 - 1) + 1 >> 1
                }) as i16;
                out = &mut out[1..];

                index_Q16 += index_increment_Q16;
            }
        }
        RESAMPLER_DOWN_ORDER_FIR1 => {
            let mut index_Q16 = 0;
            while index_Q16 < max_index_Q16 {
                /* Integer part gives pointer to buffered input */
                let buf_ptr = &buf[(index_Q16 >> 16) as usize..];

                let mut res_Q6 =
                    (((buf_ptr[0] + buf_ptr[23]) as i64 * FIR_Coefs[0] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[1] + buf_ptr[22]) as i64 * FIR_Coefs[1] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[2] + buf_ptr[21]) as i64 * FIR_Coefs[2] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[3] + buf_ptr[20]) as i64 * FIR_Coefs[3] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[4] + buf_ptr[19]) as i64 * FIR_Coefs[4] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[5] + buf_ptr[18]) as i64 * FIR_Coefs[5] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[6] + buf_ptr[17]) as i64 * FIR_Coefs[6] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[7] + buf_ptr[16]) as i64 * FIR_Coefs[7] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[8] + buf_ptr[15]) as i64 * FIR_Coefs[8] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[9] + buf_ptr[14]) as i64 * FIR_Coefs[9] as i64) >> 16) as i32;
                res_Q6 +=
                    (((buf_ptr[10] + buf_ptr[13]) as i64 * FIR_Coefs[10] as i64) >> 16) as i32;
                res_Q6 +=
                    (((buf_ptr[11] + buf_ptr[12]) as i64 * FIR_Coefs[11] as i64) >> 16) as i32;

                out[0] = (if (if 6 == 1 {
                    (res_Q6 >> 1) + (res_Q6 & 1)
                } else {
                    (res_Q6 >> 6 - 1) + 1 >> 1
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 == 1 {
                    (res_Q6 >> 1) + (res_Q6 & 1)
                } else {
                    (res_Q6 >> 6 - 1) + 1 >> 1
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 == 1 {
                    (res_Q6 >> 1) + (res_Q6 & 1)
                } else {
                    (res_Q6 >> 6 - 1) + 1 >> 1
                }) as i16;
                out = &mut out[1..];

                index_Q16 += index_increment_Q16;
            }
        }
        RESAMPLER_DOWN_ORDER_FIR2 => {
            let mut index_Q16 = 0;
            while index_Q16 < max_index_Q16 {
                /* Integer part gives pointer to buffered input */
                let buf_ptr = &buf[(index_Q16 >> 16) as usize..];

                let mut res_Q6 =
                    (((buf_ptr[0] + buf_ptr[35]) as i64 * FIR_Coefs[0] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[1] + buf_ptr[34]) as i64 * FIR_Coefs[1] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[2] + buf_ptr[33]) as i64 * FIR_Coefs[2] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[3] + buf_ptr[32]) as i64 * FIR_Coefs[3] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[4] + buf_ptr[31]) as i64 * FIR_Coefs[4] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[5] + buf_ptr[30]) as i64 * FIR_Coefs[5] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[6] + buf_ptr[29]) as i64 * FIR_Coefs[6] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[7] + buf_ptr[28]) as i64 * FIR_Coefs[7] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[8] + buf_ptr[27]) as i64 * FIR_Coefs[8] as i64) >> 16) as i32;
                res_Q6 += (((buf_ptr[9] + buf_ptr[26]) as i64 * FIR_Coefs[9] as i64) >> 16) as i32;
                res_Q6 +=
                    (((buf_ptr[10] + buf_ptr[25]) as i64 * FIR_Coefs[10] as i64) >> 16) as i32;
                res_Q6 +=
                    (((buf_ptr[11] + buf_ptr[24]) as i64 * FIR_Coefs[11] as i64) >> 16) as i32;
                res_Q6 +=
                    (((buf_ptr[12] + buf_ptr[23]) as i64 * FIR_Coefs[12] as i64) >> 16) as i32;
                res_Q6 +=
                    (((buf_ptr[13] + buf_ptr[22]) as i64 * FIR_Coefs[13] as i64) >> 16) as i32;
                res_Q6 +=
                    (((buf_ptr[14] + buf_ptr[21]) as i64 * FIR_Coefs[14] as i64) >> 16) as i32;
                res_Q6 +=
                    (((buf_ptr[15] + buf_ptr[20]) as i64 * FIR_Coefs[15] as i64) >> 16) as i32;
                res_Q6 +=
                    (((buf_ptr[16] + buf_ptr[19]) as i64 * FIR_Coefs[16] as i64) >> 16) as i32;
                res_Q6 +=
                    (((buf_ptr[17] + buf_ptr[18]) as i64 * FIR_Coefs[17] as i64) >> 16) as i32;

                out[0] = (if (if 6 == 1 {
                    (res_Q6 >> 1) + (res_Q6 & 1)
                } else {
                    (res_Q6 >> 6 - 1) + 1 >> 1
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 == 1 {
                    (res_Q6 >> 1) + (res_Q6 & 1)
                } else {
                    (res_Q6 >> 6 - 1) + 1 >> 1
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 == 1 {
                    (res_Q6 >> 1) + (res_Q6 & 1)
                } else {
                    (res_Q6 >> 6 - 1) + 1 >> 1
                }) as i16;
                out = &mut out[1..];

                index_Q16 += index_increment_Q16;
            }
        }
        _ => unreachable!(),
    }

    out
}

pub unsafe fn silk_resampler_private_down_FIR(
    S: &mut silk_resampler_state_struct,
    mut out: &mut [i16],
    mut in_0: &[i16],
) {
    let mut nSamplesIn: usize = 0;

    let mut buf: Vec<i32> = ::std::vec::from_elem(0, (S.batchSize + S.FIR_Order) as usize);

    buf[..S.FIR_Order as usize].copy_from_slice(&S.sFIR.i32_0[..S.FIR_Order as usize]);

    let FIR_Coefs = &S.Coefs[2..];
    let index_increment_Q16 = S.invRatio_Q16;
    loop {
        nSamplesIn = in_0.len().min(S.batchSize as usize);
        silk_resampler_private_AR2(
            &mut S.sIIR,
            &mut buf[S.FIR_Order as usize..][..nSamplesIn],
            &in_0[..nSamplesIn],
            S.Coefs,
        );
        let max_index_Q16 = ((nSamplesIn as u32) << 16) as i32;
        out = silk_resampler_private_down_FIR_INTERPOL(
            out,
            &buf,
            FIR_Coefs,
            S.FIR_Order,
            S.FIR_Fracs,
            max_index_Q16,
            index_increment_Q16,
        );
        in_0 = &in_0[nSamplesIn..];
        if in_0.is_empty() {
            break;
        }

        buf.copy_within(nSamplesIn..nSamplesIn + S.FIR_Order as usize, 0);
    }
    S.sFIR.i32_0[..S.FIR_Order as usize]
        .copy_from_slice(&buf[nSamplesIn..nSamplesIn + S.FIR_Order as usize]);
}
