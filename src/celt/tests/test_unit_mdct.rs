use ::libc;
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:34"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/kiss_fft.h:34"]
pub mod kiss_fft_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:9"]
    pub struct kiss_twiddle_cpx {
        pub r: libc::c_float,
        pub i: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct arch_fft_state {
        pub is_supported: libc::c_int,
        pub priv_0: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:16"]
    pub struct kiss_fft_state {
        pub nfft: libc::c_int,
        pub scale: opus_val16,
        pub shift: libc::c_int,
        pub factors: [i16; 16],
        pub bitrev: *const i16,
        pub twiddles: *const kiss_twiddle_cpx,
        pub arch_fft: *mut arch_fft_state,
    }
    use super::arch_h::opus_val16;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mdct.h:34"]
pub mod mdct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:9"]
    pub struct mdct_lookup {
        pub n: libc::c_int,
        pub maxshift: libc::c_int,
        pub kfft: [*const kiss_fft_state; 4],
        pub trig: *const libc::c_float,
    }
    use super::arch_h::opus_val16;
    use super::kiss_fft_h::kiss_fft_state;
    {
        #[c2rust::src_loc = "72:1"]
        pub fn clt_mdct_backward_c(
            l: *const mdct_lookup,
            in_0: *mut libc::c_float,
            out: *mut libc::c_float,
            window: *const opus_val16,
            overlap: libc::c_int,
            shift: libc::c_int,
            stride: libc::c_int,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "65:1"]
        pub fn clt_mdct_forward_c(
            l: *const mdct_lookup,
            in_0: *mut libc::c_float,
            out: *mut libc::c_float,
            window: *const opus_val16,
            overlap: libc::c_int,
            shift: libc::c_int,
            stride: libc::c_int,
            arch: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/modes.h:38"]
pub mod modes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:8"]
    pub struct OpusCustomMode {
        pub Fs: i32,
        pub overlap: libc::c_int,
        pub nbEBands: libc::c_int,
        pub effEBands: libc::c_int,
        pub preemph: [opus_val16; 4],
        pub eBands: *const i16,
        pub maxLM: libc::c_int,
        pub nbShortMdcts: libc::c_int,
        pub shortMdctSize: libc::c_int,
        pub nbAllocVectors: libc::c_int,
        pub allocVectors: *const libc::c_uchar,
        pub logN: *const i16,
        pub window: *const opus_val16,
        pub mdct: mdct_lookup,
        pub cache: PulseCache,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:9"]
    pub struct PulseCache {
        pub size: libc::c_int,
        pub index: *const i16,
        pub bits: *const libc::c_uchar,
        pub caps: *const libc::c_uchar,
    }
    use super::arch_h::opus_val16;
    use crate::celt::mdct::mdct_lookup;
}
#[c2rust::header_src = "/usr/include/stdio.h:32"]
pub mod stdio_h {
    {
        #[c2rust::src_loc = "356:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:34"]
pub mod mathcalls_h {
    {
        #[c2rust::src_loc = "62:17"]
        pub fn cos(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "107:17"]
        pub fn log10(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:34"]
pub mod stdlib_h {
    #[inline]
    #[c2rust::src_loc = "361:1"]
    pub unsafe fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }
    {
        #[c2rust::src_loc = "454:1"]
        pub fn rand() -> libc::c_int;
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut libc::c_void);
        #[c2rust::src_loc = "177:17"]
        pub fn strtol(
            _: *const libc::c_char,
            _: *mut *mut libc::c_char,
            _: libc::c_int,
        ) -> libc::c_long;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cpu_support.h:34"]
pub mod cpu_support_h {
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe fn opus_select_arch() -> libc::c_int {
        return 0 as libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_custom.h:38"]
pub mod opus_custom_h {
    use super::modes_h::OpusCustomMode;
    {
        #[c2rust::src_loc = "121:20"]
        pub fn opus_custom_mode_create(
            Fs: i32,
            frame_size: libc::c_int,
            error: *mut libc::c_int,
        ) -> *mut OpusCustomMode;
    }
}
pub use self::arch_h::opus_val16;
pub use self::cpu_support_h::opus_select_arch;
pub use self::kiss_fft_h::{arch_fft_state, kiss_fft_state, kiss_twiddle_cpx};
use self::mathcalls_h::{cos, log10};
pub use self::mdct_h::{clt_mdct_backward_c, clt_mdct_forward_c, mdct_lookup};
pub use self::modes_h::{OpusCustomMode, PulseCache};
use self::opus_custom_h::opus_custom_mode_create;
pub use self::stddef_h::size_t;

use self::stdio_h::printf;
pub use self::stdlib_h::{atoi, free, malloc, rand, strtol};

#[c2rust::src_loc = "44:5"]
pub static mut ret: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "45:1"]
pub unsafe fn check(
    mut in_0: *mut libc::c_float,
    mut out: *mut libc::c_float,
    mut nfft: libc::c_int,
    mut isinverse: libc::c_int,
) {
    let mut bin: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut errpow: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sigpow: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut snr: libc::c_double = 0.;
    bin = 0 as libc::c_int;
    while bin < nfft / 2 as libc::c_int {
        let mut ansr: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut difr: libc::c_double = 0.;
        k = 0 as libc::c_int;
        while k < nfft {
            let mut phase: libc::c_double = 2 as libc::c_int as libc::c_double
                * 3.14159265358979323846f64
                * (k as libc::c_double + 0.5f64 + 0.25f64 * nfft as libc::c_double)
                * (bin as libc::c_double + 0.5f64)
                / nfft as libc::c_double;
            let mut re: libc::c_double = cos(phase);
            re /= (nfft / 4 as libc::c_int) as libc::c_double;
            ansr += *in_0.offset(k as isize) as libc::c_double * re;
            k += 1;
        }
        difr = ansr - *out.offset(bin as isize) as libc::c_double;
        errpow += difr * difr;
        sigpow += ansr * ansr;
        bin += 1;
    }
    snr = 10 as libc::c_int as libc::c_double * log10(sigpow / errpow);
    printf(
        b"nfft=%d inverse=%d,snr = %f\n\0" as *const u8 as *const libc::c_char,
        nfft,
        isinverse,
        snr,
    );
    if snr < 60 as libc::c_int as libc::c_double {
        printf(
            b"** poor snr: %f **\n\0" as *const u8 as *const libc::c_char,
            snr,
        );
        ret = 1 as libc::c_int;
    }
}
#[c2rust::src_loc = "75:1"]
pub unsafe fn check_inv(
    mut in_0: *mut libc::c_float,
    mut out: *mut libc::c_float,
    mut nfft: libc::c_int,
    mut isinverse: libc::c_int,
) {
    let mut bin: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut errpow: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sigpow: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut snr: libc::c_double = 0.;
    bin = 0 as libc::c_int;
    while bin < nfft {
        let mut ansr: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut difr: libc::c_double = 0.;
        k = 0 as libc::c_int;
        while k < nfft / 2 as libc::c_int {
            let mut phase: libc::c_double = 2 as libc::c_int as libc::c_double
                * 3.14159265358979323846f64
                * (bin as libc::c_double + 0.5f64 + 0.25f64 * nfft as libc::c_double)
                * (k as libc::c_double + 0.5f64)
                / nfft as libc::c_double;
            let mut re: libc::c_double = cos(phase);
            ansr += *in_0.offset(k as isize) as libc::c_double * re;
            k += 1;
        }
        difr = ansr - *out.offset(bin as isize) as libc::c_double;
        errpow += difr * difr;
        sigpow += ansr * ansr;
        bin += 1;
    }
    snr = 10 as libc::c_int as libc::c_double * log10(sigpow / errpow);
    printf(
        b"nfft=%d inverse=%d,snr = %f\n\0" as *const u8 as *const libc::c_char,
        nfft,
        isinverse,
        snr,
    );
    if snr < 60 as libc::c_int as libc::c_double {
        printf(
            b"** poor snr: %f **\n\0" as *const u8 as *const libc::c_char,
            snr,
        );
        ret = 1 as libc::c_int;
    }
}
#[c2rust::src_loc = "106:1"]
pub unsafe fn test1d(
    mut nfft: libc::c_int,
    mut isinverse: libc::c_int,
    mut arch: libc::c_int,
) {
    let mut buflen: size_t = (::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
        .wrapping_mul(nfft as libc::c_ulong);
    let mut in_0: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut in_copy: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut out: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut window: *mut opus_val16 = 0 as *mut opus_val16;
    let mut k: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut cfg: *const mdct_lookup = 0 as *const mdct_lookup;
    let mut mode: *mut OpusCustomMode = opus_custom_mode_create(
        48000 as libc::c_int,
        960 as libc::c_int,
        0 as *mut libc::c_int,
    );
    if nfft == 1920 as libc::c_int {
        shift = 0 as libc::c_int;
    } else if nfft == 960 as libc::c_int {
        shift = 1 as libc::c_int;
    } else if nfft == 480 as libc::c_int {
        shift = 2 as libc::c_int;
    } else if nfft == 240 as libc::c_int {
        shift = 3 as libc::c_int;
    } else {
        return;
    }
    cfg = &mut (*mode).mdct;
    in_0 = malloc(buflen) as *mut libc::c_float;
    in_copy = malloc(buflen) as *mut libc::c_float;
    out = malloc(buflen) as *mut libc::c_float;
    window = malloc(
        (::core::mem::size_of::<opus_val16>() as libc::c_ulong)
            .wrapping_mul(nfft as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong),
    ) as *mut opus_val16;
    k = 0 as libc::c_int;
    while k < nfft {
        *in_0.offset(k as isize) =
            (rand() % 32768 as libc::c_int - 16384 as libc::c_int) as libc::c_float;
        k += 1;
    }
    k = 0 as libc::c_int;
    while k < nfft / 2 as libc::c_int {
        *window.offset(k as isize) = 1.0f32;
        k += 1;
    }
    k = 0 as libc::c_int;
    while k < nfft {
        *in_0.offset(k as isize) *= 32768 as libc::c_int as libc::c_float;
        k += 1;
    }
    if isinverse != 0 {
        k = 0 as libc::c_int;
        while k < nfft {
            *in_0.offset(k as isize) /= nfft as libc::c_float;
            k += 1;
        }
    }
    k = 0 as libc::c_int;
    while k < nfft {
        *in_copy.offset(k as isize) = *in_0.offset(k as isize);
        k += 1;
    }
    if isinverse != 0 {
        k = 0 as libc::c_int;
        while k < nfft {
            *out.offset(k as isize) = 0 as libc::c_int as libc::c_float;
            k += 1;
        }
        clt_mdct_backward_c(
            cfg,
            in_0,
            out,
            window,
            nfft / 2 as libc::c_int,
            shift,
            1 as libc::c_int,
            arch,
        );
        k = 0 as libc::c_int;
        while k < nfft / 4 as libc::c_int {
            *out.offset((nfft - k - 1 as libc::c_int) as isize) =
                *out.offset((nfft / 2 as libc::c_int + k) as isize);
            k += 1;
        }
        check_inv(in_0, out, nfft, isinverse);
    } else {
        clt_mdct_forward_c(
            cfg,
            in_0,
            out,
            window,
            nfft / 2 as libc::c_int,
            shift,
            1 as libc::c_int,
            arch,
        );
        check(in_copy, out, nfft, isinverse);
    }
    free(in_0 as *mut libc::c_void);
    free(in_copy as *mut libc::c_void);
    free(out as *mut libc::c_void);
    free(window as *mut libc::c_void);
}
#[c2rust::src_loc = "185:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut arch: libc::c_int = opus_select_arch();
    if argc > 1 as libc::c_int {
        let mut k: libc::c_int = 0;
        k = 1 as libc::c_int;
        while k < argc {
            test1d(atoi(*argv.offset(k as isize)), 0 as libc::c_int, arch);
            test1d(atoi(*argv.offset(k as isize)), 1 as libc::c_int, arch);
            k += 1;
        }
    } else {
        test1d(32 as libc::c_int, 0 as libc::c_int, arch);
        test1d(32 as libc::c_int, 1 as libc::c_int, arch);
        test1d(256 as libc::c_int, 0 as libc::c_int, arch);
        test1d(256 as libc::c_int, 1 as libc::c_int, arch);
        test1d(512 as libc::c_int, 0 as libc::c_int, arch);
        test1d(512 as libc::c_int, 1 as libc::c_int, arch);
        test1d(1024 as libc::c_int, 0 as libc::c_int, arch);
        test1d(1024 as libc::c_int, 1 as libc::c_int, arch);
        test1d(2048 as libc::c_int, 0 as libc::c_int, arch);
        test1d(2048 as libc::c_int, 1 as libc::c_int, arch);
        test1d(36 as libc::c_int, 0 as libc::c_int, arch);
        test1d(36 as libc::c_int, 1 as libc::c_int, arch);
        test1d(40 as libc::c_int, 0 as libc::c_int, arch);
        test1d(40 as libc::c_int, 1 as libc::c_int, arch);
        test1d(60 as libc::c_int, 0 as libc::c_int, arch);
        test1d(60 as libc::c_int, 1 as libc::c_int, arch);
        test1d(120 as libc::c_int, 0 as libc::c_int, arch);
        test1d(120 as libc::c_int, 1 as libc::c_int, arch);
        test1d(240 as libc::c_int, 0 as libc::c_int, arch);
        test1d(240 as libc::c_int, 1 as libc::c_int, arch);
        test1d(480 as libc::c_int, 0 as libc::c_int, arch);
        test1d(480 as libc::c_int, 1 as libc::c_int, arch);
        test1d(960 as libc::c_int, 0 as libc::c_int, arch);
        test1d(960 as libc::c_int, 1 as libc::c_int, arch);
        test1d(1920 as libc::c_int, 0 as libc::c_int, arch);
        test1d(1920 as libc::c_int, 1 as libc::c_int, arch);
    }
    return ret;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
