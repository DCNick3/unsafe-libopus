use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/Inlines.h:32"]
pub mod Inlines_h {
    #[inline]
    #[c2rust::src_loc = "71:1"]
    pub unsafe extern "C" fn silk_SQRT_APPROX(x: i32) -> i32 {
        let mut y: i32 = 0;
        let mut lz: i32 = 0;
        let mut frac_Q7: i32 = 0;
        if x <= 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        silk_CLZ_FRAC(x, &mut lz, &mut frac_Q7);
        if lz & 1 as libc::c_int != 0 {
            y = 32768 as libc::c_int;
        } else {
            y = 46214 as libc::c_int;
        }
        y >>= lz >> 1 as libc::c_int;
        y = (y as libc::c_long
            + (y as libc::c_long
                * (213 as libc::c_int as i16 as i32 * frac_Q7 as i16 as i32) as i16 as i64
                >> 16 as libc::c_int)) as i32;
        return y;
    }
    #[inline]
    #[c2rust::src_loc = "56:1"]
    pub unsafe extern "C" fn silk_CLZ_FRAC(in_0: i32, lz: *mut i32, frac_Q7: *mut i32) {
        let lzeros: i32 = silk_CLZ32(in_0);
        *lz = lzeros;
        *frac_Q7 = silk_ROR32(in_0, 24 as libc::c_int - lzeros) & 0x7f as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "97:1"]
    pub unsafe extern "C" fn silk_DIV32_varQ(a32: i32, b32: i32, Qres: libc::c_int) -> i32 {
        let mut a_headrm: libc::c_int = 0;
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: i32 = 0;
        let mut a32_nrm: i32 = 0;
        let mut b32_nrm: i32 = 0;
        let mut result: i32 = 0;
        a_headrm = silk_CLZ32(if a32 > 0 as libc::c_int { a32 } else { -a32 }) - 1 as libc::c_int;
        a32_nrm = ((a32 as u32) << a_headrm) as i32;
        b_headrm = silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 }) - 1 as libc::c_int;
        b32_nrm = ((b32 as u32) << b_headrm) as i32;
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int) / (b32_nrm >> 16 as libc::c_int);
        result = (a32_nrm as libc::c_long * b32_inv as i16 as i64 >> 16 as libc::c_int) as i32;
        a32_nrm = (a32_nrm as u32).wrapping_sub(
            (((b32_nrm as i64 * result as libc::c_long >> 32 as libc::c_int) as i32 as u32)
                << 3 as libc::c_int) as i32 as u32,
        ) as i32;
        result = (result as libc::c_long
            + (a32_nrm as libc::c_long * b32_inv as i16 as i64 >> 16 as libc::c_int))
            as i32;
        lshift = 29 as libc::c_int + a_headrm - b_headrm - Qres;
        if lshift < 0 as libc::c_int {
            return (((if 0x80000000 as libc::c_uint as i32 >> -lshift
                > 0x7fffffff as libc::c_int >> -lshift
            {
                if result > 0x80000000 as libc::c_uint as i32 >> -lshift {
                    0x80000000 as libc::c_uint as i32 >> -lshift
                } else {
                    if result < 0x7fffffff as libc::c_int >> -lshift {
                        0x7fffffff as libc::c_int >> -lshift
                    } else {
                        result
                    }
                }
            } else {
                if result > 0x7fffffff as libc::c_int >> -lshift {
                    0x7fffffff as libc::c_int >> -lshift
                } else {
                    if result < 0x80000000 as libc::c_uint as i32 >> -lshift {
                        0x80000000 as libc::c_uint as i32 >> -lshift
                    } else {
                        result
                    }
                }
            }) as u32)
                << -lshift) as i32;
        } else if lshift < 32 as libc::c_int {
            return result >> lshift;
        } else {
            return 0 as libc::c_int;
        };
    }
    use super::macros_h::silk_CLZ32;
    use super::SigProc_FIX_h::silk_ROR32;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:32"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:32"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int =
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/macros.h:32"]
pub mod macros_h {
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn silk_CLZ32(in32: i32) -> i32 {
        return if in32 != 0 {
            32 as libc::c_int - (EC_CLZ0 - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }
    use super::ecintrin_h::EC_CLZ0;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "398:1"]
    pub unsafe extern "C" fn silk_ROR32(a32: i32, rot: libc::c_int) -> i32 {
        let x: u32 = a32 as u32;
        let r: u32 = rot as u32;
        let m: u32 = -rot as u32;
        if rot == 0 as libc::c_int {
            return a32;
        } else if rot < 0 as libc::c_int {
            return (x << m | x >> (32 as libc::c_int as libc::c_uint).wrapping_sub(m)) as i32;
        } else {
            return (x << (32 as libc::c_int as libc::c_uint).wrapping_sub(r) | x >> r) as i32;
        };
    }
    #[inline]
    #[c2rust::src_loc = "564:1"]
    pub unsafe extern "C" fn silk_max_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
        return if a > b { a } else { b };
    }
    extern "C" {
        #[c2rust::src_loc = "193:1"]
        pub fn silk_sum_sqr_shift(
            energy: *mut i32,
            shift: *mut libc::c_int,
            x: *const i16,
            len: libc::c_int,
        );
        #[c2rust::src_loc = "377:1"]
        pub fn silk_inner_prod_aligned_scale(
            inVec1: *const i16,
            inVec2: *const i16,
            scale: libc::c_int,
            len: libc::c_int,
        ) -> i32;
    }
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
pub use self::ecintrin_h::EC_CLZ0;
pub use self::internal::__CHAR_BIT__;
pub use self::limits_h::CHAR_BIT;
pub use self::macros_h::silk_CLZ32;

pub use self::Inlines_h::{silk_CLZ_FRAC, silk_DIV32_varQ, silk_SQRT_APPROX};
pub use self::SigProc_FIX_h::{
    silk_ROR32, silk_inner_prod_aligned_scale, silk_max_int, silk_sum_sqr_shift,
};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_stereo_find_predictor(
    ratio_Q14: *mut i32,
    x: *const i16,
    y: *const i16,
    mid_res_amp_Q0: *mut i32,
    length: libc::c_int,
    mut smooth_coef_Q16: libc::c_int,
) -> i32 {
    let mut scale: libc::c_int = 0;
    let mut scale1: libc::c_int = 0;
    let mut scale2: libc::c_int = 0;
    let mut nrgx: i32 = 0;
    let mut nrgy: i32 = 0;
    let mut corr: i32 = 0;
    let mut pred_Q13: i32 = 0;
    let mut pred2_Q10: i32 = 0;
    silk_sum_sqr_shift(&mut nrgx, &mut scale1, x, length);
    silk_sum_sqr_shift(&mut nrgy, &mut scale2, y, length);
    scale = silk_max_int(scale1, scale2);
    scale = scale + (scale & 1 as libc::c_int);
    nrgy = nrgy >> scale - scale2;
    nrgx = nrgx >> scale - scale1;
    nrgx = silk_max_int(nrgx, 1 as libc::c_int);
    corr = silk_inner_prod_aligned_scale(x, y, scale, length);
    pred_Q13 = silk_DIV32_varQ(corr, nrgx, 13 as libc::c_int);
    pred_Q13 =
        if -((1 as libc::c_int) << 14 as libc::c_int) > (1 as libc::c_int) << 14 as libc::c_int {
            if pred_Q13 > -((1 as libc::c_int) << 14 as libc::c_int) {
                -((1 as libc::c_int) << 14 as libc::c_int)
            } else if pred_Q13 < (1 as libc::c_int) << 14 as libc::c_int {
                (1 as libc::c_int) << 14 as libc::c_int
            } else {
                pred_Q13
            }
        } else if pred_Q13 > (1 as libc::c_int) << 14 as libc::c_int {
            (1 as libc::c_int) << 14 as libc::c_int
        } else if pred_Q13 < -((1 as libc::c_int) << 14 as libc::c_int) {
            -((1 as libc::c_int) << 14 as libc::c_int)
        } else {
            pred_Q13
        };
    pred2_Q10 = (pred_Q13 as libc::c_long * pred_Q13 as i16 as i64 >> 16 as libc::c_int) as i32;
    smooth_coef_Q16 = silk_max_int(
        smooth_coef_Q16,
        if pred2_Q10 > 0 as libc::c_int {
            pred2_Q10
        } else {
            -pred2_Q10
        },
    );
    scale = scale >> 1 as libc::c_int;
    *mid_res_amp_Q0.offset(0 as libc::c_int as isize) = (*mid_res_amp_Q0
        .offset(0 as libc::c_int as isize)
        as libc::c_long
        + ((((silk_SQRT_APPROX(nrgx) as u32) << scale) as i32
            - *mid_res_amp_Q0.offset(0 as libc::c_int as isize)) as libc::c_long
            * smooth_coef_Q16 as i16 as i64
            >> 16 as libc::c_int)) as i32;
    nrgy = nrgy
        - (((corr as libc::c_long * pred_Q13 as i16 as i64 >> 16 as libc::c_int) as i32 as u32)
            << 3 as libc::c_int + 1 as libc::c_int) as i32;
    nrgy = nrgy
        + (((nrgx as libc::c_long * pred2_Q10 as i16 as i64 >> 16 as libc::c_int) as i32 as u32)
            << 6 as libc::c_int) as i32;
    *mid_res_amp_Q0.offset(1 as libc::c_int as isize) = (*mid_res_amp_Q0
        .offset(1 as libc::c_int as isize)
        as libc::c_long
        + ((((silk_SQRT_APPROX(nrgy) as u32) << scale) as i32
            - *mid_res_amp_Q0.offset(1 as libc::c_int as isize)) as libc::c_long
            * smooth_coef_Q16 as i16 as i64
            >> 16 as libc::c_int)) as i32;
    *ratio_Q14 = silk_DIV32_varQ(
        *mid_res_amp_Q0.offset(1 as libc::c_int as isize),
        if *mid_res_amp_Q0.offset(0 as libc::c_int as isize) > 1 as libc::c_int {
            *mid_res_amp_Q0.offset(0 as libc::c_int as isize)
        } else {
            1 as libc::c_int
        },
        14 as libc::c_int,
    );
    *ratio_Q14 = if 0 as libc::c_int > 32767 as libc::c_int {
        if *ratio_Q14 > 0 as libc::c_int {
            0 as libc::c_int
        } else if *ratio_Q14 < 32767 as libc::c_int {
            32767 as libc::c_int
        } else {
            *ratio_Q14
        }
    } else if *ratio_Q14 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if *ratio_Q14 < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        *ratio_Q14
    };
    return pred_Q13;
}
