#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cpu_support.h:32"]
pub mod cpu_support_h {
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe fn opus_select_arch() -> libc::c_int {
        return 0 as libc::c_int;
    }
}

use self::cpu_support_h::opus_select_arch;
use crate::externs::memset;
use crate::silk::structs::silk_decoder_state;
use crate::silk::CNG::silk_CNG_Reset;
use crate::silk::PLC::silk_PLC_Reset;

#[c2rust::src_loc = "37:1"]
pub unsafe fn silk_init_decoder(mut psDec: *mut silk_decoder_state) -> libc::c_int {
    memset(
        psDec as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<silk_decoder_state>() as libc::c_ulong,
    );
    (*psDec).first_frame_after_reset = 1 as libc::c_int;
    (*psDec).prev_gain_Q16 = 65536 as libc::c_int;
    (*psDec).arch = opus_select_arch();
    silk_CNG_Reset(psDec);
    silk_PLC_Reset(psDec);
    return 0 as libc::c_int;
}
