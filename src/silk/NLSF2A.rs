use crate::celt::celt::celt_fatal;
use crate::silk::bwexpander_32::silk_bwexpander_32;
use crate::silk::define::MAX_LPC_STABILIZE_ITERATIONS;
use crate::silk::table_LSF_cos::silk_LSFCosTab_FIX_Q12;
use crate::silk::LPC_fit::silk_LPC_fit;
use crate::silk::LPC_inv_pred_gain::silk_LPC_inverse_pred_gain_c;

#[c2rust::src_loc = "41:9"]
pub const QA: i32 = 16 as i32;
#[inline]
#[c2rust::src_loc = "44:1"]
unsafe fn silk_NLSF2A_find_poly(out: *mut i32, cLSF: *const i32, dd: i32) {
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut ftmp: i32 = 0;
    *out.offset(0 as i32 as isize) = ((1 as i32 as u32) << 16 as i32) as i32;
    *out.offset(1 as i32 as isize) = -*cLSF.offset(0 as i32 as isize);
    k = 1 as i32;
    while k < dd {
        ftmp = *cLSF.offset((2 as i32 * k) as isize);
        *out.offset((k + 1 as i32) as isize) = ((*out.offset((k - 1 as i32) as isize) as u32)
            << 1 as i32) as i32
            - (if 16 as i32 == 1 as i32 {
                (ftmp as i64 * *out.offset(k as isize) as i64 >> 1 as i32)
                    + (ftmp as i64 * *out.offset(k as isize) as i64 & 1 as i32 as i64)
            } else {
                (ftmp as i64 * *out.offset(k as isize) as i64 >> 16 as i32 - 1 as i32)
                    + 1 as i32 as i64
                    >> 1 as i32
            }) as i32;
        n = k;
        while n > 1 as i32 {
            let ref mut fresh0 = *out.offset(n as isize);
            *fresh0 += *out.offset((n - 2 as i32) as isize)
                - (if 16 as i32 == 1 as i32 {
                    (ftmp as i64 * *out.offset((n - 1 as i32) as isize) as i64 >> 1 as i32)
                        + (ftmp as i64 * *out.offset((n - 1 as i32) as isize) as i64
                            & 1 as i32 as i64)
                } else {
                    (ftmp as i64 * *out.offset((n - 1 as i32) as isize) as i64
                        >> 16 as i32 - 1 as i32)
                        + 1 as i32 as i64
                        >> 1 as i32
                }) as i32;
            n -= 1;
        }
        let ref mut fresh1 = *out.offset(1 as i32 as isize);
        *fresh1 -= ftmp;
        k += 1;
    }
}
#[c2rust::src_loc = "66:1"]
pub unsafe fn silk_NLSF2A(a_Q12: *mut i16, NLSF: *const i16, d: i32, _arch: i32) {
    static mut ordering16: [u8; 16] = [
        0 as i32 as u8,
        15 as i32 as u8,
        8 as i32 as u8,
        7 as i32 as u8,
        4 as i32 as u8,
        11 as i32 as u8,
        12 as i32 as u8,
        3 as i32 as u8,
        2 as i32 as u8,
        13 as i32 as u8,
        10 as i32 as u8,
        5 as i32 as u8,
        6 as i32 as u8,
        9 as i32 as u8,
        14 as i32 as u8,
        1 as i32 as u8,
    ];
    static mut ordering10: [u8; 10] = [
        0 as i32 as u8,
        9 as i32 as u8,
        6 as i32 as u8,
        3 as i32 as u8,
        4 as i32 as u8,
        5 as i32 as u8,
        8 as i32 as u8,
        1 as i32 as u8,
        2 as i32 as u8,
        7 as i32 as u8,
    ];
    let mut ordering: *const u8 = 0 as *const u8;
    let mut k: i32 = 0;
    let mut i: i32 = 0;
    let mut dd: i32 = 0;
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
    if !(d == 10 as i32 || d == 16 as i32) {
        celt_fatal(
            b"assertion failed: d==10 || d==16\0" as *const u8 as *const i8,
            b"silk/NLSF2A.c\0" as *const u8 as *const i8,
            89 as i32,
        );
    }
    ordering = if d == 16 as i32 {
        ordering16.as_ptr()
    } else {
        ordering10.as_ptr()
    };
    k = 0 as i32;
    while k < d {
        f_int = *NLSF.offset(k as isize) as i32 >> 15 as i32 - 7 as i32;
        f_frac = *NLSF.offset(k as isize) as i32 - ((f_int as u32) << 15 as i32 - 7 as i32) as i32;
        cos_val = silk_LSFCosTab_FIX_Q12[f_int as usize] as i32;
        delta = silk_LSFCosTab_FIX_Q12[(f_int + 1 as i32) as usize] as i32 - cos_val;
        cos_LSF_QA[*ordering.offset(k as isize) as usize] = if 20 as i32 - 16 as i32 == 1 as i32 {
            (((cos_val as u32) << 8 as i32) as i32 + delta * f_frac >> 1 as i32)
                + (((cos_val as u32) << 8 as i32) as i32 + delta * f_frac & 1 as i32)
        } else {
            (((cos_val as u32) << 8 as i32) as i32 + delta * f_frac
                >> 20 as i32 - 16 as i32 - 1 as i32)
                + 1 as i32
                >> 1 as i32
        };
        k += 1;
    }
    dd = d >> 1 as i32;
    silk_NLSF2A_find_poly(
        P.as_mut_ptr(),
        &mut *cos_LSF_QA.as_mut_ptr().offset(0 as i32 as isize),
        dd,
    );
    silk_NLSF2A_find_poly(
        Q.as_mut_ptr(),
        &mut *cos_LSF_QA.as_mut_ptr().offset(1 as i32 as isize),
        dd,
    );
    k = 0 as i32;
    while k < dd {
        Ptmp = P[(k + 1 as i32) as usize] + P[k as usize];
        Qtmp = Q[(k + 1 as i32) as usize] - Q[k as usize];
        a32_QA1[k as usize] = -Qtmp - Ptmp;
        a32_QA1[(d - k - 1 as i32) as usize] = Qtmp - Ptmp;
        k += 1;
    }
    silk_LPC_fit(a_Q12, a32_QA1.as_mut_ptr(), 12 as i32, QA + 1 as i32, d);
    i = 0 as i32;
    while silk_LPC_inverse_pred_gain_c(a_Q12, d) == 0 as i32 && i < MAX_LPC_STABILIZE_ITERATIONS {
        silk_bwexpander_32(
            a32_QA1.as_mut_ptr(),
            d,
            65536 as i32 - ((2 as i32 as u32) << i) as i32,
        );
        k = 0 as i32;
        while k < d {
            *a_Q12.offset(k as isize) = (if 16 as i32 + 1 as i32 - 12 as i32 == 1 as i32 {
                (a32_QA1[k as usize] >> 1 as i32) + (a32_QA1[k as usize] & 1 as i32)
            } else {
                (a32_QA1[k as usize] >> 16 as i32 + 1 as i32 - 12 as i32 - 1 as i32) + 1 as i32
                    >> 1 as i32
            }) as i16;
            k += 1;
        }
        i += 1;
    }
}
