use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:38"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
use self::arch_h::celt_fatal;

#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn silk_insertion_sort_increasing(
    a: *mut i32,
    idx: *mut libc::c_int,
    L: libc::c_int,
    K: libc::c_int,
) {
    let mut value: i32 = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !(K > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: K > 0\0" as *const u8 as *const libc::c_char,
            b"silk/sort.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
        );
    }
    if !(L > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: L > 0\0" as *const u8 as *const libc::c_char,
            b"silk/sort.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
        );
    }
    if !(L >= K) {
        celt_fatal(
            b"assertion failed: L >= K\0" as *const u8 as *const libc::c_char,
            b"silk/sort.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
        );
    }
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
    a: *mut i16,
    L: libc::c_int,
) {
    let mut value: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !(L > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: L > 0\0" as *const u8 as *const libc::c_char,
            b"silk/sort.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
        );
    }
    i = 1 as libc::c_int;
    while i < L {
        value = *a.offset(i as isize) as libc::c_int;
        j = i - 1 as libc::c_int;
        while j >= 0 as libc::c_int && value < *a.offset(j as isize) as libc::c_int {
            *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
            j -= 1;
        }
        *a.offset((j + 1 as libc::c_int) as isize) = value as i16;
        i += 1;
    }
}
