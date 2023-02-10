use ::libc;
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_insertion_sort_decreasing_FLP(
    mut a: *mut libc::c_float,
    mut idx: *mut libc::c_int,
    L: libc::c_int,
    K: libc::c_int,
) {
    let mut value: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < K {
        *idx.offset(i as isize) = i;
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < K {
        value = *a.offset(i as isize);
        j = i - 1 as libc::c_int;
        while j >= 0 as libc::c_int && value > *a.offset(j as isize) {
            *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
            *idx.offset((j + 1 as libc::c_int) as isize) = *idx.offset(j as isize);
            j -= 1;
        }
        *a.offset((j + 1 as libc::c_int) as isize) = value;
        *idx.offset((j + 1 as libc::c_int) as isize) = i;
        i += 1;
    }
    i = K;
    while i < L {
        value = *a.offset(i as isize);
        if value > *a.offset((K - 1 as libc::c_int) as isize) {
            j = K - 2 as libc::c_int;
            while j >= 0 as libc::c_int && value > *a.offset(j as isize) {
                *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
                *idx.offset((j + 1 as libc::c_int) as isize) = *idx.offset(j as isize);
                j -= 1;
            }
            *a.offset((j + 1 as libc::c_int) as isize) = value;
            *idx.offset((j + 1 as libc::c_int) as isize) = i;
        }
        i += 1;
    }
}
