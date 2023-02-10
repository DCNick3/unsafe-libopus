use ::libc;
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:36"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:36"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:36"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::stddef_h::size_t;
    use super::types_h::{__off64_t, __off_t};
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:36"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:38"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "184:1"]
    pub type celt_norm = libc::c_float;
}
#[c2rust::header_src = "/usr/include/stdio.h:36"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "356:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:37"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "454:1"]
        pub fn rand() -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:38"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "107:17"]
        pub fn log10(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/vq.h:38"]
pub mod vq_h {
    use super::arch_h::celt_norm;
    extern "C" {
        #[c2rust::src_loc = "44:1"]
        pub fn exp_rotation(
            X: *mut celt_norm,
            len: libc::c_int,
            dir: libc::c_int,
            stride: libc::c_int,
            K: libc::c_int,
            spread: libc::c_int,
        );
    }
}
pub use self::arch_h::{celt_norm, opus_val16};
use self::mathcalls_h::log10;
pub use self::stddef_h::size_t;
use self::stdio_h::{fprintf, printf, stderr};
use self::stdlib_h::rand;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__off64_t, __off_t};
use self::vq_h::exp_rotation;
pub use self::FILE_h::FILE;
#[no_mangle]
#[c2rust::src_loc = "46:5"]
pub static mut ret: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn test_rotation(mut N: libc::c_int, mut K: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut err: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ener: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut snr: libc::c_double = 0.;
    let mut snr0: libc::c_double = 0.;
    let mut x0: [opus_val16; 100] = [0.; 100];
    let mut x1: [opus_val16; 100] = [0.; 100];
    i = 0 as libc::c_int;
    while i < N {
        x0[i as usize] = (rand() % 32767 as libc::c_int - 16384 as libc::c_int) as opus_val16;
        x1[i as usize] = x0[i as usize];
        i += 1;
    }
    exp_rotation(
        x1.as_mut_ptr(),
        N,
        1 as libc::c_int,
        1 as libc::c_int,
        K,
        2 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < N {
        err += (x0[i as usize] as libc::c_double - x1[i as usize] as libc::c_double)
            * (x0[i as usize] as libc::c_double - x1[i as usize] as libc::c_double);
        ener += x0[i as usize] as libc::c_double * x0[i as usize] as libc::c_double;
        i += 1;
    }
    snr0 = 20 as libc::c_int as libc::c_double * log10(ener / err);
    ener = 0 as libc::c_int as libc::c_double;
    err = ener;
    exp_rotation(
        x1.as_mut_ptr(),
        N,
        -(1 as libc::c_int),
        1 as libc::c_int,
        K,
        2 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < N {
        err += (x0[i as usize] as libc::c_double - x1[i as usize] as libc::c_double)
            * (x0[i as usize] as libc::c_double - x1[i as usize] as libc::c_double);
        ener += x0[i as usize] as libc::c_double * x0[i as usize] as libc::c_double;
        i += 1;
    }
    snr = 20 as libc::c_int as libc::c_double * log10(ener / err);
    printf(
        b"SNR for size %d (%d pulses) is %f (was %f without inverse)\n\0" as *const u8
            as *const libc::c_char,
        N,
        K,
        snr,
        snr0,
    );
    if snr < 60 as libc::c_int as libc::c_double || snr0 > 20 as libc::c_int as libc::c_double {
        fprintf(stderr, b"FAIL!\n\0" as *const u8 as *const libc::c_char);
        ret = 1 as libc::c_int;
    }
}
#[c2rust::src_loc = "78:1"]
unsafe fn main_0() -> libc::c_int {
    test_rotation(15 as libc::c_int, 3 as libc::c_int);
    test_rotation(23 as libc::c_int, 5 as libc::c_int);
    test_rotation(50 as libc::c_int, 3 as libc::c_int);
    test_rotation(80 as libc::c_int, 1 as libc::c_int);
    return ret;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
