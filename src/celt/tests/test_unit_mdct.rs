#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = u64;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:34"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = f32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/kiss_fft.h:34"]
pub mod kiss_fft_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:9"]
    pub struct kiss_twiddle_cpx {
        pub r: f32,
        pub i: f32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct arch_fft_state {
        pub is_supported: i32,
        pub priv_0: *mut core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:16"]
    pub struct kiss_fft_state {
        pub nfft: i32,
        pub scale: opus_val16,
        pub shift: i32,
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
        pub n: i32,
        pub maxshift: i32,
        pub kfft: [*const kiss_fft_state; 4],
        pub trig: *const f32,
    }
    use super::arch_h::opus_val16;
    use super::kiss_fft_h::kiss_fft_state;
    {
        #[c2rust::src_loc = "72:1"]
        pub fn clt_mdct_backward_c(
            l: *const mdct_lookup,
            in_0: *mut f32,
            out: *mut f32,
            window: *const opus_val16,
            overlap: i32,
            shift: i32,
            stride: i32,
            arch: i32,
        );
        #[c2rust::src_loc = "65:1"]
        pub fn clt_mdct_forward_c(
            l: *const mdct_lookup,
            in_0: *mut f32,
            out: *mut f32,
            window: *const opus_val16,
            overlap: i32,
            shift: i32,
            stride: i32,
            arch: i32,
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
        pub overlap: i32,
        pub nbEBands: i32,
        pub effEBands: i32,
        pub preemph: [opus_val16; 4],
        pub eBands: *const i16,
        pub maxLM: i32,
        pub nbShortMdcts: i32,
        pub shortMdctSize: i32,
        pub nbAllocVectors: i32,
        pub allocVectors: *const u8,
        pub logN: *const i16,
        pub window: *const opus_val16,
        pub mdct: mdct_lookup,
        pub cache: PulseCache,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:9"]
    pub struct PulseCache {
        pub size: i32,
        pub index: *const i16,
        pub bits: *const u8,
        pub caps: *const u8,
    }
    use super::arch_h::opus_val16;
    use crate::celt::mdct::mdct_lookup;
}
#[c2rust::header_src = "/usr/include/stdio.h:32"]
pub mod stdio_h {
    {
        #[c2rust::src_loc = "356:12"]
        pub fn printf(_: *const i8, _: ...) -> i32;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:34"]
pub mod mathcalls_h {
    {
        #[c2rust::src_loc = "62:17"]
        pub fn cos(_: f64) -> f64;
        #[c2rust::src_loc = "107:17"]
        pub fn log10(_: f64) -> f64;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:34"]
pub mod stdlib_h {
    #[inline]
    #[c2rust::src_loc = "361:1"]
    pub unsafe fn atoi(mut __nptr: *const i8) -> i32 {
        return strtol(
            __nptr,
            0 as *mut core::ffi::c_void as *mut *mut i8,
            10 as i32,
        ) as i32;
    }
    {
        #[c2rust::src_loc = "454:1"]
        pub fn rand() -> i32;
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: u64) -> *mut core::ffi::c_void;
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut core::ffi::c_void);
        #[c2rust::src_loc = "177:17"]
        pub fn strtol(
            _: *const i8,
            _: *mut *mut i8,
            _: i32,
        ) -> i64;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cpu_support.h:34"]
pub mod cpu_support_h {
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe fn opus_select_arch() -> i32 {
        return 0 as i32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_custom.h:38"]
pub mod opus_custom_h {
    use super::modes_h::OpusCustomMode;
    {
        #[c2rust::src_loc = "121:20"]
        pub fn opus_custom_mode_create(
            Fs: i32,
            frame_size: i32,
            error: *mut i32,
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
pub static mut ret: i32 = 0 as i32;
#[c2rust::src_loc = "45:1"]
pub unsafe fn check(
    mut in_0: *mut f32,
    mut out: *mut f32,
    mut nfft: i32,
    mut isinverse: i32,
) {
    let mut bin: i32 = 0;
    let mut k: i32 = 0;
    let mut errpow: f64 = 0 as i32 as f64;
    let mut sigpow: f64 = 0 as i32 as f64;
    let mut snr: f64 = 0.;
    bin = 0 as i32;
    while bin < nfft / 2 as i32 {
        let mut ansr: f64 = 0 as i32 as f64;
        let mut difr: f64 = 0.;
        k = 0 as i32;
        while k < nfft {
            let mut phase: f64 = 2 as i32 as f64
                * 3.14159265358979323846f64
                * (k as f64 + 0.5f64 + 0.25f64 * nfft as f64)
                * (bin as f64 + 0.5f64)
                / nfft as f64;
            let mut re: f64 = cos(phase);
            re /= (nfft / 4 as i32) as f64;
            ansr += *in_0.offset(k as isize) as f64 * re;
            k += 1;
        }
        difr = ansr - *out.offset(bin as isize) as f64;
        errpow += difr * difr;
        sigpow += ansr * ansr;
        bin += 1;
    }
    snr = 10 as i32 as f64 * log10(sigpow / errpow);
    printf(
        b"nfft=%d inverse=%d,snr = %f\n\0" as *const u8 as *const i8,
        nfft,
        isinverse,
        snr,
    );
    if snr < 60 as i32 as f64 {
        printf(
            b"** poor snr: %f **\n\0" as *const u8 as *const i8,
            snr,
        );
        ret = 1 as i32;
    }
}
#[c2rust::src_loc = "75:1"]
pub unsafe fn check_inv(
    mut in_0: *mut f32,
    mut out: *mut f32,
    mut nfft: i32,
    mut isinverse: i32,
) {
    let mut bin: i32 = 0;
    let mut k: i32 = 0;
    let mut errpow: f64 = 0 as i32 as f64;
    let mut sigpow: f64 = 0 as i32 as f64;
    let mut snr: f64 = 0.;
    bin = 0 as i32;
    while bin < nfft {
        let mut ansr: f64 = 0 as i32 as f64;
        let mut difr: f64 = 0.;
        k = 0 as i32;
        while k < nfft / 2 as i32 {
            let mut phase: f64 = 2 as i32 as f64
                * 3.14159265358979323846f64
                * (bin as f64 + 0.5f64 + 0.25f64 * nfft as f64)
                * (k as f64 + 0.5f64)
                / nfft as f64;
            let mut re: f64 = cos(phase);
            ansr += *in_0.offset(k as isize) as f64 * re;
            k += 1;
        }
        difr = ansr - *out.offset(bin as isize) as f64;
        errpow += difr * difr;
        sigpow += ansr * ansr;
        bin += 1;
    }
    snr = 10 as i32 as f64 * log10(sigpow / errpow);
    printf(
        b"nfft=%d inverse=%d,snr = %f\n\0" as *const u8 as *const i8,
        nfft,
        isinverse,
        snr,
    );
    if snr < 60 as i32 as f64 {
        printf(
            b"** poor snr: %f **\n\0" as *const u8 as *const i8,
            snr,
        );
        ret = 1 as i32;
    }
}
#[c2rust::src_loc = "106:1"]
pub unsafe fn test1d(
    mut nfft: i32,
    mut isinverse: i32,
    mut arch: i32,
) {
    let mut buflen: size_t = (::core::mem::size_of::<f32>() as u64)
        .wrapping_mul(nfft as u64);
    let mut in_0: *mut f32 = 0 as *mut f32;
    let mut in_copy: *mut f32 = 0 as *mut f32;
    let mut out: *mut f32 = 0 as *mut f32;
    let mut window: *mut opus_val16 = 0 as *mut opus_val16;
    let mut k: i32 = 0;
    let mut shift: i32 = 0;
    let mut cfg: *const mdct_lookup = 0 as *const mdct_lookup;
    let mut mode: *mut OpusCustomMode = opus_custom_mode_create(
        48000 as i32,
        960 as i32,
        0 as *mut i32,
    );
    if nfft == 1920 as i32 {
        shift = 0 as i32;
    } else if nfft == 960 as i32 {
        shift = 1 as i32;
    } else if nfft == 480 as i32 {
        shift = 2 as i32;
    } else if nfft == 240 as i32 {
        shift = 3 as i32;
    } else {
        return;
    }
    cfg = &mut (*mode).mdct;
    in_0 = malloc(buflen) as *mut f32;
    in_copy = malloc(buflen) as *mut f32;
    out = malloc(buflen) as *mut f32;
    window = malloc(
        (::core::mem::size_of::<opus_val16>() as u64)
            .wrapping_mul(nfft as u64)
            .wrapping_div(2 as i32 as u64),
    ) as *mut opus_val16;
    k = 0 as i32;
    while k < nfft {
        *in_0.offset(k as isize) =
            (rand() % 32768 as i32 - 16384 as i32) as f32;
        k += 1;
    }
    k = 0 as i32;
    while k < nfft / 2 as i32 {
        *window.offset(k as isize) = 1.0f32;
        k += 1;
    }
    k = 0 as i32;
    while k < nfft {
        *in_0.offset(k as isize) *= 32768 as i32 as f32;
        k += 1;
    }
    if isinverse != 0 {
        k = 0 as i32;
        while k < nfft {
            *in_0.offset(k as isize) /= nfft as f32;
            k += 1;
        }
    }
    k = 0 as i32;
    while k < nfft {
        *in_copy.offset(k as isize) = *in_0.offset(k as isize);
        k += 1;
    }
    if isinverse != 0 {
        k = 0 as i32;
        while k < nfft {
            *out.offset(k as isize) = 0 as i32 as f32;
            k += 1;
        }
        clt_mdct_backward_c(
            cfg,
            in_0,
            out,
            window,
            nfft / 2 as i32,
            shift,
            1 as i32,
            arch,
        );
        k = 0 as i32;
        while k < nfft / 4 as i32 {
            *out.offset((nfft - k - 1 as i32) as isize) =
                *out.offset((nfft / 2 as i32 + k) as isize);
            k += 1;
        }
        check_inv(in_0, out, nfft, isinverse);
    } else {
        clt_mdct_forward_c(
            cfg,
            in_0,
            out,
            window,
            nfft / 2 as i32,
            shift,
            1 as i32,
            arch,
        );
        check(in_copy, out, nfft, isinverse);
    }
    free(in_0 as *mut core::ffi::c_void);
    free(in_copy as *mut core::ffi::c_void);
    free(out as *mut core::ffi::c_void);
    free(window as *mut core::ffi::c_void);
}
#[c2rust::src_loc = "185:1"]
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut arch: i32 = opus_select_arch();
    if argc > 1 as i32 {
        let mut k: i32 = 0;
        k = 1 as i32;
        while k < argc {
            test1d(atoi(*argv.offset(k as isize)), 0 as i32, arch);
            test1d(atoi(*argv.offset(k as isize)), 1 as i32, arch);
            k += 1;
        }
    } else {
        test1d(32 as i32, 0 as i32, arch);
        test1d(32 as i32, 1 as i32, arch);
        test1d(256 as i32, 0 as i32, arch);
        test1d(256 as i32, 1 as i32, arch);
        test1d(512 as i32, 0 as i32, arch);
        test1d(512 as i32, 1 as i32, arch);
        test1d(1024 as i32, 0 as i32, arch);
        test1d(1024 as i32, 1 as i32, arch);
        test1d(2048 as i32, 0 as i32, arch);
        test1d(2048 as i32, 1 as i32, arch);
        test1d(36 as i32, 0 as i32, arch);
        test1d(36 as i32, 1 as i32, arch);
        test1d(40 as i32, 0 as i32, arch);
        test1d(40 as i32, 1 as i32, arch);
        test1d(60 as i32, 0 as i32, arch);
        test1d(60 as i32, 1 as i32, arch);
        test1d(120 as i32, 0 as i32, arch);
        test1d(120 as i32, 1 as i32, arch);
        test1d(240 as i32, 0 as i32, arch);
        test1d(240 as i32, 1 as i32, arch);
        test1d(480 as i32, 0 as i32, arch);
        test1d(480 as i32, 1 as i32, arch);
        test1d(960 as i32, 0 as i32, arch);
        test1d(960 as i32, 1 as i32, arch);
        test1d(1920 as i32, 0 as i32, arch);
        test1d(1920 as i32, 1 as i32, arch);
    }
    return ret;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
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
            (args.len() - 1) as i32,
            args.as_mut_ptr() as *mut *mut i8,
        ) as i32)
    }
}
