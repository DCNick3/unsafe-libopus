use ::libc;
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    {
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
pub use self::stddef_h::size_t;

use self::stdio_h::{fprintf, stderr};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};

pub use self::FILE_h::FILE;
#[c2rust::src_loc = "35:1"]
unsafe fn main_0() -> libc::c_int {
    let mut i: i16 = 1 as libc::c_int as i16;
    i = ((i as libc::c_int) << 14 as libc::c_int) as i16;
    if i as libc::c_int >> 14 as libc::c_int != 1 as libc::c_int {
        fprintf(
            stderr(),
            b"i16 isn't 16 bits\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if (::core::mem::size_of::<i16>() as libc::c_ulong)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
        != ::core::mem::size_of::<i32>() as libc::c_ulong
    {
        fprintf(
            stderr(),
            b"16*2 != 32\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
