use crate::silk::resampler::ResamplerState;
use crate::silk::structs::{silk_CNG_struct, silk_PLC_struct, silk_decoder_state, SideInfoIndices};
use crate::silk::tables_NLSF_CB_WB::silk_NLSF_CB_WB;
use crate::silk::CNG::silk_CNG_Reset;
use crate::silk::PLC::silk_PLC_Reset;

pub fn silk_init_decoder() -> silk_decoder_state {
    let mut dec = silk_decoder_state {
        prev_gain_Q16: 65536,
        exc_Q14: [0; 320],
        sLPC_Q14_buf: [0; 16],
        outBuf: [0; 480],
        lagPrev: 0,
        LastGainIndex: 0,
        fs_kHz: 0,
        fs_API_hz: 0,
        nb_subfr: 0,
        frame_length: 0,
        subfr_length: 0,
        ltp_mem_length: 0,
        LPC_order: 0,
        prevNLSF_Q15: [0; 16],
        first_frame_after_reset: 1,
        pitch_lag_low_bits_iCDF: &[],
        pitch_contour_iCDF: &[],
        nFramesDecoded: 0,
        nFramesPerPacket: 0,
        ec_prevSignalType: 0,
        ec_prevLagIndex: 0,
        VAD_flags: [0; 3],
        LBRR_flag: 0,
        LBRR_flags: [0; 3],
        resampler_state: ResamplerState::default(),
        psNLSF_CB: &silk_NLSF_CB_WB,
        indices: SideInfoIndices::default(),
        sCNG: silk_CNG_struct::default(),
        lossCnt: 0,
        prevSignalType: 0,
        arch: 0,
        sPLC: silk_PLC_struct::default(),
    };

    silk_CNG_Reset(&mut dec);
    silk_PLC_Reset(&mut dec);

    dec
}
