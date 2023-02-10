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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/main_FLP.h:32"]
pub mod main_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "220:1"]
        pub fn silk_corrMatrix_FLP(
            x: *const libc::c_float,
            L: libc::c_int,
            Order: libc::c_int,
            XX: *mut libc::c_float,
        );
        #[c2rust::src_loc = "228:1"]
        pub fn silk_corrVector_FLP(
            x: *const libc::c_float,
            t: *const libc::c_float,
            L: libc::c_int,
            Order: libc::c_int,
            Xt: *mut libc::c_float,
        );
    }
}
use self::main_FLP_h::{silk_corrMatrix_FLP, silk_corrVector_FLP};
use self::SigProc_FLP_h::{silk_energy_FLP, silk_scale_vector_FLP};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_find_LTP_FLP(
    mut XX: *mut libc::c_float,
    mut xX: *mut libc::c_float,
    mut r_ptr: *const libc::c_float,
    mut lag: *const libc::c_int,
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
        lag_ptr = r_ptr
            .offset(-((*lag.offset(k as isize) + 5 as libc::c_int / 2 as libc::c_int) as isize));
        silk_corrMatrix_FLP(lag_ptr, subfr_length, 5 as libc::c_int, XX_ptr);
        silk_corrVector_FLP(lag_ptr, r_ptr, subfr_length, 5 as libc::c_int, xX_ptr);
        xx = silk_energy_FLP(r_ptr, subfr_length + 5 as libc::c_int) as libc::c_float;
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
        silk_scale_vector_FLP(XX_ptr, temp, 5 as libc::c_int * 5 as libc::c_int);
        silk_scale_vector_FLP(xX_ptr, temp, 5 as libc::c_int);
        r_ptr = r_ptr.offset(subfr_length as isize);
        XX_ptr = XX_ptr.offset((5 as libc::c_int * 5 as libc::c_int) as isize);
        xX_ptr = xX_ptr.offset(5 as libc::c_int as isize);
        k += 1;
    }
}
