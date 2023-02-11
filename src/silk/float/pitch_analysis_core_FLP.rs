use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:35"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/xmmintrin.h:35"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::{__m128, _mm_cvt_ss2si, _mm_cvtss_si32, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:35"]
pub mod SigProc_FLP_h {
    #[inline]
    #[c2rust::src_loc = "162:1"]
    pub unsafe extern "C" fn silk_float2short_array(
        out: *mut i16,
        in_0: *const libc::c_float,
        length: i32,
    ) {
        let mut k: i32 = 0;
        k = length - 1 as libc::c_int;
        while k >= 0 as libc::c_int {
            *out.offset(k as isize) = (if float2int(*in_0.offset(k as isize)) > silk_int16_MAX {
                silk_int16_MAX
            } else if float2int(*in_0.offset(k as isize)) < silk_int16_MIN {
                silk_int16_MIN
            } else {
                float2int(*in_0.offset(k as isize))
            }) as i16;
            k -= 1;
        }
    }
    #[inline]
    #[c2rust::src_loc = "175:1"]
    pub unsafe extern "C" fn silk_short2float_array(
        out: *mut libc::c_float,
        in_0: *const i16,
        length: i32,
    ) {
        let mut k: i32 = 0;
        k = length - 1 as libc::c_int;
        while k >= 0 as libc::c_int {
            *out.offset(k as isize) = *in_0.offset(k as isize) as libc::c_float;
            k -= 1;
        }
    }
    #[inline]
    #[c2rust::src_loc = "188:1"]
    pub unsafe extern "C" fn silk_log2(x: libc::c_double) -> libc::c_float {
        return (3.32192809488736f64 * x.log10()) as libc::c_float;
    }
    use super::float_cast_h::float2int;
    use super::typedef_h::{silk_int16_MAX, silk_int16_MIN};
    extern "C" {
        #[c2rust::src_loc = "134:1"]
        pub fn silk_energy_FLP(data: *const libc::c_float, dataSize: libc::c_int)
            -> libc::c_double;
        #[c2rust::src_loc = "94:1"]
        pub fn silk_insertion_sort_decreasing_FLP(
            a: *mut libc::c_float,
            idx: *mut libc::c_int,
            L: libc::c_int,
            K: libc::c_int,
        );
        #[c2rust::src_loc = "127:1"]
        pub fn silk_inner_product_FLP(
            data1: *const libc::c_float,
            data2: *const libc::c_float,
            dataSize: libc::c_int,
        ) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/float_cast.h:35"]
pub mod float_cast_h {
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn float2int(x: libc::c_float) -> i32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }
    use super::xmmintrin_h::{_mm_cvt_ss2si, _mm_set_ss};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:35"]
pub mod typedef_h {
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/pitch_est_defines.h:37"]
pub mod pitch_est_defines_h {
    #[c2rust::src_loc = "39:9"]
    pub const PE_MAX_NB_SUBFR: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "40:9"]
    pub const PE_SUBFR_LENGTH_MS: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "42:9"]
    pub const PE_LTP_MEM_LENGTH_MS: libc::c_int = 4 as libc::c_int * PE_SUBFR_LENGTH_MS;
    #[c2rust::src_loc = "49:9"]
    pub const PE_MAX_LAG_MS: libc::c_int = 18 as libc::c_int;
    #[c2rust::src_loc = "50:9"]
    pub const PE_MIN_LAG_MS: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const PE_NB_STAGE3_LAGS: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "58:9"]
    pub const PE_NB_CBKS_STAGE2: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "59:9"]
    pub const PE_NB_CBKS_STAGE2_EXT: libc::c_int = 11 as libc::c_int;
    #[c2rust::src_loc = "61:9"]
    pub const PE_NB_CBKS_STAGE3_MAX: libc::c_int = 34 as libc::c_int;
    #[c2rust::src_loc = "65:9"]
    pub const PE_NB_CBKS_STAGE3_10MS: libc::c_int = 12 as libc::c_int;
    #[c2rust::src_loc = "66:9"]
    pub const PE_NB_CBKS_STAGE2_10MS: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "68:9"]
    pub const PE_SHORTLAG_BIAS: libc::c_float = 0.2f32;
    #[c2rust::src_loc = "69:9"]
    pub const PE_PREVLAG_BIAS: libc::c_float = 0.2f32;
    #[c2rust::src_loc = "70:9"]
    pub const PE_FLATCONTOUR_BIAS: libc::c_float = 0.05f32;
    #[c2rust::src_loc = "72:9"]
    pub const SILK_PE_MIN_COMPLEX: libc::c_int = 0 as libc::c_int;
    extern "C" {
        #[c2rust::src_loc = "77:24"]
        pub static silk_CB_lags_stage2: [[i8; 11]; 4];
        #[c2rust::src_loc = "78:24"]
        pub static silk_CB_lags_stage3: [[i8; 34]; 4];
        #[c2rust::src_loc = "79:24"]
        pub static silk_Lag_range_stage3: [[[i8; 2]; 4]; 3];
        #[c2rust::src_loc = "80:24"]
        pub static silk_nb_cbk_searchs_stage3: [i8; 3];
        #[c2rust::src_loc = "83:24"]
        pub static silk_CB_lags_stage2_10_ms: [[i8; 3]; 2];
        #[c2rust::src_loc = "84:24"]
        pub static silk_CB_lags_stage3_10_ms: [[i8; 12]; 2];
        #[c2rust::src_loc = "85:24"]
        pub static silk_Lag_range_stage3_10_ms: [[i8; 2]; 2];
    }
}
use self::arch_h::opus_val32;
use crate::celt::celt::celt_fatal;

