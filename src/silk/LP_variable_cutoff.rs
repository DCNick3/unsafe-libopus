use crate::silk::biquad_alt::silk_biquad_alt_stride1;
use crate::silk::define::{TRANSITION_FRAMES, TRANSITION_INT_NUM, TRANSITION_NA, TRANSITION_NB};
use crate::silk::macros::silk_SMLAWB;
use crate::silk::structs::silk_LP_state;
use crate::silk::tables_other::{silk_Transition_LP_A_Q28, silk_Transition_LP_B_Q28};
use crate::silk::SigProc_FIX::silk_SAT16;

/*
    Elliptic/Cauer filters designed with 0.1 dB passband ripple,
    80 dB minimum stopband attenuation, and
    [0.95 : 0.15 : 0.35] normalized cut off frequencies.
*/

/// Helper function, interpolates the filter taps
#[inline]
fn silk_LP_interpolate_filter_taps(
    B_Q28: &mut [i32; 3],
    A_Q28: &mut [i32; 2],
    ind: usize,
    fac_Q16: i32,
) {
    if ind < TRANSITION_INT_NUM - 1 {
        if fac_Q16 > 0 {
            if fac_Q16 < 32768 {
                /* fac_Q16 is in range of a 16-bit int */

                /* Piece-wise linear interpolation of B and A */
                for nb in 0..TRANSITION_NB {
                    B_Q28[nb] = silk_SMLAWB(
                        silk_Transition_LP_B_Q28[ind][nb],
                        silk_Transition_LP_B_Q28[ind + 1][nb] - silk_Transition_LP_B_Q28[ind][nb],
                        fac_Q16,
                    );
                }
                for na in 0..TRANSITION_NA {
                    A_Q28[na] = silk_SMLAWB(
                        silk_Transition_LP_A_Q28[ind][na],
                        silk_Transition_LP_A_Q28[ind + 1][na] - silk_Transition_LP_A_Q28[ind][na],
                        fac_Q16,
                    );
                }
            } else {
                /* (fac_Q16 - (1 << 16)) is in range of a 16-bit int */
                assert_eq!(fac_Q16 - (1 << 16), silk_SAT16(fac_Q16 - (1 << 16)));
                /* Piece-wise linear interpolation of B and A */

                for nb in 0..TRANSITION_NB {
                    B_Q28[nb] = silk_SMLAWB(
                        silk_Transition_LP_B_Q28[ind + 1][nb],
                        silk_Transition_LP_B_Q28[ind + 1][nb] - silk_Transition_LP_B_Q28[ind][nb],
                        fac_Q16 - (1i32 << 16),
                    );
                }
                for na in 0..TRANSITION_NA {
                    A_Q28[na] = silk_SMLAWB(
                        silk_Transition_LP_A_Q28[ind + 1][na],
                        silk_Transition_LP_A_Q28[ind + 1][na] - silk_Transition_LP_A_Q28[ind][na],
                        fac_Q16 - (1i32 << 16),
                    );
                }
            }
        } else {
            *B_Q28 = silk_Transition_LP_B_Q28[ind];
            *A_Q28 = silk_Transition_LP_A_Q28[ind];
        }
    } else {
        *B_Q28 = silk_Transition_LP_B_Q28[4];
        *A_Q28 = silk_Transition_LP_A_Q28[4];
    };
}

/// Low-pass filter with variable cutoff frequency based on
/// piece-wise linear interpolation between elliptic filters
///
/// Start by setting psEncC->mode <> 0;
/// Deactivate by setting psEncC->mode = 0;
///
/// ```text
/// psLP         I/O  LP filter state
/// frame        I/O  Low-pass filtered output signal
/// frame_length I    Frame length
/// ```
pub fn silk_LP_variable_cutoff(psLP: &mut silk_LP_state, frame: &mut [i16]) {
    assert!(psLP.transition_frame_no >= 0 && psLP.transition_frame_no <= TRANSITION_FRAMES as i32);

    /* Run filter if needed */
    if psLP.mode != 0 {
        /* Calculate index and interpolation factor for interpolation */
        let fac_Q16 = (TRANSITION_FRAMES as i32 - psLP.transition_frame_no) << (16 - 6);
        let ind = fac_Q16 >> 16;
        let fac_Q16 = fac_Q16 - (ind << 16);

        let mut B_Q28: [i32; TRANSITION_NB] = [0; TRANSITION_NB];
        let mut A_Q28: [i32; TRANSITION_NA] = [0; TRANSITION_NA];

        silk_LP_interpolate_filter_taps(&mut B_Q28, &mut A_Q28, ind as usize, fac_Q16);
        psLP.transition_frame_no = if 0 > 5120 / (5 * 4) {
            if psLP.transition_frame_no + psLP.mode > 0 {
                0
            } else if psLP.transition_frame_no + psLP.mode < 5120 / (5 * 4) {
                5120 / (5 * 4)
            } else {
                psLP.transition_frame_no + psLP.mode
            }
        } else if psLP.transition_frame_no + psLP.mode > 5120 / (5 * 4) {
            5120 / (5 * 4)
        } else if psLP.transition_frame_no + psLP.mode < 0 {
            0
        } else {
            psLP.transition_frame_no + psLP.mode
        };
        silk_biquad_alt_stride1(&B_Q28, &A_Q28, &mut psLP.In_LP_State, frame);
    }
}
