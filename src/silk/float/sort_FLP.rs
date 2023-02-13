use crate::celt::celt::celt_fatal;
#[c2rust::src_loc = "39:1"]
pub unsafe fn silk_insertion_sort_decreasing_FLP(a: *mut f32, idx: *mut i32, L: i32, K: i32) {
    let mut value: f32 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if !(K > 0 as i32) {
        celt_fatal(
            b"assertion failed: K > 0\0" as *const u8 as *const i8,
            b"silk/float/sort_FLP.c\0" as *const u8 as *const i8,
            50 as i32,
        );
    }
    if !(L > 0 as i32) {
        celt_fatal(
            b"assertion failed: L > 0\0" as *const u8 as *const i8,
            b"silk/float/sort_FLP.c\0" as *const u8 as *const i8,
            51 as i32,
        );
    }
    if !(L >= K) {
        celt_fatal(
            b"assertion failed: L >= K\0" as *const u8 as *const i8,
            b"silk/float/sort_FLP.c\0" as *const u8 as *const i8,
            52 as i32,
        );
    }
    i = 0 as i32;
    while i < K {
        *idx.offset(i as isize) = i;
        i += 1;
    }
    i = 1 as i32;
    while i < K {
        value = *a.offset(i as isize);
        j = i - 1 as i32;
        while j >= 0 as i32 && value > *a.offset(j as isize) {
            *a.offset((j + 1 as i32) as isize) = *a.offset(j as isize);
            *idx.offset((j + 1 as i32) as isize) = *idx.offset(j as isize);
            j -= 1;
        }
        *a.offset((j + 1 as i32) as isize) = value;
        *idx.offset((j + 1 as i32) as isize) = i;
        i += 1;
    }
    i = K;
    while i < L {
        value = *a.offset(i as isize);
        if value > *a.offset((K - 1 as i32) as isize) {
            j = K - 2 as i32;
            while j >= 0 as i32 && value > *a.offset(j as isize) {
                *a.offset((j + 1 as i32) as isize) = *a.offset(j as isize);
                *idx.offset((j + 1 as i32) as isize) = *idx.offset(j as isize);
                j -= 1;
            }
            *a.offset((j + 1 as i32) as isize) = value;
            *idx.offset((j + 1 as i32) as isize) = i;
        }
        i += 1;
    }
}
