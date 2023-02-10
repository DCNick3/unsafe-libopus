use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
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
    pub unsafe extern "C" fn silk_CLZ32(in32: opus_int32) -> opus_int32 {
        return if in32 != 0 {
            32 as libc::c_int - (EC_CLZ0 - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }
    use super::ecintrin_h::EC_CLZ0;
    use super::opus_types_h::opus_int32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "43:9"]
    pub const silk_int32_MIN: libc::c_uint = 0x80000000 as libc::c_uint;
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/Inlines.h:32"]
pub mod Inlines_h {
    #[inline]
    #[c2rust::src_loc = "143:1"]
    pub unsafe extern "C" fn silk_INVERSE32_varQ(b32: opus_int32, Qres: libc::c_int) -> opus_int32 {
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: opus_int32 = 0;
        let mut b32_nrm: opus_int32 = 0;
        let mut err_Q32: opus_int32 = 0;
        let mut result: opus_int32 = 0;
        b_headrm = silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 }) - 1 as libc::c_int;
        b32_nrm = ((b32 as opus_uint32) << b_headrm) as opus_int32;
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int) / (b32_nrm >> 16 as libc::c_int);
        result = ((b32_inv as opus_uint32) << 16 as libc::c_int) as opus_int32;
        err_Q32 = (((((1 as libc::c_int) << 29 as libc::c_int)
            - (b32_nrm as libc::c_long * b32_inv as opus_int16 as opus_int64 >> 16 as libc::c_int)
                as opus_int32) as opus_uint32)
            << 3 as libc::c_int) as opus_int32;
        result = (result as libc::c_long
            + (err_Q32 as opus_int64 * b32_inv as libc::c_long >> 16 as libc::c_int))
            as opus_int32;
        lshift = 61 as libc::c_int - b_headrm - Qres;
        if lshift <= 0 as libc::c_int {
            return (((if 0x80000000 as libc::c_uint as opus_int32 >> -lshift
                > 0x7fffffff as libc::c_int >> -lshift
            {
                if result > 0x80000000 as libc::c_uint as opus_int32 >> -lshift {
                    0x80000000 as libc::c_uint as opus_int32 >> -lshift
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
                    if result < 0x80000000 as libc::c_uint as opus_int32 >> -lshift {
                        0x80000000 as libc::c_uint as opus_int32 >> -lshift
                    } else {
                        result
                    }
                }
            }) as opus_uint32)
                << -lshift) as opus_int32;
        } else if lshift < 32 as libc::c_int {
            return result >> lshift;
        } else {
            return 0 as libc::c_int;
        };
    }
    use super::macros_h::silk_CLZ32;
    use super::opus_types_h::{opus_int16, opus_int32, opus_int64, opus_uint32};
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
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int64, opus_uint32};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::typedef_h::{silk_int32_MAX, silk_int32_MIN};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
pub use self::Inlines_h::silk_INVERSE32_varQ;
#[c2rust::src_loc = "36:9"]
pub const A_LIMIT: libc::c_double =
    0.99975f64 * ((1 as libc::c_int as opus_int64) << 24 as libc::c_int) as libc::c_double + 0.5f64;
