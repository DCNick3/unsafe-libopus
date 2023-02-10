use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:32"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:32"]
pub mod SigProc_FLP_h {
    #[c2rust::src_loc = "143:9"]
    pub const PI: libc::c_float = 3.1415926536f32;
}
use self::arch_h::celt_fatal;
pub use self::SigProc_FLP_h::PI;
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn silk_apply_sine_window_FLP(
    px_win: *mut libc::c_float,
    px: *const libc::c_float,
    win_type: libc::c_int,
    length: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut freq: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut S0: libc::c_float = 0.;
    let mut S1: libc::c_float = 0.;
    if !(win_type == 1 as libc::c_int || win_type == 2 as libc::c_int) {
        celt_fatal(
            b"assertion failed: win_type == 1 || win_type == 2\0" as *const u8
                as *const libc::c_char,
            b"silk/float/apply_sine_window_FLP.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
        );
    }
    if !(length & 3 as libc::c_int == 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: ( length & 3 ) == 0\0" as *const u8 as *const libc::c_char,
            b"silk/float/apply_sine_window_FLP.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
        );
    }
    freq = PI / (length + 1 as libc::c_int) as libc::c_float;
    c = 2.0f32 - freq * freq;
    if win_type < 2 as libc::c_int {
        S0 = 0.0f32;
        S1 = freq;
    } else {
        S0 = 1.0f32;
        S1 = 0.5f32 * c;
    }
    k = 0 as libc::c_int;
    while k < length {
        *px_win.offset((k + 0 as libc::c_int) as isize) =
            *px.offset((k + 0 as libc::c_int) as isize) * 0.5f32 * (S0 + S1);
        *px_win.offset((k + 1 as libc::c_int) as isize) =
            *px.offset((k + 1 as libc::c_int) as isize) * S1;
        S0 = c * S1 - S0;
        *px_win.offset((k + 2 as libc::c_int) as isize) =
            *px.offset((k + 2 as libc::c_int) as isize) * 0.5f32 * (S1 + S0);
        *px_win.offset((k + 3 as libc::c_int) as isize) =
            *px.offset((k + 3 as libc::c_int) as isize) * S0;
        S1 = c * S0 - S1;
        k += 4 as libc::c_int;
    }
}
