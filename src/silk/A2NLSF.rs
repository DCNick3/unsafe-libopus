use crate::silk::bwexpander_32::silk_bwexpander_32;
use crate::silk::SigProc_FIX::silk_min_32;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:38"]
pub mod typedef_h {
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
pub use self::typedef_h::silk_int16_MAX;
use crate::silk::define::LSF_COS_TAB_SZ_FIX;
use crate::silk::table_LSF_cos::silk_LSFCosTab_FIX_Q12;

#[c2rust::src_loc = "42:9"]
pub const BIN_DIV_STEPS_A2NLSF_FIX: libc::c_int = 3 as libc::c_int;
#[c2rust::src_loc = "43:9"]
pub const MAX_ITERATIONS_A2NLSF_FIX: libc::c_int = 16 as libc::c_int;
#[inline]
#[c2rust::src_loc = "47:1"]
unsafe fn silk_A2NLSF_trans_poly(p: *mut i32, dd: libc::c_int) {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    k = 2 as libc::c_int;
    while k <= dd {
        n = dd;
        while n > k {
            let ref mut fresh0 = *p.offset((n - 2 as libc::c_int) as isize);
            *fresh0 -= *p.offset(n as isize);
            n -= 1;
        }
        let ref mut fresh1 = *p.offset((k - 2 as libc::c_int) as isize);
        *fresh1 -= ((*p.offset(k as isize) as u32) << 1 as libc::c_int) as i32;
        k += 1;
    }
}
#[inline]
#[c2rust::src_loc = "63:1"]
unsafe fn silk_A2NLSF_eval_poly(p: *mut i32, x: i32, dd: libc::c_int) -> i32 {
    let mut n: libc::c_int = 0;
    let mut x_Q16: i32 = 0;
    let mut y32: i32 = 0;
    y32 = *p.offset(dd as isize);
    x_Q16 = ((x as u32) << 4 as libc::c_int) as i32;
    if (8 as libc::c_int == dd) as libc::c_int as libc::c_long != 0 {
        y32 = (*p.offset(7 as libc::c_int as isize) as libc::c_long
            + (y32 as i64 * x_Q16 as libc::c_long >> 16 as libc::c_int)) as i32;
        y32 = (*p.offset(6 as libc::c_int as isize) as libc::c_long
            + (y32 as i64 * x_Q16 as libc::c_long >> 16 as libc::c_int)) as i32;
        y32 = (*p.offset(5 as libc::c_int as isize) as libc::c_long
            + (y32 as i64 * x_Q16 as libc::c_long >> 16 as libc::c_int)) as i32;
        y32 = (*p.offset(4 as libc::c_int as isize) as libc::c_long
            + (y32 as i64 * x_Q16 as libc::c_long >> 16 as libc::c_int)) as i32;
        y32 = (*p.offset(3 as libc::c_int as isize) as libc::c_long
            + (y32 as i64 * x_Q16 as libc::c_long >> 16 as libc::c_int)) as i32;
        y32 = (*p.offset(2 as libc::c_int as isize) as libc::c_long
            + (y32 as i64 * x_Q16 as libc::c_long >> 16 as libc::c_int)) as i32;
        y32 = (*p.offset(1 as libc::c_int as isize) as libc::c_long
            + (y32 as i64 * x_Q16 as libc::c_long >> 16 as libc::c_int)) as i32;
        y32 = (*p.offset(0 as libc::c_int as isize) as libc::c_long
            + (y32 as i64 * x_Q16 as libc::c_long >> 16 as libc::c_int)) as i32;
    } else {
        n = dd - 1 as libc::c_int;
        while n >= 0 as libc::c_int {
            y32 = (*p.offset(n as isize) as libc::c_long
                + (y32 as i64 * x_Q16 as libc::c_long >> 16 as libc::c_int))
                as i32;
            n -= 1;
        }
    }
    return y32;
}
#[inline]
#[c2rust::src_loc = "95:1"]
unsafe fn silk_A2NLSF_init(a_Q16: *const i32, P: *mut i32, Q: *mut i32, dd: libc::c_int) {
    let mut k: libc::c_int = 0;
    *P.offset(dd as isize) = ((1 as libc::c_int as u32) << 16 as libc::c_int) as i32;
    *Q.offset(dd as isize) = ((1 as libc::c_int as u32) << 16 as libc::c_int) as i32;
    k = 0 as libc::c_int;
    while k < dd {
        *P.offset(k as isize) =
            -*a_Q16.offset((dd - k - 1 as libc::c_int) as isize) - *a_Q16.offset((dd + k) as isize);
        *Q.offset(k as isize) =
            -*a_Q16.offset((dd - k - 1 as libc::c_int) as isize) + *a_Q16.offset((dd + k) as isize);
        k += 1;
    }
    k = dd;
    while k > 0 as libc::c_int {
        let ref mut fresh2 = *P.offset((k - 1 as libc::c_int) as isize);
        *fresh2 -= *P.offset(k as isize);
        let ref mut fresh3 = *Q.offset((k - 1 as libc::c_int) as isize);
        *fresh3 += *Q.offset(k as isize);
        k -= 1;
    }
    silk_A2NLSF_trans_poly(P, dd);
    silk_A2NLSF_trans_poly(Q, dd);
}
#[c2rust::src_loc = "127:1"]
pub unsafe fn silk_A2NLSF(NLSF: *mut i16, a_Q16: *mut i32, d: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut dd: libc::c_int = 0;
    let mut root_ix: libc::c_int = 0;
    let mut ffrac: libc::c_int = 0;
    let mut xlo: i32 = 0;
    let mut xhi: i32 = 0;
    let mut xmid: i32 = 0;
    let mut ylo: i32 = 0;
    let mut yhi: i32 = 0;
    let mut ymid: i32 = 0;
    let mut thr: i32 = 0;
    let mut nom: i32 = 0;
    let mut den: i32 = 0;
    let mut P: [i32; 13] = [0; 13];
    let mut Q: [i32; 13] = [0; 13];
    let mut PQ: [*mut i32; 2] = [0 as *mut i32; 2];
    let mut p: *mut i32 = 0 as *mut i32;
    PQ[0 as libc::c_int as usize] = P.as_mut_ptr();
    PQ[1 as libc::c_int as usize] = Q.as_mut_ptr();
    dd = d >> 1 as libc::c_int;
    silk_A2NLSF_init(a_Q16, P.as_mut_ptr(), Q.as_mut_ptr(), dd);
    p = P.as_mut_ptr();
    xlo = silk_LSFCosTab_FIX_Q12[0 as libc::c_int as usize] as i32;
    ylo = silk_A2NLSF_eval_poly(p, xlo, dd);
    if ylo < 0 as libc::c_int {
        *NLSF.offset(0 as libc::c_int as isize) = 0 as libc::c_int as i16;
        p = Q.as_mut_ptr();
        ylo = silk_A2NLSF_eval_poly(p, xlo, dd);
        root_ix = 1 as libc::c_int;
    } else {
        root_ix = 0 as libc::c_int;
    }
    k = 1 as libc::c_int;
    i = 0 as libc::c_int;
    thr = 0 as libc::c_int;
    loop {
        xhi = silk_LSFCosTab_FIX_Q12[k as usize] as i32;
        yhi = silk_A2NLSF_eval_poly(p, xhi, dd);
        if ylo <= 0 as libc::c_int && yhi >= thr || ylo >= 0 as libc::c_int && yhi <= -thr {
            if yhi == 0 as libc::c_int {
                thr = 1 as libc::c_int;
            } else {
                thr = 0 as libc::c_int;
            }
            ffrac = -(256 as libc::c_int);
            m = 0 as libc::c_int;
            while m < BIN_DIV_STEPS_A2NLSF_FIX {
                xmid = if 1 as libc::c_int == 1 as libc::c_int {
                    (xlo + xhi >> 1 as libc::c_int) + (xlo + xhi & 1 as libc::c_int)
                } else {
                    (xlo + xhi >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                };
                ymid = silk_A2NLSF_eval_poly(p, xmid, dd);
                if ylo <= 0 as libc::c_int && ymid >= 0 as libc::c_int
                    || ylo >= 0 as libc::c_int && ymid <= 0 as libc::c_int
                {
                    xhi = xmid;
                    yhi = ymid;
                } else {
                    xlo = xmid;
                    ylo = ymid;
                    ffrac = ffrac + (128 as libc::c_int >> m);
                }
                m += 1;
            }
            if (if ylo > 0 as libc::c_int { ylo } else { -ylo }) < 65536 as libc::c_int {
                den = ylo - yhi;
                nom = ((ylo as u32) << 8 as libc::c_int - 3 as libc::c_int) as i32
                    + (den >> 1 as libc::c_int);
                if den != 0 as libc::c_int {
                    ffrac += nom / den;
                }
            } else {
                ffrac += ylo / (ylo - yhi >> 8 as libc::c_int - 3 as libc::c_int);
            }
            *NLSF.offset(root_ix as isize) = silk_min_32(
                ((k as u32) << 8 as libc::c_int) as i32 + ffrac,
                silk_int16_MAX,
            ) as i16;
            root_ix += 1;
            if root_ix >= d {
                break;
            }
            p = PQ[(root_ix & 1 as libc::c_int) as usize];
            xlo = silk_LSFCosTab_FIX_Q12[(k - 1 as libc::c_int) as usize] as i32;
            ylo = (((1 as libc::c_int - (root_ix & 2 as libc::c_int)) as u32) << 12 as libc::c_int)
                as i32;
        } else {
            k += 1;
            xlo = xhi;
            ylo = yhi;
            thr = 0 as libc::c_int;
            if k > LSF_COS_TAB_SZ_FIX {
                i += 1;
                if i > MAX_ITERATIONS_A2NLSF_FIX {
                    *NLSF.offset(0 as libc::c_int as isize) =
                        (((1 as libc::c_int) << 15 as libc::c_int) / (d + 1 as libc::c_int)) as i16;
                    k = 1 as libc::c_int;
                    while k < d {
                        *NLSF.offset(k as isize) = (*NLSF.offset((k - 1 as libc::c_int) as isize)
                            as libc::c_int
                            + *NLSF.offset(0 as libc::c_int as isize) as libc::c_int)
                            as i16;
                        k += 1;
                    }
                    return;
                }
                silk_bwexpander_32(
                    a_Q16,
                    d,
                    65536 as libc::c_int - ((1 as libc::c_int as u32) << i) as i32,
                );
                silk_A2NLSF_init(a_Q16, P.as_mut_ptr(), Q.as_mut_ptr(), dd);
                p = P.as_mut_ptr();
                xlo = silk_LSFCosTab_FIX_Q12[0 as libc::c_int as usize] as i32;
                ylo = silk_A2NLSF_eval_poly(p, xlo, dd);
                if ylo < 0 as libc::c_int {
                    *NLSF.offset(0 as libc::c_int as isize) = 0 as libc::c_int as i16;
                    p = Q.as_mut_ptr();
                    ylo = silk_A2NLSF_eval_poly(p, xlo, dd);
                    root_ix = 1 as libc::c_int;
                } else {
                    root_ix = 0 as libc::c_int;
                }
                k = 1 as libc::c_int;
            }
        }
    }
}
