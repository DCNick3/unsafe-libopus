#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "43:9"]
    pub const silk_int32_MIN: u32 = 0x80000000 as u32;
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: i32 = 0x7fffffff as i32;
}
pub use self::typedef_h::{silk_int32_MAX, silk_int32_MIN};
use crate::silk::macros::silk_CLZ32;
use crate::silk::Inlines::silk_INVERSE32_varQ;

#[c2rust::src_loc = "36:9"]
pub const A_LIMIT: f64 = 0.99975f64 * ((1 as i32 as i64) << 24 as i32) as f64 + 0.5f64;
#[c2rust::src_loc = "42:1"]
unsafe fn LPC_inverse_pred_gain_QA_c(A_QA: *mut i32, order: i32) -> i32 {
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut mult2Q: i32 = 0;
    let mut invGain_Q30: i32 = 0;
    let mut rc_Q31: i32 = 0;
    let mut rc_mult1_Q30: i32 = 0;
    let mut rc_mult2: i32 = 0;
    let mut tmp1: i32 = 0;
    let mut tmp2: i32 = 0;
    invGain_Q30 = ((1 as i32 as i64 * ((1 as i32 as i64) << 30 as i32)) as f64 + 0.5f64) as i32;
    k = order - 1 as i32;
    while k > 0 as i32 {
        if *A_QA.offset(k as isize) > A_LIMIT as i32 || *A_QA.offset(k as isize) < -(A_LIMIT as i32)
        {
            return 0 as i32;
        }
        rc_Q31 = -(((*A_QA.offset(k as isize) as u32) << 31 as i32 - 24 as i32) as i32);
        rc_mult1_Q30 = ((1 as i32 as i64 * ((1 as i32 as i64) << 30 as i32)) as f64 + 0.5f64)
            as i32
            - (rc_Q31 as i64 * rc_Q31 as i64 >> 32 as i32) as i32;
        invGain_Q30 = (((invGain_Q30 as i64 * rc_mult1_Q30 as i64 >> 32 as i32) as i32 as u32)
            << 2 as i32) as i32;
        if invGain_Q30
            < ((1.0f32 / 1e4f32 * ((1 as i32 as i64) << 30 as i32) as f32) as f64 + 0.5f64) as i32
        {
            return 0 as i32;
        }
        mult2Q = 32 as i32
            - silk_CLZ32(if rc_mult1_Q30 > 0 as i32 {
                rc_mult1_Q30
            } else {
                -rc_mult1_Q30
            });
        rc_mult2 = silk_INVERSE32_varQ(rc_mult1_Q30, mult2Q + 30 as i32);
        n = 0 as i32;
        while n < k + 1 as i32 >> 1 as i32 {
            let mut tmp64: i64 = 0;
            tmp1 = *A_QA.offset(n as isize);
            tmp2 = *A_QA.offset((k - n - 1 as i32) as isize);
            tmp64 = if mult2Q == 1 as i32 {
                ((if (tmp1 as u32).wrapping_sub(
                    (if 31 as i32 == 1 as i32 {
                        (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                            + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                    } else {
                        (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                            >> 1 as i32
                    }) as i32 as u32,
                ) & 0x80000000 as u32
                    == 0 as i32 as u32
                {
                    if tmp1 as u32
                        & ((if 31 as i32 == 1 as i32 {
                            (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32 as u32
                            ^ 0x80000000 as u32)
                        & 0x80000000 as u32
                        != 0
                    {
                        0x80000000 as u32 as i32
                    } else {
                        tmp1 - (if 31 as i32 == 1 as i32 {
                            (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32
                    }
                } else {
                    if (tmp1 as u32 ^ 0x80000000 as u32)
                        & (if 31 as i32 == 1 as i32 {
                            (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32 as u32
                        & 0x80000000 as u32
                        != 0
                    {
                        0x7fffffff as i32
                    } else {
                        tmp1 - (if 31 as i32 == 1 as i32 {
                            (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32
                    }
                }) as i64
                    * rc_mult2 as i64
                    >> 1 as i32)
                    + ((if (tmp1 as u32).wrapping_sub(
                        (if 31 as i32 == 1 as i32 {
                            (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32 as u32,
                    ) & 0x80000000 as u32
                        == 0 as i32 as u32
                    {
                        if tmp1 as u32
                            & ((if 31 as i32 == 1 as i32 {
                                (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                    + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                            } else {
                                (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32)
                                    + 1 as i32 as i64
                                    >> 1 as i32
                            }) as i32 as u32
                                ^ 0x80000000 as u32)
                            & 0x80000000 as u32
                            != 0
                        {
                            0x80000000 as u32 as i32
                        } else {
                            tmp1 - (if 31 as i32 == 1 as i32 {
                                (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                    + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                            } else {
                                (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32)
                                    + 1 as i32 as i64
                                    >> 1 as i32
                            }) as i32
                        }
                    } else {
                        if (tmp1 as u32 ^ 0x80000000 as u32)
                            & (if 31 as i32 == 1 as i32 {
                                (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                    + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                            } else {
                                (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32)
                                    + 1 as i32 as i64
                                    >> 1 as i32
                            }) as i32 as u32
                            & 0x80000000 as u32
                            != 0
                        {
                            0x7fffffff as i32
                        } else {
                            tmp1 - (if 31 as i32 == 1 as i32 {
                                (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                    + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                            } else {
                                (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32)
                                    + 1 as i32 as i64
                                    >> 1 as i32
                            }) as i32
                        }
                    }) as i64
                        * rc_mult2 as i64
                        & 1 as i32 as i64)
            } else {
                ((if (tmp1 as u32).wrapping_sub(
                    (if 31 as i32 == 1 as i32 {
                        (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                            + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                    } else {
                        (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                            >> 1 as i32
                    }) as i32 as u32,
                ) & 0x80000000 as u32
                    == 0 as i32 as u32
                {
                    if tmp1 as u32
                        & ((if 31 as i32 == 1 as i32 {
                            (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32 as u32
                            ^ 0x80000000 as u32)
                        & 0x80000000 as u32
                        != 0
                    {
                        0x80000000 as u32 as i32
                    } else {
                        tmp1 - (if 31 as i32 == 1 as i32 {
                            (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32
                    }
                } else {
                    if (tmp1 as u32 ^ 0x80000000 as u32)
                        & (if 31 as i32 == 1 as i32 {
                            (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32 as u32
                        & 0x80000000 as u32
                        != 0
                    {
                        0x7fffffff as i32
                    } else {
                        tmp1 - (if 31 as i32 == 1 as i32 {
                            (tmp2 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp2 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp2 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32
                    }
                }) as i64
                    * rc_mult2 as i64
                    >> mult2Q - 1 as i32)
                    + 1 as i32 as i64
                    >> 1 as i32
            };
            if tmp64 > silk_int32_MAX as i64 || tmp64 < silk_int32_MIN as i64 {
                return 0 as i32;
            }
            *A_QA.offset(n as isize) = tmp64 as i32;
            tmp64 = if mult2Q == 1 as i32 {
                ((if (tmp2 as u32).wrapping_sub(
                    (if 31 as i32 == 1 as i32 {
                        (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                            + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                    } else {
                        (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                            >> 1 as i32
                    }) as i32 as u32,
                ) & 0x80000000 as u32
                    == 0 as i32 as u32
                {
                    if tmp2 as u32
                        & ((if 31 as i32 == 1 as i32 {
                            (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32 as u32
                            ^ 0x80000000 as u32)
                        & 0x80000000 as u32
                        != 0
                    {
                        0x80000000 as u32 as i32
                    } else {
                        tmp2 - (if 31 as i32 == 1 as i32 {
                            (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32
                    }
                } else {
                    if (tmp2 as u32 ^ 0x80000000 as u32)
                        & (if 31 as i32 == 1 as i32 {
                            (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32 as u32
                        & 0x80000000 as u32
                        != 0
                    {
                        0x7fffffff as i32
                    } else {
                        tmp2 - (if 31 as i32 == 1 as i32 {
                            (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32
                    }
                }) as i64
                    * rc_mult2 as i64
                    >> 1 as i32)
                    + ((if (tmp2 as u32).wrapping_sub(
                        (if 31 as i32 == 1 as i32 {
                            (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32 as u32,
                    ) & 0x80000000 as u32
                        == 0 as i32 as u32
                    {
                        if tmp2 as u32
                            & ((if 31 as i32 == 1 as i32 {
                                (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                    + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                            } else {
                                (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32)
                                    + 1 as i32 as i64
                                    >> 1 as i32
                            }) as i32 as u32
                                ^ 0x80000000 as u32)
                            & 0x80000000 as u32
                            != 0
                        {
                            0x80000000 as u32 as i32
                        } else {
                            tmp2 - (if 31 as i32 == 1 as i32 {
                                (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                    + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                            } else {
                                (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32)
                                    + 1 as i32 as i64
                                    >> 1 as i32
                            }) as i32
                        }
                    } else {
                        if (tmp2 as u32 ^ 0x80000000 as u32)
                            & (if 31 as i32 == 1 as i32 {
                                (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                    + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                            } else {
                                (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32)
                                    + 1 as i32 as i64
                                    >> 1 as i32
                            }) as i32 as u32
                            & 0x80000000 as u32
                            != 0
                        {
                            0x7fffffff as i32
                        } else {
                            tmp2 - (if 31 as i32 == 1 as i32 {
                                (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                    + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                            } else {
                                (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32)
                                    + 1 as i32 as i64
                                    >> 1 as i32
                            }) as i32
                        }
                    }) as i64
                        * rc_mult2 as i64
                        & 1 as i32 as i64)
            } else {
                ((if (tmp2 as u32).wrapping_sub(
                    (if 31 as i32 == 1 as i32 {
                        (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                            + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                    } else {
                        (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                            >> 1 as i32
                    }) as i32 as u32,
                ) & 0x80000000 as u32
                    == 0 as i32 as u32
                {
                    if tmp2 as u32
                        & ((if 31 as i32 == 1 as i32 {
                            (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32 as u32
                            ^ 0x80000000 as u32)
                        & 0x80000000 as u32
                        != 0
                    {
                        0x80000000 as u32 as i32
                    } else {
                        tmp2 - (if 31 as i32 == 1 as i32 {
                            (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32
                    }
                } else {
                    if (tmp2 as u32 ^ 0x80000000 as u32)
                        & (if 31 as i32 == 1 as i32 {
                            (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32 as u32
                        & 0x80000000 as u32
                        != 0
                    {
                        0x7fffffff as i32
                    } else {
                        tmp2 - (if 31 as i32 == 1 as i32 {
                            (tmp1 as i64 * rc_Q31 as i64 >> 1 as i32)
                                + (tmp1 as i64 * rc_Q31 as i64 & 1 as i32 as i64)
                        } else {
                            (tmp1 as i64 * rc_Q31 as i64 >> 31 as i32 - 1 as i32) + 1 as i32 as i64
                                >> 1 as i32
                        }) as i32
                    }
                }) as i64
                    * rc_mult2 as i64
                    >> mult2Q - 1 as i32)
                    + 1 as i32 as i64
                    >> 1 as i32
            };
            if tmp64 > silk_int32_MAX as i64 || tmp64 < silk_int32_MIN as i64 {
                return 0 as i32;
            }
            *A_QA.offset((k - n - 1 as i32) as isize) = tmp64 as i32;
            n += 1;
        }
        k -= 1;
    }
    if *A_QA.offset(k as isize) > A_LIMIT as i32 || *A_QA.offset(k as isize) < -(A_LIMIT as i32) {
        return 0 as i32;
    }
    rc_Q31 = -(((*A_QA.offset(0 as i32 as isize) as u32) << 31 as i32 - 24 as i32) as i32);
    rc_mult1_Q30 = ((1 as i32 as i64 * ((1 as i32 as i64) << 30 as i32)) as f64 + 0.5f64) as i32
        - (rc_Q31 as i64 * rc_Q31 as i64 >> 32 as i32) as i32;
    invGain_Q30 = (((invGain_Q30 as i64 * rc_mult1_Q30 as i64 >> 32 as i32) as i32 as u32)
        << 2 as i32) as i32;
    if invGain_Q30
        < ((1.0f32 / 1e4f32 * ((1 as i32 as i64) << 30 as i32) as f32) as f64 + 0.5f64) as i32
    {
        return 0 as i32;
    }
    return invGain_Q30;
}
#[c2rust::src_loc = "122:1"]
pub unsafe fn silk_LPC_inverse_pred_gain_c(A_Q12: *const i16, order: i32) -> i32 {
    let mut k: i32 = 0;
    let mut Atmp_QA: [i32; 24] = [0; 24];
    let mut DC_resp: i32 = 0 as i32;
    k = 0 as i32;
    while k < order {
        DC_resp += *A_Q12.offset(k as isize) as i32;
        Atmp_QA[k as usize] =
            ((*A_Q12.offset(k as isize) as i32 as u32) << 24 as i32 - 12 as i32) as i32;
        k += 1;
    }
    if DC_resp >= 4096 as i32 {
        return 0 as i32;
    }
    return LPC_inverse_pred_gain_QA_c(Atmp_QA.as_mut_ptr(), order);
}
