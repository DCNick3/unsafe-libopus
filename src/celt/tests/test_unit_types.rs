#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = u64;
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    {
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    }
}
pub use self::stddef_h::size_t;

use self::stdio_h::{fprintf, stderr};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};

pub use self::FILE_h::FILE;
#[c2rust::src_loc = "35:1"]
unsafe fn main_0() -> i32 {
    let mut i: i16 = 1 as i32 as i16;
    i = ((i as i32) << 14 as i32) as i16;
    if i as i32 >> 14 as i32 != 1 as i32 {
        fprintf(
            stderr(),
            b"i16 isn't 16 bits\n\0" as *const u8 as *const i8,
        );
        return 1 as i32;
    }
    if (::core::mem::size_of::<i16>() as u64)
        .wrapping_mul(2 as i32 as u64)
        != ::core::mem::size_of::<i32>() as u64
    {
        fprintf(
            stderr(),
            b"16*2 != 32\n\0" as *const u8 as *const i8,
        );
        return 1 as i32;
    }
    return 0 as i32;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
