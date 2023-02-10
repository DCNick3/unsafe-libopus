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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:38"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int16_t, int32_t, int64_t};
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:38"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "554:1"]
    pub unsafe extern "C" fn silk_min_32(mut a: opus_int32, mut b: opus_int32) -> opus_int32 {
        return if a < b { a } else { b };
    }
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "140:1"]
        pub fn silk_bwexpander_32(ar: *mut opus_int32, d: libc::c_int, chirp_Q16: opus_int32);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:39"]
pub mod tables_h {
    use super::opus_types_h::opus_int16;
    extern "C" {
        #[c2rust::src_loc = "108:26"]
        pub static silk_LSFCosTab_FIX_Q12: [opus_int16; 129];
    }
}
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int64, opus_uint32};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
use self::tables_h::silk_LSFCosTab_FIX_Q12;
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
pub use self::SigProc_FIX_h::{silk_bwexpander_32, silk_min_32};
#[inline]
#[c2rust::src_loc = "47:1"]
unsafe extern "C" fn silk_A2NLSF_trans_poly(mut p: *mut opus_int32, dd: libc::c_int) {
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
        *fresh1 -= ((*p.offset(k as isize) as opus_uint32) << 1 as libc::c_int) as opus_int32;
        k += 1;
    }
}
#[inline]
#[c2rust::src_loc = "63:1"]
unsafe extern "C" fn silk_A2NLSF_eval_poly(
    mut p: *mut opus_int32,
    x: opus_int32,
    dd: libc::c_int,
) -> opus_int32 {
    let mut n: libc::c_int = 0;
    let mut x_Q16: opus_int32 = 0;
    let mut y32: opus_int32 = 0;
    y32 = *p.offset(dd as isize);
    x_Q16 = ((x as opus_uint32) << 4 as libc::c_int) as opus_int32;
    if (8 as libc::c_int == dd) as libc::c_int as libc::c_long != 0 {
        y32 = (*p.offset(7 as libc::c_int as isize) as libc::c_long
            + (y32 as opus_int64 * x_Q16 as libc::c_long >> 16 as libc::c_int))
            as opus_int32;
        y32 = (*p.offset(6 as libc::c_int as isize) as libc::c_long
            + (y32 as opus_int64 * x_Q16 as libc::c_long >> 16 as libc::c_int))
            as opus_int32;
        y32 = (*p.offset(5 as libc::c_int as isize) as libc::c_long
            + (y32 as opus_int64 * x_Q16 as libc::c_long >> 16 as libc::c_int))
            as opus_int32;
        y32 = (*p.offset(4 as libc::c_int as isize) as libc::c_long
            + (y32 as opus_int64 * x_Q16 as libc::c_long >> 16 as libc::c_int))
            as opus_int32;
        y32 = (*p.offset(3 as libc::c_int as isize) as libc::c_long
            + (y32 as opus_int64 * x_Q16 as libc::c_long >> 16 as libc::c_int))
            as opus_int32;
        y32 = (*p.offset(2 as libc::c_int as isize) as libc::c_long
            + (y32 as opus_int64 * x_Q16 as libc::c_long >> 16 as libc::c_int))
            as opus_int32;
        y32 = (*p.offset(1 as libc::c_int as isize) as libc::c_long
            + (y32 as opus_int64 * x_Q16 as libc::c_long >> 16 as libc::c_int))
            as opus_int32;
        y32 = (*p.offset(0 as libc::c_int as isize) as libc::c_long
            + (y32 as opus_int64 * x_Q16 as libc::c_long >> 16 as libc::c_int))
            as opus_int32;
    } else {
        n = dd - 1 as libc::c_int;
        while n >= 0 as libc::c_int {
            y32 = (*p.offset(n as isize) as libc::c_long
                + (y32 as opus_int64 * x_Q16 as libc::c_long >> 16 as libc::c_int))
                as opus_int32;
            n -= 1;
        }
    }
    return y32;
}
#[inline]
#[c2rust::src_loc = "95:1"]
unsafe extern "C" fn silk_A2NLSF_init(
    mut a_Q16: *const opus_int32,
    mut P: *mut opus_int32,
    mut Q: *mut opus_int32,
    dd: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    *P.offset(dd as isize) = ((1 as libc::c_int as opus_uint32) << 16 as libc::c_int) as opus_int32;
    *Q.offset(dd as isize) = ((1 as libc::c_int as opus_uint32) << 16 as libc::c_int) as opus_int32;
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
#[no_mangle]
#[c2rust::src_loc = "127:1"]
pub unsafe extern "C" fn silk_A2NLSF(
    mut NLSF: *mut opus_int16,
    mut a_Q16: *mut opus_int32,
    d: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut dd: libc::c_int = 0;
    let mut root_ix: libc::c_int = 0;
    let mut ffrac: libc::c_int = 0;
    let mut xlo: opus_int32 = 0;
    let mut xhi: opus_int32 = 0;
    let mut xmid: opus_int32 = 0;
    let mut ylo: opus_int32 = 0;
    let mut yhi: opus_int32 = 0;
    let mut ymid: opus_int32 = 0;
    let mut thr: opus_int32 = 0;
    let mut nom: opus_int32 = 0;
    let mut den: opus_int32 = 0;
    let mut P: [opus_int32; 13] = [0; 13];
    let mut Q: [opus_int32; 13] = [0; 13];
    let mut PQ: [*mut opus_int32; 2] = [0 as *mut opus_int32; 2];
    let mut p: *mut opus_int32 = 0 as *mut opus_int32;
    PQ[0 as libc::c_int as usize] = P.as_mut_ptr();
    PQ[1 as libc::c_int as usize] = Q.as_mut_ptr();
    dd = d >> 1 as libc::c_int;
    silk_A2NLSF_init(a_Q16, P.as_mut_ptr(), Q.as_mut_ptr(), dd);
    p = P.as_mut_ptr();
    xlo = silk_LSFCosTab_FIX_Q12[0 as libc::c_int as usize] as opus_int32;
    ylo = silk_A2NLSF_eval_poly(p, xlo, dd);
    if ylo < 0 as libc::c_int {
        *NLSF.offset(0 as libc::c_int as isize) = 0 as libc::c_int as opus_int16;
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
        xhi = silk_LSFCosTab_FIX_Q12[k as usize] as opus_int32;
        yhi = silk_A2NLSF_eval_poly(p, xhi, dd);
        if ylo <= 0 as libc::c_int && yhi >= thr || ylo >= 0 as libc::c_int && yhi <= -thr {
            if yhi == 0 as libc::c_int {
                thr = 1 as libc::c_int;
            } else {
                thr = 0 as libc::c_int;
            }
            ffrac = -(256 as libc::c_int);
            m = 0 as libc::c_int;
            while m < 3 as libc::c_int {
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
                nom = ((ylo as opus_uint32) << 8 as libc::c_int - 3 as libc::c_int) as opus_int32
                    + (den >> 1 as libc::c_int);
                if den != 0 as libc::c_int {
                    ffrac += nom / den;
                }
            } else {
                ffrac += ylo / (ylo - yhi >> 8 as libc::c_int - 3 as libc::c_int);
            }
            *NLSF.offset(root_ix as isize) = silk_min_32(
                ((k as opus_uint32) << 8 as libc::c_int) as opus_int32 + ffrac,
                0x7fff as libc::c_int,
            ) as opus_int16;
            root_ix += 1;
            if root_ix >= d {
                break;
            }
            p = PQ[(root_ix & 1 as libc::c_int) as usize];
            xlo = silk_LSFCosTab_FIX_Q12[(k - 1 as libc::c_int) as usize] as opus_int32;
            ylo = (((1 as libc::c_int - (root_ix & 2 as libc::c_int)) as opus_uint32)
                << 12 as libc::c_int) as opus_int32;
        } else {
            k += 1;
            xlo = xhi;
            ylo = yhi;
            thr = 0 as libc::c_int;
            if k > 128 as libc::c_int {
                i += 1;
                if i > 16 as libc::c_int {
                    *NLSF.offset(0 as libc::c_int as isize) =
                        (((1 as libc::c_int) << 15 as libc::c_int) / (d + 1 as libc::c_int))
                            as opus_int16;
                    k = 1 as libc::c_int;
                    while k < d {
                        *NLSF.offset(k as isize) = (*NLSF.offset((k - 1 as libc::c_int) as isize)
                            as libc::c_int
                            + *NLSF.offset(0 as libc::c_int as isize) as libc::c_int)
                            as opus_int16;
                        k += 1;
                    }
                    return;
                }
                silk_bwexpander_32(
                    a_Q16,
                    d,
                    65536 as libc::c_int - ((1 as libc::c_int as opus_uint32) << i) as opus_int32,
                );
                silk_A2NLSF_init(a_Q16, P.as_mut_ptr(), Q.as_mut_ptr(), dd);
                p = P.as_mut_ptr();
                xlo = silk_LSFCosTab_FIX_Q12[0 as libc::c_int as usize] as opus_int32;
                ylo = silk_A2NLSF_eval_poly(p, xlo, dd);
                if ylo < 0 as libc::c_int {
                    *NLSF.offset(0 as libc::c_int as isize) = 0 as libc::c_int as opus_int16;
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
