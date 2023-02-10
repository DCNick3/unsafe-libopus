use ::libc;
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_bwexpander_FLP(
    ar: *mut libc::c_float,
    d: libc::c_int,
    chirp: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut cfac: libc::c_float = chirp;
    i = 0 as libc::c_int;
    while i < d - 1 as libc::c_int {
        *ar.offset(i as isize) *= cfac;
        cfac *= chirp;
        i += 1;
    }
    *ar.offset((d - 1 as libc::c_int) as isize) *= cfac;
}
