use crate::celt::entdec::{ec_dec, ec_dec_icdf};
use crate::celt::entenc::{ec_enc, ec_enc_icdf};
use crate::silk::define::SHELL_CODEC_FRAME_LENGTH;
use crate::silk::tables_pulses_per_block::silk_sign_iCDF;
// shifting avoids if-statement
fn silk_enc_map(a: i32) -> i32 {
    (a >> 15) + 1
}
fn silk_dec_map(a: i32) -> i32 {
    (a << 1) - 1
}

pub fn silk_encode_signs(
    psRangeEnc: &mut ec_enc,
    pulses: &[i8],
    length: i32,
    signalType: i32,
    quantOffsetType: i32,
    sum_pulses: &[i32],
) {
    let mut icdf: [u8; 2] = [0; 2];
    icdf[1] = 0;
    let mut q_ptr = pulses;

    let i = 7 * (quantOffsetType + ((signalType as u32) << 1) as i32) as i16 as i32;
    let icdf_ptr = &silk_sign_iCDF[i as usize..];

    let length = (length + 16 / 2) >> 4;
    for i in 0..length {
        let p = sum_pulses[i as usize];
        if p > 0 {
            icdf[0] = icdf_ptr[(p & 0x1f).min(6) as usize];

            for j in 0..SHELL_CODEC_FRAME_LENGTH {
                if q_ptr[j as usize] as i32 != 0 {
                    ec_enc_icdf(psRangeEnc, (q_ptr[j as usize] as i32 >> 15) + 1, &icdf, 8);
                }
            }
        }
        q_ptr = &q_ptr[SHELL_CODEC_FRAME_LENGTH as usize..];
    }
}

/// Decodes signs of excitation
///
/// ```text
/// psRangeDec                          I/O   Compressor data structure
/// pulses[]                            I/O   pulse signal
/// length                              I     length of input
/// signalType                          I     Signal type
/// quantOffsetType                     I     Quantization offset type
/// sum_pulses[ MAX_NB_SHELL_BLOCKS ]   I     Sum of absolute pulses per block
/// ```
pub fn silk_decode_signs(
    psRangeDec: &mut ec_dec,
    pulses: &mut [i16],
    signalType: i32,
    quantOffsetType: i32,
    sum_pulses: &[i32],
) {
    assert_eq!(pulses.len(), sum_pulses.len() * SHELL_CODEC_FRAME_LENGTH);

    let icdf_ix = 7 * (quantOffsetType + (signalType << 1)) as usize;
    let icdf_ptr = &silk_sign_iCDF[icdf_ix..];

    for (&p, q_ptr) in std::iter::zip(sum_pulses, pulses.chunks_mut(SHELL_CODEC_FRAME_LENGTH)) {
        if p > 0 {
            let icdf: [u8; 2] = [icdf_ptr[std::cmp::min(p & 0x1f, 6) as usize], 0];
            for q_ptr in q_ptr[..SHELL_CODEC_FRAME_LENGTH].iter_mut() {
                if *q_ptr > 0 {
                    *q_ptr *= silk_dec_map(ec_dec_icdf(psRangeDec, &icdf, 8)) as i16;
                }
            }
        }
    }
}
