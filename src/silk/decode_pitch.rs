use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:35"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:35"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:35"]
pub mod opus_types_h {
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int16_t, int32_t, int8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:35"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/pitch_est_defines.h:36"]
pub mod pitch_est_defines_h {
    use super::opus_types_h::opus_int8;
    extern "C" {
        #[c2rust::src_loc = "77:24"]
        pub static silk_CB_lags_stage2: [[opus_int8; 11]; 4];
        #[c2rust::src_loc = "78:24"]
        pub static silk_CB_lags_stage3: [[opus_int8; 34]; 4];
        #[c2rust::src_loc = "83:24"]
        pub static silk_CB_lags_stage2_10_ms: [[opus_int8; 3]; 2];
        #[c2rust::src_loc = "84:24"]
        pub static silk_CB_lags_stage3_10_ms: [[opus_int8; 12]; 2];
    }
}
use self::arch_h::celt_fatal;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int8};
use self::pitch_est_defines_h::{
    silk_CB_lags_stage2, silk_CB_lags_stage2_10_ms, silk_CB_lags_stage3, silk_CB_lags_stage3_10_ms,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::types_h::{__int16_t, __int32_t, __int8_t};
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn silk_decode_pitch(
    mut lagIndex: opus_int16,
    mut contourIndex: opus_int8,
    mut pitch_lags: *mut libc::c_int,
    Fs_kHz: libc::c_int,
    nb_subfr: libc::c_int,
) {
    let mut lag: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut min_lag: libc::c_int = 0;
    let mut max_lag: libc::c_int = 0;
    let mut cbk_size: libc::c_int = 0;
    let mut Lag_CB_ptr: *const opus_int8 = 0 as *const opus_int8;
    if Fs_kHz == 8 as libc::c_int {
        if nb_subfr == 4 as libc::c_int {
            Lag_CB_ptr = &*(*silk_CB_lags_stage2
                .as_ptr()
                .offset(0 as libc::c_int as isize))
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const opus_int8;
            cbk_size = 11 as libc::c_int;
        } else {
            if !(nb_subfr == 4 as libc::c_int >> 1 as libc::c_int) {
                celt_fatal(
                    b"assertion failed: nb_subfr == PE_MAX_NB_SUBFR >> 1\0" as *const u8
                        as *const libc::c_char,
                    b"silk/decode_pitch.c\0" as *const u8 as *const libc::c_char,
                    54 as libc::c_int,
                );
            }
            Lag_CB_ptr = &*(*silk_CB_lags_stage2_10_ms
                .as_ptr()
                .offset(0 as libc::c_int as isize))
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const opus_int8;
            cbk_size = 3 as libc::c_int;
        }
    } else if nb_subfr == 4 as libc::c_int {
        Lag_CB_ptr = &*(*silk_CB_lags_stage3
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const opus_int8;
        cbk_size = 34 as libc::c_int;
    } else {
        if !(nb_subfr == 4 as libc::c_int >> 1 as libc::c_int) {
            celt_fatal(
                b"assertion failed: nb_subfr == PE_MAX_NB_SUBFR >> 1\0" as *const u8
                    as *const libc::c_char,
                b"silk/decode_pitch.c\0" as *const u8 as *const libc::c_char,
                63 as libc::c_int,
            );
        }
        Lag_CB_ptr = &*(*silk_CB_lags_stage3_10_ms
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const opus_int8;
        cbk_size = 12 as libc::c_int;
    }
    min_lag = 2 as libc::c_int as opus_int16 as opus_int32 * Fs_kHz as opus_int16 as opus_int32;
    max_lag = 18 as libc::c_int as opus_int16 as opus_int32 * Fs_kHz as opus_int16 as opus_int32;
    lag = min_lag + lagIndex as libc::c_int;
    k = 0 as libc::c_int;
    while k < nb_subfr {
        *pitch_lags.offset(k as isize) = lag
            + *Lag_CB_ptr.offset((k * cbk_size + contourIndex as libc::c_int) as isize)
                as libc::c_int;
        *pitch_lags.offset(k as isize) = if min_lag > max_lag {
            if *pitch_lags.offset(k as isize) > min_lag {
                min_lag
            } else if *pitch_lags.offset(k as isize) < max_lag {
                max_lag
            } else {
                *pitch_lags.offset(k as isize)
            }
        } else if *pitch_lags.offset(k as isize) > max_lag {
            max_lag
        } else if *pitch_lags.offset(k as isize) < min_lag {
            min_lag
        } else {
            *pitch_lags.offset(k as isize)
        };
        k += 1;
    }
}
