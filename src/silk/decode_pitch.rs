use ::libc;
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
    #[c2rust::src_loc = "39:9"]
    pub const PE_MAX_NB_SUBFR: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "59:9"]
    pub const PE_NB_CBKS_STAGE2_EXT: libc::c_int = 11 as libc::c_int;
    #[c2rust::src_loc = "61:9"]
    pub const PE_NB_CBKS_STAGE3_MAX: libc::c_int = 34 as libc::c_int;
    #[c2rust::src_loc = "65:9"]
    pub const PE_NB_CBKS_STAGE3_10MS: libc::c_int = 12 as libc::c_int;
    #[c2rust::src_loc = "66:9"]
    pub const PE_NB_CBKS_STAGE2_10MS: libc::c_int = 3 as libc::c_int;
    extern "C" {
        #[c2rust::src_loc = "77:24"]
        pub static silk_CB_lags_stage2: [[i8; 11]; 4];
        #[c2rust::src_loc = "78:24"]
        pub static silk_CB_lags_stage3: [[i8; 34]; 4];
        #[c2rust::src_loc = "83:24"]
        pub static silk_CB_lags_stage2_10_ms: [[i8; 3]; 2];
        #[c2rust::src_loc = "84:24"]
        pub static silk_CB_lags_stage3_10_ms: [[i8; 12]; 2];
    }
}
use self::arch_h::celt_fatal;
pub use self::pitch_est_defines_h::{
    silk_CB_lags_stage2, silk_CB_lags_stage2_10_ms, silk_CB_lags_stage3, silk_CB_lags_stage3_10_ms,
    PE_MAX_NB_SUBFR, PE_NB_CBKS_STAGE2_10MS, PE_NB_CBKS_STAGE2_EXT, PE_NB_CBKS_STAGE3_10MS,
    PE_NB_CBKS_STAGE3_MAX,
};

#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn silk_decode_pitch(
    lagIndex: i16,
    contourIndex: i8,
    pitch_lags: *mut libc::c_int,
    Fs_kHz: libc::c_int,
    nb_subfr: libc::c_int,
) {
    let mut lag: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut min_lag: libc::c_int = 0;
    let mut max_lag: libc::c_int = 0;
    let mut cbk_size: libc::c_int = 0;
    let mut Lag_CB_ptr: *const i8 = 0 as *const i8;
    if Fs_kHz == 8 as libc::c_int {
        if nb_subfr == PE_MAX_NB_SUBFR {
            Lag_CB_ptr = &*(*silk_CB_lags_stage2
                .as_ptr()
                .offset(0 as libc::c_int as isize))
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const i8;
            cbk_size = PE_NB_CBKS_STAGE2_EXT;
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
            .offset(0 as libc::c_int as isize) as *const i8;
            cbk_size = PE_NB_CBKS_STAGE2_10MS;
        }
    } else if nb_subfr == PE_MAX_NB_SUBFR {
        Lag_CB_ptr = &*(*silk_CB_lags_stage3
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8;
        cbk_size = PE_NB_CBKS_STAGE3_MAX;
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
        .offset(0 as libc::c_int as isize) as *const i8;
        cbk_size = PE_NB_CBKS_STAGE3_10MS;
    }
    min_lag = 2 as libc::c_int as i16 as i32 * Fs_kHz as i16 as i32;
    max_lag = 18 as libc::c_int as i16 as i32 * Fs_kHz as i16 as i32;
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
