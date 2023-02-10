use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:38"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:38"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:38"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:38"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:38"]
pub mod SigProc_FIX_h {
    extern "C" {
        #[c2rust::src_loc = "140:1"]
        pub fn silk_bwexpander_32(ar: *mut i32, d: libc::c_int, chirp_Q16: i32);
        #[c2rust::src_loc = "148:1"]
        pub fn silk_LPC_inverse_pred_gain_c(A_Q12: *const i16, order: libc::c_int) -> i32;
        #[c2rust::src_loc = "294:1"]
        pub fn silk_LPC_fit(
            a_QOUT: *mut i16,
            a_QIN: *mut i32,
            QOUT: libc::c_int,
            QIN: libc::c_int,
            d: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:39"]
pub mod define_h {
    #[c2rust::src_loc = "138:9"]
    pub const MAX_LPC_STABILIZE_ITERATIONS: libc::c_int = 16 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:39"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "108:26"]
        pub static silk_LSFCosTab_FIX_Q12: [i16; 129];
    }
}
use self::arch_h::celt_fatal;
pub use self::define_h::MAX_LPC_STABILIZE_ITERATIONS;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
use self::tables_h::silk_LSFCosTab_FIX_Q12;
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
use self::SigProc_FIX_h::{silk_LPC_fit, silk_LPC_inverse_pred_gain_c, silk_bwexpander_32};
#[c2rust::src_loc = "41:9"]
pub const QA: libc::c_int = 16 as libc::c_int;
#[inline]
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn silk_NLSF2A_find_poly(out: *mut i32, cLSF: *const i32, dd: libc::c_int) {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ftmp: i32 = 0;
    *out.offset(0 as libc::c_int as isize) =
        ((1 as libc::c_int as u32) << 16 as libc::c_int) as i32;
    *out.offset(1 as libc::c_int as isize) = -*cLSF.offset(0 as libc::c_int as isize);
    k = 1 as libc::c_int;
    while k < dd {
        ftmp = *cLSF.offset((2 as libc::c_int * k) as isize);
        *out.offset((k + 1 as libc::c_int) as isize) =
            ((*out.offset((k - 1 as libc::c_int) as isize) as u32) << 1 as libc::c_int) as i32
                - (if 16 as libc::c_int == 1 as libc::c_int {
                    (ftmp as i64 * *out.offset(k as isize) as libc::c_long >> 1 as libc::c_int)
                        + (ftmp as i64 * *out.offset(k as isize) as libc::c_long
                            & 1 as libc::c_int as libc::c_long)
                } else {
                    (ftmp as i64 * *out.offset(k as isize) as libc::c_long
                        >> 16 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int as libc::c_long
                        >> 1 as libc::c_int
                }) as i32;
        n = k;
        while n > 1 as libc::c_int {
            let ref mut fresh0 = *out.offset(n as isize);
            *fresh0 += *out.offset((n - 2 as libc::c_int) as isize)
                - (if 16 as libc::c_int == 1 as libc::c_int {
                    (ftmp as i64 * *out.offset((n - 1 as libc::c_int) as isize) as libc::c_long
                        >> 1 as libc::c_int)
                        + (ftmp as i64
                            * *out.offset((n - 1 as libc::c_int) as isize) as libc::c_long
                            & 1 as libc::c_int as libc::c_long)
                } else {
                    (ftmp as i64 * *out.offset((n - 1 as libc::c_int) as isize) as libc::c_long
                        >> 16 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int as libc::c_long
                        >> 1 as libc::c_int
                }) as i32;
            n -= 1;
        }
        let ref mut fresh1 = *out.offset(1 as libc::c_int as isize);
        *fresh1 -= ftmp;
        k += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn silk_NLSF2A(
    a_Q12: *mut i16,
    NLSF: *const i16,
    d: libc::c_int,
    _arch: libc::c_int,
) {
    static mut ordering16: [libc::c_uchar; 16] = [
        0 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
    ];
    static mut ordering10: [libc::c_uchar; 10] = [
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
    ];
    let mut ordering: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut dd: libc::c_int = 0;
    let mut cos_LSF_QA: [i32; 24] = [0; 24];
    let mut P: [i32; 13] = [0; 13];
    let mut Q: [i32; 13] = [0; 13];
    let mut Ptmp: i32 = 0;
    let mut Qtmp: i32 = 0;
    let mut f_int: i32 = 0;
    let mut f_frac: i32 = 0;
    let mut cos_val: i32 = 0;
    let mut delta: i32 = 0;
    let mut a32_QA1: [i32; 24] = [0; 24];
    if !(d == 10 as libc::c_int || d == 16 as libc::c_int) {
        celt_fatal(
            b"assertion failed: d==10 || d==16\0" as *const u8 as *const libc::c_char,
            b"silk/NLSF2A.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
        );
    }
    ordering = if d == 16 as libc::c_int {
        ordering16.as_ptr()
    } else {
        ordering10.as_ptr()
    };
    k = 0 as libc::c_int;
    while k < d {
        f_int = *NLSF.offset(k as isize) as libc::c_int >> 15 as libc::c_int - 7 as libc::c_int;
        f_frac = *NLSF.offset(k as isize) as libc::c_int
            - ((f_int as u32) << 15 as libc::c_int - 7 as libc::c_int) as i32;
        cos_val = silk_LSFCosTab_FIX_Q12[f_int as usize] as i32;
        delta =
            silk_LSFCosTab_FIX_Q12[(f_int + 1 as libc::c_int) as usize] as libc::c_int - cos_val;
        cos_LSF_QA[*ordering.offset(k as isize) as usize] =
            if 20 as libc::c_int - 16 as libc::c_int == 1 as libc::c_int {
                (((cos_val as u32) << 8 as libc::c_int) as i32 + delta * f_frac >> 1 as libc::c_int)
                    + (((cos_val as u32) << 8 as libc::c_int) as i32 + delta * f_frac
                        & 1 as libc::c_int)
            } else {
                (((cos_val as u32) << 8 as libc::c_int) as i32 + delta * f_frac
                    >> 20 as libc::c_int - 16 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            };
        k += 1;
    }
    dd = d >> 1 as libc::c_int;
    silk_NLSF2A_find_poly(
        P.as_mut_ptr(),
        &mut *cos_LSF_QA.as_mut_ptr().offset(0 as libc::c_int as isize),
        dd,
    );
    silk_NLSF2A_find_poly(
        Q.as_mut_ptr(),
        &mut *cos_LSF_QA.as_mut_ptr().offset(1 as libc::c_int as isize),
        dd,
    );
    k = 0 as libc::c_int;
    while k < dd {
        Ptmp = P[(k + 1 as libc::c_int) as usize] + P[k as usize];
        Qtmp = Q[(k + 1 as libc::c_int) as usize] - Q[k as usize];
        a32_QA1[k as usize] = -Qtmp - Ptmp;
        a32_QA1[(d - k - 1 as libc::c_int) as usize] = Qtmp - Ptmp;
        k += 1;
    }
    silk_LPC_fit(
        a_Q12,
        a32_QA1.as_mut_ptr(),
        12 as libc::c_int,
        QA + 1 as libc::c_int,
        d,
    );
    i = 0 as libc::c_int;
    while silk_LPC_inverse_pred_gain_c(a_Q12, d) == 0 as libc::c_int
        && i < MAX_LPC_STABILIZE_ITERATIONS
    {
        silk_bwexpander_32(
            a32_QA1.as_mut_ptr(),
            d,
            65536 as libc::c_int - ((2 as libc::c_int as u32) << i) as i32,
        );
        k = 0 as libc::c_int;
        while k < d {
            *a_Q12.offset(k as isize) = (if 16 as libc::c_int + 1 as libc::c_int - 12 as libc::c_int
                == 1 as libc::c_int
            {
                (a32_QA1[k as usize] >> 1 as libc::c_int) + (a32_QA1[k as usize] & 1 as libc::c_int)
            } else {
                (a32_QA1[k as usize]
                    >> 16 as libc::c_int + 1 as libc::c_int - 12 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as i16;
            k += 1;
        }
        i += 1;
    }
}
