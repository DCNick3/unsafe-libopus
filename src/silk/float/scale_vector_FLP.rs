use ::libc;
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_scale_vector_FLP(
    mut data1: *mut libc::c_float,
    mut gain: libc::c_float,
    mut dataSize: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut dataSize4: libc::c_int = 0;
    dataSize4 = dataSize & 0xfffc as libc::c_int;
    i = 0 as libc::c_int;
    while i < dataSize4 {
        *data1.offset((i + 0 as libc::c_int) as isize) *= gain;
        *data1.offset((i + 1 as libc::c_int) as isize) *= gain;
        *data1.offset((i + 2 as libc::c_int) as isize) *= gain;
        *data1.offset((i + 3 as libc::c_int) as isize) *= gain;
        i += 4 as libc::c_int;
    }
    while i < dataSize {
        *data1.offset(i as isize) *= gain;
        i += 1;
    }
}
