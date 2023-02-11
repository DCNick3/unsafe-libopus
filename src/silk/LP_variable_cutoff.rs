use ::libc;

use crate::externs::memcpy;
use crate::silk::biquad_alt::silk_biquad_alt_stride1;
use crate::silk::define::{TRANSITION_INT_NUM, TRANSITION_NA, TRANSITION_NB};
use crate::silk::structs::silk_LP_state;
use crate::silk::tables_other::{silk_Transition_LP_A_Q28, silk_Transition_LP_B_Q28};

#[inline]
#[c2rust::src_loc = "41:1"]
unsafe extern "C" fn silk_LP_interpolate_filter_taps(
    B_Q28: *mut i32,
    A_Q28: *mut i32,
    ind: libc::c_int,
    fac_Q16: i32,
) {
    let mut nb: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    if ind < TRANSITION_INT_NUM - 1 as libc::c_int {
        if fac_Q16 > 0 as libc::c_int {
            if fac_Q16 < 32768 as libc::c_int {
                nb = 0 as libc::c_int;
                while nb < TRANSITION_NB {
                    *B_Q28.offset(nb as isize) =
                        (silk_Transition_LP_B_Q28[ind as usize][nb as usize] as libc::c_long
                            + ((silk_Transition_LP_B_Q28[(ind + 1 as libc::c_int) as usize]
                                [nb as usize]
                                - silk_Transition_LP_B_Q28[ind as usize][nb as usize])
                                as libc::c_long
                                * fac_Q16 as i16 as i64
                                >> 16 as libc::c_int)) as i32;
                    nb += 1;
                }
                na = 0 as libc::c_int;
                while na < TRANSITION_NA {
                    *A_Q28.offset(na as isize) =
                        (silk_Transition_LP_A_Q28[ind as usize][na as usize] as libc::c_long
                            + ((silk_Transition_LP_A_Q28[(ind + 1 as libc::c_int) as usize]
                                [na as usize]
                                - silk_Transition_LP_A_Q28[ind as usize][na as usize])
                                as libc::c_long
                                * fac_Q16 as i16 as i64
                                >> 16 as libc::c_int)) as i32;
                    na += 1;
                }
            } else {
                nb = 0 as libc::c_int;
                while nb < TRANSITION_NB {
                    *B_Q28.offset(nb as isize) = (silk_Transition_LP_B_Q28
                        [(ind + 1 as libc::c_int) as usize][nb as usize]
                        as libc::c_long
                        + ((silk_Transition_LP_B_Q28[(ind + 1 as libc::c_int) as usize]
                            [nb as usize]
                            - silk_Transition_LP_B_Q28[ind as usize][nb as usize])
                            as libc::c_long
                            * (fac_Q16 - ((1 as libc::c_int) << 16 as libc::c_int)) as i16 as i64
                            >> 16 as libc::c_int))
                        as i32;
                    nb += 1;
                }
                na = 0 as libc::c_int;
                while na < TRANSITION_NA {
                    *A_Q28.offset(na as isize) = (silk_Transition_LP_A_Q28
                        [(ind + 1 as libc::c_int) as usize][na as usize]
                        as libc::c_long
                        + ((silk_Transition_LP_A_Q28[(ind + 1 as libc::c_int) as usize]
                            [na as usize]
                            - silk_Transition_LP_A_Q28[ind as usize][na as usize])
                            as libc::c_long
                            * (fac_Q16 - ((1 as libc::c_int) << 16 as libc::c_int)) as i16 as i64
                            >> 16 as libc::c_int))
                        as i32;
                    na += 1;
                }
            }
        } else {
            memcpy(
                B_Q28 as *mut libc::c_void,
                (silk_Transition_LP_B_Q28[ind as usize]).as_ptr() as *const libc::c_void,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
            );
            memcpy(
                A_Q28 as *mut libc::c_void,
                (silk_Transition_LP_A_Q28[ind as usize]).as_ptr() as *const libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
            );
        }
    } else {
        memcpy(
            B_Q28 as *mut libc::c_void,
            (silk_Transition_LP_B_Q28[(5 as libc::c_int - 1 as libc::c_int) as usize]).as_ptr()
                as *const libc::c_void,
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
        memcpy(
            A_Q28 as *mut libc::c_void,
            (silk_Transition_LP_A_Q28[(5 as libc::c_int - 1 as libc::c_int) as usize]).as_ptr()
                as *const libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
    };
}
#[c2rust::src_loc = "100:1"]
pub unsafe extern "C" fn silk_LP_variable_cutoff(
    mut psLP: *mut silk_LP_state,
    frame: *mut i16,
    frame_length: libc::c_int,
) {
    let mut B_Q28: [i32; 3] = [0; 3];
    let mut A_Q28: [i32; 2] = [0; 2];
    let mut fac_Q16: i32 = 0 as libc::c_int;
    let mut ind: libc::c_int = 0 as libc::c_int;
    if (*psLP).mode != 0 as libc::c_int {
        fac_Q16 = (((5120 as libc::c_int / (5 as libc::c_int * 4 as libc::c_int)
            - (*psLP).transition_frame_no) as u32)
            << 16 as libc::c_int - 6 as libc::c_int) as i32;
        ind = fac_Q16 >> 16 as libc::c_int;
        fac_Q16 -= ((ind as u32) << 16 as libc::c_int) as i32;
        silk_LP_interpolate_filter_taps(B_Q28.as_mut_ptr(), A_Q28.as_mut_ptr(), ind, fac_Q16);
        (*psLP).transition_frame_no =
            if 0 as libc::c_int > 5120 as libc::c_int / (5 as libc::c_int * 4 as libc::c_int) {
                if (*psLP).transition_frame_no + (*psLP).mode > 0 as libc::c_int {
                    0 as libc::c_int
                } else if (*psLP).transition_frame_no + (*psLP).mode
                    < 5120 as libc::c_int / (5 as libc::c_int * 4 as libc::c_int)
                {
                    5120 as libc::c_int / (5 as libc::c_int * 4 as libc::c_int)
                } else {
                    (*psLP).transition_frame_no + (*psLP).mode
                }
            } else if (*psLP).transition_frame_no + (*psLP).mode
                > 5120 as libc::c_int / (5 as libc::c_int * 4 as libc::c_int)
            {
                5120 as libc::c_int / (5 as libc::c_int * 4 as libc::c_int)
            } else if (*psLP).transition_frame_no + (*psLP).mode < 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (*psLP).transition_frame_no + (*psLP).mode
            };
        silk_biquad_alt_stride1(
            frame,
            B_Q28.as_mut_ptr(),
            A_Q28.as_mut_ptr(),
            ((*psLP).In_LP_State).as_mut_ptr(),
            frame,
            frame_length,
        );
    }
}
