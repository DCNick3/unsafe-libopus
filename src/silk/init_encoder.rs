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
    use crate::silk::structs::silk_encoder_state;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:34"]
pub mod SigProc_FIX_h {
    extern "C" {
        #[c2rust::src_loc = "176:1"]
        pub fn silk_lin2log(inLin: i32) -> i32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:34"]
pub mod main_h {
    use crate::silk::structs::silk_VAD_state;
    extern "C" {
        #[c2rust::src_loc = "304:1"]
        pub fn silk_VAD_Init(psSilk_VAD: *mut silk_VAD_state) -> libc::c_int;
    }
}
use self::main_h::silk_VAD_Init;

pub use self::structs_FLP_h::{silk_encoder_state_FLP, silk_shape_state_FLP};

use self::SigProc_FIX_h::silk_lin2log;
use crate::externs::memset;
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn silk_init_encoder(
    mut psEnc: *mut silk_encoder_state_FLP,
    arch: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    memset(
        psEnc as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<silk_encoder_state_FLP>() as libc::c_ulong,
    );
    (*psEnc).sCmn.arch = arch;
    (*psEnc).sCmn.variable_HP_smth1_Q15 = (((silk_lin2log(
        ((60 as libc::c_int as libc::c_long * ((1 as libc::c_int as i64) << 16 as libc::c_int))
            as libc::c_double
            + 0.5f64) as i32,
    ) - ((16 as libc::c_int) << 7 as libc::c_int))
        as u32)
        << 8 as libc::c_int) as i32;
    (*psEnc).sCmn.variable_HP_smth2_Q15 = (*psEnc).sCmn.variable_HP_smth1_Q15;
    (*psEnc).sCmn.first_frame_after_reset = 1 as libc::c_int;
    ret += silk_VAD_Init(&mut (*psEnc).sCmn.sVAD);
    return ret;
}
