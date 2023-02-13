use crate::silk::macros::silk_CLZ32;
use crate::silk::SigProc_FIX::silk_ROR32;

#[inline]
#[c2rust::src_loc = "56:1"]
pub unsafe fn silk_CLZ_FRAC(in_0: i32, lz: *mut i32, frac_Q7: *mut i32) {
    let lzeros: i32 = silk_CLZ32(in_0);
    *lz = lzeros;
    *frac_Q7 = silk_ROR32(in_0, 24 as i32 - lzeros) & 0x7f as i32;
}
#[inline]
#[c2rust::src_loc = "71:1"]
pub unsafe fn silk_SQRT_APPROX(x: i32) -> i32 {
    let mut y: i32 = 0;
    let mut lz: i32 = 0;
    let mut frac_Q7: i32 = 0;
    if x <= 0 as i32 {
        return 0 as i32;
    }
    silk_CLZ_FRAC(x, &mut lz, &mut frac_Q7);
    if lz & 1 as i32 != 0 {
        y = 32768 as i32;
    } else {
        y = 46214 as i32;
    }
    y >>= lz >> 1 as i32;
    y = (y as i64
        + (y as i64 * (213 as i32 as i16 as i32 * frac_Q7 as i16 as i32) as i16 as i64
            >> 16 as i32)) as i32;
    return y;
}
#[inline]
#[c2rust::src_loc = "97:1"]
pub unsafe fn silk_DIV32_varQ(a32: i32, b32: i32, Qres: i32) -> i32 {
    let mut a_headrm: i32 = 0;
    let mut b_headrm: i32 = 0;
    let mut lshift: i32 = 0;
    let mut b32_inv: i32 = 0;
    let mut a32_nrm: i32 = 0;
    let mut b32_nrm: i32 = 0;
    let mut result: i32 = 0;
    a_headrm = silk_CLZ32(if a32 > 0 as i32 { a32 } else { -a32 }) - 1 as i32;
    a32_nrm = ((a32 as u32) << a_headrm) as i32;
    b_headrm = silk_CLZ32(if b32 > 0 as i32 { b32 } else { -b32 }) - 1 as i32;
    b32_nrm = ((b32 as u32) << b_headrm) as i32;
    b32_inv = (0x7fffffff as i32 >> 2 as i32) / (b32_nrm >> 16 as i32);
    result = (a32_nrm as i64 * b32_inv as i16 as i64 >> 16 as i32) as i32;
    a32_nrm = (a32_nrm as u32).wrapping_sub(
        (((b32_nrm as i64 * result as i64 >> 32 as i32) as i32 as u32) << 3 as i32) as i32 as u32,
    ) as i32;
    result = (result as i64 + (a32_nrm as i64 * b32_inv as i16 as i64 >> 16 as i32)) as i32;
    lshift = 29 as i32 + a_headrm - b_headrm - Qres;
    if lshift < 0 as i32 {
        return (((if 0x80000000 as u32 as i32 >> -lshift > 0x7fffffff as i32 >> -lshift {
            if result > 0x80000000 as u32 as i32 >> -lshift {
                0x80000000 as u32 as i32 >> -lshift
            } else {
                if result < 0x7fffffff as i32 >> -lshift {
                    0x7fffffff as i32 >> -lshift
                } else {
                    result
                }
            }
        } else {
            if result > 0x7fffffff as i32 >> -lshift {
                0x7fffffff as i32 >> -lshift
            } else {
                if result < 0x80000000 as u32 as i32 >> -lshift {
                    0x80000000 as u32 as i32 >> -lshift
                } else {
                    result
                }
            }
        }) as u32)
            << -lshift) as i32;
    } else if lshift < 32 as i32 {
        return result >> lshift;
    } else {
        return 0 as i32;
    };
}
#[inline]
#[c2rust::src_loc = "143:1"]
pub unsafe fn silk_INVERSE32_varQ(b32: i32, Qres: i32) -> i32 {
    let mut b_headrm: i32 = 0;
    let mut lshift: i32 = 0;
    let mut b32_inv: i32 = 0;
    let mut b32_nrm: i32 = 0;
    let mut err_Q32: i32 = 0;
    let mut result: i32 = 0;
    b_headrm = silk_CLZ32(if b32 > 0 as i32 { b32 } else { -b32 }) - 1 as i32;
    b32_nrm = ((b32 as u32) << b_headrm) as i32;
    b32_inv = (0x7fffffff as i32 >> 2 as i32) / (b32_nrm >> 16 as i32);
    result = ((b32_inv as u32) << 16 as i32) as i32;
    err_Q32 = (((((1 as i32) << 29 as i32)
        - (b32_nrm as i64 * b32_inv as i16 as i64 >> 16 as i32) as i32) as u32)
        << 3 as i32) as i32;
    result = (result as i64 + (err_Q32 as i64 * b32_inv as i64 >> 16 as i32)) as i32;
    lshift = 61 as i32 - b_headrm - Qres;
    if lshift <= 0 as i32 {
        return (((if 0x80000000 as u32 as i32 >> -lshift > 0x7fffffff as i32 >> -lshift {
            if result > 0x80000000 as u32 as i32 >> -lshift {
                0x80000000 as u32 as i32 >> -lshift
            } else {
                if result < 0x7fffffff as i32 >> -lshift {
                    0x7fffffff as i32 >> -lshift
                } else {
                    result
                }
            }
        } else {
            if result > 0x7fffffff as i32 >> -lshift {
                0x7fffffff as i32 >> -lshift
            } else {
                if result < 0x80000000 as u32 as i32 >> -lshift {
                    0x80000000 as u32 as i32 >> -lshift
                } else {
                    result
                }
            }
        }) as u32)
            << -lshift) as i32;
    } else if lshift < 32 as i32 {
        return result >> lshift;
    } else {
        return 0 as i32;
    };
}
