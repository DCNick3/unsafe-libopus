use crate::silk::define::CODE_INDEPENDENTLY;
use crate::silk::float::structs_FLP::{silk_encoder_control_FLP, silk_encoder_state_FLP};
use crate::silk::tables_other::silk_LTPScales_table_Q14;

#[c2rust::src_loc = "34:1"]
pub unsafe fn silk_LTP_scale_ctrl_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut psEncCtrl: *mut silk_encoder_control_FLP,
    condCoding: libc::c_int,
) {
    let mut round_loss: libc::c_int = 0;
    if condCoding == CODE_INDEPENDENTLY {
        round_loss = (*psEnc).sCmn.PacketLoss_perc + (*psEnc).sCmn.nFramesPerPacket;
        (*psEnc).sCmn.indices.LTP_scaleIndex = (if 0.0f32 > 2.0f32 {
            if round_loss as libc::c_float * (*psEncCtrl).LTPredCodGain * 0.1f32 > 0.0f32 {
                0.0f32
            } else if round_loss as libc::c_float * (*psEncCtrl).LTPredCodGain * 0.1f32 < 2.0f32 {
                2.0f32
            } else {
                round_loss as libc::c_float * (*psEncCtrl).LTPredCodGain * 0.1f32
            }
        } else if round_loss as libc::c_float * (*psEncCtrl).LTPredCodGain * 0.1f32 > 2.0f32 {
            2.0f32
        } else if round_loss as libc::c_float * (*psEncCtrl).LTPredCodGain * 0.1f32 < 0.0f32 {
            0.0f32
        } else {
            round_loss as libc::c_float * (*psEncCtrl).LTPredCodGain * 0.1f32
        }) as i8;
    } else {
        (*psEnc).sCmn.indices.LTP_scaleIndex = 0 as libc::c_int as i8;
    }
    (*psEncCtrl).LTP_scale = silk_LTPScales_table_Q14[(*psEnc).sCmn.indices.LTP_scaleIndex as usize]
        as libc::c_float
        / 16384.0f32;
}