pub use self::float_cast_h::float2int;
pub use self::pitch_est_defines_h::{
    silk_CB_lags_stage2, silk_CB_lags_stage2_10_ms, silk_CB_lags_stage3, silk_CB_lags_stage3_10_ms,
    silk_Lag_range_stage3, silk_Lag_range_stage3_10_ms, silk_nb_cbk_searchs_stage3,
    PE_FLATCONTOUR_BIAS, PE_LTP_MEM_LENGTH_MS, PE_MAX_LAG_MS, PE_MAX_NB_SUBFR, PE_MIN_LAG_MS,
    PE_NB_CBKS_STAGE2, PE_NB_CBKS_STAGE2_10MS, PE_NB_CBKS_STAGE2_EXT, PE_NB_CBKS_STAGE3_10MS,
    PE_NB_CBKS_STAGE3_MAX, PE_NB_STAGE3_LAGS, PE_PREVLAG_BIAS, PE_SHORTLAG_BIAS,
    PE_SUBFR_LENGTH_MS, SILK_PE_MIN_COMPLEX,
};
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
pub use self::SigProc_FLP_h::{
    silk_energy_FLP, silk_float2short_array, silk_inner_product_FLP,
    silk_insertion_sort_decreasing_FLP, silk_log2, silk_short2float_array,
};
use crate::celt::pitch::celt_pitch_xcorr_c;
use crate::externs::memset;
use crate::silk::resampler_down2::silk_resampler_down2;
use crate::silk::resampler_down2_3::silk_resampler_down2_3;
use crate::silk::SigProc_FIX::{silk_max_int, silk_min_int};

