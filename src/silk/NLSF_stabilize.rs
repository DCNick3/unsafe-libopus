use crate::silk::sort::silk_insertion_sort_increasing_all_values_int16;
use crate::silk::SigProc_FIX::{silk_max_int, silk_min_int};

pub mod typedef_h {
    pub const silk_int16_MIN: i32 = 0x8000 as i32;
    pub const silk_int16_MAX: i32 = 0x7fff as i32;
}

pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};

pub const MAX_LOOPS: i32 = 20 as i32;
pub unsafe fn silk_NLSF_stabilize(NLSF_Q15: *mut i16, NDeltaMin_Q15: *const i16, L: i32) {
    let mut i: i32 = 0;
    let mut I: i32 = 0 as i32;
    let mut k: i32 = 0;
    let mut loops: i32 = 0;
    let mut center_freq_Q15: i16 = 0;
    let mut diff_Q15: i32 = 0;
    let mut min_diff_Q15: i32 = 0;
    let mut min_center_Q15: i32 = 0;
    let mut max_center_Q15: i32 = 0;
    loops = 0 as i32;
    while loops < MAX_LOOPS {
        min_diff_Q15 = *NLSF_Q15.offset(0 as i32 as isize) as i32
            - *NDeltaMin_Q15.offset(0 as i32 as isize) as i32;
        I = 0 as i32;
        i = 1 as i32;
        while i <= L - 1 as i32 {
            diff_Q15 = *NLSF_Q15.offset(i as isize) as i32
                - (*NLSF_Q15.offset((i - 1 as i32) as isize) as i32
                    + *NDeltaMin_Q15.offset(i as isize) as i32);
            if diff_Q15 < min_diff_Q15 {
                min_diff_Q15 = diff_Q15;
                I = i;
            }
            i += 1;
        }
        diff_Q15 = ((1 as i32) << 15 as i32)
            - (*NLSF_Q15.offset((L - 1 as i32) as isize) as i32
                + *NDeltaMin_Q15.offset(L as isize) as i32);
        if diff_Q15 < min_diff_Q15 {
            min_diff_Q15 = diff_Q15;
            I = L;
        }
        if min_diff_Q15 >= 0 as i32 {
            return;
        }
        if I == 0 as i32 {
            *NLSF_Q15.offset(0 as i32 as isize) = *NDeltaMin_Q15.offset(0 as i32 as isize);
        } else if I == L {
            *NLSF_Q15.offset((L - 1 as i32) as isize) =
                (((1 as i32) << 15 as i32) - *NDeltaMin_Q15.offset(L as isize) as i32) as i16;
        } else {
            min_center_Q15 = 0 as i32;
            k = 0 as i32;
            while k < I {
                min_center_Q15 += *NDeltaMin_Q15.offset(k as isize) as i32;
                k += 1;
            }
            min_center_Q15 += *NDeltaMin_Q15.offset(I as isize) as i32 >> 1 as i32;
            max_center_Q15 = (1 as i32) << 15 as i32;
            k = L;
            while k > I {
                max_center_Q15 -= *NDeltaMin_Q15.offset(k as isize) as i32;
                k -= 1;
            }
            max_center_Q15 -= *NDeltaMin_Q15.offset(I as isize) as i32 >> 1 as i32;
            center_freq_Q15 = (if min_center_Q15 > max_center_Q15 {
                if (if 1 as i32 == 1 as i32 {
                    (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as i32)
                        + (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                            + *NLSF_Q15.offset(I as isize) as i32
                            & 1 as i32)
                } else {
                    (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as i32 - 1 as i32)
                        + 1 as i32
                        >> 1 as i32
                }) > min_center_Q15
                {
                    min_center_Q15
                } else if (if 1 as i32 == 1 as i32 {
                    (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as i32)
                        + (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                            + *NLSF_Q15.offset(I as isize) as i32
                            & 1 as i32)
                } else {
                    (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as i32 - 1 as i32)
                        + 1 as i32
                        >> 1 as i32
                }) < max_center_Q15
                {
                    max_center_Q15
                } else if 1 as i32 == 1 as i32 {
                    (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as i32)
                        + (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                            + *NLSF_Q15.offset(I as isize) as i32
                            & 1 as i32)
                } else {
                    (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        >> 1 as i32 - 1 as i32)
                        + 1 as i32
                        >> 1 as i32
                }
            } else if (if 1 as i32 == 1 as i32 {
                (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as i32)
                    + (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        & 1 as i32)
            } else {
                (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as i32 - 1 as i32)
                    + 1 as i32
                    >> 1 as i32
            }) > max_center_Q15
            {
                max_center_Q15
            } else if (if 1 as i32 == 1 as i32 {
                (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as i32)
                    + (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        & 1 as i32)
            } else {
                (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as i32 - 1 as i32)
                    + 1 as i32
                    >> 1 as i32
            }) < min_center_Q15
            {
                min_center_Q15
            } else if 1 as i32 == 1 as i32 {
                (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as i32)
                    + (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                        + *NLSF_Q15.offset(I as isize) as i32
                        & 1 as i32)
            } else {
                (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                    + *NLSF_Q15.offset(I as isize) as i32
                    >> 1 as i32 - 1 as i32)
                    + 1 as i32
                    >> 1 as i32
            }) as i16;
            *NLSF_Q15.offset((I - 1 as i32) as isize) = (center_freq_Q15 as i32
                - (*NDeltaMin_Q15.offset(I as isize) as i32 >> 1 as i32))
                as i16;
            *NLSF_Q15.offset(I as isize) = (*NLSF_Q15.offset((I - 1 as i32) as isize) as i32
                + *NDeltaMin_Q15.offset(I as isize) as i32)
                as i16;
        }
        loops += 1;
    }
    if loops == MAX_LOOPS {
        silk_insertion_sort_increasing_all_values_int16(
            &mut *NLSF_Q15.offset(0 as i32 as isize),
            L,
        );
        *NLSF_Q15.offset(0 as i32 as isize) = silk_max_int(
            *NLSF_Q15.offset(0 as i32 as isize) as i32,
            *NDeltaMin_Q15.offset(0 as i32 as isize) as i32,
        ) as i16;
        i = 1 as i32;
        while i < L {
            *NLSF_Q15.offset(i as isize) = silk_max_int(
                *NLSF_Q15.offset(i as isize) as i32,
                (if *NLSF_Q15.offset((i - 1 as i32) as isize) as i32
                    + *NDeltaMin_Q15.offset(i as isize) as i32
                    > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (*NLSF_Q15.offset((i - 1 as i32) as isize) as i32
                    + *NDeltaMin_Q15.offset(i as isize) as i32)
                    < silk_int16_MIN
                {
                    silk_int16_MIN
                } else {
                    *NLSF_Q15.offset((i - 1 as i32) as isize) as i32
                        + *NDeltaMin_Q15.offset(i as isize) as i32
                }) as i16 as i32,
            ) as i16;
            i += 1;
        }
        *NLSF_Q15.offset((L - 1 as i32) as isize) = silk_min_int(
            *NLSF_Q15.offset((L - 1 as i32) as isize) as i32,
            ((1 as i32) << 15 as i32) - *NDeltaMin_Q15.offset(L as isize) as i32,
        ) as i16;
        i = L - 2 as i32;
        while i >= 0 as i32 {
            *NLSF_Q15.offset(i as isize) = silk_min_int(
                *NLSF_Q15.offset(i as isize) as i32,
                *NLSF_Q15.offset((i + 1 as i32) as isize) as i32
                    - *NDeltaMin_Q15.offset((i + 1 as i32) as isize) as i32,
            ) as i16;
            i -= 1;
        }
    }
}
