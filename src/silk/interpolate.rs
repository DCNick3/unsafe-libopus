use crate::celt::celt::celt_fatal;

pub unsafe fn silk_interpolate(
    xi: *mut i16,
    x0: *const i16,
    x1: *const i16,
    ifact_Q2: i32,
    d: i32,
) {
    let mut i: i32 = 0;
    if !(ifact_Q2 >= 0 as i32) {
        celt_fatal(
            b"assertion failed: ifact_Q2 >= 0\0" as *const u8 as *const i8,
            b"silk/interpolate.c\0" as *const u8 as *const i8,
            45 as i32,
        );
    }
    if !(ifact_Q2 <= 4 as i32) {
        celt_fatal(
            b"assertion failed: ifact_Q2 <= 4\0" as *const u8 as *const i8,
            b"silk/interpolate.c\0" as *const u8 as *const i8,
            46 as i32,
        );
    }
    i = 0 as i32;
    while i < d {
        *xi.offset(i as isize) = (*x0.offset(i as isize) as i32
            + ((*x1.offset(i as isize) as i32 - *x0.offset(i as isize) as i32) as i16 as i32
                * ifact_Q2 as i16 as i32
                >> 2 as i32)) as i16;
        i += 1;
    }
}
