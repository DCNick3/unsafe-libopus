#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use libc::fprintf;
use libc_stdhandle::stderr;

pub mod test_opus_common_h {
    pub static mut iseed: u32 = 0;
    #[inline]
    pub unsafe fn _test_failed(mut file: *const i8, mut line: i32) -> ! {
        fprintf(
            stderr(),
            b"\n ***************************************************\n\0" as *const u8 as *const i8,
        );
        fprintf(
            stderr(),
            b" ***         A fatal error was detected.         ***\n\0" as *const u8 as *const i8,
        );
        fprintf(
            stderr(),
            b" ***************************************************\n\0" as *const u8 as *const i8,
        );
        fprintf(
            stderr(),
            b"Please report this failure and include\n\0" as *const u8 as *const i8,
        );
        fprintf(
            stderr(),
            b"'make check SEED=%u fails %s at line %d for %s'\n\0" as *const u8 as *const i8,
            iseed,
            file,
            line,
            opus_get_version_string(),
        );
        fprintf(
            stderr(),
            b"and any relevant details about your system.\n\n\0" as *const u8 as *const i8,
        );
        abort();
    }

    use libc::{abort, fprintf};
    use libc_stdhandle::stderr;
    use unsafe_libopus::opus_get_version_string;
}
pub use self::test_opus_common_h::{_test_failed, iseed};
use unsafe_libopus::externs::memset;
use unsafe_libopus::externs::{free, malloc};
use unsafe_libopus::{
    opus_decode, opus_decoder_create, opus_decoder_destroy, opus_get_version_string, OpusDecoder,
};

pub unsafe fn test_overflow() -> i32 {
    let mut decoder: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut result: i32 = 0;
    let mut error: i32 = 0;
    let mut in_0: *mut u8 = malloc(16909318 as i32 as u64) as *mut u8;
    let mut out: *mut i16 = malloc(
        ((5760 as i32 * 2 as i32) as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64),
    ) as *mut i16;
    fprintf(
        stderr(),
        b"  Checking for padding overflow... \0" as *const u8 as *const i8,
    );
    if in_0.is_null() || out.is_null() {
        fprintf(
            stderr(),
            b"FAIL (out of memory)\n\0" as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    *in_0.offset(0 as i32 as isize) = 0xff as i32 as u8;
    *in_0.offset(1 as i32 as isize) = 0x41 as i32 as u8;
    memset(
        in_0.offset(2 as i32 as isize) as *mut core::ffi::c_void,
        0xff as i32,
        (16909318 as i32 - 3 as i32) as u64,
    );
    *in_0.offset((16909318 as i32 - 1 as i32) as isize) = 0xb as i32 as u8;
    decoder = opus_decoder_create(48000 as i32, 2 as i32, &mut error);
    result = opus_decode(decoder, in_0, 16909318 as i32, out, 5760 as i32, 0 as i32);
    opus_decoder_destroy(decoder);
    free(in_0 as *mut core::ffi::c_void);
    free(out as *mut core::ffi::c_void);
    if result != -(4 as i32) {
        fprintf(stderr(), b"FAIL!\n\0" as *const u8 as *const i8);
        _test_failed(
            b"tests/test_opus_padding.c\0" as *const u8 as *const i8,
            70 as i32,
        );
    }
    fprintf(stderr(), b"OK.\n\0" as *const u8 as *const i8);
    1 as i32
}
unsafe fn main_0() -> i32 {
    let mut oversion: *const i8 = std::ptr::null::<i8>();
    let mut _tests: i32 = 0 as i32;
    iseed = 0 as i32 as u32;
    oversion = opus_get_version_string();
    if oversion.is_null() {
        _test_failed(
            b"tests/test_opus_padding.c\0" as *const u8 as *const i8,
            85 as i32,
        );
    }
    fprintf(
        stderr(),
        b"Testing %s padding.\n\0" as *const u8 as *const i8,
        oversion,
    );
    _tests += test_overflow();
    fprintf(
        stderr(),
        b"All padding tests passed.\n\0" as *const u8 as *const i8,
    );
    0 as i32
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
