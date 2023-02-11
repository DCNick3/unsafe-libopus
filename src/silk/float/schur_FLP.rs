use ::libc;

use crate::celt::celt::celt_fatal;
#[c2rust::src_loc = "34:1"]
pub unsafe fn silk_schur_FLP(
    refl_coef: *mut libc::c_float,
    auto_corr: *const libc::c_float,
    order: libc::c_int,
) -> libc::c_float {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut C: [[libc::c_double; 2]; 25] = [[0.; 2]; 25];
    let mut Ctmp1: libc::c_double = 0.;
    let mut Ctmp2: libc::c_double = 0.;
    let mut rc_tmp: libc::c_double = 0.;
    if !(order >= 0 as libc::c_int && order <= 24 as libc::c_int) {
        celt_fatal(
            b"assertion failed: order >= 0 && order <= SILK_MAX_ORDER_LPC\0" as *const u8
                as *const libc::c_char,
            b"silk/float/schur_FLP.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
        );
    }
    k = 0 as libc::c_int;
    loop {
        C[k as usize][1 as libc::c_int as usize] = *auto_corr.offset(k as isize) as libc::c_double;
        C[k as usize][0 as libc::c_int as usize] = C[k as usize][1 as libc::c_int as usize];
        k += 1;
        if !(k <= order) {
            break;
        }
    }
    k = 0 as libc::c_int;
    while k < order {
        rc_tmp = -C[(k + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
            / (if C[0 as libc::c_int as usize][1 as libc::c_int as usize]
                > 1e-9f32 as libc::c_double
            {
                C[0 as libc::c_int as usize][1 as libc::c_int as usize]
            } else {
                1e-9f32 as libc::c_double
            });
        *refl_coef.offset(k as isize) = rc_tmp as libc::c_float;
        n = 0 as libc::c_int;
        while n < order - k {
            Ctmp1 = C[(n + k + 1 as libc::c_int) as usize][0 as libc::c_int as usize];
            Ctmp2 = C[n as usize][1 as libc::c_int as usize];
            C[(n + k + 1 as libc::c_int) as usize][0 as libc::c_int as usize] =
                Ctmp1 + Ctmp2 * rc_tmp;
            C[n as usize][1 as libc::c_int as usize] = Ctmp2 + Ctmp1 * rc_tmp;
            n += 1;
        }
        k += 1;
    }
    return C[0 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_float;
}
