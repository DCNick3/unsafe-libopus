use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:9"]
    pub struct silk_NLSF_CB_struct {
        pub nVectors: i16,
        pub order: i16,
        pub quantStepSize_Q16: i16,
        pub invQuantStepSize_Q6: i16,
        pub CB1_NLSF_Q8: *const u8,
        pub CB1_Wght_Q9: *const i16,
        pub CB1_iCDF: *const u8,
        pub pred_Q8: *const u8,
        pub ec_sel: *const u8,
        pub ec_iCDF: *const u8,
        pub ec_Rates_Q5: *const u8,
        pub deltaMin_Q15: *const i16,
    }
}
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint8_t;
pub use self::structs_h::silk_NLSF_CB_struct;
pub use self::types_h::{__int16_t, __int32_t, __uint8_t};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_NLSF_unpack(
    ec_ix: *mut i16,
    pred_Q8: *mut u8,
    psNLSF_CB: *const silk_NLSF_CB_struct,
    CB1_index: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut entry: u8 = 0;
    let mut ec_sel_ptr: *const u8 = 0 as *const u8;
    ec_sel_ptr = &*((*psNLSF_CB).ec_sel)
        .offset((CB1_index * (*psNLSF_CB).order as libc::c_int / 2 as libc::c_int) as isize)
        as *const u8;
    i = 0 as libc::c_int;
    while i < (*psNLSF_CB).order as libc::c_int {
        let fresh0 = ec_sel_ptr;
        ec_sel_ptr = ec_sel_ptr.offset(1);
        entry = *fresh0;
        *ec_ix.offset(i as isize) = ((entry as libc::c_int >> 1 as libc::c_int & 7 as libc::c_int)
            as i16 as i32
            * (2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as i16 as i32)
            as i16;
        *pred_Q8.offset(i as isize) = *((*psNLSF_CB).pred_Q8).offset(
            (i + (entry as libc::c_int & 1 as libc::c_int)
                * ((*psNLSF_CB).order as libc::c_int - 1 as libc::c_int)) as isize,
        );
        *ec_ix.offset((i + 1 as libc::c_int) as isize) =
            ((entry as libc::c_int >> 5 as libc::c_int & 7 as libc::c_int) as i16 as i32
                * (2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as i16 as i32)
                as i16;
        *pred_Q8.offset((i + 1 as libc::c_int) as isize) = *((*psNLSF_CB).pred_Q8).offset(
            (i + (entry as libc::c_int >> 4 as libc::c_int & 1 as libc::c_int)
                * ((*psNLSF_CB).order as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int) as isize,
        );
        i += 2 as libc::c_int;
    }
}
