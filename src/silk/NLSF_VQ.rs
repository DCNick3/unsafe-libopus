use crate::celt::celt::celt_fatal;

pub unsafe fn silk_NLSF_VQ(
    err_Q24: *mut i32,
    in_Q15: *const i16,
    pCB_Q8: *const u8,
    pWght_Q9: *const i16,
    K: i32,
    LPC_order: i32,
) {
    let mut i: i32 = 0;
    let mut m: i32 = 0;
    let mut diff_Q15: i32 = 0;
    let mut diffw_Q24: i32 = 0;
    let mut sum_error_Q24: i32 = 0;
    let mut pred_Q24: i32 = 0;
    let mut w_Q9_ptr: *const i16 = 0 as *const i16;
    let mut cb_Q8_ptr: *const u8 = 0 as *const u8;
    if !(LPC_order & 1 as i32 == 0 as i32) {
        celt_fatal(
            b"assertion failed: ( LPC_order & 1 ) == 0\0" as *const u8 as *const i8,
            b"silk/NLSF_VQ.c\0" as *const u8 as *const i8,
            49 as i32,
        );
    }
    cb_Q8_ptr = pCB_Q8;
    w_Q9_ptr = pWght_Q9;
    i = 0 as i32;
    while i < K {
        sum_error_Q24 = 0 as i32;
        pred_Q24 = 0 as i32;
        m = LPC_order - 2 as i32;
        while m >= 0 as i32 {
            diff_Q15 = *in_Q15.offset((m + 1 as i32) as isize) as i32
                - ((*cb_Q8_ptr.offset((m + 1 as i32) as isize) as i32 as u32) << 7 as i32) as i32;
            diffw_Q24 = diff_Q15 as i16 as i32 * *w_Q9_ptr.offset((m + 1 as i32) as isize) as i32;
            sum_error_Q24 = sum_error_Q24
                + (if diffw_Q24 - (pred_Q24 >> 1 as i32) > 0 as i32 {
                    diffw_Q24 - (pred_Q24 >> 1 as i32)
                } else {
                    -(diffw_Q24 - (pred_Q24 >> 1 as i32))
                });
            pred_Q24 = diffw_Q24;
            diff_Q15 = *in_Q15.offset(m as isize) as i32
                - ((*cb_Q8_ptr.offset(m as isize) as i32 as u32) << 7 as i32) as i32;
            diffw_Q24 = diff_Q15 as i16 as i32 * *w_Q9_ptr.offset(m as isize) as i32;
            sum_error_Q24 = sum_error_Q24
                + (if diffw_Q24 - (pred_Q24 >> 1 as i32) > 0 as i32 {
                    diffw_Q24 - (pred_Q24 >> 1 as i32)
                } else {
                    -(diffw_Q24 - (pred_Q24 >> 1 as i32))
                });
            pred_Q24 = diffw_Q24;
            m -= 2 as i32;
        }
        *err_Q24.offset(i as isize) = sum_error_Q24;
        cb_Q8_ptr = cb_Q8_ptr.offset(LPC_order as isize);
        w_Q9_ptr = w_Q9_ptr.offset(LPC_order as isize);
        i += 1;
    }
}
