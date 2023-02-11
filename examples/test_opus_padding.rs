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
use libc::fprintf;
use libc_stdhandle::stderr;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/tests/test_opus_common.h:36"]
pub mod test_opus_common_h {
    #[c2rust::src_loc = "63:20"]
    pub static mut iseed: u32 = 0;
    #[inline]
    #[c2rust::src_loc = "66:1"]
    pub unsafe extern "C" fn _test_failed(
        mut file: *const libc::c_char,
        mut line: libc::c_int,
    ) -> ! {
        fprintf(
            stderr(),
            b"\n ***************************************************\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b" ***         A fatal error was detected.         ***\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b" ***************************************************\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b"Please report this failure and include\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b"'make check SEED=%u fails %s at line %d for %s'\n\0" as *const u8
                as *const libc::c_char,
            iseed,
            file,
            line,
            opus_get_version_string(),
        );
        fprintf(
            stderr(),
            b"and any relevant details about your system.\n\n\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }

    use libc::{abort, fprintf};
    use libc_stdhandle::stderr;
    use libopus_unsafe::opus_get_version_string;
}
pub use self::test_opus_common_h::{_test_failed, iseed};
use libopus_unsafe::externs::memset;
use libopus_unsafe::externs::{free, malloc};
use libopus_unsafe::{
    opus_decode, opus_decoder_create, opus_decoder_destroy, opus_get_version_string, OpusDecoder,
};

#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn test_overflow() -> libc::c_int {
    let mut decoder: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut result: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut in_0: *mut libc::c_uchar =
        malloc(16909318 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    let mut out: *mut i16 = malloc(
        ((5760 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    ) as *mut i16;
    fprintf(
        stderr(),
        b"  Checking for padding overflow... \0" as *const u8 as *const libc::c_char,
    );
    if in_0.is_null() || out.is_null() {
        fprintf(
            stderr(),
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
        fprintf(stderr(), b"FAIL!\n\0" as *const u8 as *const libc::c_char);
        _test_failed(
            b"tests/test_opus_padding.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
        );
    }
    fprintf(stderr(), b"OK.\n\0" as *const u8 as *const libc::c_char);
    1 as libc::c_int
}
#[c2rust::src_loc = "78:1"]
unsafe fn main_0() -> libc::c_int {
    let mut oversion: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut _tests: libc::c_int = 0 as libc::c_int;
    iseed = 0 as libc::c_int as u32;
    oversion = opus_get_version_string();
    if oversion.is_null() {
        _test_failed(
            b"tests/test_opus_padding.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
        );
    }
    fprintf(
        stderr(),
        b"Testing %s padding.\n\0" as *const u8 as *const libc::c_char,
        oversion,
    );
    _tests += test_overflow();
    fprintf(
        stderr(),
        b"All padding tests passed.\n\0" as *const u8 as *const libc::c_char,
    );
    0 as libc::c_int
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
