use crate::silk::lin2log::silk_lin2log;
use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/structs_FLP.h:34"]
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
    use super::silk_encoder_state;
}
pub use self::structs_FLP_h::{silk_encoder_state_FLP, silk_shape_state_FLP};
use crate::silk::define::TYPE_VOICED;
use crate::silk::structs::silk_encoder_state;

#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_HP_variable_cutoff(state_Fxx: *mut silk_encoder_state_FLP) {
    let mut quality_Q15: libc::c_int = 0;
    let mut pitch_freq_Hz_Q16: i32 = 0;
    let mut pitch_freq_log_Q7: i32 = 0;
    let mut delta_freq_Q7: i32 = 0;
    let mut psEncC1: *mut silk_encoder_state =
        &mut (*state_Fxx.offset(0 as libc::c_int as isize)).sCmn;
    if (*psEncC1).prevSignalType as libc::c_int == TYPE_VOICED {
        pitch_freq_Hz_Q16 = ((((*psEncC1).fs_kHz * 1000 as libc::c_int) as u32)
            << 16 as libc::c_int) as i32
            / (*psEncC1).prevLag;
        pitch_freq_log_Q7 =
            silk_lin2log(pitch_freq_Hz_Q16) - ((16 as libc::c_int) << 7 as libc::c_int);
        quality_Q15 = (*psEncC1).input_quality_bands_Q15[0 as libc::c_int as usize];
        pitch_freq_log_Q7 = (pitch_freq_log_Q7 as libc::c_long
            + ((((-quality_Q15 as u32) << 2 as libc::c_int) as i32 as libc::c_long
                * quality_Q15 as i16 as i64
                >> 16 as libc::c_int) as i32 as libc::c_long
                * (pitch_freq_log_Q7
                    - (silk_lin2log(
                        ((60 as libc::c_int as libc::c_long
                            * ((1 as libc::c_int as i64) << 16 as libc::c_int))
                            as libc::c_double
                            + 0.5f64) as i32,
                    ) - ((16 as libc::c_int) << 7 as libc::c_int))) as i16
                    as i64
                >> 16 as libc::c_int)) as i32;
        delta_freq_Q7 = pitch_freq_log_Q7 - ((*psEncC1).variable_HP_smth1_Q15 >> 8 as libc::c_int);
        if delta_freq_Q7 < 0 as libc::c_int {
            delta_freq_Q7 = delta_freq_Q7 * 3 as libc::c_int;
        }
        delta_freq_Q7 = if -(((0.4f32
            * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
            as libc::c_double
            + 0.5f64) as i32)
            > ((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32
        {
            if delta_freq_Q7
                > -(((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as i32)
            {
                -(((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as i32)
            } else if delta_freq_Q7
                < ((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as i32
            {
                ((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as i32
            } else {
                delta_freq_Q7
            }
        } else if delta_freq_Q7
            > ((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32
        {
            ((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32
        } else if delta_freq_Q7
            < -(((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32)
        {
            -(((0.4f32 * ((1 as libc::c_int as i64) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32)
        } else {
            delta_freq_Q7
        };
        (*psEncC1).variable_HP_smth1_Q15 = ((*psEncC1).variable_HP_smth1_Q15 as libc::c_long
            + (((*psEncC1).speech_activity_Q8 as i16 as i32 * delta_freq_Q7 as i16 as i32)
                as libc::c_long
                * ((0.1f32 * ((1 as libc::c_int as i64) << 16 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as i32 as i16 as i64
                >> 16 as libc::c_int)) as i32;
        (*psEncC1).variable_HP_smth1_Q15 = if ((silk_lin2log(60 as libc::c_int) as u32)
            << 8 as libc::c_int) as i32
            > ((silk_lin2log(100 as libc::c_int) as u32) << 8 as libc::c_int) as i32
        {
            if (*psEncC1).variable_HP_smth1_Q15
                > ((silk_lin2log(60 as libc::c_int) as u32) << 8 as libc::c_int) as i32
            {
                ((silk_lin2log(60 as libc::c_int) as u32) << 8 as libc::c_int) as i32
            } else if (*psEncC1).variable_HP_smth1_Q15
                < ((silk_lin2log(100 as libc::c_int) as u32) << 8 as libc::c_int) as i32
            {
                ((silk_lin2log(100 as libc::c_int) as u32) << 8 as libc::c_int) as i32
            } else {
                (*psEncC1).variable_HP_smth1_Q15
            }
        } else if (*psEncC1).variable_HP_smth1_Q15
            > ((silk_lin2log(100 as libc::c_int) as u32) << 8 as libc::c_int) as i32
        {
            ((silk_lin2log(100 as libc::c_int) as u32) << 8 as libc::c_int) as i32
        } else if (*psEncC1).variable_HP_smth1_Q15
            < ((silk_lin2log(60 as libc::c_int) as u32) << 8 as libc::c_int) as i32
        {
            ((silk_lin2log(60 as libc::c_int) as u32) << 8 as libc::c_int) as i32
        } else {
            (*psEncC1).variable_HP_smth1_Q15
        };
    }
}
