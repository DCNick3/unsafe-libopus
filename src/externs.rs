//! Containes implementation for some common libc functions.
//!
//! This is used to avoid unnecessarily linking to libc.

use core::mem::{self, MaybeUninit};
use core::ptr;
use core::slice;
use std::alloc::{self as rust, Layout};

const HEADER: usize = mem::size_of::<usize>();

const MALLOC_ALIGN: usize = mem::align_of::<usize>();

pub unsafe fn malloc(size: libc::c_ulong) -> *mut libc::c_void {
    let size = HEADER + size as usize;
    let layout = Layout::from_size_align_unchecked(size, MALLOC_ALIGN);
    let memory = rust::alloc(layout);
    if memory.is_null() {
        rust::handle_alloc_error(layout);
    }
    memory.cast::<usize>().write(size);
    let result = memory.add(HEADER).cast();
    result
}

pub unsafe fn calloc(n: libc::c_ulong, size: libc::c_ulong) -> *mut libc::c_void {
    let mem = malloc(n * size);
    ptr::write_bytes(mem, 0, (n * size) as usize);
    mem
}

pub unsafe fn realloc(ptr: *mut libc::c_void, new_size: libc::c_ulong) -> *mut libc::c_void {
    if ptr.is_null() {
        return calloc(1, new_size);
    }

    let mut memory = ptr.cast::<u8>().sub(HEADER);
    let size = memory.cast::<usize>().read();
    let layout = Layout::from_size_align_unchecked(size, MALLOC_ALIGN);
    let new_size = HEADER + new_size as usize;
    memory = rust::realloc(memory, layout, new_size);
    if memory.is_null() {
        let layout = Layout::from_size_align_unchecked(new_size, MALLOC_ALIGN);
        rust::handle_alloc_error(layout);
    }
    memory.cast::<usize>().write(new_size);
    memory.add(HEADER).cast()
}

pub unsafe fn free(ptr: *mut libc::c_void) {
    if ptr == ptr::null_mut() {
        return;
    }

    let memory = ptr.cast::<u8>().sub(HEADER);
    let size = memory.cast::<usize>().read();
    let layout = Layout::from_size_align_unchecked(size, MALLOC_ALIGN);
    rust::dealloc(memory, layout);
}

pub unsafe fn memcmp(
    lhs: *const libc::c_void,
    rhs: *const libc::c_void,
    count: libc::c_ulong,
) -> libc::c_int {
    let lhs = slice::from_raw_parts(lhs.cast::<u8>(), count as usize);
    let rhs = slice::from_raw_parts(rhs.cast::<u8>(), count as usize);
    lhs.cmp(rhs) as libc::c_int
}

pub unsafe fn memcpy(
    dest: *mut libc::c_void,
    src: *const libc::c_void,
    count: libc::c_ulong,
) -> *mut libc::c_void {
    ptr::copy_nonoverlapping(
        src.cast::<MaybeUninit<u8>>(),
        dest.cast::<MaybeUninit<u8>>(),
        count as usize,
    );
    dest
}

pub unsafe fn memmove(
    dest: *mut libc::c_void,
    src: *const libc::c_void,
    count: libc::c_ulong,
) -> *mut libc::c_void {
    ptr::copy(
        src.cast::<MaybeUninit<u8>>(),
        dest.cast::<MaybeUninit<u8>>(),
        count as usize,
    );
    dest
}

pub unsafe fn memset(
    dest: *mut libc::c_void,
    ch: libc::c_int,
    count: libc::c_ulong,
) -> *mut libc::c_void {
    ptr::write_bytes(dest.cast::<u8>(), ch as u8, count as usize);
    dest
}

pub unsafe fn strcmp(lhs: *const libc::c_char, rhs: *const libc::c_char) -> libc::c_int {
    let lhs = slice::from_raw_parts(lhs.cast::<u8>(), strlen(lhs) as usize);
    let rhs = slice::from_raw_parts(rhs.cast::<u8>(), strlen(rhs) as usize);
    lhs.cmp(rhs) as libc::c_int
}

pub unsafe fn strdup(src: *const libc::c_char) -> *mut libc::c_char {
    let len = strlen(src);
    let dest = malloc(len + 1);
    memcpy(dest, src.cast(), len + 1);
    dest.cast()
}

pub unsafe fn strlen(str: *const libc::c_char) -> libc::c_ulong {
    let mut end = str;
    while *end != 0 {
        end = end.add(1);
    }
    end.offset_from(str) as libc::c_ulong
}

pub unsafe fn strncmp(
    lhs: *const libc::c_char,
    rhs: *const libc::c_char,
    mut count: libc::c_ulong,
) -> libc::c_int {
    let mut lhs = lhs.cast::<u8>();
    let mut rhs = rhs.cast::<u8>();
    while count > 0 && *lhs != 0 && *lhs == *rhs {
        lhs = lhs.add(1);
        rhs = rhs.add(1);
        count -= 1;
    }
    if count == 0 {
        0
    } else {
        (*lhs).cmp(&*rhs) as libc::c_int
    }
}

// macro_rules! __assert {
//     (false $(,)?) => {
//         $crate::externs::__assert_fail(stringify!(false), file!(), line!())
//     };
//     ($assertion:expr $(,)?) => {
//         if !$assertion {
//             $crate::externs::__assert_fail(stringify!($assertion), file!(), line!());
//         }
//     };
// }
//
// pub(crate) unsafe fn __assert_fail(
//     __assertion: &'static str,
//     __file: &'static str,
//     __line: u32,
// ) -> ! {
//     struct Abort;
//     impl Drop for Abort {
//         fn drop(&mut self) {
//             panic!();
//         }
//     }
//     let _abort_on_panic = Abort;
//     panic!("{}:{}: Assertion `{}` failed.", __file, __line, __assertion);
// }
