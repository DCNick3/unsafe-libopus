use ::libc;

use crate::celt::celt::celt_fatal;

#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_interpolate(
    xi: *mut i16,
    x0: *const i16,
    x1: *const i16,
    ifact_Q2: libc::c_int,
    d: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if !(ifact_Q2 >= 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: ifact_Q2 >= 0\0" as *const u8 as *const libc::c_char,
            b"silk/interpolate.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
        );
    }
    if !(ifact_Q2 <= 4 as libc::c_int) {
        celt_fatal(
            b"assertion failed: ifact_Q2 <= 4\0" as *const u8 as *const libc::c_char,
            b"silk/interpolate.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < d {
        *xi.offset(i as isize) = (*x0.offset(i as isize) as libc::c_int
            + ((*x1.offset(i as isize) as libc::c_int - *x0.offset(i as isize) as libc::c_int)
                as i16 as i32
                * ifact_Q2 as i16 as i32
                >> 2 as libc::c_int)) as i16;
        i += 1;
    }
}
