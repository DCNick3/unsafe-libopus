use ::libc;
#[no_mangle]
#[c2rust::src_loc = "34:1"]
pub unsafe extern "C" fn silk_LTP_analysis_filter_FLP(
    mut LTP_res: *mut libc::c_float,
    mut x: *const libc::c_float,
    mut B: *const libc::c_float,
    mut pitchL: *const libc::c_int,
    mut invGains: *const libc::c_float,
    subfr_length: libc::c_int,
    nb_subfr: libc::c_int,
    pre_length: libc::c_int,
) {
    let mut x_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut x_lag_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut Btmp: [libc::c_float; 5] = [0.; 5];
    let mut LTP_res_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut inv_gain: libc::c_float = 0.;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    x_ptr = x;
    LTP_res_ptr = LTP_res;
    k = 0 as libc::c_int;
    while k < nb_subfr {
        x_lag_ptr = x_ptr.offset(-(*pitchL.offset(k as isize) as isize));
        inv_gain = *invGains.offset(k as isize);
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            Btmp[i as usize] = *B.offset((k * 5 as libc::c_int + i) as isize);
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < subfr_length + pre_length {
            *LTP_res_ptr.offset(i as isize) = *x_ptr.offset(i as isize);
            j = 0 as libc::c_int;
            while j < 5 as libc::c_int {
                *LTP_res_ptr.offset(i as isize) -= Btmp[j as usize]
                    * *x_lag_ptr.offset((5 as libc::c_int / 2 as libc::c_int - j) as isize);
                j += 1;
            }
            *LTP_res_ptr.offset(i as isize) *= inv_gain;
            x_lag_ptr = x_lag_ptr.offset(1);
            i += 1;
        }
        LTP_res_ptr = LTP_res_ptr.offset((subfr_length + pre_length) as isize);
        x_ptr = x_ptr.offset(subfr_length as isize);
        k += 1;
    }
}
