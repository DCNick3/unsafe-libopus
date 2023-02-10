use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
pub mod arch_h {}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:33"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "546:1"]
    pub unsafe extern "C" fn silk_min_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
        return if a < b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "564:1"]
    pub unsafe extern "C" fn silk_max_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
        return if a > b { a } else { b };
    }
}
use crate::celt::celt::celt_fatal;

pub use self::typedef_h::silk_int16_MAX;

pub use self::SigProc_FIX_h::{silk_max_int, silk_min_int};
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn silk_NLSF_VQ_weights_laroia(
    pNLSFW_Q_OUT: *mut i16,
    pNLSF_Q15: *const i16,
    D: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut tmp1_int: i32 = 0;
    let mut tmp2_int: i32 = 0;
    if !(D > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: D > 0\0" as *const u8 as *const libc::c_char,
            b"silk/NLSF_VQ_weights_laroia.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
        );
    }
    if !(D & 1 as libc::c_int == 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: ( D & 1 ) == 0\0" as *const u8 as *const libc::c_char,
            b"silk/NLSF_VQ_weights_laroia.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
        );
    }
    tmp1_int = silk_max_int(
        *pNLSF_Q15.offset(0 as libc::c_int as isize) as libc::c_int,
        1 as libc::c_int,
    );
    tmp1_int = ((1 as libc::c_int) << 15 as libc::c_int + 2 as libc::c_int) / tmp1_int;
    tmp2_int = silk_max_int(
        *pNLSF_Q15.offset(1 as libc::c_int as isize) as libc::c_int
            - *pNLSF_Q15.offset(0 as libc::c_int as isize) as libc::c_int,
        1 as libc::c_int,
    );
    tmp2_int = ((1 as libc::c_int) << 15 as libc::c_int + 2 as libc::c_int) / tmp2_int;
    *pNLSFW_Q_OUT.offset(0 as libc::c_int as isize) =
        silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as i16;
    k = 1 as libc::c_int;
    while k < D - 1 as libc::c_int {
        tmp1_int = silk_max_int(
            *pNLSF_Q15.offset((k + 1 as libc::c_int) as isize) as libc::c_int
                - *pNLSF_Q15.offset(k as isize) as libc::c_int,
            1 as libc::c_int,
        );
        tmp1_int = ((1 as libc::c_int) << 15 as libc::c_int + 2 as libc::c_int) / tmp1_int;
        *pNLSFW_Q_OUT.offset(k as isize) = silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as i16;
        tmp2_int = silk_max_int(
            *pNLSF_Q15.offset((k + 2 as libc::c_int) as isize) as libc::c_int
                - *pNLSF_Q15.offset((k + 1 as libc::c_int) as isize) as libc::c_int,
            1 as libc::c_int,
        );
        tmp2_int = ((1 as libc::c_int) << 15 as libc::c_int + 2 as libc::c_int) / tmp2_int;
        *pNLSFW_Q_OUT.offset((k + 1 as libc::c_int) as isize) =
            silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as i16;
        k += 2 as libc::c_int;
    }
    tmp1_int = silk_max_int(
        ((1 as libc::c_int) << 15 as libc::c_int)
            - *pNLSF_Q15.offset((D - 1 as libc::c_int) as isize) as libc::c_int,
        1 as libc::c_int,
    );
    tmp1_int = ((1 as libc::c_int) << 15 as libc::c_int + 2 as libc::c_int) / tmp1_int;
    *pNLSFW_Q_OUT.offset((D - 1 as libc::c_int) as isize) =
        silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as i16;
}
