use ::libc;
#[c2rust::src_loc = "35:1"]
pub unsafe fn silk_k2a_FLP(A: *mut libc::c_float, rc: *const libc::c_float, order: i32) {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut rck: libc::c_float = 0.;
    let mut tmp1: libc::c_float = 0.;
    let mut tmp2: libc::c_float = 0.;
    k = 0 as libc::c_int;
    while k < order {
        rck = *rc.offset(k as isize);
        n = 0 as libc::c_int;
        while n < k + 1 as libc::c_int >> 1 as libc::c_int {
            tmp1 = *A.offset(n as isize);
            tmp2 = *A.offset((k - n - 1 as libc::c_int) as isize);
            *A.offset(n as isize) = tmp1 + tmp2 * rck;
            *A.offset((k - n - 1 as libc::c_int) as isize) = tmp2 + tmp1 * rck;
            n += 1;
        }
        *A.offset(k as isize) = -rck;
        k += 1;
    }
}
