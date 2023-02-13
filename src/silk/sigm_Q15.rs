#[c2rust::src_loc = "37:25"]
static mut sigm_LUT_slope_Q10: [i32; 6] = [
    237 as i32, 153 as i32, 73 as i32, 30 as i32, 12 as i32, 7 as i32,
];
#[c2rust::src_loc = "41:25"]
static mut sigm_LUT_pos_Q15: [i32; 6] = [
    16384 as i32,
    23955 as i32,
    28861 as i32,
    31213 as i32,
    32178 as i32,
    32548 as i32,
];
#[c2rust::src_loc = "45:25"]
static mut sigm_LUT_neg_Q15: [i32; 6] = [
    16384 as i32,
    8812 as i32,
    3906 as i32,
    1554 as i32,
    589 as i32,
    219 as i32,
];
#[c2rust::src_loc = "49:1"]
pub unsafe fn silk_sigm_Q15(mut in_Q5: i32) -> i32 {
    let mut ind: i32 = 0;
    if in_Q5 < 0 as i32 {
        in_Q5 = -in_Q5;
        if in_Q5 >= 6 as i32 * 32 as i32 {
            return 0 as i32;
        } else {
            ind = in_Q5 >> 5 as i32;
            return sigm_LUT_neg_Q15[ind as usize]
                - sigm_LUT_slope_Q10[ind as usize] as i16 as i32
                    * (in_Q5 & 0x1f as i32) as i16 as i32;
        }
    } else if in_Q5 >= 6 as i32 * 32 as i32 {
        return 32767 as i32;
    } else {
        ind = in_Q5 >> 5 as i32;
        return sigm_LUT_pos_Q15[ind as usize]
            + sigm_LUT_slope_Q10[ind as usize] as i16 as i32 * (in_Q5 & 0x1f as i32) as i16 as i32;
    };
}
