use crate::silk::pitch_est_tables::{
    silk_CB_lags_stage2, silk_CB_lags_stage2_10_ms, silk_CB_lags_stage3, silk_CB_lags_stage3_10_ms,
    PE_MAX_LAG_MS, PE_MAX_NB_SUBFR, PE_MAX_NB_SUBFR_OVER_2, PE_MIN_LAG_MS, PE_NB_CBKS_STAGE2_10MS,
    PE_NB_CBKS_STAGE2_EXT, PE_NB_CBKS_STAGE3_10MS, PE_NB_CBKS_STAGE3_MAX,
};
use crate::silk::SigProc_FIX::silk_LIMIT;
use ndarray::{azip, ArrayView2};

/// Pitch analyzer function
///
/// ```text
/// lagIndex       I
/// contourIndex   O
/// pitch_lags[]   O   4 pitch values
/// Fs_kHz         I   sampling frequency (kHz)
/// nb_subfr       I   number of sub frames
/// ```
pub fn silk_decode_pitch(lagIndex: i16, contourIndex: i8, pitch_lags: &mut [i32], Fs_kHz: i32) {
    let nb_subfr = pitch_lags.len();

    let Lag_CB_ptr = match (Fs_kHz, nb_subfr) {
        (8, PE_MAX_NB_SUBFR) => ArrayView2::from_shape(
            (PE_MAX_NB_SUBFR, PE_NB_CBKS_STAGE2_EXT),
            &silk_CB_lags_stage2,
        ),
        (8, PE_MAX_NB_SUBFR_OVER_2) => ArrayView2::from_shape(
            (PE_MAX_NB_SUBFR / 2, PE_NB_CBKS_STAGE2_10MS),
            &silk_CB_lags_stage2_10_ms,
        ),
        (12 | 16, PE_MAX_NB_SUBFR) => ArrayView2::from_shape(
            (PE_MAX_NB_SUBFR, PE_NB_CBKS_STAGE3_MAX),
            &silk_CB_lags_stage3,
        ),
        (12 | 16, PE_MAX_NB_SUBFR_OVER_2) => ArrayView2::from_shape(
            (PE_MAX_NB_SUBFR / 2, PE_NB_CBKS_STAGE3_10MS),
            &silk_CB_lags_stage3_10_ms,
        ),
        (Fs_kHz, nb_subfr) => {
            unreachable!("Fs_kHz: {}, nb_subfr: {}", Fs_kHz, nb_subfr)
        }
    }
    .unwrap();

    let min_lag = PE_MIN_LAG_MS * Fs_kHz as i16 as i32;
    let max_lag = PE_MAX_LAG_MS * Fs_kHz as i16 as i32;
    let lag = min_lag + lagIndex as i32;

    azip!((out_lag in pitch_lags, lag_cb in Lag_CB_ptr.rows()) {
        let lag = lag + lag_cb[contourIndex as usize] as i32;
        *out_lag = silk_LIMIT(lag, min_lag, max_lag);
    });
}
