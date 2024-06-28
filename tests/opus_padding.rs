#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(deprecated)]

pub mod test_opus_common_h {
    pub static mut iseed: u32 = 0;
    pub unsafe fn _test_failed(mut file: *const i8, mut line: i32) -> ! {
        eprintln!();
        eprintln!(" ***************************************************");
        eprintln!(" ***         A fatal error was detected.         ***");
        eprintln!(" ***************************************************");
        eprintln!("Please report this failure and include");
        eprintln!(
            "'make check SEED={} fails {} at line {} for {}'",
            iseed,
            std::ffi::CStr::from_ptr(file as _).to_str().unwrap(),
            line,
            opus_get_version_string()
        );
        eprintln!("and any relevant details about your system.");
        panic!("test failed");
    }

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
    let mut in_0: *mut u8 = malloc(16909318) as *mut u8;
    let mut out: *mut i16 =
        malloc(((5760 * 2) as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64)) as *mut i16;
    eprintln!("  Checking for padding overflow... ");
    *in_0.offset(0 as isize) = 0xff;
    *in_0.offset(1 as isize) = 0x41;
    memset(
        in_0.offset(2 as isize) as *mut core::ffi::c_void,
        0xff,
        (16909318 - 3) as u64,
    );
    *in_0.offset((16909318 - 1) as isize) = 0xb;
    decoder = opus_decoder_create(48000, 2, &mut error);
    result = opus_decode(&mut *decoder, in_0, 16909318, out, 5760, 0);
    opus_decoder_destroy(decoder);
    free(in_0 as *mut core::ffi::c_void);
    free(out as *mut core::ffi::c_void);
    if result != -(4) {
        eprintln!("FAIL!");
        _test_failed(b"tests/test_opus_padding.c\0" as *const u8 as *const i8, 70);
    }
    eprintln!("OK.");
    1
}
unsafe fn main_0() -> i32 {
    let mut _tests: i32 = 0;
    iseed = 0;
    let oversion = opus_get_version_string();
    eprintln!("Testing {} padding.", oversion);
    _tests += test_overflow();
    eprintln!("All padding tests passed.");
    0
}

#[test]
fn test_opus_padding() {
    assert_eq!(unsafe { main_0() }, 0, "Test returned a non-zero exit code");
}
