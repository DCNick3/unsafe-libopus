use crate::celt::entdec::{ec_dec, ec_dec_icdf};
use crate::silk::tables_other::{
    silk_stereo_only_code_mid_iCDF, silk_stereo_pred_joint_iCDF, silk_stereo_pred_quant_Q13,
    silk_uniform3_iCDF, silk_uniform5_iCDF,
};
use ::libc;

#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_stereo_decode_pred(psRangeDec: *mut ec_dec, pred_Q13: *mut i32) {
    let mut n: libc::c_int = 0;
    let mut ix: [[libc::c_int; 3]; 2] = [[0; 3]; 2];
    let mut low_Q13: i32 = 0;
    let mut step_Q13: i32 = 0;
    n = ec_dec_icdf(
        psRangeDec,
        silk_stereo_pred_joint_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
    ix[0 as libc::c_int as usize][2 as libc::c_int as usize] = n / 5 as libc::c_int;
    ix[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        n - 5 as libc::c_int * ix[0 as libc::c_int as usize][2 as libc::c_int as usize];
    n = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        ix[n as usize][0 as libc::c_int as usize] = ec_dec_icdf(
            psRangeDec,
            silk_uniform3_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
        ix[n as usize][1 as libc::c_int as usize] = ec_dec_icdf(
            psRangeDec,
            silk_uniform5_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
        n += 1;
    }
    n = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        ix[n as usize][0 as libc::c_int as usize] +=
            3 as libc::c_int * ix[n as usize][2 as libc::c_int as usize];
        low_Q13 =
            silk_stereo_pred_quant_Q13[ix[n as usize][0 as libc::c_int as usize] as usize] as i32;
        step_Q13 = ((silk_stereo_pred_quant_Q13
            [(ix[n as usize][0 as libc::c_int as usize] + 1 as libc::c_int) as usize]
            as libc::c_int
            - low_Q13) as libc::c_long
            * (0.5f64 / 5 as libc::c_int as libc::c_double
                * ((1 as libc::c_int as i64) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as i32 as i16 as i64
            >> 16 as libc::c_int) as i32;
        *pred_Q13.offset(n as isize) = low_Q13
            + step_Q13 as i16 as i32
                * (2 as libc::c_int * ix[n as usize][1 as libc::c_int as usize] + 1 as libc::c_int)
                    as i16 as i32;
        n += 1;
    }
    let ref mut fresh0 = *pred_Q13.offset(0 as libc::c_int as isize);
    *fresh0 -= *pred_Q13.offset(1 as libc::c_int as isize);
}
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn silk_stereo_decode_mid_only(
    psRangeDec: *mut ec_dec,
    decode_only_mid: *mut libc::c_int,
) {
    *decode_only_mid = ec_dec_icdf(
        psRangeDec,
        silk_stereo_only_code_mid_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
}
