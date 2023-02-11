use ::libc;

use crate::externs::memset;
use crate::silk::float::structs_FLP::silk_encoder_state_FLP;
use crate::silk::lin2log::silk_lin2log;
use crate::silk::VAD::silk_VAD_Init;

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
