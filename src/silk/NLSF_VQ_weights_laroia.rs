use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:33"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "546:1"]
    pub unsafe extern "C" fn silk_min_int(
        mut a: libc::c_int,
        mut b: libc::c_int,
    ) -> libc::c_int {
        return if a < b { a } else { b };
    }
    #[inline]
    #[c2rust::src_loc = "564:1"]
    pub unsafe extern "C" fn silk_max_int(
        mut a: libc::c_int,
        mut b: libc::c_int,
    ) -> libc::c_int {
        return if a > b { a } else { b };
    }
}
pub use self::types_h::{__int16_t, __int32_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::opus_types_h::{opus_int16, opus_int32};
pub use self::typedef_h::silk_int16_MAX;
use self::arch_h::celt_fatal;
pub use self::SigProc_FIX_h::{silk_min_int, silk_max_int};
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn silk_NLSF_VQ_weights_laroia(
    mut pNLSFW_Q_OUT: *mut opus_int16,
    mut pNLSF_Q15: *const opus_int16,
    D: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut tmp1_int: opus_int32 = 0;
    let mut tmp2_int: opus_int32 = 0;
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
    *pNLSFW_Q_OUT
        .offset(
            0 as libc::c_int as isize,
        ) = silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as opus_int16;
    k = 1 as libc::c_int;
    while k < D - 1 as libc::c_int {
        tmp1_int = silk_max_int(
            *pNLSF_Q15.offset((k + 1 as libc::c_int) as isize) as libc::c_int
                - *pNLSF_Q15.offset(k as isize) as libc::c_int,
            1 as libc::c_int,
        );
        tmp1_int = ((1 as libc::c_int) << 15 as libc::c_int + 2 as libc::c_int)
            / tmp1_int;
        *pNLSFW_Q_OUT
            .offset(
                k as isize,
            ) = silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as opus_int16;
        tmp2_int = silk_max_int(
            *pNLSF_Q15.offset((k + 2 as libc::c_int) as isize) as libc::c_int
                - *pNLSF_Q15.offset((k + 1 as libc::c_int) as isize) as libc::c_int,
            1 as libc::c_int,
        );
        tmp2_int = ((1 as libc::c_int) << 15 as libc::c_int + 2 as libc::c_int)
            / tmp2_int;
        *pNLSFW_Q_OUT
            .offset(
                (k + 1 as libc::c_int) as isize,
            ) = silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as opus_int16;
        k += 2 as libc::c_int;
    }
    tmp1_int = silk_max_int(
        ((1 as libc::c_int) << 15 as libc::c_int)
            - *pNLSF_Q15.offset((D - 1 as libc::c_int) as isize) as libc::c_int,
        1 as libc::c_int,
    );
    tmp1_int = ((1 as libc::c_int) << 15 as libc::c_int + 2 as libc::c_int) / tmp1_int;
    *pNLSFW_Q_OUT
        .offset(
            (D - 1 as libc::c_int) as isize,
        ) = silk_min_int(tmp1_int + tmp2_int, silk_int16_MAX) as opus_int16;
}
