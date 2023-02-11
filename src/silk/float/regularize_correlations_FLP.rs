use ::libc;
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_regularize_correlations_FLP(
    XX: *mut libc::c_float,
    xx: *mut libc::c_float,
    noise: libc::c_float,
    D: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < D {
        *(&mut *XX.offset(0 as libc::c_int as isize) as *mut libc::c_float)
            .offset((i * D + i) as isize) += noise;
        i += 1;
    }
    *xx.offset(0 as libc::c_int as isize) += noise;
}
