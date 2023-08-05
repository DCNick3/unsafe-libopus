pub unsafe fn silk_interpolate(
    xi: *mut i16,
    x0: *const i16,
    x1: *const i16,
    ifact_Q2: i32,
    d: i32,
) {
    let mut i: i32 = 0;
    assert!(ifact_Q2 >= 0);
    assert!(ifact_Q2 <= 4);
    i = 0;
    while i < d {
        *xi.offset(i as isize) = (*x0.offset(i as isize) as i32
            + ((*x1.offset(i as isize) as i32 - *x0.offset(i as isize) as i32) as i16 as i32
                * ifact_Q2 as i16 as i32
                >> 2)) as i16;
        i += 1;
    }
}
