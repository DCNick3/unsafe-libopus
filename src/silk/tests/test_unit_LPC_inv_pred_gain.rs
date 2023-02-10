use ::libc;
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/stdio.h:32"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "454:1"]
        pub fn rand() -> libc::c_int;
        #[c2rust::src_loc = "456:1"]
        pub fn srand(__seed: libc::c_uint);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cpu_support.h:35"]
pub mod cpu_support_h {
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn opus_select_arch() -> libc::c_int {
        return 0 as libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:36"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "162:14"]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:36"]
pub mod SigProc_FIX_h {
    extern "C" {
        #[c2rust::src_loc = "148:1"]
        pub fn silk_LPC_inverse_pred_gain_c(A_Q12: *const i16, order: libc::c_int) -> i32;
    }
}
pub use self::cpu_support_h::opus_select_arch;
pub use self::stddef_h::size_t;

use self::stdio_h::{fprintf, printf, stderr};
use self::stdlib_h::{rand, srand};

pub use self::FILE_h::FILE;
use self::SigProc_FIX_h::silk_LPC_inverse_pred_gain_c;
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn check_stability(A_Q12: *mut i16, order: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sum_a: libc::c_int = 0;
    let mut sum_abs_a: libc::c_int = 0;
    sum_abs_a = 0 as libc::c_int;
    sum_a = sum_abs_a;
    j = 0 as libc::c_int;
    while j < order {
        sum_a += *A_Q12.offset(j as isize) as libc::c_int;
        sum_abs_a += if *A_Q12.offset(j as isize) as libc::c_int > 0 as libc::c_int {
            *A_Q12.offset(j as isize) as libc::c_int
        } else {
            -(*A_Q12.offset(j as isize) as libc::c_int)
        };
        j += 1;
    }
    if sum_a >= 4096 as libc::c_int {
        return 0 as libc::c_int;
    }
    if sum_abs_a < 4096 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut y: [libc::c_double; 24] = [
        0 as libc::c_int as libc::c_double,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    y[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < order {
            sum += y[j as usize] * *A_Q12.offset(j as isize) as libc::c_int as libc::c_double;
            j += 1;
        }
        j = order - 1 as libc::c_int;
        while j > 0 as libc::c_int {
            y[j as usize] = y[(j - 1 as libc::c_int) as usize];
            j -= 1;
        }
        y[0 as libc::c_int as usize] = sum * (1.0f64 / 4096 as libc::c_int as libc::c_double);
        if !(y[0 as libc::c_int as usize] < 10000 as libc::c_int as libc::c_double
            && y[0 as libc::c_int as usize] > -(10000 as libc::c_int) as libc::c_double)
        {
            return 0 as libc::c_int;
        }
        if i & 0x7 as libc::c_int == 0 as libc::c_int {
            let mut amp: libc::c_double = 0 as libc::c_int as libc::c_double;
            j = 0 as libc::c_int;
            while j < order {
                amp += y[j as usize].abs();
                j += 1;
            }
            if amp < 0.00001f64 {
                return 1 as libc::c_int;
            }
        }
        i += 1;
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "90:1"]
unsafe fn main_0() -> libc::c_int {
    let _arch: libc::c_int = opus_select_arch();
    let loop_num: libc::c_int = 10000 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    srand(0 as libc::c_int as libc::c_uint);
    printf(
        b"Testing silk_LPC_inverse_pred_gain() optimization ...\n\0" as *const u8
            as *const libc::c_char,
    );
    count = 0 as libc::c_int;
    while count < loop_num {
        let mut i: libc::c_uint = 0;
        let mut order: libc::c_int = 0;
        let mut shift: libc::c_uint = 0;
        let mut A_Q12: [i16; 24] = [0; 24];
        let mut gain: i32 = 0;
        order = 2 as libc::c_int;
        while order <= 24 as libc::c_int {
            shift = 0 as libc::c_int as libc::c_uint;
            while shift < 16 as libc::c_int as libc::c_uint {
                i = 0 as libc::c_int as libc::c_uint;
                while i < 24 as libc::c_int as libc::c_uint {
                    A_Q12[i as usize] = (rand() as i16 as libc::c_int >> shift) as i16;
                    i = i.wrapping_add(1);
                }
                gain = silk_LPC_inverse_pred_gain_c(A_Q12.as_mut_ptr(), order);
                if gain != 0 as libc::c_int && check_stability(A_Q12.as_mut_ptr(), order) == 0 {
                    fprintf(
                        stderr(),
                        b"**Loop %4d failed!**\n\0" as *const u8 as *const libc::c_char,
                        count,
                    );
                    return 1 as libc::c_int;
                }
                shift = shift.wrapping_add(1);
            }
            order += 2 as libc::c_int;
        }
        if count % 500 as libc::c_int == 0 {
            printf(
                b"Loop %4d passed\n\0" as *const u8 as *const libc::c_char,
                count,
            );
        }
        count += 1;
    }
    printf(
        b"silk_LPC_inverse_pred_gain() optimization passed\n\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
