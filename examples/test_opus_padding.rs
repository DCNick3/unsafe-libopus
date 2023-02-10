#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![feature(stdsimd)]
#![register_tool(c2rust)]

use ::libc;
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:32"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:35"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:35"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/usr/include/stdio.h:32"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut libc::c_void);
        #[c2rust::src_loc = "611:13"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:34"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/tests/test_opus_common.h:36"]
pub mod test_opus_common_h {
    #[c2rust::src_loc = "63:20"]
    pub static mut iseed: opus_uint32 = 0;
    #[inline]
    #[c2rust::src_loc = "66:1"]
    pub unsafe extern "C" fn _test_failed(
        mut file: *const libc::c_char,
        mut line: libc::c_int,
    ) -> ! {
        fprintf(
            stderr,
            b"\n ***************************************************\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b" ***         A fatal error was detected.         ***\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b" ***************************************************\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Please report this failure and include\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"'make check SEED=%u fails %s at line %d for %s'\n\0" as *const u8
                as *const libc::c_char,
            iseed,
            file,
            line,
            opus_get_version_string(),
        );
        fprintf(
            stderr,
            b"and any relevant details about your system.\n\n\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }

    use super::opus_types_h::opus_uint32;
    use super::stdio_h::{fprintf, stderr};
    use super::stdlib_h::abort;
    use libopus_unsafe::opus_get_version_string;
}
pub use self::opus_types_h::{opus_int16, opus_uint32};
use self::stdio_h::{fprintf, stderr};
use self::stdlib_h::{free, malloc};
use self::string_h::memset;
pub use self::test_opus_common_h::{_test_failed, iseed};
use libopus_unsafe::{
    opus_decode, opus_decoder_create, opus_decoder_destroy, opus_get_version_string, OpusDecoder,
};
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn test_overflow() -> libc::c_int {
    let mut decoder: *mut OpusDecoder = 0 as *mut OpusDecoder;
    let mut result: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut in_0: *mut libc::c_uchar =
        malloc(16909318 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    let mut out: *mut opus_int16 = malloc(
        ((5760 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
    ) as *mut opus_int16;
    fprintf(
        stderr,
        b"  Checking for padding overflow... \0" as *const u8 as *const libc::c_char,
    );
    if in_0.is_null() || out.is_null() {
        fprintf(
            stderr,
            b"FAIL (out of memory)\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *in_0.offset(0 as libc::c_int as isize) = 0xff as libc::c_int as libc::c_uchar;
    *in_0.offset(1 as libc::c_int as isize) = 0x41 as libc::c_int as libc::c_uchar;
    memset(
        in_0.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        0xff as libc::c_int,
        (16909318 as libc::c_int - 3 as libc::c_int) as libc::c_ulong,
    );
    *in_0.offset((16909318 as libc::c_int - 1 as libc::c_int) as isize) =
        0xb as libc::c_int as libc::c_uchar;
    decoder = opus_decoder_create(48000 as libc::c_int, 2 as libc::c_int, &mut error);
    result = opus_decode(
        decoder,
        in_0,
        16909318 as libc::c_int,
        out,
        5760 as libc::c_int,
        0 as libc::c_int,
    );
    opus_decoder_destroy(decoder);
    free(in_0 as *mut libc::c_void);
    free(out as *mut libc::c_void);
    if result != -(4 as libc::c_int) {
        fprintf(stderr, b"FAIL!\n\0" as *const u8 as *const libc::c_char);
        _test_failed(
            b"tests/test_opus_padding.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
        );
    }
    fprintf(stderr, b"OK.\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "78:1"]
unsafe fn main_0() -> libc::c_int {
    let mut oversion: *const libc::c_char = 0 as *const libc::c_char;
    let mut tests: libc::c_int = 0 as libc::c_int;
    iseed = 0 as libc::c_int as opus_uint32;
    oversion = opus_get_version_string();
    if oversion.is_null() {
        _test_failed(
            b"tests/test_opus_padding.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
        );
    }
    fprintf(
        stderr,
        b"Testing %s padding.\n\0" as *const u8 as *const libc::c_char,
        oversion,
    );
    tests += test_overflow();
    fprintf(
        stderr,
        b"All padding tests passed.\n\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
