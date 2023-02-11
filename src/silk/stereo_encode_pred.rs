use ::libc;

use crate::celt::celt::celt_fatal;
use crate::celt::entenc::{ec_enc, ec_enc_icdf};
use crate::silk::tables_other::{
    silk_stereo_only_code_mid_iCDF, silk_stereo_pred_joint_iCDF, silk_uniform3_iCDF,
    silk_uniform5_iCDF,
};

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
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn silk_stereo_encode_mid_only(psRangeEnc: *mut ec_enc, mid_only_flag: i8) {
    ec_enc_icdf(
        psRangeEnc,
        mid_only_flag as libc::c_int,
        silk_stereo_only_code_mid_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
}
