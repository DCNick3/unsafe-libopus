use crate::silk::sort::silk_insertion_sort_increasing_all_values_int16;
use crate::silk::SigProc_FIX::{silk_max_int, silk_min_int};
use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:41"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}

pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};

#[c2rust::src_loc = "44:9"]
pub const MAX_LOOPS: libc::c_int = 20 as libc::c_int;
#[c2rust::src_loc = "47:1"]
pub unsafe fn silk_NLSF_stabilize(NLSF_Q15: *mut i16, NDeltaMin_Q15: *const i16, L: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut I: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut loops: libc::c_int = 0;
    let mut center_freq_Q15: i16 = 0;
    let mut diff_Q15: i32 = 0;
    let mut min_diff_Q15: i32 = 0;
    let mut min_center_Q15: i32 = 0;
    let mut max_center_Q15: i32 = 0;
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
            *NLSF_Q15.offset(0 as libc::c_int as isize) =
                *NDeltaMin_Q15.offset(0 as libc::c_int as isize);
        } else if I == L {
            *NLSF_Q15.offset((L - 1 as libc::c_int) as isize) =
                (((1 as libc::c_int) << 15 as libc::c_int)
                    - *NDeltaMin_Q15.offset(L as isize) as libc::c_int) as i16;
        } else {
            min_center_Q15 = 0 as libc::c_int;
            k = 0 as libc::c_int;
            while k < I {
                min_center_Q15 += *NDeltaMin_Q15.offset(k as isize) as libc::c_int;
                k += 1;
            }
            min_center_Q15 += *NDeltaMin_Q15.offset(I as isize) as libc::c_int >> 1 as libc::c_int;
            max_center_Q15 = (1 as libc::c_int) << 15 as libc::c_int;
            k = L;
            while k > I {
                max_center_Q15 -= *NDeltaMin_Q15.offset(k as isize) as libc::c_int;
                k -= 1;
            }
            max_center_Q15 -= *NDeltaMin_Q15.offset(I as isize) as libc::c_int >> 1 as libc::c_int;
            center_freq_Q15 = (if min_center_Q15 > max_center_Q15 {
                if (if 1 as libc::c_int == 1 as libc::c_int {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as libc::c_int)
                        + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                            + *NLSF_Q15.offset(I as isize) as i32
                            & 1 as libc::c_int)
                } else {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) > min_center_Q15
                {
                    min_center_Q15
                } else if (if 1 as libc::c_int == 1 as libc::c_int {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as libc::c_int)
                        + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                            + *NLSF_Q15.offset(I as isize) as i32
                            & 1 as libc::c_int)
                } else {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) < max_center_Q15
                {
                    max_center_Q15
                } else if 1 as libc::c_int == 1 as libc::c_int {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as libc::c_int)
                        + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                            + *NLSF_Q15.offset(I as isize) as i32
                            & 1 as libc::c_int)
                } else {
                    (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int
                        >> 1 as libc::c_int
                }
            } else if (if 1 as libc::c_int == 1 as libc::c_int {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as libc::c_int)
                    + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        & 1 as libc::c_int)
            } else {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > max_center_Q15
            {
                max_center_Q15
            } else if (if 1 as libc::c_int == 1 as libc::c_int {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as libc::c_int)
                    + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        & 1 as libc::c_int)
            } else {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < min_center_Q15
            {
                min_center_Q15
            } else if 1 as libc::c_int == 1 as libc::c_int {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as libc::c_int)
                    + (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        & 1 as libc::c_int)
            } else {
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as i16;
            *NLSF_Q15.offset((I - 1 as libc::c_int) as isize) = (center_freq_Q15 as libc::c_int
                - (*NDeltaMin_Q15.offset(I as isize) as libc::c_int >> 1 as libc::c_int))
                as i16;
            *NLSF_Q15.offset(I as isize) =
                (*NLSF_Q15.offset((I - 1 as libc::c_int) as isize) as libc::c_int
                    + *NDeltaMin_Q15.offset(I as isize) as libc::c_int) as i16;
        }
        loops += 1;
    }
    if loops == MAX_LOOPS {
        silk_insertion_sort_increasing_all_values_int16(
            &mut *NLSF_Q15.offset(0 as libc::c_int as isize),
            L,
        );
        *NLSF_Q15.offset(0 as libc::c_int as isize) = silk_max_int(
            *NLSF_Q15.offset(0 as libc::c_int as isize) as libc::c_int,
            *NDeltaMin_Q15.offset(0 as libc::c_int as isize) as libc::c_int,
        ) as i16;
        i = 1 as libc::c_int;
        while i < L {
            *NLSF_Q15.offset(i as isize) = silk_max_int(
                *NLSF_Q15.offset(i as isize) as libc::c_int,
                (if *NLSF_Q15.offset((i - 1 as libc::c_int) as isize) as i32
                    + *NDeltaMin_Q15.offset(i as isize) as libc::c_int
                    > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (*NLSF_Q15.offset((i - 1 as libc::c_int) as isize) as i32
                    + *NDeltaMin_Q15.offset(i as isize) as libc::c_int)
                    < silk_int16_MIN
                {
                    silk_int16_MIN
                } else {
                    *NLSF_Q15.offset((i - 1 as libc::c_int) as isize) as i32
                        + *NDeltaMin_Q15.offset(i as isize) as libc::c_int
                }) as i16 as libc::c_int,
            ) as i16;
            i += 1;
        }
        *NLSF_Q15.offset((L - 1 as libc::c_int) as isize) = silk_min_int(
            *NLSF_Q15.offset((L - 1 as libc::c_int) as isize) as libc::c_int,
            ((1 as libc::c_int) << 15 as libc::c_int)
                - *NDeltaMin_Q15.offset(L as isize) as libc::c_int,
        ) as i16;
        i = L - 2 as libc::c_int;
        while i >= 0 as libc::c_int {
            *NLSF_Q15.offset(i as isize) = silk_min_int(
                *NLSF_Q15.offset(i as isize) as libc::c_int,
                *NLSF_Q15.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *NDeltaMin_Q15.offset((i + 1 as libc::c_int) as isize) as libc::c_int,
            ) as i16;
            i -= 1;
        }
    }
}
