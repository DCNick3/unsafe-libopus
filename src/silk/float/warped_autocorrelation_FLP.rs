use crate::celt::celt::celt_fatal;
#[c2rust::src_loc = "35:1"]
pub unsafe fn silk_warped_autocorrelation_FLP(
    corr: *mut f32,
    input: *const f32,
    warping: f32,
    length: i32,
    order: i32,
) {
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut tmp1: f64 = 0.;
    let mut tmp2: f64 = 0.;
    let mut state: [f64; 25] = [
        0 as i32 as f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut C: [f64; 25] = [
        0 as i32 as f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    if !(order & 1 as i32 == 0 as i32) {
        celt_fatal(
            b"assertion failed: ( order & 1 ) == 0\0" as *const u8 as *const i8,
            b"silk/float/warped_autocorrelation_FLP.c\0" as *const u8 as *const i8,
            49 as i32,
        );
    }
    n = 0 as i32;
    while n < length {
        tmp1 = *input.offset(n as isize) as f64;
        i = 0 as i32;
        while i < order {
            tmp2 = state[i as usize] + warping as f64 * (state[(i + 1 as i32) as usize] - tmp1);
            state[i as usize] = tmp1;
            C[i as usize] += state[0 as i32 as usize] * tmp1;
            tmp1 = state[(i + 1 as i32) as usize]
                + warping as f64 * (state[(i + 2 as i32) as usize] - tmp2);
            state[(i + 1 as i32) as usize] = tmp2;
            C[(i + 1 as i32) as usize] += state[0 as i32 as usize] * tmp2;
            i += 2 as i32;
        }
        state[order as usize] = tmp1;
        C[order as usize] += state[0 as i32 as usize] * tmp1;
        n += 1;
    }
    i = 0 as i32;
    while i < order + 1 as i32 {
        *corr.offset(i as isize) = C[i as usize] as f32;
        i += 1;
    }
}
