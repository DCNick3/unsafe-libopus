use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:32"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = u32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: u32,
        pub end_offs: u32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: u32,
        pub rng: u32,
        pub val: u32,
        pub ext: u32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "47:1"]
    pub type ec_enc = ec_ctx;
}

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.h:32"]
pub mod entenc_h {
    use super::entcode_h::ec_enc;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn ec_enc_icdf(
            _this: *mut ec_enc,
            _s: libc::c_int,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "68:26"]
        pub static silk_uniform3_iCDF: [u8; 3];
        #[c2rust::src_loc = "70:26"]
        pub static silk_uniform5_iCDF: [u8; 5];
        #[c2rust::src_loc = "90:26"]
        pub static silk_stereo_pred_joint_iCDF: [u8; 25];
        #[c2rust::src_loc = "91:26"]
        pub static silk_stereo_only_code_mid_iCDF: [u8; 2];
    }
}
pub use self::entcode_h::{ec_ctx, ec_enc, ec_window};
use self::entenc_h::ec_enc_icdf;
use crate::celt::celt::celt_fatal;

use self::tables_h::{
    silk_stereo_only_code_mid_iCDF, silk_stereo_pred_joint_iCDF, silk_uniform3_iCDF,
    silk_uniform5_iCDF,
};

#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_stereo_encode_pred(psRangeEnc: *mut ec_enc, ix: *mut [i8; 3]) {
    let mut n: libc::c_int = 0;
    n = 5 as libc::c_int
        * (*ix.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] as libc::c_int
        + (*ix.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] as libc::c_int;
    if !(n < 25 as libc::c_int) {
        celt_fatal(
            b"assertion failed: n < 25\0" as *const u8 as *const libc::c_char,
            b"silk/stereo_encode_pred.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
        );
    }
    ec_enc_icdf(
        psRangeEnc,
        n,
        silk_stereo_pred_joint_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
    n = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        if !(((*ix.offset(n as isize))[0 as libc::c_int as usize] as libc::c_int)
            < 3 as libc::c_int)
        {
            celt_fatal(
                b"assertion failed: ix[ n ][ 0 ] < 3\0" as *const u8 as *const libc::c_char,
                b"silk/stereo_encode_pred.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int,
            );
        }
        if !(((*ix.offset(n as isize))[1 as libc::c_int as usize] as libc::c_int)
            < 5 as libc::c_int)
        {
            celt_fatal(
                b"assertion failed: ix[ n ][ 1 ] < STEREO_QUANT_SUB_STEPS\0" as *const u8
                    as *const libc::c_char,
                b"silk/stereo_encode_pred.c\0" as *const u8 as *const libc::c_char,
                48 as libc::c_int,
            );
        }
        ec_enc_icdf(
            psRangeEnc,
            (*ix.offset(n as isize))[0 as libc::c_int as usize] as libc::c_int,
            silk_uniform3_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
        ec_enc_icdf(
            psRangeEnc,
            (*ix.offset(n as isize))[1 as libc::c_int as usize] as libc::c_int,
            silk_uniform5_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
        n += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn silk_stereo_encode_mid_only(psRangeEnc: *mut ec_enc, mid_only_flag: i8) {
    ec_enc_icdf(
        psRangeEnc,
        mid_only_flag as libc::c_int,
        silk_stereo_only_code_mid_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
}