#[c2rust::src_loc = "42:1"]
unsafe extern "C" fn LPC_inverse_pred_gain_QA_c(
    A_QA: *mut opus_int32,
    order: libc::c_int,
) -> opus_int32 {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut mult2Q: libc::c_int = 0;
    let mut invGain_Q30: opus_int32 = 0;
    let mut rc_Q31: opus_int32 = 0;
    let mut rc_mult1_Q30: opus_int32 = 0;
    let mut rc_mult2: opus_int32 = 0;
    let mut tmp1: opus_int32 = 0;
    let mut tmp2: opus_int32 = 0;
    invGain_Q30 = ((1 as libc::c_int as libc::c_long
        * ((1 as libc::c_int as opus_int64) << 30 as libc::c_int))
        as libc::c_double
        + 0.5f64) as opus_int32;
    k = order - 1 as libc::c_int;
    while k > 0 as libc::c_int {
        if *A_QA.offset(k as isize) > A_LIMIT as opus_int32
            || *A_QA.offset(k as isize) < -(A_LIMIT as opus_int32)
        {
            return 0 as libc::c_int;
        }
        rc_Q31 = -(((*A_QA.offset(k as isize) as opus_uint32)
            << 31 as libc::c_int - 24 as libc::c_int) as opus_int32);
        rc_mult1_Q30 = ((1 as libc::c_int as libc::c_long
            * ((1 as libc::c_int as opus_int64) << 30 as libc::c_int))
            as libc::c_double
            + 0.5f64) as opus_int32
            - (rc_Q31 as opus_int64 * rc_Q31 as libc::c_long >> 32 as libc::c_int) as opus_int32;
        invGain_Q30 = (((invGain_Q30 as opus_int64 * rc_mult1_Q30 as libc::c_long
            >> 32 as libc::c_int) as opus_int32 as opus_uint32)
            << 2 as libc::c_int) as opus_int32;
        if invGain_Q30
            < ((1.0f32 / 1e4f32
                * ((1 as libc::c_int as opus_int64) << 30 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as opus_int32
        {
            return 0 as libc::c_int;
        }
        mult2Q = 32 as libc::c_int
            - silk_CLZ32(if rc_mult1_Q30 > 0 as libc::c_int {
                rc_mult1_Q30
            } else {
                -rc_mult1_Q30
            });
        rc_mult2 = silk_INVERSE32_varQ(rc_mult1_Q30, mult2Q + 30 as libc::c_int);
        n = 0 as libc::c_int;
        while n < k + 1 as libc::c_int >> 1 as libc::c_int {
            let mut tmp64: opus_int64 = 0;
            tmp1 = *A_QA.offset(n as isize);
            tmp2 = *A_QA.offset((k - n - 1 as libc::c_int) as isize);
            tmp64 = if mult2Q == 1 as libc::c_int {
                ((if (tmp1 as opus_uint32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                            + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                & 1 as libc::c_int as libc::c_long)
                    } else {
                        (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_long
                            >> 1 as libc::c_int
                    }) as opus_int32 as opus_uint32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if tmp1 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32 as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as opus_int32
                    } else {
                        tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32
                    }
                } else {
                    if (tmp1 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32 as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32
                    }
                }) as opus_int64
                    * rc_mult2 as libc::c_long
                    >> 1 as libc::c_int)
                    + ((if (tmp1 as opus_uint32).wrapping_sub(
                        (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32 as opus_uint32,
                    ) & 0x80000000 as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    {
                        if tmp1 as libc::c_uint
                            & ((if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as opus_int32 as libc::c_uint
                                ^ 0x80000000 as libc::c_uint)
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x80000000 as libc::c_uint as opus_int32
                        } else {
                            tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as opus_int32
                        }
                    } else {
                        if (tmp1 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                            & (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as opus_int32 as libc::c_uint
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x7fffffff as libc::c_int
                        } else {
                            tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as opus_int32
                        }
                    }) as opus_int64
                        * rc_mult2 as libc::c_long
                        & 1 as libc::c_int as libc::c_long)
            } else {
                ((if (tmp1 as opus_uint32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                            + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                & 1 as libc::c_int as libc::c_long)
                    } else {
                        (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_long
                            >> 1 as libc::c_int
                    }) as opus_int32 as opus_uint32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if tmp1 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32 as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as opus_int32
                    } else {
                        tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32
                    }
                } else {
                    if (tmp1 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32 as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32
                    }
                }) as opus_int64
                    * rc_mult2 as libc::c_long
                    >> mult2Q - 1 as libc::c_int)
                    + 1 as libc::c_int as libc::c_long
                    >> 1 as libc::c_int
            };
            if tmp64 > silk_int32_MAX as libc::c_long || tmp64 < silk_int32_MIN as libc::c_long {
                return 0 as libc::c_int;
            }
            *A_QA.offset(n as isize) = tmp64 as opus_int32;
            tmp64 = if mult2Q == 1 as libc::c_int {
                ((if (tmp2 as opus_uint32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                            + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                & 1 as libc::c_int as libc::c_long)
                    } else {
                        (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_long
                            >> 1 as libc::c_int
                    }) as opus_int32 as opus_uint32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if tmp2 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32 as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as opus_int32
                    } else {
                        tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32
                    }
                } else {
                    if (tmp2 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32 as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32
                    }
                }) as opus_int64
                    * rc_mult2 as libc::c_long
                    >> 1 as libc::c_int)
                    + ((if (tmp2 as opus_uint32).wrapping_sub(
                        (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32 as opus_uint32,
                    ) & 0x80000000 as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    {
                        if tmp2 as libc::c_uint
                            & ((if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as opus_int32 as libc::c_uint
                                ^ 0x80000000 as libc::c_uint)
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x80000000 as libc::c_uint as opus_int32
                        } else {
                            tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as opus_int32
                        }
                    } else {
                        if (tmp2 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                            & (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as opus_int32 as libc::c_uint
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x7fffffff as libc::c_int
                        } else {
                            tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as opus_int32
                        }
                    }) as opus_int64
                        * rc_mult2 as libc::c_long
                        & 1 as libc::c_int as libc::c_long)
            } else {
                ((if (tmp2 as opus_uint32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                            + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                & 1 as libc::c_int as libc::c_long)
                    } else {
                        (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_long
                            >> 1 as libc::c_int
                    }) as opus_int32 as opus_uint32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if tmp2 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32 as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as opus_int32
                    } else {
                        tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32
                    }
                } else {
                    if (tmp2 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32 as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as opus_int64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as opus_int32
                    }
                }) as opus_int64
                    * rc_mult2 as libc::c_long
                    >> mult2Q - 1 as libc::c_int)
                    + 1 as libc::c_int as libc::c_long
                    >> 1 as libc::c_int
            };
            if tmp64 > silk_int32_MAX as libc::c_long || tmp64 < silk_int32_MIN as libc::c_long {
                return 0 as libc::c_int;
            }
            *A_QA.offset((k - n - 1 as libc::c_int) as isize) = tmp64 as opus_int32;
            n += 1;
        }
        k -= 1;
    }
    if *A_QA.offset(k as isize) > A_LIMIT as opus_int32
        || *A_QA.offset(k as isize) < -(A_LIMIT as opus_int32)
    {
        return 0 as libc::c_int;
    }
    rc_Q31 = -(((*A_QA.offset(0 as libc::c_int as isize) as opus_uint32)
        << 31 as libc::c_int - 24 as libc::c_int) as opus_int32);
    rc_mult1_Q30 = ((1 as libc::c_int as libc::c_long
        * ((1 as libc::c_int as opus_int64) << 30 as libc::c_int))
        as libc::c_double
        + 0.5f64) as opus_int32
        - (rc_Q31 as opus_int64 * rc_Q31 as libc::c_long >> 32 as libc::c_int) as opus_int32;
    invGain_Q30 = (((invGain_Q30 as opus_int64 * rc_mult1_Q30 as libc::c_long >> 32 as libc::c_int)
        as opus_int32 as opus_uint32)
        << 2 as libc::c_int) as opus_int32;
    if invGain_Q30
        < ((1.0f32 / 1e4f32
            * ((1 as libc::c_int as opus_int64) << 30 as libc::c_int) as libc::c_float)
            as libc::c_double
            + 0.5f64) as opus_int32
    {
        return 0 as libc::c_int;
    }
    return invGain_Q30;
}
#[no_mangle]
#[c2rust::src_loc = "122:1"]
pub unsafe extern "C" fn silk_LPC_inverse_pred_gain_c(
    A_Q12: *const opus_int16,
    order: libc::c_int,
) -> opus_int32 {
    let mut k: libc::c_int = 0;
    let mut Atmp_QA: [opus_int32; 24] = [0; 24];
    let mut DC_resp: opus_int32 = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < order {
        DC_resp += *A_Q12.offset(k as isize) as opus_int32;
        Atmp_QA[k as usize] = ((*A_Q12.offset(k as isize) as opus_int32 as opus_uint32)
            << 24 as libc::c_int - 12 as libc::c_int) as opus_int32;
        k += 1;
    }
    if DC_resp >= 4096 as libc::c_int {
        return 0 as libc::c_int;
    }
    return LPC_inverse_pred_gain_QA_c(Atmp_QA.as_mut_ptr(), order);
}
