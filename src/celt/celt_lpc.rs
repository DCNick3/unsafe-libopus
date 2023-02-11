use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:32"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
}
pub use self::arch_h::{opus_val16, opus_val32};
use crate::celt::celt::celt_fatal;
use crate::celt::pitch::{celt_pitch_xcorr_c, xcorr_kernel_c};
use crate::externs::memset;

#[c2rust::src_loc = "38:9"]
pub const LPC_ORDER: libc::c_int = 24 as libc::c_int;

#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn _celt_lpc(
    mut _lpc: *mut opus_val16,
    ac: *const opus_val32,
    p: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r: opus_val32 = 0.;
    let mut error: opus_val32 = *ac.offset(0 as libc::c_int as isize);
    let lpc: *mut libc::c_float = _lpc;
    memset(
        lpc as *mut libc::c_void,
        0 as libc::c_int,
        (p as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    if *ac.offset(0 as libc::c_int as isize) != 0 as libc::c_int as libc::c_float {
        i = 0 as libc::c_int;
        while i < p {
            let mut rr: opus_val32 = 0 as libc::c_int as opus_val32;
            j = 0 as libc::c_int;
            while j < i {
                rr += *lpc.offset(j as isize) * *ac.offset((i - j) as isize);
                j += 1;
            }
            rr += *ac.offset((i + 1 as libc::c_int) as isize);
            r = -(rr / error);
            *lpc.offset(i as isize) = r;
            j = 0 as libc::c_int;
            while j < i + 1 as libc::c_int >> 1 as libc::c_int {
                let mut tmp1: opus_val32 = 0.;
                let mut tmp2: opus_val32 = 0.;
                tmp1 = *lpc.offset(j as isize);
                tmp2 = *lpc.offset((i - 1 as libc::c_int - j) as isize);
                *lpc.offset(j as isize) = tmp1 + r * tmp2;
                *lpc.offset((i - 1 as libc::c_int - j) as isize) = tmp2 + r * tmp1;
                j += 1;
            }
            error = error - r * r * error;
            if error < 0.001f32 * *ac.offset(0 as libc::c_int as isize) {
                break;
            }
            i += 1;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "91:1"]
pub unsafe extern "C" fn celt_fir_c(
    x: *const opus_val16,
    num: *const opus_val16,
    y: *mut opus_val16,
    N: libc::c_int,
    ord: libc::c_int,
    _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !(x != y as *const opus_val16) {
        celt_fatal(
            b"assertion failed: x != y\0" as *const u8 as *const libc::c_char,
            b"celt/celt_lpc.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
        );
    }
    let vla = ord as usize;
    let mut rnum: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    i = 0 as libc::c_int;
    while i < ord {
        *rnum.as_mut_ptr().offset(i as isize) = *num.offset((ord - i - 1 as libc::c_int) as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < N - 3 as libc::c_int {
        let mut sum: [opus_val32; 4] = [0.; 4];
        sum[0 as libc::c_int as usize] = *x.offset(i as isize);
        sum[1 as libc::c_int as usize] = *x.offset((i + 1 as libc::c_int) as isize);
        sum[2 as libc::c_int as usize] = *x.offset((i + 2 as libc::c_int) as isize);
        sum[3 as libc::c_int as usize] = *x.offset((i + 3 as libc::c_int) as isize);
        xcorr_kernel_c(
            rnum.as_mut_ptr(),
            x.offset(i as isize).offset(-(ord as isize)),
            sum.as_mut_ptr(),
            ord,
        );
        *y.offset(i as isize) = sum[0 as libc::c_int as usize];
        *y.offset((i + 1 as libc::c_int) as isize) = sum[1 as libc::c_int as usize];
        *y.offset((i + 2 as libc::c_int) as isize) = sum[2 as libc::c_int as usize];
        *y.offset((i + 3 as libc::c_int) as isize) = sum[3 as libc::c_int as usize];
        i += 4 as libc::c_int;
    }
    while i < N {
        let mut sum_0: opus_val32 = *x.offset(i as isize);
        j = 0 as libc::c_int;
        while j < ord {
            sum_0 =
                sum_0 + *rnum.as_mut_ptr().offset(j as isize) * *x.offset((i + j - ord) as isize);
            j += 1;
        }
        *y.offset(i as isize) = sum_0;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn celt_iir(
    mut _x: *const opus_val32,
    den: *const opus_val16,
    mut _y: *mut opus_val32,
    N: libc::c_int,
    ord: libc::c_int,
    mem: *mut opus_val16,
    _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !(ord & 3 as libc::c_int == 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: (ord&3)==0\0" as *const u8 as *const libc::c_char,
            b"celt/celt_lpc.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
        );
    }
    let vla = ord as usize;
    let mut rden: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    let vla_0 = (N + ord) as usize;
    let mut y: Vec<opus_val16> = ::std::vec::from_elem(0., vla_0);
    i = 0 as libc::c_int;
    while i < ord {
        *rden.as_mut_ptr().offset(i as isize) = *den.offset((ord - i - 1 as libc::c_int) as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < ord {
        *y.as_mut_ptr().offset(i as isize) = -*mem.offset((ord - i - 1 as libc::c_int) as isize);
        i += 1;
    }
    while i < N + ord {
        *y.as_mut_ptr().offset(i as isize) = 0 as libc::c_int as opus_val16;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < N - 3 as libc::c_int {
        let mut sum: [opus_val32; 4] = [0.; 4];
        sum[0 as libc::c_int as usize] = *_x.offset(i as isize);
        sum[1 as libc::c_int as usize] = *_x.offset((i + 1 as libc::c_int) as isize);
        sum[2 as libc::c_int as usize] = *_x.offset((i + 2 as libc::c_int) as isize);
        sum[3 as libc::c_int as usize] = *_x.offset((i + 3 as libc::c_int) as isize);
        xcorr_kernel_c(
            rden.as_mut_ptr(),
            y.as_mut_ptr().offset(i as isize),
            sum.as_mut_ptr(),
            ord,
        );
        *y.as_mut_ptr().offset((i + ord) as isize) = -sum[0 as libc::c_int as usize];
        *_y.offset(i as isize) = sum[0 as libc::c_int as usize];
        sum[1 as libc::c_int as usize] = sum[1 as libc::c_int as usize]
            + *y.as_mut_ptr().offset((i + ord) as isize) * *den.offset(0 as libc::c_int as isize);
        *y.as_mut_ptr().offset((i + ord + 1 as libc::c_int) as isize) =
            -sum[1 as libc::c_int as usize];
        *_y.offset((i + 1 as libc::c_int) as isize) = sum[1 as libc::c_int as usize];
        sum[2 as libc::c_int as usize] = sum[2 as libc::c_int as usize]
            + *y.as_mut_ptr().offset((i + ord + 1 as libc::c_int) as isize)
                * *den.offset(0 as libc::c_int as isize);
        sum[2 as libc::c_int as usize] = sum[2 as libc::c_int as usize]
            + *y.as_mut_ptr().offset((i + ord) as isize) * *den.offset(1 as libc::c_int as isize);
        *y.as_mut_ptr().offset((i + ord + 2 as libc::c_int) as isize) =
            -sum[2 as libc::c_int as usize];
        *_y.offset((i + 2 as libc::c_int) as isize) = sum[2 as libc::c_int as usize];
        sum[3 as libc::c_int as usize] = sum[3 as libc::c_int as usize]
            + *y.as_mut_ptr().offset((i + ord + 2 as libc::c_int) as isize)
                * *den.offset(0 as libc::c_int as isize);
        sum[3 as libc::c_int as usize] = sum[3 as libc::c_int as usize]
            + *y.as_mut_ptr().offset((i + ord + 1 as libc::c_int) as isize)
                * *den.offset(1 as libc::c_int as isize);
        sum[3 as libc::c_int as usize] = sum[3 as libc::c_int as usize]
            + *y.as_mut_ptr().offset((i + ord) as isize) * *den.offset(2 as libc::c_int as isize);
        *y.as_mut_ptr().offset((i + ord + 3 as libc::c_int) as isize) =
            -sum[3 as libc::c_int as usize];
        *_y.offset((i + 3 as libc::c_int) as isize) = sum[3 as libc::c_int as usize];
        i += 4 as libc::c_int;
    }
    while i < N {
        let mut sum_0: opus_val32 = *_x.offset(i as isize);
        j = 0 as libc::c_int;
        while j < ord {
            sum_0 -=
                *rden.as_mut_ptr().offset(j as isize) * *y.as_mut_ptr().offset((i + j) as isize);
            j += 1;
        }
        *y.as_mut_ptr().offset((i + ord) as isize) = sum_0;
        *_y.offset(i as isize) = sum_0;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < ord {
        *mem.offset(i as isize) = *_y.offset((N - i - 1 as libc::c_int) as isize);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "210:1"]
pub unsafe extern "C" fn _celt_autocorr(
    x: *const opus_val16,
    ac: *mut opus_val32,
    window: *const opus_val16,
    overlap: libc::c_int,
    lag: libc::c_int,
    n: libc::c_int,
    arch: libc::c_int,
) -> libc::c_int {
    let mut d: opus_val32 = 0.;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let fastN: libc::c_int = n - lag;
    let mut shift: libc::c_int = 0;
    let mut xptr: *const opus_val16 = 0 as *const opus_val16;
    let vla = n as usize;
    let mut xx: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    if !(n > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: n>0\0" as *const u8 as *const libc::c_char,
            b"celt/celt_lpc.c\0" as *const u8 as *const libc::c_char,
            228 as libc::c_int,
        );
    }
    if !(overlap >= 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: overlap>=0\0" as *const u8 as *const libc::c_char,
            b"celt/celt_lpc.c\0" as *const u8 as *const libc::c_char,
            229 as libc::c_int,
        );
    }
    if overlap == 0 as libc::c_int {
        xptr = x;
    } else {
        i = 0 as libc::c_int;
        while i < n {
            *xx.as_mut_ptr().offset(i as isize) = *x.offset(i as isize);
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < overlap {
            *xx.as_mut_ptr().offset(i as isize) =
                *x.offset(i as isize) * *window.offset(i as isize);
            *xx.as_mut_ptr().offset((n - i - 1 as libc::c_int) as isize) =
                *x.offset((n - i - 1 as libc::c_int) as isize) * *window.offset(i as isize);
            i += 1;
        }
        xptr = xx.as_mut_ptr();
    }
    shift = 0 as libc::c_int;
    celt_pitch_xcorr_c(xptr, xptr, ac, fastN, lag + 1 as libc::c_int, arch);
    k = 0 as libc::c_int;
    while k <= lag {
        i = k + fastN;
        d = 0 as libc::c_int as opus_val32;
        while i < n {
            d = d + *xptr.offset(i as isize) * *xptr.offset((i - k) as isize);
            i += 1;
        }
        let ref mut fresh19 = *ac.offset(k as isize);
        *fresh19 += d;
        k += 1;
    }
    return shift;
}
