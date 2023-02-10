use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:41"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:41"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:41"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:41"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "546:1"]
    pub unsafe extern "C" fn silk_min_int(
        a: libc::c_int,
        b: libc::c_int,
    ) -> libc::c_int {
        return if a < b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "564:1"]
    pub unsafe extern "C" fn silk_max_int(
        a: libc::c_int,
        b: libc::c_int,
    ) -> libc::c_int {
        return if a > b { a } else { b };
    }
    use super::opus_types_h::opus_int16;
    extern "C" {
        #[c2rust::src_loc = "316:1"]
        pub fn silk_insertion_sort_increasing_all_values_int16(
            a: *mut opus_int16,
            L: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:41"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
pub use self::types_h::{__int16_t, __int32_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::opus_types_h::{opus_int16, opus_int32};
pub use self::SigProc_FIX_h::{
    silk_min_int, silk_max_int, silk_insertion_sort_increasing_all_values_int16,
};
pub use self::typedef_h::{silk_int16_MIN, silk_int16_MAX};
#[c2rust::src_loc = "44:9"]
pub const MAX_LOOPS: libc::c_int = 20 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn silk_NLSF_stabilize(
    NLSF_Q15: *mut opus_int16,
    NDeltaMin_Q15: *const opus_int16,
    L: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut I: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut loops: libc::c_int = 0;
    let mut center_freq_Q15: opus_int16 = 0;
    let mut diff_Q15: opus_int32 = 0;
    let mut min_diff_Q15: opus_int32 = 0;
    let mut min_center_Q15: opus_int32 = 0;
    let mut max_center_Q15: opus_int32 = 0;
    loops = 0 as libc::c_int;
    while loops < MAX_LOOPS {
        min_diff_Q15 = *NLSF_Q15.offset(0 as libc::c_int as isize) as libc::c_int
            - *NDeltaMin_Q15.offset(0 as libc::c_int as isize) as libc::c_int;
        I = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= L - 1 as libc::c_int {
            diff_Q15 = *NLSF_Q15.offset(i as isize) as libc::c_int
                - (*NLSF_Q15.offset((i - 1 as libc::c_int) as isize) as libc::c_int
                    + *NDeltaMin_Q15.offset(i as isize) as libc::c_int);
            if diff_Q15 < min_diff_Q15 {
                min_diff_Q15 = diff_Q15;
                I = i;
            }
            i += 1;
        }
        diff_Q15 = ((1 as libc::c_int) << 15 as libc::c_int)
            - (*NLSF_Q15.offset((L - 1 as libc::c_int) as isize) as libc::c_int
                + *NDeltaMin_Q15.offset(L as isize) as libc::c_int);
        if diff_Q15 < min_diff_Q15 {
            min_diff_Q15 = diff_Q15;
            I = L;
        }
        if min_diff_Q15 >= 0 as libc::c_int {
            return;
        }
        if I == 0 as libc::c_int {
            *NLSF_Q15
                .offset(
                    0 as libc::c_int as isize,
                ) = *NDeltaMin_Q15.offset(0 as libc::c_int as isize);
        } else if I == L {
            *NLSF_Q15
                .offset(
                    (L - 1 as libc::c_int) as isize,
                ) = (((1 as libc::c_int) << 15 as libc::c_int)
                - *NDeltaMin_Q15.offset(L as isize) as libc::c_int) as opus_int16;
        } else {
            min_center_Q15 = 0 as libc::c_int;
            k = 0 as libc::c_int;
            while k < I {
                min_center_Q15 += *NDeltaMin_Q15.offset(k as isize) as libc::c_int;
                k += 1;
            }
            min_center_Q15
                += *NDeltaMin_Q15.offset(I as isize) as libc::c_int >> 1 as libc::c_int;
            max_center_Q15 = (1 as libc::c_int) << 15 as libc::c_int;
            k = L;
            while k > I {
                max_center_Q15 -= *NDeltaMin_Q15.offset(k as isize) as libc::c_int;
                k -= 1;
            }
            max_center_Q15
                -= *NDeltaMin_Q15.offset(I as isize) as libc::c_int >> 1 as libc::c_int;
            center_freq_Q15 = (if min_center_Q15 > max_center_Q15 {
                if (if 1 as libc::c_int == 1 as libc::c_int {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                        + *NLSF_Q15.offset(I as isize) as opus_int32 >> 1 as libc::c_int)
                        + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                            as opus_int32 + *NLSF_Q15.offset(I as isize) as opus_int32
                            & 1 as libc::c_int)
                } else {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                        + *NLSF_Q15.offset(I as isize) as opus_int32
                        >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) > min_center_Q15
                {
                    min_center_Q15
                } else if (if 1 as libc::c_int == 1 as libc::c_int {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                        + *NLSF_Q15.offset(I as isize) as opus_int32 >> 1 as libc::c_int)
                        + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                            as opus_int32 + *NLSF_Q15.offset(I as isize) as opus_int32
                            & 1 as libc::c_int)
                } else {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                        + *NLSF_Q15.offset(I as isize) as opus_int32
                        >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) < max_center_Q15
                {
                    max_center_Q15
                } else if 1 as libc::c_int == 1 as libc::c_int {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                        + *NLSF_Q15.offset(I as isize) as opus_int32 >> 1 as libc::c_int)
                        + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize)
                            as opus_int32 + *NLSF_Q15.offset(I as isize) as opus_int32
                            & 1 as libc::c_int)
                } else {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                        + *NLSF_Q15.offset(I as isize) as opus_int32
                        >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }
            } else if (if 1 as libc::c_int == 1 as libc::c_int {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                    + *NLSF_Q15.offset(I as isize) as opus_int32 >> 1 as libc::c_int)
                    + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                        + *NLSF_Q15.offset(I as isize) as opus_int32 & 1 as libc::c_int)
            } else {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                    + *NLSF_Q15.offset(I as isize) as opus_int32
                    >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > max_center_Q15
            {
                max_center_Q15
            } else if (if 1 as libc::c_int == 1 as libc::c_int {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                    + *NLSF_Q15.offset(I as isize) as opus_int32 >> 1 as libc::c_int)
                    + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                        + *NLSF_Q15.offset(I as isize) as opus_int32 & 1 as libc::c_int)
            } else {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                    + *NLSF_Q15.offset(I as isize) as opus_int32
                    >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < min_center_Q15
            {
                min_center_Q15
            } else if 1 as libc::c_int == 1 as libc::c_int {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                    + *NLSF_Q15.offset(I as isize) as opus_int32 >> 1 as libc::c_int)
                    + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                        + *NLSF_Q15.offset(I as isize) as opus_int32 & 1 as libc::c_int)
            } else {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as opus_int32
                    + *NLSF_Q15.offset(I as isize) as opus_int32
                    >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as opus_int16;
            *NLSF_Q15
                .offset(
                    (I - 1 as libc::c_int) as isize,
                ) = (center_freq_Q15 as libc::c_int
                - (*NDeltaMin_Q15.offset(I as isize) as libc::c_int >> 1 as libc::c_int))
                as opus_int16;
            *NLSF_Q15
                .offset(
                    I as isize,
                ) = (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as libc::c_int
                + *NDeltaMin_Q15.offset(I as isize) as libc::c_int) as opus_int16;
        }
        loops += 1;
    }
    if loops == MAX_LOOPS {
        silk_insertion_sort_increasing_all_values_int16(
            &mut *NLSF_Q15.offset(0 as libc::c_int as isize),
            L,
        );
        *NLSF_Q15
            .offset(
                0 as libc::c_int as isize,
            ) = silk_max_int(
            *NLSF_Q15.offset(0 as libc::c_int as isize) as libc::c_int,
            *NDeltaMin_Q15.offset(0 as libc::c_int as isize) as libc::c_int,
        ) as opus_int16;
        i = 1 as libc::c_int;
        while i < L {
            *NLSF_Q15
                .offset(
                    i as isize,
                ) = silk_max_int(
                *NLSF_Q15.offset(i as isize) as libc::c_int,
                (if *NLSF_Q15.offset((i - 1 as libc::c_int) as isize) as opus_int32
                    + *NDeltaMin_Q15.offset(i as isize) as libc::c_int > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (*NLSF_Q15.offset((i - 1 as libc::c_int) as isize)
                    as opus_int32 + *NDeltaMin_Q15.offset(i as isize) as libc::c_int)
                    < silk_int16_MIN
                {
                    silk_int16_MIN
                } else {
                    *NLSF_Q15.offset((i - 1 as libc::c_int) as isize) as opus_int32
                        + *NDeltaMin_Q15.offset(i as isize) as libc::c_int
                }) as opus_int16 as libc::c_int,
            ) as opus_int16;
            i += 1;
        }
        *NLSF_Q15
            .offset(
                (L - 1 as libc::c_int) as isize,
            ) = silk_min_int(
            *NLSF_Q15.offset((L - 1 as libc::c_int) as isize) as libc::c_int,
            ((1 as libc::c_int) << 15 as libc::c_int)
                - *NDeltaMin_Q15.offset(L as isize) as libc::c_int,
        ) as opus_int16;
        i = L - 2 as libc::c_int;
        while i >= 0 as libc::c_int {
            *NLSF_Q15
                .offset(
                    i as isize,
                ) = silk_min_int(
                *NLSF_Q15.offset(i as isize) as libc::c_int,
                *NLSF_Q15.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *NDeltaMin_Q15.offset((i + 1 as libc::c_int) as isize)
                        as libc::c_int,
            ) as opus_int16;
            i -= 1;
        }
    }
}
