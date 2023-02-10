use ::libc;
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_inner_product_FLP(
    mut data1: *const libc::c_float,
    mut data2: *const libc::c_float,
    mut dataSize: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut result: libc::c_double = 0.;
    result = 0.0f64;
    i = 0 as libc::c_int;
    while i < dataSize - 3 as libc::c_int {
        result
            += *data1.offset((i + 0 as libc::c_int) as isize) as libc::c_double
                * *data2.offset((i + 0 as libc::c_int) as isize) as libc::c_double
                + *data1.offset((i + 1 as libc::c_int) as isize) as libc::c_double
                    * *data2.offset((i + 1 as libc::c_int) as isize) as libc::c_double
                + *data1.offset((i + 2 as libc::c_int) as isize) as libc::c_double
                    * *data2.offset((i + 2 as libc::c_int) as isize) as libc::c_double
                + *data1.offset((i + 3 as libc::c_int) as isize) as libc::c_double
                    * *data2.offset((i + 3 as libc::c_int) as isize) as libc::c_double;
        i += 4 as libc::c_int;
    }
    while i < dataSize {
        result
            += *data1.offset(i as isize) as libc::c_double
                * *data2.offset(i as isize) as libc::c_double;
        i += 1;
    }
    return result;
}
