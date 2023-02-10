use ::libc;
#[no_mangle]
#[c2rust::src_loc = "34:1"]
pub unsafe extern "C" fn silk_inner_prod_aligned_scale(
    inVec1: *const i16,
    inVec2: *const i16,
    scale: libc::c_int,
    len: libc::c_int,
) -> i32 {
    let mut i: libc::c_int = 0;
    let mut sum: i32 = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        sum =
            sum + (*inVec1.offset(i as isize) as i32 * *inVec2.offset(i as isize) as i32 >> scale);
        i += 1;
    }
    return sum;
}
