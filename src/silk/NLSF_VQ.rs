pub unsafe fn silk_NLSF_VQ(
    err_Q24: *mut i32,
    in_Q15: *const i16,
    pCB_Q8: &[u8],
    pWght_Q9: &[i16],
    K: i32,
    LPC_order: i32,
) {
    let mut i: i32 = 0;
    let mut m: i32 = 0;
    let mut diff_Q15: i32 = 0;
    let mut diffw_Q24: i32 = 0;
    let mut sum_error_Q24: i32 = 0;
    let mut pred_Q24: i32 = 0;
    assert!(LPC_order & 1 == 0);
    let mut cb_Q8_ptr = pCB_Q8;
    let mut w_Q9_ptr = pWght_Q9;
    i = 0;
    while i < K {
        sum_error_Q24 = 0;
        pred_Q24 = 0;
        m = LPC_order - 2;
        while m >= 0 {
            diff_Q15 = *in_Q15.offset((m + 1) as isize) as i32
                - ((cb_Q8_ptr[(m + 1) as usize] as i32 as u32) << 7) as i32;
            diffw_Q24 = diff_Q15 as i16 as i32 * w_Q9_ptr[(m + 1) as usize] as i32;
            sum_error_Q24 = sum_error_Q24
                + (if diffw_Q24 - (pred_Q24 >> 1) > 0 {
                    diffw_Q24 - (pred_Q24 >> 1)
                } else {
                    -(diffw_Q24 - (pred_Q24 >> 1))
                });
            pred_Q24 = diffw_Q24;
            diff_Q15 = *in_Q15.offset(m as isize) as i32
                - ((cb_Q8_ptr[m as usize] as i32 as u32) << 7) as i32;
            diffw_Q24 = diff_Q15 as i16 as i32 * w_Q9_ptr[m as usize] as i32;
            sum_error_Q24 = sum_error_Q24
                + (if diffw_Q24 - (pred_Q24 >> 1) > 0 {
                    diffw_Q24 - (pred_Q24 >> 1)
                } else {
                    -(diffw_Q24 - (pred_Q24 >> 1))
                });
            pred_Q24 = diffw_Q24;
            m -= 2;
        }
        *err_Q24.offset(i as isize) = sum_error_Q24;
        cb_Q8_ptr = &cb_Q8_ptr[LPC_order as usize..];
        w_Q9_ptr = &w_Q9_ptr[LPC_order as usize..];
        i += 1;
    }
}
