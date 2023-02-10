use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:38"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:38"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:38"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
}
pub use self::opus_types_h::{opus_int16, opus_int32};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::types_h::{__int16_t, __int32_t};
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn silk_insertion_sort_increasing(
    mut a: *mut opus_int32,
    mut idx: *mut libc::c_int,
    L: libc::c_int,
    K: libc::c_int,
) {
    let mut value: opus_int32 = 0;
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
        while j >= 0 as libc::c_int && value < *a.offset(j as isize) {
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
        if value < *a.offset((K - 1 as libc::c_int) as isize) {
            j = K - 2 as libc::c_int;
            while j >= 0 as libc::c_int && value < *a.offset(j as isize) {
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
#[no_mangle]
#[c2rust::src_loc = "135:1"]
pub unsafe extern "C" fn silk_insertion_sort_increasing_all_values_int16(
    mut a: *mut opus_int16,
    L: libc::c_int,
) {
    let mut value: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < L {
        value = *a.offset(i as isize) as libc::c_int;
        j = i - 1 as libc::c_int;
        while j >= 0 as libc::c_int && value < *a.offset(j as isize) as libc::c_int {
            *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
            j -= 1;
        }
        *a.offset((j + 1 as libc::c_int) as isize) = value as opus_int16;
        i += 1;
    }
}
