use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:32"]
pub mod SigProc_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "112:1"]
        pub fn silk_scale_vector_FLP(
            data1: *mut libc::c_float,
            gain: libc::c_float,
            dataSize: libc::c_int,
        );
        #[c2rust::src_loc = "134:1"]
        pub fn silk_energy_FLP(data: *const libc::c_float, dataSize: libc::c_int)
            -> libc::c_double;
    }
}
use self::SigProc_FLP_h::{silk_energy_FLP, silk_scale_vector_FLP};
use crate::silk::define::LTP_ORDER;
use crate::silk::float::corrMatrix_FLP::{silk_corrMatrix_FLP, silk_corrVector_FLP};

#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_find_LTP_FLP(
    XX: *mut libc::c_float,
    xX: *mut libc::c_float,
    mut r_ptr: *const libc::c_float,
    lag: *const libc::c_int,
    subfr_length: libc::c_int,
    nb_subfr: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut xX_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut XX_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut lag_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut xx: libc::c_float = 0.;
    let mut temp: libc::c_float = 0.;
    xX_ptr = xX;
    XX_ptr = XX;
    k = 0 as libc::c_int;
    while k < nb_subfr {
        lag_ptr =
            r_ptr.offset(-((*lag.offset(k as isize) + LTP_ORDER / 2 as libc::c_int) as isize));
        silk_corrMatrix_FLP(lag_ptr, subfr_length, LTP_ORDER, XX_ptr);
        silk_corrVector_FLP(lag_ptr, r_ptr, subfr_length, LTP_ORDER, xX_ptr);
        xx = silk_energy_FLP(r_ptr, subfr_length + LTP_ORDER) as libc::c_float;
        temp = 1.0f32
            / (if xx
                > 0.03f32
                    * 0.5f32
                    * (*XX_ptr.offset(0 as libc::c_int as isize)
                        + *XX_ptr.offset(24 as libc::c_int as isize))
                    + 1.0f32
            {
                xx
            } else {
                0.03f32
                    * 0.5f32
                    * (*XX_ptr.offset(0 as libc::c_int as isize)
                        + *XX_ptr.offset(24 as libc::c_int as isize))
                    + 1.0f32
            });
        silk_scale_vector_FLP(XX_ptr, temp, LTP_ORDER * LTP_ORDER);
        silk_scale_vector_FLP(xX_ptr, temp, LTP_ORDER);
        r_ptr = r_ptr.offset(subfr_length as isize);
        XX_ptr = XX_ptr.offset((LTP_ORDER * LTP_ORDER) as isize);
        xX_ptr = xX_ptr.offset(LTP_ORDER as isize);
        k += 1;
    }
}
