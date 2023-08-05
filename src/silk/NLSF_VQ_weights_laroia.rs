pub mod typedef_h {
    pub const silk_int16_MAX: i32 = i16::MAX as i32;
}
pub use self::typedef_h::silk_int16_MAX;
use crate::silk::SigProc_FIX::{silk_max_int, silk_min_int};

pub unsafe fn silk_NLSF_VQ_weights_laroia(pNLSFW_Q_OUT: *mut i16, pNLSF_Q15: *const i16, D: i32) {
    let mut k: i32 = 0;
    let mut tmp1_int: i32 = 0;
    let mut tmp2_int: i32 = 0;
    assert!(D > 0 as i32);
    assert!(D & 1 as i32 == 0 as i32);
    tmp1_int = silk_max_int(*pNLSF_Q15.offset(0 as i32 as isize) as i32, 1 as i32);
    tmp1_int = ((1 as i32) << 15 as i32 + 2 as i32) / tmp1_int;
    tmp2_int = silk_max_int(
        *pNLSF_Q15.offset(1 as i32 as isize) as i32 - *pNLSF_Q15.offset(0 as i32 as isize) as i32,
        1 as i32,
    );
    tmp2_int = ((1 as i32) << 15 as i32 + 2 as i32) / tmp2_int;
    *pNLSFW_Q_OUT.offset(0 as i32 as isize) =
        silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as i16;
    k = 1 as i32;
    while k < D - 1 as i32 {
        tmp1_int = silk_max_int(
            *pNLSF_Q15.offset((k + 1 as i32) as isize) as i32
                - *pNLSF_Q15.offset(k as isize) as i32,
            1 as i32,
        );
        tmp1_int = ((1 as i32) << 15 as i32 + 2 as i32) / tmp1_int;
        *pNLSFW_Q_OUT.offset(k as isize) = silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as i16;
        tmp2_int = silk_max_int(
            *pNLSF_Q15.offset((k + 2 as i32) as isize) as i32
                - *pNLSF_Q15.offset((k + 1 as i32) as isize) as i32,
            1 as i32,
        );
        tmp2_int = ((1 as i32) << 15 as i32 + 2 as i32) / tmp2_int;
        *pNLSFW_Q_OUT.offset((k + 1 as i32) as isize) =
            silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as i16;
        k += 2 as i32;
    }
    tmp1_int = silk_max_int(
        ((1 as i32) << 15 as i32) - *pNLSF_Q15.offset((D - 1 as i32) as isize) as i32,
        1 as i32,
    );
    tmp1_int = ((1 as i32) << 15 as i32 + 2 as i32) / tmp1_int;
    *pNLSFW_Q_OUT.offset((D - 1 as i32) as isize) =
        silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as i16;
}
