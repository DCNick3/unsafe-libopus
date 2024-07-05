use crate::celt::entdec::{ec_dec, ec_dec_icdf};
use crate::silk::code_signs::silk_decode_signs;
use crate::silk::define::{N_RATE_LEVELS, SHELL_CODEC_FRAME_LENGTH, SILK_MAX_PULSES};
use crate::silk::shell_coder::silk_shell_decoder;
use crate::silk::tables_other::silk_lsb_iCDF;
use crate::silk::tables_pulses_per_block::{silk_pulses_per_block_iCDF, silk_rate_levels_iCDF};
use itertools::izip;

/// Decode quantization indices of excitation
///
/// NB: when operating on 10ms frame size @ 12 kHz, the `pulses` should be larger than the frame size (to make it contain a whole amount of shell frames)
///
/// ```text
/// psRangeDec        I/O   Compressor data structure
/// pulses[]          O     Excitation signal
/// signalType        I     Sigtype
/// quantOffsetType   I     quantOffsetType
/// frame_length      I     Frame length
/// ```
pub fn silk_decode_pulses(
    psRangeDec: &mut ec_dec,
    pulses: &mut [i16],
    signalType: i32,
    quantOffsetType: i32,
) {
    /*********************/
    /* Decode rate level */
    /*********************/
    let RateLevelIndex = ec_dec_icdf(
        psRangeDec,
        &(silk_rate_levels_iCDF[(signalType >> 1) as usize]),
        8,
    );
    let iter = pulses.len() / SHELL_CODEC_FRAME_LENGTH;
    assert_eq!(pulses.len(), iter * SHELL_CODEC_FRAME_LENGTH);

    let mut sum_pulses: [i32; 20] = [0; 20];
    let mut nLshifts: [i32; 20] = [0; 20];

    let sum_pulses = &mut sum_pulses[..iter];
    let nLshifts = &mut nLshifts[..iter];

    /***************************************************/
    /* Sum-Weighted-Pulses Decoding                    */
    /***************************************************/
    let cdf_ptr = &silk_pulses_per_block_iCDF[RateLevelIndex as usize];
    for (out_nLshifts, out_sum_pulse) in izip!(nLshifts.iter_mut(), sum_pulses.iter_mut()) {
        let mut nLshifts = 0;
        let mut sum_pulses = ec_dec_icdf(psRangeDec, cdf_ptr, 8);
        /* LSB indication */
        while sum_pulses == SILK_MAX_PULSES as i32 + 1 {
            nLshifts += 1;
            /* When we've already got 10 LSBs, we shift the table to not allow (SILK_MAX_PULSES + 1) */
            sum_pulses = ec_dec_icdf(
                psRangeDec,
                &silk_pulses_per_block_iCDF[N_RATE_LEVELS - 1][(nLshifts == 10) as i32 as usize..],
                8,
            );
        }

        *out_nLshifts = nLshifts;
        *out_sum_pulse = sum_pulses;
    }

    /***************************************************/
    /* Shell decoding                                  */
    /***************************************************/
    for (&sum_pulses, pulses_frame) in izip!(
        sum_pulses.iter(),
        pulses.chunks_exact_mut(SHELL_CODEC_FRAME_LENGTH)
    ) {
        if sum_pulses > 0 {
            silk_shell_decoder(pulses_frame, psRangeDec, sum_pulses);
        } else {
            pulses_frame.fill(0);
        }
    }

    /***************************************************/
    /* LSB Decoding                                    */
    /***************************************************/
    for (&nLshifts, sum_pulses, pulses_frame) in izip!(
        nLshifts.iter(),
        sum_pulses.iter_mut(),
        pulses.chunks_exact_mut(SHELL_CODEC_FRAME_LENGTH)
    ) {
        if nLshifts > 0 {
            for pulse in pulses_frame {
                let mut abs_q = *pulse as i32;

                for _ in 0..nLshifts {
                    abs_q = ((abs_q as u32) << 1) as i32;
                    abs_q += ec_dec_icdf(psRangeDec, &silk_lsb_iCDF, 8);
                }

                *pulse = abs_q as i16;
            }

            /* Mark the number of pulses non-zero for sign decoding. */
            *sum_pulses |= nLshifts << 5;
        }
    }

    /****************************************/
    /* Decode and add signs to pulse signal */
    /****************************************/
    silk_decode_signs(
        psRangeDec,
        pulses,
        signalType,
        quantOffsetType,
        &sum_pulses[..iter],
    );
}