#[no_mangle]
#[c2rust::src_loc = "67:1"]
pub unsafe extern "C" fn silk_pitch_analysis_core_FLP(
    frame: *const libc::c_float,
    pitch_out: *mut libc::c_int,
    lagIndex: *mut i16,
    contourIndex: *mut i8,
    LTPCorr: *mut libc::c_float,
    mut prevLag: libc::c_int,
    search_thres1: libc::c_float,
    search_thres2: libc::c_float,
    Fs_kHz: libc::c_int,
    complexity: libc::c_int,
    nb_subfr: libc::c_int,
    arch: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut frame_8kHz: [libc::c_float; 320] = [0.; 320];
    let mut frame_4kHz: [libc::c_float; 160] = [0.; 160];
    let mut frame_8_FIX: [i16; 320] = [0; 320];
    let mut frame_4_FIX: [i16; 160] = [0; 160];
    let mut filt_state: [i32; 6] = [0; 6];
    let mut threshold: libc::c_float = 0.;
    let mut contour_bias: libc::c_float = 0.;
    let mut C: [[libc::c_float; 149]; 4] = [[0.; 149]; 4];
    let mut xcorr: [opus_val32; 65] = [0.; 65];
    let mut CC: [libc::c_float; 11] = [0.; 11];
    let mut target_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut basis_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut cross_corr: libc::c_double = 0.;
    let mut normalizer: libc::c_double = 0.;
    let mut energy: libc::c_double = 0.;
    let mut energy_tmp: libc::c_double = 0.;
    let mut d_srch: [libc::c_int; 24] = [0; 24];
    let mut d_comp: [i16; 149] = [0; 149];
    let mut length_d_srch: libc::c_int = 0;
    let mut length_d_comp: libc::c_int = 0;
    let mut Cmax: libc::c_float = 0.;
    let mut CCmax: libc::c_float = 0.;
    let mut CCmax_b: libc::c_float = 0.;
    let mut CCmax_new_b: libc::c_float = 0.;
    let mut CCmax_new: libc::c_float = 0.;
    let mut CBimax: libc::c_int = 0;
    let mut CBimax_new: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut start_lag: libc::c_int = 0;
    let mut end_lag: libc::c_int = 0;
    let mut lag_new: libc::c_int = 0;
    let mut cbk_size: libc::c_int = 0;
    let mut lag_log2: libc::c_float = 0.;
    let mut prevLag_log2: libc::c_float = 0.;
    let mut delta_lag_log2_sqr: libc::c_float = 0.;
    let mut energies_st3: [[[libc::c_float; 5]; 34]; 4] = [[[0.; 5]; 34]; 4];
    let mut cross_corr_st3: [[[libc::c_float; 5]; 34]; 4] = [[[0.; 5]; 34]; 4];
    let mut lag_counter: libc::c_int = 0;
    let mut frame_length: libc::c_int = 0;
    let mut frame_length_8kHz: libc::c_int = 0;
    let mut frame_length_4kHz: libc::c_int = 0;
    let mut sf_length: libc::c_int = 0;
    let mut sf_length_8kHz: libc::c_int = 0;
    let mut sf_length_4kHz: libc::c_int = 0;
    let mut min_lag: libc::c_int = 0;
    let mut min_lag_8kHz: libc::c_int = 0;
    let mut min_lag_4kHz: libc::c_int = 0;
    let mut max_lag: libc::c_int = 0;
    let mut max_lag_8kHz: libc::c_int = 0;
    let mut max_lag_4kHz: libc::c_int = 0;
    let mut nb_cbk_search: libc::c_int = 0;
    let mut Lag_CB_ptr: *const i8 = 0 as *const i8;
    if !(Fs_kHz == 8 as libc::c_int || Fs_kHz == 12 as libc::c_int || Fs_kHz == 16 as libc::c_int) {
        celt_fatal(
            b"assertion failed: Fs_kHz == 8 || Fs_kHz == 12 || Fs_kHz == 16\0" as *const u8
                as *const libc::c_char,
            b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
        );
    }
    if !(complexity >= 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: complexity >= SILK_PE_MIN_COMPLEX\0" as *const u8
                as *const libc::c_char,
            b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
        );
    }
    if !(complexity <= 2 as libc::c_int) {
        celt_fatal(
            b"assertion failed: complexity <= SILK_PE_MAX_COMPLEX\0" as *const u8
                as *const libc::c_char,
            b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
        );
    }
    frame_length = (PE_LTP_MEM_LENGTH_MS + nb_subfr * PE_SUBFR_LENGTH_MS) * Fs_kHz;
    frame_length_4kHz = (PE_LTP_MEM_LENGTH_MS + nb_subfr * PE_SUBFR_LENGTH_MS) * 4 as libc::c_int;
    frame_length_8kHz = (PE_LTP_MEM_LENGTH_MS + nb_subfr * PE_SUBFR_LENGTH_MS) * 8 as libc::c_int;
    sf_length = PE_SUBFR_LENGTH_MS * Fs_kHz;
    sf_length_4kHz = PE_SUBFR_LENGTH_MS * 4 as libc::c_int;
    sf_length_8kHz = PE_SUBFR_LENGTH_MS * 8 as libc::c_int;
    min_lag = PE_MIN_LAG_MS * Fs_kHz;
    min_lag_4kHz = PE_MIN_LAG_MS * 4 as libc::c_int;
    min_lag_8kHz = PE_MIN_LAG_MS * 8 as libc::c_int;
    max_lag = PE_MAX_LAG_MS * Fs_kHz - 1 as libc::c_int;
    max_lag_4kHz = PE_MAX_LAG_MS * 4 as libc::c_int;
    max_lag_8kHz = PE_MAX_LAG_MS * 8 as libc::c_int - 1 as libc::c_int;
    if Fs_kHz == 16 as libc::c_int {
        let mut frame_16_FIX: [i16; 640] = [0; 640];
        silk_float2short_array(frame_16_FIX.as_mut_ptr(), frame, frame_length);
        memset(
            filt_state.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
        silk_resampler_down2(
            filt_state.as_mut_ptr(),
            frame_8_FIX.as_mut_ptr(),
            frame_16_FIX.as_mut_ptr(),
            frame_length,
        );
        silk_short2float_array(
            frame_8kHz.as_mut_ptr(),
            frame_8_FIX.as_mut_ptr(),
            frame_length_8kHz,
        );
    } else if Fs_kHz == 12 as libc::c_int {
        let mut frame_12_FIX: [i16; 480] = [0; 480];
        silk_float2short_array(frame_12_FIX.as_mut_ptr(), frame, frame_length);
        memset(
            filt_state.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (6 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
        silk_resampler_down2_3(
            filt_state.as_mut_ptr(),
            frame_8_FIX.as_mut_ptr(),
            frame_12_FIX.as_mut_ptr(),
            frame_length,
        );
        silk_short2float_array(
            frame_8kHz.as_mut_ptr(),
            frame_8_FIX.as_mut_ptr(),
            frame_length_8kHz,
        );
    } else {
        if !(Fs_kHz == 8 as libc::c_int) {
            celt_fatal(
                b"assertion failed: Fs_kHz == 8\0" as *const u8 as *const libc::c_char,
                b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
                151 as libc::c_int,
            );
        }
        silk_float2short_array(frame_8_FIX.as_mut_ptr(), frame, frame_length_8kHz);
    }
    memset(
        filt_state.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
    silk_resampler_down2(
        filt_state.as_mut_ptr(),
        frame_4_FIX.as_mut_ptr(),
        frame_8_FIX.as_mut_ptr(),
        frame_length_8kHz,
    );
    silk_short2float_array(
        frame_4kHz.as_mut_ptr(),
        frame_4_FIX.as_mut_ptr(),
        frame_length_4kHz,
    );
    i = frame_length_4kHz - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        frame_4kHz[i as usize] = (if frame_4kHz[i as usize] as i32 as libc::c_float
            + frame_4kHz[(i - 1 as libc::c_int) as usize]
            > silk_int16_MAX as libc::c_float
        {
            silk_int16_MAX as libc::c_float
        } else if frame_4kHz[i as usize] as i32 as libc::c_float
            + frame_4kHz[(i - 1 as libc::c_int) as usize]
            < silk_int16_MIN as libc::c_float
        {
            silk_int16_MIN as libc::c_float
        } else {
            frame_4kHz[i as usize] as i32 as libc::c_float
                + frame_4kHz[(i - 1 as libc::c_int) as usize]
        }) as i16 as libc::c_float;
        i -= 1;
    }
    memset(
        C.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(nb_subfr as libc::c_ulong)
            .wrapping_mul(
                ((18 as libc::c_int * 16 as libc::c_int >> 1 as libc::c_int) + 5 as libc::c_int)
                    as libc::c_ulong,
            ),
    );
    target_ptr = &mut *frame_4kHz
        .as_mut_ptr()
        .offset(((sf_length_4kHz as u32) << 2 as libc::c_int) as i32 as isize)
        as *mut libc::c_float;
    k = 0 as libc::c_int;
    while k < nb_subfr >> 1 as libc::c_int {
        if !(target_ptr >= frame_4kHz.as_mut_ptr() as *const libc::c_float) {
            celt_fatal(
                b"assertion failed: target_ptr >= frame_4kHz\0" as *const u8 as *const libc::c_char,
                b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
                172 as libc::c_int,
            );
        }
        if !(target_ptr.offset(sf_length_8kHz as isize)
            <= frame_4kHz.as_mut_ptr().offset(frame_length_4kHz as isize) as *const libc::c_float)
        {
            celt_fatal(
                b"assertion failed: target_ptr + sf_length_8kHz <= frame_4kHz + frame_length_4kHz\0"
                    as *const u8 as *const libc::c_char,
                b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
                173 as libc::c_int,
            );
        }
        basis_ptr = target_ptr.offset(-(min_lag_4kHz as isize));
        if !(basis_ptr >= frame_4kHz.as_mut_ptr() as *const libc::c_float) {
            celt_fatal(
                b"assertion failed: basis_ptr >= frame_4kHz\0" as *const u8 as *const libc::c_char,
                b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
                178 as libc::c_int,
            );
        }
        if !(basis_ptr.offset(sf_length_8kHz as isize)
            <= frame_4kHz.as_mut_ptr().offset(frame_length_4kHz as isize) as *const libc::c_float)
        {
            celt_fatal(
                b"assertion failed: basis_ptr + sf_length_8kHz <= frame_4kHz + frame_length_4kHz\0"
                    as *const u8 as *const libc::c_char,
                b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
                179 as libc::c_int,
            );
        }
        celt_pitch_xcorr_c(
            target_ptr,
            target_ptr.offset(-(max_lag_4kHz as isize)),
            xcorr.as_mut_ptr(),
            sf_length_8kHz,
            max_lag_4kHz - min_lag_4kHz + 1 as libc::c_int,
            arch,
        );
        cross_corr = xcorr[(max_lag_4kHz - min_lag_4kHz) as usize] as libc::c_double;
        normalizer = silk_energy_FLP(target_ptr, sf_length_8kHz)
            + silk_energy_FLP(basis_ptr, sf_length_8kHz)
            + (sf_length_8kHz as libc::c_float * 4000.0f32) as libc::c_double;
        C[0 as libc::c_int as usize][min_lag_4kHz as usize] +=
            (2 as libc::c_int as libc::c_double * cross_corr / normalizer) as libc::c_float;
        d = min_lag_4kHz + 1 as libc::c_int;
        while d <= max_lag_4kHz {
            basis_ptr = basis_ptr.offset(-1);
            cross_corr = xcorr[(max_lag_4kHz - d) as usize] as libc::c_double;
            normalizer += *basis_ptr.offset(0 as libc::c_int as isize) as libc::c_double
                * *basis_ptr.offset(0 as libc::c_int as isize) as libc::c_double
                - *basis_ptr.offset(sf_length_8kHz as isize) as libc::c_double
                    * *basis_ptr.offset(sf_length_8kHz as isize) as libc::c_double;
            C[0 as libc::c_int as usize][d as usize] +=
                (2 as libc::c_int as libc::c_double * cross_corr / normalizer) as libc::c_float;
            d += 1;
        }
        target_ptr = target_ptr.offset(sf_length_8kHz as isize);
        k += 1;
    }
    i = max_lag_4kHz;
    while i >= min_lag_4kHz {
        C[0 as libc::c_int as usize][i as usize] -=
            C[0 as libc::c_int as usize][i as usize] * i as libc::c_float / 4096.0f32;
        i -= 1;
    }
    length_d_srch = 4 as libc::c_int + 2 as libc::c_int * complexity;
    if !(3 as libc::c_int * length_d_srch <= 24 as libc::c_int) {
        celt_fatal(
            b"assertion failed: 3 * length_d_srch <= PE_D_SRCH_LENGTH\0" as *const u8
                as *const libc::c_char,
            b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int,
        );
    }
    silk_insertion_sort_decreasing_FLP(
        &mut *(*C.as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(min_lag_4kHz as isize),
        d_srch.as_mut_ptr(),
        max_lag_4kHz - min_lag_4kHz + 1 as libc::c_int,
        length_d_srch,
    );
    Cmax = C[0 as libc::c_int as usize][min_lag_4kHz as usize];
    if Cmax < 0.2f32 {
        memset(
            pitch_out as *mut libc::c_void,
            0 as libc::c_int,
            (nb_subfr as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        *LTPCorr = 0.0f32;
        *lagIndex = 0 as libc::c_int as i16;
        *contourIndex = 0 as libc::c_int as i8;
        return 1 as libc::c_int;
    }
    threshold = search_thres1 * Cmax;
    i = 0 as libc::c_int;
    while i < length_d_srch {
        if C[0 as libc::c_int as usize][(min_lag_4kHz + i) as usize] > threshold {
            d_srch[i as usize] =
                (((d_srch[i as usize] + min_lag_4kHz) as u32) << 1 as libc::c_int) as i32;
            i += 1;
        } else {
            length_d_srch = i;
            break;
        }
    }
    if !(length_d_srch > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: length_d_srch > 0\0" as *const u8 as *const libc::c_char,
            b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int,
        );
    }
    i = min_lag_8kHz - 5 as libc::c_int;
    while i < max_lag_8kHz + 5 as libc::c_int {
        d_comp[i as usize] = 0 as libc::c_int as i16;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < length_d_srch {
        d_comp[d_srch[i as usize] as usize] = 1 as libc::c_int as i16;
        i += 1;
    }
    i = max_lag_8kHz + 3 as libc::c_int;
    while i >= min_lag_8kHz {
        d_comp[i as usize] = (d_comp[i as usize] as libc::c_int
            + (d_comp[(i - 1 as libc::c_int) as usize] as libc::c_int
                + d_comp[(i - 2 as libc::c_int) as usize] as libc::c_int))
            as i16;
        i -= 1;
    }
    length_d_srch = 0 as libc::c_int;
    i = min_lag_8kHz;
    while i < max_lag_8kHz + 1 as libc::c_int {
        if d_comp[(i + 1 as libc::c_int) as usize] as libc::c_int > 0 as libc::c_int {
            d_srch[length_d_srch as usize] = i;
            length_d_srch += 1;
        }
        i += 1;
    }
    i = max_lag_8kHz + 3 as libc::c_int;
    while i >= min_lag_8kHz {
        d_comp[i as usize] = (d_comp[i as usize] as libc::c_int
            + (d_comp[(i - 1 as libc::c_int) as usize] as libc::c_int
                + d_comp[(i - 2 as libc::c_int) as usize] as libc::c_int
                + d_comp[(i - 3 as libc::c_int) as usize] as libc::c_int))
            as i16;
        i -= 1;
    }
    length_d_comp = 0 as libc::c_int;
    i = min_lag_8kHz;
    while i < max_lag_8kHz + 4 as libc::c_int {
        if d_comp[i as usize] as libc::c_int > 0 as libc::c_int {
            d_comp[length_d_comp as usize] = (i - 2 as libc::c_int) as i16;
            length_d_comp += 1;
        }
        i += 1;
    }
    memset(
        C.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((4 as libc::c_int
            * ((18 as libc::c_int * 16 as libc::c_int >> 1 as libc::c_int) + 5 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    if Fs_kHz == 8 as libc::c_int {
        target_ptr = &*frame.offset((PE_LTP_MEM_LENGTH_MS * 8 as libc::c_int) as isize)
            as *const libc::c_float;
    } else {
        target_ptr = &mut *frame_8kHz
            .as_mut_ptr()
            .offset((PE_LTP_MEM_LENGTH_MS * 8 as libc::c_int) as isize)
            as *mut libc::c_float;
    }
    k = 0 as libc::c_int;
    while k < nb_subfr {
        energy_tmp = silk_energy_FLP(target_ptr, sf_length_8kHz) + 1.0f64;
        j = 0 as libc::c_int;
        while j < length_d_comp {
            d = d_comp[j as usize] as libc::c_int;
            basis_ptr = target_ptr.offset(-(d as isize));
            cross_corr = silk_inner_product_FLP(basis_ptr, target_ptr, sf_length_8kHz);
            if cross_corr > 0.0f32 as libc::c_double {
                energy = silk_energy_FLP(basis_ptr, sf_length_8kHz);
                C[k as usize][d as usize] = (2 as libc::c_int as libc::c_double * cross_corr
                    / (energy + energy_tmp))
                    as libc::c_float;
            } else {
                C[k as usize][d as usize] = 0.0f32;
            }
            j += 1;
        }
        target_ptr = target_ptr.offset(sf_length_8kHz as isize);
        k += 1;
    }
    CCmax = 0.0f32;
    CCmax_b = -1000.0f32;
    CBimax = 0 as libc::c_int;
    lag = -(1 as libc::c_int);
    if prevLag > 0 as libc::c_int {
        if Fs_kHz == 12 as libc::c_int {
            prevLag = ((prevLag as u32) << 1 as libc::c_int) as i32 / 3 as libc::c_int;
        } else if Fs_kHz == 16 as libc::c_int {
            prevLag = prevLag >> 1 as libc::c_int;
        }
        prevLag_log2 = silk_log2(prevLag as libc::c_float as libc::c_double);
    } else {
        prevLag_log2 = 0 as libc::c_int as libc::c_float;
    }
    if nb_subfr == PE_MAX_NB_SUBFR {
        cbk_size = PE_NB_CBKS_STAGE2_EXT;
        Lag_CB_ptr = &*(*silk_CB_lags_stage2
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8;
        if Fs_kHz == 8 as libc::c_int && complexity > SILK_PE_MIN_COMPLEX {
            nb_cbk_search = PE_NB_CBKS_STAGE2_EXT;
        } else {
            nb_cbk_search = PE_NB_CBKS_STAGE2;
        }
    } else {
        cbk_size = PE_NB_CBKS_STAGE2_10MS;
        Lag_CB_ptr = &*(*silk_CB_lags_stage2_10_ms
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8;
        nb_cbk_search = PE_NB_CBKS_STAGE2_10MS;
    }
    k = 0 as libc::c_int;
    while k < length_d_srch {
        d = d_srch[k as usize];
        j = 0 as libc::c_int;
        while j < nb_cbk_search {
            CC[j as usize] = 0.0f32;
            i = 0 as libc::c_int;
            while i < nb_subfr {
                CC[j as usize] += C[i as usize]
                    [(d + *Lag_CB_ptr.offset((i * cbk_size + j) as isize) as libc::c_int) as usize];
                i += 1;
            }
            j += 1;
        }
        CCmax_new = -1000.0f32;
        CBimax_new = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nb_cbk_search {
            if CC[i as usize] > CCmax_new {
                CCmax_new = CC[i as usize];
                CBimax_new = i;
            }
            i += 1;
        }
        lag_log2 = silk_log2(d as libc::c_float as libc::c_double);
        CCmax_new_b = CCmax_new - PE_SHORTLAG_BIAS * nb_subfr as libc::c_float * lag_log2;
        if prevLag > 0 as libc::c_int {
            delta_lag_log2_sqr = lag_log2 - prevLag_log2;
            delta_lag_log2_sqr *= delta_lag_log2_sqr;
            CCmax_new_b -=
                PE_PREVLAG_BIAS * nb_subfr as libc::c_float * *LTPCorr * delta_lag_log2_sqr
                    / (delta_lag_log2_sqr + 0.5f32);
        }
        if CCmax_new_b > CCmax_b && CCmax_new > nb_subfr as libc::c_float * search_thres2 {
            CCmax_b = CCmax_new_b;
            CCmax = CCmax_new;
            lag = d;
            CBimax = CBimax_new;
        }
        k += 1;
    }
    if lag == -(1 as libc::c_int) {
        memset(
            pitch_out as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        *LTPCorr = 0.0f32;
        *lagIndex = 0 as libc::c_int as i16;
        *contourIndex = 0 as libc::c_int as i8;
        return 1 as libc::c_int;
    }
    *LTPCorr = CCmax / nb_subfr as libc::c_float;
    if Fs_kHz > 8 as libc::c_int {
        if Fs_kHz == 12 as libc::c_int {
            lag = if 1 as libc::c_int == 1 as libc::c_int {
                (lag as i16 as i32 * 3 as libc::c_int as i16 as i32 >> 1 as libc::c_int)
                    + (lag as i16 as i32 * 3 as libc::c_int as i16 as i32 & 1 as libc::c_int)
            } else {
                (lag as i16 as i32 * 3 as libc::c_int as i16 as i32
                    >> 1 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            };
        } else {
            lag = ((lag as u32) << 1 as libc::c_int) as i32;
        }
        lag = if min_lag > max_lag {
            if lag > min_lag {
                min_lag
            } else if lag < max_lag {
                max_lag
            } else {
                lag
            }
        } else if lag > max_lag {
            max_lag
        } else if lag < min_lag {
            min_lag
        } else {
            lag
        };
        start_lag = silk_max_int(lag - 2 as libc::c_int, min_lag);
        end_lag = silk_min_int(lag + 2 as libc::c_int, max_lag);
        lag_new = lag;
        CBimax = 0 as libc::c_int;
        CCmax = -1000.0f32;
        silk_P_Ana_calc_corr_st3(
            cross_corr_st3.as_mut_ptr(),
            frame,
            start_lag,
            sf_length,
            nb_subfr,
            complexity,
            arch,
        );
        silk_P_Ana_calc_energy_st3(
            energies_st3.as_mut_ptr(),
            frame,
            start_lag,
            sf_length,
            nb_subfr,
            complexity,
        );
        lag_counter = 0 as libc::c_int;
        contour_bias = PE_FLATCONTOUR_BIAS / lag as libc::c_float;
        if nb_subfr == PE_MAX_NB_SUBFR {
            nb_cbk_search = silk_nb_cbk_searchs_stage3[complexity as usize] as libc::c_int;
            cbk_size = PE_NB_CBKS_STAGE3_MAX;
            Lag_CB_ptr = &*(*silk_CB_lags_stage3
                .as_ptr()
                .offset(0 as libc::c_int as isize))
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const i8;
        } else {
            nb_cbk_search = PE_NB_CBKS_STAGE3_10MS;
            cbk_size = PE_NB_CBKS_STAGE3_10MS;
            Lag_CB_ptr = &*(*silk_CB_lags_stage3_10_ms
                .as_ptr()
                .offset(0 as libc::c_int as isize))
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const i8;
        }
        target_ptr =
            &*frame.offset((PE_LTP_MEM_LENGTH_MS * Fs_kHz) as isize) as *const libc::c_float;
        energy_tmp = silk_energy_FLP(target_ptr, nb_subfr * sf_length) + 1.0f64;
        d = start_lag;
        while d <= end_lag {
            j = 0 as libc::c_int;
            while j < nb_cbk_search {
                cross_corr = 0.0f64;
                energy = energy_tmp;
                k = 0 as libc::c_int;
                while k < nb_subfr {
                    cross_corr += cross_corr_st3[k as usize][j as usize][lag_counter as usize]
                        as libc::c_double;
                    energy += energies_st3[k as usize][j as usize][lag_counter as usize]
                        as libc::c_double;
                    k += 1;
                }
                if cross_corr > 0.0f64 {
                    CCmax_new =
                        (2 as libc::c_int as libc::c_double * cross_corr / energy) as libc::c_float;
                    CCmax_new *= 1.0f32 - contour_bias * j as libc::c_float;
                } else {
                    CCmax_new = 0.0f32;
                }
                if CCmax_new > CCmax
                    && d + silk_CB_lags_stage3[0 as libc::c_int as usize][j as usize] as libc::c_int
                        <= max_lag
                {
                    CCmax = CCmax_new;
                    lag_new = d;
                    CBimax = j;
                }
                j += 1;
            }
            lag_counter += 1;
            d += 1;
        }
        k = 0 as libc::c_int;
        while k < nb_subfr {
            *pitch_out.offset(k as isize) =
                lag_new + *Lag_CB_ptr.offset((k * cbk_size + CBimax) as isize) as libc::c_int;
            *pitch_out.offset(k as isize) = if min_lag > 18 as libc::c_int * Fs_kHz {
                if *pitch_out.offset(k as isize) > min_lag {
                    min_lag
                } else if *pitch_out.offset(k as isize) < 18 as libc::c_int * Fs_kHz {
                    18 as libc::c_int * Fs_kHz
                } else {
                    *pitch_out.offset(k as isize)
                }
            } else if *pitch_out.offset(k as isize) > 18 as libc::c_int * Fs_kHz {
                18 as libc::c_int * Fs_kHz
            } else if *pitch_out.offset(k as isize) < min_lag {
                min_lag
            } else {
                *pitch_out.offset(k as isize)
            };
            k += 1;
        }
        *lagIndex = (lag_new - min_lag) as i16;
        *contourIndex = CBimax as i8;
    } else {
        k = 0 as libc::c_int;
        while k < nb_subfr {
            *pitch_out.offset(k as isize) =
                lag + *Lag_CB_ptr.offset((k * cbk_size + CBimax) as isize) as libc::c_int;
            *pitch_out.offset(k as isize) = if min_lag_8kHz > 18 as libc::c_int * 8 as libc::c_int {
                if *pitch_out.offset(k as isize) > min_lag_8kHz {
                    min_lag_8kHz
                } else if *pitch_out.offset(k as isize) < 18 as libc::c_int * 8 as libc::c_int {
                    18 as libc::c_int * 8 as libc::c_int
                } else {
                    *pitch_out.offset(k as isize)
                }
            } else if *pitch_out.offset(k as isize) > 18 as libc::c_int * 8 as libc::c_int {
                18 as libc::c_int * 8 as libc::c_int
            } else if *pitch_out.offset(k as isize) < min_lag_8kHz {
                min_lag_8kHz
            } else {
                *pitch_out.offset(k as isize)
            };
            k += 1;
        }
        *lagIndex = (lag - min_lag_8kHz) as i16;
        *contourIndex = CBimax as i8;
    }
    if !(*lagIndex as libc::c_int >= 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: *lagIndex >= 0\0" as *const u8 as *const libc::c_char,
            b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
            474 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "492:1"]
unsafe extern "C" fn silk_P_Ana_calc_corr_st3(
    cross_corr_st3: *mut [[libc::c_float; 5]; 34],
    frame: *const libc::c_float,
    start_lag: libc::c_int,
    sf_length: libc::c_int,
    nb_subfr: libc::c_int,
    complexity: libc::c_int,
    arch: libc::c_int,
) {
    let mut target_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lag_counter: libc::c_int = 0;
    let mut lag_low: libc::c_int = 0;
    let mut lag_high: libc::c_int = 0;
    let mut nb_cbk_search: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut cbk_size: libc::c_int = 0;
    let mut scratch_mem: [libc::c_float; 22] = [0.; 22];
    let mut xcorr: [opus_val32; 22] = [0.; 22];
    let mut Lag_range_ptr: *const i8 = 0 as *const i8;
    let mut Lag_CB_ptr: *const i8 = 0 as *const i8;
    if !(complexity >= 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: complexity >= SILK_PE_MIN_COMPLEX\0" as *const u8
                as *const libc::c_char,
            b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
            509 as libc::c_int,
        );
    }
    if !(complexity <= 2 as libc::c_int) {
        celt_fatal(
            b"assertion failed: complexity <= SILK_PE_MAX_COMPLEX\0" as *const u8
                as *const libc::c_char,
            b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
            510 as libc::c_int,
        );
    }
    if nb_subfr == PE_MAX_NB_SUBFR {
        Lag_range_ptr = &*(*(*silk_Lag_range_stage3.as_ptr().offset(complexity as isize))
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8;
        Lag_CB_ptr = &*(*silk_CB_lags_stage3
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8;
        nb_cbk_search = silk_nb_cbk_searchs_stage3[complexity as usize] as libc::c_int;
        cbk_size = PE_NB_CBKS_STAGE3_MAX;
    } else {
        if !(nb_subfr == 4 as libc::c_int >> 1 as libc::c_int) {
            celt_fatal(
                b"assertion failed: nb_subfr == PE_MAX_NB_SUBFR >> 1\0" as *const u8
                    as *const libc::c_char,
                b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
                518 as libc::c_int,
            );
        }
        Lag_range_ptr = &*(*silk_Lag_range_stage3_10_ms
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8;
        Lag_CB_ptr = &*(*silk_CB_lags_stage3_10_ms
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8;
        nb_cbk_search = PE_NB_CBKS_STAGE3_10MS;
        cbk_size = PE_NB_CBKS_STAGE3_10MS;
    }
    target_ptr = &*frame.offset(((sf_length as u32) << 2 as libc::c_int) as i32 as isize)
        as *const libc::c_float;
    k = 0 as libc::c_int;
    while k < nb_subfr {
        lag_counter = 0 as libc::c_int;
        lag_low = *Lag_range_ptr.offset((k * 2 as libc::c_int + 0 as libc::c_int) as isize)
            as libc::c_int;
        lag_high = *Lag_range_ptr.offset((k * 2 as libc::c_int + 1 as libc::c_int) as isize)
            as libc::c_int;
        celt_pitch_xcorr_c(
            target_ptr,
            target_ptr
                .offset(-(start_lag as isize))
                .offset(-(lag_high as isize)),
            xcorr.as_mut_ptr(),
            sf_length,
            lag_high - lag_low + 1 as libc::c_int,
            arch,
        );
        j = lag_low;
        while j <= lag_high {
            scratch_mem[lag_counter as usize] = xcorr[(lag_high - j) as usize];
            lag_counter += 1;
            j += 1;
        }
        delta = *Lag_range_ptr.offset((k * 2 as libc::c_int + 0 as libc::c_int) as isize)
            as libc::c_int;
        i = 0 as libc::c_int;
        while i < nb_cbk_search {
            idx = *Lag_CB_ptr.offset((k * cbk_size + i) as isize) as libc::c_int - delta;
            j = 0 as libc::c_int;
            while j < PE_NB_STAGE3_LAGS {
                (*cross_corr_st3.offset(k as isize))[i as usize][j as usize] =
                    scratch_mem[(idx + j) as usize];
                j += 1;
            }
            i += 1;
        }
        target_ptr = target_ptr.offset(sf_length as isize);
        k += 1;
    }
}
#[c2rust::src_loc = "559:1"]
unsafe extern "C" fn silk_P_Ana_calc_energy_st3(
    energies_st3: *mut [[libc::c_float; 5]; 34],
    frame: *const libc::c_float,
    start_lag: libc::c_int,
    sf_length: libc::c_int,
    nb_subfr: libc::c_int,
    complexity: libc::c_int,
) {
    let mut target_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut basis_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut energy: libc::c_double = 0.;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lag_counter: libc::c_int = 0;
    let mut nb_cbk_search: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut cbk_size: libc::c_int = 0;
    let mut lag_diff: libc::c_int = 0;
    let mut scratch_mem: [libc::c_float; 22] = [0.; 22];
    let mut Lag_range_ptr: *const i8 = 0 as *const i8;
    let mut Lag_CB_ptr: *const i8 = 0 as *const i8;
    if !(complexity >= 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: complexity >= SILK_PE_MIN_COMPLEX\0" as *const u8
                as *const libc::c_char,
            b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
            575 as libc::c_int,
        );
    }
    if !(complexity <= 2 as libc::c_int) {
        celt_fatal(
            b"assertion failed: complexity <= SILK_PE_MAX_COMPLEX\0" as *const u8
                as *const libc::c_char,
            b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
            576 as libc::c_int,
        );
    }
    if nb_subfr == PE_MAX_NB_SUBFR {
        Lag_range_ptr = &*(*(*silk_Lag_range_stage3.as_ptr().offset(complexity as isize))
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8;
        Lag_CB_ptr = &*(*silk_CB_lags_stage3
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8;
        nb_cbk_search = silk_nb_cbk_searchs_stage3[complexity as usize] as libc::c_int;
        cbk_size = PE_NB_CBKS_STAGE3_MAX;
    } else {
        if !(nb_subfr == 4 as libc::c_int >> 1 as libc::c_int) {
            celt_fatal(
                b"assertion failed: nb_subfr == PE_MAX_NB_SUBFR >> 1\0" as *const u8
                    as *const libc::c_char,
                b"silk/float/pitch_analysis_core_FLP.c\0" as *const u8 as *const libc::c_char,
                584 as libc::c_int,
            );
        }
        Lag_range_ptr = &*(*silk_Lag_range_stage3_10_ms
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8;
        Lag_CB_ptr = &*(*silk_CB_lags_stage3_10_ms
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const i8;
        nb_cbk_search = PE_NB_CBKS_STAGE3_10MS;
        cbk_size = PE_NB_CBKS_STAGE3_10MS;
    }
    target_ptr = &*frame.offset(((sf_length as u32) << 2 as libc::c_int) as i32 as isize)
        as *const libc::c_float;
    k = 0 as libc::c_int;
    while k < nb_subfr {
        lag_counter = 0 as libc::c_int;
        basis_ptr = target_ptr.offset(
            -((start_lag
                + *Lag_range_ptr.offset((k * 2 as libc::c_int + 0 as libc::c_int) as isize)
                    as libc::c_int) as isize),
        );
        energy = silk_energy_FLP(basis_ptr, sf_length) + 1e-3f64;
        scratch_mem[lag_counter as usize] = energy as libc::c_float;
        lag_counter += 1;
        lag_diff = *Lag_range_ptr.offset((k * 2 as libc::c_int + 1 as libc::c_int) as isize)
            as libc::c_int
            - *Lag_range_ptr.offset((k * 2 as libc::c_int + 0 as libc::c_int) as isize)
                as libc::c_int
            + 1 as libc::c_int;
        i = 1 as libc::c_int;
        while i < lag_diff {
            energy -= *basis_ptr.offset((sf_length - i) as isize) as libc::c_double
                * *basis_ptr.offset((sf_length - i) as isize) as libc::c_double;
            energy += *basis_ptr.offset(-i as isize) as libc::c_double
                * *basis_ptr.offset(-i as isize) as libc::c_double;
            scratch_mem[lag_counter as usize] = energy as libc::c_float;
            lag_counter += 1;
            i += 1;
        }
        delta = *Lag_range_ptr.offset((k * 2 as libc::c_int + 0 as libc::c_int) as isize)
            as libc::c_int;
        i = 0 as libc::c_int;
        while i < nb_cbk_search {
            idx = *Lag_CB_ptr.offset((k * cbk_size + i) as isize) as libc::c_int - delta;
            j = 0 as libc::c_int;
            while j < PE_NB_STAGE3_LAGS {
                (*energies_st3.offset(k as isize))[i as usize][j as usize] =
                    scratch_mem[(idx + j) as usize];
                j += 1;
            }
            i += 1;
        }
        target_ptr = target_ptr.offset(sf_length as isize);
        k += 1;
    }
}
