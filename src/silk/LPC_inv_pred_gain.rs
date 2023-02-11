use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "43:9"]
    pub const silk_int32_MIN: libc::c_uint = 0x80000000 as libc::c_uint;
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
}
pub use self::typedef_h::{silk_int32_MAX, silk_int32_MIN};
use crate::silk::macros::silk_CLZ32;
use crate::silk::Inlines::silk_INVERSE32_varQ;

#[c2rust::src_loc = "36:9"]
pub const A_LIMIT: libc::c_double =
    0.99975f64 * ((1 as libc::c_int as i64) << 24 as libc::c_int) as libc::c_double + 0.5f64;
#[c2rust::src_loc = "42:1"]
unsafe extern "C" fn LPC_inverse_pred_gain_QA_c(A_QA: *mut i32, order: libc::c_int) -> i32 {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut mult2Q: libc::c_int = 0;
    let mut invGain_Q30: i32 = 0;
    let mut rc_Q31: i32 = 0;
    let mut rc_mult1_Q30: i32 = 0;
    let mut rc_mult2: i32 = 0;
    let mut tmp1: i32 = 0;
    let mut tmp2: i32 = 0;
    invGain_Q30 = ((1 as libc::c_int as libc::c_long
        * ((1 as libc::c_int as i64) << 30 as libc::c_int)) as libc::c_double
        + 0.5f64) as i32;
    k = order - 1 as libc::c_int;
    while k > 0 as libc::c_int {
        if *A_QA.offset(k as isize) > A_LIMIT as i32 || *A_QA.offset(k as isize) < -(A_LIMIT as i32)
        {
            return 0 as libc::c_int;
        }
        rc_Q31 =
            -(((*A_QA.offset(k as isize) as u32) << 31 as libc::c_int - 24 as libc::c_int) as i32);
        rc_mult1_Q30 = ((1 as libc::c_int as libc::c_long
            * ((1 as libc::c_int as i64) << 30 as libc::c_int))
            as libc::c_double
            + 0.5f64) as i32
            - (rc_Q31 as i64 * rc_Q31 as libc::c_long >> 32 as libc::c_int) as i32;
        invGain_Q30 = (((invGain_Q30 as i64 * rc_mult1_Q30 as libc::c_long >> 32 as libc::c_int)
            as i32 as u32)
            << 2 as libc::c_int) as i32;
        if invGain_Q30
            < ((1.0f32 / 1e4f32 * ((1 as libc::c_int as i64) << 30 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as i32
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
            let mut tmp64: i64 = 0;
            tmp1 = *A_QA.offset(n as isize);
            tmp2 = *A_QA.offset((k - n - 1 as libc::c_int) as isize);
            tmp64 = if mult2Q == 1 as libc::c_int {
                ((if (tmp1 as u32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                            + (tmp2 as i64 * rc_Q31 as libc::c_long
                                & 1 as libc::c_int as libc::c_long)
                    } else {
                        (tmp2 as i64 * rc_Q31 as libc::c_long
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_long
                            >> 1 as libc::c_int
                    }) as i32 as u32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if tmp1 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32 as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as i32
                    } else {
                        tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32
                    }
                } else {
                    if (tmp1 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32 as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32
                    }
                }) as i64
                    * rc_mult2 as libc::c_long
                    >> 1 as libc::c_int)
                    + ((if (tmp1 as u32).wrapping_sub(
                        (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32 as u32,
                    ) & 0x80000000 as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    {
                        if tmp1 as libc::c_uint
                            & ((if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp2 as i64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp2 as i64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as i32 as libc::c_uint
                                ^ 0x80000000 as libc::c_uint)
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x80000000 as libc::c_uint as i32
                        } else {
                            tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp2 as i64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp2 as i64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as i32
                        }
                    } else {
                        if (tmp1 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                            & (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp2 as i64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp2 as i64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as i32 as libc::c_uint
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x7fffffff as libc::c_int
                        } else {
                            tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp2 as i64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp2 as i64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as i32
                        }
                    }) as i64
                        * rc_mult2 as libc::c_long
                        & 1 as libc::c_int as libc::c_long)
            } else {
                ((if (tmp1 as u32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                            + (tmp2 as i64 * rc_Q31 as libc::c_long
                                & 1 as libc::c_int as libc::c_long)
                    } else {
                        (tmp2 as i64 * rc_Q31 as libc::c_long
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_long
                            >> 1 as libc::c_int
                    }) as i32 as u32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if tmp1 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32 as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as i32
                    } else {
                        tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32
                    }
                } else {
                    if (tmp1 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32 as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        tmp1 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp2 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp2 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32
                    }
                }) as i64
                    * rc_mult2 as libc::c_long
                    >> mult2Q - 1 as libc::c_int)
                    + 1 as libc::c_int as libc::c_long
                    >> 1 as libc::c_int
            };
            if tmp64 > silk_int32_MAX as libc::c_long || tmp64 < silk_int32_MIN as libc::c_long {
                return 0 as libc::c_int;
            }
            *A_QA.offset(n as isize) = tmp64 as i32;
            tmp64 = if mult2Q == 1 as libc::c_int {
                ((if (tmp2 as u32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                            + (tmp1 as i64 * rc_Q31 as libc::c_long
                                & 1 as libc::c_int as libc::c_long)
                    } else {
                        (tmp1 as i64 * rc_Q31 as libc::c_long
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_long
                            >> 1 as libc::c_int
                    }) as i32 as u32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if tmp2 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32 as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as i32
                    } else {
                        tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32
                    }
                } else {
                    if (tmp2 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32 as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32
                    }
                }) as i64
                    * rc_mult2 as libc::c_long
                    >> 1 as libc::c_int)
                    + ((if (tmp2 as u32).wrapping_sub(
                        (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32 as u32,
                    ) & 0x80000000 as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    {
                        if tmp2 as libc::c_uint
                            & ((if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp1 as i64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp1 as i64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as i32 as libc::c_uint
                                ^ 0x80000000 as libc::c_uint)
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x80000000 as libc::c_uint as i32
                        } else {
                            tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp1 as i64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp1 as i64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as i32
                        }
                    } else {
                        if (tmp2 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                            & (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp1 as i64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp1 as i64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as i32 as libc::c_uint
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x7fffffff as libc::c_int
                        } else {
                            tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                    + (tmp1 as i64 * rc_Q31 as libc::c_long
                                        & 1 as libc::c_int as libc::c_long)
                            } else {
                                (tmp1 as i64 * rc_Q31 as libc::c_long
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_long
                                    >> 1 as libc::c_int
                            }) as i32
                        }
                    }) as i64
                        * rc_mult2 as libc::c_long
                        & 1 as libc::c_int as libc::c_long)
            } else {
                ((if (tmp2 as u32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                            + (tmp1 as i64 * rc_Q31 as libc::c_long
                                & 1 as libc::c_int as libc::c_long)
                    } else {
                        (tmp1 as i64 * rc_Q31 as libc::c_long
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_long
                            >> 1 as libc::c_int
                    }) as i32 as u32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if tmp2 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32 as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as i32
                    } else {
                        tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32
                    }
                } else {
                    if (tmp2 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32 as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        tmp2 - (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as i64 * rc_Q31 as libc::c_long >> 1 as libc::c_int)
                                + (tmp1 as i64 * rc_Q31 as libc::c_long
                                    & 1 as libc::c_int as libc::c_long)
                        } else {
                            (tmp1 as i64 * rc_Q31 as libc::c_long
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long
                                >> 1 as libc::c_int
                        }) as i32
                    }
                }) as i64
                    * rc_mult2 as libc::c_long
                    >> mult2Q - 1 as libc::c_int)
                    + 1 as libc::c_int as libc::c_long
                    >> 1 as libc::c_int
            };
            if tmp64 > silk_int32_MAX as libc::c_long || tmp64 < silk_int32_MIN as libc::c_long {
                return 0 as libc::c_int;
            }
            *A_QA.offset((k - n - 1 as libc::c_int) as isize) = tmp64 as i32;
            n += 1;
        }
        k -= 1;
    }
    if *A_QA.offset(k as isize) > A_LIMIT as i32 || *A_QA.offset(k as isize) < -(A_LIMIT as i32) {
        return 0 as libc::c_int;
    }
    rc_Q31 = -(((*A_QA.offset(0 as libc::c_int as isize) as u32)
        << 31 as libc::c_int - 24 as libc::c_int) as i32);
    rc_mult1_Q30 = ((1 as libc::c_int as libc::c_long
        * ((1 as libc::c_int as i64) << 30 as libc::c_int)) as libc::c_double
        + 0.5f64) as i32
        - (rc_Q31 as i64 * rc_Q31 as libc::c_long >> 32 as libc::c_int) as i32;
    invGain_Q30 = (((invGain_Q30 as i64 * rc_mult1_Q30 as libc::c_long >> 32 as libc::c_int) as i32
        as u32)
        << 2 as libc::c_int) as i32;
    if invGain_Q30
        < ((1.0f32 / 1e4f32 * ((1 as libc::c_int as i64) << 30 as libc::c_int) as libc::c_float)
            as libc::c_double
            + 0.5f64) as i32
    {
        return 0 as libc::c_int;
    }
    return invGain_Q30;
}
#[no_mangle]
#[c2rust::src_loc = "122:1"]
pub unsafe extern "C" fn silk_LPC_inverse_pred_gain_c(
    A_Q12: *const i16,
    order: libc::c_int,
) -> i32 {
    let mut k: libc::c_int = 0;
    let mut Atmp_QA: [i32; 24] = [0; 24];
    let mut DC_resp: i32 = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < order {
        DC_resp += *A_Q12.offset(k as isize) as i32;
        Atmp_QA[k as usize] = ((*A_Q12.offset(k as isize) as i32 as u32)
            << 24 as libc::c_int - 12 as libc::c_int) as i32;
        k += 1;
    }
    if DC_resp >= 4096 as libc::c_int {
        return 0 as libc::c_int;
    }
    return LPC_inverse_pred_gain_QA_c(Atmp_QA.as_mut_ptr(), order);
}
