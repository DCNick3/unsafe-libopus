use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/structs_FLP.h:32"]
pub mod structs_FLP_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:9"]
    pub struct silk_shape_state_FLP {
        pub LastGainIndex: i8,
        pub HarmShapeGain_smth: libc::c_float,
        pub Tilt_smth: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:9"]
    pub struct silk_encoder_state_FLP {
        pub sCmn: silk_encoder_state,
        pub sShape: silk_shape_state_FLP,
        pub x_buf: [libc::c_float; 720],
        pub LTPCorr: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "64:9"]
    pub struct silk_encoder_control_FLP {
        pub Gains: [libc::c_float; 4],
        pub PredCoef: [[libc::c_float; 16]; 2],
        pub LTPCoef: [libc::c_float; 20],
        pub LTP_scale: libc::c_float,
        pub pitchL: [libc::c_int; 4],
        pub AR: [libc::c_float; 96],
        pub LF_MA_shp: [libc::c_float; 4],
        pub LF_AR_shp: [libc::c_float; 4],
        pub Tilt: [libc::c_float; 4],
        pub HarmShapeGain: [libc::c_float; 4],
        pub Lambda: libc::c_float,
        pub input_quality: libc::c_float,
        pub coding_quality: libc::c_float,
        pub predGain: libc::c_float,
        pub LTPredCodGain: libc::c_float,
        pub ResNrg: [libc::c_float; 4],
        pub GainsUnq_Q16: [i32; 4],
        pub lastGainIndexPrev: i8,
    }
    use crate::silk::structs::silk_encoder_state;
}
use crate::silk::define::CODE_INDEPENDENTLY;
use crate::silk::tables_other::silk_LTPScales_table_Q14;

pub use self::structs_FLP_h::{
    silk_encoder_control_FLP, silk_encoder_state_FLP, silk_shape_state_FLP,
};

#[no_mangle]
#[c2rust::src_loc = "34:1"]
pub unsafe extern "C" fn silk_LTP_scale_ctrl_FLP(
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
